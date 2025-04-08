// Copyright 2020 Shift Cryptosecurity AG
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

use util::c_types::c_uchar;

/// Zero a buffer using volatile writes. Accepts null-ptr and 0-length buffers and does nothing.
///
/// * `dst` - Buffer to zero
#[no_mangle]
pub extern "C" fn rust_util_zero(mut dst: BytesMut) {
    if dst.buf.is_null() || dst.len == 0 {
        return;
    }
    util::zero(dst.as_mut())
}

/// Calls `util::name::validate()` on the provided C string.
/// SAFETY:
/// `buf` must point to a valid buffer of size `max_len`.
#[no_mangle]
pub unsafe extern "C" fn rust_util_is_name_valid(buf: *const u8, max_len: usize) -> bool {
    if max_len == 0 {
        return false;
    }
    let slice = core::slice::from_raw_parts(buf, max_len);
    match core::ffi::CStr::from_bytes_until_nul(slice) {
        Ok(cstr) => match cstr.to_str() {
            Ok(s) => util::name::validate(s, max_len - 1),
            Err(_) => false,
        },
        Err(_) => false,
    }
}

/// Convert bytes to hex representation
///
/// * `buf` - bytes to convert to hex.
/// * `out` - hex will be written here. out len must be at least 2*buf.len+1.
#[no_mangle]
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
    buf: *const c_uchar,
    len: usize,
}

#[repr(C)]
pub struct BytesMut {
    buf: *mut c_uchar,
    len: usize,
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
/// SAFTEY: buf must not be NULL and point to a valid memory area of size `len`.
#[no_mangle]
pub unsafe extern "C" fn rust_util_bytes(buf: *const c_uchar, len: usize) -> Bytes {
    Bytes { buf, len }
}

/// Convert buffer to mutable slice
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf[len-1]` must be a valid dereference
///
/// SAFTEY: buf must not be NULL and point to a valid memory area of size `len`.
#[no_mangle]
pub unsafe extern "C" fn rust_util_bytes_mut(buf: *mut c_uchar, len: usize) -> BytesMut {
    BytesMut { buf, len }
}

/// Base58Check-encode the input.
#[cfg(feature = "c-unit-testing")]
#[no_mangle]
pub extern "C" fn rust_base58_encode_check(buf: Bytes, mut out: BytesMut) -> bool {
    if buf.len == 0 {
        return false;
    }
    let encoded = bitcoin::base58::encode_check(buf.as_ref());
    out.as_mut()[..encoded.len()].copy_from_slice(encoded.as_bytes());
    // Null-terminator.
    out.as_mut()[encoded.len()] = 0;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn zeroing() {
        let mut buf = [1u8, 2, 3, 4];
        rust_util_zero(unsafe { rust_util_bytes_mut(buf.as_mut_ptr(), buf.len() - 1) });
        assert_eq!(&buf[..], &[0, 0, 0, 4]);
    }

    #[test]
    fn zeroing_empty() {
        let mut buf = [];
        rust_util_zero(unsafe { rust_util_bytes_mut(buf.as_mut_ptr(), 0) });
    }

    #[test]
    fn zeroing_null() {
        rust_util_zero(unsafe { rust_util_bytes_mut(core::ptr::null_mut(), 0) });
    }

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

    #[test]
    fn test_rust_base58_encode_check() {
        let buf = b"test";
        let mut result_buf = [0u8; 100];
        assert!(rust_base58_encode_check(
            unsafe { rust_util_bytes(buf.as_ptr(), buf.len()) },
            unsafe { rust_util_bytes_mut(result_buf.as_mut_ptr(), result_buf.len()) },
        ));
        let expected = b"LUC1eAJa5jW\0";
        assert_eq!(&result_buf[..expected.len()], expected);
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

            // Invalid
            assert!(!rust_util_is_name_valid("fo\no\0".as_ptr(), 5));
            assert!(!rust_util_is_name_valid("".as_ptr(), 0));
            assert!(!rust_util_is_name_valid("foo\0".as_ptr(), 3));
            assert!(!rust_util_is_name_valid("foo".as_ptr(), 3));
        }
    }
}
