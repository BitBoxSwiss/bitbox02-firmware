// SPDX-License-Identifier: Apache-2.0

use core::ffi::{c_char, c_void};

pub use bitbox_i18n::{format, language_code, language_from_code, translate};

pub fn translate_current<'a>(english: &'a str) -> alloc::borrow::Cow<'a, str> {
    translate(crate::memory::get_device_language(), english)
}

/// # Safety
///
/// `input` must be a valid, null-terminated string pointer. `output` must be a valid pointer to a
/// buffer of at least `output_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_i18n_translate_copy(
    input: *const c_char,
    output: *mut c_char,
    output_len: usize,
) -> bool {
    if input.is_null() || output.is_null() || output_len == 0 {
        return false;
    }
    let Ok(input) = (unsafe { util::strings::str_from_null_terminated_ptr(input) }) else {
        unsafe { *output = 0 };
        return false;
    };
    let translated = translate_current(input);
    copy_to_cstr(&translated, output.cast::<c_void>(), output_len)
}

fn copy_to_cstr(input: &str, output: *mut c_void, output_len: usize) -> bool {
    let mut len = input.len().min(output_len - 1);
    while !input.is_char_boundary(len) {
        len -= 1;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(input.as_ptr(), output.cast::<u8>(), len);
        *output.cast::<u8>().add(len) = 0;
    }
    len == input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_to_cstr_truncates_to_char_boundary() {
        let input = core::str::from_utf8(b"Gr\xc3\xb6sse").unwrap();
        let mut output = [0xff; 4];

        assert!(!copy_to_cstr(
            input,
            output.as_mut_ptr().cast::<c_void>(),
            output.len()
        ));
        assert_eq!(&output[..3], b"Gr\0");
        assert_eq!(output[3], 0xff);
    }
}
