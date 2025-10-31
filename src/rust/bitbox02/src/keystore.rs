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

use alloc::vec;
use alloc::vec::Vec;

use bitcoin::secp256k1::{All, Secp256k1};

use core::convert::TryInto;

use bitbox02_sys::keystore_error_t;

/// Length of a compressed secp256k1 pubkey.
const EC_PUBLIC_KEY_LEN: usize = 33;
pub const MAX_SEED_LENGTH: usize = bitbox02_sys::KEYSTORE_MAX_SEED_LENGTH as usize;

pub fn _is_locked() -> bool {
    unsafe { bitbox02_sys::keystore_is_locked() }
}

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword { remaining_attempts: u8 },
    MaxAttemptsExceeded,
    Unseeded,
    Memory,
    // Securechip error with the error code from securechip.c. 0 if the error is unspecified.
    SecureChip(i32),
    Salt,
    Hash,
    SeedSize,
    Encrypt,
}

impl core::convert::From<keystore_error_t> for Error {
    fn from(error: keystore_error_t) -> Self {
        match error {
            keystore_error_t::KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED => Error::MaxAttemptsExceeded,
            keystore_error_t::KEYSTORE_ERR_UNSEEDED => Error::Unseeded,
            keystore_error_t::KEYSTORE_ERR_MEMORY => Error::Memory,
            keystore_error_t::KEYSTORE_ERR_SEED_SIZE => Error::SeedSize,
            keystore_error_t::KEYSTORE_ERR_SECURECHIP => Error::SecureChip(0),
            keystore_error_t::KEYSTORE_ERR_SALT => Error::Salt,
            keystore_error_t::KEYSTORE_ERR_HASH => Error::Hash,
            keystore_error_t::KEYSTORE_ERR_ENCRYPT => Error::Encrypt,
            _ => panic!("cannot convert error"),
        }
    }
}

pub fn _unlock(password: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
    let mut remaining_attempts: u8 = 0;
    let mut securechip_result: i32 = 0;
    let mut seed = zeroize::Zeroizing::new([0u8; MAX_SEED_LENGTH].to_vec());
    let mut seed_len: usize = 0;
    match unsafe {
        bitbox02_sys::keystore_unlock(
            crate::util::str_to_cstr_vec(password)
                .unwrap()
                .as_ptr()
                .cast(),
            &mut remaining_attempts,
            &mut securechip_result,
            seed.as_mut_ptr(),
            &mut seed_len,
        )
    } {
        keystore_error_t::KEYSTORE_OK => {
            seed.truncate(seed_len);
            Ok(seed)
        }
        keystore_error_t::KEYSTORE_ERR_INCORRECT_PASSWORD => {
            Err(Error::IncorrectPassword { remaining_attempts })
        }
        keystore_error_t::KEYSTORE_ERR_SECURECHIP => Err(Error::SecureChip(securechip_result)),
        err => Err(err.into()),
    }
}

pub fn _lock() {
    unsafe { bitbox02_sys::keystore_lock() }
}

pub fn unlock_bip39_check(seed: &[u8]) -> Result<(), Error> {
    if unsafe { bitbox02_sys::keystore_unlock_bip39_check(seed.as_ptr(), seed.len()) } {
        Ok(())
    } else {
        Err(Error::CannotUnlockBIP39)
    }
}

pub fn unlock_bip39_finalize(bip39_seed: &[u8; 64]) -> Result<(), Error> {
    if unsafe { bitbox02_sys::keystore_unlock_bip39_finalize(bip39_seed.as_ptr()) } {
        Ok(())
    } else {
        Err(Error::CannotUnlockBIP39)
    }
}

#[cfg(feature = "testing")]
pub fn test_get_retained_seed_encrypted() -> &'static [u8] {
    unsafe {
        let mut len = 0usize;
        let ptr = bitbox02_sys::keystore_test_get_retained_seed_encrypted(&mut len);
        core::slice::from_raw_parts(ptr, len)
    }
}

#[cfg(feature = "testing")]
pub fn test_get_retained_bip39_seed_encrypted() -> &'static [u8] {
    unsafe {
        let mut len = 0usize;
        let ptr = bitbox02_sys::keystore_test_get_retained_bip39_seed_encrypted(&mut len);
        core::slice::from_raw_parts(ptr, len)
    }
}

pub fn _copy_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut seed = zeroize::Zeroizing::new([0u8; MAX_SEED_LENGTH].to_vec());
    let mut seed_len: usize = 0;
    match unsafe { bitbox02_sys::keystore_copy_seed(seed.as_mut_ptr(), &mut seed_len) } {
        true => {
            seed.truncate(seed_len);
            Ok(seed)
        }
        false => Err(()),
    }
}

pub fn _copy_bip39_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut bip39_seed = zeroize::Zeroizing::new(vec![0u8; 64]);
    match unsafe { bitbox02_sys::keystore_copy_bip39_seed(bip39_seed.as_mut_ptr()) } {
        true => Ok(bip39_seed),
        false => Err(()),
    }
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub recid: u8,
}

pub fn _secp256k1_sign(
    secp: &Secp256k1<All>,
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    let mut signature = [0u8; 64];
    let mut recid: core::ffi::c_int = 0;
    match unsafe {
        bitbox02_sys::keystore_secp256k1_sign(
            secp.ctx().as_ptr().cast(),
            private_key.as_ptr(),
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

pub fn _secp256k1_nonce_commit(
    secp: &Secp256k1<All>,
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_commitment: &[u8; 32],
) -> Result<[u8; EC_PUBLIC_KEY_LEN], ()> {
    let mut signer_commitment = [0u8; EC_PUBLIC_KEY_LEN];
    match unsafe {
        bitbox02_sys::keystore_secp256k1_nonce_commit(
            secp.ctx().as_ptr().cast(),
            private_key.as_ptr(),
            msg.as_ptr(),
            host_commitment.as_ptr(),
            signer_commitment.as_mut_ptr(),
        )
    } {
        true => Ok(signer_commitment),
        false => Err(()),
    }
}

pub fn _encrypt_and_store_seed(seed: &[u8], password: &str) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::keystore_encrypt_and_store_seed(
            seed.as_ptr(),
            seed.len(),
            crate::util::str_to_cstr_vec(password)
                .unwrap()
                .as_ptr()
                .cast(),
        )
    } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        err => Err(err.into()),
    }
}

#[cfg(feature = "testing")]
pub fn mock_unlocked(seed: &[u8]) {
    unsafe { bitbox02_sys::keystore_mock_unlocked(seed.as_ptr(), seed.len() as _) }
}
