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

type DefaultTimer = crate::hal::timer::BitBox02Timer;

pub async fn attestation_sign<Timer: bitbox_hal::timer::Timer>(
    memory: &mut impl Memory,
    challenge: &[u8; 32],
    signature: &mut [u8; 64],
) -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::attestation_sign::<Timer>(memory, challenge, signature).await,
        Backend::Optiga => optiga::attestation_sign(challenge, signature).await,
    }
}

pub async fn random<Timer: bitbox_hal::timer::Timer>() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => atecc::random::<Timer>().await,
        Backend::Optiga => optiga::random().await,
    }
}

pub async fn monotonic_increments_remaining<Timer: bitbox_hal::timer::Timer>() -> Result<u32, ()> {
    match backend() {
        Backend::Atecc => atecc::monotonic_increments_remaining::<Timer>().await,
        Backend::Optiga => optiga::monotonic_increments_remaining().await,
    }
}

pub async fn reset_keys<Timer: bitbox_hal::timer::Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
) -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::reset_keys::<Timer>(random, memory).await,
        Backend::Optiga => optiga::reset_keys(random, memory).await,
    }
}

pub async fn init_new_password<Timer: bitbox_hal::timer::Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => {
            atecc::init_new_password::<Timer>(random, memory, password, password_stretch_algo).await
        }
        Backend::Optiga => {
            optiga::init_new_password(random, memory, password, password_stretch_algo).await
        }
    }
}

pub async fn stretch_password<Timer: bitbox_hal::timer::Timer>(
    memory: &mut impl Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => {
            atecc::stretch_password::<Timer>(memory, password, password_stretch_algo).await
        }
        Backend::Optiga => optiga::stretch_password(memory, password, password_stretch_algo).await,
    }
}

/// Perform the secure chip KDF with the message in `msg` and return the zeroizing 32-byte
/// result.
pub async fn kdf<Timer: bitbox_hal::timer::Timer>(
    memory: &mut impl Memory,
    msg: &[u8; 32],
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    match backend() {
        Backend::Atecc => atecc::kdf::<Timer>(memory, msg).await,
        Backend::Optiga => optiga::kdf(msg).await,
    }
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub async fn u2f_counter_set<Timer: bitbox_hal::timer::Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
    counter: u32,
) -> Result<(), ()> {
    match backend() {
        Backend::Atecc => atecc::u2f_counter_set::<Timer>(random, memory, counter).await,
        Backend::Optiga => optiga::u2f_counter_set(counter).await,
    }
}

#[cfg(feature = "app-u2f")]
pub async fn u2f_counter_inc<Timer: bitbox_hal::timer::Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
) -> Result<u32, ()> {
    match backend() {
        Backend::Atecc => atecc::u2f_counter_inc::<Timer>(random, memory).await,
        Backend::Optiga => optiga::u2f_counter_inc().await,
    }
}

pub async fn model<Timer: bitbox_hal::timer::Timer>() -> Result<Model, ()> {
    match backend() {
        Backend::Atecc => atecc::model::<Timer>().await,
        Backend::Optiga => optiga::model().await,
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
pub unsafe extern "C" fn rust_securechip_setup() -> c_int {
    match backend() {
        Backend::Atecc => {
            let mut memory = crate::hal::memory::BitBox02Memory;
            let mut io_protection_key = Box::new(Zeroizing::new([0u8; 32]));
            let mut auth_key = Box::new(Zeroizing::new([0u8; 32]));
            let mut encryption_key = Box::new(Zeroizing::new([0u8; 32]));
            memory.get_io_protection_key(io_protection_key.as_mut());
            memory.get_auth_key(auth_key.as_mut());
            memory.get_encryption_key(encryption_key.as_mut());
            unsafe {
                bitbox_securechip_sys::atecc_setup(
                    io_protection_key.as_ptr(),
                    auth_key.as_ptr(),
                    encryption_key.as_ptr(),
                )
            }
        }
        Backend::Optiga => unsafe { bitbox_securechip_sys::optiga_setup() },
    }
}

/// Resets the secure-chip objects involved in password stretching.
#[unsafe(no_mangle)]
pub extern "C" fn rust_securechip_reset_keys() -> bool {
    let mut random = crate::hal::random::BitBox02Random;
    let mut memory = crate::hal::memory::BitBox02Memory;
    util::bb02_async::block_on(reset_keys::<DefaultTimer>(&mut random, &mut memory)).is_ok()
}

/// Generates a new device attestation key and writes the public key to `pubkey_out`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_gen_attestation_key(pubkey_out: *mut u8) -> bool {
    match backend() {
        Backend::Atecc => {
            let mut memory = crate::hal::memory::BitBox02Memory;
            let mut auth_key = Box::new(Zeroizing::new([0u8; 32]));
            memory.get_auth_key(auth_key.as_mut());
            unsafe {
                bitbox_securechip_sys::atecc_gen_attestation_key(auth_key.as_ptr(), pubkey_out)
            }
        }
        Backend::Optiga => unsafe { bitbox_securechip_sys::optiga_gen_attestation_key(pubkey_out) },
    }
}

/// Fills `rand_out` with 32 bytes of randomness from the secure chip.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_random(rand_out: *mut u8) -> bool {
    match util::bb02_async::block_on(random::<DefaultTimer>()) {
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
    let mut random = crate::hal::random::BitBox02Random;
    let mut memory = crate::hal::memory::BitBox02Memory;
    util::bb02_async::block_on(u2f_counter_set::<DefaultTimer>(
        &mut random,
        &mut memory,
        counter,
    ))
    .is_ok()
}

#[cfg(feature = "app-u2f")]
/// Increments the U2F counter and writes the new value to `counter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_securechip_u2f_counter_inc(counter: *mut u32) -> bool {
    assert!(!counter.is_null());
    let mut random = crate::hal::random::BitBox02Random;
    let mut memory = crate::hal::memory::BitBox02Memory;
    match util::bb02_async::block_on(u2f_counter_inc::<DefaultTimer>(&mut random, &mut memory)) {
        Ok(current) => {
            unsafe {
                *counter = current;
            }
            true
        }
        Err(()) => false,
    }
}
