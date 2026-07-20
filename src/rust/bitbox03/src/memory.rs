use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::marker::PhantomData;

use bitbox_hal as hal;
use littlefs2::{
    driver::Storage,
    fs::{Filesystem, OpenOptions},
    io::{Error as LfsError, Read, Result as LfsResult, SeekFrom, Write},
    path,
};

const DEFAULT_DEVICE_NAME: &str = "My BitBox";

const DEVICE_NAME_PATH: &littlefs2::path::Path = path!("device-name");
const BLE_ENABLED_PATH: &littlefs2::path::Path = path!("ble-enabled");
const BLE_METADATA_PATH: &littlefs2::path::Path = path!("ble-meta");
const BLE_FIRMWARE_0_PATH: &littlefs2::path::Path = path!("ble-fw-0");
const BLE_FIRMWARE_1_PATH: &littlefs2::path::Path = path!("ble-fw-1");

const DA14531_VERSION_OFFSET: u32 = 0x110;
const DA14531_VERSION_LEN: usize = 23;

pub struct BitBox03Memory<UserStorage, VendorStorage> {
    _storage: PhantomData<fn() -> (UserStorage, VendorStorage)>,
}

impl<UserStorage, VendorStorage> BitBox03Memory<UserStorage, VendorStorage> {
    pub const fn new() -> Self {
        Self {
            _storage: PhantomData,
        }
    }
}

impl<UserStorage, VendorStorage> Default for BitBox03Memory<UserStorage, VendorStorage> {
    fn default() -> Self {
        Self::new()
    }
}

fn storage_is_erased<S: Storage>(storage: &mut S) -> bool {
    let Some(len) = S::BLOCK_SIZE.checked_mul(S::BLOCK_COUNT) else {
        return false;
    };
    let mut buf = [0u8; 256];
    let mut off = 0;
    while off < len {
        let read_len = core::cmp::min(buf.len(), len - off);
        if storage.read(off, &mut buf[..read_len]).is_err() {
            return false;
        }
        if buf[..read_len].iter().any(|&byte| byte != 0xff) {
            return false;
        }
        off += read_len;
    }
    true
}

fn with_fs<S, R>(
    storage: &mut S,
    f: impl FnOnce(&Filesystem<'_, S>) -> LfsResult<R>,
) -> LfsResult<R>
where
    S: Storage,
{
    let mut alloc = Filesystem::allocate();
    let fs = Filesystem::mount_or_else(&mut alloc, storage, |mount_err, storage, _alloc| {
        if storage_is_erased(storage) {
            Filesystem::format(storage)
        } else {
            Err(mount_err)
        }
    })?;
    f(&fs)
}

fn read_file_exact<S, const N: usize>(
    fs: &Filesystem<'_, S>,
    file_path: &littlefs2::path::Path,
) -> LfsResult<Option<[u8; N]>>
where
    S: Storage,
{
    match fs.metadata(file_path) {
        Ok(metadata) if metadata.len() == N => {
            let mut out = [0u8; N];
            fs.open_file_and_then(file_path, |file| file.read_exact(&mut out))?;
            Ok(Some(out))
        }
        Ok(_) => Ok(None),
        Err(err) if err == LfsError::NO_SUCH_ENTRY => Ok(None),
        Err(err) => Err(err),
    }
}

fn read_ble_metadata<S>(storage: &mut S) -> hal::memory::BleMetadata
where
    S: Storage,
{
    with_fs(storage, |fs| {
        Ok(
            read_file_exact::<_, { hal::memory::BleMetadata::ENCODED_LEN }>(fs, BLE_METADATA_PATH)?
                .and_then(|data| hal::memory::BleMetadata::decode(&data)),
        )
    })
    .ok()
    .flatten()
    .unwrap_or_default()
}

fn write_ble_metadata<S>(
    storage: &mut S,
    metadata: &hal::memory::BleMetadata,
) -> Result<(), hal::memory::Error>
where
    S: Storage,
{
    if metadata.active_index > 1
        || metadata
            .firmware_sizes
            .iter()
            .any(|&size| size as usize > hal::memory::BLE_FIRMWARE_MAX_SIZE)
    {
        return Err(hal::memory::Error::InvalidInput);
    }
    with_fs(storage, |fs| {
        fs.write(BLE_METADATA_PATH, &metadata.encode())
    })
    .map_err(lfs_to_memory_error)
}

fn ble_firmware_path(slot: hal::memory::BleFirmwareSlot) -> &'static littlefs2::path::Path {
    match slot {
        hal::memory::BleFirmwareSlot::First => BLE_FIRMWARE_0_PATH,
        hal::memory::BleFirmwareSlot::Second => BLE_FIRMWARE_1_PATH,
    }
}

