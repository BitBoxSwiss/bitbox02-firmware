// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

/// Style property ids, for building a [`LvStyleTransition`] property list. Values match LVGL's
/// `lv_style_prop_t`.
pub mod prop {
    use crate::ffi::_lv_style_id_t;

    /// Terminator for a property list.
    pub const INV: u8 = _lv_style_id_t::LV_STYLE_PROP_INV as u8;
    pub const BG_COLOR: u8 = _lv_style_id_t::LV_STYLE_BG_COLOR as u8;
    pub const BG_OPA: u8 = _lv_style_id_t::LV_STYLE_BG_OPA as u8;
    pub const TEXT_COLOR: u8 = _lv_style_id_t::LV_STYLE_TEXT_COLOR as u8;
    pub const LINE_COLOR: u8 = _lv_style_id_t::LV_STYLE_LINE_COLOR as u8;
}

/// A style transition descriptor. When an object changes state, the listed `props` animate over
/// `time_ms` (linear easing).
///
/// LVGL stores the pointer to this descriptor (and to its property list) rather than copying them,
/// so both must live for `'static`. Declare it as a `static` and pass it to
/// [`crate::ObjExt::set_style_transition`].
pub struct LvStyleTransition(ffi::lv_style_transition_dsc_t);

// LVGL runs single-threaded; the descriptor is immutable after construction.
unsafe impl Sync for LvStyleTransition {}

impl LvStyleTransition {
    /// `props` must be a `'static` slice terminated by [`prop::INV`].
    pub const fn new(props: &'static [u8], time_ms: u32, delay_ms: u32) -> Self {
        if props.is_empty() || props[props.len() - 1] != prop::INV {
            panic!("style transition props must be terminated by prop::INV");
        }

        Self(ffi::lv_style_transition_dsc_t {
            props: props.as_ptr(),
            user_data: core::ptr::null_mut(),
            path_xcb: Some(ffi::lv_anim_path_linear),
            time: time_ms,
            delay: delay_ms,
        })
    }

    /// Borrows the underlying descriptor for [`crate::ObjExt::set_style_transition`]. When `self` is
    /// a `static`, the returned reference is `'static`.
    pub fn as_dsc(&self) -> &ffi::lv_style_transition_dsc_t {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PROPS: &[u8] = &[prop::BG_OPA, prop::BG_COLOR, prop::INV];
    const VALID_TRANSITION: LvStyleTransition = LvStyleTransition::new(VALID_PROPS, 1, 2);

    #[test]
    fn test_style_transition_new() {
        let dsc = VALID_TRANSITION.as_dsc();
        assert_eq!(dsc.props, VALID_PROPS.as_ptr());
        assert_eq!(dsc.time, 1);
        assert_eq!(dsc.delay, 2);
    }

    #[test]
    #[should_panic]
    fn test_style_transition_new_rejects_missing_terminator() {
        let _ = LvStyleTransition::new(&[prop::BG_OPA, prop::BG_COLOR], 1, 0);
    }

    #[test]
    #[should_panic]
    fn test_style_transition_new_rejects_empty_props() {
        let _ = LvStyleTransition::new(&[], 1, 0);
    }
}
