// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use bitbox_hal as hal;
use bitbox_lvgl::LvDisplay;
use bitbox_platform_host::{
    eeprom::FakeEeprom,
    memory::{FakeMemory, SimulatorStorage},
    sd::FakeSd,
    securechip::FakeSecureChip,
    timer::HostTimer,
};
use bitbox03::{BitBox03Memory, ui};
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use std::sync::Once;

mod random;
mod system;

const USER_STORAGE_LEN: usize = 128 * 1024;
const VENDOR_STORAGE_LEN: usize = 128 * 1024;

type UserStorage = SimulatorStorage<USER_STORAGE_LEN, 0>;
type VendorStorage = SimulatorStorage<VENDOR_STORAGE_LEN, 1>;
type BitBox03SimulatorStorage = BitBox03Memory<UserStorage, VendorStorage>;

pub struct BitBox03SimulatorMemory {
    bitbox03: BitBox03SimulatorStorage,
    fallback: FakeMemory,
}

impl BitBox03SimulatorMemory {
    fn new() -> Self {
        let mut fallback = FakeMemory::new();
        fallback.set_platform(hal::memory::Platform::BitBox03);
        Self {
            bitbox03: BitBox03Memory::new(),
            fallback,
        }
    }
}

impl hal::memory::Memory for BitBox03SimulatorMemory {
    const BLE_FW_FLASH_CHUNK_SIZE: u32 =
        <BitBox03SimulatorStorage as hal::memory::Memory>::BLE_FW_FLASH_CHUNK_SIZE;

    fn ble_enabled(&mut self) -> bool {
        self.bitbox03.ble_enabled()
    }

    fn ble_enable(&mut self, enable: bool) -> Result<(), ()> {
        self.bitbox03.ble_enable(enable)
    }

    fn get_active_ble_firmware_version(&mut self) -> Result<String, hal::memory::Error> {
        self.bitbox03.get_active_ble_firmware_version()
    }

    fn ble_firmware_flash_chunk(
        &mut self,
        slot: hal::memory::BleFirmwareSlot,
        chunk_index: u32,
        chunk: &[u8],
    ) -> Result<(), hal::memory::Error> {
        self.bitbox03
            .ble_firmware_flash_chunk(slot, chunk_index, chunk)
    }

    fn ble_get_metadata(&mut self) -> hal::memory::BleMetadata {
        self.bitbox03.ble_get_metadata()
    }

    fn set_ble_metadata(
        &mut self,
        metadata: &hal::memory::BleMetadata,
    ) -> Result<(), hal::memory::Error> {
        self.bitbox03.set_ble_metadata(metadata)
    }

    fn get_securechip_type(&mut self) -> Result<hal::memory::SecurechipType, ()> {
        self.fallback.get_securechip_type()
    }

    fn get_optiga_config_version(&mut self) -> Result<hal::memory::OptigaConfigVersion, ()> {
        self.fallback.get_optiga_config_version()
    }

    fn set_optiga_config_version(
        &mut self,
        version: hal::memory::OptigaConfigVersion,
    ) -> Result<(), ()> {
        self.fallback.set_optiga_config_version(version)
    }

    fn get_platform(&mut self) -> Result<hal::memory::Platform, ()> {
        self.bitbox03.get_platform()
    }

