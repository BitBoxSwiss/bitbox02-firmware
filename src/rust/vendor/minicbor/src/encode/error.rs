use core::fmt;

#[cfg(feature = "alloc")]
use alloc::string::ToString;

/// Encoding error.
#[derive(Debug)]
pub struct Error<E> {
    err: ErrorImpl<E>,
    #[cfg(not(feature = "alloc"))]
    msg: &'static str,
    #[cfg(feature = "alloc")]
    msg: alloc::string::String
}

impl<E> Error<E> {
    /// Construct an error with a generic message.
    ///
    /// With feature `"alloc"` any type `T: Display` is accepted which allows
    /// formatted strings. Otherwise only a `&'static str` can be used as a
    /// message.
    #[cfg(not(feature = "alloc"))]
    pub fn message(msg: &'static str) -> Self {
        Error { err: ErrorImpl::Message, msg }
    }

    /// Construct an error with a generic message.
    ///
    /// With feature `"alloc"` any type `T: Display` is accepted which allows
    /// formatted strings. Otherwise only a `&'static str` can be used as a
    /// message.
    #[cfg(feature = "alloc")]
    pub fn message<T: fmt::Display>(msg: T) -> Self {
        Error { err: ErrorImpl::Message, msg: msg.to_string() }
    }

    /// A write error happened.
    pub fn write(e: E) -> Self {
        Error { err: ErrorImpl::Write(e), msg: Default::default() }
    }

    /// A custom error.
    ///
    /// *Requires feature* `"std"`.
    #[cfg(feature = "std")]
    pub fn custom<T: std::error::Error + Send + Sync + 'static>(err: T) -> Self {
        Error { err: ErrorImpl::Custom(Box::new(err)), msg: Default::default() }
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

    pub fn is_message(&self) -> bool {
        matches!(self.err, ErrorImpl::Message)
    }

    pub fn is_write(&self) -> bool {
        matches!(self.err, ErrorImpl::Write(_))
    }

    #[cfg(feature = "std")]
    pub fn is_custom(&self) -> bool {
        matches!(self.err, ErrorImpl::Custom(_))
    }
}

/// Internal error representation.
#[derive(Debug)]
enum ErrorImpl<E> {
    /// Error writing bytes to a `Write` impl.
    Write(E),
    /// Generic error message.
    Message,
    /// Custom error.
    #[cfg(feature = "std")]
    Custom(Box<dyn std::error::Error + Send + Sync>)
}

impl<E: fmt::Display> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.err {
            ErrorImpl::Message => write!(f, "{}", self.msg),
            #[cfg(not(feature = "std"))]
            ErrorImpl::Write(e) =>
                if self.msg.is_empty() {
                    write!(f, "write error: {e}")
                } else {
                    write!(f, "write error: {e}, {}", self.msg)
                }
            #[cfg(feature = "std")]
            ErrorImpl::Write(_) =>
                if self.msg.is_empty() {
                    write!(f, "write error")
                } else {
                    write!(f, "write error: {}", self.msg)
                }
            #[cfg(feature = "std")]
            ErrorImpl::Custom(_) =>
                if self.msg.is_empty() {
                    write!(f, "encode error")
                } else {
                    write!(f, "encode error: {}", self.msg)
                }
        }
    }
}

#[cfg(feature = "std")]
impl<E: std::error::Error + 'static> std::error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.err {
            ErrorImpl::Message   => None,
            ErrorImpl::Write(e)  => Some(e),
            ErrorImpl::Custom(e) => Some(&**e)
        }
    }
}

