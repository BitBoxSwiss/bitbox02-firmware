use core::{fmt, str};
use crate::data::{Tag, Type};

#[cfg(feature = "alloc")]
use alloc::string::ToString;

/// Decoding error.
#[derive(Debug)]
pub struct Error {
    err: ErrorImpl,
    pos: Option<usize>,
    #[cfg(not(feature = "alloc"))]
    msg: &'static str,
    #[cfg(feature = "alloc")]
    msg: alloc::string::String
}

impl Error {
    /// The end of the input bytes has been reached.
    pub fn end_of_input() -> Self {
        Error {
            err: ErrorImpl::EndOfInput,
            pos: None,
            msg: Default::default()
        }
    }

    /// A type error.
    pub fn type_mismatch(ty: Type) -> Self {
        Error {
            err: ErrorImpl::TypeMismatch(ty),
            pos: None,
            msg: Default::default()
        }
    }

    /// A tag error.
    pub fn tag_mismatch(tg: Tag) -> Self {
        Error {
            err: ErrorImpl::TagMismatch(tg),
            pos: None,
            msg: Default::default()
        }
    }

    /// Construct an error with a generic message.
    ///
    /// With feature `"alloc"` any type `T: Display` is accepted which allows
    /// formatted strings. Otherwise only a `&'static str` can be used as a
    /// message.
    #[cfg(not(feature = "alloc"))]
    pub fn message(msg: &'static str) -> Self {
        Error {
            err: ErrorImpl::Message,
            pos: None,
            msg
        }
    }

    /// Construct an error with a generic message.
    ///
    /// With feature `"alloc"` any type `T: Display` is accepted which allows
    /// formatted strings. Otherwise only a `&'static str` can be used as a
    /// message.
    #[cfg(feature = "alloc")]
    pub fn message<T: fmt::Display>(msg: T) -> Self {
        Error {
            err: ErrorImpl::Message,
            pos: None,
            msg: msg.to_string()
        }
    }

    /// A custom error.
    ///
    /// *Requires feature* `"std"`.
    #[cfg(feature = "std")]
    pub fn custom<T: std::error::Error + Send + Sync + 'static>(err: T) -> Self {
        Error {
            err: ErrorImpl::Custom(Box::new(err)),
            pos: None,
            msg: Default::default()
        }
    }

    /// An unknown enum variant (denoted by the given index) was encountered.
    #[doc(hidden)]
    pub fn unknown_variant(idx: u32) -> Self {
        Error {
            err: ErrorImpl::UnknownVariant(idx),
            pos: None,
            msg: Default::default()
        }
    }

    /// A value, expected at the given index, was missing.
    #[doc(hidden)]
    pub fn missing_value(idx: u32) -> Self {
        Error {
            err: ErrorImpl::MissingValue(idx),
            pos: None,
            msg: Default::default()
        }
    }

    pub(crate) fn invalid_char(item: u32) -> Self {
        Error {
            err: ErrorImpl::InvalidChar(item),
            pos: None,
            msg: Default::default()
        }
    }

    pub(crate) fn utf8(err: str::Utf8Error) -> Self {
        Error {
            err: ErrorImpl::Utf8(err),
            pos: None,
            msg: Default::default()
        }
    }

    pub(crate) fn overflow(item: u64) -> Self {
        Error {
            err: ErrorImpl::Overflow(item),
            pos: None,
            msg: Default::default()
        }
    }

    /// Set the decoding position where the error happened.
    pub fn at(mut self, pos: usize) -> Self {
        self.pos = Some(pos);
        self
    }

    /// Add a message to this error value.
    ///
    /// With feature `"alloc"` any type `T: Display` is accepted which allows
    /// formatted strings. Otherwise only a `&'static str` can be used as a
    /// message.
    #[cfg(not(feature = "alloc"))]
    pub fn with_message(mut self, msg: &'static str) -> Self {
        self.msg = msg;
        self
    }

    /// Add a message to this error value.
    ///
    /// With feature `"alloc"` any type `T: Display` is accepted which allows
    /// formatted strings. Otherwise only a `&'static str` can be used as a
    /// message.
    #[cfg(feature = "alloc")]
    pub fn with_message<T: fmt::Display>(mut self, msg: T) -> Self {
        self.msg = msg.to_string();
        self
    }

    pub fn is_end_of_input(&self) -> bool {
        matches!(self.err, ErrorImpl::EndOfInput)
    }

    pub fn is_type_mismatch(&self) -> bool {
        matches!(self.err, ErrorImpl::TypeMismatch(_))
    }

    pub fn is_tag_mismatch(&self) -> bool {
        matches!(self.err, ErrorImpl::TagMismatch(_))
    }

    pub fn is_message(&self) -> bool {
        matches!(self.err, ErrorImpl::Message)
    }

    #[cfg(feature = "std")]
    pub fn is_custom(&self) -> bool {
        matches!(self.err, ErrorImpl::Custom(_))
    }

    #[doc(hidden)]
    pub fn is_unknown_variant(&self) -> bool {
        matches!(self.err, ErrorImpl::UnknownVariant(_))
    }

    #[doc(hidden)]
    pub fn is_missing_value(&self) -> bool {
        matches!(self.err, ErrorImpl::MissingValue(_))
    }
}

