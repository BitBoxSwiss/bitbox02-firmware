// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use bitbox_hal::{Memory, Random};
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

pub async fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::attestation_sign(challenge, signature).await,
        Backend::Optiga => optiga::attestation_sign(challenge, signature).await,
    }
}

pub async fn random() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => atecc::random(),
        Backend::Optiga => optiga::random().await,
    }
}

pub async fn monotonic_increments_remaining() -> Result<u32, ()> {
    match backend() {
        Backend::Atecc => atecc::monotonic_increments_remaining(),
        Backend::Optiga => optiga::monotonic_increments_remaining().await,
    }
}

pub async fn reset_keys(random: &mut impl Random, memory: &mut impl Memory) -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::reset_keys(),
        Backend::Optiga => optiga::reset_keys(random, memory).await,
    }
}

pub async fn init_new_password(
    random: &mut impl Random,
    memory: &mut impl Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => atecc::init_new_password(memory, password, password_stretch_algo),
        Backend::Optiga => {
            optiga::init_new_password(random, memory, password, password_stretch_algo).await
        }
    }
}

pub async fn stretch_password(
    memory: &mut impl Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => atecc::stretch_password(memory, password, password_stretch_algo),
        Backend::Optiga => optiga::stretch_password(memory, password, password_stretch_algo).await,
    }
}

/// Perform the secure chip KDF with the message in `msg` and return the zeroizing 32-byte
/// result.
pub async fn kdf(msg: &[u8; 32]) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => atecc::kdf(msg),
        Backend::Optiga => optiga::kdf(msg).await,
    }
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub async fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::u2f_counter_set(counter),
        Backend::Optiga => optiga::u2f_counter_set(counter).await,
    }
}

#[cfg(feature = "app-u2f")]
pub async fn u2f_counter_inc() -> Result<u32, ()> {
    match backend() {
        Backend::Atecc => atecc::u2f_counter_inc(),
        Backend::Optiga => optiga::u2f_counter_inc().await,
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
    let mut random = crate::hal::random::BitBox02Random;
    let mut memory = crate::hal::memory::BitBox02Memory;
    util::bb02_async::block_on(reset_keys(&mut random, &mut memory)).is_ok()
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
    match util::bb02_async::block_on(random()) {
        Ok(random) => {
            unsafe {
                core::ptr::copy_nonoverlapping(random.as_ptr(), rand_out, 32);
            }
            true
        }
        Err(_) => false,
    }
}

/// Sets the U2F counter to `counter`.
///
/// This is intended for initialization only.
#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_securechip_u2f_counter_set(counter: u32) -> bool {
    util::bb02_async::block_on(u2f_counter_set(counter)).is_ok()
}

#[cfg(feature = "app-u2f")]
/// Increments the U2F counter and writes the new value to `counter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_u2f_counter_inc(counter: *mut u32) -> bool {
    assert!(!counter.is_null());
    match util::bb02_async::block_on(u2f_counter_inc()) {
        Ok(current) => {
            unsafe {
                *counter = current;
            }
            true
        }
        Err(()) => false,
    }
}
