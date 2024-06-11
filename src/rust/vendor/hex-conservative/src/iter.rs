// SPDX-License-Identifier: CC0-1.0

//! Iterator that converts hex to bytes.

use core::convert::TryInto;
use core::iter::FusedIterator;
use core::str;
#[cfg(feature = "std")]
use std::io;

#[cfg(all(feature = "core2", not(feature = "std")))]
use core2::io;

use crate::error::{InvalidCharError, OddLengthStringError};

/// Convenience alias for `HexToBytesIter<HexDigitsIter<'a>>`.
pub type HexSliceToBytesIter<'a> = HexToBytesIter<HexDigitsIter<'a>>;

/// Iterator yielding bytes decoded from an iterator of pairs of hex digits.
pub struct HexToBytesIter<T: Iterator<Item = [u8; 2]>> {
    iter: T,
}

impl<'a> HexToBytesIter<HexDigitsIter<'a>> {
    /// Constructs a new `HexToBytesIter` from a string slice.
    ///
    /// # Errors
    ///
    /// If the input string is of odd length.
    #[inline]
    pub fn new(s: &'a str) -> Result<Self, OddLengthStringError> {
        if s.len() % 2 != 0 {
            Err(OddLengthStringError { len: s.len() })
        } else {
            Ok(Self::new_unchecked(s))
        }
    }

    pub(crate) fn new_unchecked(s: &'a str) -> Self {
        Self::from_pairs(HexDigitsIter::new_unchecked(s.as_bytes()))
    }
}

impl<T: Iterator<Item = [u8; 2]>> HexToBytesIter<T> {
    /// Constructs a custom hex decoding iterator from another iterator.
    #[inline]
    pub fn from_pairs(iter: T) -> Self { Self { iter } }
}

impl<T: Iterator<Item = [u8; 2]>> Iterator for HexToBytesIter<T> {
    type Item = Result<u8, InvalidCharError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let [hi, lo] = self.iter.next()?;
        Some(hex_chars_to_byte(hi, lo))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.iter.size_hint();
        (min / 2, max.map(|x| x / 2))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let [hi, lo] = self.iter.nth(n)?;
        Some(hex_chars_to_byte(hi, lo))
    }
}

impl<T: Iterator<Item = [u8; 2]> + DoubleEndedIterator> DoubleEndedIterator for HexToBytesIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let [hi, lo] = self.iter.next_back()?;
        Some(hex_chars_to_byte(hi, lo))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let [hi, lo] = self.iter.nth_back(n)?;
        Some(hex_chars_to_byte(hi, lo))
    }
}

impl<T: Iterator<Item = [u8; 2]> + ExactSizeIterator> ExactSizeIterator for HexToBytesIter<T> {}

impl<T: Iterator<Item = [u8; 2]> + FusedIterator> FusedIterator for HexToBytesIter<T> {}

