// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;
use bitbox_hal::memory::Language;

use crate::hal::{Hal, Memory};

pub use bitbox_core_utils::i18n::{
    format, is_translatable, language_code, language_from_code, translate,
};

/// Verifying at compile time that the literal exists in the English source file.
/// Prevents literals to drift from the locale file i.e. translations silently falling back to English.
#[macro_export]
macro_rules! tr {
    ($hal:expr, $msg:literal $(,)?) => {{
        const _: () = ::core::assert!(
            $crate::i18n::is_translatable($msg),
            "i18n: string literal is not a value in src/i18n/locales/en/firmware.json",
        );
        $hal.tr($msg)
    }};
}

#[macro_export]
macro_rules! tr_format {
    ($hal:expr, $msg:literal, $args:expr $(,)?) => {{
        const _: () = ::core::assert!(
            $crate::i18n::is_translatable($msg),
            "i18n: format pattern is not a value in src/i18n/locales/en/firmware.json",
        );
        $hal.tr_format($msg, $args)
    }};
}

pub trait I18n {
    fn tr<'a>(&mut self, english: &'a str) -> Cow<'a, str>;
    fn tr_format(&mut self, english: &'static str, args: &[&str]) -> String;
    fn tr_datetime(
        &mut self,
        timestamp: u32,
        timezone_offset: i32,
        date_only: bool,
    ) -> Result<String, ()>;
}

impl<T: Hal> I18n for T {
    fn tr<'a>(&mut self, english: &'a str) -> Cow<'a, str> {
        let language = self.memory().get_device_language();
        translate(language, english)
    }

    fn tr_format(&mut self, english: &'static str, args: &[&str]) -> String {
        let language = self.memory().get_device_language();
        format(language, english, args)
    }

    fn tr_datetime(
        &mut self,
        timestamp: u32,
        timezone_offset: i32,
        date_only: bool,
    ) -> Result<String, ()> {
        let locale = match self.memory().get_device_language() {
            Language::English => util::datetime::DateLocale::English,
            Language::German => util::datetime::DateLocale::German,
        };
        util::datetime::format_datetime_locale(timestamp, timezone_offset, date_only, locale)
    }
}
