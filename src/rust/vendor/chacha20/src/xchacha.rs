//! XChaCha is an extended nonce variant of ChaCha

use crate::{
    backend::soft::quarter_round,
    chacha::Key,
    max_blocks::C64,
    rounds::{Rounds, R12, R20, R8},
    ChaCha, CONSTANTS,
};
use cipher::{
    consts::{U16, U24, U32},
    errors::{LoopError, OverflowError},
    generic_array::GenericArray,
    NewCipher, SeekNum, StreamCipher, StreamCipherSeek,
};
use core::convert::TryInto;

/// EXtended ChaCha20 nonce (192-bits/24-bytes)
pub type XNonce = cipher::Nonce<XChaCha20>;

/// XChaCha20 is a ChaCha20 variant with an extended 192-bit (24-byte) nonce.
///
/// The construction is an adaptation of the same techniques used by
/// XSalsa20 as described in the paper "Extending the Salsa20 Nonce",
/// applied to the 96-bit nonce variant of ChaCha20, and derive a
/// separate subkey/nonce for each extended nonce:
///
/// <https://cr.yp.to/snuffle/xsalsa-20081128.pdf>
///
/// No authoritative specification exists for XChaCha20, however the
/// construction has "rough consensus and running code" in the form of
/// several interoperable libraries and protocols (e.g. libsodium, WireGuard)
/// and is documented in an (expired) IETF draft:
///
/// <https://tools.ietf.org/html/draft-arciszewski-xchacha-03>
pub type XChaCha20 = XChaCha<R20>;

/// XChaCha12 stream cipher (reduced-round variant of [`XChaCha20`] with 12 rounds)
pub type XChaCha12 = XChaCha<R12>;

/// XChaCha8 stream cipher (reduced-round variant of [`XChaCha20`] with 8 rounds)
pub type XChaCha8 = XChaCha<R8>;

/// XChaCha family stream cipher, generic around a number of rounds.
///
/// Use the [`XChaCha8`], [`XChaCha12`], or [`XChaCha20`] type aliases to select
/// a specific number of rounds.
///
/// Generally [`XChaCha20`] is preferred.
pub struct XChaCha<R: Rounds>(ChaCha<R, C64>);

impl<R: Rounds> NewCipher for XChaCha<R> {
    /// Key size in bytes
    type KeySize = U32;

    /// Nonce size in bytes
    type NonceSize = U24;

    #[allow(unused_mut, clippy::let_and_return)]
    fn new(key: &Key, nonce: &XNonce) -> Self {
        // TODO(tarcieri): zeroize subkey
        let subkey = hchacha::<R>(key, nonce[..16].as_ref().into());
        let mut padded_iv = GenericArray::default();
        padded_iv[4..].copy_from_slice(&nonce[16..]);
        XChaCha(ChaCha::new(&subkey, &padded_iv))
    }
}

impl<R: Rounds> StreamCipher for XChaCha<R> {
    fn try_apply_keystream(&mut self, data: &mut [u8]) -> Result<(), LoopError> {
        self.0.try_apply_keystream(data)
    }
}

impl<R: Rounds> StreamCipherSeek for XChaCha<R> {
    fn try_current_pos<T: SeekNum>(&self) -> Result<T, OverflowError> {
        self.0.try_current_pos()
    }

    fn try_seek<T: SeekNum>(&mut self, pos: T) -> Result<(), LoopError> {
        self.0.try_seek(pos)
    }
}

/// The HChaCha function: adapts the ChaCha core function in the same
/// manner that HSalsa adapts the Salsa function.
///
/// HChaCha takes 512-bits of input:
///
/// - Constants: `u32` x 4
/// - Key: `u32` x 8
/// - Nonce: `u32` x 4
///
/// It produces 256-bits of output suitable for use as a ChaCha key
///
/// For more information on HSalsa on which HChaCha is based, see:
///
/// <http://cr.yp.to/snuffle/xsalsa-20110204.pdf>
#[cfg_attr(docsrs, doc(cfg(feature = "hchacha")))]
pub fn hchacha<R: Rounds>(key: &Key, input: &GenericArray<u8, U16>) -> GenericArray<u8, U32> {
    let mut state = [0u32; 16];
    state[..4].copy_from_slice(&CONSTANTS);

    for (i, chunk) in key.chunks(4).take(8).enumerate() {
        state[4 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    for (i, chunk) in input.chunks(4).enumerate() {
        state[12 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    // R rounds consisting of R/2 column rounds and R/2 diagonal rounds
    for _ in 0..(R::COUNT / 2) {
        // column rounds
        quarter_round(0, 4, 8, 12, &mut state);
        quarter_round(1, 5, 9, 13, &mut state);
        quarter_round(2, 6, 10, 14, &mut state);
        quarter_round(3, 7, 11, 15, &mut state);

        // diagonal rounds
        quarter_round(0, 5, 10, 15, &mut state);
        quarter_round(1, 6, 11, 12, &mut state);
        quarter_round(2, 7, 8, 13, &mut state);
        quarter_round(3, 4, 9, 14, &mut state);
    }

    let mut output = GenericArray::default();

    for (i, chunk) in output.chunks_mut(4).take(4).enumerate() {
        chunk.copy_from_slice(&state[i].to_le_bytes());
    }

    for (i, chunk) in output.chunks_mut(4).skip(4).enumerate() {
        chunk.copy_from_slice(&state[i + 12].to_le_bytes());
    }

    output
}

#[cfg(test)]
mod hchacha20_tests {
    use super::*;

    //
    // Test vectors from:
    // https://tools.ietf.org/id/draft-arciszewski-xchacha-03.html#rfc.section.2.2.1
    //

    const KEY: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ];

    const INPUT: [u8; 16] = [
        0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00, 0x31, 0x41, 0x59,
        0x27,
    ];

    const OUTPUT: [u8; 32] = [
        0x82, 0x41, 0x3b, 0x42, 0x27, 0xb2, 0x7b, 0xfe, 0xd3, 0xe, 0x42, 0x50, 0x8a, 0x87, 0x7d,
        0x73, 0xa0, 0xf9, 0xe4, 0xd5, 0x8a, 0x74, 0xa8, 0x53, 0xc1, 0x2e, 0xc4, 0x13, 0x26, 0xd3,
        0xec, 0xdc,
    ];

    #[test]
    fn test_vector() {
        let actual = hchacha::<R20>(
            GenericArray::from_slice(&KEY),
            &GenericArray::from_slice(&INPUT),
        );
        assert_eq!(actual.as_slice(), &OUTPUT);
    }
}
