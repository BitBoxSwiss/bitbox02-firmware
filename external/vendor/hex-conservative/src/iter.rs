// SPDX-License-Identifier: CC0-1.0

//! Iterator that converts hex to bytes.

use core::convert::TryInto;
use core::iter::FusedIterator;
use core::str;
#[cfg(feature = "std")]
use std::io;

#[cfg(feature = "alloc")]
use crate::alloc::vec::Vec;
use crate::error::{InvalidCharError, OddLengthStringError};

/// Iterator yielding bytes decoded from an iterator of pairs of hex digits.
#[derive(Debug)]
pub struct HexToBytesIter<I>
where
    I: Iterator<Item = [u8; 2]>,
{
    iter: I,
    original_len: usize,
}

impl<'a> HexToBytesIter<HexDigitsIter<'a>> {
    /// Constructs a new `HexToBytesIter` from a string slice.
    ///
    /// # Errors
    ///
    /// If the input string is of odd length.
    #[inline]
    #[allow(dead_code)] // Remove this when making HexToBytesIter public.
    pub(crate) fn new(s: &'a str) -> Result<Self, OddLengthStringError> {
        if s.len() % 2 != 0 {
            Err(OddLengthStringError { len: s.len() })
        } else {
            Ok(Self::new_unchecked(s))
        }
    }

    #[inline]
    pub(crate) fn new_unchecked(s: &'a str) -> Self {
        Self::from_pairs(HexDigitsIter::new_unchecked(s.as_bytes()))
    }

    /// Writes all the bytes yielded by this `HexToBytesIter` to the provided slice.
    ///
    /// Stops writing if this `HexToBytesIter` yields an `InvalidCharError`.
    ///
    /// # Panics
    ///
    /// Panics if the length of this `HexToBytesIter` is not equal to the length of the provided
    /// slice.
    pub(crate) fn drain_to_slice(self, buf: &mut [u8]) -> Result<(), InvalidCharError> {
        assert_eq!(self.len(), buf.len());
        let mut ptr = buf.as_mut_ptr();
        for byte in self {
            // SAFETY: for loop iterates `len` times, and `buf` has length `len`
            unsafe {
                core::ptr::write(ptr, byte?);
                ptr = ptr.add(1);
            }
        }
        Ok(())
    }

    /// Writes all the bytes yielded by this `HexToBytesIter` to a `Vec<u8>`.
    ///
    /// This is equivalent to the combinator chain `iter().map().collect()` but was found by
    /// benchmarking to be faster.
    #[cfg(feature = "alloc")]
    pub(crate) fn drain_to_vec(self) -> Result<Vec<u8>, InvalidCharError> {
        let len = self.len();
        let mut ret = Vec::with_capacity(len);
        let mut ptr = ret.as_mut_ptr();
        for byte in self {
            // SAFETY: for loop iterates `len` times, and `ret` has a capacity of at least `len`
            unsafe {
                // docs: "`core::ptr::write` is appropriate for initializing uninitialized memory"
                core::ptr::write(ptr, byte?);
                ptr = ptr.add(1);
            }
        }
        // SAFETY: `len` elements have been initialized, and `ret` has a capacity of at least `len`
        unsafe {
            ret.set_len(len);
        }
        Ok(ret)
    }
}

impl<I> HexToBytesIter<I>
where
    I: Iterator<Item = [u8; 2]> + ExactSizeIterator,
{
    /// Constructs a custom hex decoding iterator from another iterator.
    #[inline]
    pub fn from_pairs(iter: I) -> Self { Self { original_len: iter.len(), iter } }
}

impl<I> Iterator for HexToBytesIter<I>
where
    I: Iterator<Item = [u8; 2]> + ExactSizeIterator,
{
    type Item = Result<u8, InvalidCharError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let [hi, lo] = self.iter.next()?;
        Some(hex_chars_to_byte(hi, lo).map_err(|(c, is_high)| InvalidCharError {
            invalid: c,
            pos: if is_high {
                (self.original_len - self.iter.len() - 1) * 2
            } else {
                (self.original_len - self.iter.len() - 1) * 2 + 1
            },
        }))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let [hi, lo] = self.iter.nth(n)?;
        Some(hex_chars_to_byte(hi, lo).map_err(|(c, is_high)| InvalidCharError {
            invalid: c,
            pos: if is_high {
                (self.original_len - self.iter.len() - 1) * 2
            } else {
                (self.original_len - self.iter.len() - 1) * 2 + 1
            },
        }))
    }
}

impl<I> DoubleEndedIterator for HexToBytesIter<I>
where
    I: Iterator<Item = [u8; 2]> + DoubleEndedIterator + ExactSizeIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let [hi, lo] = self.iter.next_back()?;
        Some(hex_chars_to_byte(hi, lo).map_err(|(c, is_high)| InvalidCharError {
            invalid: c,
            pos: if is_high { self.iter.len() * 2 } else { self.iter.len() * 2 + 1 },
        }))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let [hi, lo] = self.iter.nth_back(n)?;
        Some(hex_chars_to_byte(hi, lo).map_err(|(c, is_high)| InvalidCharError {
            invalid: c,
            pos: if is_high { self.iter.len() * 2 } else { self.iter.len() * 2 + 1 },
        }))
    }
}

