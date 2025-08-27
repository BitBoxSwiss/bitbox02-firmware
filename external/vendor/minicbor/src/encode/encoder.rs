use crate::{SIGNED, BYTES, TEXT, ARRAY, MAP, TAGGED, SIMPLE};
use crate::data::{Int, Tag};
use crate::encode::{Encode, Error, Write};

/// A non-allocating CBOR encoder writing encoded bytes to the given [`Write`] sink.
#[derive(Debug, Clone)]
pub struct Encoder<W> { writer: W }

impl<W: Write> Encoder<W> {
    /// Construct an `Encoder` that writes to the given [`Write`] sink.
    pub fn new(writer: W) -> Encoder<W> {
        Encoder { writer }
    }

    /// Access the inner writer.
    pub fn writer(&self) -> &W {
        &self.writer
    }

    /// Get mutable access to the inner writer.
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Get back the [`Write`] impl.
    pub fn into_writer(self) -> W {
        self.writer
    }

    /// Encode any type that implements [`Encode`].
    pub fn encode<T: Encode<()>>(&mut self, x: T) -> Result<&mut Self, Error<W::Error>> {
        x.encode(self, &mut ())?;
        Ok(self)
    }

    /// Encode any type that implements [`Encode`].
    pub fn encode_with<C, T: Encode<C>>(&mut self, x: T, ctx: &mut C) -> Result<&mut Self, Error<W::Error>> {
        x.encode(self, ctx)?;
        Ok(self)
    }

    /// Encode a `u8` value.
    pub fn u8(&mut self, x: u8) -> Result<&mut Self, Error<W::Error>> {
        if let 0 ..= 0x17 = x {
            self.put(&[x])
        } else {
            self.put(&[24, x])
        }
    }

    /// Encode an `i8` value.
    pub fn i8(&mut self, x: i8) -> Result<&mut Self, Error<W::Error>> {
        if x >= 0 {
            return self.u8(x as u8)
        }
        match (-1 - x) as u8 {
            n @ 0 ..= 0x17 => self.put(&[SIGNED | n]),
            n              => self.put(&[SIGNED | 24, n])
        }
    }

    /// Encode a `u16` value.
    pub fn u16(&mut self, x: u16) -> Result<&mut Self, Error<W::Error>> {
        match x {
            0    ..= 0x17 => self.put(&[x as u8]),
            0x18 ..= 0xff => self.put(&[24, x as u8]),
            _             => self.put(&[25])?.put(&x.to_be_bytes()[..])
        }
    }

    /// Encode an `i16` value.
    pub fn i16(&mut self, x: i16) -> Result<&mut Self, Error<W::Error>> {
        if x >= 0 {
            return self.u16(x as u16)
        }
        match (-1 - x) as u16 {
            n @ 0    ..= 0x17 => self.put(&[SIGNED | n as u8]),
            n @ 0x18 ..= 0xff => self.put(&[SIGNED | 24, n as u8]),
            n                 => self.put(&[SIGNED | 25])?.put(&n.to_be_bytes()[..])
        }
    }

    /// Encode a `u32` value.
    pub fn u32(&mut self, x: u32) -> Result<&mut Self, Error<W::Error>> {
        match x {
            0     ..= 0x17   => self.put(&[x as u8]),
            0x18  ..= 0xff   => self.put(&[24, x as u8]),
            0x100 ..= 0xffff => self.put(&[25])?.put(&(x as u16).to_be_bytes()[..]),
            _                => self.put(&[26])?.put(&x.to_be_bytes()[..])
        }
    }

    /// Encode an `i32` value.
    pub fn i32(&mut self, x: i32) -> Result<&mut Self, Error<W::Error>> {
        if x >= 0 {
            return self.u32(x as u32)
        }
        match (-1 - x) as u32 {
            n @ 0     ..= 0x17   => self.put(&[SIGNED | n as u8]),
            n @ 0x18  ..= 0xff   => self.put(&[SIGNED | 24, n as u8]),
            n @ 0x100 ..= 0xffff => self.put(&[SIGNED | 25])?.put(&(n as u16).to_be_bytes()[..]),
            n                    => self.put(&[SIGNED | 26])?.put(&n.to_be_bytes()[..])
        }
    }

