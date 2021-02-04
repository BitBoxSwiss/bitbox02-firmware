// Copyright 2021 Shift Crypto AG
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

use alloc::string::String;
use num_bigint::BigUint;

pub struct Amount<'a> {
    pub unit: &'a str,
    pub decimals: usize,
    pub value: BigUint,
}

impl<'a> Amount<'a> {
    /// Formats the amount with the right number of decimal places, suffixed with the unit. If the
    /// value (without the unit suffix) is too long to fit on the screen, it will be truncated and
    /// ellipsis ('...')  are appended.
    ///
    /// Example:
    /// - unit: FOO,
    /// - decimals: 18,
    /// - value: 38723987932742983742983742
    /// - returns: "38723987.9327... LOL"
    pub fn format(&self) -> String {
        // Truncate the number at this many chars and append '...' if truncated.
        // Empirically found to fit on one line on the screen (including unit).
        // TODO: take into account long unit strings.
        const TRUNCATE_SIZE: usize = 13;
        let v = util::decimal::format(&self.value, self.decimals);
        if v.len() > TRUNCATE_SIZE {
            format!("{}... {}", &v[..TRUNCATE_SIZE], self.unit)
        } else {
            format!("{} {}", v, self.unit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_format() {
        struct Test<'a> {
            bigendian: &'a [u8],
            decimals: usize,
            unit: &'a str,
            expected_result: &'a str,
        };

        let tests = vec![
            Test {
                // 0
                bigendian: b"",
                decimals: 6,
                unit: "LOL",
                expected_result: "0 LOL",
            },
            Test {
                // 1000000
                bigendian: b"\x0f\x42\x40",
                decimals: 6,
                unit: "LOL",
                expected_result: "1 LOL",
            },
            Test {
                // 1100000
                bigendian: b"\x10\xc8\xe0",
                decimals: 6,
                unit: "LOL",
                expected_result: "1.1 LOL",
            },
            Test {
                // 38723987932742983742983742
                bigendian: b"\x20\x08\x1f\x97\x9a\x5c\x8d\x47\x29\x0e\x3e",
                decimals: 18,
                unit: "LOL",
                expected_result: "38723987.9327... LOL",
            },
            Test {
                // 123456
                bigendian: b"\x01\xe2\x40",
                decimals: 8,
                unit: "LOL",
                expected_result: "0.00123456 LOL",
            },
            Test {
                // 123456
                bigendian: b"\x01\xe2\x40",
                decimals: 8,
                unit: "LOL",
                expected_result: "0.00123456 LOL",
            },
            Test {
                // 124567890123
                bigendian: b"\x1d\x00\xd3\x28\xcb",
                decimals: 10,
                unit: "LOL",
                expected_result: "12.4567890123 LOL",
            },
            Test {
                // 1245678901234
                bigendian: b"\x01\x22\x08\x3f\x97\xf2",
                decimals: 11,
                unit: "LOL",
                expected_result: "12.4567890123... LOL",
            },
        ];

        for test in tests.iter() {
            assert_eq!(
                Amount {
                    unit: test.unit,
                    decimals: test.decimals,
                    value: BigUint::from_bytes_be(test.bigendian),
                }
                .format(),
                test.expected_result
            );
        }
    }
}
