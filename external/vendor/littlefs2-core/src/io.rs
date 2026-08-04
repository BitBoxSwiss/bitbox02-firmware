//! Traits and types for core I/O functionality.

use core::{
    ffi::c_int,
    fmt::{self, Debug, Formatter},
};

/// The `Read` trait allows for reading bytes from a file.
pub trait Read {
    /// Read at most buf.len() bytes.
    /// Upon success, return how many bytes were read.
    fn read(&self, buf: &mut [u8]) -> Result<usize>;

    fn read_exact(&self, buf: &mut [u8]) -> Result<()> {
        // Same assumption as for `read_to_end`.
        let len = self.read(buf)?;
        if len == buf.len() {
            Ok(())
        } else {
            // TODO: Decide whether to add an equivalent of `ErrorKind::UnexpectedEof`
            Err(Error::IO)
        }
    }
}

/** The `Write` trait allows for writing bytes to a file.

By analogy with `std::io::Write`, we also define a `flush()`
method. In the current implementation, writes are final and
flush has no effect.
*/
pub trait Write {
    /// Write at most data.len() bytes.
    /// The file will not necessarily be updated unless
    /// flush is called as there is a cache.
    /// Upon success, return how many bytes were written.
    fn write(&self, data: &[u8]) -> Result<usize>;

    /// Write out all pending writes to storage.
    fn flush(&self) -> Result<()>;

    fn write_all(&self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    // failed to write whole buffer
                    return Err(Error::IO);
                }
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/** Enumeration of possible methods to seek within an I/O object.

Use the [`Seek`](../io/trait.Seek.html) trait.
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeekFrom {
    Start(u32),
    End(i32),
    Current(i32),
}

impl SeekFrom {
    pub fn off(self) -> i32 {
        match self {
            SeekFrom::Start(u) => u as i32,
            SeekFrom::End(i) => i,
            SeekFrom::Current(i) => i,
        }
    }

    pub fn whence(self) -> c_int {
        match self {
            SeekFrom::Start(_) => 0,
            SeekFrom::End(_) => 2,
            SeekFrom::Current(_) => 1,
        }
    }
}

/// Enumeration of possible methods to seek within an file that was just opened
/// Used in the `read_chunk` and `write_chunk` methods,
/// Where [`SeekFrom::Current`] would not make sense.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenSeekFrom {
    Start(u32),
    End(i32),
}

impl From<OpenSeekFrom> for SeekFrom {
    fn from(value: OpenSeekFrom) -> Self {
        match value {
            OpenSeekFrom::Start(o) => Self::Start(o),
            OpenSeekFrom::End(o) => Self::End(o),
        }
    }
}

/** The `Seek` trait provides a cursor which can be moved within a file.

It is possible to seek relative to either end or the current offset.
*/
pub trait Seek {
    /// Seek to an offset in bytes.
    /// If successful, returns the new position from start of file.
    fn seek(&self, pos: SeekFrom) -> Result<usize>;
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

/// The error type for filesystem operations.
///
/// Specific error codes are available as associated constants of this type.
///
/// ```
/// # use littlefs2_core::Error;
/// assert_eq!(Error::IO.code(), -5);
/// assert_eq!(Error::new(-5), Some(Error::IO));
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Error {
    code: c_int,
}

impl Error {
    /// Input / output error occurred.
    pub const IO: Self = Self::new_const(-5);

    /// File or filesystem was corrupt.
    pub const CORRUPTION: Self = Self::new_const(-84);

    /// No entry found with that name.
    pub const NO_SUCH_ENTRY: Self = Self::new_const(-2);

    /// File or directory already exists.
    pub const ENTRY_ALREADY_EXISTED: Self = Self::new_const(-17);

    /// Path name is not a directory.
    pub const PATH_NOT_DIR: Self = Self::new_const(-20);

    /// Path specification is to a directory.
    pub const PATH_IS_DIR: Self = Self::new_const(-21);

    /// Directory was not empty.
    pub const DIR_NOT_EMPTY: Self = Self::new_const(-39);

    /// Bad file descriptor.
    pub const BAD_FILE_DESCRIPTOR: Self = Self::new_const(-9);

    /// File is too big.
    pub const FILE_TOO_BIG: Self = Self::new_const(-27);

    /// Incorrect value specified to function.
    pub const INVALID: Self = Self::new_const(-22);

    /// No space left available for operation.
    pub const NO_SPACE: Self = Self::new_const(-28);

    /// No memory available for completing request.
    pub const NO_MEMORY: Self = Self::new_const(-12);

    /// No attribute or data available
    pub const NO_ATTRIBUTE: Self = Self::new_const(-61);

    /// Filename too long
    pub const FILENAME_TOO_LONG: Self = Self::new_const(-36);

    /// Construct an `Error` from an error code.
    ///
    /// Return values that are greater or equals to zero represent success.  In this case, `None`
    /// is returned.
    pub const fn new(code: c_int) -> Option<Self> {
        if code >= 0 {
            None
        } else {
            Some(Self { code })
        }
    }

    const fn new_const(code: c_int) -> Self {
        if code >= 0 {
            panic!("error code must be negative");
        }
        Self { code }
    }

    /// Return the error code of this error.
    pub const fn code(&self) -> c_int {
        self.code
    }
}

/// Prints a static string as the debug representation.
///
/// If unwrap or expect is used on a `Result<_, Error>`, the `Debug` implementation is not
/// always optimized out.  This leads to a significant increase of the binary size.
/// As a short-term fix, the `Debug` implementation currently always returns a static string.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #[cfg(not(feature = "debug-error"))]
        {
            f.debug_struct("Error").finish()
        }
        #[cfg(feature = "debug-error")]
        {
            match self {
                &Self::IO => f.write_str("IO"),
                &Self::CORRUPTION => f.write_str("CORRUPTION"),
                &Self::NO_SUCH_ENTRY => f.write_str("NO_SUCH_ENTRY"),
                &Self::ENTRY_ALREADY_EXISTED => f.write_str("ENTRY_ALREADY_EXISTED"),
                &Self::PATH_NOT_DIR => f.write_str("PATH_NOT_DIR"),
                &Self::PATH_IS_DIR => f.write_str("PATH_IS_DIR"),
                &Self::DIR_NOT_EMPTY => f.write_str("DIR_NOT_EMPTY"),
                &Self::BAD_FILE_DESCRIPTOR => f.write_str("BAD_FILE_DESCRIPTOR"),
                &Self::FILE_TOO_BIG => f.write_str("FILE_TOO_BIG"),
                &Self::INVALID => f.write_str("INVALID"),
                &Self::NO_SPACE => f.write_str("NO_SPACE"),
                &Self::NO_MEMORY => f.write_str("NO_MEMORY"),
                &Self::NO_ATTRIBUTE => f.write_str("NO_ATTRIBUTE"),
                &Self::FILENAME_TOO_LONG => f.write_str("FILENAME_TOO_LONG"),
                other => f.debug_tuple("Error").field(&other.code).finish(),
            }
        }
    }
}

impl From<Error> for c_int {
    fn from(error: Error) -> Self {
        error.code
    }
}
