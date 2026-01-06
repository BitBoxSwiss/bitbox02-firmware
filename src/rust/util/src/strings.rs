// SPDX-License-Identifier: Apache-2.0

extern crate alloc;
use alloc::vec::Vec;

/// Parses a utf-8 string out of a null terminated buffer. Returns `Err(())` if there
/// is no null terminator or if the bytes before the null terminator is invalid UTF8.
pub fn str_from_null_terminated(input: &[u8]) -> Result<&str, ()> {
    core::ffi::CStr::from_bytes_until_nul(input)
        .or(Err(()))?
        .to_str()
        .or(Err(()))
}

/// Parses a utf-8 string out of a null terminated buffer starting at `ptr`. Returns `Err(())` if
/// the bytes before the null terminator is invalid UTF8.
///
/// # Safety `ptr` must be not null and be a null terminated string. The resulting string is only
/// valid as long the memory pointed to by `ptr` is valid.
pub unsafe fn str_from_null_terminated_ptr<'a>(
    ptr: *const core::ffi::c_char,
) -> Result<&'a str, ()> {
    unsafe { core::ffi::CStr::from_ptr(ptr.cast()).to_str().or(Err(())) }
}

/// truncate_str truncates string `s` to `len` chars. If `s` is
/// shorter than `len`, the string is returned unchanged (no panics).
pub fn truncate_str(s: &str, len: usize) -> &str {
    if s.len() > len { &s[..len] } else { s }
}

/// Converts a Rust string to a null terminated C string by appending a null
/// terminator.  Returns `Err(())` if the input already contains a null byte.
pub fn str_to_cstr_vec(input: &str) -> Result<Vec<core::ffi::c_char>, ()> {
    let cstr = alloc::ffi::CString::new(input)
        .or(Err(()))?
        .into_bytes_with_nul();
    // into_bytes_with_nul always returns Vec<u8> independent of platform. Let's cast it to c_char
    // which is platform specific (unsigned on some platforms, signed on others).
    // Implemented without unsafe on purpose.
    Ok(cstr.into_iter().map(|c| c as _).collect())
}

/// Converts a Rust string to a null terminated C string by appending a null
/// terminator.  Returns `Err(())` if the input already contains a null byte.
pub fn str_to_cstr_vec_zeroizing(
    input: &str,
) -> Result<zeroize::Zeroizing<Vec<core::ffi::c_char>>, ()> {
    let bytes = input.as_bytes();
    if bytes.contains(&0) {
        return Err(());
    }
    let mut result: zeroize::Zeroizing<Vec<core::ffi::c_char>> =
        zeroize::Zeroizing::new(vec![0; bytes.len() + 1]);
    for (i, b) in bytes.iter().enumerate() {
        result[i] = *b as _;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_str() {
        assert_eq!(truncate_str("test", 0), "");
        assert_eq!(truncate_str("test", 1), "t");
        assert_eq!(truncate_str("test", 2), "te");
        assert_eq!(truncate_str("test", 3), "tes");
        assert_eq!(truncate_str("test", 4), "test");
        assert_eq!(truncate_str("test", 5), "test");
        assert_eq!(truncate_str("test", 6), "test");
    }

    #[test]
    fn test_str_from_null_terminated() {
        assert_eq!(str_from_null_terminated(b"\0"), Ok(""));
        assert_eq!(str_from_null_terminated(b"hello\0"), Ok("hello"));
        assert_eq!(str_from_null_terminated(b"hello\0world"), Ok("hello"));
        // valid utf8.
        assert_eq!(
            str_from_null_terminated(b"\xc3\xb6\xc3\xa4\xc3\xbc \xf0\x9f\x91\x8c\0world"),
            Ok("öäü 👌")
        );
        // invalid utf8 after the null terminator
        assert_eq!(str_from_null_terminated(b"hello\0\xFF"), Ok("hello"));
        // invalid utf8 before the null terminator
        assert!(str_from_null_terminated(b"\xFF\0world").is_err());
        // Not null terminated.
        assert!(str_from_null_terminated(b"").is_err());
        assert!(str_from_null_terminated(b"foo").is_err());
    }

    #[test]
    #[allow(clippy::manual_c_str_literals)]
    fn test_str_from_null_terminated_ptr() {
        assert_eq!(
            unsafe { str_from_null_terminated_ptr(b"\0".as_ptr().cast()) },
            Ok("")
        );
        assert_eq!(
            unsafe { str_from_null_terminated_ptr(b"hello\0".as_ptr().cast()) },
            Ok("hello")
        );
        assert_eq!(
            unsafe { str_from_null_terminated_ptr(b"hello\0world".as_ptr().cast()) },
            Ok("hello")
        );
        // valid utf8.
        assert_eq!(
            unsafe {
                str_from_null_terminated_ptr(
                    b"\xc3\xb6\xc3\xa4\xc3\xbc \xf0\x9f\x91\x8c\0world"
                        .as_ptr()
                        .cast(),
                )
            },
            Ok("öäü 👌")
        );
        // invalid utf8 after the null terminator
        assert_eq!(
            unsafe { str_from_null_terminated_ptr(b"hello\0\xFF".as_ptr().cast()) },
            Ok("hello")
        );
        // invalid utf8 before the null terminator
        assert!(unsafe { str_from_null_terminated_ptr(b"\xFF\0world".as_ptr().cast()) }.is_err());
    }

    #[test]
    fn test_str_to_cstr_vec() {
        assert_eq!(str_to_cstr_vec(""), Ok(vec![0]));
        assert_eq!(
            str_to_cstr_vec("test"),
            Ok(b"test\0"
                .iter()
                .map(|c| *c as _)
                .collect::<Vec<core::ffi::c_char>>())
        );
        assert_eq!(str_to_cstr_vec("te\0st"), Err(()));
    }

    #[test]
    fn test_str_to_cstr_vec_zeroizing() {
        assert_eq!(str_to_cstr_vec_zeroizing("").unwrap().as_slice(), &[0]);
        assert_eq!(
            str_to_cstr_vec_zeroizing("test").unwrap(),
            b"test\0"
                .iter()
                .map(|c| *c as _)
                .collect::<Vec<core::ffi::c_char>>()
                .into(),
        );
        assert_eq!(str_to_cstr_vec_zeroizing("te\0st"), Err(()));
    }
}
