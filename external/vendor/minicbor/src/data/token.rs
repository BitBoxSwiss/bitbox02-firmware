use super::{Int, Tag, Type};
use crate::encode::{self, Encode, Encoder, Write};
use crate::decode::{Decode, Error};
use crate::CborLen;
use core::fmt;

/// Representation of possible CBOR tokens.
///
/// *Requires feature* `"half"`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'b> {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Int(Int),
    F16(f32),
    F32(f32),
    F64(f64),
    Bytes(&'b [u8]),
    String(&'b str),
    Array(u64),
    Map(u64),
    Tag(Tag),
    Simple(u8),
    Break,
    Null,
    Undefined,
    /// Start of indefinite byte string.
    BeginBytes,
    /// Start of indefinite text string.
    BeginString,
    /// Start of indefinite array.
    BeginArray,
    /// Start of indefinite map.
    BeginMap
}

/// Pretty print a token.
///
/// Since we only show a single token we can not use diagnostic notation
/// as in the `Display` impl of [`crate::decode::Tokenizer`]. Instead, the following
/// syntax is used:
///
/// - Numeric values and booleans are displayed as in Rust. Floats are always
///   shown in scientific notation.
/// - Text strings are displayed in double quotes.
/// - Byte strings are displayed in single quotes prefixed with `h` and
///   hex-encoded, e.g. `h'01 02 ef'`.
/// - An array is displayed as `A[n]` where `n` denotes the number of elements.
///   The following `n` tokens are elements of this array.
/// - A map is displayed as `M[n]` where `n` denotes the number of pairs.
///   The following `n` tokens are entries of this map.
/// - Tags are displayed with `T(t)` where `t` is the tag number.
/// - Simple values are displayed as `simple(n)` where `n` denotes the numeric
///   value.
/// - Indefinite items start with:
///     * `?B[` for byte strings,
///     * `?S[` for text strings,
///     * `?A[` for arrays,
///     * `?M[` for maps,
///   and end with `]` when a `Token::Break` is encountered. All tokens
///   in between belong to the indefinite container.
/// - `Token::Null` is displayed as `null` and `Token::Undefined` as `undefined`.
impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Bool(b)     => write!(f, "{}", b),
            Token::U8(n)       => write!(f, "{}", n),
            Token::U16(n)      => write!(f, "{}", n),
            Token::U32(n)      => write!(f, "{}", n),
            Token::U64(n)      => write!(f, "{}", n),
            Token::I8(n)       => write!(f, "{}", n),
            Token::I16(n)      => write!(f, "{}", n),
            Token::I32(n)      => write!(f, "{}", n),
            Token::I64(n)      => write!(f, "{}", n),
            Token::Int(n)      => write!(f, "{}", n),
            Token::F16(n)      => write!(f, "{:e}", n),
            Token::F32(n)      => write!(f, "{:e}", n),
            Token::F64(n)      => write!(f, "{:e}", n),
            Token::String(n)   => write!(f, "\"{}\"", n),
            Token::Array(n)    => write!(f, "A[{}]", n),
            Token::Map(n)      => write!(f, "M[{}]", n),
            Token::Tag(t)      => write!(f, "T({})", u64::from(t)),
            Token::Simple(n)   => write!(f, "simple({})", n),
            Token::Break       => f.write_str("]"),
            Token::Null        => f.write_str("null"),
            Token::Undefined   => f.write_str("undefined"),
            Token::BeginBytes  => f.write_str("?B["),
            Token::BeginString => f.write_str("?S["),
            Token::BeginArray  => f.write_str("?A["),
            Token::BeginMap    => f.write_str("?M["),
            Token::Bytes(b)    => {
                f.write_str("h'")?;
                let mut i = b.len();
                for x in *b {
                    if i > 1 {
                        write!(f, "{:02x} ", x)?
                    } else {
                        write!(f, "{:02x}", x)?
                    }
                    i -= 1;
                }
                f.write_str("'")
            }
        }
    }
}

