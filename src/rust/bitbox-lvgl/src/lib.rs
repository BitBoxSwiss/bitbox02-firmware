#![no_std]

extern crate alloc;

use core::ffi::c_void;
use core::ptr::NonNull;

pub mod ffi {
    pub use bitbox_lvgl_sys::*;
}

pub use ffi::lv_align_t as LvAlign;
pub use ffi::lv_area_t as LvArea;
pub use ffi::lv_display_render_mode_t as LvDisplayRenderMode;
pub use ffi::lv_indev_state_t as LvIndevState;
pub use ffi::lv_indev_type_t as LvIndevType;
pub use ffi::lv_point_t as LvPoint;

// Keep in sync with `src/lvgl/lv_conf.h`.
const LV_DRAW_BUFFER_ALIGNMENT: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LvObj {
    inner: NonNull<ffi::lv_obj_t>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LvDisplay {
    inner: NonNull<ffi::lv_display_t>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LvIndev {
    inner: NonNull<ffi::lv_indev_t>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvLabelTextError {
    ContainsNul,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvDisplayBufferError {
    EmptyBuffer,
    UnalignedBuffer,
    BufferTooLarge,
}

impl LvObj {
    pub fn as_ptr(self) -> *mut ffi::lv_obj_t {
        self.inner.as_ptr()
    }
}

impl LvDisplay {
    pub fn as_ptr(self) -> *mut ffi::lv_display_t {
        self.inner.as_ptr()
    }
}

impl LvIndev {
    pub fn as_ptr(self) -> *mut ffi::lv_indev_t {
        self.inner.as_ptr()
    }
}

pub fn lv_init() {
    unsafe { ffi::lv_init() }
}

pub fn lv_screen_active() -> Option<LvObj> {
    NonNull::new(unsafe { ffi::lv_screen_active() }).map(|inner| LvObj { inner })
}

pub fn lv_label_create(parent: &LvObj) -> Option<LvObj> {
    NonNull::new(unsafe { ffi::lv_label_create(parent.inner.as_ptr()) })
        .map(|inner| LvObj { inner })
}

pub fn lv_label_set_text(obj: &LvObj, txt: &str) -> Result<(), LvLabelTextError> {
    let txt = alloc::ffi::CString::new(txt).map_err(|_| LvLabelTextError::ContainsNul)?;
    unsafe { ffi::lv_label_set_text(obj.inner.as_ptr(), txt.as_ptr()) }
    Ok(())
}

pub fn lv_obj_align(obj: &LvObj, align: LvAlign, x_ofs: i32, y_ofs: i32) {
    unsafe { ffi::lv_obj_align(obj.inner.as_ptr(), align, x_ofs, y_ofs) }
}

pub fn lv_timer_handler() {
    unsafe {
        ffi::lv_timer_handler();
    }
}

pub fn lv_tick_set_cb(cb: Option<unsafe extern "C" fn() -> u32>) {
    unsafe { ffi::lv_tick_set_cb(cb) }
}

pub fn lv_display_create(hor_res: i32, ver_res: i32) -> Option<LvDisplay> {
    NonNull::new(unsafe { ffi::lv_display_create(hor_res, ver_res) })
        .map(|inner| LvDisplay { inner })
}

/// # Safety
/// The buffers must remain valid and suitably aligned until LVGL no longer uses them.
pub unsafe fn lv_display_set_buffers(
    disp: &LvDisplay,
    buf1: &mut [u8],
    buf2: Option<&mut [u8]>,
    render_mode: LvDisplayRenderMode,
) -> Result<(), LvDisplayBufferError> {
    if buf1.is_empty() {
        return Err(LvDisplayBufferError::EmptyBuffer);
    }
    if (buf1.as_ptr() as usize) % LV_DRAW_BUFFER_ALIGNMENT != 0 {
        return Err(LvDisplayBufferError::UnalignedBuffer);
    }
    let mut buf_size = buf1.len();
    let buf1_ptr = buf1.as_mut_ptr();

    let buf2_ptr = if let Some(buf2) = buf2 {
        if buf2.is_empty() {
            return Err(LvDisplayBufferError::EmptyBuffer);
        }
        if (buf2.as_ptr() as usize) % LV_DRAW_BUFFER_ALIGNMENT != 0 {
            return Err(LvDisplayBufferError::UnalignedBuffer);
        }
        buf_size = core::cmp::min(buf_size, buf2.len());
        buf2.as_mut_ptr()
    } else {
        core::ptr::null_mut()
    };
    if buf_size > u32::MAX as usize {
        return Err(LvDisplayBufferError::BufferTooLarge);
    }
    unsafe {
        ffi::lv_display_set_buffers(
            disp.inner.as_ptr(),
            buf1_ptr.cast(),
            buf2_ptr.cast(),
            buf_size as u32,
            render_mode,
        )
    }
    Ok(())
}

pub fn lv_display_set_flush_cb(
    display: &LvDisplay,
    cb: Option<
        unsafe extern "C" fn(
            disp: *mut ffi::lv_display_t,
            area: *const ffi::lv_area_t,
            px_map: *mut u8,
        ),
    >,
) {
    unsafe { ffi::lv_display_set_flush_cb(display.inner.as_ptr(), cb) }
}

/// # Safety
/// `user_data` must be valid until LVGL is deinitialized.
pub unsafe fn lv_display_set_user_data(display: &LvDisplay, user_data: Option<NonNull<c_void>>) {
    unsafe {
        ffi::lv_display_set_user_data(
            display.inner.as_ptr(),
            user_data.map_or(core::ptr::null_mut(), NonNull::as_ptr),
        )
    }
}

pub fn lv_display_get_user_data(display: &LvDisplay) -> Option<NonNull<c_void>> {
    NonNull::new(unsafe { ffi::lv_display_get_user_data(display.inner.as_ptr()) })
}

pub fn lv_indev_create() -> Option<LvIndev> {
    NonNull::new(unsafe { ffi::lv_indev_create() }).map(|inner| LvIndev { inner })
}

pub fn lv_indev_set_type(indev: &LvIndev, typ: LvIndevType) {
    unsafe { ffi::lv_indev_set_type(indev.inner.as_ptr(), typ) }
}

pub fn lv_indev_set_read_cb(
    indev: &LvIndev,
    cb: Option<unsafe extern "C" fn(indev: *mut ffi::lv_indev_t, data: *mut ffi::lv_indev_data_t)>,
) {
    unsafe { ffi::lv_indev_set_read_cb(indev.inner.as_ptr(), cb) }
}

/// # Safety
/// `user_data` must be valid until LVGL is deinitialized.
pub unsafe fn lv_indev_set_user_data(indev: &LvIndev, user_data: Option<NonNull<c_void>>) {
    unsafe {
        ffi::lv_indev_set_user_data(
            indev.inner.as_ptr(),
            user_data.map_or(core::ptr::null_mut(), NonNull::as_ptr),
        )
    }
}

pub fn lv_indev_get_user_data(indev: &LvIndev) -> Option<NonNull<c_void>> {
    NonNull::new(unsafe { ffi::lv_indev_get_user_data(indev.inner.as_ptr()) })
}
