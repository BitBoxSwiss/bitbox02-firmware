// SPDX-License-Identifier: Apache-2.0

use alloc::ffi::CString;
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::{LvBlendMode, LvHandle, LvImageAlign, LvImageDsc, LvObj, LvPoint, ObjExt, class, ffi};

pub type LvImageSourceError = super::LvTextError;
pub type LvImage = LvHandle<class::ImageTag>;

pub trait ImageExt: ObjExt {
    fn set_src(&self, src: &str) -> Result<(), LvImageSourceError> {
        let src = CString::new(src).map_err(|_| LvImageSourceError::ContainsNul)?;
        unsafe { ffi::lv_image_set_src(self.as_ptr(), src.as_ptr() as *const c_void) }
        Ok(())
    }

    fn set_src_image(&self, src: &'static LvImageDsc) {
        unsafe { ffi::lv_image_set_src(self.as_ptr(), src as *const LvImageDsc as *const c_void) }
    }

    /// # Safety
    /// `src` must point to a `'static` value of a type accepted by LVGL as an image source.
    unsafe fn set_src_raw<T: ?Sized>(&self, src: Option<&'static T>) {
        unsafe {
            ffi::lv_image_set_src(
                self.as_ptr(),
                src.map_or(core::ptr::null(), |src| src as *const T as *const c_void),
            )
        }
    }

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

    fn set_bitmap_map_src(&self, src: Option<&'static LvImageDsc>) {
        unsafe {
            ffi::lv_image_set_bitmap_map_src(
                self.as_ptr(),
                src.map_or(core::ptr::null(), |src| src as *const LvImageDsc),
            )
        }
    }

    fn get_src(&self) -> *const c_void {
        unsafe { ffi::lv_image_get_src(self.as_ptr()) }
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

    fn get_bitmap_map_src(&self) -> Option<NonNull<LvImageDsc>> {
        NonNull::new(unsafe { ffi::lv_image_get_bitmap_map_src(self.as_ptr()) as *mut LvImageDsc })
    }
}

impl LvHandle<class::ImageTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_image_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl ImageExt for LvHandle<class::ImageTag> {}
impl ImageExt for LvHandle<class::CanvasTag> {}
