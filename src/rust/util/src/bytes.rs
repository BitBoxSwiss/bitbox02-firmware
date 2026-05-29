// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_uchar;

/// Format an unsigned integer as ASCII decimal into `out`.
///
/// The formatted number is left-padded to `min_width` with `pad`, null-terminated,
/// and the byte length excluding the null terminator is returned.
///
/// Panics if `out` cannot fit the formatted number and null terminator.
#[unsafe(no_mangle)]
pub extern "C" fn rust_format_uint(
    mut out: BytesMut,
    value: u32,
    min_width: u8,
    pad: c_uchar,
) -> usize {
    let mut digit_count = 1;
    let mut remaining = value;
    while remaining >= 10 {
        digit_count += 1;
        remaining /= 10;
    }

    let formatted_len = core::cmp::max(digit_count, min_width as usize);
    let out = out.as_mut();
    assert!(formatted_len < out.len());

    out[formatted_len] = 0;

    let mut value = value;
    let mut out_pos = formatted_len;
    loop {
        out_pos -= 1;
        out[out_pos] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    out[..out_pos].fill(pad);
    formatted_len
}

/// Convert bytes to hex representation
///
/// * `buf` - bytes to convert to hex.
/// * `out` - hex will be written here. out len must be at least 2*buf.len+1.
#[unsafe(no_mangle)]
pub extern "C" fn rust_util_uint8_to_hex(buf: Bytes, mut out: BytesMut) {
    let bytes = buf.as_ref();
    let hexlen = bytes.len() * 2;
    // Avoid .unwrap() here until the following compiler regression is fixed:
    // https://github.com/rust-lang/rust/issues/83925
    match hex::encode_to_slice(bytes, &mut out.as_mut()[..hexlen]) {
        Ok(()) => {}
        Err(_) => panic!("hex encoding failed"),
    }
    // Null terminator.
    out.as_mut()[hexlen] = 0;
}

#[repr(C)]
pub struct Bytes {
    pub(crate) buf: *const c_uchar,
    pub(crate) len: usize,
}

impl Bytes {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[repr(C)]
pub struct BytesMut {
    pub(crate) buf: *mut c_uchar,
    pub(crate) len: usize,
}

impl BytesMut {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl AsRef<[u8]> for Bytes {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    fn as_ref(&self) -> &[u8] {
        let buf = if self.len == 0 && self.buf.is_null() {
            core::ptr::NonNull::dangling().as_ptr()
        } else {
            self.buf
        };
        assert!(!buf.is_null());
        unsafe { core::slice::from_raw_parts(buf, self.len) }
    }
}

impl AsRef<[u8]> for BytesMut {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    fn as_ref(&self) -> &[u8] {
        let buf = if self.len == 0 && self.buf.is_null() {
            core::ptr::NonNull::dangling().as_ptr()
        } else {
            self.buf
        };
        assert!(!buf.is_null());
        unsafe { core::slice::from_raw_parts(buf, self.len) }
    }
}

impl AsMut<[u8]> for BytesMut {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    fn as_mut(&mut self) -> &mut [u8] {
        let buf = if self.len == 0 && self.buf.is_null() {
            core::ptr::NonNull::dangling().as_ptr()
        } else {
            self.buf
        };
        assert!(!buf.is_null());
        unsafe { core::slice::from_raw_parts_mut(buf, self.len) }
    }
}

/// Convert buffer to slice
///
/// * `buf` - Must be a valid pointer to an array of bytes, can be NULL if `len == 0`
/// * `len` - Length of buffer, `buf[len-1]` must be a valid dereference
///
/// # Safety
///
/// `buf` must point to a valid memory area of size `len`, unless `len == 0`, in which case `buf`
/// may be NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_util_bytes(buf: *const c_uchar, len: usize) -> Bytes {
    Bytes { buf, len }
}

/// Convert buffer to mutable slice
///
/// * `buf` - Must be a valid pointer to an array of bytes, can be NULL if `len == 0`
/// * `len` - Length of buffer, `buf[len-1]` must be a valid dereference
///
/// # Safety
///
/// `buf` must point to a valid memory area of size `len`, unless `len == 0`, in which case `buf`
/// may be NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_util_bytes_mut(buf: *mut c_uchar, len: usize) -> BytesMut {
    BytesMut { buf, len }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;
    #[test]
    #[should_panic]
    fn create_invalid_bytes_mut() {
        // Calling `as_mut()` will panic because it tries to create an invalid rust slice.
        unsafe {
            rust_util_bytes_mut(core::ptr::null_mut(), 1).as_mut();
        }
    }

    #[test]
    #[should_panic]
    fn create_invalid_bytes_ref() {
        // Calling `as_ref()` will panic because it tries to create an invalid rust slice.
        (unsafe { rust_util_bytes(core::ptr::null(), 1) }).as_ref();
    }

    #[test]
    fn create_null_bytes_ref_with_zero_len() {
        let bytes = unsafe { rust_util_bytes(core::ptr::null(), 0) };
        assert!(bytes.as_ref().is_empty());
    }

    #[test]
    fn create_null_bytes_mut_with_zero_len() {
        let mut bytes = unsafe { rust_util_bytes_mut(core::ptr::null_mut(), 0) };
        assert!(bytes.as_ref().is_empty());
        assert!(bytes.as_mut().is_empty());
    }

    #[test]
    fn test_uint8_to_hex() {
        let buf = [1u8, 2, 3, 14, 15, 255];
        let mut string = String::from("xxxxxxxxxxxxx");
        rust_util_uint8_to_hex(
            unsafe { rust_util_bytes(buf.as_ptr(), buf.len()) },
            unsafe { rust_util_bytes_mut(string.as_mut_ptr(), string.len()) },
        );
        assert_eq!(string, "0102030e0fff\0");

        // Bigger buffer also works.
        let mut string = String::from("\0xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        rust_util_uint8_to_hex(
            unsafe { rust_util_bytes(buf.as_ptr(), buf.len()) },
            unsafe { rust_util_bytes_mut(string.as_mut_ptr(), string.len()) },
        );
        assert_eq!(string, "0102030e0fff\0xxxxxxxxxxxxxxxxxxxxxxx");
    }

    #[test]
    fn test_format_uint() {
        let mut string = String::from("xxxxxxxxxxx");
        let len = rust_format_uint(
            unsafe { rust_util_bytes_mut(string.as_mut_ptr(), string.len()) },
            0,
            1,
            b'0',
        );
        assert_eq!(len, 1);
        assert_eq!(string, "0\0xxxxxxxxx");

        let mut string = String::from("xxxxxxxxxxx");
        let len = rust_format_uint(
            unsafe { rust_util_bytes_mut(string.as_mut_ptr(), string.len()) },
            42,
            4,
            b'0',
        );
        assert_eq!(len, 4);
        assert_eq!(string, "0042\0xxxxxx");

        let mut string = String::from("xxxxxxxxxxx");
        let len = rust_format_uint(
            unsafe { rust_util_bytes_mut(string.as_mut_ptr(), string.len()) },
            7,
            2,
            b' ',
        );
        assert_eq!(len, 2);
        assert_eq!(string, " 7\0xxxxxxxx");

        let mut string = String::from("xxxxxxxxxxx");
        let len = rust_format_uint(
            unsafe { rust_util_bytes_mut(string.as_mut_ptr(), string.len()) },
            u32::MAX,
            1,
            b'0',
        );
        assert_eq!(len, 10);
        assert_eq!(string, "4294967295\0");
    }
}
