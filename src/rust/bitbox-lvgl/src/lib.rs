#![no_std]

extern crate alloc;

use core::ffi::c_void;
use core::ptr::NonNull;

pub mod ffi {
    pub use bitbox_lvgl_sys::*;
}

pub use ffi::lv_align_t as LvAlign;
pub use ffi::lv_area_t as LvArea;
pub use ffi::lv_base_dir_t as LvBaseDir;
pub use ffi::lv_blend_mode_t as LvBlendMode;
pub use ffi::lv_border_side_t as LvBorderSide;
pub use ffi::lv_color_t as LvColor;
pub use ffi::lv_display_render_mode_t as LvDisplayRenderMode;
pub use ffi::lv_flex_align_t as LvFlexAlign;
pub use ffi::lv_flex_flow_t as LvFlexFlow;
pub use ffi::lv_grad_dir_t as LvGradDir;
pub use ffi::lv_grid_align_t as LvGridAlign;
pub use ffi::lv_indev_state_t as LvIndevState;
pub use ffi::lv_indev_type_t as LvIndevType;
pub use ffi::lv_opa_t as LvOpa;
pub use ffi::lv_point_t as LvPoint;
pub use ffi::lv_style_selector_t as LvStyleSelector;
pub use ffi::lv_text_align_t as LvTextAlign;
pub use ffi::lv_text_decor_t as LvTextDecor;

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

macro_rules! impl_lv_obj_style_setter {
    ($($name:ident: $value_ty:ty),+ $(,)?) => {
        $(
            pub fn $name(obj: &LvObj, value: $value_ty, selector: LvStyleSelector) {
                unsafe { ffi::$name(obj.inner.as_ptr(), value, selector) }
            }
        )+
    };
}

macro_rules! impl_lv_obj_style_optional_ref_setter {
    ($($name:ident: $value_ty:ty),+ $(,)?) => {
        $(
            pub fn $name(obj: &LvObj, value: Option<&$value_ty>, selector: LvStyleSelector) {
                unsafe {
                    ffi::$name(
                        obj.inner.as_ptr(),
                        value.map_or(core::ptr::null(), |value| value as *const $value_ty),
                        selector,
                    )
                }
            }
        )+
    };
}

macro_rules! impl_lv_obj_style_optional_void_ptr_setter {
    ($($name:ident),+ $(,)?) => {
        $(
            pub fn $name(obj: &LvObj, value: Option<NonNull<c_void>>, selector: LvStyleSelector) {
                unsafe {
                    ffi::$name(
                        obj.inner.as_ptr(),
                        value.map_or(core::ptr::null(), |value| value.as_ptr() as *const c_void),
                        selector,
                    )
                }
            }
        )+
    };
}

pub fn lv_color_make(red: u8, green: u8, blue: u8) -> LvColor {
    unsafe { ffi::lv_color_make(red, green, blue) }
}

pub fn lv_color_hex(color: u32) -> LvColor {
    unsafe { ffi::lv_color_hex(color) }
}

pub fn lv_color_hex3(color: u32) -> LvColor {
    unsafe { ffi::lv_color_hex3(color) }
}

pub fn lv_color_white() -> LvColor {
    unsafe { ffi::lv_color_white() }
}

pub fn lv_color_black() -> LvColor {
    unsafe { ffi::lv_color_black() }
}

