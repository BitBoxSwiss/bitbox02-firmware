// SPDX-License-Identifier: Apache-2.0

/// Validate a user given name. The name must be smaller or equal to `max_len` and larger than 0 in
/// size, contain no control characters, not start or end with whitespace, and contain no
/// whitespace other than space.
/// Callers must check glyph availability before displaying the name.
pub fn validate(name: &str, max_len: usize) -> bool {
    if name.is_empty() || name.len() > max_len {
        return false;
    }
    if name
        .chars()
        .any(|c| c.is_control() || (c.is_whitespace() && c != ' '))
    {
        return false;
    }
    let bytes = name.as_bytes();
    if bytes[0] == b' ' || bytes[bytes.len() - 1] == b' ' {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_validate() {
        // Max len.
        assert!(validate("foo", 5));
        assert!(validate("foo", 4));
        assert!(validate("foo", 3));
        assert!(!validate("foo", 2));
        assert!(validate("ä", 2));
        assert!(!validate("ä", 1));
        // Min len.
        assert!(!validate("", 100));

        // Name characters.
        assert!(validate("some name", 100));
        assert!(validate("BïtBöx Zürich", 100));
        assert!(!validate("\n", 100));
        assert!(!validate("\t", 100));
        assert!(validate("東京", 100));
        assert!(!validate("non\u{a0}breaking", 100));

        // Starts / ends with space.
        assert!(!validate(" foo", 100));
        assert!(!validate("foo ", 100));
    }
}
