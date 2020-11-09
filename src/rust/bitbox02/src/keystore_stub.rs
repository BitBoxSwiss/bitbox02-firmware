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

//! Stubs for testing.

extern crate alloc;
use alloc::string::String;

use crate::safeinputstring::SafeInputString;

pub const BIP39_WORDLIST_LEN: u16 = bitbox02_sys::BIP39_WORDLIST_LEN as u16;

pub fn is_locked() -> bool {
    panic!("not implemented")
}

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword { remaining_attempts: u8 },
    Unknown,
}

pub fn unlock(_password: &SafeInputString) -> Result<(), Error> {
    panic!("not implemented")
}

pub fn unlock_bip39(_mnemonic_passphrase: &SafeInputString) -> Result<(), Error> {
    panic!("not implemented")
}

pub fn create_and_store_seed(_password: &SafeInputString, _host_entropy: &[u8; 32]) -> bool {
    panic!("not implemented")
}

pub fn get_bip39_mnemonic() -> Result<zeroize::Zeroizing<String>, ()> {
    panic!("not implemented")
}

pub fn get_bip39_word(_idx: u16) -> Result<&'static str, ()> {
    panic!("not implemented")
}
