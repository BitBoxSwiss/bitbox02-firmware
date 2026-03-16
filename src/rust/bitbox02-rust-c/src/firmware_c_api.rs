// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_char;
use util::bytes::{Bytes, BytesMut};

use bitbox_hal::{Hal, Memory, memory::OptigaConfigVersion};

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

/// # Safety
///
/// `result_out` must be a valid pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_memory_optiga_config_is_v1_or_higher(result_out: *mut bool) -> bool {
    if result_out.is_null() {
        return false;
    }

    let mut hal = crate::HalImpl::new();
    match hal.memory().get_optiga_config_version() {
        Ok(version) => {
            unsafe {
                *result_out = version >= OptigaConfigVersion::V1;
            }
            true
        }
        Err(()) => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_memory_set_optiga_config_version_v1() -> bool {
    let mut hal = crate::HalImpl::new();
    hal.memory()
        .set_optiga_config_version(OptigaConfigVersion::V1)
        .is_ok()
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

#[cfg(test)]
mod tests {
    use super::*;
    use bitbox02::memory::{
        OptigaConfigVersion as MemoryOptigaConfigVersion, set_optiga_config_version,
    };

    fn setup_memory() {
        set_optiga_config_version(MemoryOptigaConfigVersion::MEMORY_OPTIGA_CONFIG_V0).unwrap();
    }

    #[test]
    fn test_rust_memory_optiga_config_is_v1_or_higher() {
        setup_memory();

        let mut is_v1_or_higher = false;
        assert!(unsafe { rust_memory_optiga_config_is_v1_or_higher(&mut is_v1_or_higher) });
        assert!(!is_v1_or_higher);

        assert!(rust_memory_set_optiga_config_version_v1());

        let mut is_v1_or_higher = false;
        assert!(unsafe { rust_memory_optiga_config_is_v1_or_higher(&mut is_v1_or_higher) });
        assert!(is_v1_or_higher);
    }

    #[test]
    fn test_rust_memory_optiga_config_is_v1_or_higher_null_pointer() {
        setup_memory();
        assert!(!unsafe { rust_memory_optiga_config_is_v1_or_higher(core::ptr::null_mut()) });
    }
}