/// Internal error representation.
#[derive(Debug)]
enum ErrorImpl {
    /// Decoding has (unexpectedly) reached the end of the input slice.
    EndOfInput,
    /// Data item to decode is not a valid `char`.
    InvalidChar(u32),
    /// Decoding a string failed because it is invalid UTF-8.
    Utf8(str::Utf8Error),
    /// A numeric value exceeds its value range.
    Overflow(u64),
    /// An unexpected type was encountered.
    TypeMismatch(Type),
    /// An unexpected tag was encountered.
    TagMismatch(Tag),
    /// An unknown enum variant was encountered.
    UnknownVariant(u32),
    /// A value was missing at the specified index.
    MissingValue(u32),
    /// Generic error message.
    Message,
    /// Custom error.
    #[cfg(feature = "std")]
    Custom(Box<dyn std::error::Error + Send + Sync>)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.err {
            ErrorImpl::EndOfInput =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "end of input bytes"),
                    ("", Some(p)) => write!(f, "end of input bytes at position {p}"),
                    (m, None)     => write!(f, "end of input bytes: {m}"),
                    (m, Some(p))  => write!(f, "end of input bytes at position {p}: {m}")
                }
            ErrorImpl::InvalidChar(n) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "invalid char {n:#x?}"),
                    ("", Some(p)) => write!(f, "invalid char {n:#x?} at position {p}"),
                    (m, None)     => write!(f, "invalid char {n:#x?}: {m}"),
                    (m, Some(p))  => write!(f, "invalid char {n:#x?} at position {p}: {m}")
                }
            #[cfg(not(feature = "std"))]
            ErrorImpl::Utf8(e) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "invalid utf-8: {e}"),
                    ("", Some(p)) => write!(f, "invalid utf-8 at position {p}: {e}"),
                    (m, None)     => write!(f, "invalid utf-8: {e}, {m}"),
                    (m, Some(p))  => write!(f, "invalid utf-8 at position {p}: {e}, {m}")
                }
            #[cfg(feature = "std")]
            ErrorImpl::Utf8(_) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "invalid utf-8"),
                    ("", Some(p)) => write!(f, "invalid utf-8 at position {p}"),
                    (m, None)     => write!(f, "invalid utf-8: {m}"),
                    (m, Some(p))  => write!(f, "invalid utf-8 at position {p}: {m}")
                }
            ErrorImpl::Overflow(n) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "{n} overflows target type"),
                    ("", Some(p)) => write!(f, "{n} overflows target type at position {p}"),
                    (m, None)     => write!(f, "{n} overflows target type: {m}"),
                    (m, Some(p))  => write!(f, "{n} overflows target type at position {p}: {m}")
                }
            ErrorImpl::TypeMismatch(t) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "unexpected type {t}"),
                    ("", Some(p)) => write!(f, "unexpected type {t} at position {p}"),
                    (m, None)     => write!(f, "unexpected type {t}: {m}"),
                    (m, Some(p))  => write!(f, "unexpected type {t} at position {p}: {m}")
                }
            ErrorImpl::TagMismatch(t) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "unexpected tag {t}"),
                    ("", Some(p)) => write!(f, "unexpected tag {t} at position {p}"),
                    (m, None)     => write!(f, "unexpected tag {t}: {m}"),
                    (m, Some(p))  => write!(f, "unexpected tag {t} at position {p}: {m}")
                }
            ErrorImpl::UnknownVariant(n) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "unknown enum variant {n}"),
                    ("", Some(p)) => write!(f, "unknown enum variant {n} at position {p}"),
                    (m, None)     => write!(f, "unknown enum variant {n}: {m}"),
                    (m, Some(p))  => write!(f, "unknown enum variant {n} at position {p}: {m}")
                }
            ErrorImpl::MissingValue(n) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "missing value at index {n}"),
                    ("", Some(p)) => write!(f, "missing value at index {n} in map or array starting at position {p}"),
                    (m, None)     => write!(f, "missing value at index {n} ({m})"),
                    (m, Some(p))  => write!(f, "missing value at index {n} ({m}) in map or array starting at position {p}")
                }
            ErrorImpl::Message =>
                if let Some(p) = self.pos {
                    write!(f, "decode error at position {p}: {}", self.msg)
                } else {
                    write!(f, "decode error: {}", self.msg)
                }
            #[cfg(feature = "std")]
            ErrorImpl::Custom(_) =>
                match (self.msg.as_ref(), self.pos) {
                    ("", None)    => write!(f, "decode error"),
                    ("", Some(p)) => write!(f, "decode error at position {p}"),
                    (m, None)     => write!(f, "decode error: {m}"),
                    (m, Some(p))  => write!(f, "decode error at position {p}: {m}")
                }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.err {
            ErrorImpl::EndOfInput
            | ErrorImpl::InvalidChar(_)
            | ErrorImpl::Overflow(_)
            | ErrorImpl::TypeMismatch(_)
            | ErrorImpl::TagMismatch(_)
            | ErrorImpl::UnknownVariant(_)
            | ErrorImpl::MissingValue(_)
            | ErrorImpl::Message
            => None,
            ErrorImpl::Utf8(e)   => Some(e),
            ErrorImpl::Custom(e) => Some(&**e)
        }
    }
}
