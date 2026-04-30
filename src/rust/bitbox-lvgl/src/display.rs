// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use core::ffi::c_void;
use core::ptr::NonNull;

use grounded::uninit::GroundedCell;

use crate::util::assert_user_data_can_attach;
use crate::{LvArea, LvColorFormat, LvDisplayRenderMode, LvHandle, class, ffi};

#[derive(Debug, PartialEq, Eq)]
pub struct LvDisplay {
    raw: NonNull<ffi::lv_display_t>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvDisplayBufferError {
    EmptyBuffer,
    UnalignedBuffer,
    BufferTooLarge,
    InvalidSize,
}

impl LvDisplay {
    pub fn new(hor_res: i32, ver_res: i32) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_display_create(hor_res, ver_res) }).map(|raw| Self { raw })
    }

    #[cfg(all(feature = "st-ltdc", target_os = "none"))]
    pub fn st_ltdc_create_direct<T, const N: usize>(
        fb1: &'static mut [T; N],
        fb2: Option<&'static mut [T; N]>,
        layer_idx: u32,
    ) -> Result<Self, LvDisplayBufferError> {
        if N == 0 || core::mem::size_of::<T>() == 0 {
            return Err(LvDisplayBufferError::EmptyBuffer);
        }
        if !(fb1.as_ptr() as usize).is_multiple_of(LV_DRAW_BUFFER_ALIGNMENT) {
            return Err(LvDisplayBufferError::UnalignedBuffer);
        }
        if let Some(fb2) = fb2.as_ref()
            && !(fb2.as_ptr() as usize).is_multiple_of(LV_DRAW_BUFFER_ALIGNMENT)
        {
            return Err(LvDisplayBufferError::UnalignedBuffer);
        }
        if core::mem::size_of::<[T; N]>() > u32::MAX as usize {
            return Err(LvDisplayBufferError::BufferTooLarge);
        }

        unsafe {
            Self::from_raw(ffi::lv_st_ltdc_create_direct(
                fb1.as_mut_ptr().cast(),
                fb2.map_or(core::ptr::null_mut(), |fb| fb.as_mut_ptr().cast()),
                layer_idx,
            ))
        }
        .ok_or(LvDisplayBufferError::InvalidSize)
    }

    /// # Safety
    /// `raw` must point to a live LVGL display handle managed by LVGL for the lifetime of the
    /// returned wrapper.
    pub unsafe fn from_raw(raw: *mut ffi::lv_display_t) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_ptr(&self) -> *mut ffi::lv_display_t {
        self.raw.as_ptr()
    }

    /// Registers the display draw buffers used by LVGL.
    pub fn set_buffers<T>(
        &self,
        buf1: &'static mut [T],
        buf2: Option<&'static mut [T]>,
        render_mode: LvDisplayRenderMode,
    ) -> Result<(), LvDisplayBufferError> {
        if buf1.is_empty() || core::mem::size_of::<T>() == 0 {
            return Err(LvDisplayBufferError::EmptyBuffer);
        }
        if !(buf1.as_ptr() as usize).is_multiple_of(LV_DRAW_BUFFER_ALIGNMENT) {
            return Err(LvDisplayBufferError::UnalignedBuffer);
        }
        let mut buf_size = core::mem::size_of_val(buf1);
        let buf1_ptr = buf1.as_mut_ptr();

        let buf2_ptr = if let Some(buf2) = buf2 {
            if buf2.is_empty() {
                return Err(LvDisplayBufferError::EmptyBuffer);
            }
            if !(buf2.as_ptr() as usize).is_multiple_of(LV_DRAW_BUFFER_ALIGNMENT) {
                return Err(LvDisplayBufferError::UnalignedBuffer);
            }
            buf_size = core::cmp::min(buf_size, core::mem::size_of_val(buf2));
            buf2.as_mut_ptr()
        } else {
            core::ptr::null_mut()
        };
        if buf_size > u32::MAX as usize {
            return Err(LvDisplayBufferError::BufferTooLarge);
        }
        unsafe {
            ffi::lv_display_set_buffers(
                self.as_ptr(),
                buf1_ptr.cast(),
                buf2_ptr.cast(),
                buf_size as u32,
                render_mode,
            )
        }
        Ok(())
    }

    /// Registers a display flush callback.
    ///
    /// `lv_display_flush_ready` is called automatically after the callback returns.
    pub fn set_flush_cb<F>(&self, cb: F)
    where
        F: FnMut(LvDisplay, &LvArea, *mut u8) + 'static,
    {
        let flush_cb = unsafe { &mut *DISPLAY_FLUSH_CB.get() };
        if flush_cb.is_some() {
            panic!("Only one display flush callback can be registered");
        }
        flush_cb.replace(Box::new(cb));

        unsafe { ffi::lv_display_set_flush_cb(self.as_ptr(), Some(flush_cb_trampoline)) }
    }

    pub fn set_user_data<T>(&self, user_data: Option<&'static mut T>) {
        let user_data_ptr = user_data.map_or(core::ptr::null_mut(), |value| {
            value as *mut T as *mut c_void
        });
        unsafe {
            // The pointers come from LVGL's display user data slot and from the `&'static mut T`
            // value that is about to be stored into the same slot.
            assert_user_data_can_attach(
                ffi::lv_display_get_user_data(self.as_ptr()),
                user_data_ptr,
            );
        }
        unsafe { ffi::lv_display_set_user_data(self.as_ptr(), user_data_ptr) }
    }

    pub fn set_color_format(&self, colorformat: LvColorFormat) {
        unsafe { ffi::lv_display_set_color_format(self.as_ptr(), colorformat) }
    }

    pub fn flush_is_last(&self) -> bool {
        unsafe { ffi::lv_display_flush_is_last(self.as_ptr()) }
    }

    pub fn get_user_data(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { ffi::lv_display_get_user_data(self.as_ptr()) })
    }

    pub fn screen_active(&self) -> Option<LvHandle<class::ObjTag>> {
        NonNull::new(unsafe { ffi::lv_screen_active() }).map(LvHandle::from_ptr)
    }

    pub fn layer_bottom(&self) -> Option<LvHandle<class::ObjTag>> {
        NonNull::new(unsafe { ffi::lv_layer_bottom() }).map(LvHandle::from_ptr)
    }

    pub fn screen_load(&self, screen: LvHandle) {
        unsafe { ffi::lv_screen_load(screen.as_ptr()) }
    }
}

