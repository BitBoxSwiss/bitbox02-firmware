// SPDX-License-Identifier: Apache-2.0

use core::ptr::NonNull;

use crate::{LvAnimEnable, LvBarMode, LvBarOrientation, LvHandle, LvObj, ObjExt, class, ffi};

pub type LvBar = LvHandle<class::BarTag>;

pub trait BarExt: ObjExt {
    fn set_value(&self, value: i32, anim: LvAnimEnable) {
        unsafe { ffi::lv_bar_set_value(self.as_ptr(), value, anim) }
    }

    fn set_start_value(&self, value: i32, anim: LvAnimEnable) {
        unsafe { ffi::lv_bar_set_start_value(self.as_ptr(), value, anim) }
    }

    fn set_range(&self, min: i32, max: i32) {
        unsafe { ffi::lv_bar_set_range(self.as_ptr(), min, max) }
    }

    fn set_min_value(&self, min: i32) {
        unsafe { ffi::lv_bar_set_min_value(self.as_ptr(), min) }
    }

    fn set_max_value(&self, max: i32) {
        unsafe { ffi::lv_bar_set_max_value(self.as_ptr(), max) }
    }

    fn set_mode(&self, mode: LvBarMode) {
        unsafe { ffi::lv_bar_set_mode(self.as_ptr(), mode) }
    }

    fn set_orientation(&self, orientation: LvBarOrientation) {
        unsafe { ffi::lv_bar_set_orientation(self.as_ptr(), orientation) }
    }

    fn get_value(&self) -> i32 {
        unsafe { ffi::lv_bar_get_value(self.as_ptr()) }
    }

    fn get_start_value(&self) -> i32 {
        unsafe { ffi::lv_bar_get_start_value(self.as_ptr()) }
    }

    fn get_min_value(&self) -> i32 {
        unsafe { ffi::lv_bar_get_min_value(self.as_ptr()) }
    }

    fn get_max_value(&self) -> i32 {
        unsafe { ffi::lv_bar_get_max_value(self.as_ptr()) }
    }

    fn get_mode(&self) -> LvBarMode {
        unsafe { ffi::lv_bar_get_mode(self.as_ptr()) }
    }

    fn get_orientation(&self) -> LvBarOrientation {
        unsafe { ffi::lv_bar_get_orientation(self.as_ptr()) }
    }

    fn is_symmetrical(&self) -> bool {
        unsafe { ffi::lv_bar_is_symmetrical(self.as_ptr()) }
    }
}

impl LvHandle<class::BarTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_bar_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl BarExt for LvHandle<class::BarTag> {}
impl BarExt for LvHandle<class::SliderTag> {}
