// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::vec::Vec;
use zeroize::Zeroizing;

pub use bitbox_securechip_sys::securechip_error_t as SecureChipError;
pub use bitbox_securechip_sys::securechip_model_t as Model;
pub use bitbox_securechip_sys::securechip_password_stretch_algo_t as PasswordStretchAlgo;

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

#[cfg_attr(
    any(
        test,
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    ),
    path = "securechip/imp_fake.rs"
)]
mod imp;

#[cfg(not(any(
    test,
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
mod atecc;
#[cfg(not(any(
    test,
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
mod optiga;

/// Signs a 32-byte attestation challenge and writes the raw 64-byte P-256 signature to
/// `signature`.
pub fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    imp::attestation_sign(challenge, signature)
}

/// Returns the remaining number of secure-chip monotonic counter increments.
pub fn monotonic_increments_remaining() -> Result<u32, ()> {
    imp::monotonic_increments_remaining()
}

/// Resets the secure-chip objects involved in password stretching.
pub fn reset_keys() -> Result<(), ()> {
    imp::reset_keys()
}

/// Prepares the secure chip for a new password and returns the stretched password.
///
/// This reinitializes the secure-chip state used for password derivation and returns the same
/// 32-byte value as [`stretch_password`] for the same `password` and
/// `password_stretch_algo`, but may require fewer secure-chip operations.
pub fn init_new_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    imp::init_new_password(password, password_stretch_algo)
}

/// Stretches `password` using secrets stored in the secure chip.
///
/// The returned value is always 32 bytes long. Calling this function increments the relevant
/// secure-chip monotonic counter.
pub fn stretch_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    imp::stretch_password(password, password_stretch_algo)
}

/// Runs the secure-chip KDF with `msg` and returns the zeroizing 32-byte result.
///
/// This must not increment a monotonic counter.
///
/// `msg` must be at most 127 bytes long.
pub fn kdf(msg: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error> {
    imp::kdf(msg)
}

#[cfg(feature = "app-u2f")]
/// Sets the U2F counter to `counter`.
///
/// This is intended for initialization only.
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    imp::u2f_counter_set(counter)
}

/// Returns the detected secure-chip model.
pub fn model() -> Result<Model, ()> {
    imp::model()
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex_lit::hex;

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

    #[test]
    fn test_kdf() {
        // Matches the deterministic host/test fake securechip KDF.
        let result = kdf(b"stub input").unwrap();
        let expected = hex!("3d7caa0407f18f6b15a6202843c883f326d614996df67940af210d91aff5b9c8");
        assert_eq!(result.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_init_new_password_invalid_password_stretch_algo() {
        assert_eq!(
            init_new_password(
                "password",
                PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0
            ),
            Err(Error::SecureChip(
                SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
            )),
        );
    }
}
