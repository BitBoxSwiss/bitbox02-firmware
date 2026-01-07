// SPDX-License-Identifier: CC0-1.0

//! The error types.
//!
//! These types are returned when hex decoding fails. The high-level ones are
//! [`DecodeFixedLengthBytesError`] and [`DecodeVariableLengthBytesError`] which represent all
//! possible ways in which hex decoding may fail in the two most common decoding scenarios.

use core::convert::Infallible;
use core::fmt;
#[cfg(feature = "std")]
use std::error::Error as StdError;
#[cfg(all(not(feature = "std"), feature = "newer-rust-version"))]
if_rust_version::if_rust_version! {
    >= 1.81 {
        use core::error::Error as StdError;
    }
}

#[cfg(feature = "std")]
macro_rules! if_std_error {
    ({ $($if_yes:tt)* } $(else { $($if_not:tt)* })?) => {
        #[cfg_attr(docsrs, doc(cfg(any(feature = "std", all(feature = "newer-rust-version", rust_version = ">= 1.81.0")))))]
        $($if_yes)*
    }
}

#[cfg(all(not(feature = "std"), feature = "newer-rust-version"))]
macro_rules! if_std_error {
    ({ $($if_yes:tt)* } $(else { $($if_not:tt)* })?) => {
        if_rust_version::if_rust_version! {
            >= 1.81 {
                #[cfg_attr(docsrs, doc(cfg(any(feature = "std", all(feature = "newer-rust-version", rust_version = ">= 1.81.0")))))]
                $($if_yes)*
            } $(else { $($if_not)* })?
        }
    }
}

#[cfg(all(not(feature = "std"), not(feature = "newer-rust-version")))]
macro_rules! if_std_error {
    ({ $($if_yes:tt)* } $(else { $($if_not:tt)* })?) => {
        $($($if_not)*)?
    }
}

/// Formats error.
///
/// If `std` feature is OFF appends error source (delimited by `: `). We do this because
/// `e.source()` is only available in std builds, without this macro the error source is lost for
/// no-std builds.
macro_rules! write_err {
    ($writer:expr, $string:literal $(, $args:expr)*; $source:expr) => {
        {
            if_std_error! {
                {
                    {
                        let _ = &$source;   // Prevents clippy warnings.
                        write!($writer, $string $(, $args)*)
                    }
                } else {
                    {
                        write!($writer, concat!($string, ": {}") $(, $args)*, $source)
                    }
                }
            }
        }
    }
}
pub(crate) use write_err;

/// Error returned when hex decoding a hex string with variable length.
///
/// This represents the first error encountered during decoding, however we may add other remaining
/// ones in the future.
///
/// This error differs from [`DecodeFixedLengthBytesError`] in that the number of bytes is only known
/// at run time - e.g. when decoding `Vec<u8>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeVariableLengthBytesError {
    /// Non-hexadecimal character.
    InvalidChar(InvalidCharError),
    /// Purported hex string had odd (not even) length.
    OddLengthString(OddLengthStringError),
}

impl DecodeVariableLengthBytesError {
    /// Adds `by_bytes` to all character positions stored inside.
    ///
    /// If you're parsing a larger string that consists of multiple hex sub-strings and want to
    /// return `InvalidCharError` you may need to use this function so that the callers of your
    /// parsing function can tell the exact position where decoding failed relative to the start of
    /// the string passed into your parsing function.
    ///
    /// Note that this function has the standard Rust overflow behavior because you should only
    /// ever pass in the position of the parsed hex string relative to the start of the entire
    /// input. In that case overflow is impossible.
    ///
    /// This method consumes and returns `self` so that calling it inside a closure passed into
    /// [`Result::map_err`] is convenient.
    #[must_use]
    #[inline]
    pub fn offset(self, by_bytes: usize) -> Self {
        use DecodeVariableLengthBytesError as E;

        match self {
            E::InvalidChar(e) => E::InvalidChar(e.offset(by_bytes)),
            E::OddLengthString(e) => E::OddLengthString(e),
        }
    }
}

impl From<Infallible> for DecodeVariableLengthBytesError {
    #[inline]
    fn from(never: Infallible) -> Self { match never {} }
}

impl fmt::Display for DecodeVariableLengthBytesError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodeVariableLengthBytesError as E;

        match *self {
            E::InvalidChar(ref e) => write_err!(f, "failed to decode hex"; e),
            E::OddLengthString(ref e) => write_err!(f, "failed to decode hex"; e),
        }
    }
}

if_std_error! {{
    impl StdError for DecodeVariableLengthBytesError {
        #[inline]
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            use DecodeVariableLengthBytesError as E;

            match *self {
                E::InvalidChar(ref e) => Some(e),
                E::OddLengthString(ref e) => Some(e),
            }
        }
    }
}}

impl From<InvalidCharError> for DecodeVariableLengthBytesError {
    #[inline]
    fn from(e: InvalidCharError) -> Self { Self::InvalidChar(e) }
}

