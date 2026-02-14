// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

use crate::hal::Memory;

pub struct BitBox02Memory;

impl Memory for BitBox02Memory {
    fn get_securechip_type(&mut self) -> Result<bitbox02::memory::SecurechipType, ()> {
        bitbox02::memory::get_securechip_type()
    }

    fn get_platform(&mut self) -> Result<bitbox02::memory::Platform, ()> {
        bitbox02::memory::get_platform()
    }

    fn get_device_name(&mut self) -> String {
        bitbox02::memory::get_device_name()
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), bitbox02::memory::Error> {
        bitbox02::memory::set_device_name(name)
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        bitbox02::memory::is_mnemonic_passphrase_enabled()
    }

    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        bitbox02::memory::set_mnemonic_passphrase_enabled(enabled)
    }

    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
        bitbox02::memory::set_seed_birthdate(timestamp)
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        bitbox02::memory::get_seed_birthdate()
    }

    fn is_seeded(&mut self) -> bool {
        bitbox02::memory::is_seeded()
    }

    fn is_initialized(&mut self) -> bool {
        bitbox02::memory::is_initialized()
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        bitbox02::memory::set_initialized()
    }

    fn get_encrypted_seed_and_hmac(
        &mut self,
    ) -> Result<(Vec<u8>, bitbox02::memory::PasswordStretchAlgo), ()> {
        bitbox02::memory::get_encrypted_seed_and_hmac()
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<(), ()> {
        bitbox02::memory::set_encrypted_seed_and_hmac(data, password_stretch_algo)
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        bitbox02::memory::reset_hww()
    }

    fn get_unlock_attempts(&mut self) -> u8 {
        bitbox02::memory::smarteeprom_get_unlock_attempts()
    }

    fn increment_unlock_attempts(&mut self) {
        bitbox02::memory::smarteeprom_increment_unlock_attempts()
    }

    fn reset_unlock_attempts(&mut self) {
        bitbox02::memory::smarteeprom_reset_unlock_attempts()
    }

    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        bitbox02::memory::get_salt_root()
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        bitbox02::memory::get_attestation_pubkey_and_certificate(
            pubkey_out,
            certificate_out,
            root_pubkey_identifier_out,
        )
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        bitbox02::memory::get_attestation_bootloader_hash()
    }

    fn multisig_set_by_hash(
        &mut self,
        hash: &[u8; 32],
        name: &str,
    ) -> Result<(), bitbox02::memory::MemoryError> {
        bitbox02::memory::multisig_set_by_hash(hash, name)
    }

    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String> {
        bitbox02::memory::multisig_get_by_hash(hash)
    }
}
