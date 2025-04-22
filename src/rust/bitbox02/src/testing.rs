// Copyright 2020-2024 Shift Crypto AG
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

use crate::keystore;

/// This mocks an unlocked keystore with the given bip39 recovery words and bip39 passphrase.
pub fn mock_unlocked_using_mnemonic(mnemonic: &str, passphrase: &str) {
    let seed = keystore::bip39_mnemonic_to_seed(mnemonic).unwrap();
    unsafe {
        bitbox02_sys::keystore_mock_unlocked(seed.as_ptr(), seed.len() as _, core::ptr::null())
    }
    keystore::unlock_bip39(passphrase).unwrap();
}

pub const TEST_MNEMONIC: &str = "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay";

/// This mocks an unlocked keystore with a fixed bip39 seed based on these bip39 recovery words:
/// `purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay`
pub fn mock_unlocked() {
    mock_unlocked_using_mnemonic(TEST_MNEMONIC, "")
}

/// This mounts a new FAT32 volume in RAM for use in unit tests. As there is only one volume, access only serially.
pub fn mock_sd() {
    unsafe {
        bitbox02_sys::sd_format();
    }
}

unsafe extern "C" fn c_mock_random_32_bytes(buf_out: *mut u8) {
    let s = core::slice::from_raw_parts_mut(buf_out, 32);
    s.copy_from_slice(b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
}

static MEMORY_IFS: bitbox02_sys::memory_interface_functions_t =
    bitbox02_sys::memory_interface_functions_t {
        random_32_bytes: Some(c_mock_random_32_bytes),
    };

/// This sets up memory in RAM for use in unit tests. As there is only one RAM volume, access only serially.
/// The memory is initialized to be like after factory setup, i.e. 0xFF everywhere followed by `memory_setup()`.
pub fn mock_memory() {
    unsafe {
        bitbox02_sys::mock_memory_factoryreset();

        assert!(bitbox02_sys::memory_setup(&MEMORY_IFS));

        bitbox02_sys::smarteeprom_bb02_config();
        bitbox02_sys::bitbox02_smarteeprom_init();
        bitbox02_sys::spi_mem_full_erase();
    }
}

/// A wrapper that adds the Sync trait to RefCell. We can use this in testing as our unit tests run
/// single-threaded.
pub struct UnsafeSyncRefCell<T>(core::cell::RefCell<T>);
impl<T> UnsafeSyncRefCell<T> {
    pub const fn new(value: T) -> Self {
        Self(core::cell::RefCell::new(value))
    }
}

unsafe impl<T> Sync for UnsafeSyncRefCell<T> {}

impl<T> core::ops::Deref for UnsafeSyncRefCell<T> {
    type Target = core::cell::RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
