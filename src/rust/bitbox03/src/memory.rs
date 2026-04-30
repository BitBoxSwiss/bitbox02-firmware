use alloc::string::String;
use alloc::vec::Vec;
use bitbox_boot_utils::{FLASH_PAGE_SIZE, IMMUTABLE_PAGE_ADDR};
use bitbox_hal as hal;
use bitbox_hal::memory::{
    BleFirmwareSlot, BleMetadata, Error, OptigaConfigVersion, PasswordStretchAlgo, Platform,
    SecurechipType,
};

use bitbox03_boot_utils::ImmutablePage as ImmutableState;

const DEFAULT_DEVICE_NAME: &str = "My BitBox";
const DEFAULT_BLE_FIRMWARE_VERSION: &str = "0.0.0";
const ENCRYPTED_SEED_MAX_LEN: usize = 96;
const NOISE_REMOTE_STATIC_PUBKEYS_LIMIT: usize = 5;
const MULTISIG_LIMIT: usize = 25;
const ACTIVE_BLE_FIRMWARE_VERSION_MAX_LEN: usize = 16;

const FLASH_BASE_NS: usize = 0x0800_0000;
const BLE_SLOT_SIZE: usize = hal::memory::BLE_FIRMWARE_MAX_SIZE;
const CONFIG_PAGE_ADDR: usize = FLASH_BASE_NS + 328 * 1024;
const BLE_SLOT_1_ADDR: usize = CONFIG_PAGE_ADDR + FLASH_PAGE_SIZE;
const BLE_SLOT_2_ADDR: usize = BLE_SLOT_1_ADDR + BLE_SLOT_SIZE;

