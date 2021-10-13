//! Newtypes for `&[u8]`, `[u8;N]` and `Vec<u8>`.
//!
//! To support specialised encoding and decoding of byte slices, byte arrays,
//! and vectors which are represented as CBOR bytes instead of arrays of `u8`s,
//! the types `ByteSlice`, `ByteArray` and `ByteVec` (requires feature "alloc")
//! are provided. These implement [`Encode`] and [`Decode`] by translating to
//! and from CBOR bytes.
//!
//! If the feature "derive" is present, specialised traits `EncodeBytes` and
//! `DecodeBytes` are also provided. These are implemented for the
//! aforementioned newtypes as well as for their `Option` variations and
//! regular `&[u8]`, `[u8; N]` and `Vec<u8>`. They enable the direct use of
//! `&[u8]`, `[u8; N]` and `Vec<u8>` in types deriving `Encode` and `Decode`
//! if used with a `#[cbor(with = "minicbor::bytes")]` annotation.

use crate::decode::{self, Decode, Decoder};
use crate::encode::{self, Encode, Encoder, Write};
use core::ops::{Deref, DerefMut};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Newtype for `[u8]`.
///
/// Used to implement `Encode` and `Decode` which translate to
/// CBOR bytes instead of arrays for `u8`s.
#[repr(transparent)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ByteSlice([u8]);

impl<'a> From<&'a [u8]> for &'a ByteSlice {
    fn from(xs: &'a [u8]) -> Self {
        unsafe {
            &*(xs as *const [u8] as *const ByteSlice)
        }
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut ByteSlice {
    fn from(xs: &'a mut [u8]) -> Self {
        unsafe {
            &mut *(xs as *mut [u8] as *mut ByteSlice)
        }
    }
}

impl Deref for ByteSlice {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteSlice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for ByteSlice {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for ByteSlice {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<'a, 'b: 'a> Decode<'b> for &'a ByteSlice {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        d.bytes().map(<&ByteSlice>::from)
    }
}

impl Encode for ByteSlice {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self)?.ok()
    }
}

#[cfg(feature = "alloc")]
impl core::borrow::Borrow<ByteSlice> for Vec<u8> {
    fn borrow(&self) -> &ByteSlice {
        self.as_slice().into()
    }
}

#[cfg(feature = "alloc")]
impl core::borrow::BorrowMut<ByteSlice> for Vec<u8> {
    fn borrow_mut(&mut self) -> &mut ByteSlice {
        self.as_mut_slice().into()
    }
}

#[cfg(feature = "alloc")]
impl core::borrow::Borrow<ByteSlice> for ByteVec {
    fn borrow(&self) -> &ByteSlice {
        self.as_slice().into()
    }
}

#[cfg(feature = "alloc")]
impl core::borrow::BorrowMut<ByteSlice> for ByteVec {
    fn borrow_mut(&mut self) -> &mut ByteSlice {
        self.as_mut_slice().into()
    }
}

impl<const N: usize> core::borrow::Borrow<ByteSlice> for ByteArray<N> {
    fn borrow(&self) -> &ByteSlice {
        self.0[..].into()
    }
}

impl<const N: usize> core::borrow::BorrowMut<ByteSlice> for ByteArray<N> {
    fn borrow_mut(&mut self) -> &mut ByteSlice {
        (&mut self.0[..]).into()
    }
}

#[cfg(feature = "alloc")]
impl alloc::borrow::ToOwned for ByteSlice {
    type Owned = ByteVec;

    fn to_owned(&self) -> Self::Owned {
        ByteVec::from(self.to_vec())
    }
}

/// Newtype for `[u8; N]`.
///
/// Used to implement `Encode` and `Decode` which translate to
/// CBOR bytes instead of arrays for `u8`s.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ByteArray<const N: usize>([u8; N]);

impl<const N: usize> From<[u8; N]> for ByteArray<N> {
    fn from(a: [u8; N]) -> Self {
        ByteArray(a)
    }
}

impl<const N: usize> From<ByteArray<N>> for [u8; N] {
    fn from(a: ByteArray<N>) -> Self {
        a.0
    }
}

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8; N]> for ByteArray<N> {
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8; N]> for ByteArray<N> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

impl<'b, const N: usize> Decode<'b> for ByteArray<N> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        use core::convert::TryFrom;
        let slice = d.bytes()?;
        let array = <[u8; N]>::try_from(slice).map_err(|_| {
            decode::Error::Message("byte slice length does not match expected array length")
        })?;
        Ok(ByteArray(array))
    }
}

