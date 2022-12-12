//! The [`Write`] trait definition and implementations.

/// A type that writes byte slices.
pub trait Write {
    type Error;

    /// Write the whole byte slice.
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

impl<W: Write + ?Sized> Write for &mut W {
    type Error = W::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        (**self).write_all(buf)
    }
}

impl Write for &mut [u8] {
    type Error = EndOfSlice;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        if self.len() < buf.len() {
            return Err(EndOfSlice(()))
        }
        let this = core::mem::take(self);
        let (prefix, suffix) = this.split_at_mut(buf.len());
        prefix.copy_from_slice(buf);
        *self = suffix;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl Write for alloc::vec::Vec<u8> {
    type Error = core::convert::Infallible;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

/// Wrapper around a `Write` impl that keeps track of the write position.
#[derive(Debug)]
pub struct Cursor<W>(W, usize);

impl<W> Cursor<W> {
    pub fn new(w: W) -> Self {
        Cursor(w, 0)
    }

    /// Get the current position.
    pub fn position(&self) -> usize {
        self.1
    }

    /// Access the inner writer.
    pub fn get_ref(&self) -> &W {
        &self.0
    }

    /// Unique access to the inner writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.0
    }

    /// Deconstruct into the inner writer.
    pub fn into_inner(self) -> W {
        self.0
    }
}

impl Write for Cursor<&mut [u8]> {
    type Error = EndOfSlice;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        let mut slice = &mut self.0[self.1 ..];
        slice.write_all(buf)?;
        self.1 += buf.len();
        Ok(())
    }
}

impl<const N: usize> Write for Cursor<[u8; N]> {
    type Error = EndOfArray;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        let mut slice = &mut self.0[self.1 ..];
        slice.write_all(buf).map_err(|_| EndOfArray(()))?;
        self.1 += buf.len();
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl Write for Cursor<alloc::boxed::Box<[u8]>> {
    type Error = EndOfSlice;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        let mut slice = &mut self.0[self.1 ..];
        slice.write_all(buf)?;
        self.1 += buf.len();
        Ok(())
    }
}

/// An adapter for `std::io::Write` types that implements [`Write`].
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct Writer<W>(W);

#[cfg(feature = "std")]
impl<W> Writer<W> {
    pub fn new(w: W) -> Self {
        Writer(w)
    }

    /// Access the inner writer.
    pub fn get_ref(&self) -> &W {
        &self.0
    }

    /// Unique access to the inner writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.0
    }

    /// Deconstruct into the inner writer.
    pub fn into_inner(self) -> W {
        self.0
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> Write for Writer<W> {
    type Error = std::io::Error;

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        std::io::Write::write_all(&mut self.0, buf)
    }
}

/// An error indicating the end of a slice.
#[derive(Debug)]
pub struct EndOfSlice(());

impl core::fmt::Display for EndOfSlice {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("end of slice")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EndOfSlice {}

/// An error indicating the end of an array.
#[derive(Debug)]
pub struct EndOfArray(());

impl core::fmt::Display for EndOfArray {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("end of array")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EndOfArray {}

