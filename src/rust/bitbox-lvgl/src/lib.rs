#![no_std]

extern crate alloc;

pub mod ffi {
    #![allow(non_camel_case_types)]

    use core::ffi::{c_char, c_void};

    // Opaque type
    #[repr(C)]
    pub struct lv_obj_t {
        _data: (),
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Opaque type
    #[repr(C)]
    pub struct lv_display_t {
        _data: (),
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Opaque type
    #[repr(C)]
    pub struct lv_indev_t {
        _data: (),
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct lv_point_t {
        pub x: i32,
        pub y: i32,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct lv_area_t {
        pub x1: i32,
        pub y1: i32,
        pub x2: i32,
        pub y2: i32,
    }

    #[repr(C)]
    pub enum lv_align_t {
        LV_ALIGN_DEFAULT = 0,
        LV_ALIGN_TOP_LEFT,
        LV_ALIGN_TOP_MID,
        LV_ALIGN_TOP_RIGHT,
        LV_ALIGN_BOTTOM_LEFT,
        LV_ALIGN_BOTTOM_MID,
        LV_ALIGN_BOTTOM_RIGHT,
        LV_ALIGN_LEFT_MID,
        LV_ALIGN_RIGHT_MID,
        LV_ALIGN_CENTER,

        LV_ALIGN_OUT_TOP_LEFT,
        LV_ALIGN_OUT_TOP_MID,
        LV_ALIGN_OUT_TOP_RIGHT,
        LV_ALIGN_OUT_BOTTOM_LEFT,
        LV_ALIGN_OUT_BOTTOM_MID,
        LV_ALIGN_OUT_BOTTOM_RIGHT,
        LV_ALIGN_OUT_LEFT_TOP,
        LV_ALIGN_OUT_LEFT_MID,
        LV_ALIGN_OUT_LEFT_BOTTOM,
        LV_ALIGN_OUT_RIGHT_TOP,
        LV_ALIGN_OUT_RIGHT_MID,
        LV_ALIGN_OUT_RIGHT_BOTTOM,
    }

    #[repr(C)]
    pub enum lv_display_render_mode_t {
        LV_DISPLAY_RENDER_MODE_PARTIAL,
        LV_DISPLAY_RENDER_MODE_DIRECT,
        LV_DISPLAY_RENDER_MODE_FULL,
    }
    #[repr(C)]
    pub enum lv_indev_type_t {
        LV_INDEV_TYPE_NONE,
        LV_INDEV_TYPE_POINTER,
        LV_INDEV_TYPE_KEYPAD,
        LV_INDEV_TYPE_BUTTON,
        LV_INDEV_TYPE_ENCODER,
    }

    #[repr(C)]
    pub enum lv_indev_gesture_type_t {
        LV_INDEV_GESTURE_NONE = 0,
        LV_INDEV_GESTURE_PINCH,
        LV_INDEV_GESTURE_SWIPE,
        LV_INDEV_GESTURE_ROTATE,
        LV_INDEV_GESTURE_TWO_FINGERS_SWIPE,
        LV_INDEV_GESTURE_SCROLL,
        LV_INDEV_GESTURE_CNT,
    }

    #[repr(C)]
    pub enum lv_indev_state_t {
        LV_INDEV_STATE_RELEASED = 0,
        LV_INDEV_STATE_PRESSED,
    }

    #[repr(C)]
    pub struct lv_indev_data_t {
        pub gesture_type:
            [lv_indev_gesture_type_t; lv_indev_gesture_type_t::LV_INDEV_GESTURE_CNT as usize],
        pub gesture_data: [*mut c_void; lv_indev_gesture_type_t::LV_INDEV_GESTURE_CNT as usize],

        pub state: lv_indev_state_t,

        pub point: lv_point_t,
        pub key: u32,
        pub btn_id: u32,
        pub enc_diff: i16,

        pub timestamp: u32,
        pub continue_reading: bool,
    }

    unsafe extern "C" {
        pub unsafe fn lv_init();
        pub unsafe fn lv_timer_handler();
        pub unsafe fn lv_tick_set_cb(cb: extern "C" fn() -> u32);

        pub unsafe fn lv_display_create(hor_res: i32, ver_res: i32) -> *mut lv_display_t;
        pub unsafe fn lv_display_set_buffers(
            disp: *mut lv_display_t,
            buf1: *mut c_void,
            buf2: *mut c_void,
            buf_size: u32,
            render_mode: lv_display_render_mode_t,
        );
        pub unsafe fn lv_display_set_flush_cb(
            disp: *mut lv_display_t,
            cb: unsafe extern "C" fn(
                disp: *mut lv_display_t,
                area: *const lv_area_t,
                px_map: *mut u8,
            ),
        );

        pub unsafe fn lv_screen_active() -> *mut lv_obj_t;

        pub unsafe fn lv_label_create(parent: *mut lv_obj_t) -> *mut lv_obj_t;
        pub unsafe fn lv_label_set_text(obj: *mut lv_obj_t, text: *const c_char);

        pub unsafe fn lv_obj_align(obj: *mut lv_obj_t, align: lv_align_t, x_ofs: i32, y_ofs: i32);

        pub unsafe fn lv_display_flush_ready(display: *mut lv_display_t);
        pub unsafe fn lv_display_set_user_data(display: *mut lv_display_t, user_data: *mut c_void);
        pub unsafe fn lv_display_get_user_data(display: *mut lv_display_t) -> *mut c_void;

        pub unsafe fn lv_indev_create() -> *mut lv_indev_t;
        pub unsafe fn lv_indev_set_type(indev: *mut lv_indev_t, typ: lv_indev_type_t);
        pub unsafe fn lv_indev_set_read_cb(
            indev: *mut lv_indev_t,
            cb: unsafe extern "C" fn(indev: *mut lv_indev_t, data: *mut lv_indev_data_t),
        );
        pub unsafe fn lv_indev_set_user_data(display: *mut lv_indev_t, user_data: *mut c_void);
        pub unsafe fn lv_indev_get_user_data(display: *mut lv_indev_t) -> *mut c_void;

    }
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
    unsafe { ffi::lv_timer_handler() }
}

pub fn lv_tick_set_cb(cb: extern "C" fn() -> u32) {
    unsafe { ffi::lv_tick_set_cb(cb) }
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
    let buf1 = buf1.as_ptr() as *mut _;
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
    unsafe { ffi::lv_display_set_flush_cb(display.inner, cb) }
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
    unsafe { ffi::lv_indev_set_read_cb(indev.inner, cb) }
}

/// # Safety
/// user_data must be valid until lvgl is deinitalized
pub unsafe fn lv_indev_set_user_data(indev: &LvIndev, user_data: *mut core::ffi::c_void) {
    unsafe { ffi::lv_indev_set_user_data(indev.inner, user_data) }
}

pub fn lv_indev_get_user_data(indev: &LvIndev) -> *mut core::ffi::c_void {
    unsafe { ffi::lv_indev_get_user_data(indev.inner) }
}
