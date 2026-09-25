// SPDX-License-Identifier: Apache-2.0

use core::ffi::CStr;
use core::ptr::NonNull;

use super::label::LvLabel;
use crate::{LvHandle, LvObj, LvTextAlign, ObjExt, ZeroizingText, class, ffi};
use util::strings::optional_cstr_from_ptr;

pub type LvTextareaTextError = super::LvTextError;
pub type LvTextarea = LvHandle<class::TextareaTag>;

pub trait TextareaExt: ObjExt {
    fn add_char(&self, ch: u32) {
        unsafe { ffi::lv_textarea_add_char(self.as_ptr(), ch) }
    }

    fn add_text(&self, txt: &str) -> Result<(), LvTextareaTextError> {
        let txt = ZeroizingText::new(txt)?;
        unsafe { ffi::lv_textarea_add_text(self.as_ptr(), txt.as_ptr()) }
        Ok(())
    }

    fn delete_char(&self) {
        unsafe { ffi::lv_textarea_delete_char(self.as_ptr()) }
    }

    fn delete_char_forward(&self) {
        unsafe { ffi::lv_textarea_delete_char_forward(self.as_ptr()) }
    }

    fn set_text(&self, txt: &str) -> Result<(), LvTextareaTextError> {
        let txt = ZeroizingText::new(txt)?;
        unsafe { ffi::lv_textarea_set_text(self.as_ptr(), txt.as_ptr()) }
        Ok(())
    }

    fn set_placeholder_text(&self, txt: &str) -> Result<(), LvTextareaTextError> {
        let txt = ZeroizingText::new(txt)?;
        unsafe { ffi::lv_textarea_set_placeholder_text(self.as_ptr(), txt.as_ptr()) }
        Ok(())
    }

    fn set_cursor_pos(&self, pos: i32) {
        unsafe { ffi::lv_textarea_set_cursor_pos(self.as_ptr(), pos) }
    }

    fn set_cursor_click_pos(&self, enable: bool) {
        unsafe { ffi::lv_textarea_set_cursor_click_pos(self.as_ptr(), enable) }
    }

    fn set_password_mode(&self, enable: bool) {
        unsafe { ffi::lv_textarea_set_password_mode(self.as_ptr(), enable) }
    }

    fn set_password_bullet(&self, bullet: &str) -> Result<(), LvTextareaTextError> {
        let bullet = ZeroizingText::new(bullet)?;
        unsafe { ffi::lv_textarea_set_password_bullet(self.as_ptr(), bullet.as_ptr()) }
        Ok(())
    }

    fn set_one_line(&self, enable: bool) {
        unsafe { ffi::lv_textarea_set_one_line(self.as_ptr(), enable) }
    }

    fn set_accepted_chars(&self, accepted: Option<&'static CStr>) {
        unsafe {
            ffi::lv_textarea_set_accepted_chars(
                self.as_ptr(),
                accepted.map_or(core::ptr::null(), CStr::as_ptr),
            )
        }
    }

    fn set_max_length(&self, max_length: u32) {
        unsafe { ffi::lv_textarea_set_max_length(self.as_ptr(), max_length) }
    }

    fn set_insert_replace(&self, text: Option<&'static CStr>) {
        unsafe {
            ffi::lv_textarea_set_insert_replace(
                self.as_ptr(),
                text.map_or(core::ptr::null(), CStr::as_ptr),
            )
        }
    }

    fn set_text_selection(&self, enable: bool) {
        unsafe { ffi::lv_textarea_set_text_selection(self.as_ptr(), enable) }
    }

    fn set_password_show_time(&self, time_ms: u32) {
        unsafe { ffi::lv_textarea_set_password_show_time(self.as_ptr(), time_ms) }
    }

    fn set_align(&self, align: LvTextAlign) {
        unsafe { ffi::lv_textarea_set_align(self.as_ptr(), align) }
    }

    fn get_text(&self) -> Option<ZeroizingText> {
        unsafe {
            // Snapshot the current text instead of borrowing LVGL-owned storage.
            optional_cstr_from_ptr(ffi::lv_textarea_get_text(self.as_ptr()))
                .map(ZeroizingText::from_cstr)
        }
    }

