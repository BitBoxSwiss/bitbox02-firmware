// SPDX-License-Identifier: Apache-2.0

use super::util;
use super::util::LvEventRegistrationError;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::{
    LvAlign, LvBaseDir, LvBlendMode, LvBorderSide, LvColor, LvEventCode, LvFlexAlign, LvFlexFlow,
    LvFont, LvGradDir, LvGridAlign, LvOpa, LvStyleSelector, LvTextAlign, LvTextDecor, class, ffi,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvTypeError {
    ClassMismatch,
}

pub type LvObj = LvHandle<class::ObjTag>;

#[derive(Debug, PartialEq, Eq)]
pub struct LvHandle<C: class::LvClass = class::ObjTag> {
    raw: NonNull<ffi::lv_obj_t>,
    _class: PhantomData<C>,
}

impl<C: class::LvClass> LvHandle<C> {
    pub(crate) fn from_ptr(raw: NonNull<ffi::lv_obj_t>) -> Self {
        Self {
            raw,
            _class: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::lv_obj_t {
        self.raw.as_ptr()
    }

    pub(crate) fn cast<Base: class::LvClass>(self) -> LvHandle<Base> {
        LvHandle::from_ptr(self.raw)
    }

    pub fn has_class<Target: class::LvClass>(&self) -> bool {
        unsafe { ffi::lv_obj_has_class(self.as_ptr(), Target::class_ptr()) }
    }

    pub fn try_downcast<Target: class::LvClass>(self) -> Result<LvHandle<Target>, LvTypeError> {
        if self.has_class::<Target>() {
            Ok(LvHandle::from_ptr(self.raw))
        } else {
            Err(LvTypeError::ClassMismatch)
        }
    }
}

impl LvHandle<class::ObjTag> {
    pub fn new() -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_obj_create(core::ptr::null_mut()) }).map(LvHandle::from_ptr)
    }

    pub fn with_parent(parent: &LvHandle<impl class::LvClass>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_obj_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }
}

macro_rules! impl_obj_style_setter_methods {
    ($($name:ident => $ffi_name:ident: $value_ty:ty),+ $(,)?) => {
        $(
            fn $name(&self, value: $value_ty, selector: LvStyleSelector) {
                unsafe { ffi::$ffi_name(self.as_ptr(), value, selector) }
            }
        )+
    };
}

macro_rules! impl_obj_style_optional_ref_setter_methods {
    ($($name:ident => $ffi_name:ident: $value_ty:ty),+ $(,)?) => {
        $(
            fn $name(&self, value: Option<&'static $value_ty>, selector: LvStyleSelector) {
                unsafe {
                    ffi::$ffi_name(
                        self.as_ptr(),
                        value.map_or(core::ptr::null(), |value| value as *const $value_ty),
                        selector,
                    )
                }
            }
        )+
    };
}

macro_rules! impl_obj_style_optional_void_ptr_setter_methods {
    ($($name:ident => $ffi_name:ident),+ $(,)?) => {
        $(
            /// # Safety
            /// The pointed value type must exactly match what LVGL expects for this style field.
            /// LVGL stores the raw pointer in the style state, so the value must remain valid and
            /// must not be repurposed for as long as the style can be used. Image-source style
            /// fields must also satisfy LVGL's image source tagging rules.
            unsafe fn $name<T>(&self, value: Option<&'static T>, selector: LvStyleSelector) {
                unsafe {
                    ffi::$ffi_name(
                        self.as_ptr(),
                        value.map_or(core::ptr::null(), |value| value as *const T as *const c_void),
                        selector,
                    )
                }
            }
        )+
    };
}

pub trait ObjExt {
    fn as_ptr(&self) -> *mut ffi::lv_obj_t;

    fn align(&self, align: LvAlign, x_ofs: i32, y_ofs: i32) {
        unsafe { ffi::lv_obj_align(self.as_ptr(), align, x_ofs, y_ofs) }
    }

    fn set_pos(&self, x: i32, y: i32) {
        unsafe { ffi::lv_obj_set_pos(self.as_ptr(), x, y) }
    }

    fn set_size(&self, width: i32, height: i32) {
        unsafe { ffi::lv_obj_set_size(self.as_ptr(), width, height) }
    }

