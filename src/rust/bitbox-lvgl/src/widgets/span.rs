// SPDX-License-Identifier: Apache-2.0

use core::ffi::CStr;
use core::ptr::NonNull;

use ::util::strings::optional_cstr_from_ptr;
use alloc::borrow::ToOwned;
use alloc::ffi::CString;

use crate::{
    LvAlign, LvBaseDir, LvBlendMode, LvBorderSide, LvColor, LvFlexAlign, LvFlexFlow, LvFont,
    LvGradDir, LvGridAlign, LvHandle, LvObj, LvOpa, LvPoint, LvSpanCoords, LvSpanMode,
    LvSpanOverflow, LvTextAlign, LvTextDecor, ObjExt, class, ffi,
};

pub type LvSpanTextError = super::LvTextError;
pub type LvSpangroup = LvHandle<class::SpangroupTag>;

#[derive(Debug, PartialEq, Eq)]
pub struct LvSpan {
    raw: NonNull<ffi::lv_span_t>,
}

macro_rules! impl_span_style_setter_methods {
    ($($name:ident => $ffi_name:ident: $value_ty:ty),+ $(,)?) => {
        $(
            pub fn $name(&self, value: $value_ty) {
                unsafe { ffi::$ffi_name(self.style_ptr(), value) }
            }
        )+
    };
}

macro_rules! impl_span_style_optional_ref_setter_methods {
    ($($name:ident => $ffi_name:ident: $value_ty:ty),+ $(,)?) => {
        $(
            pub fn $name(&self, value: Option<&'static $value_ty>) {
                unsafe {
                    ffi::$ffi_name(
                        self.style_ptr(),
                        value.map_or(core::ptr::null(), |value| value as *const $value_ty),
                    )
                }
            }
        )+
    };
}

macro_rules! impl_span_style_optional_void_ptr_setter_methods {
    ($($name:ident => $ffi_name:ident),+ $(,)?) => {
        $(
            /// # Safety
            /// The pointed value type must exactly match what LVGL expects for this style field.
            /// LVGL stores the raw pointer in the span style, so the value must remain valid and
            /// must not be repurposed for as long as the style can be used. Image-source style
            /// fields must also satisfy LVGL's image source tagging rules.
            pub unsafe fn $name<T>(&self, value: Option<&'static T>) {
                unsafe {
                    ffi::$ffi_name(
                        self.style_ptr(),
                        value.map_or(core::ptr::null(), |value| {
                            value as *const T as *const core::ffi::c_void
                        }),
                    )
                }
            }
        )+
    };
}

