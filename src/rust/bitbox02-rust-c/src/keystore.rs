// Copyright 2025 Shift Cryptosecurity AG
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
use crate::util::Bytes;

use bitbox02::keystore::keystore_error_t;
use bitbox02_rust::keystore::Error;

#[no_mangle]
pub unsafe extern "C" fn rust_keystore_encrypt_and_store_seed(
    seed: Bytes,
    password: *const core::ffi::c_char,
) -> u8 {
    let password_str = core::ffi::CStr::from_ptr(password).to_str().unwrap();
    match bitbox02_rust::keystore::encrypt_and_store_seed(seed.as_ref(), password_str) {
        Ok(()) => 0,
        Err(err) => match err {
            // In C this err didn't exist and ERR_MEMORY was used.
            Error::AlreadyInitialized => keystore_error_t::KEYSTORE_ERR_MEMORY as _,
            Error::Memory => keystore_error_t::KEYSTORE_ERR_MEMORY as _,
            Error::SeedSize => keystore_error_t::KEYSTORE_ERR_SEED_SIZE as _,
            Error::SecureChip => keystore_error_t::KEYSTORE_ERR_SECURECHIP as _,
            Error::IncorrectPassword => keystore_error_t::KEYSTORE_ERR_INCORRECT_PASSWORD as _,
        },
    }
}
