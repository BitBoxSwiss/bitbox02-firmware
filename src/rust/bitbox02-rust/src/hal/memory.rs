// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

pub trait Memory {
    fn get_securechip_type(&mut self) -> Result<bitbox02::memory::SecurechipType, ()>;
    fn get_platform(&mut self) -> Result<bitbox02::memory::Platform, ()>;
    fn get_device_name(&mut self) -> String;
    fn set_device_name(&mut self, name: &str) -> Result<(), bitbox02::memory::Error>;
    fn is_mnemonic_passphrase_enabled(&mut self) -> bool;
    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()>;
    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()>;
    fn get_seed_birthdate(&mut self) -> u32;
    fn is_seeded(&mut self) -> bool;
    fn is_initialized(&mut self) -> bool;
    fn set_initialized(&mut self) -> Result<(), ()>;
    fn get_encrypted_seed_and_hmac(
        &mut self,
    ) -> Result<(Vec<u8>, bitbox02::memory::PasswordStretchAlgo), ()>;
    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<(), ()>;
    fn reset_hww(&mut self) -> Result<(), ()>;
    fn get_unlock_attempts(&mut self) -> u8;
    fn increment_unlock_attempts(&mut self);
    fn reset_unlock_attempts(&mut self);
    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()>;
    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()>;
    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32];
    fn multisig_set_by_hash(
        &mut self,
        hash: &[u8; 32],
        name: &str,
    ) -> Result<(), bitbox02::memory::MemoryError>;
    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String>;
}
