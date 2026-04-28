// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::extra_unused_type_parameters)]

use alloc::boxed::Box;
use bitbox_securechip::{Error, Model, PasswordStretchAlgo, SecureChipError};
use hex_lit::hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::Zeroizing;

const PASSWORD_STRETCH_KEY: &[u8] = b"unit-test";
const KDF_KEY: [u8; 32] = hex!("d2e1e6b18b6c6b08433edbc1d168c1a0043774a4221877e79ed56684be5ac01b");

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
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

pub async fn attestation_sign<Timer: bitbox_hal::timer::Timer>(
    _memory: &mut impl bitbox_hal::Memory,
    _challenge: &[u8; 32],
    _signature: &mut [u8; 64],
) -> Result<(), ()> {
    Err(())
}

pub async fn random<Timer: bitbox_hal::timer::Timer>() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    Ok(Box::new(Zeroizing::new([0u8; 32])))
}

pub async fn monotonic_increments_remaining<Timer: bitbox_hal::timer::Timer>() -> Result<u32, ()> {
    Ok(1)
}

pub async fn reset_keys<Timer: bitbox_hal::timer::Timer>(
    _random: &mut impl bitbox_hal::Random,
    _memory: &mut impl bitbox_hal::Memory,
) -> Result<(), ()> {
    Ok(())
}

pub async fn init_new_password<Timer: bitbox_hal::timer::Timer>(
    _random: &mut impl bitbox_hal::Random,
    _memory: &mut impl bitbox_hal::Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    if password_stretch_algo != PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1 {
        return Err(Error::SecureChip(
            SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
        ));
    }
    Ok(Box::new(Zeroizing::new(hmac_sha256(
        PASSWORD_STRETCH_KEY,
        password.as_bytes(),
    ))))
}

pub async fn stretch_password<Timer: bitbox_hal::timer::Timer>(
    _memory: &mut impl bitbox_hal::Memory,
    password: &str,
    _password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    Ok(Box::new(Zeroizing::new(hmac_sha256(
        PASSWORD_STRETCH_KEY,
        password.as_bytes(),
    ))))
}

/// Perform the secure chip KDF with the message in `msg` and return the zeroizing 32-byte
/// result.
pub async fn kdf<Timer: bitbox_hal::timer::Timer>(
    _memory: &mut impl bitbox_hal::Memory,
    msg: &[u8; 32],
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    Ok(Box::new(Zeroizing::new(hmac_sha256(&KDF_KEY, msg))))
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub async fn u2f_counter_set<Timer: bitbox_hal::timer::Timer>(
    _random: &mut impl bitbox_hal::Random,
    _memory: &mut impl bitbox_hal::Memory,
    counter: u32,
) -> Result<(), ()> {
    U2F_COUNTER.write(counter);
    Ok(())
}

#[cfg(feature = "app-u2f")]
pub async fn u2f_counter_inc<Timer: bitbox_hal::timer::Timer>(
    _random: &mut impl bitbox_hal::Random,
    _memory: &mut impl bitbox_hal::Memory,
) -> Result<u32, ()> {
    let current = U2F_COUNTER.read().wrapping_add(1);
    U2F_COUNTER.write(current);
    Ok(current)
}

pub async fn model<Timer: bitbox_hal::timer::Timer>() -> Result<Model, ()> {
    Ok(Model::ATECC_ATECC608B)
}

#[cfg(feature = "app-u2f")]
/// Increments the fake host-side U2F counter and writes the new value to `counter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_u2f_counter_inc(counter: *mut u32) -> bool {
    assert!(!counter.is_null());
    let mut random = crate::hal::random::BitBox02Random;
    let mut memory = crate::hal::memory::BitBox02Memory;
    match util::bb02_async::block_on(u2f_counter_inc::<crate::hal::timer::BitBox02Timer>(
        &mut random,
        &mut memory,
    )) {
        Ok(current) => {
            unsafe {
                *counter = current;
            }
            true
        }
        Err(()) => false,
    }
}
