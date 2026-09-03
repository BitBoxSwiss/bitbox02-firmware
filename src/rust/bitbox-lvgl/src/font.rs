// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

#[derive(Clone, Copy, Debug)]
pub struct LvFont {
    raw: &'static ffi::lv_font_t,
}

impl LvFont {
    /// # Safety
    /// `raw` must point to a valid LVGL font descriptor and all pointers reachable from it must
    /// remain valid for the program lifetime.
    pub const unsafe fn new(raw: &'static ffi::lv_font_t) -> Self {
        Self { raw }
    }

    pub fn as_ptr(self) -> *const ffi::lv_font_t {
        self.raw as *const ffi::lv_font_t
    }

    /// The maximum line height required by the font, in pixels.
    pub fn line_height(self) -> i32 {
        self.raw.line_height
    }

    /// The baseline position, measured up from the bottom of the line box.
    pub fn base_line(self) -> i32 {
        self.raw.base_line
    }
}

impl PartialEq for LvFont {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.raw, other.raw)
    }
}

impl Eq for LvFont {}

pub mod fonts {
    use super::LvFont;
    use crate::ffi;

    pub const INTER_REGULAR_24: LvFont = unsafe { LvFont::new(&ffi::inter_regular_24) };
    pub const INTER_REGULAR_32: LvFont = unsafe { LvFont::new(&ffi::inter_regular_32) };
    pub const INTER_REGULAR_48: LvFont = unsafe { LvFont::new(&ffi::inter_regular_48) };
    pub const INTER_MEDIUM_20: LvFont = unsafe { LvFont::new(&ffi::inter_medium_20) };
    pub const INTER_MEDIUM_32: LvFont = unsafe { LvFont::new(&ffi::inter_medium_32) };
    pub const INTER_BOLD_32: LvFont = unsafe { LvFont::new(&ffi::inter_bold_32) };
    pub const INTER_BOLD_48: LvFont = unsafe { LvFont::new(&ffi::inter_bold_48) };
}

#[cfg(test)]
mod tests {
    use super::{LvFont, fonts};

    #[test]
    fn test_font_ptr_roundtrip() {
        let font = fonts::INTER_BOLD_48;
        assert_eq!(
            font.as_ptr(),
            unsafe { LvFont::new(&crate::ffi::inter_bold_48) }.as_ptr()
        );
    }
}
