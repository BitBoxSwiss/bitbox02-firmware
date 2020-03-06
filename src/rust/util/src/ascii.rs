/// Returns true if all bytes are in this set (including the space ' '):
///
/// `` !"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~``
///
/// Note that newline, tab, etc. are not part of this set.
pub fn all_ascii<T: AsRef<[u8]>>(bytes: T) -> bool {
    bytes.as_ref().iter().all(|&b| b >= 32 && b <= 126)
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    static ALL_ASCII: &[u8] = "! \"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~".as_bytes();

    #[test]
    fn test_all_ascii() {
        // All ascii chars.
        assert!(all_ascii(ALL_ASCII));
        // Edge cases: highest and lowest non ascii chars.
        assert!(!all_ascii(b"\x7f"));
        assert!(!all_ascii(b"\x19"));
        assert!(!all_ascii(b"\n"));
        assert!(!all_ascii(b"\t"));
        // Works for any AsRef<[u8]>
        let trait_obj: &dyn AsRef<[u8]> = &"abc";
        assert!(all_ascii(trait_obj));
    }
}
