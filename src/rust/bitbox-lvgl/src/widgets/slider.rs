// SPDX-License-Identifier: Apache-2.0

use core::ptr::NonNull;

use super::bar::{BarExt, LvBar};
use crate::{LvAnimEnable, LvHandle, LvObj, LvSliderMode, LvSliderOrientation, class, ffi};

pub type LvSlider = LvHandle<class::SliderTag>;

pub trait SliderExt: BarExt {
    fn get_left_value(&self) -> i32 {
        unsafe { ffi::lv_slider_get_left_value(self.as_ptr()) }
    }

    fn is_dragged(&self) -> bool {
        unsafe { ffi::lv_slider_is_dragged(self.as_ptr()) }
    }
}

impl LvHandle<class::SliderTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_slider_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn set_value(&self, value: i32, anim: LvAnimEnable) {
        unsafe { ffi::lv_slider_set_value(self.as_ptr(), value, anim) }
    }

    pub fn set_start_value(&self, value: i32, anim: LvAnimEnable) {
        unsafe { ffi::lv_slider_set_start_value(self.as_ptr(), value, anim) }
    }

    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { ffi::lv_slider_set_range(self.as_ptr(), min, max) }
    }

    pub fn set_min_value(&self, min: i32) {
        unsafe { ffi::lv_slider_set_min_value(self.as_ptr(), min) }
    }

    pub fn set_max_value(&self, max: i32) {
        unsafe { ffi::lv_slider_set_max_value(self.as_ptr(), max) }
    }

    pub fn set_mode(&self, mode: LvSliderMode) {
        unsafe { ffi::lv_slider_set_mode(self.as_ptr(), mode) }
    }

    pub fn set_orientation(&self, orientation: LvSliderOrientation) {
        unsafe { ffi::lv_slider_set_orientation(self.as_ptr(), orientation) }
    }

    pub fn get_value(&self) -> i32 {
        unsafe { ffi::lv_slider_get_value(self.as_ptr()) }
    }

    pub fn get_left_value(&self) -> i32 {
        unsafe { ffi::lv_slider_get_left_value(self.as_ptr()) }
    }

    pub fn get_min_value(&self) -> i32 {
        unsafe { ffi::lv_slider_get_min_value(self.as_ptr()) }
    }

    pub fn get_max_value(&self) -> i32 {
        unsafe { ffi::lv_slider_get_max_value(self.as_ptr()) }
    }

    pub fn is_dragged(&self) -> bool {
        unsafe { ffi::lv_slider_is_dragged(self.as_ptr()) }
    }

    pub fn get_mode(&self) -> LvSliderMode {
        unsafe { ffi::lv_slider_get_mode(self.as_ptr()) }
    }

    pub fn get_orientation(&self) -> LvSliderOrientation {
        unsafe { ffi::lv_slider_get_orientation(self.as_ptr()) }
    }

    pub fn is_symmetrical(&self) -> bool {
        unsafe { ffi::lv_slider_is_symmetrical(self.as_ptr()) }
    }

    pub fn to_bar(self) -> LvBar {
        self.cast()
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl SliderExt for LvHandle<class::SliderTag> {}
