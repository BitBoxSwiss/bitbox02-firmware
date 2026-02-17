// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

use crate::hal::Memory;
use crate::hal::memory::{Error, PasswordStretchAlgo, Platform, SecurechipType};

pub(crate) struct BitBox02Memory;

fn to_hal_securechip_type(securechip_type: bitbox02::memory::SecurechipType) -> SecurechipType {
    match securechip_type {
        bitbox02::memory::SecurechipType::Atecc => SecurechipType::Atecc,
        bitbox02::memory::SecurechipType::Optiga => SecurechipType::Optiga,
    }
}

fn to_hal_platform(platform: bitbox02::memory::Platform) -> Platform {
    match platform {
        bitbox02::memory::Platform::BitBox02 => Platform::BitBox02,
        bitbox02::memory::Platform::BitBox02Plus => Platform::BitBox02Plus,
    }
}

fn to_hal_password_stretch_algo(
    algo: bitbox02::memory::PasswordStretchAlgo,
) -> PasswordStretchAlgo {
    match algo {
        bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0 => {
            PasswordStretchAlgo::V0
        }
        bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1 => {
            PasswordStretchAlgo::V1
        }
    }
}

fn to_hal_error(error: bitbox02::memory::MemoryError) -> Error {
    match error {
        bitbox02::memory::MemoryError::MEMORY_OK => {
            unreachable!("MEMORY_OK must not be converted to hal::memory::Error")
        }
        bitbox02::memory::MemoryError::MEMORY_ERR_INVALID_INPUT => Error::InvalidInput,
        bitbox02::memory::MemoryError::MEMORY_ERR_FULL => Error::Full,
        bitbox02::memory::MemoryError::MEMORY_ERR_DUPLICATE_NAME => Error::DuplicateName,
        bitbox02::memory::MemoryError::MEMORY_ERR_UNKNOWN => Error::Unknown,
    }
}

pub(super) fn to_bitbox02_password_stretch_algo(
    algo: PasswordStretchAlgo,
) -> bitbox02::memory::PasswordStretchAlgo {
    match algo {
        PasswordStretchAlgo::V0 => {
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0
        }
        PasswordStretchAlgo::V1 => {
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1
        }
    }
}

impl Memory for BitBox02Memory {
    fn ble_enabled(&mut self) -> bool {
        bitbox02::memory::ble_enabled()
    }

    fn ble_enable(&mut self, enable: bool) -> Result<(), ()> {
        bitbox02::memory::ble_enable(enable)
    }

    fn get_securechip_type(&mut self) -> Result<SecurechipType, ()> {
        bitbox02::memory::get_securechip_type().map(to_hal_securechip_type)
    }

    fn get_platform(&mut self) -> Result<Platform, ()> {
        bitbox02::memory::get_platform().map(to_hal_platform)
    }

    fn get_device_name(&mut self) -> String {
        bitbox02::memory::get_device_name()
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), Error> {
        bitbox02::memory::set_device_name(name).map_err(to_hal_error)
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

    fn get_encrypted_seed_and_hmac(&mut self) -> Result<(Vec<u8>, PasswordStretchAlgo), ()> {
        bitbox02::memory::get_encrypted_seed_and_hmac()
            .map(|(seed, algo)| (seed, to_hal_password_stretch_algo(algo)))
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<(), ()> {
        bitbox02::memory::set_encrypted_seed_and_hmac(
            data,
            to_bitbox02_password_stretch_algo(password_stretch_algo),
        )
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

    fn multisig_set_by_hash(&mut self, hash: &[u8; 32], name: &str) -> Result<(), Error> {
        bitbox02::memory::multisig_set_by_hash(hash, name).map_err(to_hal_error)
    }

    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String> {
        bitbox02::memory::multisig_get_by_hash(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hal_securechip_type() {
        assert_eq!(
            to_hal_securechip_type(bitbox02::memory::SecurechipType::Atecc),
            SecurechipType::Atecc,
        );
        assert_eq!(
            to_hal_securechip_type(bitbox02::memory::SecurechipType::Optiga),
            SecurechipType::Optiga,
        );
    }

    #[test]
    fn test_to_hal_platform() {
        assert_eq!(
            to_hal_platform(bitbox02::memory::Platform::BitBox02),
            Platform::BitBox02,
        );
        assert_eq!(
            to_hal_platform(bitbox02::memory::Platform::BitBox02Plus),
            Platform::BitBox02Plus,
        );
    }

    #[test]
    fn test_to_hal_error() {
        let cases = [
            (
                bitbox02::memory::MemoryError::MEMORY_ERR_INVALID_INPUT,
                Error::InvalidInput,
            ),
            (bitbox02::memory::MemoryError::MEMORY_ERR_FULL, Error::Full),
            (
                bitbox02::memory::MemoryError::MEMORY_ERR_DUPLICATE_NAME,
                Error::DuplicateName,
            ),
            (
                bitbox02::memory::MemoryError::MEMORY_ERR_UNKNOWN,
                Error::Unknown,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(to_hal_error(input), expected);
        }
    }

    #[test]
    #[should_panic(expected = "MEMORY_OK must not be converted to hal::memory::Error")]
    fn test_to_hal_error_memory_ok_panics() {
        let _ = to_hal_error(bitbox02::memory::MemoryError::MEMORY_OK);
    }

    #[test]
    fn test_password_stretch_algo_mappings() {
        assert_eq!(
            to_hal_password_stretch_algo(
                bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0,
            ),
            PasswordStretchAlgo::V0,
        );
        assert_eq!(
            to_hal_password_stretch_algo(
                bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1,
            ),
            PasswordStretchAlgo::V1,
        );
        assert_eq!(
            to_bitbox02_password_stretch_algo(PasswordStretchAlgo::V0) as i32,
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0 as i32,
        );
        assert_eq!(
            to_bitbox02_password_stretch_algo(PasswordStretchAlgo::V1) as i32,
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1 as i32,
        );
    }
}
