// SPDX-License-Identifier: Apache-2.0

use core::ptr::NonNull;

use crate::{LvArcMode, LvHandle, LvObj, LvValuePrecise, ObjExt, class, ffi};

pub type LvArc = LvHandle<class::ArcTag>;

pub trait ArcExt: ObjExt {
    fn set_start_angle(&self, start: LvValuePrecise) {
        unsafe { ffi::lv_arc_set_start_angle(self.as_ptr(), start) }
    }

    fn set_end_angle(&self, end: LvValuePrecise) {
        unsafe { ffi::lv_arc_set_end_angle(self.as_ptr(), end) }
    }

    fn set_angles(&self, start: LvValuePrecise, end: LvValuePrecise) {
        unsafe { ffi::lv_arc_set_angles(self.as_ptr(), start, end) }
    }

    fn set_bg_start_angle(&self, start: LvValuePrecise) {
        unsafe { ffi::lv_arc_set_bg_start_angle(self.as_ptr(), start) }
    }

    fn set_bg_end_angle(&self, end: LvValuePrecise) {
        unsafe { ffi::lv_arc_set_bg_end_angle(self.as_ptr(), end) }
    }

    fn set_bg_angles(&self, start: LvValuePrecise, end: LvValuePrecise) {
        unsafe { ffi::lv_arc_set_bg_angles(self.as_ptr(), start, end) }
    }

    fn set_rotation(&self, rotation: i32) {
        unsafe { ffi::lv_arc_set_rotation(self.as_ptr(), rotation) }
    }

    fn set_mode(&self, mode: LvArcMode) {
        unsafe { ffi::lv_arc_set_mode(self.as_ptr(), mode) }
    }

    fn set_value(&self, value: i32) {
        unsafe { ffi::lv_arc_set_value(self.as_ptr(), value) }
    }

    fn set_range(&self, min: i32, max: i32) {
        unsafe { ffi::lv_arc_set_range(self.as_ptr(), min, max) }
    }

    fn set_min_value(&self, min: i32) {
        unsafe { ffi::lv_arc_set_min_value(self.as_ptr(), min) }
    }

    fn set_max_value(&self, max: i32) {
        unsafe { ffi::lv_arc_set_max_value(self.as_ptr(), max) }
    }

    fn set_change_rate(&self, rate: u32) {
        unsafe { ffi::lv_arc_set_change_rate(self.as_ptr(), rate) }
    }

    fn set_knob_offset(&self, offset: i32) {
        unsafe { ffi::lv_arc_set_knob_offset(self.as_ptr(), offset) }
    }

    fn get_angle_start(&self) -> LvValuePrecise {
        unsafe { ffi::lv_arc_get_angle_start(self.as_ptr()) }
    }

    fn get_angle_end(&self) -> LvValuePrecise {
        unsafe { ffi::lv_arc_get_angle_end(self.as_ptr()) }
    }

    fn get_bg_angle_start(&self) -> LvValuePrecise {
        unsafe { ffi::lv_arc_get_bg_angle_start(self.as_ptr()) }
    }

    fn get_bg_angle_end(&self) -> LvValuePrecise {
        unsafe { ffi::lv_arc_get_bg_angle_end(self.as_ptr()) }
    }

    fn get_value(&self) -> i32 {
        unsafe { ffi::lv_arc_get_value(self.as_ptr()) }
    }

    fn get_min_value(&self) -> i32 {
        unsafe { ffi::lv_arc_get_min_value(self.as_ptr()) }
    }

    fn get_max_value(&self) -> i32 {
        unsafe { ffi::lv_arc_get_max_value(self.as_ptr()) }
    }

    fn get_mode(&self) -> LvArcMode {
        unsafe { ffi::lv_arc_get_mode(self.as_ptr()) }
    }

    fn get_rotation(&self) -> i32 {
        unsafe { ffi::lv_arc_get_rotation(self.as_ptr()) }
    }

    fn get_knob_offset(&self) -> i32 {
        unsafe { ffi::lv_arc_get_knob_offset(self.as_ptr()) }
    }
}

impl LvHandle<class::ArcTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_arc_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl ArcExt for LvHandle<class::ArcTag> {}
impl ArcExt for LvHandle<class::SpinnerTag> {}
