// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

/// Maximum device name length in bytes, excluding the null terminator used in C strings.
pub const DEVICE_NAME_MAX_LEN: usize = 63;
/// Maximum multisig account name length in bytes, excluding the null terminator used in C
/// strings.
pub const MULTISIG_NAME_MAX_LEN: usize = 30;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PasswordStretchAlgo {
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
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidInput,
    Full,
    DuplicateName,
    Unknown,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BleMetadata {
    pub allowed_firmware_hash: [u8; 32],
    pub active_index: u8,
    pub firmware_sizes: [u16; 2],
    pub firmware_checksums: [u8; 2],
}

pub trait Memory {
    fn ble_enabled(&mut self) -> bool;
    fn ble_enable(&mut self, enable: bool) -> Result<(), ()>;
    fn ble_get_metadata(&mut self) -> BleMetadata;
    fn set_ble_metadata(&mut self, metadata: &BleMetadata) -> Result<(), Error>;
    fn get_securechip_type(&mut self) -> Result<SecurechipType, ()>;
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
    /// `name` must be non-empty and at most [`MULTISIG_NAME_MAX_LEN`] bytes long.
    fn multisig_set_by_hash(&mut self, hash: &[u8; 32], name: &str) -> Result<(), Error>;
    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String>;
}
