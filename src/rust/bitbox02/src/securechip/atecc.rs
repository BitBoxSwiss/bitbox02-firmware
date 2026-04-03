// SPDX-License-Identifier: Apache-2.0

use super::*;
use alloc::vec::Vec;
use core::ffi::c_int;
use zeroize::Zeroizing;

pub fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::atecc_attestation_sign(challenge.as_ptr(), signature.as_mut_ptr())
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn monotonic_increments_remaining() -> Result<u32, ()> {
    let mut result = 0u32;
    match unsafe { bitbox02_sys::atecc_monotonic_increments_remaining(&mut result) } {
        true => Ok(result),
        false => Err(()),
    }
}

pub fn reset_keys() -> Result<(), ()> {
    match unsafe { bitbox02_sys::atecc_reset_keys() } {
        true => Ok(()),
        false => Err(()),
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
        bitbox02_sys::atecc_init_new_password(
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

pub fn stretch_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    let password = util::strings::str_to_cstr_vec_zeroizing(password)
        .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_INVALID_ARGS))?;
    let mut stretched = Zeroizing::new(vec![0u8; 32]);
    let status = unsafe {
        bitbox02_sys::atecc_stretch_password(
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

pub fn kdf(msg: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error> {
    let mut result = Zeroizing::new(vec![0u8; 32]);
    let status = unsafe { bitbox02_sys::atecc_kdf(msg.as_ptr(), msg.len(), result.as_mut_ptr()) };
    if status == 0 {
        Ok(result)
    } else {
        Err(Error::from_status(status))
    }
}

#[cfg(feature = "app-u2f")]
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    match u2f_counter_set_raw(counter) {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn model() -> Result<Model, ()> {
    let mut model = core::mem::MaybeUninit::uninit();
    match unsafe { bitbox02_sys::atecc_model(model.as_mut_ptr()) } {
        true => Ok(unsafe { model.assume_init() }),
        false => Err(()),
    }
}

pub(super) unsafe fn setup(ifs: *const bitbox02_sys::securechip_interface_functions_t) -> c_int {
    unsafe { bitbox02_sys::atecc_setup(ifs) }
}

pub(super) unsafe fn gen_attestation_key(pubkey_out: *mut u8) -> bool {
    unsafe { bitbox02_sys::atecc_gen_attestation_key(pubkey_out) }
}

pub(super) unsafe fn random(rand_out: *mut u8) -> bool {
    unsafe { bitbox02_sys::atecc_random(rand_out) }
}

pub(super) fn u2f_counter_set_raw(counter: u32) -> bool {
    unsafe { bitbox02_sys::atecc_u2f_counter_set(counter) }
}

#[cfg(feature = "app-u2f")]
pub(super) unsafe fn u2f_counter_inc(counter: *mut u32) -> bool {
    unsafe { bitbox02_sys::atecc_u2f_counter_inc(counter) }
}