const MUTABLE_STORAGE_MAGIC: u32 = 0x3342_4253;
const MUTABLE_STORAGE_VERSION: u32 = 3;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum StoredPasswordStretchAlgo {
    V0 = 0,
    V1 = 1,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct StoredMultisigEntry {
    in_use: u8,
    name_len: u8,
    _reserved: [u8; 2],
    hash: [u8; 32],
    name: [u8; hal::memory::MULTISIG_NAME_MAX_LEN],
}

impl StoredMultisigEntry {
    const fn empty() -> Self {
        Self {
            in_use: 0,
            name_len: 0,
            _reserved: [0; 2],
            hash: [0; 32],
            name: [0; hal::memory::MULTISIG_NAME_MAX_LEN],
        }
    }

    fn name(&self) -> Option<String> {
        if self.in_use == 0 {
            return None;
        }
        decode_string(&self.name, self.name_len)
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed(1))]
struct MutableState {
    magic: u32,
    version: u32,
    checksum: u32,
    ble_enabled: u8,
    initialized: u8,
    is_seeded: u8,
    mnemonic_passphrase_enabled: u8,
    active_ble_firmware_version_len: u8,
    device_name_len: u8,
    encrypted_seed_len: u8,
    encrypted_seed_password_stretch_algo: u8,
    noise_remote_static_pubkeys_len: u8,
    _reserved0: [u8; 4],
    seed_birthdate: u32,
    ble_metadata: BleMetadata,
    active_ble_firmware_version: [u8; ACTIVE_BLE_FIRMWARE_VERSION_MAX_LEN],
    device_name: [u8; hal::memory::DEVICE_NAME_MAX_LEN],
    encrypted_seed_and_hmac: [u8; ENCRYPTED_SEED_MAX_LEN],
    noise_static_private_key_generation: u8,
    _reserved1: [u8; 3],
    noise_static_private_key: [u8; 32],
    noise_remote_static_pubkeys: [[u8; 32]; NOISE_REMOTE_STATIC_PUBKEYS_LIMIT],
    salt_root: [u8; 32],
    multisig_entries: [StoredMultisigEntry; MULTISIG_LIMIT],
}

const _: [(); FLASH_PAGE_SIZE - core::mem::size_of::<MutableState>()] =
    [(); FLASH_PAGE_SIZE - core::mem::size_of::<MutableState>()];

impl MutableState {
    fn default_state() -> Self {
        let noise_static_private_key_generation = 0;
        Self {
            magic: MUTABLE_STORAGE_MAGIC,
            version: MUTABLE_STORAGE_VERSION,
            checksum: 0,
            ble_enabled: 1,
            initialized: 0,
            is_seeded: 0,
            mnemonic_passphrase_enabled: 0,
            active_ble_firmware_version_len: DEFAULT_BLE_FIRMWARE_VERSION.len() as u8,
            device_name_len: 0,
            encrypted_seed_len: 0,
            encrypted_seed_password_stretch_algo: StoredPasswordStretchAlgo::V1 as u8,
            noise_remote_static_pubkeys_len: 0,
            _reserved0: [0; 4],
            seed_birthdate: 0,
            ble_metadata: BleMetadata {
                allowed_firmware_hash: [0; 32],
                active_index: 0,
                firmware_sizes: [0; 2],
                firmware_checksums: [0; 2],
            },
            active_ble_firmware_version: array_from_str(DEFAULT_BLE_FIRMWARE_VERSION),
            device_name: [0; hal::memory::DEVICE_NAME_MAX_LEN],
            encrypted_seed_and_hmac: [0; ENCRYPTED_SEED_MAX_LEN],
            noise_static_private_key_generation,
            _reserved1: [0; 3],
            noise_static_private_key: make_noise_static_private_key(
                noise_static_private_key_generation,
            ),
            noise_remote_static_pubkeys: [[0; 32]; NOISE_REMOTE_STATIC_PUBKEYS_LIMIT],
            salt_root: *b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            multisig_entries: [StoredMultisigEntry::empty(); MULTISIG_LIMIT],
        }
    }

    fn load() -> Self {
        let mut bytes = [0xff; core::mem::size_of::<MutableState>()];
        flash_backend::read(CONFIG_PAGE_ADDR, &mut bytes);
        let state = unsafe { core::ptr::read_unaligned(bytes.as_ptr().cast::<MutableState>()) };
        if state.is_valid() {
            state
        } else {
            Self::default_state()
        }
    }

    fn store(mut self) -> Result<(), ()> {
        self.checksum = 0;
        self.checksum = checksum_bytes(self.as_bytes());

        let mut page = [0xff; FLASH_PAGE_SIZE];
        let bytes = self.as_bytes();
        page[..bytes.len()].copy_from_slice(bytes);
        flash_backend::write_page(CONFIG_PAGE_ADDR, &page)
    }

    fn is_valid(&self) -> bool {
        self.magic == MUTABLE_STORAGE_MAGIC
            && self.version == MUTABLE_STORAGE_VERSION
            && self.checksum == checksum_bytes_with_zeroed_checksum(self)
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const MutableState).cast::<u8>(),
                core::mem::size_of::<MutableState>(),
            )
        }
    }
}

fn load_immutable_state() -> ImmutableState {
    let mut bytes = [0xff; FLASH_PAGE_SIZE];
    flash_backend::read(IMMUTABLE_PAGE_ADDR, &mut bytes);
    ImmutableState::from_page_bytes(&bytes).unwrap_or_else(|_| ImmutableState::blank())
}

fn load_immutable_state_for_update() -> Result<ImmutableState, ()> {
    let mut bytes = [0xff; FLASH_PAGE_SIZE];
    flash_backend::read(IMMUTABLE_PAGE_ADDR, &mut bytes);
    ImmutableState::from_page_bytes(&bytes)
}

#[allow(dead_code)]
fn store_immutable_state(state: ImmutableState) -> Result<(), ()> {
    flash_backend::write_page(IMMUTABLE_PAGE_ADDR, &state.to_page_bytes())
}

pub fn set_attestation_bootloader_hash(hash: &[u8; 32]) -> Result<(), ()> {
    let mut state = load_immutable_state_for_update()?;
    state.attestation_bootloader_hash = *hash;
    store_immutable_state(state)
}

pub fn get_attestation_bootloader_hash() -> Result<[u8; 32], ()> {
    Ok(load_immutable_state_for_update()?.attestation_bootloader_hash)
}

