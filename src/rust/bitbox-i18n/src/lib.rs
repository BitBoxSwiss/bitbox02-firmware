// SPDX-License-Identifier: Apache-2.0

#![no_std]

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
        .binary_search_by_key(&key, |(source_key, _)| *source_key)
        .ok()
        .map(|index| table[index].1)
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
        out.push_str(args.next().expect("not enough format arguments"));
        rest = &rest[start + len..];
    }
    assert!(args.next().is_none(), "too many format arguments");
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
    fn test_generated_table_sorted() {
        assert!(DE_TRANSLATIONS.windows(2).all(|pair| pair[0].0 < pair[1].0));
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
                "Fee is {}%\nof the send amount.\nProceed?",
                &["17.0"],
            ),
            "Gebühr: 17.0%\ndes Sendebetrags.\nFortfahren?"
        );
    }

    #[test]
    fn test_format_pattern() {
        assert_eq!(format_pattern("No arguments", &[]), "No arguments");
        assert_eq!(
            format_pattern("{} / {:?} / {}", &["first", "second", "third"]),
            "first / second / third"
        );
    }

    #[test]
    #[should_panic(expected = "not enough format arguments")]
    fn test_format_pattern_too_few_arguments() {
        format_pattern("Fee is {}%", &[]);
    }

    #[test]
    #[should_panic(expected = "too many format arguments")]
    fn test_format_pattern_too_many_arguments() {
        format_pattern("Fee is {}%", &["17.0", "unexpected"]);
    }
}
