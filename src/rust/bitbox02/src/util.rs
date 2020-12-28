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

/// Must be given a null-terminated string
/// # Safety
/// ptr must be not NULL and the memory must be valid until a null byte.
pub unsafe fn strlen_ptr(ptr: *const u8) -> isize {
    let mut end = ptr;
    loop {
        if *end == 0 {
            return end.offset_from(ptr);
        }
        end = end.offset(1);
    }
}

/// Parses a utf-8 string out of a null terminated buffer. Returns `Err(())` if there
/// is no null terminator or if the bytes before the null terminator is invalid UTF8.
pub fn str_from_null_terminated(input: &[u8]) -> Result<&str, ()> {
    let len = input.iter().position(|&c| c == 0).ok_or(())?;
    core::str::from_utf8(&input[0..len]).or(Err(()))
}

/// Macro for creating a stack allocated buffer with the content of a string and a null-terminator
///
/// Example usage:
///
/// ```
/// # #[macro_use] extern crate bitbox02;
/// let name = "sample_string";
/// let buf = match str_to_cstr!(name, 50) {
///     Ok(buf) => buf,
///     Err(msg) => panic!(msg),
/// };
/// ```
#[macro_export]
macro_rules! str_to_cstr {
    ($input:expr, $len:expr) => {{
        let mut buf = [0u8; $len + 1];
        if !$input.is_ascii() {
            Err("non-ascii input")
        } else {
            let len = core::cmp::min($len, $input.len());
            {
                // Take a slice of buf of the correct length
                let buf = &mut buf[..len];
                // Take a slice of input of the correct length
                let input = &$input.as_bytes()[..len];
                buf.copy_from_slice(input);
            }
            if $input.len() > len {
                Err("str is too long")
            } else {
                Ok(buf)
            }
        }
    }};
}

#[macro_export]
macro_rules! str_to_cstr_force {
    ($input:expr, $len:expr) => {
        match $crate::str_to_cstr!($input, $len) {
            Ok(buf) => buf,
            Err(msg) => panic!(msg),
        }
    };
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
    let bytes = input.as_bytes();
    if bytes.contains(&0) {
        Err(())
    } else {
        let mut out = Vec::with_capacity(input.len() + 1);
        out.extend_from_slice(bytes);
        out.push(0); // null terminator
        Ok(out)
    }
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
    fn test_strlen_ptr() {
        assert_eq!(unsafe { strlen_ptr(b"\0".as_ptr()) }, 0);
        assert_eq!(unsafe { strlen_ptr(b"a\0".as_ptr()) }, 1);
        assert_eq!(unsafe { strlen_ptr(b"abcdef\0".as_ptr()) }, 6);
        assert_eq!(unsafe { strlen_ptr(b"abcdef\0defghji".as_ptr()) }, 6);
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
    fn test_str_to_cstr_vec() {
        assert_eq!(str_to_cstr_vec(""), Ok(b"\0".to_vec()));
        assert_eq!(str_to_cstr_vec("test"), Ok(b"test\0".to_vec()));
        assert_eq!(str_to_cstr_vec("te\0st"), Err(()));
    }
}