    fn get_device_name(&mut self) -> String {
        self.bitbox03.get_device_name()
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), hal::memory::Error> {
        self.bitbox03.set_device_name(name)
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        self.fallback.is_mnemonic_passphrase_enabled()
    }

    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        self.fallback.set_mnemonic_passphrase_enabled(enabled)
    }

    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
        self.fallback.set_seed_birthdate(timestamp)
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        self.fallback.get_seed_birthdate()
    }

    fn is_seeded(&mut self) -> bool {
        self.fallback.is_seeded()
    }

    fn is_initialized(&mut self) -> bool {
        self.fallback.is_initialized()
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        self.fallback.set_initialized()
    }

    fn get_encrypted_seed_and_hmac(
        &mut self,
    ) -> Result<(Vec<u8>, hal::memory::PasswordStretchAlgo), ()> {
        self.fallback.get_encrypted_seed_and_hmac()
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: hal::memory::PasswordStretchAlgo,
    ) -> Result<(), ()> {
        self.fallback
            .set_encrypted_seed_and_hmac(data, password_stretch_algo)
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        self.fallback.reset_hww()
    }

    fn get_noise_static_private_key(&mut self) -> Result<zeroize::Zeroizing<[u8; 32]>, ()> {
        self.fallback.get_noise_static_private_key()
    }

    fn check_noise_remote_static_pubkey(&mut self, pubkey: &[u8; 32]) -> bool {
        self.fallback.check_noise_remote_static_pubkey(pubkey)
    }

    fn add_noise_remote_static_pubkey(&mut self, pubkey: &[u8; 32]) -> Result<(), ()> {
        self.fallback.add_noise_remote_static_pubkey(pubkey)
    }

    fn get_io_protection_key(&mut self, out: &mut [u8; 32]) {
        self.fallback.get_io_protection_key(out)
    }

    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        self.fallback.get_salt_root()
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        self.fallback.get_attestation_pubkey_and_certificate(
            pubkey_out,
            certificate_out,
            root_pubkey_identifier_out,
        )
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        self.fallback.get_attestation_bootloader_hash()
    }

    fn multisig_set_by_hash(
        &mut self,
        hash: &[u8; 32],
        name: &str,
    ) -> Result<(), hal::memory::Error> {
        self.fallback.multisig_set_by_hash(hash, name)
    }

    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String> {
        self.fallback.multisig_get_by_hash(hash)
    }
}

struct BitBox03State {
    ui: ui::BitBox03Ui<HostTimer>,
    random: random::BitBox03Random,
    sd: FakeSd,
    securechip: FakeSecureChip,
    memory: BitBox03SimulatorMemory,
    eeprom: FakeEeprom,
    system: system::BitBox03System,
}

impl BitBox03State {
    fn new() -> Self {
        let mut sd = FakeSd::new();
        sd.inserted = Some(true);

        Self {
            ui: ui::BitBox03Ui::new(),
            random: random::BitBox03Random {},
            sd,
            securechip: FakeSecureChip::new(),
            memory: BitBox03SimulatorMemory::new(),
            eeprom: FakeEeprom::new(),
            system: system::BitBox03System {},
        }
    }
}

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

static BITBOX03_INIT: Once = Once::new();
static BITBOX03: Singleton<MaybeUninit<BitBox03State>> = Singleton::new(MaybeUninit::uninit());

fn state() -> &'static mut BitBox03State {
    BITBOX03_INIT.call_once(|| unsafe {
        (*BITBOX03.get()).write(BitBox03State::new());
    });
    unsafe { (&mut *BITBOX03.get()).assume_init_mut() }
}

#[derive(Copy, Clone, Default)]
pub struct BitBox03;

impl BitBox03 {
    pub const fn new() -> BitBox03 {
        BitBox03
    }

    pub fn init(&mut self, display: LvDisplay) {
        state().ui.init(display);
    }
}

impl hal::Hal for BitBox03 {
    type Ui = ui::BitBox03Ui<HostTimer>;
    type Random = random::BitBox03Random;
    type Sd = FakeSd;
    type SecureChip = FakeSecureChip;
    type Memory = BitBox03SimulatorMemory;
    type Eeprom = FakeEeprom;
    type System = system::BitBox03System;

    fn as_mut(
        &mut self,
    ) -> hal::HalSubsystems<
        '_,
        Self::Ui,
        Self::Random,
        Self::Sd,
        Self::SecureChip,
        Self::Memory,
        Self::Eeprom,
        Self::System,
    > {
        let state = state();
        hal::HalSubsystems {
            ui: &mut state.ui,
            random: &mut state.random,
            sd: &mut state.sd,
            securechip: &mut state.securechip,
            memory: &mut state.memory,
            eeprom: &mut state.eeprom,
            system: &mut state.system,
        }
    }
}
