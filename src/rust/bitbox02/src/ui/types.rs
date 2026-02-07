// SPDX-License-Identifier: Apache-2.0

extern crate alloc;
use alloc::boxed::Box;

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

pub type SelectWordCb<'a> = Box<dyn FnMut(u8) + 'a>;
pub type ContinueCancelCb<'a> = Box<dyn FnMut() + 'a>;

pub struct MenuParams<'a> {
    pub words: &'a [&'a str],
    pub title: Option<&'a str>,
    pub select_word: bool,
    pub continue_on_last: bool,
    pub cancel: bool,
}

pub type TrinaryChoiceCb<'a> = Box<dyn FnMut(TrinaryChoice) + 'a>;

pub type AcceptRejectCb<'a> = Box<dyn FnMut(bool) + 'a>;
