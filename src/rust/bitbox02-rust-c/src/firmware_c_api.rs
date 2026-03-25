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
    let mut hal = unsafe { crate::acquire_bitbox02hal(hal) };
    let device_name = hal.memory().get_device_name();
    let ble_enabled = hal.memory().ble_enabled();
    let has_ble = matches!(
        hal.memory().get_platform(),
        Ok(bitbox_hal::memory::Platform::BitBox02Plus),
    );
    drop(hal);
    bitbox02_rust::main_loop::main_loop(device_name, ble_enabled, has_ble)
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
    let mut hal = unsafe { crate::acquire_bitbox02hal(hal) };
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
pub unsafe extern "C" fn rust_memory_optiga_config_is_v1_or_higher(
    hal: *mut crate::BitBox02HAL,
    result_out: *mut bool,
) -> bool {
    if result_out.is_null() {
        return false;
    }

    let mut hal = unsafe { crate::acquire_bitbox02hal(hal) };
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
pub unsafe extern "C" fn rust_memory_set_optiga_config_version_v1(
    hal: *mut crate::BitBox02HAL,
) -> bool {
    let mut hal = unsafe { crate::acquire_bitbox02hal(hal) };
    hal.memory()
        .set_optiga_config_version(OptigaConfigVersion::V1)
        .is_ok()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_memory_get_securechip_type(
    hal: *mut crate::BitBox02HAL,
) -> rust_memory_securechip_type_t {
    let mut hal = unsafe { crate::acquire_bitbox02hal(hal) };
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
pub unsafe extern "C" fn rust_memory_get_io_protection_key(
    hal: *mut crate::BitBox02HAL,
    mut key_out: BytesMut,
) {
    let mut hal = unsafe { crate::acquire_bitbox02hal(hal) };
    hal.memory()
        .get_io_protection_key(key_out.as_mut().try_into().unwrap());
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_keystore_get_u2f_seed(
    hal: *mut crate::BitBox02HAL,
    mut seed_out: util::bytes::BytesMut,
) -> bool {
    let mut hal = unsafe { crate::acquire_bitbox02hal(hal) };
    match bitbox02_rust::keystore::get_u2f_seed(&mut *hal) {
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
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn make_hal() -> crate::BitBox02HAL {
        crate::make_test_hal()
    }

    fn with_test_hal<T>(f: impl FnOnce(&mut crate::BitBox02HAL) -> T) -> T {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|err| err.into_inner());
        let mut hal = make_hal();
        bitbox02::hal::BitBox02Hal::reset_for_testing();
        unsafe { crate::rust_bitbox02hal_init(&mut hal) };
        f(&mut hal)
    }

    fn setup_memory() {
        set_optiga_config_version(MemoryOptigaConfigVersion::MEMORY_OPTIGA_CONFIG_V0).unwrap();
    }

    #[test]
    fn test_rust_memory_optiga_config_is_v1_or_higher() {
        with_test_hal(|hal| {
            setup_memory();

            let mut is_v1_or_higher = false;
            assert!(unsafe {
                rust_memory_optiga_config_is_v1_or_higher(hal, &mut is_v1_or_higher)
            });
            assert!(!is_v1_or_higher);

            assert!(unsafe { rust_memory_set_optiga_config_version_v1(hal) });

            let mut is_v1_or_higher = false;
            assert!(unsafe {
                rust_memory_optiga_config_is_v1_or_higher(hal, &mut is_v1_or_higher)
            });
            assert!(is_v1_or_higher);
        });
    }

    #[test]
    fn test_rust_memory_optiga_config_is_v1_or_higher_null_pointer() {
        with_test_hal(|hal| {
            setup_memory();
            assert!(!unsafe {
                rust_memory_optiga_config_is_v1_or_higher(hal, core::ptr::null_mut())
            });
        });
    }

    #[test]
    fn test_rust_memory_get_securechip_type() {
        with_test_hal(|hal| {
            let expected = match get_securechip_type().unwrap() {
                MemorySecurechipType::Atecc => {
                    rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_ATECC
                }
                MemorySecurechipType::Optiga => {
                    rust_memory_securechip_type_t::RUST_MEMORY_SECURECHIP_TYPE_OPTIGA
                }
            };
            assert_eq!(unsafe { rust_memory_get_securechip_type(hal) }, expected);
        });
    }

    #[test]
    fn test_rust_memory_get_io_protection_key() {
        with_test_hal(|hal| {
            let mut expected = [0u8; 32];
            unsafe { crate::acquire_bitbox02hal(hal) }
                .memory()
                .get_io_protection_key(&mut expected);
            let mut actual = [0u8; 32];
            unsafe {
                rust_memory_get_io_protection_key(
                    hal,
                    util::bytes::rust_util_bytes_mut(actual.as_mut_ptr(), actual.len()),
                )
            };
            assert_eq!(actual, expected);
        });
    }
}