fn write_ble_firmware_chunk<S>(
    storage: &mut S,
    slot: hal::memory::BleFirmwareSlot,
    chunk_index: u32,
    chunk: &[u8],
    flash_chunk_size: u32,
) -> Result<(), hal::memory::Error>
where
    S: Storage,
{
    if chunk.is_empty() || chunk.len() > flash_chunk_size as usize {
        return Err(hal::memory::Error::InvalidInput);
    }
    let offset = chunk_index
        .checked_mul(flash_chunk_size)
        .ok_or(hal::memory::Error::InvalidInput)? as usize;
    let end = offset
        .checked_add(chunk.len())
        .ok_or(hal::memory::Error::InvalidInput)?;
    if end > hal::memory::BLE_FIRMWARE_MAX_SIZE {
        return Err(hal::memory::Error::InvalidInput);
    }

    let file_path = ble_firmware_path(slot);
    with_fs(storage, |fs| {
        if chunk_index == 0 {
            return fs.write(file_path, chunk);
        }
        let metadata = fs.metadata(file_path)?;
        if metadata.len() < offset {
            return Err(LfsError::INVALID);
        }
        OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(false)
            .open_and_then(fs, file_path, |file| {
                file.seek(SeekFrom::Start(offset as u32))?;
                file.write_all(chunk)
            })
    })
    .map_err(lfs_to_memory_error)
}

fn lfs_to_memory_error(error: LfsError) -> hal::memory::Error {
    match error {
        error if error == LfsError::NO_SPACE => hal::memory::Error::Full,
        error if error == LfsError::INVALID => hal::memory::Error::InvalidInput,
        _ => hal::memory::Error::Unknown,
    }
}

