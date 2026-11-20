// SPDX-License-Identifier: Apache-2.0

use alloc::borrow::ToOwned;
use alloc::ffi::CString;
use alloc::vec::Vec;
use core::ffi::CStr;
use core::ptr::NonNull;

use crate::{LvHandle, LvLabelLongMode, LvObj, LvPoint, ObjExt, class, ffi};
use util::strings::{optional_cstr_from_ptr, str_to_cstr_vec_zeroizing};
use zeroize::Zeroizing;

pub type LvLabelTextError = super::LvTextError;
pub type LvLabel = LvHandle<class::LabelTag>;

pub trait LabelExt: ObjExt {
    fn set_text(&self, txt: &str) -> Result<(), LvLabelTextError> {
        let txt = str_to_cstr_vec_zeroizing(txt).map_err(|_| LvLabelTextError::ContainsNul)?;
        unsafe { ffi::lv_label_set_text(self.as_ptr(), txt.as_ptr()) }
        Ok(())
    }

    fn copy_text_from(&self, source: &LvLabel) {
        // The setter can dispatch callbacks, so stop borrowing LVGL storage before calling it.
        let txt: Zeroizing<Vec<u8>> = unsafe {
            let text =
                optional_cstr_from_ptr(ffi::lv_label_get_text(source.as_ptr())).unwrap_or(c"");
            Zeroizing::new(text.to_bytes_with_nul().to_vec())
        };
        unsafe { ffi::lv_label_set_text(self.as_ptr(), txt.as_ptr().cast()) }
    }

    fn set_text_static(&self, text: Option<&'static CStr>) {
        unsafe {
            ffi::lv_label_set_text_static(
                self.as_ptr(),
                text.map_or(core::ptr::null(), CStr::as_ptr),
            )
        }
    }

    fn set_long_mode(&self, long_mode: LvLabelLongMode) {
        unsafe { ffi::lv_label_set_long_mode(self.as_ptr(), long_mode) }
    }

    fn set_text_selection_start(&self, index: u32) {
        unsafe { ffi::lv_label_set_text_selection_start(self.as_ptr(), index) }
    }

    fn set_text_selection_end(&self, index: u32) {
        unsafe { ffi::lv_label_set_text_selection_end(self.as_ptr(), index) }
    }

    fn set_recolor(&self, enable: bool) {
        unsafe { ffi::lv_label_set_recolor(self.as_ptr(), enable) }
    }

    fn get_text(&self) -> Option<CString> {
        unsafe {
            // Snapshot the current text instead of borrowing LVGL-owned storage.
            optional_cstr_from_ptr(ffi::lv_label_get_text(self.as_ptr()))
                .map(|text| text.to_owned())
        }
    }

    fn get_long_mode(&self) -> LvLabelLongMode {
        unsafe { ffi::lv_label_get_long_mode(self.as_ptr()) }
    }

    fn get_letter_pos(&self, char_id: u32) -> LvPoint {
        let mut pos = LvPoint { x: 0, y: 0 };
        unsafe { ffi::lv_label_get_letter_pos(self.as_ptr(), char_id, &mut pos) }
        pos
    }

    fn get_letter_on(&self, pos: &LvPoint, bidi: bool) -> u32 {
        let mut pos = LvPoint { x: pos.x, y: pos.y };
        unsafe { ffi::lv_label_get_letter_on(self.as_ptr(), &mut pos, bidi) }
    }

    fn is_char_under_pos(&self, pos: &LvPoint) -> bool {
        let mut pos = LvPoint { x: pos.x, y: pos.y };
        unsafe { ffi::lv_label_is_char_under_pos(self.as_ptr(), &mut pos) }
    }

    fn get_text_selection_start(&self) -> u32 {
        unsafe { ffi::lv_label_get_text_selection_start(self.as_ptr()) }
    }

    fn get_text_selection_end(&self) -> u32 {
        unsafe { ffi::lv_label_get_text_selection_end(self.as_ptr()) }
    }

    fn get_recolor(&self) -> bool {
        unsafe { ffi::lv_label_get_recolor(self.as_ptr()) }
    }

    fn ins_text(&self, pos: u32, txt: &str) -> Result<(), LvLabelTextError> {
        let txt = CString::new(txt).map_err(|_| LvLabelTextError::ContainsNul)?;
        unsafe { ffi::lv_label_ins_text(self.as_ptr(), pos, txt.as_ptr()) }
        Ok(())
    }

    fn cut_text(&self, pos: u32, count: u32) {
        unsafe { ffi::lv_label_cut_text(self.as_ptr(), pos, count) }
    }
}

impl LvHandle<class::LabelTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_label_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl LabelExt for LvHandle<class::LabelTag> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_text_from_independent_storage() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(16, 16).unwrap();
        let screen = LvObj::new().unwrap();
        let source = LvLabel::new(&screen).unwrap();
        let destination = LvLabel::new(&screen).unwrap();

        source.set_text("abandon").unwrap();
        destination.set_text("old").unwrap();
        destination.copy_text_from(&source);
        source.set_text("zoo").unwrap();
        assert_eq!(destination.get_text().unwrap().as_bytes(), b"abandon");

        unsafe { source.delete() };
        assert_eq!(destination.get_text().unwrap().as_bytes(), b"abandon");

        unsafe {
            screen.delete();
            ffi::lv_display_delete(display.as_ptr());
        }
    }

    #[test]
    fn test_copy_text_from_self() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(16, 16).unwrap();
        let screen = LvObj::new().unwrap();
        let label = LvLabel::new(&screen).unwrap();

        for text in ["abandon", "zoo", ""] {
            label.set_text(text).unwrap();
            label.copy_text_from(&label);
            assert_eq!(label.get_text().unwrap().as_bytes(), text.as_bytes());
        }

        unsafe {
            screen.delete();
            ffi::lv_display_delete(display.as_ptr());
        }
    }
}
