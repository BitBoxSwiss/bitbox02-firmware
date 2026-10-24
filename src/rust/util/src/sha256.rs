// SPDX-License-Identifier: Apache-2.0

//! Reuse Bitcoin's SHA-256 backend through the RustCrypto digest interface.

use bitcoin::hashes::{Hash, HashEngine, sha256};
use digest::{
    FixedOutput, FixedOutputReset, HashMarker, Output, OutputSizeUser, Reset, Update,
    consts::{U32, U64},
    core_api::BlockSizeUser,
};

#[derive(Clone, Default)]
pub struct Sha256(sha256::HashEngine);

impl HashMarker for Sha256 {}

impl OutputSizeUser for Sha256 {
    type OutputSize = U32;
}

impl BlockSizeUser for Sha256 {
    type BlockSize = U64;
}

impl Update for Sha256 {
    fn update(&mut self, data: &[u8]) {
        self.0.input(data);
    }
}

impl FixedOutput for Sha256 {
    fn finalize_into(self, out: &mut Output<Self>) {
        out.copy_from_slice(sha256::Hash::from_engine(self.0).as_byte_array());
    }
}

impl Reset for Sha256 {
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl FixedOutputReset for Sha256 {
    fn finalize_into_reset(&mut self, out: &mut Output<Self>) {
        core::mem::take(self).finalize_into(out);
    }
}

#[cfg(test)]
mod tests {
    use super::Sha256;
    use digest::Digest;
    use hmac::{Hmac, Mac, SimpleHmac};

    #[test]
    fn test_sha256_matches_rustcrypto() {
        let data: std::vec::Vec<u8> = (0..1024).map(|i| i as u8).collect();
        for len in [
            0, 1, 31, 32, 55, 56, 63, 64, 65, 119, 120, 127, 128, 129, 1024,
        ] {
            let input = &data[..len];
            let expected = sha2::Sha256::digest(input);
            assert_eq!(Sha256::digest(input), expected);
            for chunk_size in [1, 7, 64, 113] {
                let mut engine = Sha256::new();
                for chunk in input.chunks(chunk_size) {
                    engine.update(chunk);
                }
                assert_eq!(engine.clone().finalize(), expected);
                assert_eq!(engine.finalize_reset(), expected);
                engine.update(input);
                assert_eq!(engine.finalize_reset(), expected);
                assert_eq!(engine.finalize(), sha2::Sha256::digest([]));
                let mut engine = Sha256::new();
                engine.update(input);
                engine.reset();
                assert_eq!(engine.finalize(), sha2::Sha256::digest([]));
            }
        }
    }

    #[test]
    fn test_hmac_matches_rustcrypto() {
        for key_len in [0, 32, 63, 64, 65, 131] {
            for msg_len in [0, 1, 55, 56, 64, 65, 128] {
                let key = std::vec![0xa5; key_len];
                let msg = std::vec![0x5a; msg_len];
                let mut reference = Hmac::<sha2::Sha256>::new_from_slice(&key).unwrap();
                reference.update(&msg);
                let expected = reference.finalize().into_bytes();
                let mut mac = SimpleHmac::<Sha256>::new_from_slice(&key).unwrap();
                for chunk in msg.chunks(7) {
                    mac.update(chunk);
                }
                assert_eq!(mac.finalize_reset().into_bytes(), expected);
                mac.update(&msg);
                assert_eq!(mac.finalize().into_bytes(), expected);
            }
        }
    }
}
