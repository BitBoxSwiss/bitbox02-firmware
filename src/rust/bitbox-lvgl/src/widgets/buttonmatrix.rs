// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::ffi::{CStr, c_char};
use core::ptr::NonNull;

use crate::{LvButtonmatrixCtrl, LvHandle, LvObj, ObjExt, class, ffi};
use util::strings::{cstr_array_from_ptr, optional_cstr_from_ptr};

pub type LvButtonmatrix = LvHandle<class::ButtonmatrixTag>;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LvButtonmatrixMapEntry(*const c_char);

impl LvButtonmatrixMapEntry {
    pub const fn new(value: &'static CStr) -> Self {
        Self(value.as_ptr())
    }

    fn as_cstr(self) -> &'static CStr {
        unsafe { CStr::from_ptr(self.0) }
    }

    fn is_sentinel(self) -> bool {
        self.as_cstr().to_bytes().is_empty()
    }

    fn is_newline(self) -> bool {
        self.as_cstr().to_bytes() == b"\n"
    }
}

pub(crate) fn validate_map(map: &[LvButtonmatrixMapEntry]) -> usize {
    let Some((last, entries)) = map.split_last() else {
        panic!("LVGL button map must be terminated by an empty string sentinel");
    };

    assert!(
        last.is_sentinel(),
        "LVGL button map must be terminated by an empty string sentinel"
    );
    assert!(
        entries.iter().all(|entry| !entry.is_sentinel()),
        "LVGL button map may only contain an empty string sentinel as the last entry"
    );

    entries.iter().filter(|entry| !entry.is_newline()).count()
}

fn button_count_from_cstr_map(map: &[&CStr]) -> usize {
    map.iter().filter(|entry| entry.to_bytes() != b"\n").count()
}

fn assert_ctrl_map_len(expected: usize, actual: usize) {
    assert_eq!(
        actual, expected,
        "LVGL buttonmatrix control map length must match the number of buttons in the map"
    );
}

pub trait ButtonmatrixExt: ObjExt {
    /// `map` must point to a `'static` LVGL button-matrix map terminated by an empty string.
    fn set_map(&self, map: &'static [LvButtonmatrixMapEntry]) {
        validate_map(map);
        unsafe { ffi::lv_buttonmatrix_set_map(self.as_ptr(), map.as_ptr().cast()) }
    }

    fn set_ctrl_map(&self, ctrl_map: &[LvButtonmatrixCtrl]) {
        assert_ctrl_map_len(button_count_from_cstr_map(&self.get_map()), ctrl_map.len());
        unsafe { ffi::lv_buttonmatrix_set_ctrl_map(self.as_ptr(), ctrl_map.as_ptr()) }
    }

    fn set_selected_button(&self, button_id: u32) {
        unsafe { ffi::lv_buttonmatrix_set_selected_button(self.as_ptr(), button_id) }
    }

    fn set_button_ctrl(&self, button_id: u32, ctrl: LvButtonmatrixCtrl) {
        unsafe { ffi::lv_buttonmatrix_set_button_ctrl(self.as_ptr(), button_id, ctrl) }
    }

    fn clear_button_ctrl(&self, button_id: u32, ctrl: LvButtonmatrixCtrl) {
        unsafe { ffi::lv_buttonmatrix_clear_button_ctrl(self.as_ptr(), button_id, ctrl) }
    }

    fn set_button_ctrl_all(&self, ctrl: LvButtonmatrixCtrl) {
        unsafe { ffi::lv_buttonmatrix_set_button_ctrl_all(self.as_ptr(), ctrl) }
    }

    fn clear_button_ctrl_all(&self, ctrl: LvButtonmatrixCtrl) {
        unsafe { ffi::lv_buttonmatrix_clear_button_ctrl_all(self.as_ptr(), ctrl) }
    }

    fn set_button_width(&self, button_id: u32, width: u32) {
        unsafe { ffi::lv_buttonmatrix_set_button_width(self.as_ptr(), button_id, width) }
    }

    fn set_one_checked(&self, enable: bool) {
        unsafe { ffi::lv_buttonmatrix_set_one_checked(self.as_ptr(), enable) }
    }

    fn get_map(&self) -> Vec<&CStr> {
        unsafe {
            // LVGL returns either null or a valid button-matrix map array terminated by a null
            // pointer or empty string sentinel, with entries alive for the duration of this borrow.
            cstr_array_from_ptr(ffi::lv_buttonmatrix_get_map(self.as_ptr()))
        }
    }

    fn get_selected_button(&self) -> u32 {
        unsafe { ffi::lv_buttonmatrix_get_selected_button(self.as_ptr()) }
    }

    fn get_button_text(&self, button_id: u32) -> Option<&CStr> {
        unsafe {
            // LVGL returns either null or a valid NUL-terminated button text pointer owned by
            // the object and alive for the duration of this borrow.
            optional_cstr_from_ptr(ffi::lv_buttonmatrix_get_button_text(
                self.as_ptr(),
                button_id,
            ))
        }
    }

    fn has_button_ctrl(&self, button_id: u32, ctrl: LvButtonmatrixCtrl) -> bool {
        unsafe { ffi::lv_buttonmatrix_has_button_ctrl(self.as_ptr(), button_id, ctrl) }
    }

    fn get_one_checked(&self) -> bool {
        unsafe { ffi::lv_buttonmatrix_get_one_checked(self.as_ptr()) }
    }
}

impl LvHandle<class::ButtonmatrixTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_buttonmatrix_create(parent.as_ptr()) })
            .map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl ButtonmatrixExt for LvHandle<class::ButtonmatrixTag> {}
impl ButtonmatrixExt for LvHandle<class::KeyboardTag> {}

#[cfg(test)]
mod tests {
    use super::{
        LvButtonmatrixMapEntry, assert_ctrl_map_len, button_count_from_cstr_map, validate_map,
    };

    #[test]
    fn test_validate_map_counts_buttons() {
        let map = &[
            LvButtonmatrixMapEntry::new(c"left"),
            LvButtonmatrixMapEntry::new(c"right"),
            LvButtonmatrixMapEntry::new(c"\n"),
            LvButtonmatrixMapEntry::new(c"ok"),
            LvButtonmatrixMapEntry::new(c""),
        ];

        assert_eq!(validate_map(map), 3);
    }

    #[test]
    #[should_panic(expected = "LVGL button map must be terminated by an empty string sentinel")]
    fn test_validate_map_requires_sentinel() {
        let map = &[LvButtonmatrixMapEntry::new(c"only")];

        let _ = validate_map(map);
    }

    #[test]
    #[should_panic(
        expected = "LVGL button map may only contain an empty string sentinel as the last entry"
    )]
    fn test_validate_map_rejects_internal_sentinel() {
        let map = &[
            LvButtonmatrixMapEntry::new(c"left"),
            LvButtonmatrixMapEntry::new(c""),
            LvButtonmatrixMapEntry::new(c"right"),
            LvButtonmatrixMapEntry::new(c""),
        ];

        let _ = validate_map(map);
    }

    #[test]
    fn test_button_count_from_cstr_map_counts_buttons() {
        let map = &[c"left", c"right", c"\n", c"ok"];

        assert_eq!(button_count_from_cstr_map(map), 3);
    }

    #[test]
    #[should_panic(
        expected = "LVGL buttonmatrix control map length must match the number of buttons in the map"
    )]
    fn test_set_ctrl_map_rejects_incorrect_length() {
        assert_ctrl_map_len(2, 1);
    }
}
