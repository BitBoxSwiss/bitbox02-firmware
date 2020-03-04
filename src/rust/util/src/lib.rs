#![no_std]

pub mod c_types;

/// Guaranteed to wipe the provided buffer
pub fn zero(dst: &mut [u8]) {
    for p in dst {
        unsafe { core::ptr::write_volatile(p, 0) };
    }
}

/// Converts binary data to hex representation. Requires that resulting string is `buf.len() * 2`.
pub fn u8_to_hex(buf: &[u8], res: &mut str) {
    const DIGITS: &'static [u8; 16] = b"0123456789abcdef";
    if buf.len() * 2 != res.len() {
        panic!("Invalid lengths {} {}", buf.len(), res.len());
    }
    // Unsafe notice
    // The code below will only write ascii characters to the string. Unsafe is needed to get raw
    // access to the string bytes.
    unsafe {
        let res_bytes = res.as_bytes_mut();
        for i in 0..buf.len() {
            let c = buf[i];
            res_bytes[i * 2] = DIGITS[(c >> 4 & 0xF) as usize];
            res_bytes[i * 2 + 1] = DIGITS[(c & 0xF) as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::prelude::v1::*;
    #[test]
    fn zeroing() {
        let mut buf = [1u8, 2, 3];
        assert_ne!(&buf[..], &[0, 0, 0]);
        zero(&mut buf[..]);
        assert_eq!(&buf[..], &[0, 0, 0]);
    }

    #[test]
    fn zeroing2() {
        let mut buf = [1u8, 2, 3];
        zero(&mut buf[0..1]);
        assert_eq!(&buf[..], &[0, 2, 3]);
    }

    #[test]
    fn zeroing3() {
        let mut buf = [1u8, 2, 3];
        zero(&mut buf[1..2]);
        assert_eq!(&buf[..], &[1, 0, 3]);
    }

    #[test]
    fn u8_to_hexing() {
        let buf = [1u8, 2, 3, 14, 15, 16];
        let mut tmp = [0u8; 12];
        let string = unsafe { core::str::from_utf8_unchecked_mut(&mut tmp) };
        u8_to_hex(&buf[..], string);
        assert_eq!(string, "0102030e0f10");
    }

    #[test]
    fn u8_to_hexing2() {
        let buf = [1u8, 2, 3, 14, 15, 16];
        let mut string = String::from("xxxxxxxxxxxxxx");
        u8_to_hex(&buf[..], &mut string[1..13]);
        assert_eq!(string, "x0102030e0f10x");
    }
    #[test]
    fn u8_to_hexing3() {
        let buf = [1u8, 2, 3, 14, 15, 255];
        let mut string = String::from("xxxxxxxxxxxxxx");
        u8_to_hex(&buf[..], &mut string[1..13]);
        assert_eq!(string, "x0102030e0fffx");
    }
}
