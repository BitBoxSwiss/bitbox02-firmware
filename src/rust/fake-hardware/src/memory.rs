// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use bitbox_hal::memory::{
    BLE_FIRMWARE_MAX_SIZE, BleFirmwareSlot, BleMetadata, DEVICE_NAME_MAX_LEN, Error,
    MULTISIG_NAME_MAX_LEN, Memory, OptigaConfigVersion, PasswordStretchAlgo, Platform,
    SecurechipType,
};
use std::sync::{LazyLock, Mutex};
use zeroize::Zeroizing;

const MULTISIG_LIMIT: usize = 25;
const ENCRYPTED_SEED_AND_HMAC_MAX_LEN: usize = 96;
const DEFAULT_ACTIVE_BLE_FIRMWARE_VERSION: &str = "0.0.0";
const DEFAULT_DEVICE_NAME: &str = "My BitBox";
const DEFAULT_IO_PROTECTION_KEY: [u8; 32] = [0x42; 32];
const DEFAULT_SALT_ROOT: [u8; 32] = *b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
const BB02_SALT_ROOT: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
];
const BB02_ALLOWED_FIRMWARE_HASH: [u8; 32] = [
    0x1e, 0x4a, 0xa8, 0x36, 0x4e, 0x93, 0x5c, 0x07, 0x85, 0xe4, 0xf8, 0x91, 0x20, 0x83, 0x07, 0xd8,
    0x32, 0xf7, 0x88, 0x17, 0x2e, 0x4b, 0xf6, 0x16, 0x21, 0xde, 0x6d, 0xf9, 0xec, 0x3c, 0x21, 0x5f,
];

#[derive(Clone)]
struct State {
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
    device_name: Option<String>,
    unlock_attempts: u8,
    salt_root: [u8; 32],
    attestation_device_pubkey: Option<[u8; 64]>,
    attestation_certificate: Option<[u8; 64]>,
    attestation_root_pubkey_identifier: Option<[u8; 32]>,
    attestation_bootloader_hash: [u8; 32],
    multisig_entries: Vec<([u8; 32], String)>,
}

impl State {
    fn new() -> Self {
        Self {
            ble_enabled: true,
            ble_metadata: BleMetadata {
                allowed_firmware_hash: [0; 32],
                active_index: 0,
                firmware_sizes: [0; 2],
                firmware_checksums: [0; 2],
            },
            ble_firmware_slots: [
                vec![0xff; BLE_FIRMWARE_MAX_SIZE],
                vec![0xff; BLE_FIRMWARE_MAX_SIZE],
            ],
            active_ble_firmware_version: DEFAULT_ACTIVE_BLE_FIRMWARE_VERSION.into(),
            securechip_type: SecurechipType::Optiga,
            optiga_config_version: OptigaConfigVersion::V0,
            platform: Platform::BitBox02,
            initialized: false,
            is_seeded: false,
            mnemonic_passphrase_enabled: false,
            seed_birthdate: 0,
            encrypted_seed_and_hmac: None,
            device_name: None,
            unlock_attempts: 0,
            salt_root: DEFAULT_SALT_ROOT,
            attestation_device_pubkey: None,
            attestation_certificate: None,
            attestation_root_pubkey_identifier: None,
            attestation_bootloader_hash: [0; 32],
            multisig_entries: Vec::new(),
        }
    }

    fn init_bitbox02_simulator(&mut self) {
        *self = Self::new();
        self.platform = Platform::BitBox02Plus;
        self.securechip_type = SecurechipType::Optiga;
        self.ble_enabled = false;
        self.ble_metadata.allowed_firmware_hash = BB02_ALLOWED_FIRMWARE_HASH;
        self.salt_root = BB02_SALT_ROOT;
    }

    fn init_bitbox03_simulator(&mut self) {
        *self = Self::new();
        self.platform = Platform::BitBox02;
        self.securechip_type = SecurechipType::Optiga;
        self.ble_enabled = false;
    }

    fn reset_hww(&mut self) {
        self.initialized = false;
        self.is_seeded = false;
        self.mnemonic_passphrase_enabled = false;
        self.seed_birthdate = 0;
        self.encrypted_seed_and_hmac = None;
        self.device_name = None;
        self.multisig_entries = Vec::new();
    }
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::new()));

fn with_state<R>(callback: impl FnOnce(&mut State) -> R) -> R {
    callback(&mut STATE.lock().unwrap())
}