impl<const N: usize> Encode for ByteArray<N> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&self.0[..])?.ok()
    }
}

/// Newtype for `Vec<u8>`.
///
/// Used to implement `Encode` and `Decode` which translate to
/// CBOR bytes instead of arrays for `u8`s.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ByteVec(Vec<u8>);

#[cfg(feature = "alloc")]
impl From<Vec<u8>> for ByteVec {
    fn from(xs: Vec<u8>) -> Self {
        ByteVec(xs)
    }
}

#[cfg(feature = "alloc")]
impl From<ByteVec> for Vec<u8> {
    fn from(b: ByteVec) -> Self {
        b.0
    }
}

#[cfg(feature = "alloc")]
impl Deref for ByteVec {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl DerefMut for ByteVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "alloc")]
impl Decode<'_> for ByteVec {
    fn decode(d: &mut Decoder<'_>) -> Result<Self, decode::Error> {
        d.bytes().map(|xs| xs.to_vec().into())
    }
}

#[cfg(feature = "alloc")]
impl Encode for ByteVec {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self)?.ok()
    }
}

// Traits /////////////////////////////////////////////////////////////////////

/// Like [`Encode`] but specific for encoding of byte slices.
#[cfg(feature = "derive")]
pub trait EncodeBytes {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>>;
}

/// Like [`Decode`] but specific for decoding from byte slices.
#[cfg(feature = "derive")]
pub trait DecodeBytes<'b>: Sized {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self, decode::Error>;
}

#[cfg(feature = "derive")]
impl<'a, T: EncodeBytes + ?Sized> EncodeBytes for &'a T {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        (**self).encode_bytes(e)
    }
}

#[cfg(feature = "derive")]
impl EncodeBytes for [u8] {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self)?.ok()
    }
}

#[cfg(feature = "derive")]
impl<'a, 'b: 'a> DecodeBytes<'b> for &'a [u8] {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        d.bytes()
    }
}

#[cfg(all(feature = "derive"))]
impl<const N: usize> EncodeBytes for [u8; N] {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&self[..])?.ok()
    }
}

#[cfg(all(feature = "derive"))]
impl<'b, const N: usize> DecodeBytes<'b> for [u8; N] {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        ByteArray::decode(d).map(ByteArray::into)
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl EncodeBytes for Vec<u8> {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self.as_slice())?.ok()
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<'b> DecodeBytes<'b> for Vec<u8> {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        d.bytes().map(Vec::from)
    }
}

#[cfg(feature = "derive")]
impl EncodeBytes for ByteSlice {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        Self::encode(self, e)
    }
}

#[cfg(feature = "derive")]
impl<'a, 'b: 'a> DecodeBytes<'b> for &'a ByteSlice {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        Self::decode(d)
    }
}

#[cfg(all(feature = "derive"))]
impl<const N: usize> EncodeBytes for ByteArray<N> {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        Self::encode(self, e)
    }
}

#[cfg(all(feature = "derive"))]
impl<'b, const N: usize> DecodeBytes<'b> for ByteArray<N> {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        Self::decode(d)
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl EncodeBytes for ByteVec {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        Self::encode(self, e)
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<'b> DecodeBytes<'b> for ByteVec {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        Self::decode(d)
    }
}

#[cfg(feature = "derive")]
impl<T: EncodeBytes> EncodeBytes for Option<T> {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        if let Some(x) = self {
            x.encode_bytes(e)
        } else {
            e.null()?.ok()
        }
    }
}

#[cfg(feature = "derive")]
impl<'b, T: DecodeBytes<'b>> DecodeBytes<'b> for Option<T> {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        if crate::data::Type::Null == d.datatype()? {
            d.skip()?;
            return Ok(None)
        }
        T::decode_bytes(d).map(Some)
    }
}

/// Freestanding function calling `DecodeBytes::decode_bytes`.
///
/// For use in `#[cbor(with = "minicbor::bytes")]` or `#[cbor(decode_with =
/// "minicbor::bytes::decode")]`.
#[cfg(feature = "derive")]
pub fn decode<'b, T>(d: &mut Decoder<'b>) -> Result<T, decode::Error>
where
    T: DecodeBytes<'b>
{
    T::decode_bytes(d)
}

/// Freestanding function calling `EncodeBytes::encode_bytes`.
///
/// For use in `#[cbor(with = "minicbor::bytes")]` or `#[cbor(encode_with =
/// "minicbor::bytes::encode")]`.
#[cfg(feature = "derive")]
pub fn encode<T, W>(xs: &T, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>>
where
    T: EncodeBytes,
    W: Write
{
    T::encode_bytes(xs, e)
}