impl<I> ExactSizeIterator for HexToBytesIter<I> where I: Iterator<Item = [u8; 2]> + ExactSizeIterator
{}

impl<I> FusedIterator for HexToBytesIter<I> where
    I: Iterator<Item = [u8; 2]> + ExactSizeIterator + FusedIterator
{
}

#[cfg(feature = "std")]
impl<I> io::Read for HexToBytesIter<I>
where
    I: Iterator<Item = [u8; 2]> + ExactSizeIterator + FusedIterator,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0usize;
        for dst in buf {
            match self.next() {
                Some(Ok(src)) => {
                    *dst = src;
                    bytes_read += 1;
                }
                Some(Err(e)) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
                None => break,
            }
        }
        Ok(bytes_read)
    }
}

/// An internal iterator returning hex digits from a string.
///
/// Generally you shouldn't need to refer to this or bother with it and just use
/// [`HexToBytesIter::new`] consuming the returned value and use `HexSliceToBytesIter` if you need
/// to refer to the iterator in your types.
#[derive(Debug)]
pub struct HexDigitsIter<'a> {
    // Invariant: the length of the chunks is 2.
    // Technically, this is `iter::Map` but we can't use it because fn is anonymous.
    // We can swap this for actual `ArrayChunks` once it's stable.
    iter: core::slice::ChunksExact<'a, u8>,
}

impl<'a> HexDigitsIter<'a> {
    #[inline]
    fn new_unchecked(digits: &'a [u8]) -> Self { Self { iter: digits.chunks_exact(2) } }
}

impl Iterator for HexDigitsIter<'_> {
    type Item = [u8; 2];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|digits| digits.try_into().expect("HexDigitsIter invariant"))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|digits| digits.try_into().expect("HexDigitsIter invariant"))
    }
}

impl DoubleEndedIterator for HexDigitsIter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|digits| digits.try_into().expect("HexDigitsIter invariant"))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|digits| digits.try_into().expect("HexDigitsIter invariant"))
    }
}

impl ExactSizeIterator for HexDigitsIter<'_> {}

impl core::iter::FusedIterator for HexDigitsIter<'_> {}

