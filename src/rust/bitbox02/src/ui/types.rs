// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate alloc;
use alloc::boxed::Box;

// Taking the constant straight from C, as it's excluding the null terminator.
#[cfg_attr(feature = "testing", allow(dead_code))]
pub(crate) const MAX_LABEL_SIZE: usize = bitbox02_sys::MAX_LABEL_SIZE as _;

pub enum Font {
    Default,
    Password11X12,
    Monogram5X9,
}

impl Font {
    #[cfg_attr(feature = "testing", allow(dead_code))]
    pub(crate) fn as_ptr(&self) -> *const bitbox02_sys::UG_FONT {
        match self {
            Font::Default => core::ptr::null() as *const _,
            Font::Password11X12 => unsafe { &bitbox02_sys::font_password_11X12 },
            Font::Monogram5X9 => unsafe { &bitbox02_sys::font_monogram_5X9 },
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        Font::Default
    }
}

#[derive(Default)]
pub struct ConfirmParams<'a> {
    /// The confirmation title of the screen. Max 200 chars, otherwise **panic**.
    pub title: &'a str,
    /// The confirmation body of the screen. Max 200 chars, otherwise **panic**.
    pub body: &'a str,
    pub font: Font,
    /// If true, the body is horizontally scrollable.
    pub scrollable: bool,
    /// If true, require the hold gesture to confirm instead of tap.
    pub longtouch: bool,
    /// If true, the user can only confirm, not reject.
    pub accept_only: bool,
    /// if true, the accept icon is a right arrow instead of a checkmark (indicating going to the
    /// "next" screen).
    pub accept_is_nextarrow: bool,
    /// Print the value of this variable in the corner. Will not print when 0
    pub display_size: usize,
}

impl<'a> ConfirmParams<'a> {
    #[cfg_attr(feature = "testing", allow(dead_code))]
    pub(crate) fn to_c_params(&self) -> bitbox02_sys::confirm_params_t {
        // We truncate at a bit higher than MAX_LABEL_SIZE, so the label component will correctly
        // truncate and append '...'.
        const TRUNCATE_SIZE: usize = MAX_LABEL_SIZE + 1;

        bitbox02_sys::confirm_params_t {
            title: crate::str_to_cstr_force!(
                crate::util::truncate_str(self.title, TRUNCATE_SIZE),
                TRUNCATE_SIZE
            )
            .as_ptr(),
            body: crate::str_to_cstr_force!(
                crate::util::truncate_str(self.body, TRUNCATE_SIZE),
                TRUNCATE_SIZE
            )
            .as_ptr(),
            font: self.font.as_ptr(),
            scrollable: self.scrollable,
            longtouch: self.longtouch,
            accept_only: self.accept_only,
            accept_is_nextarrow: self.accept_is_nextarrow,
            display_size: self.display_size as _,
        }
    }
}

pub struct MenuParams<'a> {
    pub words: &'a [&'a str],
    pub title: Option<&'a str>,
    pub select_word_cb: Option<Box<dyn FnMut(u8) + 'a>>,
    pub continue_on_last_cb: Option<Box<dyn FnMut() + 'a>>,
    pub cancel_cb: Option<Box<dyn FnMut() + 'a>>,
}
