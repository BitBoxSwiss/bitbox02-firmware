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

use util::c_types::{c_char, c_uchar, size_t};

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

#[no_mangle]
pub extern "C" fn rust_util_all_ascii_bytes(bytes: Bytes) -> bool {
    util::ascii::all_ascii(bytes)
}

#[no_mangle]
pub extern "C" fn rust_util_all_ascii(cstr: CStr) -> bool {
    let s: &str = cstr.as_ref();
    util::ascii::all_ascii(s)
}

#[no_mangle]
pub extern "C" fn rust_util_validate_name(cstr: CStr, max_len: size_t) -> bool {
    let s: &str = cstr.as_ref();
    util::name::validate(s, max_len)
}

/// Convert bytes to hex representation
///
/// * `buf` - bytes to convert to hex.
/// * `out` - hex will be written here. out len must be at least 2*buf.len+1.
#[no_mangle]
pub extern "C" fn rust_util_uint8_to_hex(buf: Bytes, mut out: CStrMut) {
    let min_len = buf.len * 2;
    out.write(min_len, |out| {
        hex::encode_to_slice(&buf, out).unwrap();
    });
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
        let mut buf = self.buf;
        if self.len == 0 && self.buf.is_null() {
            buf = core::ptr::NonNull::dangling().as_ptr();
        }
        assert!(!buf.is_null());
        unsafe { core::slice::from_raw_parts(buf, self.len) }
    }
}

impl AsRef<[u8]> for BytesMut {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    fn as_ref(&self) -> &[u8] {
        let mut buf = self.buf;
        if self.len == 0 && self.buf.is_null() {
            buf = core::ptr::NonNull::dangling().as_ptr();
        }
        assert!(!buf.is_null());
        unsafe { core::slice::from_raw_parts(buf, self.len) }
    }
}

impl AsMut<[u8]> for BytesMut {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    fn as_mut(&mut self) -> &mut [u8] {
        let mut buf = self.buf;
        if self.len == 0 && self.buf.is_null() {
            buf = core::ptr::NonNull::dangling().as_ptr();
        }
        assert!(!buf.is_null());
        unsafe { core::slice::from_raw_parts_mut(buf, self.len) }
    }
}

/// CStr is a null-terminated string. Null pointers are interpreted as empty strings.
#[repr(C)]
pub struct CStr {
    buf: *const c_char,
    len: usize,
}

impl CStr {
    /// Create a CStr from a null-terminated string or null pointer. Unsafe because it will read
    /// until it finds a null character.
    pub unsafe fn new(buf: *const c_char) -> Self {
        let mut buf = buf;
        let mut len = 0;
        if buf.is_null() {
            buf = core::ptr::NonNull::dangling().as_ptr();
            len = 0;
        } else {
            let mut b = buf;
            while b.read() != 0 {
                len += 1;
                b = b.offset(1);
            }
        }
        CStr { buf, len }
    }
}

impl AsRef<str> for CStr {
    fn as_ref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(self.buf, self.len)) }
    }
}

/// CStrMut is a "growable" container which keeps track of some array allocated by C with a length
/// and a capacity state. It always contains a null-terminated string. The string (excluding null
/// terminator) can therefore be maximally `capacity-1` long.
#[repr(C)]
pub struct CStrMut {
    buf: *mut c_char,
    len: usize,
    cap: usize,
}

impl CStrMut {
    /// Create a new growable string with capacity `cap`. Only allowed for non-null pointers with
    /// length or null pointers with 0 length due to limitation in `core::slice`. Unsafe because it
    /// will read until it finds a null character.
    pub unsafe fn new(buf: *mut c_char, cap: usize) -> Self {
        let mut len = 0;
        let mut buf = buf;
        if buf.is_null() {
            if cap != 0 {
                panic!("Null pointer can't have capacity");
            }
            buf = core::ptr::NonNull::dangling().as_ptr();
        } else {
            let mut b = buf;
            while b.read() != 0 {
                len += 1;
                b = b.offset(1);
                if len == cap {
                    panic!("CStrMut not null terminated");
                }
            }
        }

        CStrMut { buf, len, cap }
    }

