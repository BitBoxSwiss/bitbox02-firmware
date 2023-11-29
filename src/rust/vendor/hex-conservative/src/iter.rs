// SPDX-License-Identifier: CC0-1.0

//! Iterator that converts hex to bytes.

use core::iter::FusedIterator;
use core::str;
#[cfg(feature = "std")]
use std::io;

#[cfg(all(feature = "core2", not(feature = "std")))]
use core2::io;

use crate::parse::HexToBytesError;

/// Iterator over a hex-encoded string slice which decodes hex and yields bytes.
pub struct HexToBytesIter<'a> {
    /// The [`Bytes`] iterator whose next two bytes will be decoded to yield the next byte.
    ///
    /// # Invariants
    ///
    /// `iter` is guaranteed to be of even length.
    ///
    /// [`Bytes`]: core::str::Bytes
    iter: str::Bytes<'a>,
}

impl<'a> HexToBytesIter<'a> {
    /// Constructs a new `HexToBytesIter` from a string slice.
    ///
    /// # Errors
    ///
    /// If the input string is of odd length.
    pub fn new(s: &'a str) -> Result<HexToBytesIter<'a>, HexToBytesError> {
        if s.len() % 2 != 0 {
            Err(HexToBytesError::OddLengthString(s.len()))
        } else {
            Ok(HexToBytesIter { iter: s.bytes() })
        }
    }
}

impl<'a> Iterator for HexToBytesIter<'a> {
    type Item = Result<u8, HexToBytesError>;

    fn next(&mut self) -> Option<Result<u8, HexToBytesError>> {
        let hi = self.iter.next()?;
        let lo = self.iter.next().expect("iter length invariant violated, this is a bug");
        Some(hex_chars_to_byte(hi, lo))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.iter.size_hint();
        (min / 2, max.map(|x| x / 2))
    }
}

impl<'a> DoubleEndedIterator for HexToBytesIter<'a> {
    fn next_back(&mut self) -> Option<Result<u8, HexToBytesError>> {
        let lo = self.iter.next_back()?;
        let hi = self.iter.next_back().expect("iter length invariant violated, this is a bug");
        Some(hex_chars_to_byte(hi, lo))
    }
}

impl<'a> ExactSizeIterator for HexToBytesIter<'a> {
    fn len(&self) -> usize { self.iter.len() / 2 }
}

impl<'a> FusedIterator for HexToBytesIter<'a> {}

#[cfg(any(feature = "std", feature = "core2"))]
impl<'a> io::Read for HexToBytesIter<'a> {
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

/// `hi` and `lo` are bytes representing hex characters.
fn hex_chars_to_byte(hi: u8, lo: u8) -> Result<u8, HexToBytesError> {
    let hih = (hi as char).to_digit(16).ok_or(HexToBytesError::InvalidChar(hi))?;
    let loh = (lo as char).to_digit(16).ok_or(HexToBytesError::InvalidChar(lo))?;

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
        let v = vec![0xde, 0xad, 0xbe, 0xef];

        for (i, b) in HexToBytesIter::new(hex).unwrap().enumerate() {
            assert_eq!(b.unwrap(), v[i]);
        }
    }

    #[test]
    fn decode_iter_backward() {
        let hex = "deadbeef";
        let v = vec![0xef, 0xbe, 0xad, 0xde];

        for (i, b) in HexToBytesIter::new(hex).unwrap().rev().enumerate() {
            assert_eq!(b.unwrap(), v[i]);
        }
    }

    #[test]
    fn encode_iter() {
        let v = vec![0xde, 0xad, 0xbe, 0xef];
        let hex = "deadbeef";

        for (i, c) in BytesToHexIter::new(v.iter().cloned()).enumerate() {
            assert_eq!(c, hex.chars().nth(i).unwrap());
        }
    }

    #[test]
    fn encode_iter_backwards() {
        let v = vec![0xde, 0xad, 0xbe, 0xef];
        let hex = "efbeadde";

        for (i, c) in BytesToHexIter::new(v.iter().cloned()).rev().enumerate() {
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
