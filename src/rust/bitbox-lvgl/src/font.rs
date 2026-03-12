// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

#[derive(Clone, Copy, Debug)]
pub struct LvFont {
    raw: &'static ffi::lv_font_t,
}

impl LvFont {
    pub const fn new(raw: &'static ffi::lv_font_t) -> Self {
        Self { raw }
    }

    pub(crate) fn as_ptr(self) -> *const ffi::lv_font_t {
        self.raw as *const ffi::lv_font_t
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

    pub const INTER_REGULAR_32: LvFont = unsafe { LvFont::new(&ffi::inter_regular_32) };
    pub const INTER_REGULAR_48: LvFont = unsafe { LvFont::new(&ffi::inter_regular_48) };
    pub const INTER_BOLD_32: LvFont = unsafe { LvFont::new(&ffi::inter_bold_32) };
    pub const INTER_BOLD_48: LvFont = unsafe { LvFont::new(&ffi::inter_bold_48) };
}

#[cfg(test)]
mod tests {
    use super::{LvFont, fonts};

    #[test]
    fn test_fonts_are_copy() {
        let font = fonts::INTER_REGULAR_32;
        let copied = font;
        assert_eq!(font, copied);
    }

    #[test]
    fn test_font_ptr_roundtrip() {
        let font = fonts::INTER_BOLD_48;
        assert_eq!(
            font.as_ptr(),
            LvFont::new(unsafe { &crate::ffi::inter_bold_48 }).as_ptr()
        );
    }
}
