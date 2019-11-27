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

use core::fmt;

pub struct Ipv4Addr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

impl From<[u8; 4]> for Ipv4Addr {
    fn from(octets: [u8; 4]) -> Self {
        Ipv4Addr {
            a: octets[0],
            b: octets[1],
            c: octets[2],
            d: octets[3],
        }
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.a, self.b, self.c, self.d)
    }
}

/// FixedCString is always null terminated when other things are written to it. Useful for communicating with C code.
pub struct FixedCString<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> FixedCString<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        FixedCString {
            buf: buf,
            offset: 0,
        }
    }
}

impl<'a> fmt::Write for FixedCString<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() + 1 {
            return Err(fmt::Error);
        }
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes.len();

        // Set last character to null
        self.buf[self.offset] = 0;

        Ok(())
    }
}

impl<'a> fmt::Debug for FixedCString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(core::str::from_utf8(&self.buf[..self.offset]).unwrap(), f)
    }
}

impl<'a> fmt::Display for FixedCString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(core::str::from_utf8(&self.buf[..self.offset]).unwrap(), f)
    }
}

///// FixedString is a fixed capacity string which is backed by a mutable [u8] slice.
///// It is useful for printing formatted strings into the buffer
///// If we had std::io in core this wouldn't have been needed bacause &mut [u8] implements
///// std::io::Write.
//pub struct FixedString<'a> {
//    buf: &'a mut [u8],
//    offset: usize,
//}
//
//impl<'a> FixedString<'a> {
//    pub fn new(buf: &'a mut [u8]) -> Self {
//        FixedString {
//            buf: buf,
//            offset: 0,
//        }
//    }
//}
//
//impl<'a> fmt::Write for FixedString<'a> {
//    fn write_str(&mut self, s: &str) -> fmt::Result {
//        let bytes = s.as_bytes();
//
//        // Skip over already-copied data
//        let remainder = &mut self.buf[self.offset..];
//        // Check if there is space remaining (return error instead of panicking)
//        if remainder.len() < bytes.len() {
//            return Err(fmt::Error);
//        }
//        // Make the two slices the same length
//        let remainder = &mut remainder[..bytes.len()];
//        // Copy
//        remainder.copy_from_slice(bytes);
//
//        // Update offset to avoid overwriting
//        self.offset += bytes.len();
//
//        Ok(())
//    }
//}
//
//impl<'a> fmt::Debug for FixedString<'a> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        fmt::Debug::fmt(core::str::from_utf8(&self.buf[..self.offset]).unwrap(), f)
//    }
//}
//
//impl<'a> fmt::Display for FixedString<'a> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        fmt::Display::fmt(core::str::from_utf8(&self.buf[..self.offset]).unwrap(), f)
//    }
//}