pub fn get_stored_attestation_device_pubkey() -> Result<Option<[u8; 64]>, ()> {
    let pubkey = load_immutable_state_for_update()?.attestation_device_pubkey;
    if pubkey.iter().all(|byte| *byte == 0) {
        Ok(None)
    } else {
        Ok(Some(pubkey))
    }
}

pub fn set_attestation_device_pubkey(pubkey: &[u8; 64]) -> Result<(), ()> {
    let mut state = load_immutable_state_for_update()?;
    state.attestation_present = 0;
    state.attestation_device_pubkey = *pubkey;
    state.attestation_certificate = [0; 64];
    state.attestation_root_pubkey_identifier = [0; 32];
    store_immutable_state(state)
}

pub fn set_attestation_certificate(
    pubkey: &[u8; 64],
    certificate: &[u8; 64],
    root_pubkey_identifier: &[u8; 32],
) -> Result<(), ()> {
    let mut state = load_immutable_state_for_update()?;
    if state.attestation_device_pubkey != *pubkey {
        return Err(());
    }
    state.attestation_present = 1;
    state.attestation_device_pubkey = *pubkey;
    state.attestation_certificate = *certificate;
    state.attestation_root_pubkey_identifier = *root_pubkey_identifier;
    store_immutable_state(state)
}

trait Checksummed {
    fn as_bytes(&self) -> &[u8];
    fn zero_checksum(&mut self);
}

impl Checksummed for MutableState {
    fn as_bytes(&self) -> &[u8] {
        MutableState::as_bytes(self)
    }

    fn zero_checksum(&mut self) {
        self.checksum = 0;
    }
}

fn checksum_bytes_with_zeroed_checksum<T: Checksummed + Copy>(state: &T) -> u32 {
    let mut copy = *state;
    copy.zero_checksum();
    checksum_bytes(copy.as_bytes())
}

