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

pub const HARDENED: u32 = 0x80000000;

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub fn to_string_no_prefix(keypath: &[u32]) -> String {
    keypath
        .iter()
        .map(|&el| {
            if el >= HARDENED {
                format!("{}'", el - HARDENED)
            } else {
                format!("{}", el)
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// Turn a keypath represented as a list of u32 to a string, e.g. "m/84'/0'/0'". Hardened elements
/// are bigger or equal to `HARDENED`
pub fn to_string(keypath: &[u32]) -> String {
    format!("m/{}", to_string_no_prefix(keypath))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(to_string(&[]), "m/");
        assert_eq!(to_string(&[0]), "m/0");
        assert_eq!(to_string(&[0, 0, 0, 0]), "m/0/0/0/0");
        assert_eq!(to_string(&[HARDENED]), "m/0'");
        assert_eq!(
            to_string(&[0xFFFFFFFF, HARDENED - 1]),
            "m/2147483647'/2147483647"
        );
        assert_eq!(
            to_string(&[84 + HARDENED, 1 + HARDENED, 0 + HARDENED, 1, 100]),
            "m/84'/1'/0'/1/100"
        );
        assert_eq!(
            to_string(&[48 + HARDENED, 1 + HARDENED, 0 + HARDENED, 2 + HARDENED]),
            "m/48'/1'/0'/2'"
        );
    }
}
