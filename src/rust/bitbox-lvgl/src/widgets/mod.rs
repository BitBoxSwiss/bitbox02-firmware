// SPDX-License-Identifier: Apache-2.0

pub mod arc;
pub mod bar;
pub mod button;
pub mod buttonmatrix;
pub mod canvas;
pub mod class;
pub mod image;
pub mod keyboard;
pub mod label;
pub mod obj;
pub mod slider;
pub mod span;
pub mod spinner;
pub mod textarea;
mod util;

pub use util::{LvMapError, LvTextError};

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::class;
    use super::*;

    fn dummy_handle<C: class::LvClass>() -> obj::LvHandle<C> {
        obj::LvHandle::from_ptr(NonNull::dangling())
    }

    fn assert_image_ext<T: image::ImageExt>() {}
    fn assert_arc_ext<T: arc::ArcExt>() {}
    fn assert_bar_ext<T: bar::BarExt>() {}
    fn assert_buttonmatrix_ext<T: buttonmatrix::ButtonmatrixExt>() {}
    fn assert_spangroup_ext<T: span::SpangroupExt>() {}

    #[test]
    fn test_derived_widgets_implement_base_extension_traits() {
        assert_image_ext::<canvas::LvCanvas>();
        assert_arc_ext::<spinner::LvSpinner>();
        assert_bar_ext::<slider::LvSlider>();
        assert_buttonmatrix_ext::<keyboard::LvKeyboard>();
        assert_spangroup_ext::<span::LvSpangroup>();
    }

    #[test]
    fn test_explicit_upcasts_preserve_pointer() {
        let canvas = dummy_handle::<class::CanvasTag>();
        assert_eq!(canvas.to_image().as_ptr(), canvas.as_ptr());
        assert_eq!(canvas.to_obj().as_ptr(), canvas.as_ptr());

        let spinner = dummy_handle::<class::SpinnerTag>();
        assert_eq!(spinner.to_arc().as_ptr(), spinner.as_ptr());
        assert_eq!(spinner.to_obj().as_ptr(), spinner.as_ptr());

        let slider = dummy_handle::<class::SliderTag>();
        assert_eq!(slider.to_bar().as_ptr(), slider.as_ptr());
        assert_eq!(slider.to_obj().as_ptr(), slider.as_ptr());

        let keyboard = dummy_handle::<class::KeyboardTag>();
        assert_eq!(keyboard.to_buttonmatrix().as_ptr(), keyboard.as_ptr());
        assert_eq!(keyboard.to_obj().as_ptr(), keyboard.as_ptr());

        let spangroup = dummy_handle::<class::SpangroupTag>();
        assert_eq!(spangroup.to_obj().as_ptr(), spangroup.as_ptr());
    }
}