impl_lv_obj_style_setter!(
    lv_obj_set_style_width: i32,
    lv_obj_set_style_min_width: i32,
    lv_obj_set_style_max_width: i32,
    lv_obj_set_style_height: i32,
    lv_obj_set_style_min_height: i32,
    lv_obj_set_style_max_height: i32,
    lv_obj_set_style_length: i32,
    lv_obj_set_style_x: i32,
    lv_obj_set_style_y: i32,
    lv_obj_set_style_align: LvAlign,
    lv_obj_set_style_transform_width: i32,
    lv_obj_set_style_transform_height: i32,
    lv_obj_set_style_translate_x: i32,
    lv_obj_set_style_translate_y: i32,
    lv_obj_set_style_translate_radial: i32,
    lv_obj_set_style_transform_scale_x: i32,
    lv_obj_set_style_transform_scale_y: i32,
    lv_obj_set_style_transform_rotation: i32,
    lv_obj_set_style_transform_pivot_x: i32,
    lv_obj_set_style_transform_pivot_y: i32,
    lv_obj_set_style_transform_skew_x: i32,
    lv_obj_set_style_transform_skew_y: i32,
    lv_obj_set_style_pad_top: i32,
    lv_obj_set_style_pad_bottom: i32,
    lv_obj_set_style_pad_left: i32,
    lv_obj_set_style_pad_right: i32,
    lv_obj_set_style_pad_row: i32,
    lv_obj_set_style_pad_column: i32,
    lv_obj_set_style_pad_radial: i32,
    lv_obj_set_style_margin_top: i32,
    lv_obj_set_style_margin_bottom: i32,
    lv_obj_set_style_margin_left: i32,
    lv_obj_set_style_margin_right: i32,
    lv_obj_set_style_bg_color: LvColor,
    lv_obj_set_style_bg_opa: LvOpa,
    lv_obj_set_style_bg_grad_color: LvColor,
    lv_obj_set_style_bg_grad_dir: LvGradDir,
    lv_obj_set_style_bg_main_stop: i32,
    lv_obj_set_style_bg_grad_stop: i32,
    lv_obj_set_style_bg_main_opa: LvOpa,
    lv_obj_set_style_bg_grad_opa: LvOpa,
    lv_obj_set_style_bg_image_opa: LvOpa,
    lv_obj_set_style_bg_image_recolor: LvColor,
    lv_obj_set_style_bg_image_recolor_opa: LvOpa,
    lv_obj_set_style_bg_image_tiled: bool,
    lv_obj_set_style_border_color: LvColor,
    lv_obj_set_style_border_opa: LvOpa,
    lv_obj_set_style_border_width: i32,
    lv_obj_set_style_border_side: LvBorderSide,
    lv_obj_set_style_border_post: bool,
    lv_obj_set_style_outline_width: i32,
    lv_obj_set_style_outline_color: LvColor,
    lv_obj_set_style_outline_opa: LvOpa,
    lv_obj_set_style_outline_pad: i32,
    lv_obj_set_style_shadow_width: i32,
    lv_obj_set_style_shadow_offset_x: i32,
    lv_obj_set_style_shadow_offset_y: i32,
    lv_obj_set_style_shadow_spread: i32,
    lv_obj_set_style_shadow_color: LvColor,
    lv_obj_set_style_shadow_opa: LvOpa,
    lv_obj_set_style_image_opa: LvOpa,
    lv_obj_set_style_image_recolor: LvColor,
    lv_obj_set_style_image_recolor_opa: LvOpa,
    lv_obj_set_style_line_width: i32,
    lv_obj_set_style_line_dash_width: i32,
    lv_obj_set_style_line_dash_gap: i32,
    lv_obj_set_style_line_rounded: bool,
    lv_obj_set_style_line_color: LvColor,
    lv_obj_set_style_line_opa: LvOpa,
    lv_obj_set_style_arc_width: i32,
    lv_obj_set_style_arc_rounded: bool,
    lv_obj_set_style_arc_color: LvColor,
    lv_obj_set_style_arc_opa: LvOpa,
    lv_obj_set_style_text_color: LvColor,
    lv_obj_set_style_text_opa: LvOpa,
    lv_obj_set_style_text_letter_space: i32,
    lv_obj_set_style_text_line_space: i32,
    lv_obj_set_style_text_decor: LvTextDecor,
    lv_obj_set_style_text_align: LvTextAlign,
    lv_obj_set_style_text_outline_stroke_color: LvColor,
    lv_obj_set_style_text_outline_stroke_width: i32,
    lv_obj_set_style_text_outline_stroke_opa: LvOpa,
    lv_obj_set_style_radius: i32,
    lv_obj_set_style_radial_offset: i32,
    lv_obj_set_style_clip_corner: bool,
    lv_obj_set_style_opa: LvOpa,
    lv_obj_set_style_opa_layered: LvOpa,
    lv_obj_set_style_color_filter_opa: LvOpa,
    lv_obj_set_style_recolor: LvColor,
    lv_obj_set_style_recolor_opa: LvOpa,
    lv_obj_set_style_anim_duration: u32,
    lv_obj_set_style_blend_mode: LvBlendMode,
    lv_obj_set_style_layout: u16,
    lv_obj_set_style_base_dir: LvBaseDir,
    lv_obj_set_style_rotary_sensitivity: u32,
    lv_obj_set_style_flex_flow: LvFlexFlow,
    lv_obj_set_style_flex_main_place: LvFlexAlign,
    lv_obj_set_style_flex_cross_place: LvFlexAlign,
    lv_obj_set_style_flex_track_place: LvFlexAlign,
    lv_obj_set_style_flex_grow: u8,
    lv_obj_set_style_grid_column_align: LvGridAlign,
    lv_obj_set_style_grid_row_align: LvGridAlign,
    lv_obj_set_style_grid_cell_column_pos: i32,
    lv_obj_set_style_grid_cell_x_align: LvGridAlign,
    lv_obj_set_style_grid_cell_column_span: i32,
    lv_obj_set_style_grid_cell_row_pos: i32,
    lv_obj_set_style_grid_cell_y_align: LvGridAlign,
    lv_obj_set_style_grid_cell_row_span: i32,
);

