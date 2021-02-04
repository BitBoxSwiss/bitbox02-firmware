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
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use std::boxed::Box;

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
    pub backup_create: Option<Box<dyn Fn(u32, u32) -> Result<(), super::backup::Error>>>,
    pub keystore_encode_xpub_at_keypath:
        Option<Box<dyn Fn(&[u32], super::keystore::xpub_type_t) -> Result<String, ()>>>,
    pub keystore_secp256k1_pubkey_uncompressed: Option<
        Box<dyn Fn(&[u32]) -> Result<[u8; super::keystore::EC_PUBLIC_KEY_UNCOMPRESSED_LEN], ()>>,
    >,
    pub keystore_secp256k1_sign: Option<
        Box<dyn Fn(&[u32], &[u8; 32], &[u8; 32]) -> Result<super::keystore::SignResult, ()>>,
    >,
    pub keystore_get_bip39_word: Option<Box<dyn Fn(u16) -> Result<zeroize::Zeroizing<String>, ()>>>,
    pub keystore_bip39_mnemonic_to_seed:
        Option<Box<dyn Fn(&str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()>>>,
    pub btc_address_simple: Option<
        Box<
            dyn Fn(
                bitbox02_sys::BTCCoin,
                bitbox02_sys::BTCScriptConfig_SimpleType,
                &[u32],
            ) -> Result<String, ()>,
        >,
    >,
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

pub fn mock(data: Data) {
    *DATA.0.borrow_mut() = data;
}
