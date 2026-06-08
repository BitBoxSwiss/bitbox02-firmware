// SPDX-License-Identifier: Apache-2.0

/// Validate a user given name. The name must be smaller or equal to `max_len` and larger than 0 in
/// size, consist only of safe display characters, not start or end with whitespace, and contain no
/// whitespace other than space.
pub fn validate(name: &str, max_len: usize) -> bool {
    if name.is_empty() || name.len() > max_len {
        return false;
    }
    if !super::display::is_safe_text(name, false) {
        return false;
    }
    if name.starts_with(' ') || name.ends_with(' ') {
        return false;
    }
    true
}

//
// C interface
//

/// Calls `util::name::validate()` on the provided C string.
///
/// # Safety
///
/// `buf` must point to a valid buffer of size `max_len`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_util_is_name_valid(buf: *const u8, max_len: usize) -> bool {
    if max_len == 0 {
        return false;
    }
    let slice = unsafe { core::slice::from_raw_parts(buf, max_len) };
    match core::ffi::CStr::from_bytes_until_nul(slice) {
        Ok(cstr) => match cstr.to_str() {
            Ok(s) => validate(s, max_len - 1),
            Err(_) => false,
        },
        Err(_) => false,
    }
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

        // Safe display characters.
        assert!(validate("some name", 100));
        assert!(validate("BïtBöx Łódź", 100));
        assert!(!validate("\n", 100));
        assert!(!validate("\t", 100));
        assert!(!validate("東京", 100));
        assert!(!validate("non\u{a0}breaking", 100));

        // Starts / ends with space.
        assert!(!validate(" foo", 100));
        assert!(!validate("foo ", 100));
    }

    #[test]
    // For clarity we want explicit null terminators in the strings below, not CStr literals
    // `c"..."`.
    #[allow(clippy::manual_c_str_literals)]
    fn test_rust_util_is_name_valid() {
        unsafe {
            // Valid
            assert!(rust_util_is_name_valid("foo\0".as_ptr(), 4));
            assert!(rust_util_is_name_valid("foo\0........".as_ptr(), 12));
            assert!(rust_util_is_name_valid("BïtBöx\0".as_ptr(), 10));

            // Invalid
            assert!(!rust_util_is_name_valid("fo\no\0".as_ptr(), 5));
            assert!(!rust_util_is_name_valid("".as_ptr(), 0));
            assert!(!rust_util_is_name_valid("foo\0".as_ptr(), 3));
            assert!(!rust_util_is_name_valid("foo".as_ptr(), 3));
        }
    }
}
