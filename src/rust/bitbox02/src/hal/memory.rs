// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

use bitbox_hal::Memory;
use bitbox_hal::memory::{
    BleFirmwareSlot, BleMetadata, Error, PasswordStretchAlgo, Platform, SecurechipType,
};

pub struct BitBox02Memory;

fn to_hal_securechip_type(securechip_type: crate::memory::SecurechipType) -> SecurechipType {
    match securechip_type {
        crate::memory::SecurechipType::Atecc => SecurechipType::Atecc,
        crate::memory::SecurechipType::Optiga => SecurechipType::Optiga,
    }
}

fn to_hal_platform(platform: crate::memory::Platform) -> Platform {
    match platform {
        crate::memory::Platform::BitBox02 => Platform::BitBox02,
        crate::memory::Platform::BitBox02Plus => Platform::BitBox02Plus,
    }
}

fn to_hal_password_stretch_algo(algo: crate::memory::PasswordStretchAlgo) -> PasswordStretchAlgo {
    match algo {
        crate::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0 => {
            PasswordStretchAlgo::V0
        }
        crate::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1 => {
            PasswordStretchAlgo::V1
        }
    }
}

fn to_hal_error(error: crate::memory::MemoryError) -> Error {
    match error {
        crate::memory::MemoryError::MEMORY_OK => {
            unreachable!("MEMORY_OK must not be converted to hal::memory::Error")
        }
        crate::memory::MemoryError::MEMORY_ERR_INVALID_INPUT => Error::InvalidInput,
        crate::memory::MemoryError::MEMORY_ERR_FULL => Error::Full,
        crate::memory::MemoryError::MEMORY_ERR_DUPLICATE_NAME => Error::DuplicateName,
        crate::memory::MemoryError::MEMORY_ERR_UNKNOWN => Error::Unknown,
    }
}

fn to_hal_ble_metadata(metadata: crate::memory::BleMetadata) -> BleMetadata {
    BleMetadata {
        allowed_firmware_hash: metadata.allowed_firmware_hash,
        active_index: metadata.active_index,
        firmware_sizes: metadata.firmware_sizes,
        firmware_checksums: metadata.firmware_checksums,
    }
}

pub(super) fn to_bitbox02_password_stretch_algo(
    algo: PasswordStretchAlgo,
) -> crate::memory::PasswordStretchAlgo {
    match algo {
        PasswordStretchAlgo::V0 => {
            crate::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0
        }
        PasswordStretchAlgo::V1 => {
            crate::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1
        }
    }
}

fn to_bitbox02_ble_metadata(metadata: &BleMetadata) -> crate::memory::BleMetadata {
    crate::memory::BleMetadata {
        allowed_firmware_hash: metadata.allowed_firmware_hash,
        active_index: metadata.active_index,
        firmware_sizes: metadata.firmware_sizes,
        firmware_checksums: metadata.firmware_checksums,
    }
}

impl Memory for BitBox02Memory {
    /// We want to write FW to the memory chip in erase-size chunks, so that we don't repeatedly
    /// need to read-erase-write the same sector.
    const BLE_FW_FLASH_CHUNK_SIZE: u32 = 4096;

    fn ble_enabled(&mut self) -> bool {
        crate::memory::ble_enabled()
    }

    fn ble_enable(&mut self, enable: bool) -> Result<(), ()> {
        crate::memory::ble_enable(enable)
    }

    fn get_active_ble_firmware_version(&mut self) -> Result<String, Error> {
        crate::spi_mem::get_active_ble_firmware_version().map_err(|_| Error::Unknown)
    }

    fn ble_firmware_flash_chunk(
        &mut self,
        slot: BleFirmwareSlot,
        chunk_index: u32,
        chunk: &[u8],
    ) -> Result<(), Error> {
        if chunk.len() > Self::BLE_FW_FLASH_CHUNK_SIZE as usize {
            return Err(Error::InvalidInput);
        }
        let base = match slot {
            BleFirmwareSlot::First => crate::spi_mem::BLE_FIRMWARE_1_ADDR,
            BleFirmwareSlot::Second => crate::spi_mem::BLE_FIRMWARE_2_ADDR,
        };
        let chunk_offset: u32 = chunk_index
            .checked_mul(Self::BLE_FW_FLASH_CHUNK_SIZE)
            .ok_or(Error::InvalidInput)?;
        let address = base.checked_add(chunk_offset).ok_or(Error::InvalidInput)?;
        crate::spi_mem::write_protected(address, chunk).map_err(|_| Error::Unknown)
    }

    fn ble_get_metadata(&mut self) -> BleMetadata {
        to_hal_ble_metadata(crate::memory::get_ble_metadata())
    }

    fn set_ble_metadata(&mut self, metadata: &BleMetadata) -> Result<(), Error> {
        let metadata = to_bitbox02_ble_metadata(metadata);
        crate::memory::set_ble_metadata(&metadata).map_err(|_| Error::Unknown)
    }

    fn get_securechip_type(&mut self) -> Result<SecurechipType, ()> {
        crate::memory::get_securechip_type().map(to_hal_securechip_type)
    }

    fn get_platform(&mut self) -> Result<Platform, ()> {
        crate::memory::get_platform().map(to_hal_platform)
    }

