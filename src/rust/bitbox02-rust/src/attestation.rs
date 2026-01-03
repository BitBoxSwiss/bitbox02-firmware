// SPDX-License-Identifier: Apache-2.0

use crate::hal::{Memory, SecureChip};
use sha2::{Digest, Sha256};

pub struct Data {
    pub bootloader_hash: [u8; 32],
    pub device_pubkey: [u8; 64],
    pub certificate: [u8; 64],
    pub root_pubkey_identifier: [u8; 32],
    pub challenge_signature: [u8; 64],
}

pub fn perform(hal: &mut impl crate::hal::Hal, host_challenge: [u8; 32]) -> Result<Data, ()> {
    let mut result = Data {
        bootloader_hash: [0; 32],
        device_pubkey: [0; 64],
        certificate: [0; 64],
        root_pubkey_identifier: [0; 32],
        challenge_signature: [0; 64],
    };
    hal.memory().get_attestation_pubkey_and_certificate(
        &mut result.device_pubkey,
        &mut result.certificate,
        &mut result.root_pubkey_identifier,
    )?;
    result.bootloader_hash = hal.memory().get_attestation_bootloader_hash();
    let hash: [u8; 32] = Sha256::digest(host_challenge).into();
    hal.securechip()
        .attestation_sign(&hash, &mut result.challenge_signature)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_perform_success() {
        let mut hal = TestingHal::new();

        let expected_pubkey = [0x55u8; 64];
        let expected_certificate = [0x66u8; 64];
        let expected_root_id = [0x77u8; 32];
        let expected_bootloader_hash = [0x88u8; 32];
        let expected_signature = [0x99u8; 64];

        hal.memory.set_attestation_certificate(
            &expected_pubkey,
            &expected_certificate,
            &expected_root_id,
        );
        hal.memory
            .set_attestation_bootloader_hash(&expected_bootloader_hash);
        hal.securechip
            .set_mock_attestation_signature(&expected_signature);

        let host_challenge = [0x42u8; 32];

        let data = perform(&mut hal, host_challenge).unwrap();

        assert_eq!(data.device_pubkey, expected_pubkey);
        assert_eq!(data.certificate, expected_certificate);
        assert_eq!(data.root_pubkey_identifier, expected_root_id);
        assert_eq!(data.bootloader_hash, expected_bootloader_hash);
        assert_eq!(data.challenge_signature, expected_signature);

        let expected_hash: [u8; 32] = Sha256::digest(host_challenge).into();
        assert_eq!(
            hal.securechip.last_attestation_challenge().unwrap(),
            expected_hash
        );
    }

    #[test]
    fn test_perform_attestation_not_set() {
        let mut hal = TestingHal::new();
        let host_challenge = [0u8; 32];

        // No attestation data configured on hal.memory(),
        // so get_attestation_pubkey_and_certificate should fail
        // and perform() should propagate Err(()).
        assert!(perform(&mut hal, host_challenge).is_err());
    }
}