fn checksum_bytes(bytes: &[u8]) -> u32 {
    let mut checksum = 0u32;
    for chunk in bytes.chunks(4) {
        let mut word = [0u8; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        checksum = checksum.rotate_left(5) ^ u32::from_le_bytes(word);
    }
    checksum
}

fn array_from_str<const N: usize>(value: &str) -> [u8; N] {
    let mut out = [0; N];
    out[..value.len()].copy_from_slice(value.as_bytes());
    out
}

fn decode_string(bytes: &[u8], len: u8) -> Option<String> {
    let len = len as usize;
    if len == 0 || len > bytes.len() {
        return None;
    }
    core::str::from_utf8(&bytes[..len]).ok().map(String::from)
}

fn stored_password_stretch_algo(algo: PasswordStretchAlgo) -> StoredPasswordStretchAlgo {
    match algo {
        PasswordStretchAlgo::V0 => StoredPasswordStretchAlgo::V0,
        PasswordStretchAlgo::V1 => StoredPasswordStretchAlgo::V1,
    }
}

fn hal_password_stretch_algo(algo: StoredPasswordStretchAlgo) -> PasswordStretchAlgo {
    match algo {
        StoredPasswordStretchAlgo::V0 => PasswordStretchAlgo::V0,
        StoredPasswordStretchAlgo::V1 => PasswordStretchAlgo::V1,
    }
}

fn make_noise_static_private_key(generation: u8) -> [u8; 32] {
    let mut key = [generation.wrapping_add(1); 32];
    key[0] &= 248;
    key[31] &= 127;
    key[31] |= 64;
    key
}

fn ble_slot_base(slot: BleFirmwareSlot) -> usize {
    match slot {
        BleFirmwareSlot::First => BLE_SLOT_1_ADDR,
        BleFirmwareSlot::Second => BLE_SLOT_2_ADDR,
    }
}

fn validate_name(name: &str, max_len: usize) -> bool {
    util::name::validate(name, max_len)
}

pub struct BitBox03Memory;

impl BitBox03Memory {
    pub const fn new() -> Self {
        Self
    }
}

impl hal::memory::Memory for BitBox03Memory {
    const BLE_FW_FLASH_CHUNK_SIZE: u32 = FLASH_PAGE_SIZE as u32;

    fn ble_enabled(&mut self) -> bool {
        MutableState::load().ble_enabled != 0
    }

    fn ble_enable(&mut self, enable: bool) -> Result<(), ()> {
        let mut state = MutableState::load();
        state.ble_enabled = enable as u8;
        state.store()
    }

    fn get_active_ble_firmware_version(&mut self) -> Result<String, Error> {
        let state = MutableState::load();
        Ok(decode_string(
            &state.active_ble_firmware_version,
            state.active_ble_firmware_version_len,
        )
        .unwrap_or_else(|| String::from(DEFAULT_BLE_FIRMWARE_VERSION)))
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

        let page_offset = (chunk_index as usize)
            .checked_mul(FLASH_PAGE_SIZE)
            .ok_or(Error::InvalidInput)?;
        let chunk_end = page_offset
            .checked_add(chunk.len())
            .ok_or(Error::InvalidInput)?;
        if chunk_end > BLE_SLOT_SIZE {
            return Err(Error::InvalidInput);
        }

        let address = ble_slot_base(slot)
            .checked_add(page_offset)
            .ok_or(Error::InvalidInput)?;
        let mut page = [0xff; FLASH_PAGE_SIZE];
        page[..chunk.len()].copy_from_slice(chunk);
        flash_backend::write_page(address, &page).map_err(|_| Error::Unknown)
    }

    fn ble_get_metadata(&mut self) -> BleMetadata {
        MutableState::load().ble_metadata
    }

    fn set_ble_metadata(&mut self, metadata: &BleMetadata) -> Result<(), Error> {
        let mut state = MutableState::load();
        state.ble_metadata = *metadata;
        state.store().map_err(|_| Error::Unknown)
    }

    fn get_securechip_type(&mut self) -> Result<SecurechipType, ()> {
        Ok(SecurechipType::Optiga)
    }

    fn get_platform(&mut self) -> Result<Platform, ()> {
        #[cfg(target_arch = "arm")]
        {
            let version = bitbox_platform_stm32u5::otp::hardware_version().ok_or(())?;
            match version.platform() {
                0xffff => {
                    tracing::warn!("OTP platform version is 0xffff, defaulting to BitBox03");
                    Ok(Platform::BitBox03)
                }
                bitbox_platform_stm32u5::otp::HARDWARE_VERSION_PLATFORM_BITBOX03 => {
                    Ok(Platform::BitBox03)
                }
                _ => Err(()),
            }
        }

        #[cfg(not(target_arch = "arm"))]
        {
            Ok(Platform::BitBox03)
        }
    }

    fn get_device_name(&mut self) -> String {
        let state = MutableState::load();
        decode_string(&state.device_name, state.device_name_len)
            .filter(|name| validate_name(name, hal::memory::DEVICE_NAME_MAX_LEN))
            .unwrap_or_else(|| String::from(DEFAULT_DEVICE_NAME))
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), Error> {
        if !validate_name(name, hal::memory::DEVICE_NAME_MAX_LEN) {
            return Err(Error::InvalidInput);
        }
        let mut state = MutableState::load();
        state.device_name = array_from_str(name);
        state.device_name_len = name.len() as u8;
        state.store().map_err(|_| Error::Unknown)
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        MutableState::load().mnemonic_passphrase_enabled != 0
    }

    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        let mut state = MutableState::load();
        state.mnemonic_passphrase_enabled = enabled as u8;
        state.store()
    }

    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
        let mut state = MutableState::load();
        state.seed_birthdate = timestamp;
        state.store()
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        MutableState::load().seed_birthdate
    }

    fn is_seeded(&mut self) -> bool {
        MutableState::load().is_seeded != 0
    }

    fn is_initialized(&mut self) -> bool {
        MutableState::load().initialized != 0
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        let mut state = MutableState::load();
        state.initialized = 1;
        state.store()
    }

    fn get_encrypted_seed_and_hmac(&mut self) -> Result<(Vec<u8>, PasswordStretchAlgo), ()> {
        let state = MutableState::load();
        if state.is_seeded == 0 || state.encrypted_seed_len as usize > ENCRYPTED_SEED_MAX_LEN {
            return Err(());
        }
        let algo = match state.encrypted_seed_password_stretch_algo {
            x if x == StoredPasswordStretchAlgo::V0 as u8 => StoredPasswordStretchAlgo::V0,
            x if x == StoredPasswordStretchAlgo::V1 as u8 => StoredPasswordStretchAlgo::V1,
            _ => return Err(()),
        };
        Ok((
            state.encrypted_seed_and_hmac[..state.encrypted_seed_len as usize].to_vec(),
            hal_password_stretch_algo(algo),
        ))
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<(), ()> {
        if data.len() > ENCRYPTED_SEED_MAX_LEN {
            return Err(());
        }
        let mut state = MutableState::load();
        state.encrypted_seed_and_hmac = [0; ENCRYPTED_SEED_MAX_LEN];
        state.encrypted_seed_and_hmac[..data.len()].copy_from_slice(data);
        state.encrypted_seed_len = data.len() as u8;
        state.encrypted_seed_password_stretch_algo =
            stored_password_stretch_algo(password_stretch_algo) as u8;
        state.is_seeded = 1;
        state.store()
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        let mut state = MutableState::load();
        state.initialized = 0;
        state.is_seeded = 0;
        state.mnemonic_passphrase_enabled = 0;
        state.seed_birthdate = 0;
        state.device_name = [0; hal::memory::DEVICE_NAME_MAX_LEN];
        state.device_name_len = 0;
        state.encrypted_seed_and_hmac = [0; ENCRYPTED_SEED_MAX_LEN];
        state.encrypted_seed_len = 0;
        state.noise_static_private_key_generation =
            state.noise_static_private_key_generation.wrapping_add(1);
        state.noise_static_private_key =
            make_noise_static_private_key(state.noise_static_private_key_generation);
        state.noise_remote_static_pubkeys = [[0; 32]; NOISE_REMOTE_STATIC_PUBKEYS_LIMIT];
        state.noise_remote_static_pubkeys_len = 0;
        state.multisig_entries = [StoredMultisigEntry::empty(); MULTISIG_LIMIT];
        state.store()
    }

    fn get_noise_static_private_key(&mut self) -> Result<zeroize::Zeroizing<[u8; 32]>, ()> {
        Ok(zeroize::Zeroizing::new(
            MutableState::load().noise_static_private_key,
        ))
    }

    fn check_noise_remote_static_pubkey(&mut self, pubkey: &[u8; 32]) -> bool {
        let state = MutableState::load();
        state.noise_remote_static_pubkeys[..state.noise_remote_static_pubkeys_len as usize]
            .iter()
            .any(|stored| stored == pubkey)
    }

    fn add_noise_remote_static_pubkey(&mut self, pubkey: &[u8; 32]) -> Result<(), ()> {
        let mut state = MutableState::load();
        if state.noise_remote_static_pubkeys[..state.noise_remote_static_pubkeys_len as usize]
            .iter()
            .any(|stored| stored == pubkey)
        {
            return Ok(());
        }
        if state.noise_remote_static_pubkeys_len as usize == NOISE_REMOTE_STATIC_PUBKEYS_LIMIT {
            state.noise_remote_static_pubkeys.rotate_left(1);
            state.noise_remote_static_pubkeys[NOISE_REMOTE_STATIC_PUBKEYS_LIMIT - 1] = *pubkey;
        } else {
            state.noise_remote_static_pubkeys[state.noise_remote_static_pubkeys_len as usize] =
                *pubkey;
            state.noise_remote_static_pubkeys_len += 1;
        }
        state.store()
    }

    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        let salt_root = MutableState::load().salt_root;
        if salt_root.iter().all(|&byte| byte == 0xff) {
            Err(())
        } else {
            Ok(zeroize::Zeroizing::new(salt_root.to_vec()))
        }
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        let state = load_immutable_state();
        if state.attestation_present == 0 {
            return Err(());
        }
        *pubkey_out = state.attestation_device_pubkey;
        *certificate_out = state.attestation_certificate;
        *root_pubkey_identifier_out = state.attestation_root_pubkey_identifier;
        Ok(())
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        load_immutable_state().attestation_bootloader_hash
    }

    fn multisig_set_by_hash(&mut self, hash: &[u8; 32], name: &str) -> Result<(), Error> {
        if !validate_name(name, hal::memory::MULTISIG_NAME_MAX_LEN) {
            return Err(Error::InvalidInput);
        }

        let mut state = MutableState::load();
        for entry in &state.multisig_entries {
            if entry.in_use == 0 {
                continue;
            }
            if entry.name().as_deref() == Some(name) {
                if &entry.hash != hash {
                    return Err(Error::DuplicateName);
                }
                return Ok(());
            }
        }

        if let Some(entry) = state
            .multisig_entries
            .iter_mut()
            .find(|entry| entry.in_use != 0 && &entry.hash == hash)
        {
            entry.name = array_from_str(name);
            entry.name_len = name.len() as u8;
            return state.store().map_err(|_| Error::Unknown);
        }

        let Some(entry) = state
            .multisig_entries
            .iter_mut()
            .find(|entry| entry.in_use == 0)
        else {
            return Err(Error::Full);
        };
        entry.in_use = 1;
        entry.hash = *hash;
        entry.name = array_from_str(name);
        entry.name_len = name.len() as u8;
        state.store().map_err(|_| Error::Unknown)
    }

    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String> {
        let state = MutableState::load();
        state
            .multisig_entries
            .iter()
            .find(|entry| entry.in_use != 0 && &entry.hash == hash)
            .and_then(StoredMultisigEntry::name)
    }

    fn get_optiga_config_version(&mut self) -> Result<OptigaConfigVersion, ()> {
        Ok(OptigaConfigVersion::V1)
    }

    fn set_optiga_config_version(&mut self, version: OptigaConfigVersion) -> Result<(), ()> {
        match version {
            OptigaConfigVersion::V1 => Ok(()),
            OptigaConfigVersion::V0 => Err(()),
        }
    }

    fn get_io_protection_key(&mut self, out: &mut [u8; 32]) {
        *out = load_immutable_state().io_protection_key;
    }
}

