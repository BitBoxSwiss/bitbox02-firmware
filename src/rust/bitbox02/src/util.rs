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

/// Must be given a null-terminated string
pub unsafe fn strlen_ptr(ptr: *const u8) -> isize {
    let mut end = ptr;
    loop {
        if *end == 0 {
            // Can be changed to (*const u8).offset_from() when stabilized
            return isize::wrapping_sub(end as _, ptr as _);
        }
        end = end.offset(1);
    }
}

pub fn strlen_slice(input: &[u8]) -> usize {
    if let Some(nullidx) = input.iter().position(|&c| c == 0) {
        nullidx
    } else {
        input.len()
    }
}

/// Parses a utf-8 string out of a null terminated fixed length array
pub fn str_from_null_terminated(input: &[u8]) -> Result<&str, core::str::Utf8Error> {
    let len = strlen_slice(input);
    core::str::from_utf8(&input[0..len])
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
///     Err(_) => panic!("to short"),
/// };
/// ```
#[macro_export]
macro_rules! str_to_cstr {
    ($input:expr, $len:literal) => {{
        let mut buf = [0u8; $len + 1];
        if !$input.is_ascii() {
            Err(buf)
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
                Err(buf)
            } else {
                Ok(buf)
            }
        }
    }};
}
