#![allow(clippy::unusual_byte_groupings)]

use crate::{ARRAY, BREAK, BYTES, MAP, SIMPLE, TAGGED, TEXT, SIGNED, UNSIGNED};
use crate::data::{Int, Tag, Type};
use crate::decode::{Decode, Error};
use core::{char, f32, i8, i16, i32, i64};
use core::{marker, str};

/// A non-allocating CBOR decoder.
#[derive(Debug, Clone)]
pub struct Decoder<'b> {
    buf: &'b [u8],
    pos: usize
}

impl<'b> Decoder<'b> {
    /// Construct a `Decoder` for the given byte slice.
    pub fn new(bytes: &'b [u8]) -> Self {
        Decoder { buf: bytes, pos: 0 }
    }

    /// Decode any type that implements [`Decode`].
    pub fn decode<T: Decode<'b, ()>>(&mut self) -> Result<T, Error> {
        T::decode(self, &mut ())
    }

    /// Decode any type that implements [`Decode`].
    pub fn decode_with<C, T: Decode<'b, C>>(&mut self, ctx: &mut C) -> Result<T, Error> {
        T::decode(self, ctx)
    }

    /// Get the current decode position.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Set the current decode position.
    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos
    }

    /// Get a reference to the input bytes.
    pub fn input(&self) -> &'b [u8] {
        self.buf
    }

    /// Get a decoding probe to look ahead what is coming next.
    ///
    /// This will not affect the decoding state of `self` and after the
    /// returned `Probe` has been dropped, decoding can continue from
    /// its current position as if `probe` was never called.
    pub fn probe<'a>(&'a mut self) -> Probe<'a, 'b> {
        Probe {
            decoder: self.clone(),
            _marker: marker::PhantomData
        }
    }

    /// Decode a `bool` value.
    pub fn bool(&mut self) -> Result<bool, Error> {
        let p = self.pos;
        match self.read()? {
            0xf4 => Ok(false),
            0xf5 => Ok(true),
            b    => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected bool"))
        }
    }

    /// Decode a `u8` value.
    pub fn u8(&mut self) -> Result<u8, Error> {
        let p = self.pos;
        match self.read()? {
            n @ 0 ..= 0x17 => Ok(n),
            0x18           => self.read(),
            0x19           => self.read_array().map(u16::from_be_bytes).and_then(|n| try_as(n, "when converting u16 to u8", p)),
            0x1a           => self.read_array().map(u32::from_be_bytes).and_then(|n| try_as(n, "when converting u32 to u8", p)),
            0x1b           => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to u8", p)),
            b              => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected u8"))
        }
    }

    /// Decode a `u16` value.
    pub fn u16(&mut self) -> Result<u16, Error> {
        let p = self.pos;
        match self.read()? {
            n @ 0 ..= 0x17 => Ok(u16::from(n)),
            0x18           => self.read().map(u16::from),
            0x19           => self.read_array().map(u16::from_be_bytes),
            0x1a           => self.read_array().map(u32::from_be_bytes).and_then(|n| try_as(n, "when converting u32 to u16", p)),
            0x1b           => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to u16", p)),
            b              => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected u16"))
        }
    }

    /// Decode a `u32` value.
    pub fn u32(&mut self) -> Result<u32, Error> {
        let p = self.pos;
        match self.read()? {
            n @ 0 ..= 0x17 => Ok(u32::from(n)),
            0x18           => self.read().map(u32::from),
            0x19           => self.read_array().map(u16::from_be_bytes).map(u32::from),
            0x1a           => self.read_array().map(u32::from_be_bytes),
            0x1b           => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to u32", p)),
            b              => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected u32"))
        }
    }

    /// Decode a `u64` value.
    pub fn u64(&mut self) -> Result<u64, Error> {
        let p = self.pos;
        let n = self.read()?;
        self.unsigned(n, p)
    }

    /// Decode an `i8` value.
    pub fn i8(&mut self) -> Result<i8, Error> {
        let p = self.pos;
        match self.read()? {
            n @ 0x00 ..= 0x17 => Ok(n as i8),
            0x18              => self.read().and_then(|n| try_as(n, "when converting u8 to i8", p)),
            0x19              => self.read_array().map(u16::from_be_bytes).and_then(|n| try_as(n, "when converting u16 to i8", p)),
            0x1a              => self.read_array().map(u32::from_be_bytes).and_then(|n| try_as(n, "when converting u32 to i8", p)),
            0x1b              => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to i8", p)),
            n @ 0x20 ..= 0x37 => Ok(-1 - (n - 0x20) as i8),
            0x38              => self.read().and_then(|n| try_as(n, "when converting u8 to i8", p).map(|n: i8| -1 - n)),
            0x39              => self.read_array().map(u16::from_be_bytes).and_then(|n| try_as(n, "when converting u16 to i8", p).map(|n: i8| -1 - n)),
            0x3a              => self.read_array().map(u32::from_be_bytes).and_then(|n| try_as(n, "when converting u32 to i8", p).map(|n: i8| -1 - n)),
            0x3b              => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to i8", p).map(|n: i8| -1 - n)),
            b                 => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected i8"))
        }
    }

    /// Decode an `i16` value.
    pub fn i16(&mut self) -> Result<i16, Error> {
        let p = self.pos;
        match self.read()? {
            n @ 0x00 ..= 0x17 => Ok(i16::from(n)),
            0x18              => self.read().map(i16::from),
            0x19              => self.read_array().map(u16::from_be_bytes).and_then(|n| try_as(n, "when converting u16 to i16", p)),
            0x1a              => self.read_array().map(u32::from_be_bytes).and_then(|n| try_as(n, "when converting u32 to i16", p)),
            0x1b              => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to i16", p)),
            n @ 0x20 ..= 0x37 => Ok(-1 - i16::from(n - 0x20)),
            0x38              => self.read().map(|n| -1 - i16::from(n)),
            0x39              => self.read_array().map(u16::from_be_bytes).and_then(|n| try_as(n, "when converting u16 to i16", p).map(|n: i16| -1 - n)),
            0x3a              => self.read_array().map(u32::from_be_bytes).and_then(|n| try_as(n, "when converting u32 to i16", p).map(|n: i16| -1 - n)),
            0x3b              => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to i16", p).map(|n: i16| -1 - n)),
            b                 => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected i16"))
        }
    }

    /// Decode an `i32` value.
    pub fn i32(&mut self) -> Result<i32, Error> {
        let p = self.pos;
        match self.read()? {
            n @ 0x00 ..= 0x17 => Ok(i32::from(n)),
            0x18              => self.read().map(i32::from),
            0x19              => self.read_array().map(u16::from_be_bytes).map(i32::from),
            0x1a              => self.read_array().map(u32::from_be_bytes).and_then(|n| try_as(n, "when converting u32 to i32", p)),
            0x1b              => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to i32", p)),
            n @ 0x20 ..= 0x37 => Ok(-1 - i32::from(n - 0x20)),
            0x38              => self.read().map(|n| -1 - i32::from(n)),
            0x39              => self.read_array().map(u16::from_be_bytes).map(|n| -1 - i32::from(n)),
            0x3a              => self.read_array().map(u32::from_be_bytes).and_then(|n| try_as(n, "when converting u32 to i32", p).map(|n: i32| -1 - n)),
            0x3b              => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to i32", p).map(|n: i32| -1 - n)),
            b                 => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected i32"))
        }
    }

    /// Decode an `i64` value.
    pub fn i64(&mut self) -> Result<i64, Error> {
        let p = self.pos;
        match self.read()? {
            n @ 0x00 ..= 0x17 => Ok(i64::from(n)),
            0x18              => self.read().map(i64::from),
            0x19              => self.read_array().map(u16::from_be_bytes).map(i64::from),
            0x1a              => self.read_array().map(u32::from_be_bytes).map(i64::from),
            0x1b              => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to i64", p)),
            n @ 0x20 ..= 0x37 => Ok(-1 - i64::from(n - 0x20)),
            0x38              => self.read().map(|n| -1 - i64::from(n)),
            0x39              => self.read_array().map(u16::from_be_bytes).map(|n| -1 - i64::from(n)),
            0x3a              => self.read_array().map(u32::from_be_bytes).map(|n| -1 - i64::from(n)),
            0x3b              => self.read_array().map(u64::from_be_bytes).and_then(|n| try_as(n, "when converting u64 to i64", p).map(|n: i64| -1 - n)),
            b                 => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected i64"))
        }
    }

    /// Decode a CBOR integer.
    ///
    /// See [`Int`] for details regarding the value range of CBOR integers.
    pub fn int(&mut self) -> Result<Int, Error> {
        let p = self.pos;
        match self.read()? {
            n @ 0x00 ..= 0x17 => Ok(Int::pos(n)),
            0x18              => self.read().map(Int::pos),
            0x19              => self.read_array().map(u16::from_be_bytes).map(Int::pos),
            0x1a              => self.read_array().map(u32::from_be_bytes).map(Int::pos),
            0x1b              => self.read_array().map(u64::from_be_bytes).map(Int::pos),
            n @ 0x20 ..= 0x37 => Ok(Int::neg(n - 0x20)),
            0x38              => self.read().map(Int::neg),
            0x39              => self.read_array().map(u16::from_be_bytes).map(Int::neg),
            0x3a              => self.read_array().map(u32::from_be_bytes).map(Int::neg),
            0x3b              => self.read_array().map(u64::from_be_bytes).map(Int::neg),
            b                 => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected int"))
        }
    }

    /// Decode a half float (`f16`) and return it in an `f32`.
    ///
    /// Only available when the feature `half` is present.
    #[cfg(feature = "half")]
    pub fn f16(&mut self) -> Result<f32, Error> {
        let p = self.pos;
        let b = self.read()?;
        if 0xf9 != b {
            return Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected f16"))
        }
        Ok(half::f16::from_bits(u16::from_be_bytes(self.read_array()?)).to_f32())
    }

    /// Decode an `f32` value.
    pub fn f32(&mut self) -> Result<f32, Error> {
        let p = self.pos;
        match self.current()? {
            #[cfg(feature = "half")]
            0xf9 => self.f16(),
            0xfa => {
                self.read()?;
                Ok(f32::from_be_bytes(self.read_array()?))
            }
            b => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected f32"))
        }
    }

    /// Decode an `f64` value.
    pub fn f64(&mut self) -> Result<f64, Error> {
        let p = self.pos;
        match self.current()? {
            #[cfg(feature = "half")]
            0xf9 => self.f16().map(f64::from),
            0xfa => self.f32().map(f64::from),
            0xfb => {
                self.read()?;
                Ok(f64::from_be_bytes(self.read_array()?))
            }
            b => Err(Error::type_mismatch(self.type_of(b)?).at(p).with_message("expected f64"))
        }
    }

    /// Decode a `char` value.
    pub fn char(&mut self) -> Result<char, Error> {
        let p = self.pos;
        let n = self.u32()?;
        char::from_u32(n).ok_or_else(|| Error::invalid_char(n).at(p))
    }

    /// Decode a byte slice.
    ///
    /// This only decodes byte slices of definite lengths.
    /// See [`Decoder::bytes_iter`] for indefinite byte slice support.
    pub fn bytes(&mut self) -> Result<&'b [u8], Error> {
        let p = self.pos;
        let b = self.read()?;
        if BYTES != type_of(b) || info_of(b) == 31 {
            return Err(Error::type_mismatch(self.type_of(b)?)
                .with_message("expected bytes (definite length)")
                .at(p))
        }
        let n = u64_to_usize(self.unsigned(info_of(b), p)?, p)?;
        self.read_slice(n)
    }

    /// Iterate over byte slices.
    ///
    /// This supports indefinite byte slices by returing a byte slice on each
    /// iterator step. If a single definite slice is decoded the iterator will
    /// only yield one item.
    pub fn bytes_iter(&mut self) -> Result<BytesIter<'_, 'b>, Error> {
        let p = self.pos;
        let b = self.read()?;
        if BYTES != type_of(b) {
            return Err(Error::type_mismatch(self.type_of(b)?)
                .with_message("expected bytes")
                .at(p))
        }
        match info_of(b) {
            31 => Ok(BytesIter { decoder: self, len: None }),
            n  => {
                let len = u64_to_usize(self.unsigned(n, p)?, p)?;
                Ok(BytesIter { decoder: self, len: Some(len) })
            }
        }
    }

    /// Decode a string slice.
    ///
    /// This only decodes string slices of definite lengths.
    /// See [`Decoder::str_iter`] for indefinite string slice support.
    pub fn str(&mut self) -> Result<&'b str, Error> {
        let p = self.pos;
        let b = self.read()?;
        if TEXT != type_of(b) || info_of(b) == 31 {
            return Err(Error::type_mismatch(self.type_of(b)?)
                .with_message("expected text (definite length)")
                .at(p))
        }
        let n = u64_to_usize(self.unsigned(info_of(b), p)?, p)?;
        let d = self.read_slice(n)?;
        str::from_utf8(d).map_err(|e| Error::utf8(e).at(p))
    }

    /// Iterate over string slices.
    ///
    /// This supports indefinite string slices by returing a string slice on
    /// each iterator step. If a single definite slice is decoded the iterator
    /// will only yield one item.
    pub fn str_iter(&mut self) -> Result<StrIter<'_, 'b>, Error> {
        let p = self.pos;
        let b = self.read()?;
        if TEXT != type_of(b) {
            return Err(Error::type_mismatch(self.type_of(b)?)
                .with_message("expected text")
                .at(p))
        }
        match info_of(b) {
            31 => Ok(StrIter { decoder: self, len: None, pos: p }),
            n  => {
                let len = u64_to_usize(self.unsigned(n, p)?, p)?;
                Ok(StrIter { decoder: self, len: Some(len), pos: p })
            }
        }
    }

    /// Begin decoding an array.
    ///
    /// CBOR arrays are heterogenous collections and may be of indefinite
    /// length. If the length is known it is returned as a `Some`, for
    /// indefinite arrays a `None` is returned.
    pub fn array(&mut self) -> Result<Option<u64>, Error> {
        let p = self.pos;
        let b = self.read()?;
        if ARRAY != type_of(b) {
            return Err(Error::type_mismatch(self.type_of(b)?)
                .with_message("expected array")
                .at(p))
        }
        match info_of(b) {
            31 => Ok(None),
            n  => Ok(Some(self.unsigned(n, p)?))
        }
    }

    /// Iterate over all array elements.
    ///
    /// This supports indefinite and definite length arrays and uses the
    /// [`Decode`] trait to decode each element. Consequently *only
    /// homogenous arrays are supported by this method*.
    pub fn array_iter<T>(&mut self) -> Result<ArrayIter<'_, 'b, T>, Error>
    where
        T: Decode<'b, ()>
    {
        let len = self.array()?;
        Ok(ArrayIter { decoder: self, len, _mark: marker::PhantomData })
    }

    /// Iterate over all array elements.
    ///
    /// This supports indefinite and definite length arrays and uses the
    /// [`Decode`] trait to decode each element. Consequently *only
    /// homogenous arrays are supported by this method*.
    pub fn array_iter_with<'a, C, T>(&'a mut self, ctx: &'a mut C) -> Result<ArrayIterWithCtx<'a, 'b, C, T>, Error>
    where
        T: Decode<'b, C>
    {
        let len = self.array()?;
        Ok(ArrayIterWithCtx { decoder: self, ctx, len, _mark: marker::PhantomData })
    }

    /// Begin decoding a map.
    ///
    /// CBOR maps are heterogenous collections (both in keys and in values)
    /// and may be of indefinite length. If the length is known it is returned
    /// as a `Some`, for indefinite maps a `None` is returned.
    pub fn map(&mut self) -> Result<Option<u64>, Error> {
        let p = self.pos;
        let b = self.read()?;
        if MAP != type_of(b) {
            return Err(Error::type_mismatch(self.type_of(b)?)
                .with_message("expected map")
                .at(p))
        }
        match info_of(b) {
            31 => Ok(None),
            n  => Ok(Some(self.unsigned(n, p)?))
        }
    }

    /// Iterate over all map entries.
    ///
    /// This supports indefinite and definite length maps and uses the
    /// [`Decode`] trait to decode each key and value. Consequently *only
    /// homogenous maps are supported by this method*.
    pub fn map_iter<K, V>(&mut self) -> Result<MapIter<'_, 'b, K, V>, Error>
    where
        K: Decode<'b, ()>,
        V: Decode<'b, ()>
    {
        let len = self.map()?;
        Ok(MapIter { decoder: self, len, _mark: marker::PhantomData })
    }

    /// Iterate over all map entries.
    ///
    /// This supports indefinite and definite length maps and uses the
    /// [`Decode`] trait to decode each key and value. Consequently *only
    /// homogenous maps are supported by this method*.
    pub fn map_iter_with<'a, C, K, V>(&'a mut self, ctx: &'a mut C) -> Result<MapIterWithCtx<'a, 'b, C, K, V>, Error>
    where
        K: Decode<'b, C>,
        V: Decode<'b, C>
    {
        let len = self.map()?;
        Ok(MapIterWithCtx { decoder: self, ctx, len, _mark: marker::PhantomData })
    }

    /// Decode a CBOR tag.
    pub fn tag(&mut self) -> Result<Tag, Error> {
        let p = self.pos;
        let b = self.read()?;
        if TAGGED != type_of(b) {
            return Err(Error::type_mismatch(self.type_of(b)?)
                .with_message("expected tag")
                .at(p))
        }
        self.unsigned(info_of(b), p).map(Tag::new)
    }

    /// Decode a CBOR null value.
    pub fn null(&mut self) -> Result<(), Error> {
        let p = self.pos;
        match self.read()? {
            0xf6 => Ok(()),
            n    => Err(Error::type_mismatch(self.type_of(n)?)
                .with_message("expected null")
                .at(p))
        }
    }

    /// Decode a CBOR undefined value.
    pub fn undefined(&mut self) -> Result<(), Error> {
        let p = self.pos;
        match self.read()? {
            0xf7 => Ok(()),
            n    => Err(Error::type_mismatch(self.type_of(n)?)
                .with_message("expected undefined")
                .at(p))
        }
    }

    /// Decode a CBOR simple value.
    pub fn simple(&mut self) -> Result<u8, Error> {
        let p = self.pos;
        match self.read()? {
            n @ SIMPLE ..= 0xf3 => Ok(n - SIMPLE),
            0xf8 => self.read(),
            n    => Err(Error::type_mismatch(self.type_of(n)?)
                .with_message("expected simple value")
                .at(p))
        }
    }

    /// Inspect the CBOR type at the current position.
    pub fn datatype(&self) -> Result<Type, Error> {
        self.type_of(self.current()?)
    }

    /// Iterate over a series of CBOR tokens.
    #[cfg(feature = "half")]
    pub fn tokens<'a>(&'a mut self) -> crate::decode::Tokenizer<'a, 'b> {
        crate::decode::Tokenizer::from(self)
    }

    /// Skip over the current CBOR value.
    #[cfg(feature = "alloc")]
    pub fn skip(&mut self) -> Result<(), Error> {
        // Unless we encounter indefinite-length arrays or maps inside of regular
        // maps or arrays we only need to count how many more CBOR items we need
        // to skip (initially starting with 1) or how many more break bytes we
        // need to expect (initially starting with 0).
        //
        // If we do need to handle indefinite items (other than bytes or strings),
        // inside of regular maps or arrays, we switch to using a stack of length
        // information, starting with the remaining number of potential breaks we
        // are still expecting and the number of items we still need to skip over
        // at that point.

        let mut nrounds = 1u64; // number of iterations over array and map elements
        let mut irounds = 0u64; // number of indefinite iterations
        let mut stack: alloc::vec::Vec<Option<u64>> = alloc::vec::Vec::new();

        while nrounds > 0 || irounds > 0 || !stack.is_empty() {
            match self.current()? {
                UNSIGNED ..= 0x1b => { self.u64()?; }
                SIGNED   ..= 0x3b => { self.int()?; }
                BYTES    ..= 0x5f => { for v in self.bytes_iter()? { v?; } }
                TEXT     ..= 0x7f => { for v in self.str_iter()? { v?; } }
                ARRAY    ..= 0x9f =>
                    match self.array()? {
                        Some(0) => {}
                        Some(n) =>
                            if nrounds == 0 && irounds == 0 {
                                stack.push(Some(n))
                            } else {
                                nrounds = nrounds.saturating_add(n)
                            }
                        None =>
                            if nrounds == 0 && irounds == 0 {
                                stack.push(None)
                            } else if nrounds < 2 {
                                irounds = irounds.saturating_add(1)
                            } else {
                                for _ in 0 .. irounds {
                                    stack.push(None)
                                }
                                stack.push(Some(nrounds - 1));
                                stack.push(None);
                                nrounds = 0;
                                irounds = 0
                            }
                    }
                MAP ..= 0xbf =>
                    match self.map()? {
                        Some(0) => {}
                        Some(n) =>
                            if nrounds == 0 && irounds == 0 {
                                stack.push(Some(n.saturating_mul(2)))
                            } else {
                                nrounds = nrounds.saturating_add(n.saturating_mul(2))
                            }
                        None =>
                            if nrounds == 0 && irounds == 0 {
                                stack.push(None)
                            } else if nrounds < 2 {
                                irounds = irounds.saturating_add(1)
                            } else {
                                for _ in 0 .. irounds {
                                    stack.push(None)
                                }
                                stack.push(Some(nrounds - 1));
                                stack.push(None);
                                nrounds = 0;
                                irounds = 0
                            }
                    }
                TAGGED ..= 0xdb => {
                    let p = self.pos;
                    self.read().and_then(|n| self.unsigned(info_of(n), p))?;
                    continue
                }
                SIMPLE ..= 0xfb => {
                    let p = self.pos;
                    self.read().and_then(|n| self.unsigned(info_of(n), p))?;
                }
                BREAK => {
                    self.read()?;
                    if nrounds == 0 && irounds == 0 {
                        if let Some(None) = stack.last() {
                            stack.pop();
                        }
                    } else {
                        irounds = irounds.saturating_sub(1)
                    }
                }
                other => return Err(Error::type_mismatch(self.type_of(other)?)
                    .at(self.pos)
                    .with_message("unknown type"))
            }
            if nrounds == 0 && irounds == 0 {
                while let Some(Some(0)) = stack.last() {
                     stack.pop();
                }
                match stack.last_mut() {
                    Some(Some(n)) => { *n -= 1 }
                    Some(None)    => {}
                    None          => break
                }
            } else {
                nrounds = nrounds.saturating_sub(1)
            }
        }

        Ok(())
    }

    /// Skip over the current CBOR value.
    ///
    /// Without feature `alloc`, skipping over maps or arrays that contain an
    /// indefinite-length map or array will return an error.
    #[cfg(not(feature = "alloc"))]
    pub fn skip(&mut self) -> Result<(), Error> {
        let mut nrounds = 1u64; // number of iterations over array and map elements
        let mut irounds = 0u64; // number of indefinite iterations

        let error_msg =
            "arrays and maps of indefinite length inside of \
            regular arrays or maps require feature flag `alloc`";

        while nrounds > 0 || irounds > 0 {
            match self.current()? {
                UNSIGNED ..= 0x1b => { self.u64()?; }
                SIGNED   ..= 0x3b => { self.int()?; }
                BYTES    ..= 0x5f => { for v in self.bytes_iter()? { v?; } }
                TEXT     ..= 0x7f => { for v in self.str_iter()? { v?; } }
                ARRAY    ..= 0x9f =>
                    if let Some(n) = self.array()? {
                        nrounds = nrounds.saturating_add(n)
                    } else if nrounds < 2 {
                        irounds = irounds.saturating_add(1)
                    } else {
                        return Err(Error::message(error_msg))
                    }
                MAP ..= 0xbf =>
                    if let Some(n) = self.map()? {
                        nrounds = nrounds.saturating_add(n.saturating_mul(2))
                    } else if nrounds < 2 {
                        irounds = irounds.saturating_add(1)
                    } else {
                        return Err(Error::message(error_msg))
                    }
                TAGGED ..= 0xdb => {
                    let p = self.pos;
                    self.read().and_then(|n| self.unsigned(info_of(n), p))?;
                    continue
                }
                SIMPLE ..= 0xfb => {
                    let p = self.pos;
                    self.read().and_then(|n| self.unsigned(info_of(n), p))?;
                }
                BREAK => {
                    self.read()?;
                    irounds = irounds.saturating_sub(1)
                }
                other => return Err(Error::type_mismatch(self.type_of(other)?)
                    .at(self.pos)
                    .with_message("not supported"))
            }
            nrounds = nrounds.saturating_sub(1)
        }

        Ok(())
    }

    /// Decode a `u64` value beginning with `b`.
    pub(crate) fn unsigned(&mut self, b: u8, p: usize) -> Result<u64, Error> {
        match b {
            n @ 0 ..= 0x17 => Ok(u64::from(n)),
            0x18 => self.read().map(u64::from),
            0x19 => self.read_array().map(u16::from_be_bytes).map(u64::from),
            0x1a => self.read_array().map(u32::from_be_bytes).map(u64::from),
            0x1b => self.read_array().map(u64::from_be_bytes),
            _    => Err(Error::type_mismatch(self.type_of(b)?)
                .with_message("expected u64")
                .at(p))
        }
    }

    /// Get the byte at the current position.
    fn current(&self) -> Result<u8, Error> {
        if let Some(b) = self.buf.get(self.pos) {
            return Ok(*b)
        }
        Err(Error::end_of_input())
    }

    /// Consume and return the byte at the current position.
    fn read(&mut self) -> Result<u8, Error> {
        if let Some(b) = self.buf.get(self.pos) {
            self.pos += 1;
            return Ok(*b)
        }
        Err(Error::end_of_input())
    }

    /// Peek to the next byte.
    fn peek(&self) -> Result<u8, Error> {
        self.pos.checked_add(1)
            .and_then(|i| self.buf.get(i).copied())
            .ok_or_else(Error::end_of_input)
    }

    /// Consume and return *n* bytes starting at the current position.
    fn read_slice(&mut self, n: usize) -> Result<&'b [u8], Error> {
        if let Some(b) = self.pos.checked_add(n).and_then(|end| self.buf.get(self.pos .. end)) {
            self.pos += n;
            return Ok(b)
        }
        Err(Error::end_of_input())
    }

    /// Consume and return *N* bytes starting at the current position.
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        self.read_slice(N).map(|slice| {
            let mut a = [0; N];
            a.copy_from_slice(slice);
            a
        })
    }

    /// Map the given byte to a [`Type`].
    fn type_of(&self, n: u8) -> Result<Type, Error> {
        Ok(match n {
            0x00 ..= 0x18        => Type::U8,
            0x19                 => Type::U16,
            0x1a                 => Type::U32,
            0x1b                 => Type::U64,
            0x20 ..= 0x37        => Type::I8,
            0x38                 => if self.peek()? < 0x80 { Type::I8  } else { Type::I16 }
            0x39                 => if self.peek()? < 0x80 { Type::I16 } else { Type::I32 }
            0x3a                 => if self.peek()? < 0x80 { Type::I32 } else { Type::I64 }
            0x3b                 => if self.peek()? < 0x80 { Type::I64 } else { Type::Int }
            0x40 ..= 0x5b        => Type::Bytes,
            0x5f                 => Type::BytesIndef,
            0x60 ..= 0x7b        => Type::String,
            0x7f                 => Type::StringIndef,
            0x80 ..= 0x9b        => Type::Array,
            0x9f                 => Type::ArrayIndef,
            0xa0 ..= 0xbb        => Type::Map,
            0xbf                 => Type::MapIndef,
            0xc0 ..= 0xdb        => Type::Tag,
            0xe0 ..= 0xf3 | 0xf8 => Type::Simple,
            0xf4 | 0xf5          => Type::Bool,
            0xf6                 => Type::Null,
            0xf7                 => Type::Undefined,
            0xf9                 => Type::F16,
            0xfa                 => Type::F32,
            0xfb                 => Type::F64,
            0xff                 => Type::Break,
            n                    => Type::Unknown(n)
        })
    }
}

