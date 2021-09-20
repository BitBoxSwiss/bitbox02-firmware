// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Small mocking infrastructure for testing.

extern crate alloc;
extern crate std;
use core::cell::RefCell;
use std::boxed::Box;

use crate::keystore;

use core::convert::TryInto;

#[derive(Default)]
pub struct Data {
    pub memory_set_device_name: Option<Box<dyn Fn(&str) -> Result<(), super::memory::Error>>>,
    pub ui_confirm_create: Option<Box<dyn Fn(&super::ui::ConfirmParams) -> bool>>,
    pub reset: Option<Box<dyn Fn(bool)>>,
    pub memory_set_mnemonic_passphrase_enabled: Option<Box<dyn Fn(bool) -> Result<(), ()>>>,
    pub sdcard_inserted: Option<bool>,
    pub ui_sdcard_create_arg: Option<bool>,
    pub memory_set_seed_birthdate: Option<Box<dyn Fn(u32) -> Result<(), ()>>>,
    pub memory_is_initialized: Option<bool>,
    pub memory_set_initialized_result: Option<Result<(), ()>>,
    pub ui_transaction_address_create: Option<Box<dyn Fn(&str, &str) -> bool>>,
    pub ui_transaction_fee_create: Option<Box<dyn Fn(&str, &str) -> bool>>,
}

pub struct SafeData(pub RefCell<Data>);

// Safety: must hold MUTEX lock before accessing.
unsafe impl Sync for SafeData {}

lazy_static! {
    pub static ref DATA: SafeData = SafeData(RefCell::new(Default::default()));
    pub static ref MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
}

/// Provide mock implementations and data. This also locks the keystore - use `mock_unlocked()` to mock a seeded and unlocked keystore.
pub fn mock(data: Data) {
    *DATA.0.borrow_mut() = data;
    keystore::lock();
}

/// This mocks an unlocked keystore with a fixed bip39 seed based on these bip39 recovery words:
/// `purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay`
pub fn mock_unlocked() {
    let seed: [u8; 32] = keystore::bip39_mnemonic_to_seed("purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay").unwrap().as_slice().try_into().unwrap();
    unsafe { bitbox02_sys::mock_state(seed.as_ptr(), core::ptr::null()) }
    keystore::unlock_bip39(&crate::input::SafeInputString::new()).unwrap();
}

/// This mounts a new FAT32 volume in RAM for use in unit tests. As there is only one volume, access only when holding `MUTEX`.
pub fn mock_sd() {
    unsafe {
        bitbox02_sys::sd_format();
    }
}
