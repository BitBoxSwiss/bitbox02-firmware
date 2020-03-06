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

#[no_mangle]
pub extern "C" fn rust_util_all_ascii_bytes(bytes: Bytes) -> bool {
    util::ascii::all_ascii(bytes)
}

#[no_mangle]
pub extern "C" fn rust_util_all_ascii(cstr: CStr) -> bool {
    let s: &str = cstr.as_ref();
    util::ascii::all_ascii(s)
}

/// Convert bytes to hex representation
///
/// * `buf` - bytes to convert to hex.
/// * `out` - hex will be written here. out len must be at least 2*buf.len+1.
#[no_mangle]
pub extern "C" fn rust_util_uint8_to_hex(buf: Bytes, mut out: CStrMut) {
    // UNSAFE: We promise that we null terminate the string.
    let out = unsafe { out.as_bytes_mut() };
    let min_len = buf.len * 2 + 1;
    if out.len() < min_len {
        panic!("rust_util_uint8_to_hex: out buffer too small");
    }
    hex::encode_to_slice(&buf, &mut out[0..min_len - 1]).unwrap();
    out[min_len - 1] = b'\0';
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

        // Find null terminator
        let mut len = 0;
        unsafe {
            let mut b = buf;
            while b.read() != 0 {
                b = b.offset(1);
                len += 1;
                if len == self.len {
                    panic!("CStrMut not null terminated");
                }
            }
        }

        unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, len)) }
    }
}

impl CStrMut {
    /// Create a slice to a buffer. Only allowed for non-null pointers with length or null pointers
    /// with 0 length due to limitation in `core::slice`.
    /// The caller must ensure that the string is null terminated.
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let mut buf = self.buf;
        if self.len == 0 && self.buf.is_null() {
            buf = core::ptr::NonNull::dangling().as_ptr();
        }
        assert!(!buf.is_null());
        core::slice::from_raw_parts_mut(buf, self.len)
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

/// Convert buffer to str.
///
/// * `buf` - Must be a valid pointer to a null terminated array of bytes.
#[no_mangle]
pub extern "C" fn rust_util_cstr(buf: *const c_char) -> CStr {
    assert!(!buf.is_null());
    let mut len = 0;
    let mut b = buf;
    unsafe {
        while b.read() != 0 {
            len += 1;
            b = b.offset(1);
        }
    }
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
    fn test_rust_util_cstr() {
        let cstr = rust_util_cstr(b"\0".as_ptr());
        assert_eq!(cstr.as_ref(), "");
        assert_eq!(cstr.len, 0);

        let cstr = rust_util_cstr(b"foo\0bar".as_ptr());
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
        let mut cstr_mut = rust_util_cstr_mut(start.as_mut_ptr(), start.len());
        assert_eq!(cstr_mut.len, start.len());
        assert_eq!(cstr_mut.as_ref(), "foo");
        let bytes_mut = unsafe { cstr_mut.as_bytes_mut() };
        bytes_mut[0] = b'g';
        assert_eq!(cstr_mut.as_ref(), "goo");
    }

    #[test]
    #[should_panic]
    fn test_invalid_cstr_mut() {
        let mut buf = [1, 2, 3];
        let cstr_mut = rust_util_cstr_mut(buf.as_mut_ptr(), buf.len());
        // panics as there is no null terminator.
        cstr_mut.as_ref();
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
        let mut string = String::from("xxxxxxxxxxxxxx");
        rust_util_uint8_to_hex(
            rust_util_bytes(buf.as_ptr(), buf.len() - 1),
            rust_util_cstr_mut(string.as_mut_ptr(), string.len() - 1),
        );
        assert_eq!(string, "0102030e0fff\0x");

        // Bigger buffer also works.
        let mut string = String::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        rust_util_uint8_to_hex(
            rust_util_bytes(buf.as_ptr(), buf.len() - 1),
            rust_util_cstr_mut(string.as_mut_ptr(), string.len()),
        );
        assert_eq!(string, "0102030e0fff\0xxxxxxxxxxxxxxxxxxxxxxx");
    }
}
