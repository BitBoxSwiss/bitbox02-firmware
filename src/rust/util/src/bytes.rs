// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_uchar;

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
        Err(err) => panic!("{:?}", err),
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
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf[len-1]` must be a valid dereference
///
/// # Safety
///
/// buf must not be NULL and point to a valid memory area of size `len`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_util_bytes(buf: *const c_uchar, len: usize) -> Bytes {
    Bytes { buf, len }
}

/// Convert buffer to mutable slice
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf[len-1]` must be a valid dereference
///
/// # Safety
///
/// buf must not be NULL and point to a valid memory area of size `len`.
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
}
