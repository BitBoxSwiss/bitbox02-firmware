// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;
use bitbox_hal::memory::Language;

include!(concat!(env!("OUT_DIR"), "/i18n.rs"));

const FNV1A64_OFFSET: u64 = 0xcbf29ce484222325;
const FNV1A64_PRIME: u64 = 0x100000001b3;

pub fn language_from_code(code: &str) -> Option<Language> {
    let code = code.as_bytes();
    if code.len() < 2 {
        return None;
    }
    if code.len() > 2 && code[2] != b'-' && code[2] != b'_' {
        return None;
    }

    match (code[0].to_ascii_lowercase(), code[1].to_ascii_lowercase()) {
        (b'e', b'n') => Some(Language::English),
        (b'd', b'e') => Some(Language::German),
        _ => None,
    }
}

pub fn language_code(language: Language) -> &'static str {
    match language {
        Language::English => "en",
        Language::German => "de",
    }
}

pub fn translate<'a>(language: Language, english: &'a str) -> Cow<'a, str> {
    match language {
        Language::English => Cow::Borrowed(english),
        Language::German => translate_from_table(english, DE_TRANSLATIONS)
            .map(Cow::Borrowed)
            .unwrap_or(Cow::Borrowed(english)),
    }
}

pub fn format(language: Language, english: &'static str, args: &[&str]) -> String {
    let translated = translate(language, english);
    format_pattern(&translated, args)
}

pub const fn is_translatable(english: &str) -> bool {
    let key = translation_key(english);
    let mut lo = 0;
    let mut hi = VALID_EN_HASHES.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let value = VALID_EN_HASHES[mid];
        if value == key {
            return true;
        } else if value < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    false
}

fn translate_from_table<'a>(english: &str, table: &'a [(u64, &str)]) -> Option<&'a str> {
    let key = translation_key(english);
    table
        .iter()
        .find_map(|(source_key, translated)| (*source_key == key).then_some(*translated))
}

const fn translation_key(value: &str) -> u64 {
    let bytes = value.as_bytes();
    let mut hash = FNV1A64_OFFSET;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(FNV1A64_PRIME);
        i += 1;
    }
    hash
}

fn format_pattern(pattern: &str, args: &[&str]) -> String {
    let mut out =
        String::with_capacity(pattern.len() + args.iter().map(|arg| arg.len()).sum::<usize>());
    let mut rest = pattern;
    let mut args = args.iter();

    while let Some((start, len)) = next_placeholder(rest) {
        out.push_str(&rest[..start]);
        match args.next() {
            Some(arg) => out.push_str(arg),
            None => out.push_str(&rest[start..start + len]),
        }
        rest = &rest[start + len..];
    }
    out.push_str(rest);
    out
}

fn next_placeholder(value: &str) -> Option<(usize, usize)> {
    match (value.find("{}"), value.find("{:?}")) {
        (Some(a), Some(b)) if a <= b => Some((a, 2)),
        (Some(_), Some(b)) => Some((b, 4)),
        (Some(a), None) => Some((a, 2)),
        (None, Some(b)) => Some((b, 4)),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_code() {
        assert_eq!(language_from_code("en"), Some(Language::English));
        assert_eq!(language_from_code("EN-us"), Some(Language::English));
        assert_eq!(language_from_code("de"), Some(Language::German));
        assert_eq!(language_from_code("de_CH"), Some(Language::German));
        assert_eq!(language_from_code("dex"), None);
        assert_eq!(language_from_code("it"), None);
    }

    #[test]
    fn test_translate() {
        assert_eq!(
            translate(Language::English, "Proceed to upgrade?").as_ref(),
            "Proceed to upgrade?"
        );
        assert_eq!(
            translate(Language::German, "Proceed to upgrade?").as_ref(),
            "Upgrade fortsetzen?"
        );
        assert_eq!(
            translate(Language::German, "Not translated").as_ref(),
            "Not translated"
        );
    }

    #[test]
    fn test_is_translatable() {
        assert!(is_translatable("Proceed to upgrade?"));
        assert!(!is_translatable("Proceed to upgrade"));
        assert!(!is_translatable("definitely not a source string"));
    }

    #[test]
    fn test_format() {
        assert_eq!(
            format(
                Language::German,
                "The fee is {}%\nthe send amount.\nProceed?",
                &["17.0"],
            ),
            "Die Gebühr beträgt 17.0%\ndes Sendebetrags.\nFortfahren?"
        );
    }
}