#[cfg(any(feature = "std", feature = "core2"))]
impl<T: Iterator<Item = [u8; 2]> + FusedIterator> io::Read for HexToBytesIter<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0usize;
        for dst in buf {
            match self.next() {
                Some(Ok(src)) => {
                    *dst = src;
                    bytes_read += 1;
                }
                _ => break,
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

impl<'a> Iterator for HexDigitsIter<'a> {
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

impl<'a> DoubleEndedIterator for HexDigitsIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|digits| digits.try_into().expect("HexDigitsIter invariant"))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|digits| digits.try_into().expect("HexDigitsIter invariant"))
    }
}

impl<'a> ExactSizeIterator for HexDigitsIter<'a> {}

impl<'a> core::iter::FusedIterator for HexDigitsIter<'a> {}

/// `hi` and `lo` are bytes representing hex characters.
fn hex_chars_to_byte(hi: u8, lo: u8) -> Result<u8, InvalidCharError> {
    let hih = (hi as char).to_digit(16).ok_or(InvalidCharError { invalid: hi })?;
    let loh = (lo as char).to_digit(16).ok_or(InvalidCharError { invalid: lo })?;

    let ret = (hih << 4) + loh;
    Ok(ret as u8)
}

/// Iterator over bytes which encodes the bytes and yields hex characters.
pub struct BytesToHexIter<I: Iterator<Item = u8>> {
    /// The iterator whose next byte will be encoded to yield hex characters.
    iter: I,
    /// The low character of the pair (high, low) of hex characters encoded per byte.
    low: Option<char>,
}

impl<I> BytesToHexIter<I>
where
    I: Iterator<Item = u8>,
{
    /// Constructs a new `BytesToHexIter` from a byte iterator.
    pub fn new(iter: I) -> BytesToHexIter<I> { Self { iter, low: None } }
}

impl<I> Iterator for BytesToHexIter<I>
where
    I: Iterator<Item = u8>,
{
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        match self.low {
            Some(c) => {
                self.low = None;
                Some(c)
            }
            None => self.iter.next().map(|b| {
                let (high, low) = byte_to_hex_chars(b);
                self.low = Some(low);
                high
            }),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.iter.size_hint();
        match self.low {
            Some(_) => (min * 2 + 1, max.map(|max| max * 2 + 1)),
            None => (min * 2, max.map(|max| max * 2)),
        }
    }
}

impl<I> DoubleEndedIterator for BytesToHexIter<I>
where
    I: DoubleEndedIterator + Iterator<Item = u8>,
{
    #[inline]
    fn next_back(&mut self) -> Option<char> {
        match self.low {
            Some(c) => {
                self.low = None;
                Some(c)
            }
            None => self.iter.next_back().map(|b| {
                let (high, low) = byte_to_hex_chars(b);
                self.low = Some(low);
                high
            }),
        }
    }
}

impl<I> ExactSizeIterator for BytesToHexIter<I>
where
    I: ExactSizeIterator + Iterator<Item = u8>,
{
    #[inline]
    fn len(&self) -> usize { self.iter.len() * 2 }
}

impl<I> FusedIterator for BytesToHexIter<I> where I: FusedIterator + Iterator<Item = u8> {}

/// Returns the (high, low) hex characters encoding `b`.
fn byte_to_hex_chars(b: u8) -> (char, char) {
    const HEX_TABLE: [u8; 16] = *b"0123456789abcdef";

    let high = HEX_TABLE[usize::from(b >> 4)];
    let low = HEX_TABLE[usize::from(b & 0b00001111)];

    (char::from(high), char::from(low))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_byte() {
        let tcs =
            vec![(0x00, ('0', '0')), (0x0a, ('0', 'a')), (0xad, ('a', 'd')), (0xff, ('f', 'f'))];
        for (b, (high, low)) in tcs {
            assert_eq!(byte_to_hex_chars(b), (high, low));
        }
        assert_eq!(byte_to_hex_chars(0x00), ('0', '0'));
        assert_eq!(byte_to_hex_chars(0x0a), ('0', 'a'));
        assert_eq!(byte_to_hex_chars(0xad), ('a', 'd'));
        assert_eq!(byte_to_hex_chars(0xff), ('f', 'f'));
    }

    #[test]
    fn decode_iter_forward() {
        let hex = "deadbeef";
        let bytes = [0xde, 0xad, 0xbe, 0xef];

        for (i, b) in HexToBytesIter::new(hex).unwrap().enumerate() {
            assert_eq!(b.unwrap(), bytes[i]);
        }
    }

    #[test]
    fn decode_iter_backward() {
        let hex = "deadbeef";
        let bytes = [0xef, 0xbe, 0xad, 0xde];

        for (i, b) in HexToBytesIter::new(hex).unwrap().rev().enumerate() {
            assert_eq!(b.unwrap(), bytes[i]);
        }
    }

    #[test]
    fn encode_iter() {
        let bytes = [0xde, 0xad, 0xbe, 0xef];
        let hex = "deadbeef";

        for (i, c) in BytesToHexIter::new(bytes.iter().cloned()).enumerate() {
            assert_eq!(c, hex.chars().nth(i).unwrap());
        }
    }

    #[test]
    fn encode_iter_backwards() {
        let bytes = [0xde, 0xad, 0xbe, 0xef];
        let hex = "efbeadde";

        for (i, c) in BytesToHexIter::new(bytes.iter().cloned()).rev().enumerate() {
            assert_eq!(c, hex.chars().nth(i).unwrap());
        }
    }

    #[test]
    fn roundtrip_forward() {
        let hex = "deadbeefcafebabe";
        let bytes_iter = HexToBytesIter::new(hex).unwrap().map(|res| res.unwrap());
        let got = BytesToHexIter::new(bytes_iter).collect::<String>();
        assert_eq!(got, hex);
    }

    #[test]
    fn roundtrip_backward() {
        let hex = "deadbeefcafebabe";
        let bytes_iter = HexToBytesIter::new(hex).unwrap().rev().map(|res| res.unwrap());
        let got = BytesToHexIter::new(bytes_iter).rev().collect::<String>();
        assert_eq!(got, hex);
    }
}
