//! CBOR data types, tokens and tags.

#[cfg(feature = "half")]
mod token;

use core::fmt;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "half")]
pub use token::Token;

/// CBOR data types.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Type {
    Bool,
    Null,
    Undefined,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Int,
    F16,
    F32,
    F64,
    Simple,
    Bytes,
    BytesIndef,
    String,
    StringIndef,
    Array,
    ArrayIndef,
    Map,
    MapIndef,
    Tag,
    Break,
    Unknown(u8)
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool        => f.write_str("bool"),
            Type::Null        => f.write_str("null"),
            Type::Undefined   => f.write_str("undefined"),
            Type::U8          => f.write_str("u8"),
            Type::U16         => f.write_str("u16"),
            Type::U32         => f.write_str("u32"),
            Type::U64         => f.write_str("u64"),
            Type::I8          => f.write_str("i8"),
            Type::I16         => f.write_str("i16"),
            Type::I32         => f.write_str("i32"),
            Type::I64         => f.write_str("i64"),
            Type::Int         => f.write_str("int"),
            Type::F16         => f.write_str("f16"),
            Type::F32         => f.write_str("f32"),
            Type::F64         => f.write_str("f64"),
            Type::Simple      => f.write_str("simple"),
            Type::Bytes       => f.write_str("bytes"),
            Type::BytesIndef  => f.write_str("indefinite bytes"),
            Type::String      => f.write_str("string"),
            Type::StringIndef => f.write_str("indefinite string"),
            Type::Array       => f.write_str("array"),
            Type::ArrayIndef  => f.write_str("indefinite array"),
            Type::Map         => f.write_str("map"),
            Type::MapIndef    => f.write_str("indefinite map"),
            Type::Tag         => f.write_str("tag"),
            Type::Break       => f.write_str("break"),
            Type::Unknown(n)  => write!(f, "{:#x}", n)
        }
    }
}

/// CBOR data item tag.
///
/// See [`IanaTag`] for currently known tag values which have been registered.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Tag(u64);

