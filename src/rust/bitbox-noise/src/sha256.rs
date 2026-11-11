// SPDX-License-Identifier: Apache-2.0

// Adapter derived from noise-rust-crypto 0.6.2, originally licensed under the Unlicense.
// https://github.com/sopium/noise-rust/blob/36bc85bd05db04b0d469843275ccafd848cddb99/noise-rust-crypto/src/lib.rs

use noise_protocol::{Hash, U8Array};
use sha2::Digest;

use crate::sensitive::Sensitive;

#[derive(Default, Clone)]
pub struct Sha256(sha2::Sha256);

impl Hash for Sha256 {
    fn name() -> &'static str {
        "SHA256"
    }

    type Block = [u8; 64];
    type Output = Sensitive<[u8; 32]>;

    fn input(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    fn result(&mut self) -> Self::Output {
        Self::Output::from_slice(self.0.finalize_reset().as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;

    #[test]
    fn test_result_resets() {
        let mut hash = Sha256::default();
        hash.input(b"a");
        hash.input(b"bc");
        let mut cloned = hash.clone();
        let expected = hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(hash.result().as_slice(), expected);
        assert_eq!(cloned.result().as_slice(), expected);
        assert_eq!(
            hash.result().as_slice(),
            hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
        hash.input(b"discarded");
        hash.reset();
        hash.input(b"abc");
        assert_eq!(hash.result().as_slice(), expected);
    }
}
