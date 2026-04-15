// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec::Vec};
use bitbox_securechip::{Error, Model, PasswordStretchAlgo, SecureChipError};
use hex_lit::hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::Zeroizing;

const PASSWORD_STRETCH_KEY: &[u8] = b"unit-test";
const KDF_KEY: [u8; 32] = hex!("d2e1e6b18b6c6b08433edbc1d168c1a0043774a4221877e79ed56684be5ac01b");

#[cfg(feature = "app-u2f")]
static U2F_COUNTER: util::cell::SyncCell<u32> = util::cell::SyncCell::new(0);

type HmacSha256 = Hmac<Sha256>;

fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(data);
    let result = mac.finalize().into_bytes();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

pub fn attestation_sign(_challenge: &[u8; 32], _signature: &mut [u8; 64]) -> Result<(), ()> {
    Err(())
}

pub fn random() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    Ok(Box::new(Zeroizing::new([0u8; 32])))
}

pub fn monotonic_increments_remaining() -> Result<u32, ()> {
    Ok(1)
}

pub fn reset_keys() -> Result<(), ()> {
    Ok(())
}

pub fn init_new_password(
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    if password_stretch_algo != PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1 {
        return Err(Error::SecureChip(
            SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
        ));
    }
    Ok(Zeroizing::new(
        hmac_sha256(PASSWORD_STRETCH_KEY, password.as_bytes()).to_vec(),
    ))
}

pub fn stretch_password(
    password: &str,
    _password_stretch_algo: PasswordStretchAlgo,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    Ok(Zeroizing::new(
        hmac_sha256(PASSWORD_STRETCH_KEY, password.as_bytes()).to_vec(),
    ))
}

/// Perform the secure chip KDF with the message in `msg` and return the zeroizing 32-byte
/// result.
pub fn kdf(msg: &[u8; 32]) -> Result<Zeroizing<Vec<u8>>, Error> {
    Ok(Zeroizing::new(hmac_sha256(&KDF_KEY, msg).to_vec()))
}

#[cfg(feature = "app-u2f")]
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    U2F_COUNTER.write(counter);
    Ok(())
}

pub fn model() -> Result<Model, ()> {
    Ok(Model::ATECC_ATECC608B)
}

#[cfg(feature = "app-u2f")]
/// Increments the fake host-side U2F counter and writes the current value to `counter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_u2f_counter_inc(counter: *mut u32) -> bool {
    assert!(!counter.is_null());
    let current = U2F_COUNTER.read();
    U2F_COUNTER.write(current.wrapping_add(1));
    unsafe { *counter = current };
    true
}