    /// Reads UTF-8 text through a zeroizing snapshot, treating a missing buffer as empty.
    ///
    /// The callback can update the textarea: it never borrows LVGL-owned storage that an
    /// update could invalidate. Panics if the textarea contains invalid UTF-8.
    fn with_text<R>(&self, f: impl FnOnce(&str) -> R) -> R {
        let text = self.get_text();
        f(text.as_ref().map_or("", |text| {
            text.to_str().expect("textarea content must be valid UTF-8")
        }))
    }

    fn get_placeholder_text(&self) -> Option<ZeroizingText> {
        unsafe {
            // Snapshot the current placeholder instead of borrowing LVGL-owned storage.
            optional_cstr_from_ptr(ffi::lv_textarea_get_placeholder_text(self.as_ptr()))
                .map(ZeroizingText::from_cstr)
        }
    }

    fn get_label(&self) -> Option<LvLabel> {
        NonNull::new(unsafe { ffi::lv_textarea_get_label(self.as_ptr()) }).map(LvHandle::from_ptr)
    }

    fn get_cursor_pos(&self) -> u32 {
        unsafe { ffi::lv_textarea_get_cursor_pos(self.as_ptr()) }
    }

    fn get_cursor_click_pos(&self) -> bool {
        unsafe { ffi::lv_textarea_get_cursor_click_pos(self.as_ptr()) }
    }

    fn get_password_mode(&self) -> bool {
        unsafe { ffi::lv_textarea_get_password_mode(self.as_ptr()) }
    }

    fn get_password_bullet(&self) -> Option<ZeroizingText> {
        unsafe {
            // Snapshot the current bullet instead of borrowing LVGL-owned storage.
            optional_cstr_from_ptr(ffi::lv_textarea_get_password_bullet(self.as_ptr()))
                .map(ZeroizingText::from_cstr)
        }
    }

    fn get_one_line(&self) -> bool {
        unsafe { ffi::lv_textarea_get_one_line(self.as_ptr()) }
    }

    fn get_accepted_chars(&self) -> Option<ZeroizingText> {
        unsafe {
            // Snapshot the accepted-char list instead of borrowing LVGL-owned storage.
            optional_cstr_from_ptr(ffi::lv_textarea_get_accepted_chars(self.as_ptr()))
                .map(ZeroizingText::from_cstr)
        }
    }

    fn get_max_length(&self) -> u32 {
        unsafe { ffi::lv_textarea_get_max_length(self.as_ptr()) }
    }

    fn text_is_selected(&self) -> bool {
        unsafe { ffi::lv_textarea_text_is_selected(self.as_ptr()) }
    }

    fn get_text_selection(&self) -> bool {
        unsafe { ffi::lv_textarea_get_text_selection(self.as_ptr()) }
    }

    fn get_password_show_time(&self) -> u32 {
        unsafe { ffi::lv_textarea_get_password_show_time(self.as_ptr()) }
    }

    fn get_current_char(&self) -> u32 {
        unsafe { ffi::lv_textarea_get_current_char(self.as_ptr()) }
    }

    fn clear_selection(&self) {
        unsafe { ffi::lv_textarea_clear_selection(self.as_ptr()) }
    }

    fn cursor_right(&self) {
        unsafe { ffi::lv_textarea_cursor_right(self.as_ptr()) }
    }

    fn cursor_left(&self) {
        unsafe { ffi::lv_textarea_cursor_left(self.as_ptr()) }
    }

    fn cursor_down(&self) {
        unsafe { ffi::lv_textarea_cursor_down(self.as_ptr()) }
    }

    fn cursor_up(&self) {
        unsafe { ffi::lv_textarea_cursor_up(self.as_ptr()) }
    }
}

impl LvHandle<class::TextareaTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_textarea_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl TextareaExt for LvHandle<class::TextareaTag> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_text_allows_replacement() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let textarea = LvTextarea::new(&screen).unwrap();
        assert!(textarea.with_text(str::is_empty));
        textarea.set_text("öäü").unwrap();
        let len = textarea.with_text(|text| {
            textarea.set_text("replacement").unwrap();
            assert_eq!(text, "öäü");
            text.len()
        });
        assert_eq!(len, 6);
        textarea.with_text(|text| assert_eq!(text, "replacement"));
        unsafe { textarea.delete() };
    }
}
