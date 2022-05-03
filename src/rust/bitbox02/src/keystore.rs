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
pub const EC_PUBLIC_KEY_LEN: usize = bitbox02_sys::EC_PUBLIC_KEY_LEN as _;
pub const MAX_SEED_LENGTH: usize = bitbox02_sys::KEYSTORE_MAX_SEED_LENGTH as usize;

pub fn is_locked() -> bool {
    unsafe { bitbox02_sys::keystore_is_locked() }
}

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword { remaining_attempts: u8 },
    MaxAttemptsExceeded,
    Unseeded,
    Memory,
    // Securechip error with the error code from securechip.c
    ScKdf(i32),
    Salt,
    Hash,
    SeedSize,
    Unknown,
}

pub fn unlock(password: &SafeInputString) -> Result<(), Error> {
    let mut remaining_attempts: u8 = 0;
    let mut securechip_result: i32 = 0;
    match unsafe {
        bitbox02_sys::keystore_unlock(
            password.as_cstr(),
            &mut remaining_attempts,
            &mut securechip_result,
        )
    } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        keystore_error_t::KEYSTORE_ERR_INCORRECT_PASSWORD => {
            Err(Error::IncorrectPassword { remaining_attempts })
        }
        keystore_error_t::KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED => Err(Error::MaxAttemptsExceeded),
        keystore_error_t::KEYSTORE_ERR_UNSEEDED => Err(Error::Unseeded),
        keystore_error_t::KEYSTORE_ERR_MEMORY => Err(Error::Memory),
        keystore_error_t::KEYSTORE_ERR_SEED_SIZE => Err(Error::SeedSize),
        keystore_error_t::KEYSTORE_ERR_SC_KDF => Err(Error::ScKdf(securechip_result)),
        keystore_error_t::KEYSTORE_ERR_SALT => Err(Error::Salt),
        keystore_error_t::KEYSTORE_ERR_HASH => Err(Error::Hash),
    }
}

pub fn lock() {
    unsafe { bitbox02_sys::keystore_lock() }
}

pub fn unlock_bip39(mnemonic_passphrase: &SafeInputString) -> Result<(), Error> {
    if unsafe { bitbox02_sys::keystore_unlock_bip39(mnemonic_passphrase.as_cstr()) } {
        Ok(())
    } else {
        Err(Error::CannotUnlockBIP39)
    }
}

pub fn create_and_store_seed(password: &SafeInputString, host_entropy: &[u8]) -> bool {
    unsafe {
        bitbox02_sys::keystore_create_and_store_seed(
            password.as_cstr(),
            host_entropy.as_ptr(),
            host_entropy.len() as _,
        )
    }
}

