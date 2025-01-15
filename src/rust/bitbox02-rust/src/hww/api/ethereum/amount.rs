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
use num_traits::{ToPrimitive, Zero};

pub struct Amount<'a> {
    pub unit: &'a str,
    pub decimals: usize,
    pub value: BigUint,
}

impl Amount<'_> {
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

/// Computes the percentage of the fee of the amount, up to one decimal point.
/// Returns None if the amount is 0 or either fee or amount cannot be represented by `f64`.
pub fn calculate_percentage(fee: &BigUint, amount: &BigUint) -> Option<f64> {
    if amount.is_zero() {
        return None;
    }
    Some(100. * fee.to_f64()? / amount.to_f64()?)
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
        }

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
            Test {
                // 123456
                bigendian: b"\x01\xe2\x40",
                decimals: 0,
                unit: "LOL",
                expected_result: "123456 LOL",
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

    #[test]
    pub fn test_calculate_percentage() {
        let p = |f: u64, a: u64| calculate_percentage(&f.into(), &a.into());
        assert_eq!(p(1, 0), None);
        assert_eq!(p(3, 4), Some(75.));
        assert_eq!(p(0, 100), Some(0.));
        assert_eq!(p(1, 100), Some(1.));
        assert_eq!(p(9, 100), Some(9.));
        assert_eq!(p(10, 100), Some(10.));
        assert_eq!(p(99, 100), Some(99.));
        assert_eq!(p(909, 1000), Some(90.9));
        assert_eq!(
            calculate_percentage(
                // 63713280000000000
                &BigUint::from_bytes_be(b"\xe2\x5a\xe3\xfd\xe0\x00\x00"),
                // 530564000000000000
                &BigUint::from_bytes_be(b"\x07\x5c\xf1\x25\x9e\x9c\x40\x00"),
            ),
            Some(12.008594627603833)
        );
    }
}
