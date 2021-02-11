// Copyright 2020 Shift Cryptosecurity AG
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

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::input::SafeInputString;
use bitbox02_sys::keystore_error_t;

pub use bitbox02_sys::xpub_type_t;

pub const BIP39_WORDLIST_LEN: u16 = bitbox02_sys::BIP39_WORDLIST_LEN as u16;
pub const EC_PUBLIC_KEY_UNCOMPRESSED_LEN: usize = bitbox02_sys::EC_PUBLIC_KEY_UNCOMPRESSED_LEN as _;

pub fn is_locked() -> bool {
    unsafe { bitbox02_sys::keystore_is_locked() }
}

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword { remaining_attempts: u8 },
    Unknown,
}

pub fn unlock(password: &SafeInputString) -> Result<(), Error> {
    let mut remaining_attempts: u8 = 0;
    match unsafe { bitbox02_sys::keystore_unlock(password.as_cstr(), &mut remaining_attempts) } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        keystore_error_t::KEYSTORE_ERR_INCORRECT_PASSWORD => {
            Err(Error::IncorrectPassword { remaining_attempts })
        }
        keystore_error_t::KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED => Err(Error::Unknown),
        keystore_error_t::KEYSTORE_ERR_GENERIC => Err(Error::Unknown),
    }
}

pub fn unlock_bip39(mnemonic_passphrase: &SafeInputString) -> Result<(), Error> {
    if unsafe { bitbox02_sys::keystore_unlock_bip39(mnemonic_passphrase.as_cstr()) } {
        Ok(())
    } else {
        Err(Error::CannotUnlockBIP39)
    }
}

pub fn create_and_store_seed(password: &SafeInputString, host_entropy: &[u8; 32]) -> bool {
    unsafe {
        bitbox02_sys::keystore_create_and_store_seed(password.as_cstr(), host_entropy.as_ptr())
    }
}

#[derive(Copy, Clone)]
struct ZeroizedMnemonic([u8; 256]);
impl core::default::Default for ZeroizedMnemonic {
    fn default() -> Self {
        ZeroizedMnemonic([0; 256])
    }
}
impl zeroize::DefaultIsZeroes for ZeroizedMnemonic {}

pub fn get_bip39_mnemonic() -> Result<zeroize::Zeroizing<String>, ()> {
    let mut mnemonic = zeroize::Zeroizing::new(ZeroizedMnemonic([0u8; 256]));
    match unsafe {
        bitbox02_sys::keystore_get_bip39_mnemonic(mnemonic.0.as_mut_ptr(), mnemonic.0.len() as _)
    } {
        false => Err(()),
        true => Ok(zeroize::Zeroizing::new(
            crate::util::str_from_null_terminated(&mnemonic.0[..])
                .unwrap()
                .into(),
        )),
    }
}

#[cfg(feature = "testing")]
pub fn get_bip39_word(idx: u16) -> Result<zeroize::Zeroizing<String>, ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_get_bip39_word.as_ref().unwrap()(idx)
}

/// `idx` must be smaller than BIP39_WORDLIST_LEN.
#[cfg(not(feature = "testing"))]
pub fn get_bip39_word(idx: u16) -> Result<zeroize::Zeroizing<String>, ()> {
    let mut word_ptr: *mut u8 = core::ptr::null_mut();
    match unsafe { bitbox02_sys::keystore_get_bip39_word(idx, &mut word_ptr) } {
        false => Err(()),
        true => {
            let word = unsafe {
                let len = crate::util::strlen_ptr(word_ptr);
                let slice = core::slice::from_raw_parts(word_ptr, len as _);
                zeroize::Zeroizing::new(core::str::from_utf8(&slice[..]).unwrap().into())
            };
            unsafe {
                bitbox02_sys::wally_free_string(word_ptr as _);
            }
            Ok(word)
        }
    }
}

/// An opaque C type which gives access to all BIP39 words.
pub struct Bip39Wordlist([*const u8; BIP39_WORDLIST_LEN as usize]);

