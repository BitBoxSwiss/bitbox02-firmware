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
pub unsafe extern "C" fn rust_main_loop(hal: *mut crate::BitBox02HAL) -> ! {
    bitbox02_rust::main_loop::main_loop(unsafe { crate::bitbox02hal_mut(hal) })
}

/// # Safety
///
/// `purpose` must be a valid, null-terminated UTF-8 string pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_salt_hash_data(
    hal: *mut crate::BitBox02HAL,
    data: Bytes,
    purpose: *const c_char,
    mut hash_out: BytesMut,
) -> bool {
    let purpose_str = match unsafe { util::strings::str_from_null_terminated_ptr(purpose) } {
        Ok(purpose) => purpose,
        Err(()) => return false,
    };
    match bitbox02_rust::salt::hash_data(
        unsafe { crate::bitbox02hal_mut(hal) }.memory(),
        data.as_ref(),
        purpose_str,
    ) {
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
pub unsafe extern "C" fn rust_memory_optiga_config_is_v1_or_higher(
    hal: *mut crate::BitBox02HAL,
    result_out: *mut bool,
) -> bool {
    if result_out.is_null() {
        return false;
    }

    match unsafe { crate::bitbox02hal_mut(hal) }
        .memory()
        .get_optiga_config_version()
    {
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
pub unsafe extern "C" fn rust_memory_set_optiga_config_version_v1(
    hal: *mut crate::BitBox02HAL,
) -> bool {
    unsafe { crate::bitbox02hal_mut(hal) }
        .memory()
        .set_optiga_config_version(OptigaConfigVersion::V1)
        .is_ok()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_memory_get_securechip_type(
    hal: *mut crate::BitBox02HAL,
) -> rust_memory_securechip_type_t {
    match unsafe { crate::bitbox02hal_mut(hal) }
        .memory()
        .get_securechip_type()
    {
        Ok(SecurechipType::Optiga) => {
            rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_OPTIGA
        }
        Ok(SecurechipType::Atecc) | Err(()) => {
            rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_ATECC
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_memory_get_io_protection_key(
    hal: *mut crate::BitBox02HAL,
    mut key_out: BytesMut,
) {
    unsafe { crate::bitbox02hal_mut(hal) }
        .memory()
        .get_io_protection_key(key_out.as_mut().try_into().unwrap());
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_keystore_get_u2f_seed(
    hal: *mut crate::BitBox02HAL,
    mut seed_out: util::bytes::BytesMut,
) -> bool {
    match bitbox02_rust::keystore::get_u2f_seed(unsafe { crate::bitbox02hal_mut(hal) }) {
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
    use bitbox_hal::{Hal, Memory};
    use bitbox02::memory::{
        OptigaConfigVersion as MemoryOptigaConfigVersion, SecurechipType as MemorySecurechipType,
        get_securechip_type, set_optiga_config_version,
    };

    fn make_hal() -> crate::BitBox02HAL {
        crate::make_test_hal()
    }

    fn setup_memory() {
        set_optiga_config_version(MemoryOptigaConfigVersion::MEMORY_OPTIGA_CONFIG_V0).unwrap();
    }

    #[test]
    fn test_rust_memory_optiga_config_is_v1_or_higher() {
        setup_memory();
        let mut hal = make_hal();

        let mut is_v1_or_higher = false;
        assert!(unsafe {
            rust_memory_optiga_config_is_v1_or_higher(&mut hal, &mut is_v1_or_higher)
        });
        assert!(!is_v1_or_higher);

        assert!(unsafe { rust_memory_set_optiga_config_version_v1(&mut hal) });

        let mut is_v1_or_higher = false;
        assert!(unsafe {
            rust_memory_optiga_config_is_v1_or_higher(&mut hal, &mut is_v1_or_higher)
        });
        assert!(is_v1_or_higher);
    }

    #[test]
    fn test_rust_memory_optiga_config_is_v1_or_higher_null_pointer() {
        let mut hal = make_hal();
        setup_memory();
        assert!(!unsafe {
            rust_memory_optiga_config_is_v1_or_higher(&mut hal, core::ptr::null_mut())
        });
    }

    #[test]
    fn test_rust_memory_get_securechip_type() {
        let mut hal = make_hal();
        let expected = match get_securechip_type().unwrap() {
            MemorySecurechipType::Atecc => {
                rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_ATECC
            }
            MemorySecurechipType::Optiga => {
                rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_OPTIGA
            }
        };
        assert_eq!(
            unsafe { rust_memory_get_securechip_type(&mut hal) },
            expected
        );
    }

    #[test]
    fn test_rust_memory_get_io_protection_key() {
        let mut hal = make_hal();
        let mut expected = [0u8; 32];
        unsafe { crate::bitbox02hal_mut(&mut hal) }
            .memory()
            .get_io_protection_key(&mut expected);
        let mut actual = [0u8; 32];
        unsafe {
            rust_memory_get_io_protection_key(
                &mut hal,
                util::bytes::rust_util_bytes_mut(actual.as_mut_ptr(), actual.len()),
            )
        };
        assert_eq!(actual, expected);
    }
}