/// An iterator over byte slices.
///
/// Returned from [`Decoder::bytes_iter`].
#[derive(Debug)]
pub struct BytesIter<'a, 'b> {
    decoder: &'a mut Decoder<'b>,
    len: Option<usize>
}

impl<'a, 'b> Iterator for BytesIter<'a, 'b> {
    type Item = Result<&'b [u8], Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.len {
            None => match self.decoder.current() {
                Ok(BREAK) => self.decoder.read().map(|_| None).transpose(),
                Ok(_)     => Some(self.decoder.bytes()),
                Err(e)    => Some(Err(e))
            }
            Some(0) => None,
            Some(n) => {
                self.len = Some(0);
                Some(self.decoder.read_slice(n))
            }
        }
    }
}

/// An iterator over string slices.
///
/// Returned from [`Decoder::str_iter`].
#[derive(Debug)]
pub struct StrIter<'a, 'b> {
    decoder: &'a mut Decoder<'b>,
    len: Option<usize>,
    pos: usize
}

impl<'a, 'b> Iterator for StrIter<'a, 'b> {
    type Item = Result<&'b str, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.len {
            None => match self.decoder.current() {
                Ok(BREAK) => self.decoder.read().map(|_| None).transpose(),
                Ok(_)     => Some(self.decoder.str()),
                Err(e)    => Some(Err(e))
            }
            Some(0) => None,
            Some(n) => {
                self.len = Some(0);
                Some(self.decoder.read_slice(n).and_then(|d| str::from_utf8(d).map_err(|e| Error::utf8(e).at(self.pos))))
            }
        }
    }
}

