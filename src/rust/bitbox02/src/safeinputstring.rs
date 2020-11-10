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

extern crate alloc;
use alloc::boxed::Box;

const INPUT_STRING_MAX_SIZE: usize = bitbox02_sys::INPUT_STRING_MAX_SIZE as _;

/// C-style including null terminator. Does *not* implement Copy, so
/// that we can have a Drop to zero the contents.
// TODO: use a reusable zero-on-drop buffer type
pub struct SafeInputString(Box<[u8; INPUT_STRING_MAX_SIZE]>);

impl SafeInputString {
    /// Makes a SafeInputString buffer filled with 0.
    pub fn new() -> SafeInputString {
        SafeInputString(Box::new([0; INPUT_STRING_MAX_SIZE]))
    }

    /// Copies the string bytes from `source` without additional allocations.
    pub fn copy_from(&mut self, source: &Self) {
        self.0.copy_from_slice(&source.0[..]);
    }

    /// Returns the underlying C string buffer (null terminated), to be used in C function calls.
    pub fn as_cstr(&self) -> *const util::c_types::c_char {
        &*self.0 as *const _
    }

    /// Returns the buffer size (including null terminator).
    pub fn cap(&self) -> usize {
        self.0.len()
    }

    /// Returns a &str instance for use in Rust. panics if the
    /// string is not valid UTF-8 or not null terminated.
    pub fn as_str(&self) -> &str {
        let len = self.0.iter().position(|&x| x == 0).unwrap();
        core::str::from_utf8(&self.0[..len]).unwrap()
    }

    /// Zeroes the whole string buffer.
    pub fn clear(&mut self) {
        util::zero(&mut self.0[..]);
    }
}

impl AsMut<[u8; INPUT_STRING_MAX_SIZE]> for SafeInputString {
    fn as_mut(&mut self) -> &mut [u8; INPUT_STRING_MAX_SIZE] {
        &mut self.0
    }
}

impl Drop for SafeInputString {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;

    fn from(buf: &[u8]) -> SafeInputString {
        let mut pw = SafeInputString::new();
        pw.as_mut()[..buf.len()].copy_from_slice(&buf);
        pw
    }

    #[test]
    fn test_copy_from() {
        let mut pw = SafeInputString::new();
        pw.copy_from(&SafeInputString::new());
        assert_eq!(pw.as_str(), "");

        pw.copy_from(&from(b"foo bar\0"));
        assert_eq!(pw.as_str(), "foo bar");
    }

    #[test]
    fn test_as_str() {
        assert_eq!(SafeInputString::new().as_str(), "");

        assert_eq!(from(b"ab\0").as_str(), "ab");
        assert_eq!(from(b"foo test").as_str(), "foo test");
    }

    #[test]
    fn test_clear() {
        let mut pw = from(b"non zero\0");
        pw.clear();
        assert_eq!(pw.as_str(), "");
    }
}
