// SPDX-License-Identifier: CC0-1.0

//! Error code for the `hex-conservative` crate.

use core::fmt;

use crate::write_err;

/// Formats error.
///
/// If `std` feature is OFF appends error source (delimited by `: `). We do this because
/// `e.source()` is only available in std builds, without this macro the error source is lost for
/// no-std builds.
#[macro_export]
macro_rules! write_err {
    ($writer:expr, $string:literal $(, $args:expr)*; $source:expr) => {
        {
            #[cfg(feature = "std")]
            {
                let _ = &$source;   // Prevents clippy warnings.
                write!($writer, $string $(, $args)*)
            }
            #[cfg(not(feature = "std"))]
            {
                write!($writer, concat!($string, ": {}") $(, $args)*, $source)
            }
        }
    }
}

/// Hex decoding error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexToBytesError {
    /// Non-hexadecimal character.
    InvalidChar(InvalidCharError),
    /// Purported hex string had odd length.
    OddLengthString(OddLengthStringError),
}

impl fmt::Display for HexToBytesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HexToBytesError::*;

        match *self {
            InvalidChar(ref e) => write_err!(f, "invalid char, failed to create bytes from hex"; e),
            OddLengthString(ref e) =>
                write_err!(f, "odd length, failed to create bytes from hex"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HexToBytesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use HexToBytesError::*;

        match *self {
            InvalidChar(ref e) => Some(e),
            OddLengthString(ref e) => Some(e),
        }
    }
}

impl From<InvalidCharError> for HexToBytesError {
    #[inline]
    fn from(e: InvalidCharError) -> Self { Self::InvalidChar(e) }
}

impl From<OddLengthStringError> for HexToBytesError {
    #[inline]
    fn from(e: OddLengthStringError) -> Self { Self::OddLengthString(e) }
}

/// Invalid hex character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidCharError {
    pub(crate) invalid: u8,
}

impl InvalidCharError {
    /// Returns the invalid character byte.
    pub fn invalid_char(&self) -> u8 { self.invalid }
}

impl fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid hex char {}", self.invalid)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidCharError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}

/// Purported hex string had odd length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OddLengthStringError {
    pub(crate) len: usize,
}

impl OddLengthStringError {
    /// Returns the odd length of the input string.
    pub fn length(&self) -> usize { self.len }
}

impl fmt::Display for OddLengthStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "odd hex string length {}", self.len)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OddLengthStringError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}

/// Hex decoding error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexToArrayError {
    /// Non-hexadecimal character.
    InvalidChar(InvalidCharError),
    /// Tried to parse fixed-length hash from a string with the wrong length.
    InvalidLength(InvalidLengthError),
}

impl fmt::Display for HexToArrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HexToArrayError::*;

        match *self {
            InvalidChar(ref e) => crate::write_err!(f, "failed to parse hex digit"; e),
            InvalidLength(ref e) => write_err!(f, "failed to parse hex"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HexToArrayError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use HexToArrayError::*;

        match *self {
            InvalidChar(ref e) => Some(e),
            InvalidLength(ref e) => Some(e),
        }
    }
}

impl From<InvalidCharError> for HexToArrayError {
    #[inline]
    fn from(e: InvalidCharError) -> Self { Self::InvalidChar(e) }
}

impl From<InvalidLengthError> for HexToArrayError {
    #[inline]
    fn from(e: InvalidLengthError) -> Self { Self::InvalidLength(e) }
}

/// Tried to parse fixed-length hash from a string with the wrong length.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct InvalidLengthError {
    /// The expected length.
    pub expected: usize,
    /// The invalid length.
    pub invalid: usize,
}

impl fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invilad hex string length {} (expected {})", self.invalid, self.expected)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidLengthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}
