// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

/// Marker trait for LVGL object classes.
pub trait LvClass {
    fn class_ptr() -> *const ffi::lv_obj_class_t;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanvasTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArcTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpinnerTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SliderTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonmatrixTag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardTag;

macro_rules! impl_lv_class {
    ($class:ty, $ffi_symbol:ident) => {
        impl LvClass for $class {
            fn class_ptr() -> *const ffi::lv_obj_class_t {
                core::ptr::addr_of!(ffi::$ffi_symbol)
            }
        }
    };
}

impl_lv_class!(ObjTag, lv_obj_class);
impl_lv_class!(LabelTag, lv_label_class);
impl_lv_class!(ImageTag, lv_image_class);
impl_lv_class!(CanvasTag, lv_canvas_class);
impl_lv_class!(ArcTag, lv_arc_class);
impl_lv_class!(SpinnerTag, lv_spinner_class);
impl_lv_class!(ButtonTag, lv_button_class);
impl_lv_class!(BarTag, lv_bar_class);
impl_lv_class!(SliderTag, lv_slider_class);
impl_lv_class!(TextareaTag, lv_textarea_class);
impl_lv_class!(ButtonmatrixTag, lv_buttonmatrix_class);
impl_lv_class!(KeyboardTag, lv_keyboard_class);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_pointers_non_null() {
        assert!(!ObjTag::class_ptr().is_null());
        assert!(!CanvasTag::class_ptr().is_null());
        assert!(!SpinnerTag::class_ptr().is_null());
        assert!(!KeyboardTag::class_ptr().is_null());
    }
}