    fn set_width(&self, width: i32) {
        unsafe { ffi::lv_obj_set_width(self.as_ptr(), width) }
    }

    fn set_height(&self, height: i32) {
        unsafe { ffi::lv_obj_set_height(self.as_ptr(), height) }
    }

    fn add_event_cb<F>(&self, filter: LvEventCode, cb: F) -> Result<(), LvEventRegistrationError>
    where
        F: FnMut() + 'static,
    {
        util::add_event_cb(self.as_ptr(), filter, cb)
    }

    fn add_click_cb<F>(&self, cb: F) -> Result<(), LvEventRegistrationError>
    where
        F: FnMut() + 'static,
    {
        self.add_event_cb(crate::LvEventCode::LV_EVENT_CLICKED, cb)
    }

    /// # Safety
    ///
    /// After deletion, LVGL frees the object and may recursively free its children. Callers must
    /// ensure that no other Rust handles to the same object tree are used again.
    unsafe fn delete(self)
    where
        Self: Sized,
    {
        unsafe { ffi::lv_obj_delete(self.as_ptr()) }
    }

    fn set_layout(&self, layout: crate::LvLayout) {
        unsafe { ffi::lv_obj_set_layout(self.as_ptr(), layout as u32) }
    }
    fn set_flex_flow(&self, flow: crate::LvFlexFlow) {
        unsafe { ffi::lv_obj_set_flex_flow(self.as_ptr(), flow) }
    }