impl<'b, C> Decode<'b, C> for Token<'b> {
    fn decode(d: &mut crate::Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        match d.datatype()? {
            Type::Bool   => d.bool().map(Token::Bool),
            Type::U8     => d.u8().map(Token::U8),
            Type::U16    => d.u16().map(Token::U16),
            Type::U32    => d.u32().map(Token::U32),
            Type::U64    => d.u64().map(Token::U64),
            Type::I8     => d.i8().map(Token::I8),
            Type::I16    => d.i16().map(Token::I16),
            Type::I32    => d.i32().map(Token::I32),
            Type::I64    => d.i64().map(Token::I64),
            Type::Int    => d.int().map(Token::Int),
            Type::F16    => d.f16().map(Token::F16),
            Type::F32    => d.f32().map(Token::F32),
            Type::F64    => d.f64().map(Token::F64),
            Type::Bytes  => d.bytes().map(Token::Bytes),
            Type::String => d.str().map(Token::String),
            Type::Tag    => d.tag().map(Token::Tag),
            Type::Simple => d.simple().map(Token::Simple),
            Type::Array  => {
                let p = d.position();
                if let Some(n) = d.array()? {
                    Ok(Token::Array(n))
                } else {
                    Err(Error::type_mismatch(Type::Array).at(p).with_message("missing array length"))
                }
            }
            Type::Map => {
                let p = d.position();
                if let Some(n) = d.map()? {
                    Ok(Token::Map(n))
                } else {
                    Err(Error::type_mismatch(Type::Array).at(p).with_message("missing map length"))
                }
            }
            Type::BytesIndef   => { skip_byte(d); Ok(Token::BeginBytes)  }
            Type::StringIndef  => { skip_byte(d); Ok(Token::BeginString) }
            Type::ArrayIndef   => { skip_byte(d); Ok(Token::BeginArray)  }
            Type::MapIndef     => { skip_byte(d); Ok(Token::BeginMap)    }
            Type::Null         => { skip_byte(d); Ok(Token::Null)        }
            Type::Undefined    => { skip_byte(d); Ok(Token::Undefined)   }
            Type::Break        => { skip_byte(d); Ok(Token::Break)       }
            t@Type::Unknown(_) => Err(Error::type_mismatch(t)
                .at(d.position())
                .with_message("unknown cbor type"))
        }
    }
}

fn skip_byte(d: &mut crate::Decoder<'_>) {
    d.set_position(d.position() + 1)
}

impl<'b, C> Encode<C> for Token<'b> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _: &mut C) -> Result<(), encode::Error<W::Error>> {
        match *self {
            Token::Bool(val)   => e.bool(val)?,
            Token::U8(val)     => e.u8(val)?,
            Token::U16(val)    => e.u16(val)?,
            Token::U32(val)    => e.u32(val)?,
            Token::U64(val)    => e.u64(val)?,
            Token::I8(val)     => e.i8(val)?,
            Token::I16(val)    => e.i16(val)?,
            Token::I32(val)    => e.i32(val)?,
            Token::I64(val)    => e.i64(val)?,
            Token::Int(val)    => e.int(val)?,
            Token::F16(val)    => e.f16(val)?,
            Token::F32(val)    => e.f32(val)?,
            Token::F64(val)    => e.f64(val)?,
            Token::Bytes(val)  => e.bytes(val)?,
            Token::String(val) => e.str(val)?,
            Token::Array(val)  => e.array(val)?,
            Token::Map(val)    => e.map(val)?,
            Token::Tag(val)    => e.tag(val)?,
            Token::Simple(val) => e.simple(val)?,
            Token::Break       => e.end()?,
            Token::Null        => e.null()?,
            Token::Undefined   => e.undefined()?,
            Token::BeginBytes  => e.begin_bytes()?,
            Token::BeginString => e.begin_str()?,
            Token::BeginArray  => e.begin_array()?,
            Token::BeginMap    => e.begin_map()?
        };
        Ok(())
    }
}

impl<'b, C> CborLen<C> for Token<'b> {
    fn cbor_len(&self, ctx: &mut C) -> usize {
        match self {
            Token::Bool(val)   => val.cbor_len(ctx),
            Token::U8(val)     => val.cbor_len(ctx),
            Token::U16(val)    => val.cbor_len(ctx),
            Token::U32(val)    => val.cbor_len(ctx),
            Token::U64(val)    => val.cbor_len(ctx),
            Token::I8(val)     => val.cbor_len(ctx),
            Token::I16(val)    => val.cbor_len(ctx),
            Token::I32(val)    => val.cbor_len(ctx),
            Token::I64(val)    => val.cbor_len(ctx),
            Token::Int(val)    => val.cbor_len(ctx),
            Token::F16(val)    => val.cbor_len(ctx),
            Token::F32(val)    => val.cbor_len(ctx),
            Token::F64(val)    => val.cbor_len(ctx),
            Token::Bytes(val)  => val.cbor_len(ctx),
            Token::String(val) => val.cbor_len(ctx),
            Token::Array(val)  => val.cbor_len(ctx),
            Token::Map(val)    => val.cbor_len(ctx),
            Token::Tag(val)    => val.cbor_len(ctx),
            Token::Simple(val) => val.cbor_len(ctx),
            Token::Break       => 1,
            Token::Null        => 1,
            Token::Undefined   => 1,
            Token::BeginBytes  => 1,
            Token::BeginString => 1,
            Token::BeginArray  => 1,
            Token::BeginMap    => 1
        }
    }
}