fn is_valid_name(name: &str, max_len: usize) -> bool {
    if name.is_empty() || name.len() > max_len || !name.is_ascii() {
        return false;
    }
    let bytes = name.as_bytes();
    if bytes[0] == b' ' || bytes[bytes.len() - 1] == b' ' {
        return false;
    }
    bytes.iter().all(|b| matches!(*b, b' '..=b'~'))
}

fn normalize_device_name(name: &str) -> Result<String, Error> {
    if !name.is_ascii() {
        return Err(Error::InvalidInput);
    }
    let truncated = if name.len() > DEVICE_NAME_MAX_LEN {
        &name[..DEVICE_NAME_MAX_LEN]
    } else {
        name
    };
    if !is_valid_name(truncated, DEVICE_NAME_MAX_LEN) {
        return Err(Error::InvalidInput);
    }
    Ok(truncated.into())
}

fn is_invalid_multisig_hash(hash: &[u8; 32]) -> bool {
    hash.iter().all(|byte| *byte == 0xff)
}

#[derive(Copy, Clone, Default)]
pub struct FakeMemory;

impl FakeMemory {
    pub fn new() -> Self {
        reset();
        Self
    }

    pub fn set_securechip_type(&mut self, securechip_type: SecurechipType) {
        with_state(|state| state.securechip_type = securechip_type);
    }

    pub fn set_platform(&mut self, platform: Platform) {
        with_state(|state| state.platform = platform);
    }

    pub fn set_unlock_attempts_for_testing(&mut self, attempts: u8) {
        with_state(|state| state.unlock_attempts = attempts);
    }

    pub fn set_salt_root(&mut self, salt_root: &[u8; 32]) {
        with_state(|state| state.salt_root = *salt_root);
    }

    pub fn set_attestation_certificate(
        &mut self,
        pubkey: &[u8; 64],
        certificate: &[u8; 64],
        root_pubkey_identifier: &[u8; 32],
    ) {
        with_state(|state| {
            state.attestation_device_pubkey = Some(*pubkey);
            state.attestation_certificate = Some(*certificate);
            state.attestation_root_pubkey_identifier = Some(*root_pubkey_identifier);
        });
    }

    pub fn set_attestation_bootloader_hash(&mut self, hash: &[u8; 32]) {
        with_state(|state| state.attestation_bootloader_hash = *hash);
    }

    pub fn ble_firmware_slot_data(&self, slot: BleFirmwareSlot) -> Vec<u8> {
        STATE.lock().unwrap().ble_firmware_slots[match slot {
            BleFirmwareSlot::First => 0,
            BleFirmwareSlot::Second => 1,
        }]
        .clone()
    }
}

pub fn reset() {
    with_state(|state| *state = State::new());
}

pub fn init_bitbox02_simulator() {
    with_state(State::init_bitbox02_simulator);
}

pub fn init_bitbox03_simulator() {
    with_state(State::init_bitbox03_simulator);
}

impl Memory for FakeMemory {
    const BLE_FW_FLASH_CHUNK_SIZE: u32 = 4096;

    fn ble_enabled(&mut self) -> bool {
        with_state(|state| state.ble_enabled)
    }

    fn ble_enable(&mut self, enable: bool) -> Result<(), ()> {
        with_state(|state| state.ble_enabled = enable);
        Ok(())
    }

    fn get_active_ble_firmware_version(&mut self) -> Result<String, Error> {
        Ok(with_state(|state| {
            state.active_ble_firmware_version.clone()
        }))
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

        with_state(|state| {
            let chunk_offset = (chunk_index as usize)
                .checked_mul(Self::BLE_FW_FLASH_CHUNK_SIZE as usize)
                .ok_or(Error::InvalidInput)?;
            let chunk_end = chunk_offset
                .checked_add(chunk.len())
                .ok_or(Error::InvalidInput)?;

            let slot_data = match slot {
                BleFirmwareSlot::First => &mut state.ble_firmware_slots[0],
                BleFirmwareSlot::Second => &mut state.ble_firmware_slots[1],
            };
            if chunk_end > slot_data.len() {
                return Err(Error::InvalidInput);
            }
            slot_data[chunk_offset..chunk_end].copy_from_slice(chunk);
            Ok(())
        })
    }

    fn ble_get_metadata(&mut self) -> BleMetadata {
        with_state(|state| state.ble_metadata)
    }

    fn set_ble_metadata(&mut self, metadata: &BleMetadata) -> Result<(), Error> {
        with_state(|state| state.ble_metadata = *metadata);
        Ok(())
    }

