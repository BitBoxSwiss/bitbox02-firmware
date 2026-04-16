// SPDX-License-Identifier: Apache-2.0

use crate::{Error, Model, PasswordStretchAlgo, SecureChipError};
use alloc::{boxed::Box, vec, vec::Vec};
use zeroize::Zeroizing;

pub fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    match unsafe {
        bitbox_securechip_sys::atecc_attestation_sign(challenge.as_ptr(), signature.as_mut_ptr())
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn random() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut result = Box::new(Zeroizing::new([0u8; 32]));
    let status = unsafe { bitbox_securechip_sys::atecc_random(result.as_mut_ptr()) };
    if status == 0 {
        Ok(result)
    } else {
        Err(Error::from_status(status))
    }
}
pub fn monotonic_increments_remaining() -> Result<u32, ()> {
    let mut result = 0u32;
    match unsafe { bitbox_securechip_sys::atecc_monotonic_increments_remaining(&mut result) } {
        true => Ok(result),
        false => Err(()),
    }
}

pub fn reset_keys() -> Result<(), ()> {
    match unsafe { bitbox_securechip_sys::atecc_reset_keys() } {
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
        bitbox_securechip_sys::atecc_init_new_password(
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
        bitbox_securechip_sys::atecc_stretch_password(
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

pub fn kdf(msg: &[u8; 32]) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut result = Box::new(Zeroizing::new([0u8; 32]));
    let status =
        unsafe { bitbox_securechip_sys::atecc_kdf(msg.as_ptr(), msg.len(), result.as_mut_ptr()) };
    if status == 0 {
        Ok(result)
    } else {
        Err(Error::from_status(status))
    }
}

#[cfg(feature = "app-u2f")]
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    match unsafe { bitbox_securechip_sys::atecc_u2f_counter_set(counter) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn model() -> Result<Model, ()> {
    let mut model = core::mem::MaybeUninit::uninit();
    match unsafe { bitbox_securechip_sys::atecc_model(model.as_mut_ptr()) } {
        true => Ok(unsafe { model.assume_init() }),
        false => Err(()),
    }
}
