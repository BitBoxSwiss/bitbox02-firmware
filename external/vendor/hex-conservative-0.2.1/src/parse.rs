// SPDX-License-Identifier: CC0-1.0

//! Hex encoding and decoding.

use core::{fmt, str};

use arrayvec::ArrayVec;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use crate::alloc::vec::Vec;
use crate::error::InvalidLengthError;
use crate::iter::HexToBytesIter;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::error::{HexToBytesError, HexToArrayError};

/// Trait for objects that can be deserialized from hex strings.
pub trait FromHex: Sized {
    /// Error type returned while parsing hex string.
    type Error: Sized + fmt::Debug + fmt::Display;

    /// Produces an object from a hex string.
    fn from_hex(s: &str) -> Result<Self, Self::Error>;
}

#[cfg(any(test, feature = "std", feature = "alloc"))]
impl FromHex for Vec<u8> {
    type Error = HexToBytesError;

    fn from_hex(s: &str) -> Result<Self, Self::Error> {
        HexToBytesIter::new(s)?.map(|result| result.map_err(Into::into)).collect()
    }
}

impl<const LEN: usize> FromHex for [u8; LEN] {
    type Error = HexToArrayError;

    fn from_hex(s: &str) -> Result<Self, Self::Error> {
        if s.len() == LEN * 2 {
            let mut ret = ArrayVec::<u8, LEN>::new();
            // checked above
            for byte in HexToBytesIter::new_unchecked(s) {
                ret.push(byte?);
            }
            Ok(ret.into_inner().expect("inner is full"))
        } else {
            Err(InvalidLengthError { invalid: s.len(), expected: 2 * LEN }.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::DisplayHex;

    #[test]
    #[cfg(feature = "alloc")]
    fn hex_error() {
        use crate::error::{InvalidCharError, OddLengthStringError};

        let oddlen = "0123456789abcdef0";
        let badchar1 = "Z123456789abcdef";
        let badchar2 = "012Y456789abcdeb";
        let badchar3 = "«23456789abcdef";

        assert_eq!(Vec::<u8>::from_hex(oddlen), Err(OddLengthStringError { len: 17 }.into()));
        assert_eq!(
            <[u8; 4]>::from_hex(oddlen),
            Err(InvalidLengthError { invalid: 17, expected: 8 }.into())
        );
        assert_eq!(Vec::<u8>::from_hex(badchar1), Err(InvalidCharError { invalid: b'Z' }.into()));
        assert_eq!(Vec::<u8>::from_hex(badchar2), Err(InvalidCharError { invalid: b'Y' }.into()));
        assert_eq!(Vec::<u8>::from_hex(badchar3), Err(InvalidCharError { invalid: 194 }.into()));
    }

    #[test]
    fn hex_to_array() {
        let len_sixteen = "0123456789abcdef";
        assert!(<[u8; 8]>::from_hex(len_sixteen).is_ok());
    }

    #[test]
    fn hex_to_array_error() {
        let len_sixteen = "0123456789abcdef";
        assert_eq!(
            <[u8; 4]>::from_hex(len_sixteen),
            Err(InvalidLengthError { invalid: 16, expected: 8 }.into())
        )
    }

    #[test]
    fn mixed_case() {
        let s = "DEADbeef0123";
        let want_lower = "deadbeef0123";
        let want_upper = "DEADBEEF0123";

        let v = Vec::<u8>::from_hex(s).expect("valid hex");
        assert_eq!(format!("{:x}", v.as_hex()), want_lower);
        assert_eq!(format!("{:X}", v.as_hex()), want_upper);
    }
}