/// An iterator over array elements.
///
/// Returned from [`Decoder::array_iter`].
#[derive(Debug)]
pub struct ArrayIter<'a, 'b, T> {
    decoder: &'a mut Decoder<'b>,
    len: Option<u64>,
    _mark: marker::PhantomData<fn(T)>
}

impl<'a, 'b, T: Decode<'b, ()>> Iterator for ArrayIter<'a, 'b, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.len {
            None => match self.decoder.current() {
                Ok(BREAK) => self.decoder.read().map(|_| None).transpose(),
                Ok(_)     => Some(T::decode(self.decoder, &mut ())),
                Err(e)    => Some(Err(e))
            }
            Some(0) => None,
            Some(n) => {
                self.len = Some(n - 1);
                Some(T::decode(self.decoder, &mut ()))
            }
        }
    }
}

/// An iterator over array elements.
///
/// Returned from [`Decoder::array_iter_with`].
#[derive(Debug)]
pub struct ArrayIterWithCtx<'a, 'b, C, T> {
    decoder: &'a mut Decoder<'b>,
    ctx: &'a mut C,
    len: Option<u64>,
    _mark: marker::PhantomData<fn(T)>
}

impl<'a, 'b, C, T: Decode<'b, C>> Iterator for ArrayIterWithCtx<'a, 'b, C, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.len {
            None => match self.decoder.current() {
                Ok(BREAK) => self.decoder.read().map(|_| None).transpose(),
                Ok(_)     => Some(T::decode(self.decoder, self.ctx)),
                Err(e)    => Some(Err(e))
            }
            Some(0) => None,
            Some(n) => {
                self.len = Some(n - 1);
                Some(T::decode(self.decoder, self.ctx))
            }
        }
    }
}

