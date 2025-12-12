// SPDX-License-Identifier: Apache-2.0

use crate::hal::SecureChip;
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
    bitbox02::memory::get_attestation_pubkey_and_certificate(
        &mut result.device_pubkey,
        &mut result.certificate,
        &mut result.root_pubkey_identifier,
    )?;
    let hash: [u8; 32] = Sha256::digest(host_challenge).into();
    result.bootloader_hash = bitbox02::memory::get_attestation_bootloader_hash();
    hal.securechip()
        .attestation_sign(&hash, &mut result.challenge_signature)?;
    Ok(result)
}
