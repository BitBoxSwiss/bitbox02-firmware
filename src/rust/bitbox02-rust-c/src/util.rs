use util::c_types::{c_char, c_uchar};

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

/// Convert bytes to hex representation
///
/// * `buf_ptr` - Must be a valid pointer to an array of bytes
/// * `buf_len` - Length of buffer, `buf_ptr[buf_len-1]` must be a valid dereference
/// * `out_ptr` - Must be a valid pointer to an array of bytes that is buf_len*2+1 long
#[no_mangle]
pub extern "C" fn rust_util_uint8_to_hex(buf: Bytes, mut out: CStrMut) {
    let out_len = out.as_ref().len();
    // UNSAFE: We promise that we never write non-utf8 valid bytes to the `str`.
    let out = unsafe { out.as_mut().as_bytes_mut() };
    hex::encode_to_slice(&buf, &mut out[0..out_len - 1]).unwrap();
    out[out_len - 1] = b'\0';
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

#[repr(C)]
pub struct CStr {
    buf: *const c_char,
    len: usize,
}

#[repr(C)]
pub struct CStrMut {
    buf: *mut c_char,
    len: usize,
}

impl AsRef<str> for CStr {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    fn as_ref(&self) -> &str {
        let mut buf = self.buf;
        if self.len == 0 && self.buf.is_null() {
            buf = core::ptr::NonNull::dangling().as_ptr();
        }
        assert!(!buf.is_null());
        unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, self.len)) }
    }
}

impl AsRef<str> for CStrMut {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    fn as_ref(&self) -> &str {
        let mut buf = self.buf;
        if self.len == 0 && self.buf.is_null() {
            buf = core::ptr::NonNull::dangling().as_ptr();
        }
        assert!(!buf.is_null());
        unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, self.len)) }
    }
}

impl AsMut<str> for CStrMut {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    fn as_mut(&mut self) -> &mut str {
        let mut buf = self.buf;
        if self.len == 0 && self.buf.is_null() {
            buf = core::ptr::NonNull::dangling().as_ptr();
        }
        assert!(!buf.is_null());
        unsafe {
            core::str::from_utf8_unchecked_mut(core::slice::from_raw_parts_mut(buf, self.len))
        }
    }
}

/// Convert buffer to slice
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf_ptr[buf_len-1]` must be a valid dereference
#[no_mangle]
pub extern "C" fn rust_util_bytes(buf: *const c_uchar, len: usize) -> Bytes {
    Bytes { buf, len }
}

/// Convert buffer to mutable slice
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf_ptr[buf_len-1]` must be a valid dereference
#[no_mangle]
pub extern "C" fn rust_util_bytes_mut(buf: *mut c_uchar, len: usize) -> BytesMut {
    BytesMut { buf, len }
}

/// Convert buffer to str
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf_ptr[buf_len-1]` must be a valid dereference
#[no_mangle]
pub extern "C" fn rust_util_cstr(buf: *const c_char, len: usize) -> CStr {
    CStr { buf, len }
}

/// Convert buffer to mutable str
///
/// * `buf` - Must be a valid pointer to an array of bytes
/// * `len` - Length of buffer, `buf_ptr[buf_len-1]` must be a valid dereference
#[no_mangle]
pub extern "C" fn rust_util_cstr_mut(buf: *mut c_char, len: usize) -> CStrMut {
    CStrMut { buf, len }
}

#[cfg(test)]
mod tests {
    extern crate std;
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
    fn u8_to_hexing() {
        let buf = [1u8, 2, 3, 14, 15, 255, 1];
        let mut string = String::from("xxxxxxxxxxxxxx");
        rust_util_uint8_to_hex(
            rust_util_bytes(buf.as_ptr(), buf.len() - 1),
            rust_util_cstr_mut(string.as_mut_ptr(), string.len() - 1),
        );
        assert_eq!(string, "0102030e0fff\0x");
    }
}
