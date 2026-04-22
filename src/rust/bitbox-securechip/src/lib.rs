// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

use bitbox_securechip_sys as ffi;

pub mod atecc;
pub mod optiga;

pub use ffi::securechip_error_t as SecureChipError;
pub use ffi::securechip_model_t as Model;
pub use ffi::securechip_password_stretch_algo_t as PasswordStretchAlgo;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SecureChip(SecureChipError),
    Status(i32),
}

// Keep in sync with securechip.h's securechip_error_t.
const SECURECHIP_ERRORS: [SecureChipError; 17] = [
    // Errors common to any securechip implementation
    SecureChipError::SC_ERR_IFS,
    SecureChipError::SC_ERR_INVALID_ARGS,
    SecureChipError::SC_ERR_CONFIG_MISMATCH,
    SecureChipError::SC_ERR_SALT,
    SecureChipError::SC_ERR_INCORRECT_PASSWORD,
    SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
    SecureChipError::SC_ERR_MEMORY,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_from_status() {
        let cases = [
            SecureChipError::SC_ERR_IFS,
            SecureChipError::SC_ERR_INVALID_ARGS,
            SecureChipError::SC_ERR_CONFIG_MISMATCH,
            SecureChipError::SC_ERR_SALT,
            SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
            SecureChipError::SC_ERR_MEMORY,
            SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG,
            SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_DATA,
            SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_IO,
            SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_AUTH,
            SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_ENC,
            SecureChipError::SC_ATECC_ERR_RESET_KEYS,
            SecureChipError::SC_OPTIGA_ERR_CREATE,
            SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_METADATA,
            SecureChipError::SC_OPTIGA_ERR_PAL,
            SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
        ];

        for error in cases {
            assert_eq!(Error::from_status(error as i32), Error::SecureChip(error),);
        }

        assert_eq!(Error::from_status(7), Error::Status(7));
        assert_eq!(Error::from_status(-9999), Error::Status(-9999));
    }
}