impl From<OddLengthStringError> for DecodeVariableLengthBytesError {
    #[inline]
    fn from(e: OddLengthStringError) -> Self { Self::OddLengthString(e) }
}

/// Invalid hex character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidCharError {
    pub(crate) invalid: u8,
    pub(crate) pos: usize,
}

impl From<Infallible> for InvalidCharError {
    #[inline]
    fn from(never: Infallible) -> Self { match never {} }
}

impl InvalidCharError {
    /// Returns the invalid character byte.
    #[inline]
    pub(crate) fn invalid_char(&self) -> u8 { self.invalid }
    /// Returns the position of the invalid character byte.
    #[inline]
    pub fn pos(&self) -> usize { self.pos }

    /// Adds `by_bytes` to all character positions stored inside.
    ///
    /// **Important**: if you have `DecodeVariableLengthBytesError` or `DecodeFixedLengthBytesError` you
    /// should call the method *on them* - do not match them and manually call this method. Doing
    /// so may lead to broken behavior in the future.
    ///
    /// If you're parsing a larger string that consists of multiple hex sub-strings and want to
    /// return `InvalidCharError` you may need to use this function so that the callers of your
    /// parsing function can tell the exact position where decoding failed relative to the start of
    /// the string passed into your parsing function.
    ///
    /// Note that this function has the standard Rust overflow behavior because you should only
    /// ever pass in the position of the parsed hex string relative to the start of the entire
    /// input. In that case overflow is impossible.
    ///
    /// This method consumes and returns `self` so that calling it inside a closure passed into
    /// [`Result::map_err`] is convenient.
    #[must_use]
    #[inline]
    pub fn offset(mut self, by_bytes: usize) -> Self {
        self.pos += by_bytes;
        self
    }
}

/// Note that the implementation displays position as 1-based instead of 0-based to be more
/// suitable to end users who might be non-programmers.
impl fmt::Display for InvalidCharError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We're displaying this for general audience, not programmers, so we want to do 1-based
        // position but that might confuse programmers who might think it's 0-based. Hopefully
        // using more wordy approach will avoid the confusion.

        // format_args! would be simpler but we can't use it because of  Rust issue #92698.
        struct Format<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result>(F);
        impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Display for Format<F> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0(f) }
        }

        // The lifetime is not extended in MSRV, so we need this.
        let which;
        let which: &dyn fmt::Display = match self.pos() {
            0 => &"1st",
            1 => &"2nd",
            2 => &"3rd",
            pos => {
                which = Format(move |f| write!(f, "{}th", pos + 1));
                &which
            }
        };

        // The lifetime is not extended in MSRV, so we need these.
        let chr_ascii;
        let chr_non_ascii;

        let invalid_char = self.invalid_char();
        // We're currently not storing the entire character, so we need to make sure values >=
        // 128 don't get misinterpreted as ISO-8859-1.
        let chr: &dyn fmt::Display = if self.invalid_char().is_ascii() {
            // Yes, the Debug output is correct here. Display would print the characters
            // directly which would be confusing in case of control characters and it would
            // also mess up the formatting. The `Debug` implementation of `char` properly
            // escapes such characters.
            chr_ascii = Format(move |f| write!(f, "{:?}", invalid_char as char));
            &chr_ascii
        } else {
            chr_non_ascii = Format(move |f| write!(f, "{:#02x}", invalid_char));
            &chr_non_ascii
        };

        write!(f, "the {} character, {}, is not a valid hex digit", which, chr)
    }
}

if_std_error! {{
    impl StdError for InvalidCharError {}
}}

/// Purported hex string had odd length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OddLengthStringError {
    pub(crate) len: usize,
}

impl From<Infallible> for OddLengthStringError {
    #[inline]
    fn from(never: Infallible) -> Self { match never {} }
}

impl OddLengthStringError {
    /// Returns the odd length of the input string.
    #[inline]
    pub fn length(&self) -> usize { self.len }
}

impl fmt::Display for OddLengthStringError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.length() == 1 {
            write!(f, "the hex string is 1 byte long which is not an even number")
        } else {
            write!(f, "the hex string is {} bytes long which is not an even number", self.length())
        }
    }
}

if_std_error! {{
    impl StdError for OddLengthStringError {}
}}

/// Error returned when hex decoding bytes whose length is known at compile time.
///
/// This error differs from [`DecodeVariableLengthBytesError`] in that the number of bytes is known at
/// compile time - e.g. when decoding to an array of bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeFixedLengthBytesError {
    /// Non-hexadecimal character.
    InvalidChar(InvalidCharError),
    /// Tried to parse fixed-length hash from a string with the wrong length.
    InvalidLength(InvalidLengthError),
}

