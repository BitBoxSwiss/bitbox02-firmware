// SPDX-License-Identifier: Apache-2.0

use digest::Digest;
use noise_protocol::{Hash, U8Array};
use noise_rust_crypto::sensitive::Sensitive;

#[derive(Default, Clone)]
pub struct Sha256(util::sha256::Sha256);

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
