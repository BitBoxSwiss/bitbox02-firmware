use util::c_types::{c_char, c_uchar, c_void};

#[no_mangle]
pub extern "C" fn rust_util_zero(dst: *mut c_void, len: usize) -> *mut c_void {
    let slice = unsafe { core::slice::from_raw_parts_mut(dst as *mut u8, len) };
    util::zero(slice);
    dst
}

#[no_mangle]
pub extern "C" fn rust_util_uint8_to_hex(
    buf_ptr: *const c_uchar,
    buf_len: usize,
    out_buf: *mut c_char,
) {
    let buf = unsafe { core::slice::from_raw_parts(buf_ptr, buf_len) };
    let out = unsafe {
        core::str::from_utf8_unchecked_mut(core::slice::from_raw_parts_mut(out_buf, buf_len * 2))
    };
    hex::encode_to_slice(buf, unsafe { out.as_bytes_mut() }).unwrap();
    unsafe { *out_buf.offset((buf_len * 2) as isize) = b'\0' };
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn zeroing() {
        let mut buf = [1u8, 2, 3, 4];
        rust_util_zero(buf.as_mut_ptr() as _, buf.len() - 1);
        assert_eq!(&buf[..], &[0, 0, 0, 4]);
    }

    #[test]
    fn u8_to_hexing() {
        let buf = [1u8, 2, 3, 14, 15, 255, 1];
        let mut string = String::from("xxxxxxxxxxxxxx");
        rust_util_uint8_to_hex(buf.as_ptr(), buf.len() - 1, string.as_mut_ptr());
        assert_eq!(string, "0102030e0fff\0x");
    }
}