impl DecodeFixedLengthBytesError {
    /// Adds `by_bytes` to all character positions stored inside.
    ///
    /// If you're parsing a larger string that consists of multiple hex sub-strings and want to
    /// return `InvalidCharError` you may need to use this function so that the callers of your
    /// parsing function can tell the exact position where decoding failed relative to the start of
    /// the string passed into your parsing function.
    ///
    /// Note that this function has the standard Rust overflow behavior because you should only
    /// ever pass in the position of the parsed hex string relative to the start of the entire
    /// input. In that case overflow is impossible.
    ///
    /// This method consumes and returns `self` so that calling it inside a closure passed into
    /// [`Result::map_err`] is convenient.
    #[must_use]
    #[inline]
    pub fn offset(self, by_bytes: usize) -> Self {
        use DecodeFixedLengthBytesError as E;

        match self {
            E::InvalidChar(e) => E::InvalidChar(e.offset(by_bytes)),
            E::InvalidLength(e) => E::InvalidLength(e),
        }
    }
}

impl From<Infallible> for DecodeFixedLengthBytesError {
    #[inline]
    fn from(never: Infallible) -> Self { match never {} }
}

impl fmt::Display for DecodeFixedLengthBytesError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodeFixedLengthBytesError as E;

        match *self {
            E::InvalidChar(ref e) => write_err!(f, "failed to parse hex"; e),
            E::InvalidLength(ref e) => write_err!(f, "failed to parse hex"; e),
        }
    }
}

if_std_error! {{
    impl StdError for DecodeFixedLengthBytesError {
        #[inline]
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            use DecodeFixedLengthBytesError as E;

            match *self {
                E::InvalidChar(ref e) => Some(e),
                E::InvalidLength(ref e) => Some(e),
            }
        }
    }
}}

impl From<InvalidCharError> for DecodeFixedLengthBytesError {
    #[inline]
    fn from(e: InvalidCharError) -> Self { Self::InvalidChar(e) }
}

impl From<InvalidLengthError> for DecodeFixedLengthBytesError {
    #[inline]
    fn from(e: InvalidLengthError) -> Self { Self::InvalidLength(e) }
}

/// Tried to parse fixed-length hash from a string with the wrong length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidLengthError {
    /// The expected length.
    pub(crate) expected: usize,
    /// The invalid length.
    pub(crate) invalid: usize,
}

impl From<Infallible> for InvalidLengthError {
    #[inline]
    fn from(never: Infallible) -> Self { match never {} }
}

impl InvalidLengthError {
    /// Returns the expected length.
    ///
    /// Note that this represents both the number of bytes and the number of characters that needs
    /// to be passed into the decoder, since the hex digits are ASCII and thus always 1-byte long.
    #[inline]
    pub fn expected_length(&self) -> usize { self.expected }

    /// Returns the number of *hex bytes* passed to the hex decoder.
    ///
    /// Note that this does not imply the number of characters nor hex digits since they may be
    /// invalid (wide unicode chars).
    #[inline]
    pub fn invalid_length(&self) -> usize { self.invalid }
}

impl fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            // Note on singular vs plural: expected length is never odd, so it cannot be 1
            "the hex string is {} bytes long but exactly {} bytes were required",
            self.invalid_length(),
            self.expected_length()
        )
    }
}

if_std_error! {{
    impl StdError for InvalidLengthError {}
}}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use crate::{decode_to_array, decode_to_vec};

    fn check_source<T: std::error::Error>(error: &T) {
        assert!(error.source().is_some());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn invalid_char_error() {
        let result = decode_to_vec("12G4");
        let error = result.unwrap_err();
        if let DecodeVariableLengthBytesError::InvalidChar(e) = error {
            assert!(!format!("{}", e).is_empty());
            assert_eq!(e.invalid_char(), b'G');
            assert_eq!(e.pos(), 2);
        } else {
            panic!("Expected InvalidCharError");
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn odd_length_string_error() {
        let result = decode_to_vec("123");
        let error = result.unwrap_err();
        assert!(!format!("{}", error).is_empty());
        check_source(&error);
        if let DecodeVariableLengthBytesError::OddLengthString(e) = error {
            assert!(!format!("{}", e).is_empty());
            assert_eq!(e.length(), 3);
        } else {
            panic!("Expected OddLengthStringError");
        }
    }

    #[test]
    fn invalid_length_error() {
        let result = decode_to_array::<4>("123");
        let error = result.unwrap_err();
        assert!(!format!("{}", error).is_empty());
        check_source(&error);
        if let DecodeFixedLengthBytesError::InvalidLength(e) = error {
            assert!(!format!("{}", e).is_empty());
            assert_eq!(e.expected_length(), 8);
            assert_eq!(e.invalid_length(), 3);
        } else {
            panic!("Expected InvalidLengthError");
        }
    }

    #[test]
    fn to_bytes_error() {
        let error =
            DecodeVariableLengthBytesError::OddLengthString(OddLengthStringError { len: 7 });
        assert!(!format!("{}", error).is_empty());
        check_source(&error);
    }

    #[test]
    fn to_array_error() {
        let error = DecodeFixedLengthBytesError::InvalidLength(InvalidLengthError {
            expected: 8,
            invalid: 7,
        });
        assert!(!format!("{}", error).is_empty());
        check_source(&error);
    }
}
