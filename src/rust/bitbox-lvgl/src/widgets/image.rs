// SPDX-License-Identifier: Apache-2.0

use crate::{LvBlendMode, LvHandle, LvImageAlign, LvObj, LvPoint, ObjExt, class, ffi};

pub trait ImageExt: ObjExt {
    fn set_offset_x(&self, x: i32) {
        unsafe { ffi::lv_image_set_offset_x(self.as_ptr(), x) }
    }

    fn set_offset_y(&self, y: i32) {
        unsafe { ffi::lv_image_set_offset_y(self.as_ptr(), y) }
    }

    fn set_rotation(&self, angle: i32) {
        unsafe { ffi::lv_image_set_rotation(self.as_ptr(), angle) }
    }

    fn set_pivot(&self, x: i32, y: i32) {
        unsafe { ffi::lv_image_set_pivot(self.as_ptr(), x, y) }
    }

    fn set_pivot_x(&self, x: i32) {
        unsafe { ffi::lv_image_set_pivot_x(self.as_ptr(), x) }
    }

    fn set_pivot_y(&self, y: i32) {
        unsafe { ffi::lv_image_set_pivot_y(self.as_ptr(), y) }
    }

    fn set_scale(&self, zoom: u32) {
        unsafe { ffi::lv_image_set_scale(self.as_ptr(), zoom) }
    }

    fn set_scale_x(&self, zoom: u32) {
        unsafe { ffi::lv_image_set_scale_x(self.as_ptr(), zoom) }
    }

    fn set_scale_y(&self, zoom: u32) {
        unsafe { ffi::lv_image_set_scale_y(self.as_ptr(), zoom) }
    }

    fn set_blend_mode(&self, blend_mode: LvBlendMode) {
        unsafe { ffi::lv_image_set_blend_mode(self.as_ptr(), blend_mode) }
    }

    fn set_antialias(&self, antialias: bool) {
        unsafe { ffi::lv_image_set_antialias(self.as_ptr(), antialias) }
    }

    fn set_inner_align(&self, align: LvImageAlign) {
        unsafe { ffi::lv_image_set_inner_align(self.as_ptr(), align) }
    }

    fn get_offset_x(&self) -> i32 {
        unsafe { ffi::lv_image_get_offset_x(self.as_ptr()) }
    }

    fn get_offset_y(&self) -> i32 {
        unsafe { ffi::lv_image_get_offset_y(self.as_ptr()) }
    }

    fn get_rotation(&self) -> i32 {
        unsafe { ffi::lv_image_get_rotation(self.as_ptr()) }
    }

    fn get_pivot(&self) -> LvPoint {
        let mut pivot = LvPoint { x: 0, y: 0 };
        unsafe { ffi::lv_image_get_pivot(self.as_ptr(), &mut pivot) }
        pivot
    }

    fn get_scale(&self) -> i32 {
        unsafe { ffi::lv_image_get_scale(self.as_ptr()) }
    }

    fn get_scale_x(&self) -> i32 {
        unsafe { ffi::lv_image_get_scale_x(self.as_ptr()) }
    }

    fn get_scale_y(&self) -> i32 {
        unsafe { ffi::lv_image_get_scale_y(self.as_ptr()) }
    }

    fn get_src_width(&self) -> i32 {
        unsafe { ffi::lv_image_get_src_width(self.as_ptr()) }
    }

    fn get_src_height(&self) -> i32 {
        unsafe { ffi::lv_image_get_src_height(self.as_ptr()) }
    }

    fn get_transformed_width(&self) -> i32 {
        unsafe { ffi::lv_image_get_transformed_width(self.as_ptr()) }
    }

    fn get_transformed_height(&self) -> i32 {
        unsafe { ffi::lv_image_get_transformed_height(self.as_ptr()) }
    }

    fn get_blend_mode(&self) -> LvBlendMode {
        unsafe { ffi::lv_image_get_blend_mode(self.as_ptr()) }
    }

    fn get_antialias(&self) -> bool {
        unsafe { ffi::lv_image_get_antialias(self.as_ptr()) }
    }

    fn get_inner_align(&self) -> LvImageAlign {
        unsafe { ffi::lv_image_get_inner_align(self.as_ptr()) }
    }
}

impl LvHandle<class::ImageTag> {
    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl ImageExt for LvHandle<class::CanvasTag> {}
