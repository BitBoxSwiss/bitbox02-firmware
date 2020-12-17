//! ChaCha20 stream cipher implementation.
//!
//! Adapted from the `ctr` crate.

// TODO(tarcieri): figure out how to unify this with the `ctr` crate (see #95)

use crate::{
    block::{Block, BUFFER_SIZE},
    rounds::{Rounds, R12, R20, R8},
    BLOCK_SIZE, MAX_BLOCKS,
};
use core::{
    convert::TryInto,
    fmt::{self, Debug},
};
use stream_cipher::{
    consts::{U12, U32},
    LoopError, NewStreamCipher, OverflowError, SeekNum, SyncStreamCipher, SyncStreamCipherSeek,
};

/// ChaCha8 stream cipher (reduced-round variant of ChaCha20 with 8 rounds)
pub type ChaCha8 = Cipher<R8>;

/// ChaCha12 stream cipher (reduced-round variant of ChaCha20 with 12 rounds)
pub type ChaCha12 = Cipher<R12>;

/// ChaCha20 stream cipher (RFC 8439 version with 96-bit nonce)
pub type ChaCha20 = Cipher<R20>;

/// ChaCha20 key type (256-bits/32-bytes)
///
/// Implemented as an alias for [`GenericArray`].
///
/// (NOTE: all variants of [`ChaCha20`] including `XChaCha20` use the same key type)
pub type Key = stream_cipher::Key<ChaCha20>;

/// Nonce type (96-bits/12-bytes)
///
/// Implemented as an alias for [`GenericArray`].
pub type Nonce = stream_cipher::Nonce<ChaCha20>;

/// Internal buffer
type Buffer = [u8; BUFFER_SIZE];

/// How much to increment the counter by for each buffer we generate.
/// Normally this is 1 but the AVX2 backend uses double-wide buffers.
// TODO(tarcieri): support a parallel blocks count like the `ctr` crate
// See: <https://github.com/RustCrypto/stream-ciphers/blob/907e94b/ctr/src/lib.rs#L73>
const COUNTER_INCR: u64 = (BUFFER_SIZE as u64) / (BLOCK_SIZE as u64);

/// ChaCha family stream cipher, generic around a number of rounds.
///
/// Use the [`ChaCha8`], [`ChaCha12`], or [`ChaCha20`] type aliases to select
/// a specific number of rounds.
///
/// Generally [`ChaCha20`] is preferred.
pub struct Cipher<R: Rounds> {
    /// ChaCha20 block function initialized with a key and IV
    block: Block<R>,

    /// Buffer containing previous block function output
    buffer: Buffer,

    /// Position within buffer, or `None` if the buffer is not in use
    buffer_pos: u8,

    /// Current counter value relative to the start of the keystream
    counter: u64,

    /// Offset of the initial counter in the keystream. This is derived from
    /// the extra 4 bytes in the 96-byte nonce RFC 8439 version (or is always
    /// 0 in the legacy version)
    counter_offset: u64,
}

impl<R: Rounds> NewStreamCipher for Cipher<R> {
    /// Key size in bytes
    type KeySize = U32;

    /// Nonce size in bytes
    type NonceSize = U12;

    fn new(key: &Key, nonce: &Nonce) -> Self {
        let block = Block::new(
            key.as_slice().try_into().unwrap(),
            nonce[4..12].try_into().unwrap(),
        );

        let counter_offset = (u64::from(nonce[0]) & 0xff) << 32
            | (u64::from(nonce[1]) & 0xff) << 40
            | (u64::from(nonce[2]) & 0xff) << 48
            | (u64::from(nonce[3]) & 0xff) << 56;

        Self {
            block,
            buffer: [0u8; BUFFER_SIZE],
            buffer_pos: 0,
            counter: 0,
            counter_offset,
        }
    }
}

impl<R: Rounds> SyncStreamCipher for Cipher<R> {
    fn try_apply_keystream(&mut self, mut data: &mut [u8]) -> Result<(), LoopError> {
        self.check_data_len(data)?;
        let pos = self.buffer_pos as usize;

        let mut counter = self.counter;
        // xor with leftover bytes from the last call if any
        if pos != 0 {
            if data.len() < BUFFER_SIZE - pos {
                let n = pos + data.len();
                xor(data, &self.buffer[pos..n]);
                self.buffer_pos = n as u8;
                return Ok(());
            } else {
                let (l, r) = data.split_at_mut(BUFFER_SIZE - pos);
                data = r;
                xor(l, &self.buffer[pos..]);
                counter = counter.checked_add(COUNTER_INCR).unwrap();
            }
        }

        let mut chunks = data.chunks_exact_mut(BUFFER_SIZE);
        for chunk in &mut chunks {
            // TODO(tarcieri): double check this should be checked and not wrapping
            let counter_with_offset = self.counter_offset.checked_add(counter).unwrap();
            self.block.apply_keystream(counter_with_offset, chunk);
            counter = counter.checked_add(COUNTER_INCR).unwrap();
        }

        let rem = chunks.into_remainder();
        self.buffer_pos = rem.len() as u8;
        self.counter = counter;
        if !rem.is_empty() {
            self.generate_block(counter);
            xor(rem, &self.buffer[..rem.len()]);
        }

        Ok(())
    }
}

impl<R: Rounds> SyncStreamCipherSeek for Cipher<R> {
    fn try_current_pos<T: SeekNum>(&self) -> Result<T, OverflowError> {
        // quick and dirty fix, until ctr-like parallel block processing will be added
        let (counter, pos) = if self.buffer_pos < BLOCK_SIZE as u8 {
            (self.counter, self.buffer_pos)
        } else {
            (
                self.counter.checked_add(1).ok_or(OverflowError)?,
                self.buffer_pos - BLOCK_SIZE as u8,
            )
        };
        T::from_block_byte(counter, pos, BLOCK_SIZE as u8)
    }

    fn try_seek<T: SeekNum>(&mut self, pos: T) -> Result<(), LoopError> {
        let res = pos.to_block_byte(BLOCK_SIZE as u8)?;
        self.counter = res.0;
        self.buffer_pos = res.1;
        if self.buffer_pos != 0 {
            self.generate_block(self.counter);
        }
        Ok(())
    }
}

impl<R: Rounds> Cipher<R> {
    /// Check data length
    fn check_data_len(&self, data: &[u8]) -> Result<(), LoopError> {
        let leftover_bytes = BUFFER_SIZE - self.buffer_pos as usize;
        if data.len() < leftover_bytes {
            return Ok(());
        }
        let blocks = 1 + (data.len() - leftover_bytes) / BLOCK_SIZE;
        let res = self.counter.checked_add(blocks as u64).ok_or(LoopError)?;
        if res <= MAX_BLOCKS as u64 {
            Ok(())
        } else {
            Err(LoopError)
        }
    }

    /// Generate a block, storing it in the internal buffer
    #[inline]
    fn generate_block(&mut self, counter: u64) {
        // TODO(tarcieri): double check this should be checked and not wrapping
        let counter_with_offset = self.counter_offset.checked_add(counter).unwrap();
        self.block.generate(counter_with_offset, &mut self.buffer);
    }
}

impl<R: Rounds> Debug for Cipher<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Cipher {{ .. }}")
    }
}

#[inline(always)]
fn xor(buf: &mut [u8], key: &[u8]) {
    debug_assert_eq!(buf.len(), key.len());
    for (a, b) in buf.iter_mut().zip(key) {
        *a ^= *b;
    }
}
