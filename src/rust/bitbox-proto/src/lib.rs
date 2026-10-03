// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod pb {
    include!("./generated/shiftcrypto.bitbox02.rs");
}

pub mod pb_backup {
    include!("./generated/shiftcrypto.bitbox02.backups.rs");
}

impl zeroize::Zeroize for pb_backup::BackupData {
    fn zeroize(&mut self) {
        self.seed_length.zeroize();
        self.seed.zeroize();
        self.birthdate.zeroize();
        self.generator.zeroize();
    }
}

// Keep seed cleanup on the owning message, including when it is nested in a backup.
impl Drop for pb_backup::BackupData {
    fn drop(&mut self) {
        zeroize::Zeroize::zeroize(self);
    }
}

// Also clear passphrases discarded during decoding, invalid-state handling or cancellation.
impl Drop for pb::UnlockHostInfoRequest {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.passphrase.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zeroize::Zeroize;

    #[test]
    fn test_backup_data_zeroize() {
        let mut data = pb_backup::BackupData {
            seed_length: 32,
            seed: prost::alloc::vec![0x5a; 32],
            birthdate: 1234,
            generator: "test".into(),
        };
        data.zeroize();
        assert_eq!(data.seed_length, 0);
        assert!(data.seed.is_empty());
        assert_eq!(data.birthdate, 0);
        assert!(data.generator.is_empty());
    }
}
