// SPDX-License-Identifier: Apache-2.0

use super::*;
use core::ffi::{c_char, c_int};
use util::cell::SyncCell;

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
    match unsafe { attestation_sign_ffi(challenge.as_ptr(), signature.as_mut_ptr()) } {
        true => Ok(()),
        false => Err(()),
    }
}

unsafe fn attestation_sign_ffi(challenge: *const u8, signature_out: *mut u8) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_attestation_sign(challenge, signature_out) },
        Backend::Optiga => unsafe {
            bitbox02_sys::optiga_attestation_sign(challenge, signature_out)
        },
    }
}

pub fn monotonic_increments_remaining() -> Result<u32, ()> {
    let mut result = 0u32;
    match unsafe { monotonic_increments_remaining_ffi(&mut result) } {
        true => Ok(result),
        false => Err(()),
    }
}

unsafe fn monotonic_increments_remaining_ffi(remaining_out: *mut u32) -> bool {
    match backend() {
        Backend::Atecc => unsafe {
            bitbox02_sys::atecc_monotonic_increments_remaining(remaining_out)
        },
        Backend::Optiga => unsafe {
            bitbox02_sys::optiga_monotonic_increments_remaining(remaining_out)
        },
    }
}

pub fn reset_keys() -> Result<(), ()> {
    match reset_keys_ffi() {
        true => Ok(()),
        false => Err(()),
    }
}

fn reset_keys_ffi() -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_reset_keys() },
        Backend::Optiga => unsafe { bitbox02_sys::optiga_reset_keys() },
    }
}

pub fn init_new_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    let password = util::strings::str_to_cstr_vec_zeroizing(password)
        .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_INVALID_ARGS))?;
    let mut stretched = Zeroizing::new(vec![0u8; 32]);
    let status = unsafe {
        init_new_password_ffi(
            password.as_ptr().cast(),
            password_stretch_algo,
            stretched.as_mut_ptr(),
        )
    };
    if status == 0 {
        Ok(stretched)
    } else {
        Err(Error::from_status(status))
    }
}

unsafe fn init_new_password_ffi(
    password: *const c_char,
    password_stretch_algo: PasswordStretchAlgo,
    stretched_out: *mut u8,
) -> c_int {
    match backend() {
        Backend::Atecc => unsafe {
            bitbox02_sys::atecc_init_new_password(password, password_stretch_algo, stretched_out)
        },
        Backend::Optiga => unsafe {
            bitbox02_sys::optiga_init_new_password(password, password_stretch_algo, stretched_out)
        },
    }
}

pub fn stretch_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    let password = util::strings::str_to_cstr_vec_zeroizing(password)
        .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_INVALID_ARGS))?;
    let mut stretched = Zeroizing::new(vec![0u8; 32]);
    let status = unsafe {
        stretch_password_ffi(
            password.as_ptr().cast(),
            password_stretch_algo,
            stretched.as_mut_ptr(),
        )
    };
    if status == 0 {
        Ok(stretched)
    } else {
        Err(Error::from_status(status))
    }
}

unsafe fn stretch_password_ffi(
    password: *const c_char,
    password_stretch_algo: PasswordStretchAlgo,
    stretched_out: *mut u8,
) -> c_int {
    match backend() {
        Backend::Atecc => unsafe {
            bitbox02_sys::atecc_stretch_password(password, password_stretch_algo, stretched_out)
        },
        Backend::Optiga => unsafe {
            bitbox02_sys::optiga_stretch_password(password, password_stretch_algo, stretched_out)
        },
    }
}

/// Perform the secure chip KDF with the message in `msg` and return the zeroizing 32-byte
/// result.
pub fn kdf(msg: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error> {
    let mut result = Zeroizing::new(vec![0u8; 32]);
    let status = unsafe { kdf_ffi(msg.as_ptr(), msg.len(), result.as_mut_ptr()) };
    if status == 0 {
        Ok(result)
    } else {
        Err(Error::from_status(status))
    }
}

unsafe fn kdf_ffi(msg: *const u8, len: usize, kdf_out: *mut u8) -> c_int {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_kdf(msg, len, kdf_out) },
        Backend::Optiga => unsafe { bitbox02_sys::optiga_kdf_external(msg, len, kdf_out) },
    }
}

#[cfg(feature = "app-u2f")]
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    match u2f_counter_set_ffi(counter) {
        true => Ok(()),
        false => Err(()),
    }
}

fn u2f_counter_set_ffi(counter: u32) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_u2f_counter_set(counter) },
        Backend::Optiga => unsafe { bitbox02_sys::optiga_u2f_counter_set(counter) },
    }
}

pub fn model() -> Result<Model, ()> {
    let mut model = core::mem::MaybeUninit::uninit();
    match unsafe { model_ffi(model.as_mut_ptr()) } {
        true => Ok(unsafe { model.assume_init() }),
        false => Err(()),
    }
}

unsafe fn model_ffi(model_out: *mut Model) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_model(model_out) },
        Backend::Optiga => unsafe { bitbox02_sys::optiga_model(model_out) },
    }
}

unsafe fn gen_attestation_key_ffi(pubkey_out: *mut u8) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_gen_attestation_key(pubkey_out) },
        Backend::Optiga => unsafe { bitbox02_sys::optiga_gen_attestation_key(pubkey_out) },
    }
}

unsafe fn random_ffi(rand_out: *mut u8) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_random(rand_out) },
        Backend::Optiga => unsafe { bitbox02_sys::optiga_random(rand_out) },
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
    ifs: *const bitbox02_sys::securechip_interface_functions_t,
) -> c_int {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_setup(ifs) },
        Backend::Optiga => unsafe { bitbox02_sys::optiga_setup(ifs) },
    }
}

/// Resets the secure-chip objects involved in password stretching.
#[unsafe(no_mangle)]
pub extern "C" fn rust_securechip_reset_keys() -> bool {
    reset_keys_ffi()
}

/// Generates a new device attestation key and writes the public key to `pubkey_out`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_gen_attestation_key(pubkey_out: *mut u8) -> bool {
    unsafe { gen_attestation_key_ffi(pubkey_out) }
}

/// Fills `rand_out` with 32 bytes of randomness from the secure chip.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_random(rand_out: *mut u8) -> bool {
    unsafe { random_ffi(rand_out) }
}

/// Sets the U2F counter to `counter`.
///
/// This is intended for initialization only.
#[unsafe(no_mangle)]
pub extern "C" fn rust_securechip_u2f_counter_set(counter: u32) -> bool {
    u2f_counter_set_ffi(counter)
}

#[cfg(feature = "app-u2f")]
/// Increments the U2F counter and writes the current value to `counter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_u2f_counter_inc(counter: *mut u32) -> bool {
    match backend() {
        Backend::Atecc => unsafe { bitbox02_sys::atecc_u2f_counter_inc(counter) },
        Backend::Optiga => unsafe { bitbox02_sys::optiga_u2f_counter_inc(counter) },
    }
}
