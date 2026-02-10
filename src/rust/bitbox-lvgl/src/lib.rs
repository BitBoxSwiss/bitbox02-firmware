#![no_std]

extern crate alloc;

pub mod ffi {
    pub use bitbox_lvgl_sys::*;
}

pub use ffi::lv_align_t as LvAlign;
pub use ffi::lv_area_t as LvArea;
pub use ffi::lv_display_render_mode_t as LvDisplayRenderMode;
pub use ffi::lv_indev_state_t as LvIndevState;
pub use ffi::lv_indev_type_t as LvIndevType;
pub use ffi::lv_point_t as LvPoint;

pub struct LvObj {
    inner: *mut ffi::lv_obj_t,
}

pub struct LvDisplay {
    inner: *mut ffi::lv_display_t,
}

pub struct LvIndev {
    inner: *mut ffi::lv_indev_t,
}

pub fn lv_init() {
    unsafe { ffi::lv_init() }
}

pub fn lv_screen_active() -> LvObj {
    LvObj {
        inner: unsafe { ffi::lv_screen_active() },
    }
}

pub fn lv_label_create(parent: &LvObj) -> LvObj {
    LvObj {
        inner: unsafe { ffi::lv_label_create(parent.inner) },
    }
}

pub fn lv_label_set_text(obj: &LvObj, txt: &str) {
    if let Ok(txt) = alloc::ffi::CString::new(txt) {
        let txt = txt.as_bytes_with_nul();
        unsafe { ffi::lv_label_set_text(obj.inner, txt.as_ptr() as *const _) }
    } else {
        panic!("Invalid label text");
    }
}

pub fn lv_obj_align(obj: &LvObj, align: LvAlign, x_ofs: i32, y_ofs: i32) {
    unsafe { ffi::lv_obj_align(obj.inner, align, x_ofs, y_ofs) }
}

pub fn lv_timer_handler() {
    unsafe {
        ffi::lv_timer_handler();
    }
}

pub fn lv_tick_set_cb(cb: extern "C" fn() -> u32) {
    let cb: unsafe extern "C" fn() -> u32 = unsafe { core::mem::transmute(cb) };
    unsafe { ffi::lv_tick_set_cb(Some(cb)) }
}

pub fn lv_display_create(hor_res: i32, ver_res: i32) -> LvDisplay {
    LvDisplay {
        inner: unsafe { ffi::lv_display_create(hor_res, ver_res) },
    }
}

pub fn lv_display_set_buffers(
    disp: &LvDisplay,
    buf1: &mut [u8],
    buf2: Option<&mut [u8]>,
    buf_size: u32,
    render_mode: LvDisplayRenderMode,
) {
    let buf1 = buf1.as_mut_ptr() as *mut _;
    let buf2 = if let Some(b) = buf2 {
        b.as_mut_ptr() as *mut _
    } else {
        core::ptr::null_mut()
    };
    unsafe { ffi::lv_display_set_buffers(disp.inner, buf1, buf2, buf_size, render_mode) }
}

pub fn lv_display_set_flush_cb(
    display: &LvDisplay,
    cb: unsafe extern "C" fn(
        disp: *mut ffi::lv_display_t,
        area: *const ffi::lv_area_t,
        px_map: *mut u8,
    ),
) {
    unsafe { ffi::lv_display_set_flush_cb(display.inner, Some(cb)) }
}

/// # Safety
/// user_data must be valid until lvgl is deinitalized
pub unsafe fn lv_display_set_user_data(display: &LvDisplay, user_data: *mut core::ffi::c_void) {
    unsafe { ffi::lv_display_set_user_data(display.inner, user_data) }
}

pub fn lv_display_get_user_data(display: &LvDisplay) -> *mut core::ffi::c_void {
    unsafe { ffi::lv_display_get_user_data(display.inner) }
}

pub fn lv_indev_create() -> LvIndev {
    LvIndev {
        inner: unsafe { ffi::lv_indev_create() },
    }
}

pub fn lv_indev_set_type(indev: &LvIndev, typ: LvIndevType) {
    unsafe { ffi::lv_indev_set_type(indev.inner, typ) }
}

pub fn lv_indev_set_read_cb(
    indev: &LvIndev,
    cb: unsafe extern "C" fn(indev: *mut ffi::lv_indev_t, data: *mut ffi::lv_indev_data_t),
) {
    unsafe { ffi::lv_indev_set_read_cb(indev.inner, Some(cb)) }
}

/// # Safety
/// user_data must be valid until lvgl is deinitalized
pub unsafe fn lv_indev_set_user_data(indev: &LvIndev, user_data: *mut core::ffi::c_void) {
    unsafe { ffi::lv_indev_set_user_data(indev.inner, user_data) }
}

pub fn lv_indev_get_user_data(indev: &LvIndev) -> *mut core::ffi::c_void {
    unsafe { ffi::lv_indev_get_user_data(indev.inner) }
}