impl Tag {
    pub const fn new(n: u64) -> Self {
        Self(n)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<Tag> for u64 {
    fn from(t: Tag) -> Self {
        t.0
    }
}

impl From<&Tag> for u64 {
    fn from(t: &Tag) -> Self {
        t.0
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// IANA registered tags.
///
/// See <https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml> for details.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[non_exhaustive]
pub enum IanaTag {
    DateTime,
    Timestamp,
    PosBignum,
    NegBignum,
    Decimal,
    Bigfloat,
    ToBase64Url,
    ToBase64,
    ToBase16,
    Cbor,
    Uri,
    Base64Url,
    Base64,
    Regex,
    Mime,
    // Typed arrays (RFC 8746):
    HomogenousArray,
    TypedArrayU8,
    TypedArrayU8Clamped,
    TypedArrayU16B,
    TypedArrayU32B,
    TypedArrayU64B,
    TypedArrayU16L,
    TypedArrayU32L,
    TypedArrayU64L,
    TypedArrayI8,
    TypedArrayI16B,
    TypedArrayI32B,
    TypedArrayI64B,
    TypedArrayI16L,
    TypedArrayI32L,
    TypedArrayI64L,
    TypedArrayF16B,
    TypedArrayF32B,
    TypedArrayF64B,
    TypedArrayF128B,
    TypedArrayF16L,
    TypedArrayF32L,
    TypedArrayF64L,
    TypedArrayF128L,
    MultiDimArrayR, // row-major order
    MultiDimArrayC, // column-major order
}

impl IanaTag {
    pub fn tag(self) -> Tag {
        self.into()
    }
}

impl TryFrom<Tag> for IanaTag {
    type Error = UnknownTag;

    fn try_from(t: Tag) -> Result<Self, Self::Error> {
        match t.into() {
            0x00  => Ok(Self::DateTime),
            0x01  => Ok(Self::Timestamp),
            0x02  => Ok(Self::PosBignum),
            0x03  => Ok(Self::NegBignum),
            0x04  => Ok(Self::Decimal),
            0x05  => Ok(Self::Bigfloat),
            0x15  => Ok(Self::ToBase64Url),
            0x16  => Ok(Self::ToBase64),
            0x17  => Ok(Self::ToBase16),
            0x18  => Ok(Self::Cbor),
            0x20  => Ok(Self::Uri),
            0x21  => Ok(Self::Base64Url),
            0x22  => Ok(Self::Base64),
            0x23  => Ok(Self::Regex),
            0x24  => Ok(Self::Mime),
            0x28  => Ok(Self::MultiDimArrayR),
            0x29  => Ok(Self::HomogenousArray),
            0x40  => Ok(Self::TypedArrayU8),
            0x41  => Ok(Self::TypedArrayU16B),
            0x42  => Ok(Self::TypedArrayU32B),
            0x43  => Ok(Self::TypedArrayU64B),
            0x44  => Ok(Self::TypedArrayU8Clamped),
            0x45  => Ok(Self::TypedArrayU16L),
            0x46  => Ok(Self::TypedArrayU32L),
            0x47  => Ok(Self::TypedArrayU64L),
            0x48  => Ok(Self::TypedArrayI8),
            0x49  => Ok(Self::TypedArrayI16B),
            0x4a  => Ok(Self::TypedArrayI32B),
            0x4b  => Ok(Self::TypedArrayI64B),
            0x4d  => Ok(Self::TypedArrayI16L),
            0x4e  => Ok(Self::TypedArrayI32L),
            0x4f  => Ok(Self::TypedArrayI64L),
            0x50  => Ok(Self::TypedArrayF16B),
            0x51  => Ok(Self::TypedArrayF32B),
            0x52  => Ok(Self::TypedArrayF64B),
            0x53  => Ok(Self::TypedArrayF128B),
            0x54  => Ok(Self::TypedArrayF16L),
            0x55  => Ok(Self::TypedArrayF32L),
            0x56  => Ok(Self::TypedArrayF64L),
            0x57  => Ok(Self::TypedArrayF128L),
            0x410 => Ok(Self::MultiDimArrayC),
            _     => Err(UnknownTag(t))
        }
    }
}

impl From<IanaTag> for Tag {
    fn from(t: IanaTag) -> Tag {
        match t {
            IanaTag::DateTime            => Tag::new(0x00),
            IanaTag::Timestamp           => Tag::new(0x01),
            IanaTag::PosBignum           => Tag::new(0x02),
            IanaTag::NegBignum           => Tag::new(0x03),
            IanaTag::Decimal             => Tag::new(0x04),
            IanaTag::Bigfloat            => Tag::new(0x05),
            IanaTag::ToBase64Url         => Tag::new(0x15),
            IanaTag::ToBase64            => Tag::new(0x16),
            IanaTag::ToBase16            => Tag::new(0x17),
            IanaTag::Cbor                => Tag::new(0x18),
            IanaTag::Uri                 => Tag::new(0x20),
            IanaTag::Base64Url           => Tag::new(0x21),
            IanaTag::Base64              => Tag::new(0x22),
            IanaTag::Regex               => Tag::new(0x23),
            IanaTag::Mime                => Tag::new(0x24),
            IanaTag::MultiDimArrayR      => Tag::new(0x28),
            IanaTag::HomogenousArray     => Tag::new(0x29),
            IanaTag::TypedArrayU8        => Tag::new(0x40),
            IanaTag::TypedArrayU16B      => Tag::new(0x41),
            IanaTag::TypedArrayU32B      => Tag::new(0x42),
            IanaTag::TypedArrayU64B      => Tag::new(0x43),
            IanaTag::TypedArrayU8Clamped => Tag::new(0x44),
            IanaTag::TypedArrayU16L      => Tag::new(0x45),
            IanaTag::TypedArrayU32L      => Tag::new(0x46),
            IanaTag::TypedArrayU64L      => Tag::new(0x47),
            IanaTag::TypedArrayI8        => Tag::new(0x48),
            IanaTag::TypedArrayI16B      => Tag::new(0x49),
            IanaTag::TypedArrayI32B      => Tag::new(0x4a),
            IanaTag::TypedArrayI64B      => Tag::new(0x4b),
            IanaTag::TypedArrayI16L      => Tag::new(0x4d),
            IanaTag::TypedArrayI32L      => Tag::new(0x4e),
            IanaTag::TypedArrayI64L      => Tag::new(0x4f),
            IanaTag::TypedArrayF16B      => Tag::new(0x50),
            IanaTag::TypedArrayF32B      => Tag::new(0x51),
            IanaTag::TypedArrayF64B      => Tag::new(0x52),
            IanaTag::TypedArrayF128B     => Tag::new(0x53),
            IanaTag::TypedArrayF16L      => Tag::new(0x54),
            IanaTag::TypedArrayF32L      => Tag::new(0x55),
            IanaTag::TypedArrayF64L      => Tag::new(0x56),
            IanaTag::TypedArrayF128L     => Tag::new(0x57),
            IanaTag::MultiDimArrayC      => Tag::new(0x410)
        }
    }
}

impl From<&IanaTag> for Tag {
    fn from(t: &IanaTag) -> Tag {
        t.into()
    }
}

impl From<IanaTag> for u64 {
    fn from(t: IanaTag) -> Self {
        Tag::from(t).into()
    }
}

impl From<&IanaTag> for u64 {
    fn from(t: &IanaTag) -> Self {
        Tag::from(t).into()
    }
}

/// Error indicating that a tag value is unknown to [`IanaTag`].
#[derive(Debug)]
pub struct UnknownTag(Tag);

impl fmt::Display for UnknownTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown tag: {:#x}", self.0.as_u64())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnknownTag {}


/// Statically tag a value.
///
/// # Example
///
/// ```
/// use minicbor::data::{IanaTag, Tagged};
///
/// let input = [
///     0xc0, 0x74, 0x32, 0x30, 0x31, 0x33, 0x2d, 0x30,
///     0x33, 0x2d, 0x32, 0x31, 0x54, 0x32, 0x30, 0x3a,
///     0x30, 0x34, 0x3a, 0x30, 0x30, 0x5a
/// ];
///
/// let date_time: Tagged<0, &str> = minicbor::decode(&input)?;
/// assert_eq!(date_time.tag(), IanaTag::DateTime.tag());
/// assert_eq!(date_time.value(), &"2013-03-21T20:04:00Z");
///
/// # Ok::<_, Box<dyn std::error::Error>>(())
///
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tagged<const N: u64, T>(T);

impl<const N: u64, T> Tagged<N, T> {
    pub const TAG: Tag = Tag::new(N);

    pub const fn new(val: T) -> Self {
        Self(val)
    }

    pub const fn tag(&self) -> Tag {
        Self::TAG
    }

    pub const fn value(&self) -> &T {
        &self.0
    }

    pub fn into_value(self) -> T {
        self.0
    }
}

impl<const N: u64, T> From<T> for Tagged<N, T> {
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

impl<const N: u64, T> Deref for Tagged<N, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: u64, T> DerefMut for Tagged<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// CBOR integer type that covers values of [-2<sup>64</sup>, 2<sup>64</sup> - 1]
///
/// CBOR integers keep the sign bit in the major type so there is one extra bit
/// available for signed numbers compared to Rust's integer types. This type can
/// be used to encode and decode the full CBOR integer range.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int { neg: bool, val: u64 }

/// Max. CBOR integer value (2<sup>64</sup> - 1).
pub const MAX_INT: Int = Int { neg: false, val: u64::MAX };

/// Min. CBOR integer value (-2<sup>64</sup>).
pub const MIN_INT: Int = Int { neg: true, val: u64::MAX };

impl Int {
    pub(crate) fn pos<T: Into<u64>>(val: T) -> Self {
        Int { neg: false, val: val.into() }
    }

    pub(crate) fn neg<T: Into<u64>>(val: T) -> Self {
        Int { neg: true, val: val.into() }
    }

    pub(crate) fn value(&self) -> u64 {
        self.val
    }

    pub(crate) fn is_negative(&self) -> bool {
        self.neg
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", i128::from(*self))
    }
}

// Introductions:

impl From<u8> for Int {
    fn from(i: u8) -> Self {
        Int::from(u64::from(i))
    }
}

impl From<u16> for Int {
    fn from(i: u16) -> Self {
        Int::from(u64::from(i))
    }
}

impl From<u32> for Int {
    fn from(i: u32) -> Self {
        Int::from(u64::from(i))
    }
}

impl From<u64> for Int {
    fn from(i: u64) -> Self {
        Int::pos(i)
    }
}

impl TryFrom<u128> for Int {
    type Error = TryFromIntError;

    fn try_from(i: u128) -> Result<Self, Self::Error> {
        Ok(Int::from(u64::try_from(i).map_err(|_| TryFromIntError("u64"))?))
    }
}

impl From<i8> for Int {
    fn from(i: i8) -> Self {
        Int::from(i64::from(i))
    }
}

impl From<i16> for Int {
    fn from(i: i16) -> Self {
        Int::from(i64::from(i))
    }
}

impl From<i32> for Int {
    fn from(i: i32) -> Self {
        Int::from(i64::from(i))
    }
}

impl From<i64> for Int {
    fn from(i: i64) -> Self {
        if i.is_negative() {
            Int { neg: true, val: (-1 - i) as u64 }
        } else {
            Int { neg: false, val: i as u64 }
        }
    }
}

impl TryFrom<i128> for Int {
    type Error = TryFromIntError;

    fn try_from(i: i128) -> Result<Self, Self::Error> {
        if i.is_negative() {
            if i < -0x1_0000_0000_0000_0000 {
                Err(TryFromIntError("Int"))
            } else {
                Ok(Int { neg: true, val: (-1 - i) as u64 })
            }
        } else if i > 0xFFFF_FFFF_FFFF_FFFF {
            Err(TryFromIntError("Int"))
        } else {
            Ok(Int { neg: false, val: i as u64 })
        }
    }
}

// Eliminations:

impl TryFrom<Int> for u8 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        u64::try_from(i).and_then(|n| u8::try_from(n).map_err(|_| TryFromIntError("u8")))
    }
}

impl TryFrom<Int> for u16 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        u64::try_from(i).and_then(|n| u16::try_from(n).map_err(|_| TryFromIntError("u16")))
    }
}

impl TryFrom<Int> for u32 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        u64::try_from(i).and_then(|n| u32::try_from(n).map_err(|_| TryFromIntError("u32")))
    }
}

