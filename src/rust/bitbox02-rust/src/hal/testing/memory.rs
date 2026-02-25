// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

use crate::hal::memory::{
    BleFirmwareSlot, BleMetadata, Error, PasswordStretchAlgo, Platform, SecurechipType,
};

pub struct TestingMemory {
    ble_enabled: bool,
    ble_metadata: BleMetadata,
    ble_firmware_slots: [Vec<u8>; 2],
    active_ble_firmware_version: String,
    securechip_type: SecurechipType,
    platform: Platform,
    initialized: bool,
    is_seeded: bool,
    mnemonic_passphrase_enabled: bool,
    seed_birthdate: u32,
    encrypted_seed_and_hmac: Option<(Vec<u8>, PasswordStretchAlgo)>,
    device_name: Option<String>,
    unlock_attempts: u8,
    salt_root: [u8; 32],
    attestation_device_pubkey: Option<[u8; 64]>,
    attestation_certificate: Option<[u8; 64]>,
    attestation_root_pubkey_identifier: Option<[u8; 32]>,
    attestation_bootloader_hash: [u8; 32],
    multisig_entries: Vec<([u8; 32], String)>,
}

// Same as MEMORY_MULTISIG_NUM_ENTRIES in memory.h.
const MULTISIG_LIMIT: usize = 25;

impl TestingMemory {
    pub fn new() -> Self {
        Self {
            ble_enabled: true,
            ble_metadata: BleMetadata {
                allowed_firmware_hash: [0; 32],
                active_index: 0,
                firmware_sizes: [0; 2],
                firmware_checksums: [0; 2],
            },
            ble_firmware_slots: [
                vec![0xff; crate::hal::memory::BLE_FIRMWARE_MAX_SIZE],
                vec![0xff; crate::hal::memory::BLE_FIRMWARE_MAX_SIZE],
            ],
            active_ble_firmware_version: "0.0.0".into(),
            securechip_type: SecurechipType::Optiga,
            platform: Platform::BitBox02,
            initialized: false,
            is_seeded: false,
            mnemonic_passphrase_enabled: false,
            seed_birthdate: 0,
            encrypted_seed_and_hmac: None,
            device_name: None,
            unlock_attempts: 0,
            salt_root: *b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            attestation_device_pubkey: None,
            attestation_certificate: None,
            attestation_root_pubkey_identifier: None,
            attestation_bootloader_hash: [0; 32],
            multisig_entries: Vec::new(),
        }
    }

    pub fn set_securechip_type(&mut self, securechip_type: SecurechipType) {
        self.securechip_type = securechip_type;
    }

    pub fn set_platform(&mut self, platform: Platform) {
        self.platform = platform;
    }

    pub fn set_unlock_attempts_for_testing(&mut self, attempts: u8) {
        self.unlock_attempts = attempts;
    }

    pub fn set_salt_root(&mut self, salt_root: &[u8; 32]) {
        self.salt_root = *salt_root;
    }

    pub fn set_attestation_certificate(
        &mut self,
        pubkey: &[u8; 64],
        certificate: &[u8; 64],
        root_pubkey_identifier: &[u8; 32],
    ) {
        self.attestation_device_pubkey = Some(*pubkey);
        self.attestation_certificate = Some(*certificate);
        self.attestation_root_pubkey_identifier = Some(*root_pubkey_identifier);
    }

    pub fn set_attestation_bootloader_hash(&mut self, hash: &[u8; 32]) {
        self.attestation_bootloader_hash = *hash;
    }

    pub fn ble_firmware_slot_data(&self, slot: BleFirmwareSlot) -> &[u8] {
        match slot {
            BleFirmwareSlot::First => &self.ble_firmware_slots[0],
            BleFirmwareSlot::Second => &self.ble_firmware_slots[1],
        }
    }
}

impl crate::hal::Memory for TestingMemory {
    const BLE_FW_FLASH_CHUNK_SIZE: u32 = 4096;

    fn ble_enabled(&mut self) -> bool {
        self.ble_enabled
    }

    fn ble_enable(&mut self, enable: bool) -> Result<(), ()> {
        self.ble_enabled = enable;
        Ok(())
    }

    fn get_active_ble_firmware_version(&mut self) -> Result<String, Error> {
        Ok(self.active_ble_firmware_version.clone())
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

        let chunk_offset = (chunk_index as usize)
            .checked_mul(Self::BLE_FW_FLASH_CHUNK_SIZE as usize)
            .ok_or(Error::InvalidInput)?;
        let chunk_end = chunk_offset
            .checked_add(chunk.len())
            .ok_or(Error::InvalidInput)?;

        let slot_data = match slot {
            BleFirmwareSlot::First => &mut self.ble_firmware_slots[0],
            BleFirmwareSlot::Second => &mut self.ble_firmware_slots[1],
        };
        if chunk_end > slot_data.len() {
            return Err(Error::InvalidInput);
        }
        slot_data[chunk_offset..chunk_end].copy_from_slice(chunk);
        Ok(())
    }