    /// Encode a `u64` value.
    pub fn u64(&mut self, x: u64) -> Result<&mut Self, Error<W::Error>> {
        match x {
            0        ..= 0x17        => self.put(&[x as u8]),
            0x18     ..= 0xff        => self.put(&[24, x as u8]),
            0x100    ..= 0xffff      => self.put(&[25])?.put(&(x as u16).to_be_bytes()[..]),
            0x1_0000 ..= 0xffff_ffff => self.put(&[26])?.put(&(x as u32).to_be_bytes()[..]),
            _                        => self.put(&[27])?.put(&x.to_be_bytes()[..])
        }
    }

    /// Encode an `i64` value.
    pub fn i64(&mut self, x: i64) -> Result<&mut Self, Error<W::Error>> {
        if x >= 0 {
            return self.u64(x as u64)
        }
        match (-1 - x) as u64 {
            n @ 0        ..= 0x17        => self.put(&[SIGNED | n as u8]),
            n @ 0x18     ..= 0xff        => self.put(&[SIGNED | 24, n as u8]),
            n @ 0x100    ..= 0xffff      => self.put(&[SIGNED | 25])?.put(&(n as u16).to_be_bytes()[..]),
            n @ 0x1_0000 ..= 0xffff_ffff => self.put(&[SIGNED | 26])?.put(&(n as u32).to_be_bytes()[..]),
            n                            => self.put(&[SIGNED | 27])?.put(&n.to_be_bytes()[..])
        }
    }

    /// Encode a CBOR integer.
    ///
    /// See [`Int`] for details regarding the value range of CBOR integers.
    pub fn int(&mut self, x: Int) -> Result<&mut Self, Error<W::Error>> {
        if !x.is_negative() {
            return self.u64(x.value())
        }
        match x.value() {
            n @ 0        ..= 0x17        => self.put(&[SIGNED | n as u8]),
            n @ 0x18     ..= 0xff        => self.put(&[SIGNED | 24, n as u8]),
            n @ 0x100    ..= 0xffff      => self.put(&[SIGNED | 25])?.put(&(n as u16).to_be_bytes()[..]),
            n @ 0x1_0000 ..= 0xffff_ffff => self.put(&[SIGNED | 26])?.put(&(n as u32).to_be_bytes()[..]),
            n                            => self.put(&[SIGNED | 27])?.put(&n.to_be_bytes()[..])
        }
    }

