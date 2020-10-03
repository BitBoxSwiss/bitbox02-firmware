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

/// Returns true if all bytes are in this set, including the space ` `:
///
/// ```text
/// !"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
/// ```
///
/// Unlike std's is_ascii, control characters such as newline, tab, etc. are not part
/// of the set.
pub fn is_printable_ascii<T: AsRef<[u8]>>(bytes: T) -> bool {
    bytes.as_ref().iter().all(|&b| b >= 32 && b <= 126)
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    static ALL_ASCII: &[u8] = "! \"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~".as_bytes();

    #[test]
    fn test_is_printable_ascii() {
        // All ascii chars.
        assert!(is_printable_ascii(ALL_ASCII));
        // Edge cases: highest and lowest non ascii chars.
        assert!(!is_printable_ascii(b"\x7f"));
        assert!(!is_printable_ascii(b"\x19"));
        assert!(!is_printable_ascii(b"\n"));
        assert!(!is_printable_ascii(b"\t"));
        // Works for any AsRef<[u8]>
        let trait_obj: &dyn AsRef<[u8]> = &"abc";
        assert!(is_printable_ascii(trait_obj));
    }
}
