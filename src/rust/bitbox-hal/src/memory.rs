// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

/// Maximum device name length in bytes, excluding the null terminator used in C strings.
pub const DEVICE_NAME_MAX_LEN: usize = 63;
/// Maximum multisig account name length in bytes, excluding the null terminator used in C
/// strings.
pub const MULTISIG_NAME_MAX_LEN: usize = 30;
/// Maximum allowed BLE firmware size in bytes.
pub const BLE_FIRMWARE_MAX_SIZE: usize = 32 * 1024;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PasswordStretchAlgo {
    V0,
    V1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum OptigaConfigVersion {
    V0,
    V1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SecurechipType {
    Atecc,
    Optiga,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Platform {
    BitBox02,
    BitBox02Plus,
    BitBox03,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidInput,
    Full,
    DuplicateName,
    Unknown,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct BleMetadata {
    pub allowed_firmware_hash: [u8; 32],
    pub active_index: u8,
    pub firmware_sizes: [u16; 2],
    pub firmware_checksums: [u8; 2],
}

impl BleMetadata {
    const VERSION: u8 = 1;
    pub const ENCODED_LEN: usize = 40;

    pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
        let mut out = [0u8; Self::ENCODED_LEN];
        out[0] = Self::VERSION;
        out[1] = self.active_index;
        out[2..4].copy_from_slice(&self.firmware_sizes[0].to_le_bytes());
        out[4..6].copy_from_slice(&self.firmware_sizes[1].to_le_bytes());
        out[6] = self.firmware_checksums[0];
        out[7] = self.firmware_checksums[1];
        out[8..40].copy_from_slice(&self.allowed_firmware_hash);
        out
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() != Self::ENCODED_LEN {
            return None;
        }
        if data[0] != Self::VERSION {
            return None;
        }
        let active_index = data[1];
        if active_index > 1 {
            return None;
        }
        let firmware_sizes = [
            u16::from_le_bytes(data[2..4].try_into().unwrap()),
            u16::from_le_bytes(data[4..6].try_into().unwrap()),
        ];
        if firmware_sizes
            .iter()
            .any(|&size| size as usize > BLE_FIRMWARE_MAX_SIZE)
        {
            return None;
        }
        let mut allowed_firmware_hash = [0u8; 32];
        allowed_firmware_hash.copy_from_slice(&data[8..40]);
        Some(Self {
            allowed_firmware_hash,
            active_index,
            firmware_sizes,
            firmware_checksums: [data[6], data[7]],
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BleFirmwareSlot {
    First,
    Second,
}

pub trait Memory {
    /// We want to write FW to the memory chip in erase-size chunks, so that we don't repeatedly
    /// need to read-erase-write the same sector.
    const BLE_FW_FLASH_CHUNK_SIZE: u32;

    fn ble_enabled(&mut self) -> bool;
    fn ble_enable(&mut self, enable: bool) -> Result<(), ()>;
    fn get_active_ble_firmware_version(&mut self) -> Result<String, Error>;
    fn ble_firmware_flash_chunk(
        &mut self,
        slot: BleFirmwareSlot,
        chunk_index: u32,
        chunk: &[u8],
    ) -> Result<(), Error>;
    fn ble_get_metadata(&mut self) -> BleMetadata;
    fn set_ble_metadata(&mut self, metadata: &BleMetadata) -> Result<(), Error>;
    fn get_securechip_type(&mut self) -> Result<SecurechipType, ()>;
    fn get_optiga_config_version(&mut self) -> Result<OptigaConfigVersion, ()>;
    fn set_optiga_config_version(&mut self, version: OptigaConfigVersion) -> Result<(), ()>;
    fn get_platform(&mut self) -> Result<Platform, ()>;
    fn get_device_name(&mut self) -> String;
    /// `name` must be non-empty and at most [`DEVICE_NAME_MAX_LEN`] bytes long.
    fn set_device_name(&mut self, name: &str) -> Result<(), Error>;
    fn is_mnemonic_passphrase_enabled(&mut self) -> bool;
    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()>;
    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()>;
    fn get_seed_birthdate(&mut self) -> u32;
    fn is_seeded(&mut self) -> bool;
    fn is_initialized(&mut self) -> bool;
    fn set_initialized(&mut self) -> Result<(), ()>;
    fn get_encrypted_seed_and_hmac(&mut self) -> Result<(Vec<u8>, PasswordStretchAlgo), ()>;
    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<(), ()>;
    fn reset_hww(&mut self) -> Result<(), ()>;
    fn get_noise_static_private_key(&mut self) -> Result<zeroize::Zeroizing<[u8; 32]>, ()>;
    fn check_noise_remote_static_pubkey(&mut self, pubkey: &[u8; 32]) -> bool;
    fn add_noise_remote_static_pubkey(&mut self, pubkey: &[u8; 32]) -> Result<(), ()>;
    fn get_io_protection_key(&mut self, out: &mut [u8; 32]);
    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()>;
    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()>;
    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32];
    /// `name` must be non-empty and at most [`MULTISIG_NAME_MAX_LEN`] bytes long.
    fn multisig_set_by_hash(&mut self, hash: &[u8; 32], name: &str) -> Result<(), Error>;
    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String>;
}
