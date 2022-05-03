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
use alloc::boxed::Box;
use alloc::string::String;
use core::cell::RefCell;

use crate::keystore;

#[derive(Default)]
pub struct Data {
    pub ui_confirm_create: Option<Box<dyn Fn(&super::ui::ConfirmParams) -> bool>>,
    pub reset: Option<Box<dyn Fn(bool)>>,
    pub sdcard_inserted: Option<bool>,
    pub ui_sdcard_create_arg: Option<bool>,
    pub ui_transaction_address_create: Option<Box<dyn Fn(&str, &str) -> bool>>,
    pub ui_transaction_fee_create: Option<Box<dyn Fn(&str, &str) -> bool>>,
    pub ui_trinary_input_string_create:
        Option<Box<dyn Fn(&super::ui::TrinaryInputStringParams) -> String>>,
}

pub struct SafeData(pub RefCell<Data>);

// Safety: must not be accessed concurrently.
unsafe impl Sync for SafeData {}

lazy_static! {
    pub static ref DATA: SafeData = SafeData(RefCell::new(Default::default()));
}

/// Provide mock implementations and data. This also locks the keystore - use `mock_unlocked()` to mock a seeded and unlocked keystore.
pub fn mock(data: Data) {
    *DATA.0.borrow_mut() = data;
    keystore::lock();
}

/// This mocks an unlocked keystore with the given bip39 recovery words.
pub fn mock_unlocked_using_mnemonic(mnemonic: &str) {
    let seed = keystore::bip39_mnemonic_to_seed(mnemonic).unwrap();
    unsafe {
        bitbox02_sys::keystore_mock_unlocked(seed.as_ptr(), seed.len() as _, core::ptr::null())
    }
    keystore::unlock_bip39(&crate::input::SafeInputString::new()).unwrap();
}

pub const TEST_MNEMONIC: &str = "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay";

/// This mocks an unlocked keystore with a fixed bip39 seed based on these bip39 recovery words:
/// `purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay`
pub fn mock_unlocked() {
    mock_unlocked_using_mnemonic(TEST_MNEMONIC)
}

/// This mounts a new FAT32 volume in RAM for use in unit tests. As there is only one volume, access only serially.
pub fn mock_sd() {
    unsafe {
        bitbox02_sys::sd_format();
    }
}

/// This sets up memory in RAM for use in unit tests. As there is only one RAM volume, access only serially.
/// The memory is initialized to be like after factory setup, i.e. 0xFF everywhere followed by `memory_setup()`.
pub fn mock_memory() {
    unsafe {
        bitbox02_sys::mock_memory_factoryreset();

        unsafe extern "C" fn c_mock_random_32_bytes(buf_out: *mut u8) {
            let s = core::slice::from_raw_parts_mut(buf_out, 32);
            s.copy_from_slice(b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        }

        let ifs = bitbox02_sys::memory_interface_functions_t {
            random_32_bytes: Some(c_mock_random_32_bytes),
        };
        assert!(bitbox02_sys::memory_setup(&ifs));

        bitbox02_sys::smarteeprom_bb02_config();
        bitbox02_sys::bitbox02_smarteeprom_init();
    }
}