    impl_obj_style_setter_methods!(
        set_style_width => lv_obj_set_style_width: i32,
        set_style_min_width => lv_obj_set_style_min_width: i32,
        set_style_max_width => lv_obj_set_style_max_width: i32,
        set_style_height => lv_obj_set_style_height: i32,
        set_style_min_height => lv_obj_set_style_min_height: i32,
        set_style_max_height => lv_obj_set_style_max_height: i32,
        set_style_length => lv_obj_set_style_length: i32,
        set_style_x => lv_obj_set_style_x: i32,
        set_style_y => lv_obj_set_style_y: i32,
        set_style_align => lv_obj_set_style_align: LvAlign,
        set_style_transform_width => lv_obj_set_style_transform_width: i32,
        set_style_transform_height => lv_obj_set_style_transform_height: i32,
        set_style_translate_x => lv_obj_set_style_translate_x: i32,
        set_style_translate_y => lv_obj_set_style_translate_y: i32,
        set_style_translate_radial => lv_obj_set_style_translate_radial: i32,
        set_style_transform_scale_x => lv_obj_set_style_transform_scale_x: i32,
        set_style_transform_scale_y => lv_obj_set_style_transform_scale_y: i32,
        set_style_transform_rotation => lv_obj_set_style_transform_rotation: i32,
        set_style_transform_pivot_x => lv_obj_set_style_transform_pivot_x: i32,
        set_style_transform_pivot_y => lv_obj_set_style_transform_pivot_y: i32,
        set_style_transform_skew_x => lv_obj_set_style_transform_skew_x: i32,
        set_style_transform_skew_y => lv_obj_set_style_transform_skew_y: i32,
        set_style_pad_top => lv_obj_set_style_pad_top: i32,
        set_style_pad_bottom => lv_obj_set_style_pad_bottom: i32,
        set_style_pad_left => lv_obj_set_style_pad_left: i32,
        set_style_pad_right => lv_obj_set_style_pad_right: i32,
        set_style_pad_row => lv_obj_set_style_pad_row: i32,
        set_style_pad_column => lv_obj_set_style_pad_column: i32,
        set_style_pad_radial => lv_obj_set_style_pad_radial: i32,
        set_style_margin_top => lv_obj_set_style_margin_top: i32,
        set_style_margin_bottom => lv_obj_set_style_margin_bottom: i32,
        set_style_margin_left => lv_obj_set_style_margin_left: i32,
        set_style_margin_right => lv_obj_set_style_margin_right: i32,
        set_style_bg_color => lv_obj_set_style_bg_color: LvColor,
        set_style_bg_opa => lv_obj_set_style_bg_opa: LvOpa,
        set_style_bg_grad_color => lv_obj_set_style_bg_grad_color: LvColor,
        set_style_bg_grad_dir => lv_obj_set_style_bg_grad_dir: LvGradDir,
        set_style_bg_main_stop => lv_obj_set_style_bg_main_stop: i32,
        set_style_bg_grad_stop => lv_obj_set_style_bg_grad_stop: i32,
        set_style_bg_main_opa => lv_obj_set_style_bg_main_opa: LvOpa,
        set_style_bg_grad_opa => lv_obj_set_style_bg_grad_opa: LvOpa,
        set_style_bg_image_opa => lv_obj_set_style_bg_image_opa: LvOpa,
        set_style_bg_image_recolor => lv_obj_set_style_bg_image_recolor: LvColor,
        set_style_bg_image_recolor_opa => lv_obj_set_style_bg_image_recolor_opa: LvOpa,
        set_style_bg_image_tiled => lv_obj_set_style_bg_image_tiled: bool,
        set_style_border_color => lv_obj_set_style_border_color: LvColor,
        set_style_border_opa => lv_obj_set_style_border_opa: LvOpa,
        set_style_border_width => lv_obj_set_style_border_width: i32,
        set_style_border_side => lv_obj_set_style_border_side: LvBorderSide,
        set_style_border_post => lv_obj_set_style_border_post: bool,
        set_style_outline_width => lv_obj_set_style_outline_width: i32,
        set_style_outline_color => lv_obj_set_style_outline_color: LvColor,
        set_style_outline_opa => lv_obj_set_style_outline_opa: LvOpa,
        set_style_outline_pad => lv_obj_set_style_outline_pad: i32,
        set_style_shadow_width => lv_obj_set_style_shadow_width: i32,
        set_style_shadow_offset_x => lv_obj_set_style_shadow_offset_x: i32,
        set_style_shadow_offset_y => lv_obj_set_style_shadow_offset_y: i32,
        set_style_shadow_spread => lv_obj_set_style_shadow_spread: i32,
        set_style_shadow_color => lv_obj_set_style_shadow_color: LvColor,
        set_style_shadow_opa => lv_obj_set_style_shadow_opa: LvOpa,
        set_style_image_opa => lv_obj_set_style_image_opa: LvOpa,
        set_style_image_recolor => lv_obj_set_style_image_recolor: LvColor,
        set_style_image_recolor_opa => lv_obj_set_style_image_recolor_opa: LvOpa,
        set_style_line_width => lv_obj_set_style_line_width: i32,
        set_style_line_dash_width => lv_obj_set_style_line_dash_width: i32,
        set_style_line_dash_gap => lv_obj_set_style_line_dash_gap: i32,
        set_style_line_rounded => lv_obj_set_style_line_rounded: bool,
        set_style_line_color => lv_obj_set_style_line_color: LvColor,
        set_style_line_opa => lv_obj_set_style_line_opa: LvOpa,
        set_style_arc_width => lv_obj_set_style_arc_width: i32,
        set_style_arc_rounded => lv_obj_set_style_arc_rounded: bool,
        set_style_arc_color => lv_obj_set_style_arc_color: LvColor,
        set_style_arc_opa => lv_obj_set_style_arc_opa: LvOpa,
        set_style_text_color => lv_obj_set_style_text_color: LvColor,
        set_style_text_opa => lv_obj_set_style_text_opa: LvOpa,
        set_style_text_letter_space => lv_obj_set_style_text_letter_space: i32,
        set_style_text_line_space => lv_obj_set_style_text_line_space: i32,
        set_style_text_decor => lv_obj_set_style_text_decor: LvTextDecor,
        set_style_text_align => lv_obj_set_style_text_align: LvTextAlign,
        set_style_text_outline_stroke_color => lv_obj_set_style_text_outline_stroke_color: LvColor,
        set_style_text_outline_stroke_width => lv_obj_set_style_text_outline_stroke_width: i32,
        set_style_text_outline_stroke_opa => lv_obj_set_style_text_outline_stroke_opa: LvOpa,
        set_style_radius => lv_obj_set_style_radius: i32,
        set_style_radial_offset => lv_obj_set_style_radial_offset: i32,
        set_style_clip_corner => lv_obj_set_style_clip_corner: bool,
        set_style_opa => lv_obj_set_style_opa: LvOpa,
        set_style_opa_layered => lv_obj_set_style_opa_layered: LvOpa,
        set_style_color_filter_opa => lv_obj_set_style_color_filter_opa: LvOpa,
        set_style_recolor => lv_obj_set_style_recolor: LvColor,
        set_style_recolor_opa => lv_obj_set_style_recolor_opa: LvOpa,
        set_style_anim_duration => lv_obj_set_style_anim_duration: u32,
        set_style_blend_mode => lv_obj_set_style_blend_mode: LvBlendMode,
        set_style_layout => lv_obj_set_style_layout: u16,
        set_style_base_dir => lv_obj_set_style_base_dir: LvBaseDir,
        set_style_rotary_sensitivity => lv_obj_set_style_rotary_sensitivity: u32,
        set_style_flex_flow => lv_obj_set_style_flex_flow: LvFlexFlow,
        set_style_flex_main_place => lv_obj_set_style_flex_main_place: LvFlexAlign,
        set_style_flex_cross_place => lv_obj_set_style_flex_cross_place: LvFlexAlign,
        set_style_flex_track_place => lv_obj_set_style_flex_track_place: LvFlexAlign,
        set_style_flex_grow => lv_obj_set_style_flex_grow: u8,
        set_style_grid_column_align => lv_obj_set_style_grid_column_align: LvGridAlign,
        set_style_grid_row_align => lv_obj_set_style_grid_row_align: LvGridAlign,
        set_style_grid_cell_column_pos => lv_obj_set_style_grid_cell_column_pos: i32,
        set_style_grid_cell_x_align => lv_obj_set_style_grid_cell_x_align: LvGridAlign,
        set_style_grid_cell_column_span => lv_obj_set_style_grid_cell_column_span: i32,
        set_style_grid_cell_row_pos => lv_obj_set_style_grid_cell_row_pos: i32,
        set_style_grid_cell_y_align => lv_obj_set_style_grid_cell_y_align: LvGridAlign,
        set_style_grid_cell_row_span => lv_obj_set_style_grid_cell_row_span: i32,
    );

