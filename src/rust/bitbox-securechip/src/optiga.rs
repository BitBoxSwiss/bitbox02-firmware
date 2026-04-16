// SPDX-License-Identifier: Apache-2.0

use crate::{Error, Model, PasswordStretchAlgo, SecureChipError};
use alloc::boxed::Box;
use zeroize::Zeroizing;

mod ops;

const OID_COUNTER: u16 = bitbox_securechip_sys::OID_COUNTER as u16;
const MONOTONIC_COUNTER_MAX_USE: u32 = bitbox_securechip_sys::MONOTONIC_COUNTER_MAX_USE;
const OID_HMAC: u16 = bitbox_securechip_sys::OID_HMAC as u16;
const KDF_LEN: usize = 32;
const OPTIGA_HMAC_SHA_256: bitbox_securechip_sys::optiga_hmac_type_t =
    bitbox_securechip_sys::optiga_hmac_type::OPTIGA_HMAC_SHA_256;

pub fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    match unsafe {
        bitbox_securechip_sys::optiga_attestation_sign(challenge.as_ptr(), signature.as_mut_ptr())
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn random() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut result = Box::new(Zeroizing::new([0u8; 32]));
    let status = unsafe { bitbox_securechip_sys::optiga_random(result.as_mut_ptr()) };
    if status == 0 {
        Ok(result)
    } else {
        Err(Error::from_status(status))
    }
}

pub async fn monotonic_increments_remaining() -> Result<u32, ()> {
    let mut counter_buf = [0; 4];
    ops::util_read_data(OID_COUNTER, 0, &mut counter_buf)
        .await
        .map_err(|_| ())?;
    let counter = u32::from_be_bytes(counter_buf);
    if counter > MONOTONIC_COUNTER_MAX_USE {
        panic!("optiga monotonic counter larger than max");
    }
    Ok(MONOTONIC_COUNTER_MAX_USE - counter)
}

pub fn reset_keys() -> Result<(), ()> {
    match unsafe { bitbox_securechip_sys::optiga_reset_keys() } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn init_new_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let password = util::strings::str_to_cstr_vec_zeroizing(password)
        .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_INVALID_ARGS))?;
    let mut stretched = Box::new(Zeroizing::new([0u8; 32]));
    let status = unsafe {
        bitbox_securechip_sys::optiga_init_new_password(
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
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let password = util::strings::str_to_cstr_vec_zeroizing(password)
        .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_INVALID_ARGS))?;
    let mut stretched = Box::new(Zeroizing::new([0u8; 32]));
    let status = unsafe {
        bitbox_securechip_sys::optiga_stretch_password(
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

pub async fn kdf(msg: &[u8; KDF_LEN]) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut result = Box::new(Zeroizing::new([0u8; 32]));
    ops::crypt_hmac(OPTIGA_HMAC_SHA_256, OID_HMAC, msg, result.as_mut()).await?;
    Ok(result)
}

#[cfg(feature = "app-u2f")]
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    match unsafe { bitbox_securechip_sys::optiga_u2f_counter_set(counter) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn model() -> Result<Model, ()> {
    Ok(Model::OPTIGA_TRUST_M_V3)
}
