// SPDX-License-Identifier: Apache-2.0

use core::ffi::CStr;
use core::ptr::NonNull;

use super::util;
use crate::{LvHandle, LvLabelLongMode, LvObj, LvPoint, ObjExt, class, ffi};

pub type LvLabelTextError = super::LvTextError;
pub type LvLabel = LvHandle<class::LabelTag>;

pub trait LabelExt: ObjExt {
    fn set_text(&self, txt: &str) -> Result<(), LvLabelTextError> {
        let txt = util::cstring(txt)?;
        unsafe { ffi::lv_label_set_text(self.as_ptr(), txt.as_ptr()) }
        Ok(())
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

    fn get_text(&self) -> Option<&CStr> {
        util::optional_cstr_from_ptr(unsafe { ffi::lv_label_get_text(self.as_ptr()) })
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
        let txt = util::cstring(txt)?;
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