#[cfg(not(target_arch = "arm"))]
mod flash_backend {
    use super::*;
    use core::cell::UnsafeCell;

    struct Singleton<T>(UnsafeCell<T>);

    impl<T> Singleton<T> {
        const fn new(value: T) -> Self {
            Self(UnsafeCell::new(value))
        }

        fn get(&self) -> *mut T {
            self.0.get()
        }
    }

    unsafe impl<T> Sync for Singleton<T> {}

    struct Storage {
        immutable_page: [u8; FLASH_PAGE_SIZE],
        config_page: [u8; FLASH_PAGE_SIZE],
        ble_slot_1: [u8; BLE_SLOT_SIZE],
        ble_slot_2: [u8; BLE_SLOT_SIZE],
    }

    static STORAGE: Singleton<Storage> = Singleton::new(Storage {
        immutable_page: [0xff; FLASH_PAGE_SIZE],
        config_page: [0xff; FLASH_PAGE_SIZE],
        ble_slot_1: [0xff; BLE_SLOT_SIZE],
        ble_slot_2: [0xff; BLE_SLOT_SIZE],
    });

    fn storage() -> &'static mut Storage {
        unsafe { &mut *STORAGE.get() }
    }

    fn region_bounds(addr: usize, len: usize) -> (usize, usize) {
        let end = addr.checked_add(len).unwrap();
        (addr, end)
    }

    pub(super) fn read(addr: usize, out: &mut [u8]) {
        let (addr, end) = region_bounds(addr, out.len());
        let storage = storage();
        if addr >= IMMUTABLE_PAGE_ADDR && end <= IMMUTABLE_PAGE_ADDR + FLASH_PAGE_SIZE {
            let offset = addr - IMMUTABLE_PAGE_ADDR;
            out.copy_from_slice(&storage.immutable_page[offset..offset + out.len()]);
        } else if addr >= CONFIG_PAGE_ADDR && end <= CONFIG_PAGE_ADDR + FLASH_PAGE_SIZE {
            let offset = addr - CONFIG_PAGE_ADDR;
            out.copy_from_slice(&storage.config_page[offset..offset + out.len()]);
        } else if addr >= BLE_SLOT_1_ADDR && end <= BLE_SLOT_1_ADDR + BLE_SLOT_SIZE {
            let offset = addr - BLE_SLOT_1_ADDR;
            out.copy_from_slice(&storage.ble_slot_1[offset..offset + out.len()]);
        } else if addr >= BLE_SLOT_2_ADDR && end <= BLE_SLOT_2_ADDR + BLE_SLOT_SIZE {
            let offset = addr - BLE_SLOT_2_ADDR;
            out.copy_from_slice(&storage.ble_slot_2[offset..offset + out.len()]);
        } else {
            panic!("unexpected flash read address: {addr:#x}..{end:#x}");
        }
    }

    pub(super) fn write_page(addr: usize, page: &[u8; FLASH_PAGE_SIZE]) -> Result<(), ()> {
        let (addr, end) = region_bounds(addr, FLASH_PAGE_SIZE);
        let storage = storage();
        if addr == IMMUTABLE_PAGE_ADDR {
            storage.immutable_page.copy_from_slice(page);
            Ok(())
        } else if addr == CONFIG_PAGE_ADDR {
            storage.config_page.copy_from_slice(page);
            Ok(())
        } else if addr >= BLE_SLOT_1_ADDR && end <= BLE_SLOT_1_ADDR + BLE_SLOT_SIZE {
            let offset = addr - BLE_SLOT_1_ADDR;
            storage.ble_slot_1[offset..offset + FLASH_PAGE_SIZE].copy_from_slice(page);
            Ok(())
        } else if addr >= BLE_SLOT_2_ADDR && end <= BLE_SLOT_2_ADDR + BLE_SLOT_SIZE {
            let offset = addr - BLE_SLOT_2_ADDR;
            storage.ble_slot_2[offset..offset + FLASH_PAGE_SIZE].copy_from_slice(page);
            Ok(())
        } else {
            Err(())
        }
    }

    #[cfg(test)]
    pub(super) fn erase_all() {
        let storage = storage();
        storage.immutable_page.fill(0xff);
        storage.config_page.fill(0xff);
        storage.ble_slot_1.fill(0xff);
        storage.ble_slot_2.fill(0xff);
    }
}

