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

use alloc::vec::Vec;
use zeroize::Zeroizing;

pub use bitbox02_sys::securechip_error_t as SecureChipError;
pub use bitbox02_sys::securechip_model_t as Model;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SecureChip(SecureChipError),
    Status(i32),
}

// Keep in sync with securechip.h's securechip_error_t.
const SECURECHIP_ERRORS: [SecureChipError; 15] = [
    // Errors common to any securechip implementation
    SecureChipError::SC_ERR_IFS,
    SecureChipError::SC_ERR_INVALID_ARGS,
    SecureChipError::SC_ERR_CONFIG_MISMATCH,
    SecureChipError::SC_ERR_SALT,
    SecureChipError::SC_ERR_INCORRECT_PASSWORD,
    // Errors specific to the ATECC
    SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG,
    SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_DATA,
    SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_IO,
    SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_AUTH,
    SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_ENC,
    SecureChipError::SC_ATECC_ERR_RESET_KEYS,
    // Errors specific to the Optiga
    SecureChipError::SC_OPTIGA_ERR_CREATE,
    SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_METADATA,
    SecureChipError::SC_OPTIGA_ERR_PAL,
    SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
];

fn securechip_error_from_status(status: i32) -> Option<SecureChipError> {
    SECURECHIP_ERRORS
        .iter()
        .copied()
        .find(|err| *err as i32 == status)
}

impl Error {
    fn from_status(status: i32) -> Self {
        if status < 0 {
            match securechip_error_from_status(status) {
                Some(err) => Error::SecureChip(err),
                None => Error::Status(status),
            }
        } else {
            Error::Status(status)
        }
    }
}

pub fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::securechip_attestation_sign(challenge.as_ptr(), signature.as_mut_ptr())
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn monotonic_increments_remaining() -> Result<u32, ()> {
    let mut result: u32 = 0;
    match unsafe { bitbox02_sys::securechip_monotonic_increments_remaining(&mut result as _) } {
        true => Ok(result),
        false => Err(()),
    }
}

pub fn reset_keys() -> Result<(), ()> {
    match unsafe { bitbox02_sys::securechip_reset_keys() } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn init_new_password(password: &str) -> Result<(), Error> {
    let password = crate::util::str_to_cstr_vec_zeroizing(password)
        .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_INVALID_ARGS))?;
    let status = unsafe { bitbox02_sys::securechip_init_new_password(password.as_ptr().cast()) };
    if status == 0 {
        Ok(())
    } else {
        Err(Error::from_status(status))
    }
}

pub fn stretch_password(password: &str) -> Result<Zeroizing<Vec<u8>>, Error> {
    let password = crate::util::str_to_cstr_vec_zeroizing(password)
        .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_INVALID_ARGS))?;
    let mut stretched = Zeroizing::new(vec![0u8; 32]);
    let status = unsafe {
        bitbox02_sys::securechip_stretch_password(password.as_ptr().cast(), stretched.as_mut_ptr())
    };
    if status == 0 {
        Ok(stretched)
    } else {
        Err(Error::from_status(status))
    }
}

/// Perform the secure chip KDF with the message in `msg` and return the zeroizing 32-byte result.
pub fn kdf(msg: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error> {
    let mut result = Zeroizing::new(vec![0u8; 32]);
    let status =
        unsafe { bitbox02_sys::securechip_kdf(msg.as_ptr(), msg.len(), result.as_mut_ptr()) };
    if status == 0 {
        Ok(result)
    } else {
        Err(Error::from_status(status))
    }
}

#[cfg(feature = "app-u2f")]
#[cfg(not(feature = "testing"))]
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    match unsafe { bitbox02_sys::securechip_u2f_counter_set(counter) } {
        true => Ok(()),
        false => Err(()),
    }
}

#[cfg(feature = "app-u2f")]
#[cfg(feature = "testing")]
pub fn u2f_counter_set(_counter: u32) -> Result<(), ()> {
    Ok(())
}

pub fn model() -> Result<Model, ()> {
    let mut ver = core::mem::MaybeUninit::uninit();
    match unsafe { bitbox02_sys::securechip_model(ver.as_mut_ptr()) } {
        true => Ok(unsafe { ver.assume_init() }),
        false => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex_lit::hex;

    #[test]
    fn test_kdf() {
        // Matches the deterministic HMAC result returned by test/hardware-fakes/src/fake_securechip.c.
        let result = kdf(b"stub input").unwrap();
        let expected = hex!("3d7caa0407f18f6b15a6202843c883f326d614996df67940af210d91aff5b9c8");
        assert_eq!(result.as_slice(), expected.as_slice());
    }
}