impl Bip39Wordlist {
    pub fn as_ptr(&self) -> *const *const u8 {
        self.0.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Drop for Bip39Wordlist {
    fn drop(&mut self) {
        for ptr in self.0.iter() {
            unsafe {
                bitbox02_sys::wally_free_string(*ptr as _);
            }
        }
    }
}

pub fn get_bip39_wordlist() -> Result<Bip39Wordlist, ()> {
    let mut result = Bip39Wordlist([core::ptr::null(); BIP39_WORDLIST_LEN as usize]);
    for i in 0..BIP39_WORDLIST_LEN {
        let mut word_ptr: *mut u8 = core::ptr::null_mut();
        match unsafe { bitbox02_sys::keystore_get_bip39_word(i, &mut word_ptr) } {
            false => return Err(()),
            true => result.0[i as usize] = word_ptr,
        }
    }
    Ok(result)
}

#[cfg(not(feature = "testing"))]
pub fn secp256k1_pubkey_uncompressed(
    keypath: &[u32],
) -> Result<[u8; EC_PUBLIC_KEY_UNCOMPRESSED_LEN], ()> {
    let mut pubkey = [0u8; EC_PUBLIC_KEY_UNCOMPRESSED_LEN];
    match unsafe {
        bitbox02_sys::keystore_secp256k1_pubkey_uncompressed(
            keypath.as_ptr(),
            keypath.len() as _,
            pubkey.as_mut_ptr(),
        )
    } {
        true => Ok(pubkey),
        false => Err(()),
    }
}

#[cfg(feature = "testing")]
pub fn secp256k1_pubkey_uncompressed(
    keypath: &[u32],
) -> Result<[u8; EC_PUBLIC_KEY_UNCOMPRESSED_LEN], ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_secp256k1_pubkey_uncompressed
        .as_ref()
        .unwrap()(keypath)
}

#[cfg(not(feature = "testing"))]
pub fn encode_xpub_at_keypath(keypath: &[u32], xpub_type: xpub_type_t) -> Result<String, ()> {
    let mut xpub = [0u8; bitbox02_sys::XPUB_ENCODED_LEN as _];
    match unsafe {
        bitbox02_sys::keystore_encode_xpub_at_keypath(
            keypath.as_ptr(),
            keypath.len() as _,
            xpub_type,
            xpub.as_mut_ptr(),
            xpub.len() as _,
        )
    } {
        true => Ok(crate::util::str_from_null_terminated(&xpub[..])
            .unwrap()
            .into()),
        false => Err(()),
    }
}

#[cfg(feature = "testing")]
pub fn encode_xpub_at_keypath(keypath: &[u32], xpub_type: xpub_type_t) -> Result<String, ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_encode_xpub_at_keypath.as_ref().unwrap()(keypath, xpub_type)
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub recid: u8,
}

#[cfg(not(feature = "testing"))]
pub fn secp256k1_sign(
    keypath: &[u32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    let mut signature = [0u8; 64];
    let mut recid: util::c_types::c_int = 0;
    match unsafe {
        bitbox02_sys::keystore_secp256k1_sign(
            keypath.as_ptr(),
            keypath.len() as _,
            msg.as_ptr(),
            host_nonce.as_ptr(),
            signature.as_mut_ptr(),
            &mut recid,
        )
    } {
        true => Ok(SignResult {
            signature,
            recid: recid.try_into().unwrap(),
        }),
        false => Err(()),
    }
}

#[cfg(feature = "testing")]
pub fn secp256k1_sign(
    keypath: &[u32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_secp256k1_sign.as_ref().unwrap()(keypath, msg, host_nonce)
}

#[cfg(not(feature = "testing"))]
pub fn bip39_mnemonic_to_seed(mnemonic: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mnemonic = zeroize::Zeroizing::new(crate::util::str_to_cstr_vec(mnemonic)?);
    let mut seed = zeroize::Zeroizing::new([0u8; 32]);
    let mut seed_len: util::c_types::c_uint = 0;
    match unsafe {
        bitbox02_sys::keystore_bip39_mnemonic_to_seed(
            mnemonic.as_ptr(),
            seed.as_mut_ptr(),
            &mut seed_len,
        )
    } {
        true => Ok(zeroize::Zeroizing::new(seed[..seed_len as usize].to_vec())),
        false => Err(()),
    }
}

#[cfg(feature = "testing")]
pub fn bip39_mnemonic_to_seed(mnemonic: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_bip39_mnemonic_to_seed.as_ref().unwrap()(mnemonic)
}

pub fn root_fingerprint() -> Result<[u8; 4], ()> {
    let mut fingerprint = [0u8; 4];
    match unsafe { bitbox02_sys::keystore_get_root_fingerprint(fingerprint.as_mut_ptr()) } {
        true => Ok(fingerprint),
        false => Err(()),
    }
}

pub fn encrypt_and_store_seed(seed: &[u8], password: &str) -> Result<(), ()> {
    let password = zeroize::Zeroizing::new(crate::util::str_to_cstr_vec(password)?);
    match unsafe {
        bitbox02_sys::keystore_encrypt_and_store_seed(
            seed.as_ptr(),
            seed.len() as _,
            password.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}