    impl_obj_style_optional_ref_setter_methods!(
        set_style_bg_grad => lv_obj_set_style_bg_grad: ffi::lv_grad_dsc_t,
        set_style_image_colorkey => lv_obj_set_style_image_colorkey: ffi::lv_image_colorkey_t,
        set_style_color_filter_dsc => lv_obj_set_style_color_filter_dsc: ffi::lv_color_filter_dsc_t,
        set_style_anim => lv_obj_set_style_anim: ffi::lv_anim_t,
        set_style_transition => lv_obj_set_style_transition: ffi::lv_style_transition_dsc_t,
    );

    impl_obj_style_optional_void_ptr_setter_methods!(
        set_style_bg_image_src => lv_obj_set_style_bg_image_src,
        set_style_arc_image_src => lv_obj_set_style_arc_image_src,
        set_style_bitmap_mask_src => lv_obj_set_style_bitmap_mask_src,
    );

    fn set_style_text_font(&self, value: LvFont, selector: LvStyleSelector) {
        unsafe { ffi::lv_obj_set_style_text_font(self.as_ptr(), value.as_ptr(), selector) }
    }

    fn remove_style_text_font(&self, selector: LvStyleSelector) -> bool {
        unsafe {
            ffi::lv_obj_remove_local_style_prop(
                self.as_ptr(),
                ffi::_lv_style_id_t::LV_STYLE_TEXT_FONT as ffi::lv_style_prop_t,
                selector,
            )
        }
    }

    fn set_style_grid_column_dsc_array(
        &self,
        value: Option<&'static [i32]>,
        selector: LvStyleSelector,
    ) {
        if let Some(value) = value
            && let Some(last) = value.last()
        {
            if *last != ffi::LV_GRID_TEMPLATE_LAST as i32 {
                panic!("invalid input");
            }
            unsafe {
                ffi::lv_obj_set_style_grid_column_dsc_array(self.as_ptr(), value.as_ptr(), selector)
            }
        }
    }