/// `hi` and `lo` are bytes representing hex characters.
///
/// Returns the valid byte or the invalid input byte and a bool indicating error for `hi` or `lo`.
fn hex_chars_to_byte(hi: u8, lo: u8) -> Result<u8, (u8, bool)> {
    let hih = (hi as char).to_digit(16).ok_or((hi, true))?;
    let loh = (lo as char).to_digit(16).ok_or((lo, false))?;

    let ret = (hih << 4) + loh;
    Ok(ret as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_iter_forward() {
        let hex = "deadbeef";
        let bytes = [0xde, 0xad, 0xbe, 0xef];

        for (i, b) in HexToBytesIter::new(hex).unwrap().enumerate() {
            assert_eq!(b.unwrap(), bytes[i]);
        }

        let mut iter = HexToBytesIter::new(hex).unwrap();
        for i in (0..=bytes.len()).rev() {
            assert_eq!(iter.len(), i);
            let _ = iter.next();
        }
    }

    #[test]
    fn decode_iter_backward() {
        let hex = "deadbeef";
        let bytes = [0xef, 0xbe, 0xad, 0xde];

        for (i, b) in HexToBytesIter::new(hex).unwrap().rev().enumerate() {
            assert_eq!(b.unwrap(), bytes[i]);
        }

        let mut iter = HexToBytesIter::new(hex).unwrap().rev();
        for i in (0..=bytes.len()).rev() {
            assert_eq!(iter.len(), i);
            let _ = iter.next();
        }
    }

    #[test]
    fn hex_to_digits_size_hint() {
        let hex = "deadbeef";
        let iter = HexDigitsIter::new_unchecked(hex.as_bytes());
        // HexDigitsIter yields two digits at a time `[u8; 2]`.
        assert_eq!(iter.size_hint(), (4, Some(4)));
    }

    #[test]
    fn hex_to_bytes_size_hint() {
        let hex = "deadbeef";
        let iter = HexToBytesIter::new_unchecked(hex);
        assert_eq!(iter.size_hint(), (4, Some(4)));
    }

    #[test]
    fn hex_to_bytes_slice_drain() {
        let hex = "deadbeef";
        let want = [0xde, 0xad, 0xbe, 0xef];
        let iter = HexToBytesIter::new_unchecked(hex);
        let mut got = [0u8; 4];
        iter.drain_to_slice(&mut got).unwrap();
        assert_eq!(got, want);

        let hex = "";
        let want = [];
        let iter = HexToBytesIter::new_unchecked(hex);
        let mut got = [];
        iter.drain_to_slice(&mut got).unwrap();
        assert_eq!(got, want);
    }

    #[test]
    #[should_panic]
    // Don't test panic message because it is from `debug_assert`.
    #[allow(clippy::should_panic_without_expect)]
    fn hex_to_bytes_slice_drain_panic_empty() {
        let hex = "deadbeef";
        let iter = HexToBytesIter::new_unchecked(hex);
        let mut got = [];
        iter.drain_to_slice(&mut got).unwrap();
    }

    #[test]
    #[should_panic]
    // Don't test panic message because it is from `debug_assert`.
    #[allow(clippy::should_panic_without_expect)]
    fn hex_to_bytes_slice_drain_panic_too_small() {
        let hex = "deadbeef";
        let iter = HexToBytesIter::new_unchecked(hex);
        let mut got = [0u8; 3];
        iter.drain_to_slice(&mut got).unwrap();
    }

    #[test]
    #[should_panic]
    // Don't test panic message because it is from `debug_assert`.
    #[allow(clippy::should_panic_without_expect)]
    fn hex_to_bytes_slice_drain_panic_too_big() {
        let hex = "deadbeef";
        let iter = HexToBytesIter::new_unchecked(hex);
        let mut got = [0u8; 5];
        iter.drain_to_slice(&mut got).unwrap();
    }

    #[test]
    fn hex_to_bytes_slice_drain_first_char_error() {
        let hex = "geadbeef";
        let iter = HexToBytesIter::new_unchecked(hex);
        let mut got = [0u8; 4];
        assert_eq!(
            iter.drain_to_slice(&mut got).unwrap_err(),
            InvalidCharError { invalid: b'g', pos: 0 }
        );
    }

    #[test]
    fn hex_to_bytes_slice_drain_middle_char_error() {
        let hex = "deadgeef";
        let iter = HexToBytesIter::new_unchecked(hex);
        let mut got = [0u8; 4];
        assert_eq!(
            iter.drain_to_slice(&mut got).unwrap_err(),
            InvalidCharError { invalid: b'g', pos: 4 }
        );
    }

    #[test]
    fn hex_to_bytes_slice_drain_end_char_error() {
        let hex = "deadbeeg";
        let iter = HexToBytesIter::new_unchecked(hex);
        let mut got = [0u8; 4];
        assert_eq!(
            iter.drain_to_slice(&mut got).unwrap_err(),
            InvalidCharError { invalid: b'g', pos: 7 }
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn hex_to_bytes_vec_drain() {
        let hex = "deadbeef";
        let want = [0xde, 0xad, 0xbe, 0xef];
        let iter = HexToBytesIter::new_unchecked(hex);
        let got = iter.drain_to_vec().unwrap();
        assert_eq!(got, want);

        let hex = "";
        let iter = HexToBytesIter::new_unchecked(hex);
        let got = iter.drain_to_vec().unwrap();
        assert!(got.is_empty());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn hex_to_bytes_vec_drain_first_char_error() {
        let hex = "geadbeef";
        let iter = HexToBytesIter::new_unchecked(hex);
        assert_eq!(iter.drain_to_vec().unwrap_err(), InvalidCharError { invalid: b'g', pos: 0 });
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn hex_to_bytes_vec_drain_middle_char_error() {
        let hex = "deadgeef";
        let iter = HexToBytesIter::new_unchecked(hex);
        assert_eq!(iter.drain_to_vec().unwrap_err(), InvalidCharError { invalid: b'g', pos: 4 });
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn hex_to_bytes_vec_drain_end_char_error() {
        let hex = "deadbeeg";
        let iter = HexToBytesIter::new_unchecked(hex);
        assert_eq!(iter.drain_to_vec().unwrap_err(), InvalidCharError { invalid: b'g', pos: 7 });
    }

    #[test]
    #[cfg(feature = "std")]
    fn hex_to_bytes_iter_read() {
        use std::io::Read;

        let hex = "deadbeef";
        let mut iter = HexToBytesIter::new(hex).unwrap();
        let mut buf = [0u8; 4];
        let bytes_read = iter.read(&mut buf).unwrap();
        assert_eq!(bytes_read, 4);
        assert_eq!(buf, [0xde, 0xad, 0xbe, 0xef]);

        let hex = "deadbeef";
        let mut iter = HexToBytesIter::new(hex).unwrap();
        let mut buf = [0u8; 2];
        let bytes_read = iter.read(&mut buf).unwrap();
        assert_eq!(bytes_read, 2);
        assert_eq!(buf, [0xde, 0xad]);

        let hex = "deadbeef";
        let mut iter = HexToBytesIter::new(hex).unwrap();
        let mut buf = [0u8; 6];
        let bytes_read = iter.read(&mut buf).unwrap();
        assert_eq!(bytes_read, 4);
        assert_eq!(buf[..4], [0xde, 0xad, 0xbe, 0xef]);

        let hex = "deadbeefXX";
        let mut iter = HexToBytesIter::new(hex).unwrap();
        let mut buf = [0u8; 6];
        let err = iter.read(&mut buf).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }
}
