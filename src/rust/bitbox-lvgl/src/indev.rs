// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_void;
use core::ptr::NonNull;

use crate::util::assert_user_data_can_attach;
use crate::{LvIndevType, ffi};

#[derive(Debug, PartialEq, Eq)]
pub struct LvIndev {
    raw: NonNull<ffi::lv_indev_t>,
}

impl LvIndev {
    pub fn new() -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_indev_create() }).map(|raw| Self { raw })
    }

    pub fn as_ptr(&self) -> *mut ffi::lv_indev_t {
        self.raw.as_ptr()
    }

    pub fn set_type(&self, typ: LvIndevType) {
        unsafe { ffi::lv_indev_set_type(self.as_ptr(), typ) }
    }

    pub fn set_read_cb(
        &self,
        cb: Option<
            unsafe extern "C" fn(indev: *mut ffi::lv_indev_t, data: *mut ffi::lv_indev_data_t),
        >,
    ) {
        unsafe { ffi::lv_indev_set_read_cb(self.as_ptr(), cb) }
    }

    pub fn set_user_data<T>(&self, user_data: Option<&'static mut T>) {
        let user_data_ptr = user_data.map_or(core::ptr::null_mut(), |value| {
            value as *mut T as *mut c_void
        });
        unsafe {
            // The pointers come from LVGL's input-device user data slot and from the
            // `&'static mut T` value that is about to be stored into the same slot.
            assert_user_data_can_attach(ffi::lv_indev_get_user_data(self.as_ptr()), user_data_ptr);
        }
        unsafe { ffi::lv_indev_set_user_data(self.as_ptr(), user_data_ptr) }
    }

    pub fn get_user_data(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { ffi::lv_indev_get_user_data(self.as_ptr()) })
    }
}

pub fn set_type(indev: &LvIndev, typ: LvIndevType) {
    indev.set_type(typ)
}

pub fn set_read_cb(
    indev: &LvIndev,
    cb: Option<unsafe extern "C" fn(indev: *mut ffi::lv_indev_t, data: *mut ffi::lv_indev_data_t)>,
) {
    indev.set_read_cb(cb)
}

pub fn set_user_data<T>(indev: &LvIndev, user_data: Option<&'static mut T>) {
    indev.set_user_data(user_data)
}

pub fn get_user_data(indev: &LvIndev) -> Option<NonNull<c_void>> {
    indev.get_user_data()
}
