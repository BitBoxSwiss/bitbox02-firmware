// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
pub unsafe fn str_from_null_terminated_ptr<'a>(ptr: *const u8) -> Result<&'a str, ()> {
    core::ffi::CStr::from_ptr(ptr.cast()).to_str().or(Err(()))
}

/// truncate_str truncates string `s` to `len` chars. If `s` is
/// shorter than `len`, the string is returned unchanged (no panics).
pub fn truncate_str(s: &str, len: usize) -> &str {
    if s.len() > len {
        &s[..len]
    } else {
        s
    }
}

/// Converts a Rust string to a null terminated C string by appending a null
/// terminator.  Returns `Err(())` if the input already contians a null byte.
pub fn str_to_cstr_vec(input: &str) -> Result<Vec<u8>, ()> {
    Ok(alloc::ffi::CString::new(input)
        .or(Err(()))?
        .into_bytes_with_nul())
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
            unsafe { str_from_null_terminated_ptr(b"\0".as_ptr()) },
            Ok("")
        );
        assert_eq!(
            unsafe { str_from_null_terminated_ptr(b"hello\0".as_ptr()) },
            Ok("hello")
        );
        assert_eq!(
            unsafe { str_from_null_terminated_ptr(b"hello\0world".as_ptr()) },
            Ok("hello")
        );
        // valid utf8.
        assert_eq!(
            unsafe {
                str_from_null_terminated_ptr(
                    b"\xc3\xb6\xc3\xa4\xc3\xbc \xf0\x9f\x91\x8c\0world".as_ptr(),
                )
            },
            Ok("öäü 👌")
        );
        // invalid utf8 after the null terminator
        assert_eq!(
            unsafe { str_from_null_terminated_ptr(b"hello\0\xFF".as_ptr()) },
            Ok("hello")
        );
        // invalid utf8 before the null terminator
        assert!(unsafe { str_from_null_terminated_ptr(b"\xFF\0world".as_ptr()) }.is_err());
    }

    #[test]
    fn test_str_to_cstr_vec() {
        assert_eq!(str_to_cstr_vec(""), Ok(b"\0".to_vec()));
        assert_eq!(str_to_cstr_vec("test"), Ok(b"test\0".to_vec()));
        assert_eq!(str_to_cstr_vec("te\0st"), Err(()));
    }
}
