use core::{fmt, str};
use crate::data::Type;

/// Decoding errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Decoding has (unexpectedly) reached the end of the input slice.
    EndOfInput,
    /// Data item to decode is not a valid `char`.
    InvalidChar(u32),
    /// Decoding a string failed because it is invalid UTF-8.
    Utf8(str::Utf8Error),
    /// A numeric value exceeds its value range.
    Overflow(u64, &'static str),
    /// An unexpected type was encountered.
    TypeMismatch(Type, &'static str),
    /// An unknown enum variant was encountered.
    UnknownVariant(u32),
    /// A value was missing at the specified index.
    MissingValue(u32, &'static str),
    /// Generic error message.
    Message(&'static str),
    /// Custom error.
    #[cfg(feature = "std")]
    Custom(Box<dyn std::error::Error + Send + Sync>)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EndOfInput         => f.write_str("end of input bytes"),
            Error::InvalidChar(n)     => write!(f, "invalid char: {:#x?}", n),
            Error::Utf8(e)            => write!(f, "invalid utf-8: {}", e),
            Error::Overflow(n, m)     => write!(f, "{}: {} overflows target type", m, n),
            Error::TypeMismatch(t, m) => write!(f, "unexpected type: {}, {}", t, m),
            Error::UnknownVariant(n)  => write!(f, "unknown enum variant {}", n),
            Error::MissingValue(n, s) => write!(f, "missing value at index {} for {}", n, s),
            Error::Message(m)         => write!(f, "{}", m),
            #[cfg(feature = "std")]
            Error::Custom(e)          => write!(f, "{}", e)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Custom(e) => Some(&**e),
            Error::Utf8(e) => Some(e),
            Error::EndOfInput
            | Error::InvalidChar(_)
            | Error::Overflow(..)
            | Error::TypeMismatch(..)
            | Error::UnknownVariant(_)
            | Error::MissingValue(..)
            | Error::Message(_)
            => None
        }
    }
}

impl From<str::Utf8Error> for Error {
    fn from(e: str::Utf8Error) -> Self {
        Error::Utf8(e)
    }
}

