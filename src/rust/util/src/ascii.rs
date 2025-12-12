// SPDX-License-Identifier: Apache-2.0

#[derive(PartialEq)]
pub enum Charset {
    /// All printable ascii chars (including space):
    /// ```text
    /// !"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
    /// ```
    /// Note that newline, tab, etc. are not part of this set.
    All,
    /// Same as `All`, plus newline.
    AllNewline,
}

/// Returns true if all bytes are in the given `charset`.
pub fn is_printable_ascii<T: AsRef<[u8]>>(bytes: T, charset: Charset) -> bool {
    bytes
        .as_ref()
        .iter()
        .all(|&b| (32..=126).contains(&b) || (charset == Charset::AllNewline && b == b'\n'))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    static ALL_ASCII: &[u8] = "! \"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~".as_bytes();

    #[test]
    fn test_is_printable_ascii() {
        // All ascii chars.
        assert!(is_printable_ascii(ALL_ASCII, Charset::All));
        // Edge cases: highest and lowest non ascii chars.
        assert!(!is_printable_ascii(b"\x7f", Charset::All));
        assert!(!is_printable_ascii(b"\x19", Charset::All));
        assert!(!is_printable_ascii(b"\n", Charset::All));
        assert!(!is_printable_ascii(b"\t", Charset::All));
        // Works for any AsRef<[u8]>
        let trait_obj: &dyn AsRef<[u8]> = &"abc";
        assert!(is_printable_ascii(trait_obj, Charset::All));

        // Newline allowed
        assert!(is_printable_ascii("test\nnewline", Charset::AllNewline));
    }
}
