// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::vec::Vec;
use zeroize::Zeroizing;

pub use bitbox_securechip::{Error, Model, PasswordStretchAlgo, SecureChipError};

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
