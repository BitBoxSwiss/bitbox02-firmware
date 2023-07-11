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

extern crate alloc;
use alloc::string::{String, ToString};

use num_bigint::BigUint;

pub trait Format: ToString {}

impl Format for u64 {}

impl Format for &BigUint {}

/// Formats integer `value` as `value / 10^decimals`, with up to `decimals` decimal places,
/// trimming traling zeroes.
/// E.g. "123450" with decimals=3: "123.45".
/// Value must consists only of '0'-'9' digits.
pub fn format<F: Format>(value: F, decimals: usize) -> String {
    let mut v: String = format!("{:0>width$}", value.to_string(), width = decimals + 1);
    v.insert(v.len() - decimals, '.');
    v.trim_end_matches('0').trim_end_matches('.').into()
}

/// Formats integer `value` as `value / 10^decimals`, with up to `decimals` decimal places,
/// without trimming traling zeroes.
/// E.g. "123450" with decimals=3: "123.450".
/// Value must consists only of '0'-'9' digits.
pub fn format_no_trim<F: Format>(value: F, decimals: usize) -> String {
    let mut v: String = format!("{:0>width$}", value.to_string(), width = decimals + 1);
    if decimals > 0 {
        v.insert(v.len() - decimals, '.');
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_int() {
        assert_eq!(format(0u64, 0), "0");
        assert_eq!(format(0u64, 2), "0");
        assert_eq!(format(0u64, 6), "0");

        assert_eq!(format(1u64, 0), "1");
        assert_eq!(format(1u64, 1), "0.1");
        assert_eq!(format(1u64, 2), "0.01");
        assert_eq!(format(1u64, 6), "0.000001");

        assert_eq!(format(12345u64, 0), "12345");
        assert_eq!(format(12345u64, 1), "1234.5");
        assert_eq!(format(12345u64, 2), "123.45");
        assert_eq!(format(12345u64, 3), "12.345");
        assert_eq!(format(12345u64, 4), "1.2345");
        assert_eq!(format(12345u64, 5), "0.12345");
        assert_eq!(format(12345u64, 6), "0.012345");
        assert_eq!(format(12345u64, 7), "0.0012345");
        assert_eq!(format(123450u64, 7), "0.012345");
        assert_eq!(format(1234500u64, 7), "0.12345");
    }

    #[test]
    fn test_format_int_no_trim() {
        assert_eq!(format_no_trim(0u64, 0), "0");
        assert_eq!(format_no_trim(0u64, 2), "0.00");
        assert_eq!(format_no_trim(0u64, 6), "0.000000");

        assert_eq!(format_no_trim(1u64, 0), "1");
        assert_eq!(format_no_trim(1u64, 1), "0.1");
        assert_eq!(format_no_trim(1u64, 2), "0.01");
        assert_eq!(format_no_trim(1u64, 6), "0.000001");

        assert_eq!(format_no_trim(12345u64, 0), "12345");
        assert_eq!(format_no_trim(12345u64, 1), "1234.5");
        assert_eq!(format_no_trim(12345u64, 2), "123.45");
        assert_eq!(format_no_trim(12345u64, 3), "12.345");
        assert_eq!(format_no_trim(12345u64, 4), "1.2345");
        assert_eq!(format_no_trim(12345u64, 5), "0.12345");
        assert_eq!(format_no_trim(12345u64, 6), "0.012345");
        assert_eq!(format_no_trim(12345u64, 7), "0.0012345");
        assert_eq!(format_no_trim(123450u64, 7), "0.0123450");
        assert_eq!(format_no_trim(1234500u64, 7), "0.1234500");
    }
}