#[cfg(target_arch = "arm")]
mod flash_backend {
    use super::*;
    use bitbox_platform_stm32u5::flash;

    pub(super) fn read(addr: usize, out: &mut [u8]) {
        flash::read(addr, out);
    }

    pub(super) fn write_page(addr: usize, page: &[u8; FLASH_PAGE_SIZE]) -> Result<(), ()> {
        flash::write_page(addr, page).map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_DEVICE_NAME, ImmutableState, PasswordStretchAlgo, flash_backend,
        set_attestation_bootloader_hash, set_attestation_certificate,
        set_attestation_device_pubkey, store_immutable_state,
    };
    use bitbox_boot_utils::{
        IMMUTABLE_PAGE_MAGIC as IMMUTABLE_STORAGE_MAGIC,
        IMMUTABLE_PAGE_VERSION as IMMUTABLE_STORAGE_VERSION,
    };
    use bitbox_hal::{Hal, Memory};

    #[test]
    fn test_initialized_persists() {
        flash_backend::erase_all();

        let mut bitbox = crate::BitBox03::new();
        assert!(!bitbox.memory().is_initialized());

        bitbox.memory().set_initialized().unwrap();

        let mut bitbox = crate::BitBox03::new();
        assert!(bitbox.memory().is_initialized());
    }