/// An iterator over map entries.
///
/// Returned from [`Decoder::map_iter`].
#[derive(Debug)]
pub struct MapIter<'a, 'b, K, V> {
    decoder: &'a mut Decoder<'b>,
    len: Option<u64>,
    _mark: marker::PhantomData<fn(K, V)>
}

impl<'a, 'b, K, V> Iterator for MapIter<'a, 'b, K, V>
where
    K: Decode<'b, ()>,
    V: Decode<'b, ()>
{
    type Item = Result<(K, V), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        fn pair<'b, K, V>(d: &mut Decoder<'b>) -> Result<(K, V), Error>
        where
            K: Decode<'b, ()>,
            V: Decode<'b, ()>
        {
            Ok((K::decode(d, &mut ())?, V::decode(d, &mut ())?))
        }
        match self.len {
            None => match self.decoder.current() {
                Ok(BREAK) => self.decoder.read().map(|_| None).transpose(),
                Ok(_)  => Some(pair(self.decoder)),
                Err(e) => Some(Err(e))
            }
            Some(0) => None,
            Some(n) => {
                self.len = Some(n - 1);
                Some(pair(self.decoder))
            }
        }
    }
}

/// An iterator over map entries.
///
/// Returned from [`Decoder::map_iter_with`].
#[derive(Debug)]
pub struct MapIterWithCtx<'a, 'b, C, K, V> {
    decoder: &'a mut Decoder<'b>,
    ctx: &'a mut C,
    len: Option<u64>,
    _mark: marker::PhantomData<fn(K, V)>
}

