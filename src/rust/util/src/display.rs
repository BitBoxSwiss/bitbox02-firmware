// SPDX-License-Identifier: Apache-2.0

/// Returns whether `character` is safe to show as text with the project's extended Latin fonts.
///
/// Non-breaking space and soft hyphen are intentionally excluded because they render
/// indistinguishably from ordinary ASCII characters in the BitBox02 fonts.
pub fn is_safe_char(character: char) -> bool {
    matches!(
        character,
        '\u{20}'..='\u{7e}' | '\u{a1}'..='\u{ac}' | '\u{ae}'..='\u{17f}'
    )
}

/// Returns whether every character in `text` is safe to display. Newlines can optionally be
/// accepted as workflow-specific layout markers.
pub fn is_safe_text(text: &str, allow_newline: bool) -> bool {
    text.chars()
        .all(|character| (allow_newline && character == '\n') || is_safe_char(character))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_text() {
        assert!(is_safe_text("BitBox", false));
        assert!(is_safe_text("BïtBöx Łódź", false));
        assert!(is_safe_text("first\nsecond", true));

        assert!(!is_safe_text("first\nsecond", false));
        assert!(!is_safe_text("tab\t", true));
        assert!(!is_safe_text("non\u{a0}breaking space", false));
        assert!(!is_safe_text("soft\u{ad}hyphen", false));
        assert!(!is_safe_text("東京", false));
        assert!(!is_safe_text("emoji 😃", false));
    }
}
