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
//! aforementioned newtypes as well as for their `Option` variations, regular
//! `&[u8]`, `[u8; N]`, `Vec<u8>` and for `Cow<'_, [u8]>` if the alloc feature
//! is given. They enable the direct use of `&[u8]`, `[u8; N]`, `Vec<u8>` and
//! `Cow<'_, [u8]>` in types deriving `Encode` and `Decode` if used with a
//! `#[cbor(with = "minicbor::bytes")]` annotation.

use crate::decode::{self, Decode, Decoder};
use crate::encode::{self, Encode, Encoder, Write, CborLen};
use core::ops::{Deref, DerefMut};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(all(feature = "alloc", feature = "derive"))]
use alloc::borrow::{Cow, ToOwned};

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

impl<'a, 'b: 'a, C> Decode<'b, C> for &'a ByteSlice {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, decode::Error> {
        d.bytes().map(<&ByteSlice>::from)
    }
}

impl<C> Encode<C> for ByteSlice {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _: &mut C) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self)?.ok()
    }
}

impl<C> CborLen<C> for ByteSlice {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        let n = self.len();
        n.cbor_len(ctx) + n
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

impl<'b, C, const N: usize> Decode<'b, C> for ByteArray<N> {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, decode::Error> {
        let pos   = d.position();
        let slice = d.bytes()?;
        let array = <[u8; N]>::try_from(slice).map_err(|_| {
            decode::Error::message("byte slice length does not match expected array length").at(pos)
        })?;
        Ok(ByteArray(array))
    }
}

impl<C, const N: usize> Encode<C> for ByteArray<N> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _: &mut C) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&self.0[..])?.ok()
    }
}

impl<C, const N: usize> CborLen<C> for ByteArray<N> {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        N.cbor_len(ctx) + N
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
impl<C> Decode<'_, C> for ByteVec {
    fn decode(d: &mut Decoder<'_>, _: &mut C) -> Result<Self, decode::Error> {
        d.bytes().map(|xs| xs.to_vec().into())
    }
}

#[cfg(feature = "alloc")]
impl<C> Encode<C> for ByteVec {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _: &mut C) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self)?.ok()
    }
}

#[cfg(feature = "alloc")]
impl<C> CborLen<C> for ByteVec {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        let n = self.len();
        n.cbor_len(ctx) + n
    }
}

// Traits /////////////////////////////////////////////////////////////////////

/// Like [`Encode`] but specific for encoding of byte slices.
#[cfg(feature = "derive")]
pub trait EncodeBytes<C> {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>>;

    fn is_nil(&self) -> bool {
        false
    }
}

/// Like [`Decode`] but specific for decoding from byte slices.
#[cfg(feature = "derive")]
pub trait DecodeBytes<'b, C>: Sized {
    fn decode_bytes(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error>;

    fn nil() -> Option<Self> {
        None
    }
}

/// Like [`CborLen`] but specific for byte slices.
#[cfg(feature = "derive")]
pub trait CborLenBytes<C> {
    fn cbor_len(&self, ctx: &mut C) -> usize;
}

#[cfg(feature = "derive")]
impl<'a, C, T: EncodeBytes<C> + ?Sized> EncodeBytes<C> for &'a T {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>> {
        (**self).encode_bytes(e, ctx)
    }
}

#[cfg(feature = "derive")]
impl<C> EncodeBytes<C> for [u8] {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, _: &mut C) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self)?.ok()
    }
}

#[cfg(feature = "derive")]
impl<'a, 'b: 'a, C> DecodeBytes<'b, C> for &'a [u8] {
    fn decode_bytes(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, decode::Error> {
        d.bytes()
    }
}

#[cfg(feature = "derive")]
impl<'a, C, T: CborLenBytes<C> + ?Sized> CborLenBytes<C> for &'a T {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        (**self).cbor_len(ctx)
    }
}

#[cfg(feature = "derive")]
impl<C> CborLenBytes<C> for [u8] {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        let n = self.len();
        n.cbor_len(ctx) + n
    }
}

#[cfg(feature = "derive")]
impl<C, const N: usize> EncodeBytes<C> for [u8; N] {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, _: &mut C) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&self[..])?.ok()
    }
}

#[cfg(feature = "derive")]
impl<'b, C, const N: usize> DecodeBytes<'b, C> for [u8; N] {
    fn decode_bytes(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
        ByteArray::decode(d, ctx).map(ByteArray::into)
    }
}

