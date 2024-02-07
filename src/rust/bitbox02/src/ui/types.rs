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
use alloc::vec::Vec;

use util::Survive;

pub use bitbox02_sys::trinary_choice_t as TrinaryChoice;

// Taking the constant straight from C, as it's excluding the null terminator.
#[cfg_attr(any(feature = "testing", feature = "c-unit-testing"), allow(dead_code))]
pub(crate) const MAX_LABEL_SIZE: usize = bitbox02_sys::MAX_LABEL_SIZE as _;

#[derive(Default)]
pub enum Font {
    #[default]
    Default,
    Password11X12,
    Monogram5X9,
}

impl Font {
    #[cfg_attr(any(feature = "testing", feature = "c-unit-testing"), allow(dead_code))]
    pub(crate) fn as_ptr(&self) -> *const bitbox02_sys::UG_FONT {
        match self {
            Font::Default => core::ptr::null() as *const _,
            Font::Password11X12 => unsafe { &bitbox02_sys::font_password_11X12 },
            Font::Monogram5X9 => unsafe { &bitbox02_sys::font_monogram_5X9 },
        }
    }
}

#[derive(Default)]
pub struct ConfirmParams<'a> {
    /// The confirmation title of the screen. Max 200 chars, otherwise **panic**.
    pub title: &'a str,
    pub title_autowrap: bool,
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
    #[cfg_attr(any(feature = "testing", feature = "c-unit-testing"), allow(dead_code))]
    /// `title_scratch` and `body_scratch` exist to keep the data
    /// alive for as long as the C params live.
    pub(crate) fn to_c_params(
        &self,
        title_scatch: &'a mut Vec<u8>,
        body_scratch: &'a mut Vec<u8>,
    ) -> Survive<'a, bitbox02_sys::confirm_params_t> {
        // We truncate at a bit higher than MAX_LABEL_SIZE, so the label component will correctly
        // truncate and append '...'.
        const TRUNCATE_SIZE: usize = MAX_LABEL_SIZE + 1;
        *title_scatch =
            crate::util::str_to_cstr_vec(crate::util::truncate_str(self.title, TRUNCATE_SIZE))
                .unwrap();
        *body_scratch =
            crate::util::str_to_cstr_vec(crate::util::truncate_str(self.body, TRUNCATE_SIZE))
                .unwrap();
        Survive::new(bitbox02_sys::confirm_params_t {
            title: title_scatch.as_ptr(),
            title_autowrap: self.title_autowrap,
            body: body_scratch.as_ptr(),
            font: self.font.as_ptr(),
            scrollable: self.scrollable,
            longtouch: self.longtouch,
            accept_only: self.accept_only,
            accept_is_nextarrow: self.accept_is_nextarrow,
            display_size: self.display_size as _,
        })
    }
}

#[derive(Default)]
pub struct TrinaryInputStringParams<'a> {
    /// The confirmation title of the screen. Max 200 chars, otherwise **panic**.
    pub title: &'a str,
    /// Currently specialized to the BIP39 wordlist. Can be extended if needed.
    pub wordlist: Option<&'a crate::keystore::Bip39Wordlist>,
    pub number_input: bool,
    pub hide: bool,
    pub special_chars: bool,
    pub longtouch: bool,
    pub cancel_is_backbutton: bool,
}

impl<'a> TrinaryInputStringParams<'a> {
    #[cfg_attr(any(feature = "testing", feature = "c-unit-testing"), allow(dead_code))]
    pub(crate) fn to_c_params(
        &self,
        title_scratch: &'a mut Vec<u8>,
    ) -> Survive<'a, bitbox02_sys::trinary_input_string_params_t> {
        // We truncate at a bit higher than MAX_LABEL_SIZE, so the label component will correctly
        // truncate and append '...'.
        const TRUNCATE_SIZE: usize = MAX_LABEL_SIZE + 1;

        *title_scratch =
            crate::util::str_to_cstr_vec(crate::util::truncate_str(self.title, TRUNCATE_SIZE))
                .unwrap();

        Survive::new(bitbox02_sys::trinary_input_string_params_t {
            title: title_scratch.as_ptr(),
            wordlist: match self.wordlist {
                None => core::ptr::null(),
                Some(wordlist) => wordlist.as_ptr(),
            },
            wordlist_size: match self.wordlist {
                None => 0,
                Some(wordlist) => wordlist.len() as _,
            },
            number_input: self.number_input,
            hide: self.hide,
            special_chars: self.special_chars,
            longtouch: self.longtouch,
            cancel_is_backbutton: self.cancel_is_backbutton,
        })
    }
}

pub type SelectWordCb<'a> = Box<dyn FnMut(u8) + 'a>;
pub type ContinueCancelCb<'a> = Box<dyn FnMut() + 'a>;

pub struct MenuParams<'a> {
    pub words: &'a [&'a str],
    pub title: Option<&'a str>,
    pub select_word_cb: Option<SelectWordCb<'a>>,
    pub continue_on_last_cb: Option<ContinueCancelCb<'a>>,
    pub cancel_cb: Option<ContinueCancelCb<'a>>,
}

pub type TrinaryChoiceCb<'a> = Box<dyn FnMut(TrinaryChoice) + 'a>;

pub type AcceptRejectCb<'a> = Box<dyn FnMut(bool) + 'a>;
