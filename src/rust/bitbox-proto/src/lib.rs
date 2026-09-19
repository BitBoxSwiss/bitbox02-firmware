// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod pb {
    include!("./generated/shiftcrypto.bitbox02.rs");
}

pub mod pb_backup {
    include!("./generated/shiftcrypto.bitbox02.backups.rs");
}

// Also clear passphrases discarded during decoding, invalid-state handling or cancellation.
impl Drop for pb::UnlockHostInfoRequest {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.passphrase.zeroize();
    }
}
