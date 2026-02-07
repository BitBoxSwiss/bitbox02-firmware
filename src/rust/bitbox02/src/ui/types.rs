// SPDX-License-Identifier: Apache-2.0

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

#[derive(Default)]
pub struct TrinaryInputStringParams<'a> {
    /// The confirmation title of the screen. Max 200 chars, otherwise **panic**.
    pub title: &'a str,
    /// Currently specialized to the BIP39 wordlist: a list of BIP39 word indices. Can be extended if needed.
    pub wordlist: Option<&'a [u16]>,
    pub number_input: bool,
    pub hide: bool,
    pub special_chars: bool,
    pub longtouch: bool,
    pub cancel_is_backbutton: bool,
    pub default_to_digits: bool,
}

impl<'a> TrinaryInputStringParams<'a> {
    #[cfg_attr(any(feature = "testing", feature = "c-unit-testing"), allow(dead_code))]
    pub(crate) fn to_c_params(
        &self,
        title_scratch: &'a mut Vec<core::ffi::c_char>,
    ) -> Survive<'a, bitbox02_sys::trinary_input_string_params_t> {
        // We truncate at a bit higher than MAX_LABEL_SIZE, so the label component will correctly
        // truncate and append '...'.
        const TRUNCATE_SIZE: usize = MAX_LABEL_SIZE + 1;

        *title_scratch =
            util::strings::str_to_cstr_vec(util::strings::truncate_str(self.title, TRUNCATE_SIZE))
                .unwrap();

        Survive::new(bitbox02_sys::trinary_input_string_params_t {
            title: title_scratch.as_ptr().cast(),
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
            default_to_digits: self.default_to_digits,
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