    fn get_device_name(&mut self) -> String {
        crate::memory::get_device_name()
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), Error> {
        crate::memory::set_device_name(name).map_err(to_hal_error)
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        crate::memory::is_mnemonic_passphrase_enabled()
    }

    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        crate::memory::set_mnemonic_passphrase_enabled(enabled)
    }

    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
        crate::memory::set_seed_birthdate(timestamp)
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        crate::memory::get_seed_birthdate()
    }

    fn is_seeded(&mut self) -> bool {
        crate::memory::is_seeded()
    }

    fn is_initialized(&mut self) -> bool {
        crate::memory::is_initialized()
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        crate::memory::set_initialized()
    }

    fn get_encrypted_seed_and_hmac(&mut self) -> Result<(Vec<u8>, PasswordStretchAlgo), ()> {
        crate::memory::get_encrypted_seed_and_hmac()
            .map(|(seed, algo)| (seed, to_hal_password_stretch_algo(algo)))
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<(), ()> {
        crate::memory::set_encrypted_seed_and_hmac(
            data,
            to_bitbox02_password_stretch_algo(password_stretch_algo),
        )
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        crate::memory::reset_hww()
    }

    fn get_unlock_attempts(&mut self) -> u8 {
        crate::memory::smarteeprom_get_unlock_attempts()
    }

    fn increment_unlock_attempts(&mut self) {
        crate::memory::smarteeprom_increment_unlock_attempts()
    }

    fn reset_unlock_attempts(&mut self) {
        crate::memory::smarteeprom_reset_unlock_attempts()
    }

    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        crate::memory::get_salt_root()
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        crate::memory::get_attestation_pubkey_and_certificate(
            pubkey_out,
            certificate_out,
            root_pubkey_identifier_out,
        )
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        crate::memory::get_attestation_bootloader_hash()
    }

    fn multisig_set_by_hash(&mut self, hash: &[u8; 32], name: &str) -> Result<(), Error> {
        crate::memory::multisig_set_by_hash(hash, name).map_err(to_hal_error)
    }

    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String> {
        crate::memory::multisig_get_by_hash(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hal_securechip_type() {
        assert_eq!(
            to_hal_securechip_type(crate::memory::SecurechipType::Atecc),
            SecurechipType::Atecc,
        );
        assert_eq!(
            to_hal_securechip_type(crate::memory::SecurechipType::Optiga),
            SecurechipType::Optiga,
        );
    }

    #[test]
    fn test_to_hal_platform() {
        assert_eq!(
            to_hal_platform(crate::memory::Platform::BitBox02),
            Platform::BitBox02,
        );
        assert_eq!(
            to_hal_platform(crate::memory::Platform::BitBox02Plus),
            Platform::BitBox02Plus,
        );
    }

    #[test]
    fn test_to_hal_error() {
        let cases = [
            (
                crate::memory::MemoryError::MEMORY_ERR_INVALID_INPUT,
                Error::InvalidInput,
            ),
            (crate::memory::MemoryError::MEMORY_ERR_FULL, Error::Full),
            (
                crate::memory::MemoryError::MEMORY_ERR_DUPLICATE_NAME,
                Error::DuplicateName,
            ),
            (
                crate::memory::MemoryError::MEMORY_ERR_UNKNOWN,
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
        let _ = to_hal_error(crate::memory::MemoryError::MEMORY_OK);
    }

    #[test]
    fn test_password_stretch_algo_mappings() {
        assert_eq!(
            to_hal_password_stretch_algo(
                crate::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0,
            ),
            PasswordStretchAlgo::V0,
        );
        assert_eq!(
            to_hal_password_stretch_algo(
                crate::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1,
            ),
            PasswordStretchAlgo::V1,
        );
        assert_eq!(
            to_bitbox02_password_stretch_algo(PasswordStretchAlgo::V0) as i32,
            crate::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0 as i32,
        );
        assert_eq!(
            to_bitbox02_password_stretch_algo(PasswordStretchAlgo::V1) as i32,
            crate::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1 as i32,
        );
    }

    #[test]
    fn test_to_hal_ble_metadata() {
        let input = crate::memory::BleMetadata {
            allowed_firmware_hash: [0x11; 32],
            active_index: 1,
            firmware_sizes: [1234, 5678],
            firmware_checksums: [0xaa, 0xbb],
        };

        let output = to_hal_ble_metadata(input);

        assert_eq!(output.allowed_firmware_hash, [0x11; 32]);
        assert_eq!(output.active_index, 1);
        assert_eq!(output.firmware_sizes, [1234, 5678]);
        assert_eq!(output.firmware_checksums, [0xaa, 0xbb]);
    }

    #[test]
    fn test_to_bitbox02_ble_metadata() {
        let input = BleMetadata {
            allowed_firmware_hash: [0x22; 32],
            active_index: 0,
            firmware_sizes: [100, 200],
            firmware_checksums: [0xcc, 0xdd],
        };

        let output = to_bitbox02_ble_metadata(&input);

        assert_eq!(output.allowed_firmware_hash, [0x22; 32]);
        assert_eq!(output.active_index, 0);
        assert_eq!(output.firmware_sizes, [100, 200]);
        assert_eq!(output.firmware_checksums, [0xcc, 0xdd]);
    }
}
