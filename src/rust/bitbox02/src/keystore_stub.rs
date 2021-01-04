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

pub use bitbox02_sys::xpub_type_t;

extern crate alloc;
use alloc::string::String;

use crate::input::SafeInputString;

pub const BIP39_WORDLIST_LEN: u16 = bitbox02_sys::BIP39_WORDLIST_LEN as u16;
pub const EC_PUBLIC_KEY_UNCOMPRESSED_LEN: usize = bitbox02_sys::EC_PUBLIC_KEY_UNCOMPRESSED_LEN as _;

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

pub fn get_bip39_word(_idx: u16) -> Result<zeroize::Zeroizing<String>, ()> {
    panic!("not implemented")
}

pub struct Bip39Wordlist([*const u8; BIP39_WORDLIST_LEN as usize]);

impl Bip39Wordlist {
    pub fn as_ptr(&self) -> *const *const u8 {
        self.0.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn get_bip39_wordlist() -> Result<Bip39Wordlist, ()> {
    panic!("not implemented")
}

pub fn secp256k1_pubkey_uncompressed(
    keypath: &[u32],
) -> Result<[u8; EC_PUBLIC_KEY_UNCOMPRESSED_LEN], ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_secp256k1_pubkey_uncompressed
        .as_ref()
        .unwrap()(keypath)
}

pub fn encode_xpub_at_keypath(keypath: &[u32], xpub_type: xpub_type_t) -> Result<String, ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_encode_xpub_at_keypath.as_ref().unwrap()(keypath, xpub_type)
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub recid: u8,
}

pub fn secp256k1_sign(
    keypath: &[u32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_secp256k1_sign.as_ref().unwrap()(keypath, msg, host_nonce)
}