    #[test]
    fn test_device_name_roundtrip() {
        flash_backend::erase_all();

        let mut bitbox = crate::BitBox03::new();
        assert_eq!(bitbox.memory().get_device_name(), DEFAULT_DEVICE_NAME);

        bitbox.memory().set_device_name("BitBox03 Dev").unwrap();

        let mut bitbox = crate::BitBox03::new();
        assert_eq!(bitbox.memory().get_device_name(), "BitBox03 Dev");
    }

    #[test]
    fn test_reset_hww_clears_mutable_state() {
        flash_backend::erase_all();

        let mut bitbox = crate::BitBox03::new();
        bitbox.memory().set_initialized().unwrap();
        bitbox.memory().set_device_name("BitBox03 Dev").unwrap();
        bitbox
            .memory()
            .set_encrypted_seed_and_hmac(&[0x11; 32], PasswordStretchAlgo::V1)
            .unwrap();

        bitbox.memory().reset_hww().unwrap();

        let mut bitbox = crate::BitBox03::new();
        assert!(!bitbox.memory().is_initialized());
        assert!(!bitbox.memory().is_seeded());
        assert_eq!(bitbox.memory().get_device_name(), DEFAULT_DEVICE_NAME);
    }

    #[test]
    fn test_reset_hww_preserves_immutable_page() {
        flash_backend::erase_all();

        let immutable = ImmutableState {
            magic: IMMUTABLE_STORAGE_MAGIC,
            version: IMMUTABLE_STORAGE_VERSION,
            root_pubkeys: [[0; 64]; 3],
            attestation_present: 1,
            _reserved0: [0; 3],
            io_protection_key: [0x88; 32],
            attestation_device_pubkey: [0x99; 64],
            attestation_certificate: [0xaa; 64],
            attestation_root_pubkey_identifier: [0xbb; 32],
            attestation_bootloader_hash: [0xcc; 32],
        };
        store_immutable_state(immutable).unwrap();

        let mut bitbox = crate::BitBox03::new();
        bitbox.memory().set_initialized().unwrap();
        bitbox.memory().reset_hww().unwrap();

        let mut io_protection_key = [0u8; 32];
        let mut attestation_pubkey = [0u8; 64];
        let mut attestation_certificate = [0u8; 64];
        let mut root_identifier = [0u8; 32];

        bitbox
            .memory()
            .get_io_protection_key(&mut io_protection_key);
        bitbox
            .memory()
            .get_attestation_pubkey_and_certificate(
                &mut attestation_pubkey,
                &mut attestation_certificate,
                &mut root_identifier,
            )
            .unwrap();

        assert_eq!(io_protection_key, [0x88; 32]);
        assert_eq!(attestation_pubkey, [0x99; 64]);
        assert_eq!(attestation_certificate, [0xaa; 64]);
        assert_eq!(root_identifier, [0xbb; 32]);
        assert_eq!(
            bitbox.memory().get_attestation_bootloader_hash(),
            [0xcc; 32]
        );
    }

