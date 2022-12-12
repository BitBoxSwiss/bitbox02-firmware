//! Information about CBOR data types and tags.

use core::fmt;

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
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Tag {
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
    Unassigned(u64)
}

impl Tag {
    pub(crate) fn from(n: u64) -> Self {
        match n {
            0x00 => Tag::DateTime,
            0x01 => Tag::Timestamp,
            0x02 => Tag::PosBignum,
            0x03 => Tag::NegBignum,
            0x04 => Tag::Decimal,
            0x05 => Tag::Bigfloat,
            0x15 => Tag::ToBase64Url,
            0x16 => Tag::ToBase64,
            0x17 => Tag::ToBase16,
            0x18 => Tag::Cbor,
            0x20 => Tag::Uri,
            0x21 => Tag::Base64Url,
            0x22 => Tag::Base64,
            0x23 => Tag::Regex,
            0x24 => Tag::Mime,
            _    => Tag::Unassigned(n)
        }
    }

    pub(crate) fn numeric(self) -> u64 {
        match self {
            Tag::DateTime      => 0x00,
            Tag::Timestamp     => 0x01,
            Tag::PosBignum     => 0x02,
            Tag::NegBignum     => 0x03,
            Tag::Decimal       => 0x04,
            Tag::Bigfloat      => 0x05,
            Tag::ToBase64Url   => 0x15,
            Tag::ToBase64      => 0x16,
            Tag::ToBase16      => 0x17,
            Tag::Cbor          => 0x18,
            Tag::Uri           => 0x20,
            Tag::Base64Url     => 0x21,
            Tag::Base64        => 0x22,
            Tag::Regex         => 0x23,
            Tag::Mime          => 0x24,
            Tag::Unassigned(n) => n
        }
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