const LV_DRAW_BUFFER_ALIGNMENT: usize = crate::ffi::LV_DRAW_BUF_ALIGN as usize;

type DisplayFlushCb = Box<dyn FnMut(LvDisplay, &LvArea, *mut u8) + 'static>;

static DISPLAY_FLUSH_CB: GroundedCell<Option<DisplayFlushCb>> = GroundedCell::const_init();

struct FlushReadyGuard {
    display: *mut ffi::lv_display_t,
}

impl Drop for FlushReadyGuard {
    fn drop(&mut self) {
        unsafe { ffi::lv_display_flush_ready(self.display) }
    }
}

extern "C" fn flush_cb_trampoline(
    display: *mut ffi::lv_display_t,
    area: *const ffi::lv_area_t,
    px_map: *mut u8,
) {
    let Some(display) = NonNull::new(display) else {
        return;
    };
    let _flush_ready_guard = FlushReadyGuard {
        display: display.as_ptr(),
    };

    if area.is_null() {
        return;
    }

    let display = LvDisplay { raw: display };
    let area = unsafe { &*area };

    let flush_cb = unsafe { &mut *DISPLAY_FLUSH_CB.get() };
    if let Some(flush_cb) = flush_cb.as_mut() {
        (flush_cb)(display, area, px_map);
    }
}