impl_lv_obj_style_optional_ref_setter!(
    lv_obj_set_style_bg_grad: ffi::lv_grad_dsc_t,
    lv_obj_set_style_image_colorkey: ffi::lv_image_colorkey_t,
    lv_obj_set_style_text_font: ffi::lv_font_t,
    lv_obj_set_style_color_filter_dsc: ffi::lv_color_filter_dsc_t,
    lv_obj_set_style_anim: ffi::lv_anim_t,
    lv_obj_set_style_transition: ffi::lv_style_transition_dsc_t,
);

impl_lv_obj_style_optional_void_ptr_setter!(
    lv_obj_set_style_bg_image_src,
    lv_obj_set_style_arc_image_src,
    lv_obj_set_style_bitmap_mask_src,
);

pub fn lv_obj_set_style_grid_column_dsc_array(
    obj: &LvObj,
    value: Option<&[i32]>,
    selector: LvStyleSelector,
) {
    unsafe {
        ffi::lv_obj_set_style_grid_column_dsc_array(
            obj.inner.as_ptr(),
            value.map_or(core::ptr::null(), <[i32]>::as_ptr),
            selector,
        )
    }
}

pub fn lv_obj_set_style_grid_row_dsc_array(
    obj: &LvObj,
    value: Option<&[i32]>,
    selector: LvStyleSelector,
) {
    unsafe {
        ffi::lv_obj_set_style_grid_row_dsc_array(
            obj.inner.as_ptr(),
            value.map_or(core::ptr::null(), <[i32]>::as_ptr),
            selector,
        )
    }
}

pub fn lv_obj_set_style_pad_all(obj: &LvObj, value: i32, selector: LvStyleSelector) {
    lv_obj_set_style_pad_left(obj, value, selector);
    lv_obj_set_style_pad_right(obj, value, selector);
    lv_obj_set_style_pad_top(obj, value, selector);
    lv_obj_set_style_pad_bottom(obj, value, selector);
}

pub fn lv_obj_set_style_pad_hor(obj: &LvObj, value: i32, selector: LvStyleSelector) {
    lv_obj_set_style_pad_left(obj, value, selector);
    lv_obj_set_style_pad_right(obj, value, selector);
}

pub fn lv_obj_set_style_pad_ver(obj: &LvObj, value: i32, selector: LvStyleSelector) {
    lv_obj_set_style_pad_top(obj, value, selector);
    lv_obj_set_style_pad_bottom(obj, value, selector);
}

pub fn lv_obj_set_style_margin_all(obj: &LvObj, value: i32, selector: LvStyleSelector) {
    lv_obj_set_style_margin_left(obj, value, selector);
    lv_obj_set_style_margin_right(obj, value, selector);
    lv_obj_set_style_margin_top(obj, value, selector);
    lv_obj_set_style_margin_bottom(obj, value, selector);
}

pub fn lv_obj_set_style_margin_hor(obj: &LvObj, value: i32, selector: LvStyleSelector) {
    lv_obj_set_style_margin_left(obj, value, selector);
    lv_obj_set_style_margin_right(obj, value, selector);
}

pub fn lv_obj_set_style_margin_ver(obj: &LvObj, value: i32, selector: LvStyleSelector) {
    lv_obj_set_style_margin_top(obj, value, selector);
    lv_obj_set_style_margin_bottom(obj, value, selector);
}

pub fn lv_obj_set_style_pad_gap(obj: &LvObj, value: i32, selector: LvStyleSelector) {
    lv_obj_set_style_pad_row(obj, value, selector);
    lv_obj_set_style_pad_column(obj, value, selector);
}

pub fn lv_obj_set_style_size(obj: &LvObj, width: i32, height: i32, selector: LvStyleSelector) {
    lv_obj_set_style_width(obj, width, selector);
    lv_obj_set_style_height(obj, height, selector);
}

pub fn lv_obj_set_style_transform_scale(obj: &LvObj, value: i32, selector: LvStyleSelector) {
    lv_obj_set_style_transform_scale_x(obj, value, selector);
    lv_obj_set_style_transform_scale_y(obj, value, selector);
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