#[cfg(feature = "derive")]
impl<C, const N: usize> CborLenBytes<C> for [u8; N] {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        N.cbor_len(ctx) + N
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<C> EncodeBytes<C> for Vec<u8> {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, _: &mut C) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self.as_slice())?.ok()
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<'b, C> DecodeBytes<'b, C> for Vec<u8> {
    fn decode_bytes(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, decode::Error> {
        d.bytes().map(Vec::from)
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<C> CborLenBytes<C> for Vec<u8> {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        let n = self.len();
        n.cbor_len(ctx) + n
    }
}

#[cfg(feature = "derive")]
impl<C> EncodeBytes<C> for ByteSlice {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>> {
        Self::encode(self, e, ctx)
    }
}

#[cfg(feature = "derive")]
impl<'a, 'b: 'a, C> DecodeBytes<'b, C> for &'a ByteSlice {
    fn decode_bytes(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
        Self::decode(d, ctx)
    }
}

#[cfg(feature = "derive")]
impl<C> CborLenBytes<C> for ByteSlice {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        <Self as CborLen<C>>::cbor_len(self, ctx)
    }
}

#[cfg(feature = "derive")]
impl<C, const N: usize> EncodeBytes<C> for ByteArray<N> {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>> {
        Self::encode(self, e, ctx)
    }
}

#[cfg(feature = "derive")]
impl<'b, C, const N: usize> DecodeBytes<'b, C> for ByteArray<N> {
    fn decode_bytes(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
        Self::decode(d, ctx)
    }
}

#[cfg(feature = "derive")]
impl<C, const N: usize> CborLenBytes<C> for ByteArray<N> {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        <Self as CborLen<C>>::cbor_len(self, ctx)
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<C> EncodeBytes<C> for ByteVec {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>> {
        Self::encode(self, e, ctx)
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<'b, C> DecodeBytes<'b, C> for ByteVec {
    fn decode_bytes(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
        Self::decode(d, ctx)
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<C> CborLenBytes<C> for ByteVec {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        <Self as CborLen<C>>::cbor_len(self, ctx)
    }
}

#[cfg(feature = "derive")]
impl<C, T: EncodeBytes<C>> EncodeBytes<C> for Option<T> {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>> {
        if let Some(x) = self {
            x.encode_bytes(e, ctx)
        } else {
            e.null()?.ok()
        }
    }

    fn is_nil(&self) -> bool {
        self.is_none()
    }
}

#[cfg(feature = "derive")]
impl<'b, C, T: DecodeBytes<'b, C>> DecodeBytes<'b, C> for Option<T> {
    fn decode_bytes(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
        if crate::data::Type::Null == d.datatype()? {
            d.skip()?;
            return Ok(None)
        }
        T::decode_bytes(d, ctx).map(Some)
    }

    fn nil() -> Option<Self> {
        Some(None)
    }
}

#[cfg(feature = "derive")]
impl<C, T: CborLenBytes<C>> CborLenBytes<C> for Option<T> {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        if let Some(x) = self {
            x.cbor_len(ctx)
        } else {
            1
        }
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<C> EncodeBytes<C> for Cow<'_, [u8]> {
    fn encode_bytes<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>> {
        self.as_ref().encode_bytes(e, ctx)
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<'b, C> DecodeBytes<'b, C> for Cow<'_, [u8]> {
    fn decode_bytes(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
        let slice = <&'b ByteSlice>::decode_bytes(d, ctx)?;
        Ok(Cow::Owned(slice.to_owned().into()))
    }
}

#[cfg(all(feature = "alloc", feature = "derive"))]
impl<C> CborLenBytes<C> for Cow<'_, [u8]> {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        <_ as CborLenBytes<C>>::cbor_len(self.as_ref(), ctx)
    }
}

/// Freestanding function calling `DecodeBytes::decode_bytes`.
///
/// For use in `#[cbor(with = "minicbor::bytes")]` or `#[cbor(decode_with =
/// "minicbor::bytes::decode")]`.
#[cfg(feature = "derive")]
pub fn decode<'b, C, T>(d: &mut Decoder<'b>, ctx: &mut C) -> Result<T, decode::Error>
where
    T: DecodeBytes<'b, C>
{
    T::decode_bytes(d, ctx)
}

#[cfg(feature = "derive")]
pub fn nil<'b, C, T>() -> Option<T>
where
    T: DecodeBytes<'b, C>
{
    T::nil()
}

/// Freestanding function calling `EncodeBytes::encode_bytes`.
///
/// For use in `#[cbor(with = "minicbor::bytes")]` or `#[cbor(encode_with =
/// "minicbor::bytes::encode")]`.
#[cfg(feature = "derive")]
pub fn encode<C, T, W>(xs: &T, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>>
where
    T: EncodeBytes<C>,
    W: Write
{
    T::encode_bytes(xs, e, ctx)
}

#[cfg(feature = "derive")]
pub fn is_nil<C, T>(xs: &T) -> bool
where
    T: EncodeBytes<C>
{
    T::is_nil(xs)
}

#[cfg(feature = "derive")]
pub fn cbor_len<C, T>(xs: T, ctx: &mut C) -> usize
where
    T: CborLenBytes<C>
{
    xs.cbor_len(ctx)
}