impl TryFrom<Int> for u64 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        if i.neg {
            return Err(TryFromIntError("u64"))
        }
        Ok(i.val)
    }
}

impl TryFrom<Int> for u128 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        if i.neg {
            return Err(TryFromIntError("u128"))
        }
        Ok(u128::from(i.val))
    }
}

impl TryFrom<Int> for i8 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        i64::try_from(i).and_then(|n| i8::try_from(n).map_err(|_| TryFromIntError("i8")))
    }
}

impl TryFrom<Int> for i16 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        i64::try_from(i).and_then(|n| i16::try_from(n).map_err(|_| TryFromIntError("i16")))
    }
}

impl TryFrom<Int> for i32 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        i64::try_from(i).and_then(|n| i32::try_from(n).map_err(|_| TryFromIntError("i32")))
    }
}

impl TryFrom<Int> for i64 {
    type Error = TryFromIntError;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        let j = i64::try_from(i.val).map_err(|_| TryFromIntError("i64"))?;
        Ok(if i.neg { -1 - j } else { j })
    }
}

impl From<Int> for i128 {
    fn from(i: Int) -> Self {
        let j = i128::from(i.val);
        if i.neg { -1 - j } else { j }
    }
}

/// Error when conversion of a CBOR [`Int`] to another type failed.
#[derive(Debug)]
pub struct TryFromIntError(&'static str);

impl fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value out of {} range", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromIntError {}