#[cfg(feature = "testing")]
pub fn copy_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut seed = zeroize::Zeroizing::new([0u8; MAX_SEED_LENGTH]);
    let mut seed_len: u32 = 0;
    match unsafe { bitbox02_sys::keystore_copy_seed(seed.as_mut_ptr(), &mut seed_len) } {
        true => Ok(zeroize::Zeroizing::new(seed[..seed_len as usize].to_vec())),
        false => Err(()),
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

/// `idx` must be smaller than BIP39_WORDLIST_LEN.
pub fn get_bip39_word(idx: u16) -> Result<zeroize::Zeroizing<String>, ()> {
    let mut word_ptr: *mut u8 = core::ptr::null_mut();
    match unsafe { bitbox02_sys::keystore_get_bip39_word(idx, &mut word_ptr) } {
        false => Err(()),
        true => {
            let word = unsafe {
                let len = crate::util::strlen_ptr(word_ptr);
                let slice = core::slice::from_raw_parts(word_ptr, len as _);
                zeroize::Zeroizing::new(core::str::from_utf8(slice).unwrap().into())
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

pub struct SignResult {
    pub signature: [u8; 64],
    pub recid: u8,
}

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

pub fn secp256k1_nonce_commit(
    keypath: &[u32],
    msg: &[u8; 32],
    host_commitment: &[u8; 32],
) -> Result<[u8; EC_PUBLIC_KEY_LEN], ()> {
    let mut signer_commitment = [0u8; EC_PUBLIC_KEY_LEN];
    match unsafe {
        bitbox02_sys::keystore_secp256k1_nonce_commit(
            keypath.as_ptr(),
            keypath.len() as _,
            msg.as_ptr(),
            host_commitment.as_ptr(),
            signer_commitment.as_mut_ptr(),
        )
    } {
        true => Ok(signer_commitment),
        false => Err(()),
    }
}

pub fn bip39_mnemonic_to_seed(mnemonic: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mnemonic = zeroize::Zeroizing::new(crate::util::str_to_cstr_vec(mnemonic)?);
    let mut seed = zeroize::Zeroizing::new([0u8; MAX_SEED_LENGTH]);
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

pub fn root_fingerprint() -> Result<[u8; 4], ()> {
    let mut fingerprint = [0u8; 4];
    match unsafe { bitbox02_sys::keystore_get_root_fingerprint(fingerprint.as_mut_ptr()) } {
        true => Ok(fingerprint),
        false => Err(()),
    }
}

pub fn encrypt_and_store_seed(seed: &[u8], password: &SafeInputString) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::keystore_encrypt_and_store_seed(
            seed.as_ptr(),
            seed.len() as _,
            password.as_cstr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn get_ed25519_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut seed = zeroize::Zeroizing::new([0u8; 96].to_vec());
    match unsafe { bitbox02_sys::keystore_get_ed25519_seed(seed.as_mut_ptr()) } {
        true => Ok(seed),
        false => Err(()),
    }
}

pub fn secp256k1_schnorr_bip86_sign(keypath: &[u32], msg: &[u8; 32]) -> Result<[u8; 64], ()> {
    let mut signature = [0u8; 64];
    match unsafe {
        bitbox02_sys::keystore_secp256k1_schnorr_bip86_sign(
            keypath.as_ptr(),
            keypath.len() as _,
            msg.as_ptr(),
            signature.as_mut_ptr(),
        )
    } {
        true => Ok(signature),
        false => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{mock_unlocked, TEST_MNEMONIC};

    #[test]
    fn test_bip39_mnemonic_to_seed() {
        assert!(bip39_mnemonic_to_seed("invalid").is_err());

        // Zero seed
        assert_eq!(
            bip39_mnemonic_to_seed("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap().as_ref() as &[u8],
            &[0u8; 16],
        );

        // 12 words
        assert_eq!(
            bip39_mnemonic_to_seed(
                "trust cradle viable innocent stand equal little small junior frost laundry room"
            )
            .unwrap()
            .as_ref() as &[u8],
            b"\xe9\xa6\x3f\xcd\x3a\x4d\x48\x98\x20\xa6\x63\x79\x2b\xad\xf6\xdd",
        );

        // 18 words
        assert_eq!(
            bip39_mnemonic_to_seed("pupil parent toe bright slam plastic spy suspect verb battle nominee loan call crystal upset razor luggage join").unwrap().as_ref() as &[u8],
            b"\xad\xf4\x07\x8e\x0e\x0c\xb1\x4c\x34\xd6\xd6\xf2\x82\x6a\x57\xc1\x82\x06\x6a\xbb\xcd\x95\x84\xcf",
        );

        // 24 words
        assert_eq!(
            bip39_mnemonic_to_seed("purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay").unwrap().as_ref() as &[u8],
            b"\xae\x45\xd4\x02\x3a\xfa\x4a\x48\x68\x77\x51\x69\xfe\xa5\xf5\xe4\x97\xf7\xa1\xa4\xd6\x22\x9a\xd0\x23\x9e\x68\x9b\x48\x2e\xd3\x5e",
        );
    }

    #[test]
    fn test_get_bip39_mnemonic() {
        lock();
        assert!(get_bip39_mnemonic().is_err());

        mock_unlocked();

        assert_eq!(
            get_bip39_mnemonic().unwrap().as_ref() as &str,
            TEST_MNEMONIC
        );
    }

    #[test]
    fn test_get_bip39_word() {
        assert!(get_bip39_word(2048).is_err());

        assert_eq!(get_bip39_word(0).unwrap().as_ref() as &str, "abandon");
        assert_eq!(get_bip39_word(2047).unwrap().as_ref() as &str, "zoo");
        assert_eq!(get_bip39_word(563).unwrap().as_ref() as &str, "edit");
    }

    #[test]
    fn test_get_ed25519_seed() {
        // No seed on a locked keystore.
        lock();
        assert!(get_ed25519_seed().is_err());

        mock_unlocked();
        assert_eq!(
            get_ed25519_seed().unwrap().as_ref() as &[u8],
            b"\xf8\xcb\x28\x85\x37\x60\x2b\x90\xd1\x29\x75\x4b\xdd\x0e\x4b\xed\xf9\xe2\x92\x3a\x04\xb6\x86\x7e\xdb\xeb\xc7\x93\xa7\x17\x6f\x5d\xca\xc5\xc9\x5d\x5f\xd2\x3a\x8e\x01\x6c\x95\x57\x69\x0e\xad\x1f\x00\x2b\x0f\x35\xd7\x06\xff\x8e\x59\x84\x1c\x09\xe0\xb6\xbb\x23\xf0\xa5\x91\x06\x42\xd0\x77\x98\x17\x40\x2e\x5e\x7a\x75\x54\x95\xe7\x44\xf5\x5c\xf1\x1e\x49\xee\xfd\x22\xa4\x60\xe9\xb2\xf7\x53",
        );
    }
}