    fn set_style_grid_row_dsc_array(
        &self,
        value: Option<&'static [i32]>,
        selector: LvStyleSelector,
    ) {
        if let Some(value) = value
            && let Some(last) = value.last()
        {
            if *last != ffi::LV_GRID_TEMPLATE_LAST as i32 {
                panic!("invalid input");
            }
            unsafe {
                ffi::lv_obj_set_style_grid_row_dsc_array(self.as_ptr(), value.as_ptr(), selector)
            }
        }
    }
}

impl<C: class::LvClass> ObjExt for LvHandle<C> {
    fn as_ptr(&self) -> *mut ffi::lv_obj_t {
        LvHandle::as_ptr(self)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use alloc::rc::Rc;
    use core::cell::Cell;
    use core::ptr;

    use super::*;

    #[test]
    fn test_style_methods_exist() {
        let _: fn(&LvObj, LvColor, LvStyleSelector) = <LvObj as ObjExt>::set_style_text_color;
        let _: fn(&LvObj, LvFont, LvStyleSelector) = <LvObj as ObjExt>::set_style_text_font;
        let _: fn(&LvObj, LvStyleSelector) -> bool = <LvObj as ObjExt>::remove_style_text_font;
        let _: unsafe fn(&LvObj, Option<&'static u8>, LvStyleSelector) =
            <LvObj as ObjExt>::set_style_bg_image_src::<u8>;
        let _: fn(&LvObj, crate::LvEventCode, fn()) -> Result<(), crate::LvEventRegistrationError> =
            <LvObj as ObjExt>::add_event_cb::<fn()>;
        let _: fn(&LvObj, fn()) -> Result<(), crate::LvEventRegistrationError> =
            <LvObj as ObjExt>::add_click_cb::<fn()>;
    }

    #[test]
    fn test_add_event_cb_invokes_callback() {
        let _lock = crate::test_util::lock_and_init();

        let display = crate::LvDisplay::new(16, 16).unwrap();
        let screen = display.screen_active().unwrap();
        let obj = LvObj::with_parent(&screen).unwrap();
        let called = Rc::new(Cell::new(0));
        let called_cb = Rc::clone(&called);
        obj.add_event_cb(crate::LvEventCode::LV_EVENT_CLICKED, move || {
            called_cb.set(called_cb.get() + 1);
        })
        .unwrap();

        let result = unsafe {
            ffi::lv_obj_send_event(
                obj.as_ptr(),
                crate::LvEventCode::LV_EVENT_CLICKED,
                ptr::null_mut(),
            )
        };
        assert_eq!(result, ffi::lv_result_t::LV_RESULT_OK);
        assert_eq!(called.get(), 1);

        unsafe { obj.delete() };
    }

    #[test]
    fn test_add_event_cb_delete_event_invokes_callback() {
        let _lock = crate::test_util::lock_and_init();

        let display = crate::LvDisplay::new(16, 16).unwrap();
        let screen = display.screen_active().unwrap();
        let obj = LvObj::with_parent(&screen).unwrap();
        let called = Rc::new(Cell::new(false));
        let called_cb = Rc::clone(&called);
        obj.add_event_cb(crate::LvEventCode::LV_EVENT_DELETE, move || {
            called_cb.set(true);
        })
        .unwrap();

        unsafe { obj.delete() };

        assert!(called.get());
    }

    #[test]
    fn test_add_event_cb_delete_during_callback_is_safe() {
        let _lock = crate::test_util::lock_and_init();

        let display = crate::LvDisplay::new(16, 16).unwrap();
        let screen = display.screen_active().unwrap();
        let obj = LvObj::with_parent(&screen).unwrap();
        let obj_ptr = obj.as_ptr();
        let called = Rc::new(Cell::new(false));
        let called_cb = Rc::clone(&called);
        obj.add_event_cb(crate::LvEventCode::LV_EVENT_CLICKED, move || {
            called_cb.set(true);
            unsafe { ffi::lv_obj_delete(obj_ptr) };
        })
        .unwrap();

        let result = unsafe {
            ffi::lv_obj_send_event(
                obj.as_ptr(),
                crate::LvEventCode::LV_EVENT_CLICKED,
                ptr::null_mut(),
            )
        };
        assert_eq!(result, ffi::lv_result_t::LV_RESULT_INVALID);
        assert!(called.get());
    }
}