impl LvSpan {
    pub(crate) fn from_ptr(raw: NonNull<ffi::lv_span_t>) -> Self {
        Self { raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::lv_span_t {
        self.raw.as_ptr()
    }

    pub fn set_text(&self, text: &str) -> Result<(), LvSpanTextError> {
        let text = CString::new(text).map_err(|_| LvSpanTextError::ContainsNul)?;
        unsafe { ffi::lv_span_set_text(self.as_ptr(), text.as_ptr()) }
        Ok(())
    }

    pub fn set_text_static(&self, text: Option<&'static CStr>) {
        unsafe {
            ffi::lv_span_set_text_static(
                self.as_ptr(),
                text.map_or(core::ptr::null(), CStr::as_ptr),
            )
        }
    }

    /// Returns the span's built-in style.
    ///
    /// After mutating it, call [`SpangroupExt::refresh`] on the parent spangroup so LVGL updates
    /// layout and rendering.
    pub fn get_style(&self) -> NonNull<ffi::lv_style_t> {
        NonNull::new(unsafe { ffi::lv_span_get_style(self.as_ptr()) })
            .expect("lv_span_get_style returned NULL")
    }

    fn style_ptr(&self) -> *mut ffi::lv_style_t {
        self.get_style().as_ptr()
    }

    pub fn get_text(&self) -> Option<CString> {
        unsafe {
            optional_cstr_from_ptr(ffi::lv_span_get_text(self.as_ptr())).map(|text| text.to_owned())
        }
    }

    impl_span_style_setter_methods!(
        set_style_width => lv_style_set_width: i32,
        set_style_min_width => lv_style_set_min_width: i32,
        set_style_max_width => lv_style_set_max_width: i32,
        set_style_height => lv_style_set_height: i32,
        set_style_min_height => lv_style_set_min_height: i32,
        set_style_max_height => lv_style_set_max_height: i32,
        set_style_length => lv_style_set_length: i32,
        set_style_x => lv_style_set_x: i32,
        set_style_y => lv_style_set_y: i32,
        set_style_align => lv_style_set_align: LvAlign,
        set_style_transform_width => lv_style_set_transform_width: i32,
        set_style_transform_height => lv_style_set_transform_height: i32,
        set_style_translate_x => lv_style_set_translate_x: i32,
        set_style_translate_y => lv_style_set_translate_y: i32,
        set_style_translate_radial => lv_style_set_translate_radial: i32,
        set_style_transform_scale_x => lv_style_set_transform_scale_x: i32,
        set_style_transform_scale_y => lv_style_set_transform_scale_y: i32,
        set_style_transform_rotation => lv_style_set_transform_rotation: i32,
        set_style_transform_pivot_x => lv_style_set_transform_pivot_x: i32,
        set_style_transform_pivot_y => lv_style_set_transform_pivot_y: i32,
        set_style_transform_skew_x => lv_style_set_transform_skew_x: i32,
        set_style_transform_skew_y => lv_style_set_transform_skew_y: i32,
        set_style_pad_top => lv_style_set_pad_top: i32,
        set_style_pad_bottom => lv_style_set_pad_bottom: i32,
        set_style_pad_left => lv_style_set_pad_left: i32,
        set_style_pad_right => lv_style_set_pad_right: i32,
        set_style_pad_row => lv_style_set_pad_row: i32,
        set_style_pad_column => lv_style_set_pad_column: i32,
        set_style_pad_radial => lv_style_set_pad_radial: i32,
        set_style_margin_top => lv_style_set_margin_top: i32,
        set_style_margin_bottom => lv_style_set_margin_bottom: i32,
        set_style_margin_left => lv_style_set_margin_left: i32,
        set_style_margin_right => lv_style_set_margin_right: i32,
        set_style_bg_color => lv_style_set_bg_color: LvColor,
        set_style_bg_opa => lv_style_set_bg_opa: LvOpa,
        set_style_bg_grad_color => lv_style_set_bg_grad_color: LvColor,
        set_style_bg_grad_dir => lv_style_set_bg_grad_dir: LvGradDir,
        set_style_bg_main_stop => lv_style_set_bg_main_stop: i32,
        set_style_bg_grad_stop => lv_style_set_bg_grad_stop: i32,
        set_style_bg_main_opa => lv_style_set_bg_main_opa: LvOpa,
        set_style_bg_grad_opa => lv_style_set_bg_grad_opa: LvOpa,
        set_style_bg_image_opa => lv_style_set_bg_image_opa: LvOpa,
        set_style_bg_image_recolor => lv_style_set_bg_image_recolor: LvColor,
        set_style_bg_image_recolor_opa => lv_style_set_bg_image_recolor_opa: LvOpa,
        set_style_bg_image_tiled => lv_style_set_bg_image_tiled: bool,
        set_style_border_color => lv_style_set_border_color: LvColor,
        set_style_border_opa => lv_style_set_border_opa: LvOpa,
        set_style_border_width => lv_style_set_border_width: i32,
        set_style_border_side => lv_style_set_border_side: LvBorderSide,
        set_style_border_post => lv_style_set_border_post: bool,
        set_style_outline_width => lv_style_set_outline_width: i32,
        set_style_outline_color => lv_style_set_outline_color: LvColor,
        set_style_outline_opa => lv_style_set_outline_opa: LvOpa,
        set_style_outline_pad => lv_style_set_outline_pad: i32,
        set_style_shadow_width => lv_style_set_shadow_width: i32,
        set_style_shadow_offset_x => lv_style_set_shadow_offset_x: i32,
        set_style_shadow_offset_y => lv_style_set_shadow_offset_y: i32,
        set_style_shadow_spread => lv_style_set_shadow_spread: i32,
        set_style_shadow_color => lv_style_set_shadow_color: LvColor,
        set_style_shadow_opa => lv_style_set_shadow_opa: LvOpa,
        set_style_image_opa => lv_style_set_image_opa: LvOpa,
        set_style_image_recolor => lv_style_set_image_recolor: LvColor,
        set_style_image_recolor_opa => lv_style_set_image_recolor_opa: LvOpa,
        set_style_line_width => lv_style_set_line_width: i32,
        set_style_line_dash_width => lv_style_set_line_dash_width: i32,
        set_style_line_dash_gap => lv_style_set_line_dash_gap: i32,
        set_style_line_rounded => lv_style_set_line_rounded: bool,
        set_style_line_color => lv_style_set_line_color: LvColor,
        set_style_line_opa => lv_style_set_line_opa: LvOpa,
        set_style_arc_width => lv_style_set_arc_width: i32,
        set_style_arc_rounded => lv_style_set_arc_rounded: bool,
        set_style_arc_color => lv_style_set_arc_color: LvColor,
        set_style_arc_opa => lv_style_set_arc_opa: LvOpa,
        set_style_text_color => lv_style_set_text_color: LvColor,
        set_style_text_opa => lv_style_set_text_opa: LvOpa,
        set_style_text_letter_space => lv_style_set_text_letter_space: i32,
        set_style_text_line_space => lv_style_set_text_line_space: i32,
        set_style_text_decor => lv_style_set_text_decor: LvTextDecor,
        set_style_text_align => lv_style_set_text_align: LvTextAlign,
        set_style_text_outline_stroke_color => lv_style_set_text_outline_stroke_color: LvColor,
        set_style_text_outline_stroke_width => lv_style_set_text_outline_stroke_width: i32,
        set_style_text_outline_stroke_opa => lv_style_set_text_outline_stroke_opa: LvOpa,
        set_style_radius => lv_style_set_radius: i32,
        set_style_radial_offset => lv_style_set_radial_offset: i32,
        set_style_clip_corner => lv_style_set_clip_corner: bool,
        set_style_opa => lv_style_set_opa: LvOpa,
        set_style_opa_layered => lv_style_set_opa_layered: LvOpa,
        set_style_color_filter_opa => lv_style_set_color_filter_opa: LvOpa,
        set_style_recolor => lv_style_set_recolor: LvColor,
        set_style_recolor_opa => lv_style_set_recolor_opa: LvOpa,
        set_style_anim_duration => lv_style_set_anim_duration: u32,
        set_style_blend_mode => lv_style_set_blend_mode: LvBlendMode,
        set_style_layout => lv_style_set_layout: u16,
        set_style_base_dir => lv_style_set_base_dir: LvBaseDir,
        set_style_rotary_sensitivity => lv_style_set_rotary_sensitivity: u32,
        set_style_flex_flow => lv_style_set_flex_flow: LvFlexFlow,
        set_style_flex_main_place => lv_style_set_flex_main_place: LvFlexAlign,
        set_style_flex_cross_place => lv_style_set_flex_cross_place: LvFlexAlign,
        set_style_flex_track_place => lv_style_set_flex_track_place: LvFlexAlign,
        set_style_flex_grow => lv_style_set_flex_grow: u8,
        set_style_grid_column_align => lv_style_set_grid_column_align: LvGridAlign,
        set_style_grid_row_align => lv_style_set_grid_row_align: LvGridAlign,
        set_style_grid_cell_column_pos => lv_style_set_grid_cell_column_pos: i32,
        set_style_grid_cell_x_align => lv_style_set_grid_cell_x_align: LvGridAlign,
        set_style_grid_cell_column_span => lv_style_set_grid_cell_column_span: i32,
        set_style_grid_cell_row_pos => lv_style_set_grid_cell_row_pos: i32,
        set_style_grid_cell_y_align => lv_style_set_grid_cell_y_align: LvGridAlign,
        set_style_grid_cell_row_span => lv_style_set_grid_cell_row_span: i32,
    );

    impl_span_style_optional_ref_setter_methods!(
        set_style_bg_grad => lv_style_set_bg_grad: ffi::lv_grad_dsc_t,
        set_style_image_colorkey => lv_style_set_image_colorkey: ffi::lv_image_colorkey_t,
        set_style_color_filter_dsc => lv_style_set_color_filter_dsc: ffi::lv_color_filter_dsc_t,
        set_style_anim => lv_style_set_anim: ffi::lv_anim_t,
        set_style_transition => lv_style_set_transition: ffi::lv_style_transition_dsc_t,
    );

    impl_span_style_optional_void_ptr_setter_methods!(
        set_style_bg_image_src => lv_style_set_bg_image_src,
        set_style_arc_image_src => lv_style_set_arc_image_src,
        set_style_bitmap_mask_src => lv_style_set_bitmap_mask_src,
    );

    pub fn set_style_text_font(&self, value: LvFont) {
        unsafe { ffi::lv_style_set_text_font(self.style_ptr(), value.as_ptr()) }
    }

    pub fn remove_style_text_font(&self) -> bool {
        unsafe {
            ffi::lv_style_remove_prop(
                self.style_ptr(),
                ffi::_lv_style_id_t::LV_STYLE_TEXT_FONT as ffi::lv_style_prop_t,
            )
        }
    }

    pub fn set_style_grid_column_dsc_array(&self, value: Option<&'static [i32]>) {
        if let Some(value) = value
            && let Some(last) = value.last()
        {
            if *last != ffi::LV_GRID_TEMPLATE_LAST as i32 {
                panic!("invalid input");
            }
            unsafe { ffi::lv_style_set_grid_column_dsc_array(self.style_ptr(), value.as_ptr()) }
        }
    }

    pub fn set_style_grid_row_dsc_array(&self, value: Option<&'static [i32]>) {
        if let Some(value) = value
            && let Some(last) = value.last()
        {
            if *last != ffi::LV_GRID_TEMPLATE_LAST as i32 {
                panic!("invalid input");
            }
            unsafe { ffi::lv_style_set_grid_row_dsc_array(self.style_ptr(), value.as_ptr()) }
        }
    }
}

pub trait SpangroupExt: ObjExt {
    fn add_span(&self) -> Option<LvSpan> {
        NonNull::new(unsafe { ffi::lv_spangroup_add_span(self.as_ptr()) }).map(LvSpan::from_ptr)
    }

    /// # Safety
    ///
    /// LVGL frees the span immediately. Callers must ensure that no Rust handles derived from the
    /// deleted span are used again.
    unsafe fn delete_span(&self, span: LvSpan) {
        unsafe { ffi::lv_spangroup_delete_span(self.as_ptr(), span.as_ptr()) }
    }

    fn set_span_text(&self, span: &LvSpan, text: &str) -> Result<(), LvSpanTextError> {
        let text = CString::new(text).map_err(|_| LvSpanTextError::ContainsNul)?;
        unsafe { ffi::lv_spangroup_set_span_text(self.as_ptr(), span.as_ptr(), text.as_ptr()) }
        Ok(())
    }

    fn set_span_text_static(&self, span: &LvSpan, text: Option<&'static CStr>) {
        unsafe {
            ffi::lv_spangroup_set_span_text_static(
                self.as_ptr(),
                span.as_ptr(),
                text.map_or(core::ptr::null(), CStr::as_ptr),
            )
        }
    }

    fn set_span_style(&self, span: &LvSpan, style: &ffi::lv_style_t) {
        unsafe { ffi::lv_spangroup_set_span_style(self.as_ptr(), span.as_ptr(), style) }
    }

    fn set_align(&self, align: LvTextAlign) {
        unsafe { ffi::lv_spangroup_set_align(self.as_ptr(), align) }
    }

    fn set_overflow(&self, overflow: LvSpanOverflow) {
        unsafe { ffi::lv_spangroup_set_overflow(self.as_ptr(), overflow) }
    }

    fn set_indent(&self, indent: i32) {
        unsafe { ffi::lv_spangroup_set_indent(self.as_ptr(), indent) }
    }

    fn set_mode(&self, mode: LvSpanMode) {
        unsafe { ffi::lv_spangroup_set_mode(self.as_ptr(), mode) }
    }

    fn set_max_lines(&self, lines: i32) {
        unsafe { ffi::lv_spangroup_set_max_lines(self.as_ptr(), lines) }
    }

    fn get_child(&self, index: i32) -> Option<LvSpan> {
        NonNull::new(unsafe { ffi::lv_spangroup_get_child(self.as_ptr(), index) })
            .map(LvSpan::from_ptr)
    }

    fn get_span_count(&self) -> u32 {
        unsafe { ffi::lv_spangroup_get_span_count(self.as_ptr()) }
    }

    fn get_align(&self) -> LvTextAlign {
        unsafe { ffi::lv_spangroup_get_align(self.as_ptr()) }
    }

    fn get_overflow(&self) -> LvSpanOverflow {
        unsafe { ffi::lv_spangroup_get_overflow(self.as_ptr()) }
    }

    fn get_indent(&self) -> i32 {
        unsafe { ffi::lv_spangroup_get_indent(self.as_ptr()) }
    }

    fn get_mode(&self) -> LvSpanMode {
        unsafe { ffi::lv_spangroup_get_mode(self.as_ptr()) }
    }

    fn get_max_lines(&self) -> i32 {
        unsafe { ffi::lv_spangroup_get_max_lines(self.as_ptr()) }
    }

    fn get_max_line_height(&self) -> i32 {
        unsafe { ffi::lv_spangroup_get_max_line_height(self.as_ptr()) }
    }

    fn get_expand_width(&self, max_width: u32) -> u32 {
        unsafe { ffi::lv_spangroup_get_expand_width(self.as_ptr(), max_width) }
    }

    fn get_expand_height(&self, width: i32) -> i32 {
        unsafe { ffi::lv_spangroup_get_expand_height(self.as_ptr(), width) }
    }

    fn get_span_coords(&self, span: &LvSpan) -> LvSpanCoords {
        unsafe { ffi::lv_spangroup_get_span_coords(self.as_ptr(), span.as_ptr()) }
    }

    fn get_span_by_point(&self, point: &LvPoint) -> Option<LvSpan> {
        NonNull::new(unsafe { ffi::lv_spangroup_get_span_by_point(self.as_ptr(), point) })
            .map(LvSpan::from_ptr)
    }

    fn refresh(&self) {
        unsafe { ffi::lv_spangroup_refresh(self.as_ptr()) }
    }
}

impl LvHandle<class::SpangroupTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_spangroup_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl SpangroupExt for LvHandle<class::SpangroupTag> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_ptr_roundtrip() {
        let raw = NonNull::dangling();
        let span = LvSpan::from_ptr(raw);
        assert_eq!(span.as_ptr(), raw.as_ptr());
    }

    #[test]
    fn test_span_style_methods_exist() {
        let _: fn(&LvSpan, crate::LvColor) = LvSpan::set_style_text_color;
        let _: fn(&LvSpan, crate::LvFont) = LvSpan::set_style_text_font;
        let _: fn(&LvSpan) -> bool = LvSpan::remove_style_text_font;
        let _: fn(&LvSpan, Option<&'static [i32]>) = LvSpan::set_style_grid_column_dsc_array;
        let _: fn(&LvSpan, Option<&'static [i32]>) = LvSpan::set_style_grid_row_dsc_array;
        let _: unsafe fn(&LvSpan, Option<&'static u8>) = LvSpan::set_style_bg_image_src::<u8>;
    }
}