    /// Provide a mutable slice to an unused range of the buffer. The provided function `f` must
    /// fill the requested buffer with utf-8 valid characters and it must not write a null
    /// character in the buffer.
    ///
    /// # Panics
    ///
    /// This function panics in case the provided buffer contains NULL or non-valid utf-8
    /// characters after function `f` is applied. It will also panic if more bytes are requested
    /// then are available.
    pub fn write<F>(&mut self, req: usize, f: F)
    where
        F: FnOnce(&mut [u8]) -> (),
    {
        // Must be room for requested amount of bytes and null terminator.
        if self.cap - self.len < req + 1 {
            panic!("Not enough bytes left in buffer");
        }
        let len = self.len;
        let slice = unsafe { self.as_bytes_mut() };
        let slice = &mut slice[len..len + req + 1];
        let write_slice = &mut slice[0..req];
        f(write_slice);
        if write_slice.iter().any(|&c| c == 0) {
            panic!("null terminated strings can't contain null");
        }
        if core::str::from_utf8(write_slice).is_err() {
            panic!("strings must be valid utf-8");
        }
        slice[req] = 0;
        self.len += req;
    }

    /// Get slice of underlying byte array. Unsafe because you have to ensure that length is up to
    /// date and that there is a null character at `buf[len]`.
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        core::slice::from_raw_parts_mut(self.buf, self.cap)
    }
}

impl AsRef<str> for CStrMut {
    fn as_ref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(self.buf, self.len)) }
    }
}

impl core::fmt::Write for CStrMut {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write(s.len(), |buf| {
            buf.copy_from_slice(s.as_bytes());
        });
        Ok(())
    }
}

/// Convert buffer to slice
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf[len-1]` must be a valid dereference
#[no_mangle]
pub extern "C" fn rust_util_bytes(buf: *const c_uchar, len: usize) -> Bytes {
    Bytes { buf, len }
}

/// Convert buffer to mutable slice
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf[len-1]` must be a valid dereference
#[no_mangle]
pub extern "C" fn rust_util_bytes_mut(buf: *mut c_uchar, len: usize) -> BytesMut {
    BytesMut { buf, len }
}

/// Convert buffer to str.
///
/// * `buf` - Must be a valid pointer to a null terminated array of bytes.
#[no_mangle]
pub unsafe extern "C" fn rust_util_cstr(buf: *const c_char) -> CStr {
    CStr::new(buf)
}

