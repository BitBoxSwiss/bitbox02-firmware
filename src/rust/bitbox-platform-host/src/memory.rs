// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

use bitbox_hal::filesystem::{
    DataPartition, Error as FsError, MountPolicy, Result as FsResult, Volume,
};
use bitbox_hal::memory::{
    BleFirmwareSlot, BleMetadata, Error, OptigaConfigVersion, PasswordStretchAlgo, Platform,
    SecurechipType,
};

pub struct FakeMemory {
    ble_enabled: bool,
    ble_metadata: BleMetadata,
    ble_firmware_slots: [Vec<u8>; 2],
    active_ble_firmware_version: String,
    securechip_type: SecurechipType,
    optiga_config_version: OptigaConfigVersion,
    platform: Platform,
    initialized: bool,
    is_seeded: bool,
    mnemonic_passphrase_enabled: bool,
    seed_birthdate: u32,
    encrypted_seed_and_hmac: Option<(Vec<u8>, PasswordStretchAlgo)>,
    noise_static_private_key_generation: u8,
    noise_static_private_key: [u8; 32],
    noise_remote_static_pubkeys: Vec<[u8; 32]>,
    device_name: Option<String>,
    salt_root: [u8; 32],
    attestation_device_pubkey: Option<[u8; 64]>,
    attestation_certificate: Option<[u8; 64]>,
    attestation_root_pubkey_identifier: Option<[u8; 32]>,
    attestation_bootloader_hash: [u8; 32],
    multisig_entries: Vec<([u8; 32], String)>,
    filesystem_formatted: [bool; 2],
    filesystem_files: [Vec<(String, Vec<u8>)>; 2],
}

// Same as MEMORY_MULTISIG_NUM_ENTRIES in memory.h.
const MULTISIG_LIMIT: usize = 25;
const NOISE_REMOTE_STATIC_PUBKEYS_LIMIT: usize = 5;

fn make_noise_static_private_key(generation: u8) -> [u8; 32] {
    let mut key = [generation.wrapping_add(1); 32];
    key[0] &= 248;
    key[31] &= 127;
    key[31] |= 64;
    key
}

impl FakeMemory {
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
                vec![0xff; bitbox_hal::memory::BLE_FIRMWARE_MAX_SIZE],
                vec![0xff; bitbox_hal::memory::BLE_FIRMWARE_MAX_SIZE],
            ],
            active_ble_firmware_version: "0.0.0".into(),
            securechip_type: SecurechipType::Optiga,
            optiga_config_version: OptigaConfigVersion::V0,
            platform: Platform::BitBox02,
            initialized: false,
            is_seeded: false,
            mnemonic_passphrase_enabled: false,
            seed_birthdate: 0,
            encrypted_seed_and_hmac: None,
            noise_static_private_key_generation: 0,
            noise_static_private_key: make_noise_static_private_key(0),
            noise_remote_static_pubkeys: Vec::new(),
            device_name: None,
            salt_root: *b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            attestation_device_pubkey: None,
            attestation_certificate: None,
            attestation_root_pubkey_identifier: None,
            attestation_bootloader_hash: [0; 32],
            multisig_entries: Vec::new(),
            filesystem_formatted: [false; 2],
            filesystem_files: [Vec::new(), Vec::new()],
        }
    }

    pub fn set_securechip_type(&mut self, securechip_type: SecurechipType) {
        self.securechip_type = securechip_type;
    }

    pub fn set_platform(&mut self, platform: Platform) {
        self.platform = platform;
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

fn partition_index(partition: DataPartition) -> usize {
    match partition {
        DataPartition::Vendor => 0,
        DataPartition::User => 1,
    }
}

fn validate_path(path: &str) -> FsResult<()> {
    if path.is_empty() || !path.is_ascii() || path.as_bytes().contains(&0) {
        return Err(FsError::InvalidInput);
    }
    Ok(())
}

struct FakeVolume<'a> {
    files: &'a mut Vec<(String, Vec<u8>)>,
}

impl FakeVolume<'_> {
    fn file_index(&self, path: &str) -> Option<usize> {
        self.files
            .iter()
            .position(|(stored_path, _)| stored_path == path)
    }
}

impl Volume for FakeVolume<'_> {
    fn read_file(&mut self, path: &str, out: &mut Vec<u8>) -> FsResult<()> {
        validate_path(path)?;
        let Some((_, data)) = self
            .files
            .iter()
            .find(|(stored_path, _)| stored_path == path)
        else {
            return Err(FsError::NoSuchEntry);
        };
        out.clear();
        out.extend_from_slice(data);
        Ok(())
    }

    fn write_file(&mut self, path: &str, data: &[u8]) -> FsResult<()> {
        validate_path(path)?;
        if let Some(index) = self.file_index(path) {
            self.files[index].1.clear();
            self.files[index].1.extend_from_slice(data);
        } else {
            self.files.push((String::from(path), data.to_vec()));
        }
        Ok(())
    }

    fn remove_file(&mut self, path: &str) -> FsResult<()> {
        validate_path(path)?;
        let Some(index) = self.file_index(path) else {
            return Err(FsError::NoSuchEntry);
        };
        self.files.remove(index);
        Ok(())
    }

    fn rename(&mut self, from: &str, to: &str) -> FsResult<()> {
        validate_path(from)?;
        validate_path(to)?;
        if self.file_index(to).is_some() {
            return Err(FsError::AlreadyExists);
        }
        let Some(index) = self.file_index(from) else {
            return Err(FsError::NoSuchEntry);
        };
        self.files[index].0 = String::from(to);
        Ok(())
    }

    fn exists(&mut self, path: &str) -> FsResult<bool> {
        validate_path(path)?;
        Ok(self.file_index(path).is_some())
    }
}