    fn get_securechip_type(&mut self) -> Result<SecurechipType, ()> {
        Ok(with_state(|state| state.securechip_type))
    }

    fn get_optiga_config_version(&mut self) -> Result<OptigaConfigVersion, ()> {
        Ok(with_state(|state| state.optiga_config_version))
    }

    fn set_optiga_config_version(&mut self, version: OptigaConfigVersion) -> Result<(), ()> {
        with_state(|state| state.optiga_config_version = version);
        Ok(())
    }

    fn get_platform(&mut self) -> Result<Platform, ()> {
        Ok(with_state(|state| state.platform))
    }

    fn get_device_name(&mut self) -> String {
        with_state(|state| {
            state
                .device_name
                .clone()
                .unwrap_or_else(|| DEFAULT_DEVICE_NAME.into())
        })
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), Error> {
        let name = normalize_device_name(name)?;
        with_state(|state| state.device_name = Some(name));
        Ok(())
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        with_state(|state| state.mnemonic_passphrase_enabled)
    }

    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        with_state(|state| state.mnemonic_passphrase_enabled = enabled);
        Ok(())
    }

    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
        with_state(|state| state.seed_birthdate = timestamp);
        Ok(())
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        with_state(|state| state.seed_birthdate)
    }

    fn is_seeded(&mut self) -> bool {
        with_state(|state| state.is_seeded)
    }

    fn is_initialized(&mut self) -> bool {
        with_state(|state| state.initialized)
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        with_state(|state| state.initialized = true);
        Ok(())
    }

    fn get_encrypted_seed_and_hmac(&mut self) -> Result<(Vec<u8>, PasswordStretchAlgo), ()> {
        with_state(|state| state.encrypted_seed_and_hmac.clone().ok_or(()))
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<(), ()> {
        if data.len() > ENCRYPTED_SEED_AND_HMAC_MAX_LEN {
            return Err(());
        }
        with_state(|state| {
            state.encrypted_seed_and_hmac = Some((data.to_vec(), password_stretch_algo));
            state.is_seeded = true;
        });
        Ok(())
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        with_state(State::reset_hww);
        Ok(())
    }

    fn get_unlock_attempts(&mut self) -> u8 {
        with_state(|state| state.unlock_attempts)
    }

    fn increment_unlock_attempts(&mut self) {
        with_state(|state| state.unlock_attempts = state.unlock_attempts.saturating_add(1));
    }

    fn reset_unlock_attempts(&mut self) {
        with_state(|state| state.unlock_attempts = 0);
    }

    fn get_io_protection_key(&mut self, out: &mut [u8; 32]) {
        *out = DEFAULT_IO_PROTECTION_KEY;
    }

    fn get_salt_root(&mut self) -> Result<Zeroizing<Vec<u8>>, ()> {
        with_state(|state| {
            if state.salt_root.iter().all(|byte| *byte == 0xff) {
                Err(())
            } else {
                Ok(Zeroizing::new(state.salt_root.to_vec()))
            }
        })
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        with_state(|state| {
            match (
                state.attestation_device_pubkey,
                state.attestation_certificate,
                state.attestation_root_pubkey_identifier,
            ) {
                (Some(pubkey), Some(certificate), Some(root_id)) => {
                    *pubkey_out = pubkey;
                    *certificate_out = certificate;
                    *root_pubkey_identifier_out = root_id;
                    Ok(())
                }
                _ => Err(()),
            }
        })
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        with_state(|state| state.attestation_bootloader_hash)
    }

    fn multisig_set_by_hash(&mut self, hash: &[u8; 32], name: &str) -> Result<(), Error> {
        if is_invalid_multisig_hash(hash) || !is_valid_name(name, MULTISIG_NAME_MAX_LEN) {
            return Err(Error::InvalidInput);
        }

        with_state(|state| {
            for (existing_hash, existing_name) in &state.multisig_entries {
                if existing_name == name {
                    if existing_hash != hash {
                        return Err(Error::DuplicateName);
                    }
                    return Ok(());
                }
            }

            if let Some((_, existing_name)) = state
                .multisig_entries
                .iter_mut()
                .find(|(existing_hash, _)| existing_hash == hash)
            {
                *existing_name = name.into();
                return Ok(());
            }

            if state.multisig_entries.len() >= MULTISIG_LIMIT {
                return Err(Error::Full);
            }

            state.multisig_entries.push((*hash, name.into()));
            Ok(())
        })
    }

    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String> {
        STATE
            .lock()
            .unwrap()
            .multisig_entries
            .iter()
            .find(|(existing_hash, _)| existing_hash == hash)
            .map(|entry| entry.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use std::sync::{LazyLock, Mutex};

    static TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    fn with_reset<T>(callback: impl FnOnce(&mut FakeMemory) -> T) -> T {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        let mut memory = FakeMemory;
        callback(&mut memory)
    }

    #[test]
    fn test_shared_state() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        let mut first = FakeMemory;
        let mut second = FakeMemory;

        assert!(!first.is_initialized());
        first.set_initialized().unwrap();
        assert!(second.is_initialized());
    }

    #[test]
    fn test_init_bitbox02_simulator() {
        let _guard = TEST_LOCK.lock().unwrap();
        init_bitbox02_simulator();
        let mut memory = FakeMemory;

        assert_eq!(memory.get_platform(), Ok(Platform::BitBox02Plus));
        assert_eq!(memory.get_securechip_type(), Ok(SecurechipType::Optiga));
        assert!(!memory.ble_enabled());
        assert_eq!(
            memory.ble_get_metadata().allowed_firmware_hash,
            BB02_ALLOWED_FIRMWARE_HASH
        );
        assert_eq!(memory.get_salt_root().unwrap().as_slice(), &BB02_SALT_ROOT);
    }

    #[test]
    fn test_init_bitbox03_simulator() {
        let _guard = TEST_LOCK.lock().unwrap();
        init_bitbox03_simulator();
        let mut memory = FakeMemory;

        assert_eq!(memory.get_platform(), Ok(Platform::BitBox02));
        assert_eq!(memory.get_securechip_type(), Ok(SecurechipType::Optiga));
        assert!(!memory.ble_enabled());
        assert_eq!(
            memory.get_active_ble_firmware_version().unwrap(),
            DEFAULT_ACTIVE_BLE_FIRMWARE_VERSION
        );
    }

    #[test]
    fn test_set_device_name() {
        with_reset(|memory| {
            assert_eq!(memory.get_device_name(), DEFAULT_DEVICE_NAME);

            let max_len_name = "DeviceName_ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxy";
            assert_eq!(max_len_name.len(), DEVICE_NAME_MAX_LEN);
            memory.set_device_name(max_len_name).unwrap();
            assert_eq!(memory.get_device_name(), max_len_name);

            let long_name = format!("{max_len_name}foobar");
            memory.set_device_name(&long_name).unwrap();
            assert_eq!(memory.get_device_name(), max_len_name);

            for invalid_name in ["", " name", "name ", "foo\nbar", "Ä", "漢字"] {
                assert_eq!(
                    memory.set_device_name(invalid_name),
                    Err(Error::InvalidInput)
                );
            }
        });
    }

    #[test]
    fn test_encrypted_seed_and_hmac() {
        with_reset(|memory| {
            let data = [0x42; ENCRYPTED_SEED_AND_HMAC_MAX_LEN];
            memory
                .set_encrypted_seed_and_hmac(&data, PasswordStretchAlgo::V1)
                .unwrap();
            assert!(memory.is_seeded());
            assert_eq!(
                memory.get_encrypted_seed_and_hmac().unwrap(),
                (data.to_vec(), PasswordStretchAlgo::V1)
            );
            assert_eq!(
                memory.set_encrypted_seed_and_hmac(
                    &[0u8; ENCRYPTED_SEED_AND_HMAC_MAX_LEN + 1],
                    PasswordStretchAlgo::V0
                ),
                Err(())
            );
        });
    }

    #[test]
    fn test_reset_hww() {
        with_reset(|memory| {
            memory.set_initialized().unwrap();
            memory.set_mnemonic_passphrase_enabled(true).unwrap();
            memory.set_seed_birthdate(123).unwrap();
            memory
                .set_encrypted_seed_and_hmac(&[1, 2, 3], PasswordStretchAlgo::V0)
                .unwrap();
            memory.set_device_name("Custom name").unwrap();
            memory
                .multisig_set_by_hash(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "name1")
                .unwrap();

            memory.reset_hww().unwrap();

            assert!(!memory.is_initialized());
            assert!(!memory.is_seeded());
            assert!(!memory.is_mnemonic_passphrase_enabled());
            assert_eq!(memory.get_seed_birthdate(), 0);
            assert!(memory.get_encrypted_seed_and_hmac().is_err());
            assert_eq!(memory.get_device_name(), DEFAULT_DEVICE_NAME);
            assert!(
                memory
                    .multisig_get_by_hash(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
                    .is_none()
            );
        });
    }

    #[test]
    fn test_unlock_attempts() {
        with_reset(|memory| {
            assert_eq!(memory.get_unlock_attempts(), 0);
            memory.increment_unlock_attempts();
            memory.increment_unlock_attempts();
            assert_eq!(memory.get_unlock_attempts(), 2);
            memory.reset_unlock_attempts();
            assert_eq!(memory.get_unlock_attempts(), 0);
        });
    }

    #[test]
    fn test_salt_root() {
        with_reset(|memory| {
            assert_eq!(
                memory.get_salt_root().unwrap().as_slice(),
                &DEFAULT_SALT_ROOT
            );
            with_state(|state| state.salt_root = [0xff; 32]);
            assert!(memory.get_salt_root().is_err());
        });
    }

    #[test]
    fn test_get_io_protection_key() {
        with_reset(|memory| {
            let mut key = [0; 32];
            memory.get_io_protection_key(&mut key);
            assert_eq!(key, DEFAULT_IO_PROTECTION_KEY);
        });
    }

    #[test]
    fn test_attestation() {
        with_reset(|memory| {
            let pubkey = [1u8; 64];
            let certificate = [2u8; 64];
            let root_id = [3u8; 32];
            with_state(|state| {
                state.attestation_device_pubkey = Some(pubkey);
                state.attestation_certificate = Some(certificate);
                state.attestation_root_pubkey_identifier = Some(root_id);
                state.attestation_bootloader_hash = [4u8; 32];
            });

            let mut pubkey_out = [0u8; 64];
            let mut certificate_out = [0u8; 64];
            let mut root_id_out = [0u8; 32];
            memory
                .get_attestation_pubkey_and_certificate(
                    &mut pubkey_out,
                    &mut certificate_out,
                    &mut root_id_out,
                )
                .unwrap();

            assert_eq!(pubkey_out, pubkey);
            assert_eq!(certificate_out, certificate);
            assert_eq!(root_id_out, root_id);
            assert_eq!(memory.get_attestation_bootloader_hash(), [4u8; 32]);
        });
    }

    #[test]
    fn test_multisig() {
        with_reset(|memory| {
            let hash1 = *b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
            let hash2 = *b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

            assert!(memory.multisig_get_by_hash(&hash1).is_none());
            memory.multisig_set_by_hash(&hash1, "name1").unwrap();
            memory
                .multisig_set_by_hash(&hash1, "name 1 renamed")
                .unwrap();
            assert_eq!(
                memory.multisig_get_by_hash(&hash1).as_deref(),
                Some("name 1 renamed")
            );
            assert_eq!(
                memory.multisig_set_by_hash(&hash2, "name 1 renamed"),
                Err(Error::DuplicateName)
            );
            assert_eq!(
                memory.multisig_set_by_hash(&[0xff; 32], "name"),
                Err(Error::InvalidInput)
            );
            assert_eq!(
                memory.multisig_set_by_hash(&hash2, ""),
                Err(Error::InvalidInput)
            );
        });
    }

    #[test]
    fn test_multisig_full() {
        with_reset(|memory| {
            for i in 0..MULTISIG_LIMIT {
                let mut hash = [0u8; 32];
                hash.fill((i + i) as u8);
                let name = format!("name{i}");
                memory.multisig_set_by_hash(&hash, &name).unwrap();
            }

            let hash = [0x7eu8; 32];
            assert_eq!(
                memory.multisig_set_by_hash(&hash, "overflow"),
                Err(Error::Full)
            );
        });
    }

    #[test]
    fn test_ble_flash_chunk() {
        with_reset(|memory| {
            let chunk = [0x55; 32];
            memory
                .ble_firmware_flash_chunk(BleFirmwareSlot::Second, 1, &chunk)
                .unwrap();

            with_state(|state| {
                let offset = FakeMemory::BLE_FW_FLASH_CHUNK_SIZE as usize;
                assert_eq!(
                    &state.ble_firmware_slots[1][offset..offset + chunk.len()],
                    &chunk
                );
            });

            assert_eq!(
                memory.ble_firmware_flash_chunk(
                    BleFirmwareSlot::First,
                    0,
                    &[0u8; FakeMemory::BLE_FW_FLASH_CHUNK_SIZE as usize + 1]
                ),
                Err(Error::InvalidInput)
            );
        });
    }
}