/// Convert buffer to mutable str. The whole buffer is considered empty from start.
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `cap` - Length of buffer, `buf_ptr[cap-1]` must be a valid dereference
#[no_mangle]
pub unsafe extern "C" fn rust_util_cstr_mut(buf: *mut c_char, cap: usize) -> CStrMut {
    if !buf.is_null() {
        buf.write(0);
    }
    CStrMut::new(buf, cap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn zeroing() {
        let mut buf = [1u8, 2, 3, 4];
        rust_util_zero(rust_util_bytes_mut(buf.as_mut_ptr(), buf.len() - 1));
        assert_eq!(&buf[..], &[0, 0, 0, 4]);
    }

    #[test]
    fn zeroing_empty() {
        let mut buf = [];
        rust_util_zero(rust_util_bytes_mut(buf.as_mut_ptr(), 0));
    }

    #[test]
    fn zeroing_null() {
        rust_util_zero(rust_util_bytes_mut(core::ptr::null_mut(), 0));
    }

    #[test]
    fn test_rust_util_cstr() {
        let cstr = unsafe { rust_util_cstr(b"\0".as_ptr()) };
        assert_eq!(cstr.as_ref(), "");
        assert_eq!(cstr.len, 0);

        let cstr = unsafe { rust_util_cstr(b"foo\0bar".as_ptr()) };
        assert_eq!(cstr.as_ref(), "foo");
        assert_eq!(cstr.len, 3);
    }

    #[test]
    #[should_panic]
    fn create_invalid_bytes_mut() {
        // Calling `as_mut()` will panic because it tries to create an invalid rust slice.
        rust_util_bytes_mut(core::ptr::null_mut(), 1).as_mut();
    }

    #[test]
    #[should_panic]
    fn create_invalid_bytes_ref() {
        // Calling `as_ref()` will panic because it tries to create an invalid rust slice.
        rust_util_bytes(core::ptr::null(), 1).as_ref();
    }

    #[test]
    fn test_cstr_mut() {
        let mut start = String::from("foo\0bar");
        let mut cstr_mut = unsafe { rust_util_cstr_mut(start.as_mut_ptr(), start.len()) };
        assert_eq!(cstr_mut.len, 0);
        assert_eq!(cstr_mut.as_ref(), "");
        cstr_mut.write(1, |buf| buf[0] = b'g');
        assert_eq!(cstr_mut.as_ref(), "g");
    }

    #[test]
    fn test_cstr_mut_new() {
        let mut start = String::from("foo\0bar");
        let mut cstr_mut = unsafe { CStrMut::new(start.as_mut_ptr(), start.len()) };
        assert_eq!(cstr_mut.len, 3);
        assert_eq!(cstr_mut.as_ref(), "foo");
        cstr_mut.write(1, |buf| buf[0] = b'g');
        assert_eq!(cstr_mut.as_ref(), "foog");
    }

    #[test]
    #[should_panic]
    fn test_invalid_cstr_mut() {
        let mut buf = [1, 2, 3];
        let cstr_mut = unsafe { CStrMut::new(buf.as_mut_ptr(), buf.len()) };
        // panics as there is no null terminator.
        cstr_mut.as_ref();
    }

    #[test]
    #[should_panic]
    fn test_invalid_cstr_mut_write_null() {
        let mut s = String::from("abc\0xxx");
        let mut cstr_mut = unsafe { CStrMut::new(s.as_mut_ptr(), s.len()) };
        cstr_mut.write(1, |buf| buf[0] = 0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_cstr_mut_out_of_buffer() {
        let mut s = String::from("abc\0");
        let mut cstr_mut = unsafe { CStrMut::new(s.as_mut_ptr(), s.len()) };
        cstr_mut.write(1, |buf| buf[0] = b'd');
    }

    #[test]
    fn test_cstr_mut_write() {
        let mut buf = vec![0; 9];
        let mut cstr_mut = unsafe { CStrMut::new(buf.as_mut_ptr(), buf.len()) };
        use std::fmt::Write;
        assert!(write!(&mut cstr_mut, "test").is_ok());
        assert!(buf.starts_with(b"test\0"));
        assert!(write!(&mut cstr_mut, " foo").is_ok());
        assert!(buf.starts_with(b"test foo\0"));
    }

    #[test]
    #[should_panic]
    fn test_cstr_mut_write_too_much() {
        let mut buf = vec![0; 9];
        let mut cstr_mut = unsafe { CStrMut::new(buf.as_mut_ptr(), buf.len()) };
        use std::fmt::Write;
        let _ = write!(&mut cstr_mut, "test foo ");
    }

    #[test]
    fn test_all_ascii_bytes() {
        let buf = b"foo";
        assert!(rust_util_all_ascii_bytes(rust_util_bytes(
            buf.as_ptr(),
            buf.len()
        )));
    }

    #[test]
    fn u8_to_hexing() {
        let buf = [1u8, 2, 3, 14, 15, 255, 1];
        let mut string = String::from("\0xxxxxxxxxxxxx");
        rust_util_uint8_to_hex(rust_util_bytes(buf.as_ptr(), buf.len() - 1), unsafe {
            rust_util_cstr_mut(string.as_mut_ptr(), string.len() - 1)
        });
        assert_eq!(string, "0102030e0fff\0x");

        // Bigger buffer also works.
        let mut string = String::from("\0xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        rust_util_uint8_to_hex(rust_util_bytes(buf.as_ptr(), buf.len() - 1), unsafe {
            rust_util_cstr_mut(string.as_mut_ptr(), string.len())
        });
        assert_eq!(string, "0102030e0fff\0xxxxxxxxxxxxxxxxxxxxxxx");
    }
}