impl<'a, 'b, C, K, V> Iterator for MapIterWithCtx<'a, 'b, C, K, V>
where
    K: Decode<'b, C>,
    V: Decode<'b, C>
{
    type Item = Result<(K, V), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        fn pair<'b, C, K, V>(d: &mut Decoder<'b>, ctx: &mut C) -> Result<(K, V), Error>
        where
            K: Decode<'b, C>,
            V: Decode<'b, C>
        {
            Ok((K::decode(d, ctx)?, V::decode(d, ctx)?))
        }
        match self.len {
            None => match self.decoder.current() {
                Ok(BREAK) => self.decoder.read().map(|_| None).transpose(),
                Ok(_)  => Some(pair(self.decoder, self.ctx)),
                Err(e) => Some(Err(e))
            }
            Some(0) => None,
            Some(n) => {
                self.len = Some(n - 1);
                Some(pair(self.decoder, self.ctx))
            }
        }
    }
}

/// A decoding probe to to look ahead what comes next.
///
/// A `Probe` derefs to [`Decoder`] and thus can be used like one without
/// affecting the decoder from which it was created.
//
// The current implementation just clones `Decoder` as it is very cheap
// to do so. `Probe` is nevertheless introduced to discourage use of
// `Decoder::clone` in client code for this purpose so that it stays
// independent of the current implementation.
// With a more heavyweight `Decoder`, `Probe` could only store a reference
// and the current position which it restores in a `Drop` impl.
#[derive(Debug)]
pub struct Probe<'a, 'b> {
    decoder: Decoder<'b>,
    _marker: marker::PhantomData<&'a mut ()>
}

impl<'b> core::ops::Deref for Probe<'_, 'b> {
    type Target = Decoder<'b>;

    fn deref(&self) -> &Self::Target {
        &self.decoder
    }
}

impl<'b> core::ops::DerefMut for Probe<'_, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.decoder
    }
}

/// Get the major type info of the given byte (highest 3 bits).
pub(crate) fn type_of(b: u8) -> u8 {
    b & 0b111_00000
}

/// Get the additionl type info of the given byte (lowest 5 bits).
pub(crate) fn info_of(b: u8) -> u8 {
    b & 0b000_11111
}

fn u64_to_usize(n: u64, pos: usize) -> Result<usize, Error> {
    n.try_into().map_err(|_| Error::overflow(n).at(pos).with_message("when converting u64 to usize"))
}

fn try_as<A, B>(val: A, msg: &'static str, pos: usize) -> Result<B, Error>
where
    A: TryInto<B> + Into<u64> + Copy
{
    val.try_into().map_err(|_| Error::overflow(val.into()).at(pos).with_message(msg))
}

