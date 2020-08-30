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

use crate::password::Password;
use bitbox02_sys::keystore_error_t;

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword { remaining_attempts: u8 },
    Unknown,
}

/// All C keystore functions needed in Rust. All methods have a default implementation to make
/// mocking it easier.
pub trait Keystore {
    fn is_locked() -> bool {
        panic!("not implemented")
    }
    fn unlock(_password: &Password) -> Result<(), Error> {
        panic!("not implemented")
    }
    fn unlock_bip39(_mnemonic_passphrase: &Password) -> Result<(), Error> {
        panic!("not implemented")
    }
    fn create_and_store_seed(_password: &Password, _host_entropy: &[u8; 32]) -> bool {
        panic!("not implemented")
    }
}

/// Exposes the C functions safely to Rust.
pub enum CKeyStore {}

impl Keystore for CKeyStore {
    fn is_locked() -> bool {
        unsafe { bitbox02_sys::keystore_is_locked() }
    }

    fn unlock(password: &Password) -> Result<(), Error> {
        let mut remaining_attempts: u8 = 0;
        match unsafe { bitbox02_sys::keystore_unlock(password.as_cstr(), &mut remaining_attempts) }
        {
            keystore_error_t::KEYSTORE_OK => Ok(()),
            keystore_error_t::KEYSTORE_ERR_INCORRECT_PASSWORD => {
                Err(Error::IncorrectPassword { remaining_attempts })
            }
            keystore_error_t::KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED => Err(Error::Unknown),
            keystore_error_t::KEYSTORE_ERR_GENERIC => Err(Error::Unknown),
        }
    }

    fn unlock_bip39(mnemonic_passphrase: &Password) -> Result<(), Error> {
        if unsafe { bitbox02_sys::keystore_unlock_bip39(mnemonic_passphrase.as_cstr()) } {
            Ok(())
        } else {
            Err(Error::CannotUnlockBIP39)
        }
    }

    fn create_and_store_seed(password: &Password, host_entropy: &[u8; 32]) -> bool {
        unsafe {
            bitbox02_sys::keystore_create_and_store_seed(password.as_cstr(), host_entropy.as_ptr())
        }
    }
}
