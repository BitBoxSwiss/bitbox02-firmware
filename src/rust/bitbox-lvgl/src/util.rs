// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_void;

/// # Safety
///
/// `current` and `new` must each be null or point to the user data object currently attached to
/// LVGL / about to be attached to LVGL, respectively.
pub(crate) unsafe fn assert_user_data_can_attach(current: *mut c_void, new: *mut c_void) {
    if !current.is_null() && !new.is_null() {
        panic!("user data already attached");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_user_data_can_attach_when_empty() {
        unsafe {
            assert_user_data_can_attach(
                core::ptr::null_mut(),
                core::ptr::dangling_mut::<u8>().cast(),
            );
        }
    }

    #[test]
    fn test_assert_user_data_can_attach_when_clearing() {
        unsafe {
            assert_user_data_can_attach(
                core::ptr::dangling_mut::<u8>().cast(),
                core::ptr::null_mut(),
            );
        }
    }

    #[test]
    #[should_panic(expected = "user data already attached")]
    fn test_assert_user_data_can_attach_when_replacing() {
        unsafe {
            assert_user_data_can_attach(
                core::ptr::dangling_mut::<u8>().cast(),
                core::ptr::dangling_mut::<u16>().cast(),
            );
        }
    }
}