    fn ble_get_metadata(&mut self) -> BleMetadata {
        self.ble_metadata
    }

    fn set_ble_metadata(&mut self, metadata: &BleMetadata) -> Result<(), Error> {
        self.ble_metadata = *metadata;
        Ok(())
    }

    fn get_securechip_type(&mut self) -> Result<SecurechipType, ()> {
        Ok(self.securechip_type)
    }

    fn get_platform(&mut self) -> Result<Platform, ()> {
        Ok(self.platform)
    }

    fn get_device_name(&mut self) -> String {
        self.device_name
            .clone()
            .unwrap_or_else(|| "My BitBox".into())
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), Error> {
        self.device_name = Some(name.into());
        Ok(())
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        self.mnemonic_passphrase_enabled
    }

    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        self.mnemonic_passphrase_enabled = enabled;
        Ok(())
    }

    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
        self.seed_birthdate = timestamp;
        Ok(())
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        self.seed_birthdate
    }

    fn is_seeded(&mut self) -> bool {
        self.is_seeded
    }

    fn is_initialized(&mut self) -> bool {
        self.initialized
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        self.initialized = true;
        Ok(())
    }

    fn get_encrypted_seed_and_hmac(
        &mut self,
    ) -> Result<(alloc::vec::Vec<u8>, PasswordStretchAlgo), ()> {
        self.encrypted_seed_and_hmac.clone().ok_or(())
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<(), ()> {
        // 96 is the max space allocated in BitBox02's memory for this.
        if data.len() > 96 {
            return Err(());
        }
        self.encrypted_seed_and_hmac = Some((data.to_vec(), password_stretch_algo));
        self.is_seeded = true;
        Ok(())
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        self.initialized = false;
        self.is_seeded = false;
        self.mnemonic_passphrase_enabled = false;
        self.seed_birthdate = 0;
        self.encrypted_seed_and_hmac = None;
        self.device_name = None;
        self.multisig_entries = Vec::new();
        Ok(())
    }

    fn get_unlock_attempts(&mut self) -> u8 {
        self.unlock_attempts
    }

    fn increment_unlock_attempts(&mut self) {
        self.unlock_attempts += 1;
    }

    fn reset_unlock_attempts(&mut self) {
        self.unlock_attempts = 0;
    }

    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        if self.salt_root.iter().all(|&b| b == 0xff) {
            Err(())
        } else {
            Ok(zeroize::Zeroizing::new(self.salt_root.to_vec()))
        }
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        match (
            self.attestation_device_pubkey,
            self.attestation_certificate,
            self.attestation_root_pubkey_identifier,
        ) {
            (Some(pubkey), Some(certificate), Some(root_id)) => {
                *pubkey_out = pubkey;
                *certificate_out = certificate;
                *root_pubkey_identifier_out = root_id;
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        self.attestation_bootloader_hash
    }

    fn multisig_set_by_hash(&mut self, hash: &[u8; 32], name: &str) -> Result<(), Error> {
        // Validate input
        if name.is_empty() {
            return Err(Error::InvalidInput);
        }
        // Check for duplicate name with different hash
        for (existing_hash, existing_name) in &self.multisig_entries {
            if existing_name == name {
                if existing_hash != hash {
                    // Mirror bitbox02::memory multisig_set_by_hash semantics (duplicate-name / full-table),
                    // even if these branches are not currently exercised in bitbox02-rust tests.
                    return Err(Error::DuplicateName);
                }
                // same name, same hash (already stored)
                return Ok(());
            }
        }
        // Try to find existing entry with same hash
        if let Some((_, existing_name)) = self
            .multisig_entries
            .iter_mut()
            .find(|(existing_hash, _)| existing_hash == hash)
        {
            // rename: same hash, new name
            *existing_name = String::from(name);
            return Ok(());
        }
        if self.multisig_entries.len() >= MULTISIG_LIMIT {
            // See comment above about mirroring bitbox02::memory semantics.
            return Err(Error::Full);
        }
        // Insert new entry
        self.multisig_entries.push((*hash, String::from(name)));
        Ok(())
    }

    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String> {
        self.multisig_entries
            .iter()
            .find(|(existing_hash, _)| existing_hash == hash)
            .map(|(_, name)| name.clone())
    }
}
