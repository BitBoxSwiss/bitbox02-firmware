// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;
use bitbox_hal::memory::Language;

use crate::hal::{Hal, Memory};

pub use bitbox_i18n::{
    SUPPORTED_LANGUAGES, checked_message, format, language_code, language_from_code,
    placeholder_count, translate,
};

/// Verifying at compile time that the message exists in the English source file.
/// Prevents messages from drifting from the locale file and silently falling back to English.
#[macro_export]
macro_rules! tr {
    ($hal:expr, $msg:expr $(,)?) => {{
        const _: &str = $crate::i18n::checked_message($msg);
        #[allow(clippy::disallowed_methods)]
        {
            $hal.tr(const { $msg })
        }
    }};
}

#[macro_export]
macro_rules! tr_format {
    ($hal:expr, $msg:expr, $args:expr $(,)?) => {{
        const _: &str = $crate::i18n::checked_message($msg);
        let args: &[&str; $crate::i18n::placeholder_count($msg)] = $args;
        #[allow(clippy::disallowed_methods)]
        {
            $hal.tr_format(const { $msg }, args)
        }
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
    #[allow(clippy::disallowed_methods)]
    fn tr<'a>(&mut self, english: &'a str) -> Cow<'a, str> {
        let language = self.memory().get_device_language();
        translate(language, english)
    }

    #[allow(clippy::disallowed_methods)]
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
