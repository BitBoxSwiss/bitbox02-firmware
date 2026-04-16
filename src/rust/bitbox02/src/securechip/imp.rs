// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec::Vec};
use bitbox_securechip::{Error, Model, PasswordStretchAlgo, atecc, optiga};
use core::ffi::c_int;
use util::cell::SyncCell;
use zeroize::Zeroizing;

#[derive(Copy, Clone)]
enum Backend {
    Atecc,
    Optiga,
}

static BACKEND: SyncCell<Option<Backend>> = SyncCell::new(None);

fn backend() -> Backend {
    BACKEND.read().unwrap()
}

pub fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::attestation_sign(challenge, signature),
        Backend::Optiga => optiga::attestation_sign(challenge, signature),
    }
}

pub fn random() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => atecc::random(),
        Backend::Optiga => optiga::random(),
    }
}

pub async fn monotonic_increments_remaining() -> Result<u32, ()> {
    match backend() {
        Backend::Atecc => atecc::monotonic_increments_remaining(),
        Backend::Optiga => optiga::monotonic_increments_remaining().await,
    }
}

pub fn reset_keys() -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::reset_keys(),
        Backend::Optiga => optiga::reset_keys(),
    }
}

pub fn init_new_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    match backend() {
        Backend::Atecc => atecc::init_new_password(password, password_stretch_algo),
        Backend::Optiga => optiga::init_new_password(password, password_stretch_algo),
    }
}

pub fn stretch_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    match backend() {
        Backend::Atecc => atecc::stretch_password(password, password_stretch_algo),
        Backend::Optiga => optiga::stretch_password(password, password_stretch_algo),
    }
}

/// Perform the secure chip KDF with the message in `msg` and return the zeroizing 32-byte
/// result.
pub fn kdf(msg: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error> {
    match backend() {
        Backend::Atecc => atecc::kdf(msg),
        Backend::Optiga => optiga::kdf(msg),
    }
}

#[cfg(feature = "app-u2f")]
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::u2f_counter_set(counter),
        Backend::Optiga => optiga::u2f_counter_set(counter),
    }
}

pub fn model() -> Result<Model, ()> {
    match backend() {
        Backend::Atecc => atecc::model(),
        Backend::Optiga => optiga::model(),
    }
}

/// Discovers which secure chip is present and selects the matching backend for subsequent
/// `rust_securechip_*` calls.
///
/// Returns `true` on success.
#[unsafe(no_mangle)]
pub extern "C" fn rust_securechip_init() -> bool {
    BACKEND.write(Some(match crate::memory::get_securechip_type() {
        Ok(crate::memory::SecurechipType::Optiga) => Backend::Optiga,
        Ok(crate::memory::SecurechipType::Atecc) | Err(()) => Backend::Atecc,
    }));
    true
}

/// Initializes the backend-specific secure chip communication.
///
/// On the first successful call, the selected backend may also configure and lock the secure
/// chip as needed.
///
/// Returns `0` on success. Negative values are [`SecureChipError`] codes. Positive values are
/// backend-specific status codes from CryptoAuthLib or the Optiga library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_setup(
    ifs: *const bitbox_securechip_sys::securechip_interface_functions_t,
) -> c_int {
    match backend() {
        Backend::Atecc => unsafe { bitbox_securechip_sys::atecc_setup(ifs) },
        Backend::Optiga => unsafe { bitbox_securechip_sys::optiga_setup(ifs) },
    }
}

/// Resets the secure-chip objects involved in password stretching.
#[unsafe(no_mangle)]
pub extern "C" fn rust_securechip_reset_keys() -> bool {
    match backend() {
        Backend::Atecc => atecc::reset_keys(),
        Backend::Optiga => optiga::reset_keys(),
    }
    .is_ok()
}

/// Generates a new device attestation key and writes the public key to `pubkey_out`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_gen_attestation_key(pubkey_out: *mut u8) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox_securechip_sys::atecc_gen_attestation_key(pubkey_out) },
        Backend::Optiga => unsafe { bitbox_securechip_sys::optiga_gen_attestation_key(pubkey_out) },
    }
}

/// Fills `rand_out` with 32 bytes of randomness from the secure chip.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_random(rand_out: *mut u8) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox_securechip_sys::atecc_random(rand_out) == 0 },
        Backend::Optiga => unsafe { bitbox_securechip_sys::optiga_random(rand_out) == 0 },
    }
}

/// Sets the U2F counter to `counter`.
///
/// This is intended for initialization only.
#[unsafe(no_mangle)]
pub extern "C" fn rust_securechip_u2f_counter_set(counter: u32) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox_securechip_sys::atecc_u2f_counter_set(counter) },
        Backend::Optiga => unsafe { bitbox_securechip_sys::optiga_u2f_counter_set(counter) },
    }
}

#[cfg(feature = "app-u2f")]
/// Increments the U2F counter and writes the current value to `counter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_u2f_counter_inc(counter: *mut u32) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox_securechip_sys::atecc_u2f_counter_inc(counter) },
        Backend::Optiga => unsafe { bitbox_securechip_sys::optiga_u2f_counter_inc(counter) },
    }
}
