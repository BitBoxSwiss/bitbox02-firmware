// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::NonNull;

use super::image::{ImageExt, LvImage};
use super::util;
use crate::{LvColor, LvColor32, LvColorFormat, LvHandle, LvImageDsc, LvObj, LvOpa, class, ffi};

pub type LvCanvas = LvHandle<class::CanvasTag>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvCanvasCreateError {
    InvalidDimensions,
    CreateFailed,
    EventRegistrationFailed,
}

pub trait CanvasExt: ImageExt {
    /// # Safety
    /// `buf` must remain valid for the lifetime required by LVGL.
    unsafe fn set_buffer_raw<T: ?Sized>(
        &self,
        buf: &'static mut T,
        width: i32,
        height: i32,
        color_format: LvColorFormat,
    ) {
        unsafe {
            ffi::lv_canvas_set_buffer(
                self.as_ptr(),
                buf as *mut T as *mut c_void,
                width,
                height,
                color_format,
            )
        }
    }

    /// # Safety
    /// `draw_buf` must remain valid for the lifetime required by LVGL.
    unsafe fn set_draw_buf_raw(&self, draw_buf: &'static mut ffi::lv_draw_buf_t) {
        unsafe { ffi::lv_canvas_set_draw_buf(self.as_ptr(), draw_buf) }
    }

    fn set_px(&self, x: i32, y: i32, color: LvColor, opa: LvOpa) {
        unsafe { ffi::lv_canvas_set_px(self.as_ptr(), x, y, color, opa) }
    }

    fn set_palette(&self, index: u8, color: LvColor32) {
        unsafe { ffi::lv_canvas_set_palette(self.as_ptr(), index, color) }
    }

    fn get_draw_buf(&self) -> Option<NonNull<ffi::lv_draw_buf_t>> {
        NonNull::new(unsafe { ffi::lv_canvas_get_draw_buf(self.as_ptr()) })
    }

    fn get_px(&self, x: i32, y: i32) -> LvColor32 {
        unsafe { ffi::lv_canvas_get_px(self.as_ptr(), x, y) }
    }

    fn get_image(&self) -> Option<NonNull<LvImageDsc>> {
        NonNull::new(unsafe { ffi::lv_canvas_get_image(self.as_ptr()) })
    }

    fn get_buf(&self) -> *const c_void {
        unsafe { ffi::lv_canvas_get_buf(self.as_ptr()) }
    }
}

impl LvHandle<class::CanvasTag> {
    pub fn new(
        parent: &LvHandle<impl class::LvClass>,
        data: Vec<[u8; 4]>,
        width: u32,
        height: u32,
    ) -> Result<Self, LvCanvasCreateError> {
        let Ok(width_i32) = i32::try_from(width) else {
            return Err(LvCanvasCreateError::InvalidDimensions);
        };
        let Ok(height_i32) = i32::try_from(height) else {
            return Err(LvCanvasCreateError::InvalidDimensions);
        };
        let Some(pixel_count) = width.checked_mul(height) else {
            return Err(LvCanvasCreateError::InvalidDimensions);
        };
        let Ok(pixel_count) = usize::try_from(pixel_count) else {
            return Err(LvCanvasCreateError::InvalidDimensions);
        };
        if data.len() != pixel_count {
            return Err(LvCanvasCreateError::InvalidDimensions);
        }
        let Some(canvas) = NonNull::new(unsafe { ffi::lv_canvas_create(parent.as_ptr()) }) else {
            return Err(LvCanvasCreateError::CreateFailed);
        };
        let canvas = LvHandle::from_ptr(canvas);

        let attachment = util::attach_to_object(&canvas, data)
            .map_err(|_| LvCanvasCreateError::EventRegistrationFailed)?;

        unsafe {
            ffi::lv_canvas_set_buffer(
                canvas.as_ptr(),
                (*attachment.as_ptr()).as_ptr() as *mut c_void,
                width_i32,
                height_i32,
                ffi::lv_color_format_t::LV_COLOR_FORMAT_ARGB8888,
            )
        };

        Ok(canvas)
    }

    pub fn to_image(self) -> LvImage {
        self.cast()
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl CanvasExt for LvHandle<class::CanvasTag> {}