impl<UserStorage, VendorStorage> hal::memory::Memory for BitBox03Memory<UserStorage, VendorStorage>
where
    UserStorage: Storage + Default,
    VendorStorage: Storage + Default,
{
    const BLE_FW_FLASH_CHUNK_SIZE: u32 = 4096;

    fn ble_enabled(&mut self) -> bool {
        let mut storage = UserStorage::default();
        with_fs(&mut storage, |fs| {
            Ok(match read_file_exact::<_, 1>(fs, BLE_ENABLED_PATH)? {
                Some([0]) => false,
                Some(_) | None => true,
            })
        })
        .unwrap_or(true)
    }

    fn ble_enable(&mut self, enable: bool) -> Result<(), ()> {
        let mut storage = UserStorage::default();
        with_fs(&mut storage, |fs| {
            fs.write(BLE_ENABLED_PATH, &[enable as u8])
        })
        .map_err(|_| ())
    }

    fn get_active_ble_firmware_version(&mut self) -> Result<String, hal::memory::Error> {
        let mut storage = VendorStorage::default();
        let metadata = read_ble_metadata(&mut storage);
        let slot = if metadata.active_index == 0 {
            hal::memory::BleFirmwareSlot::First
        } else {
            hal::memory::BleFirmwareSlot::Second
        };
        let file_path = ble_firmware_path(slot);
        let version = with_fs(&mut storage, |fs| {
            let mut version = [0u8; DA14531_VERSION_LEN];
            fs.open_file_and_then(file_path, |file| {
                file.seek(SeekFrom::Start(DA14531_VERSION_OFFSET))?;
                file.read_exact(&mut version)
            })?;
            Ok(version)
        })
        .map_err(lfs_to_memory_error)?;
        Ok(format!("{}", u16::from_le_bytes([version[1], version[2]])))
    }

    fn ble_firmware_flash_chunk(
        &mut self,
        slot: hal::memory::BleFirmwareSlot,
        chunk_index: u32,
        chunk: &[u8],
    ) -> Result<(), hal::memory::Error> {
        let mut storage = VendorStorage::default();
        write_ble_firmware_chunk(
            &mut storage,
            slot,
            chunk_index,
            chunk,
            Self::BLE_FW_FLASH_CHUNK_SIZE,
        )
    }

    fn ble_get_metadata(&mut self) -> hal::memory::BleMetadata {
        let mut storage = VendorStorage::default();
        read_ble_metadata(&mut storage)
    }

    fn set_ble_metadata(
        &mut self,
        metadata: &hal::memory::BleMetadata,
    ) -> Result<(), hal::memory::Error> {
        let mut storage = VendorStorage::default();
        write_ble_metadata(&mut storage, metadata)
    }

    fn get_securechip_type(&mut self) -> Result<hal::memory::SecurechipType, ()> {
        todo!()
    }

    fn get_platform(&mut self) -> Result<hal::memory::Platform, ()> {
        Ok(hal::memory::Platform::BitBox03)
    }

    fn get_device_name(&mut self) -> String {
        let mut storage = UserStorage::default();
        with_fs(&mut storage, |fs| {
            let mut buf = [0u8; hal::memory::DEVICE_NAME_MAX_LEN];
            let len = match fs.open_file_and_then(DEVICE_NAME_PATH, |file| file.read(&mut buf)) {
                Ok(len) => len,
                Err(err) if err == LfsError::NO_SUCH_ENTRY => return Ok(None),
                Err(err) => return Err(err),
            };
            if len == 0 {
                return Ok(None);
            }

            let Ok(name) = core::str::from_utf8(&buf[..len]) else {
                return Ok(None);
            };
            Ok(Some(name.to_string()))
        })
        .ok()
        .flatten()
        .unwrap_or_else(|| DEFAULT_DEVICE_NAME.to_string())
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), hal::memory::Error> {
        if !util::name::validate(name, hal::memory::DEVICE_NAME_MAX_LEN) {
            return Err(hal::memory::Error::InvalidInput);
        }
        let mut storage = UserStorage::default();
        with_fs(&mut storage, |fs| {
            fs.write(DEVICE_NAME_PATH, name.as_bytes())
        })
        .map_err(lfs_to_memory_error)
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        todo!()
    }

    fn set_mnemonic_passphrase_enabled(&mut self, _enabled: bool) -> Result<(), ()> {
        todo!()
    }

    fn set_seed_birthdate(&mut self, _timestamp: u32) -> Result<(), ()> {
        todo!()
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        todo!()
    }

    fn is_seeded(&mut self) -> bool {
        todo!()
    }

    fn is_initialized(&mut self) -> bool {
        todo!()
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn get_encrypted_seed_and_hmac(
        &mut self,
    ) -> Result<(Vec<u8>, hal::memory::PasswordStretchAlgo), ()> {
        todo!()
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        _data: &[u8],
        _password_stretch_algo: hal::memory::PasswordStretchAlgo,
    ) -> Result<(), ()> {
        todo!()
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn get_noise_static_private_key(&mut self) -> Result<zeroize::Zeroizing<[u8; 32]>, ()> {
        todo!()
    }

    fn check_noise_remote_static_pubkey(&mut self, _pubkey: &[u8; 32]) -> bool {
        todo!()
    }

    fn add_noise_remote_static_pubkey(&mut self, _pubkey: &[u8; 32]) -> Result<(), ()> {
        todo!()
    }

    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        todo!()
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        _pubkey_out: &mut [u8; 64],
        _certificate_out: &mut [u8; 64],
        _root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        todo!()
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        todo!()
    }

    fn multisig_set_by_hash(
        &mut self,
        _hash: &[u8; 32],
        _name: &str,
    ) -> Result<(), hal::memory::Error> {
        todo!()
    }

    fn multisig_get_by_hash(&self, _hash: &[u8; 32]) -> Option<String> {
        todo!()
    }

    fn get_optiga_config_version(&mut self) -> Result<hal::memory::OptigaConfigVersion, ()> {
        todo!()
    }

    fn set_optiga_config_version(
        &mut self,
        _version: hal::memory::OptigaConfigVersion,
    ) -> Result<(), ()> {
        todo!()
    }

    fn get_io_protection_key(&mut self, _out: &mut [u8; 32]) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitbox_hal::Memory;
    use bitbox_hal::memory::{
        BLE_FIRMWARE_MAX_SIZE, BleFirmwareSlot, BleMetadata, DEVICE_NAME_MAX_LEN, Error,
    };
    use bitbox_platform_host::memory::SimulatorStorage;
    use std::sync::{Mutex, MutexGuard};

    const USER_STORAGE_LEN: usize = 128 * 1024;
    const VENDOR_STORAGE_LEN: usize = 128 * 1024;

    type UserStorage = SimulatorStorage<USER_STORAGE_LEN, 0>;
    type VendorStorage = SimulatorStorage<VENDOR_STORAGE_LEN, 1>;
    type TestMemory = BitBox03Memory<UserStorage, VendorStorage>;

    static STORAGE_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn setup() -> (MutexGuard<'static, ()>, TestMemory) {
        let guard = STORAGE_TEST_LOCK.lock().unwrap();
        UserStorage::reset();
        VendorStorage::reset();
        (guard, TestMemory::new())
    }

    #[test]
    fn test_ble_enabled() {
        let (_guard, mut memory) = setup();

        assert!(memory.ble_enabled());
        memory.ble_enable(false).unwrap();
        assert!(!memory.ble_enabled());
        memory.ble_enable(true).unwrap();
        assert!(memory.ble_enabled());
    }

    #[test]
    fn test_get_set_device_name() {
        let (_guard, mut memory) = setup();

        assert_eq!(memory.get_device_name(), DEFAULT_DEVICE_NAME);

        memory.set_device_name("Test BitBox03").unwrap();
        assert_eq!(memory.get_device_name(), "Test BitBox03");

        let max_len_name = "DeviceName_ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxy";
        assert_eq!(max_len_name.len(), DEVICE_NAME_MAX_LEN);
        memory.set_device_name(max_len_name).unwrap();
        assert_eq!(memory.get_device_name(), max_len_name);

        assert_eq!(memory.set_device_name(""), Err(Error::InvalidInput));
        assert_eq!(memory.set_device_name(" name"), Err(Error::InvalidInput));
        assert_eq!(memory.get_device_name(), max_len_name);
    }

    #[test]
    fn test_ble_get_set_metadata() {
        let (_guard, mut memory) = setup();

        assert_eq!(memory.ble_get_metadata(), BleMetadata::default());

        let metadata = BleMetadata {
            active_index: 1,
            firmware_sizes: [123, 456],
            firmware_checksums: [7, 8],
            allowed_firmware_hash: [9; 32],
        };
        memory.set_ble_metadata(&metadata).unwrap();
        assert_eq!(memory.ble_get_metadata(), metadata);

        let mut invalid = metadata;
        invalid.active_index = 2;
        assert_eq!(memory.set_ble_metadata(&invalid), Err(Error::InvalidInput));
        assert_eq!(memory.ble_get_metadata(), metadata);

        let mut invalid = metadata;
        invalid.firmware_sizes[0] = (BLE_FIRMWARE_MAX_SIZE + 1) as u16;
        assert_eq!(memory.set_ble_metadata(&invalid), Err(Error::InvalidInput));
        assert_eq!(memory.ble_get_metadata(), metadata);
    }

    #[test]
    fn test_ble_firmware_flash_chunk() {
        let (_guard, mut memory) = setup();

        assert_eq!(
            memory.ble_firmware_flash_chunk(BleFirmwareSlot::First, 0, &[]),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            memory.ble_firmware_flash_chunk(
                BleFirmwareSlot::First,
                0,
                &[0; <TestMemory as Memory>::BLE_FW_FLASH_CHUNK_SIZE as usize + 1],
            ),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            memory.ble_firmware_flash_chunk(
                BleFirmwareSlot::First,
                BLE_FIRMWARE_MAX_SIZE as u32 / <TestMemory as Memory>::BLE_FW_FLASH_CHUNK_SIZE,
                &[0],
            ),
            Err(Error::InvalidInput)
        );

        let mut firmware = [0xff; 512];
        firmware[DA14531_VERSION_OFFSET as usize + 1] = 0x34;
        firmware[DA14531_VERSION_OFFSET as usize + 2] = 0x12;
        memory
            .ble_firmware_flash_chunk(BleFirmwareSlot::First, 0, &firmware)
            .unwrap();
        assert_eq!(memory.get_active_ble_firmware_version().unwrap(), "4660");
    }
}
