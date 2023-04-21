//! Legacy version of ChaCha20 with a 64-bit nonce

use crate::{
    chacha::{ChaCha, Key},
    max_blocks::C64,
    rounds::R20,
};

use cipher::{
    consts::{U32, U8},
    errors::{LoopError, OverflowError},
    NewCipher, SeekNum, StreamCipher, StreamCipherSeek,
};

/// Size of the nonce for the legacy ChaCha20 stream cipher
#[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
pub type LegacyNonce = cipher::Nonce<ChaCha20Legacy>;

/// The ChaCha20 stream cipher (legacy "djb" construction with 64-bit nonce).
///
/// The `legacy` Cargo feature must be enabled to use this.
#[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
pub struct ChaCha20Legacy(ChaCha<R20, C64>);

impl NewCipher for ChaCha20Legacy {
    /// Key size in bytes
    type KeySize = U32;

    /// Nonce size in bytes
    type NonceSize = U8;

    fn new(key: &Key, nonce: &LegacyNonce) -> Self {
        let mut exp_iv = [0u8; 12];
        exp_iv[4..].copy_from_slice(nonce);
        ChaCha20Legacy(ChaCha::new(key, &exp_iv.into()))
    }
}

impl StreamCipher for ChaCha20Legacy {
    fn try_apply_keystream(&mut self, data: &mut [u8]) -> Result<(), LoopError> {
        self.0.try_apply_keystream(data)
    }
}

impl StreamCipherSeek for ChaCha20Legacy {
    fn try_current_pos<T: SeekNum>(&self) -> Result<T, OverflowError> {
        self.0.try_current_pos()
    }

    fn try_seek<T: SeekNum>(&mut self, pos: T) -> Result<(), LoopError> {
        self.0.try_seek(pos)
    }
}