impl bitbox_hal::filesystem::Filesystem for FakeMemory {
    fn with_volume<R, F>(
        &mut self,
        partition: DataPartition,
        policy: MountPolicy,
        f: F,
    ) -> FsResult<R>
    where
        F: FnOnce(&mut dyn Volume) -> FsResult<R>,
    {
        let index = partition_index(partition);
        if !self.filesystem_formatted[index] {
            match policy {
                MountPolicy::MountOnly => return Err(FsError::Corrupt),
                MountPolicy::FormatIfEmpty => self.filesystem_formatted[index] = true,
            }
        }
        let mut volume = FakeVolume {
            files: &mut self.filesystem_files[index],
        };
        f(&mut volume)
    }

    fn format_volume(&mut self, partition: DataPartition) -> FsResult<()> {
        let index = partition_index(partition);
        self.filesystem_files[index].clear();
        self.filesystem_formatted[index] = true;
        Ok(())
    }
}

impl bitbox_hal::Memory for FakeMemory {
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

    fn get_optiga_config_version(&mut self) -> Result<OptigaConfigVersion, ()> {
        Ok(self.optiga_config_version)
    }

    fn set_optiga_config_version(&mut self, version: OptigaConfigVersion) -> Result<(), ()> {
        self.optiga_config_version = version;
        Ok(())
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

    fn get_encrypted_seed_and_hmac(&mut self) -> Result<(Vec<u8>, PasswordStretchAlgo), ()> {
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
        self.noise_static_private_key_generation =
            self.noise_static_private_key_generation.wrapping_add(1);
        self.noise_static_private_key =
            make_noise_static_private_key(self.noise_static_private_key_generation);
        self.noise_remote_static_pubkeys = Vec::new();
        self.device_name = None;
        self.multisig_entries = Vec::new();
        Ok(())
    }

    fn get_noise_static_private_key(&mut self) -> Result<zeroize::Zeroizing<[u8; 32]>, ()> {
        Ok(zeroize::Zeroizing::new(self.noise_static_private_key))
    }

    fn check_noise_remote_static_pubkey(&mut self, pubkey: &[u8; 32]) -> bool {
        self.noise_remote_static_pubkeys
            .iter()
            .any(|stored_pubkey| stored_pubkey == pubkey)
    }

    fn add_noise_remote_static_pubkey(&mut self, pubkey: &[u8; 32]) -> Result<(), ()> {
        if self.check_noise_remote_static_pubkey(pubkey) {
            return Ok(());
        }
        if self.noise_remote_static_pubkeys.len() == NOISE_REMOTE_STATIC_PUBKEYS_LIMIT {
            self.noise_remote_static_pubkeys.remove(0);
        }
        self.noise_remote_static_pubkeys.push(*pubkey);
        Ok(())
    }

    fn get_io_protection_key(&mut self, _out: &mut [u8; 32]) {
        panic!("unused")
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

#[cfg(test)]
mod tests {
    use super::*;
    use bitbox_hal::filesystem::Filesystem;

    #[test]
    fn test_filesystem_format_if_empty() {
        let mut memory = FakeMemory::new();
        assert_eq!(
            memory.with_volume(DataPartition::User, MountPolicy::MountOnly, |_| Ok(())),
            Err(FsError::Corrupt)
        );

        memory
            .with_volume(DataPartition::User, MountPolicy::FormatIfEmpty, |volume| {
                volume.write_file("settings.bin", b"user")?;
                Ok(())
            })
            .unwrap();

        let mut out = Vec::new();
        memory
            .with_volume(DataPartition::User, MountPolicy::MountOnly, |volume| {
                volume.read_file("settings.bin", &mut out)
            })
            .unwrap();
        assert_eq!(out, b"user");
    }

    #[test]
    fn test_filesystem_partitions_are_isolated() {
        let mut memory = FakeMemory::new();
        memory.format_volume(DataPartition::Vendor).unwrap();
        memory.format_volume(DataPartition::User).unwrap();

        memory
            .with_volume(DataPartition::Vendor, MountPolicy::MountOnly, |volume| {
                volume.write_file("data.bin", b"vendor")
            })
            .unwrap();
        memory
            .with_volume(DataPartition::User, MountPolicy::MountOnly, |volume| {
                volume.write_file("data.bin", b"user")
            })
            .unwrap();

        let mut out = Vec::new();
        memory
            .with_volume(DataPartition::Vendor, MountPolicy::MountOnly, |volume| {
                volume.read_file("data.bin", &mut out)
            })
            .unwrap();
        assert_eq!(out, b"vendor");

        memory
            .with_volume(DataPartition::User, MountPolicy::MountOnly, |volume| {
                volume.read_file("data.bin", &mut out)
            })
            .unwrap();
        assert_eq!(out, b"user");
    }

    #[test]
    fn test_filesystem_rename_and_remove() {
        let mut memory = FakeMemory::new();
        memory
            .with_volume(DataPartition::User, MountPolicy::FormatIfEmpty, |volume| {
                volume.write_file("old.bin", b"data")?;
                volume.rename("old.bin", "new.bin")?;
                assert!(!volume.exists("old.bin")?);
                assert!(volume.exists("new.bin")?);
                volume.remove_file("new.bin")?;
                assert!(!volume.exists("new.bin")?);
                Ok(())
            })
            .unwrap();
    }
}
