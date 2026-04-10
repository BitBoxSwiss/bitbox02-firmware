// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_char;
use util::bytes::{Bytes, BytesMut};

use bitbox_hal::{
    Hal, Memory,
    memory::{OptigaConfigVersion, SecurechipType},
};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum rust_memory_securechip_type_t {
    RUST_MEMORY_SECURECHIP_TYPE_ATECC = 0,
    RUST_MEMORY_SECURECHIP_TYPE_OPTIGA = 1,
}

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
    match bitbox_core_utils::salt::hash_data(hal.memory(), data.as_ref(), purpose_str) {
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

#[unsafe(no_mangle)]
pub extern "C" fn rust_memory_get_securechip_type() -> rust_memory_securechip_type_t {
    let mut hal = crate::HalImpl::new();
    match hal.memory().get_securechip_type() {
        Ok(SecurechipType::Optiga) => {
            rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_OPTIGA
        }
        Ok(SecurechipType::Atecc) | Err(()) => {
            rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_ATECC
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_memory_get_io_protection_key(mut key_out: BytesMut) {
    let mut hal = crate::HalImpl::new();
    hal.memory()
        .get_io_protection_key(key_out.as_mut().try_into().unwrap());
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
        OptigaConfigVersion as MemoryOptigaConfigVersion, SecurechipType as MemorySecurechipType,
        get_securechip_type, set_optiga_config_version,
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

    #[test]
    fn test_rust_memory_get_securechip_type() {
        let expected = match get_securechip_type().unwrap() {
            MemorySecurechipType::Atecc => {
                rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_ATECC
            }
            MemorySecurechipType::Optiga => {
                rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_OPTIGA
            }
        };
        assert_eq!(rust_memory_get_securechip_type(), expected);
    }

    #[test]
    fn test_rust_memory_get_io_protection_key() {
        let mut hal = crate::HalImpl::new();
        let mut expected = [0u8; 32];
        hal.memory().get_io_protection_key(&mut expected);
        let mut actual = [0u8; 32];
        rust_memory_get_io_protection_key(unsafe {
            util::bytes::rust_util_bytes_mut(actual.as_mut_ptr(), actual.len())
        });
        assert_eq!(actual, expected);
    }
}
