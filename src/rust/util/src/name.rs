/// Validate a user given name. The name must be smaller or equal to `max_len` and larger than 0 in
/// size, consist of printable ASCII characters only (and space), not
/// start or end with whitespace, and contain no whitespace other than space.
pub fn validate(name: &str, max_len: usize) -> bool {
    if name.len() == 0 || name.len() > max_len {
        return false;
    }
    if !super::ascii::all_ascii(name) {
        return false;
    }
    // Safe because all_ascii passed.
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
        // Min len.
        assert!(!validate("", 100));

        // Ascii.
        assert!(validate("some name", 100));
        assert!(!validate("\n", 100));
        assert!(!validate("\t", 100));

        // Starts / ends with space.
        assert!(!validate(" foo", 100));
        assert!(!validate("foo ", 100));
    }
}
