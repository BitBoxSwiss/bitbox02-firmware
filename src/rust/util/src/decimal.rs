// SPDX-License-Identifier: Apache-2.0

extern crate alloc;
use alloc::string::{String, ToString};

use num_bigint::{BigInt, BigUint};

pub trait Format: ToString {
    fn decimal_string(&self) -> String {
        self.to_string()
    }
}

impl Format for u64 {}

impl Format for &BigUint {
    fn decimal_string(&self) -> String {
        biguint_to_string(self)
    }
}

pub fn biguint_to_string(value: &BigUint) -> String {
    // Scalar division avoids pulling in num-bigint's large-number radix conversion routines.
    if value == &BigUint::ZERO {
        return "0".into();
    }
    let mut value = value.clone();
    let mut digits = alloc::vec::Vec::new();
    while value != BigUint::ZERO {
        digits.push(b'0' + u8::try_from(&value % 10u32).unwrap());
        value /= 10u32;
    }
    digits.reverse();
    String::from_utf8(digits).unwrap()
}

pub fn bigint_to_string(value: &BigInt) -> String {
    let mut text = biguint_to_string(value.magnitude());
    if value.sign() == num_bigint::Sign::Minus {
        text.insert(0, '-');
    }
    text
}

/// Formats integer `value` as `value / 10^decimals`, with up to `decimals` decimal places,
/// trimming traling zeroes.
/// E.g. "123450" with decimals=3: "123.45".
/// Value must consists only of '0'-'9' digits.
pub fn format<F: Format>(value: F, decimals: usize) -> String {
    let mut v: String = format!("{:0>width$}", value.decimal_string(), width = decimals + 1);
    v.insert(v.len() - decimals, '.');
    v.trim_end_matches('0').trim_end_matches('.').into()
}

/// Formats integer `value` as `value / 10^decimals`, with up to `decimals` decimal places,
/// without trimming traling zeroes.
/// E.g. "123450" with decimals=3: "123.450".
/// Value must consists only of '0'-'9' digits.
pub fn format_no_trim<F: Format>(value: F, decimals: usize) -> String {
    let mut v: String = format!("{:0>width$}", value.decimal_string(), width = decimals + 1);
    if decimals > 0 {
        v.insert(v.len() - decimals, '.');
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biguint_to_string() {
        let two_to_256 = BigUint::from(1u32) << 256usize;
        for value in [
            BigUint::ZERO,
            BigUint::from(1u32),
            BigUint::from(9u32),
            BigUint::from(10u32),
            BigUint::from(1_020_030u32),
            BigUint::from(u64::MAX),
            &two_to_256 - 1u32,
            two_to_256,
        ] {
            assert_eq!(biguint_to_string(&value), value.to_string());
        }
    }

    #[test]
    fn test_bigint_to_string() {
        for value in [
            BigInt::from(0),
            BigInt::from(1),
            BigInt::from(-1),
            BigInt::from(i64::MIN),
            -(BigInt::from(1u32) << 255usize),
        ] {
            assert_eq!(bigint_to_string(&value), value.to_string());
        }
    }

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