    #[test]
    fn test_attestation_factory_setup_roundtrip() {
        flash_backend::erase_all();

        let mut immutable = ImmutableState::blank();
        immutable.root_pubkeys[0] = [0x44; 64];
        immutable.io_protection_key = [0x55; 32];
        store_immutable_state(immutable).unwrap();

        let expected_hash = [0x66; 32];
        let expected_pubkey = [0x77; 64];
        let expected_certificate = [0x88; 64];
        let expected_root_pubkey_identifier = [0x99; 32];

        set_attestation_bootloader_hash(&expected_hash).unwrap();
        set_attestation_device_pubkey(&expected_pubkey).unwrap();
        assert!(
            set_attestation_certificate(
                &[0x11; 64],
                &expected_certificate,
                &expected_root_pubkey_identifier,
            )
            .is_err()
        );
        set_attestation_certificate(
            &expected_pubkey,
            &expected_certificate,
            &expected_root_pubkey_identifier,
        )
        .unwrap();

        let state = super::load_immutable_state();
        assert_eq!(state.root_pubkeys[0], [0x44; 64]);
        assert_eq!(state.io_protection_key, [0x55; 32]);
        assert_eq!(state.attestation_bootloader_hash, expected_hash);
        assert_eq!(state.attestation_device_pubkey, expected_pubkey);
        assert_eq!(state.attestation_certificate, expected_certificate);
        assert_eq!(
            state.attestation_root_pubkey_identifier,
            expected_root_pubkey_identifier
        );
        assert_eq!(state.attestation_present, 1);
    }
}