    /// Encode a CBOR `null` value.
    pub fn null(&mut self) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[SIMPLE | 22])
    }

    /// Encode a CBOR `undefined` value.
    pub fn undefined(&mut self) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[SIMPLE | 23])
    }

    /// Encode a CBOR simple value.
    pub fn simple(&mut self, x: u8) -> Result<&mut Self, Error<W::Error>> {
        if x < 0x14 {
            self.put(&[SIMPLE | x])
        } else {
            self.put(&[SIMPLE | 24, x])
        }
    }

    /// Encode an `f32` value as a half float (`f16)`.
    ///
    /// *Requires feature* `"half"`.
    ///
    /// **NB**: The conversion from `f32` to `f16` is potentially lossy.
    /// Generally values are truncated and rounded to the nearest 16-bit
    /// value, except:
    ///
    ///   - 32-bit values which do not fit into 16 bit become ±∞.
    ///   - 32-bit subnormal values become ±0.
    ///   - Exponents smaller than the min. 16-bit exponent become
    ///     16-bit subnormals or ±0.
    ///
    /// For further details please consult the [half][1] crate which is
    /// used internally for `f16` support.
    ///
    /// [1]: https://crates.io/crates/half
    #[cfg(feature = "half")]
    pub fn f16(&mut self, x: f32) -> Result<&mut Self, Error<W::Error>> {
        let n = half::f16::from_f32(x).to_bits();
        self.put(&[SIMPLE | 25])?.put(&n.to_be_bytes()[..])
    }

    /// Encode an `f32` value.
    pub fn f32(&mut self, x: f32) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[SIMPLE | 26])?.put(&x.to_be_bytes()[..])
    }

    /// Encode an `f64` value.
    pub fn f64(&mut self, x: f64) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[SIMPLE | 27])?.put(&x.to_be_bytes()[..])
    }

    /// Encode a `bool` value.
    pub fn bool(&mut self, x: bool) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[SIMPLE | if x { 0x15 } else { 0x14 }])
    }

    /// Encode a `char` value.
    pub fn char(&mut self, x: char) -> Result<&mut Self, Error<W::Error>> {
        self.u32(u32::from(x))
    }

    /// Encode a CBOR tag.
    pub fn tag<T: Into<Tag>>(&mut self, x: T) -> Result<&mut Self, Error<W::Error>> {
        self.type_len(TAGGED, x.into().into())
    }

    /// Encode a byte slice.
    pub fn bytes(&mut self, x: &[u8]) -> Result<&mut Self, Error<W::Error>> {
        self.type_len(BYTES, x.len() as u64)?.put(x)
    }

    /// Encode a string slice.
    pub fn str(&mut self, x: &str) -> Result<&mut Self, Error<W::Error>> {
        self.type_len(TEXT, x.len() as u64)?.put(x.as_bytes())
    }

    /// Begin encoding an array with `len` elements.
    pub fn array(&mut self, len: u64) -> Result<&mut Self, Error<W::Error>> {
        self.type_len(ARRAY, len)
    }

    /// Begin encoding a map with `len` entries.
    pub fn map(&mut self, len: u64) -> Result<&mut Self, Error<W::Error>> {
        self.type_len(MAP, len)
    }

    /// Begin encoding an array of unknown size.
    ///
    /// Use [`Encoder::end`] to terminate the array.
    pub fn begin_array(&mut self) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[0x9f])
    }

    /// Begin encoding an indefinite number of byte slices.
    ///
    /// Use [`Encoder::end`] to terminate.
    pub fn begin_bytes(&mut self) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[0x5f])
    }

    /// Begin encoding a map of unknown size.
    ///
    /// Use [`Encoder::end`] to terminate the map.
    pub fn begin_map(&mut self) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[0xbf])
    }

    /// Begin encoding an indefinite number of string slices.
    ///
    /// Use [`Encoder::end`] to terminate.
    pub fn begin_str(&mut self) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[0x7f])
    }

    /// Terminate an indefinite collection.
    pub fn end(&mut self) -> Result<&mut Self, Error<W::Error>> {
        self.put(&[0xff])
    }

    /// Syntactic sugar for `Ok(())`.
    pub fn ok(&mut self) -> Result<(), Error<W::Error>> {
        Ok(())
    }

    /// Encode a sequence of CBOR tokens.
    #[cfg(feature = "half")]
    pub fn tokens<'a, 'b: 'a, I>(&mut self, tokens: I) -> Result<(), Error<W::Error>>
    where
        I: IntoIterator<Item = &'a crate::data::Token<'b>>
    {
        for t in tokens {
            self.encode(t)?;
        }
        Ok(())
    }

    /// Write the encoded byte slice.
    pub(crate) fn put(&mut self, b: &[u8]) -> Result<&mut Self, Error<W::Error>> {
        self.writer.write_all(b).map_err(Error::write)?;
        Ok(self)
    }

    /// Write type and length information.
    fn type_len(&mut self, t: u8, x: u64) -> Result<&mut Self, Error<W::Error>> {
        match x {
            0        ..= 0x17        => self.put(&[t | x as u8]),
            0x18     ..= 0xff        => self.put(&[t | 24, x as u8]),
            0x100    ..= 0xffff      => self.put(&[t | 25])?.put(&(x as u16).to_be_bytes()[..]),
            0x1_0000 ..= 0xffff_ffff => self.put(&[t | 26])?.put(&(x as u32).to_be_bytes()),
            _                        => self.put(&[t | 27])?.put(&x.to_be_bytes()[..])
        }
    }
}

