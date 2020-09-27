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

extern crate std;
use core::cell::RefCell;
use std::boxed::Box;

#[derive(Default)]
pub struct Data {
    pub memory_set_device_name: Option<Box<dyn Fn(&str) -> Result<(), ()>>>,
    pub ui_confirm_create_body: Option<std::string::String>,
    pub ui_confirm_create_result: Option<bool>,
    pub reset: Option<Box<dyn Fn(bool)>>,
    pub memory_set_mnemonic_passphrase_enabled: Option<Box<dyn Fn(bool) -> Result<(), ()>>>,
    pub sdcard_inserted: Option<Box<dyn Fn() -> bool>>,
    pub ui_sdcard_create_arg: Option<bool>,
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
