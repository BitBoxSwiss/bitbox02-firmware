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

impl Type {
    pub(crate) fn read(n: u8) -> Self {
        match n {
            0x00 ..= 0x18        => Type::U8,
            0x19                 => Type::U16,
            0x1a                 => Type::U32,
            0x1b                 => Type::U64,
            0x20 ..= 0x38        => Type::I8,
            0x39                 => Type::I16,
            0x3a                 => Type::I32,
            0x3b                 => Type::I64,
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
            _                    => Type::Unknown(n)
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

