// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_char;
use util::bytes::{Bytes, BytesMut};

use bitbox_hal::Hal;

#[cfg(not(any(feature = "c-unit-testing", feature = "simulator-graphical")))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_main_loop() -> ! {
    bitbox02_rust::main_loop::main_loop(&mut crate::HalImpl::new())
}

/// # Safety
///
/// `purpose` must be a valid, null-terminated UTF-8 string pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_salt_hash_data(
    data: Bytes,
    purpose: *const c_char,
    mut hash_out: BytesMut,
) -> bool {
    let purpose_str = match unsafe { util::strings::str_from_null_terminated_ptr(purpose) } {
        Ok(purpose) => purpose,
        Err(()) => return false,
    };
    let mut hal = crate::HalImpl::new();
    match bitbox02_rust::salt::hash_data(hal.memory(), data.as_ref(), purpose_str) {
        Ok(hash) => {
            hash_out.as_mut()[..32].copy_from_slice(&hash);
            true
        }
        Err(()) => false,
    }
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_keystore_get_u2f_seed(mut seed_out: util::bytes::BytesMut) -> bool {
    match bitbox02_rust::keystore::get_u2f_seed(&mut crate::HalImpl::new()) {
        Ok(seed) => {
            seed_out.as_mut().copy_from_slice(&seed);
            true
        }
        Err(_) => false,
    }
}
