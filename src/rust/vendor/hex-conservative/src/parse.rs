// SPDX-License-Identifier: CC0-1.0

//! Hex encoding and decoding.

use core::{fmt, str};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use crate::alloc::vec::Vec;
use crate::iter::HexToBytesIter;

/// Trait for objects that can be deserialized from hex strings.
pub trait FromHex: Sized {
    /// Error type returned while parsing hex string.
    type Err: From<HexToBytesError> + Sized + fmt::Debug + fmt::Display;

    /// Produces an object from a byte iterator.
    fn from_byte_iter<I>(iter: I) -> Result<Self, Self::Err>
    where
        I: Iterator<Item = Result<u8, HexToBytesError>> + ExactSizeIterator + DoubleEndedIterator;

    /// Produces an object from a hex string.
    fn from_hex(s: &str) -> Result<Self, Self::Err> {
        Self::from_byte_iter(HexToBytesIter::new(s)?)
    }
}

#[cfg(any(test, feature = "std", feature = "alloc"))]
impl FromHex for Vec<u8> {
    type Err = HexToBytesError;

    fn from_byte_iter<I>(iter: I) -> Result<Self, Self::Err>
    where
        I: Iterator<Item = Result<u8, HexToBytesError>> + ExactSizeIterator + DoubleEndedIterator,
    {
        iter.collect()
    }
}

/// Hex decoding error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HexToBytesError {
    /// Non-hexadecimal character.
    InvalidChar(u8),
    /// Purported hex string had odd length.
    OddLengthString(usize),
}

impl fmt::Display for HexToBytesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::HexToBytesError::*;

        match *self {
            InvalidChar(ch) => write!(f, "invalid hex character {}", ch),
            OddLengthString(ell) => write!(f, "odd hex string length {}", ell),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HexToBytesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::HexToBytesError::*;

        match self {
            InvalidChar(_) | OddLengthString(_) => None,
        }
    }
}

macro_rules! impl_fromhex_array {
    ($len:expr) => {
        impl FromHex for [u8; $len] {
            type Err = HexToArrayError;

            fn from_byte_iter<I>(iter: I) -> Result<Self, Self::Err>
            where
                I: Iterator<Item = Result<u8, HexToBytesError>>
                    + ExactSizeIterator
                    + DoubleEndedIterator,
            {
                if iter.len() == $len {
                    let mut ret = [0; $len];
                    for (n, byte) in iter.enumerate() {
                        ret[n] = byte?;
                    }
                    Ok(ret)
                } else {
                    Err(HexToArrayError::InvalidLength(2 * $len, 2 * iter.len()))
                }
            }
        }
    };
}

impl_fromhex_array!(2);
impl_fromhex_array!(4);
impl_fromhex_array!(6);
impl_fromhex_array!(8);
impl_fromhex_array!(10);
impl_fromhex_array!(12);
impl_fromhex_array!(14);
impl_fromhex_array!(16);
impl_fromhex_array!(20);
impl_fromhex_array!(24);
impl_fromhex_array!(28);
impl_fromhex_array!(32);
impl_fromhex_array!(33);
impl_fromhex_array!(64);
impl_fromhex_array!(65);
impl_fromhex_array!(128);
impl_fromhex_array!(256);
impl_fromhex_array!(384);
impl_fromhex_array!(512);

/// Hex decoding error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HexToArrayError {
    /// Conversion error while parsing hex string.
    Conversion(HexToBytesError),
    /// Tried to parse fixed-length hash from a string with the wrong length (got, want).
    InvalidLength(usize, usize),
}

impl fmt::Display for HexToArrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HexToArrayError::*;

        match *self {
            Conversion(ref e) => crate::write_err!(f, "conversion error"; e),
            InvalidLength(got, want) =>
                write!(f, "bad hex string length {} (expected {})", got, want),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HexToArrayError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use HexToArrayError::*;

        match *self {
            Conversion(ref e) => Some(e),
            InvalidLength(_, _) => None,
        }
    }
}

impl From<HexToBytesError> for HexToArrayError {
    fn from(e: HexToBytesError) -> Self { Self::Conversion(e) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::DisplayHex;

    #[test]
    #[cfg(feature = "alloc")]
    fn hex_error() {
        use HexToBytesError::*;

        let oddlen = "0123456789abcdef0";
        let badchar1 = "Z123456789abcdef";
        let badchar2 = "012Y456789abcdeb";
        let badchar3 = "«23456789abcdef";

        assert_eq!(Vec::<u8>::from_hex(oddlen), Err(OddLengthString(17)));
        assert_eq!(
            <[u8; 4]>::from_hex(oddlen),
            Err(HexToArrayError::Conversion(OddLengthString(17)))
        );
        assert_eq!(Vec::<u8>::from_hex(badchar1), Err(InvalidChar(b'Z')));
        assert_eq!(Vec::<u8>::from_hex(badchar2), Err(InvalidChar(b'Y')));
        assert_eq!(Vec::<u8>::from_hex(badchar3), Err(InvalidChar(194)));
    }

    #[test]
    fn hex_to_array() {
        let len_sixteen = "0123456789abcdef";
        assert!(<[u8; 8]>::from_hex(len_sixteen).is_ok());
    }
    #[test]
    fn hex_to_array_error() {
        use HexToArrayError::*;
        let len_sixteen = "0123456789abcdef";
        assert_eq!(<[u8; 4]>::from_hex(len_sixteen), Err(InvalidLength(8, 16)));
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
