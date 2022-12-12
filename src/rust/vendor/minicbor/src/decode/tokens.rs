//! Generic CBOR tokenization.

use core::fmt;
use crate::Decoder;
use crate::data::{Int, Tag, Type};
use crate::decode::Error;

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

/// An [`Iterator`] over CBOR tokens.
///
/// The `Iterator` implementation calls [`Tokenizer::token`] until end of input has been reached.
///
/// *Requires feature* `"half"`.
#[derive(Debug, Clone)]
pub struct Tokenizer<'b> {
    decoder: Decoder<'b>
}

impl<'b> Iterator for Tokenizer<'b> {
    type Item = Result<Token<'b>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.token() {
            Ok(t) => Some(Ok(t)),
            Err(e) if e.is_end_of_input() => None,
            Err(e) => Some(Err(e))
        }
    }
}

impl<'b> From<Decoder<'b>> for Tokenizer<'b> {
    fn from(d: Decoder<'b>) -> Self {
        Tokenizer { decoder: d }
    }
}

impl<'b> Tokenizer<'b> {
    /// Create a new Tokenizer for the given input bytes.
    pub fn new(bytes: &'b[u8]) -> Self {
        Tokenizer { decoder: Decoder::new(bytes) }
    }

    /// Decode the next token.
    ///
    /// Note that a sequence of tokens may not necessarily represent
    /// well-formed CBOR items.
    pub fn token(&mut self) -> Result<Token<'b>, Error> {
        match self.try_token() {
            Ok(tk) => Ok(tk),
            Err(e) => {
                self.decoder.set_position(self.decoder.input().len()); // drain decoder
                Err(e)
            }
        }
    }

    fn try_token(&mut self) -> Result<Token<'b>, Error> {
        match self.decoder.datatype()? {
            Type::Bool         => self.decoder.bool().map(Token::Bool),
            Type::U8           => self.decoder.u8().map(Token::U8),
            Type::U16          => self.decoder.u16().map(Token::U16),
            Type::U32          => self.decoder.u32().map(Token::U32),
            Type::U64          => self.decoder.u64().map(Token::U64),
            Type::I8           => self.decoder.i8().map(Token::I8),
            Type::I16          => self.decoder.i16().map(Token::I16),
            Type::I32          => self.decoder.i32().map(Token::I32),
            Type::I64          => self.decoder.i64().map(Token::I64),
            Type::Int          => self.decoder.int().map(Token::Int),
            Type::F16          => self.decoder.f16().map(Token::F16),
            Type::F32          => self.decoder.f32().map(Token::F32),
            Type::F64          => self.decoder.f64().map(Token::F64),
            Type::Bytes        => self.decoder.bytes().map(Token::Bytes),
            Type::String       => self.decoder.str().map(Token::String),
            Type::Tag          => self.decoder.tag().map(Token::Tag),
            Type::Simple       => self.decoder.simple().map(Token::Simple),
            Type::Array        => {
                let p = self.decoder.position();
                if let Some(n) = self.decoder.array()? {
                    Ok(Token::Array(n))
                } else {
                    Err(Error::type_mismatch(Type::Array).at(p).with_message("missing array length"))
                }
            }
            Type::Map          => {
                let p = self.decoder.position();
                if let Some(n) = self.decoder.map()? {
                    Ok(Token::Map(n))
                } else {
                    Err(Error::type_mismatch(Type::Array).at(p).with_message("missing map length"))
                }
            }
            Type::BytesIndef   => { self.skip_byte(); Ok(Token::BeginBytes)  }
            Type::StringIndef  => { self.skip_byte(); Ok(Token::BeginString) }
            Type::ArrayIndef   => { self.skip_byte(); Ok(Token::BeginArray)  }
            Type::MapIndef     => { self.skip_byte(); Ok(Token::BeginMap)    }
            Type::Null         => { self.skip_byte(); Ok(Token::Null)        }
            Type::Undefined    => { self.skip_byte(); Ok(Token::Undefined)   }
            Type::Break        => { self.skip_byte(); Ok(Token::Break)       }
            t@Type::Unknown(_) => Err(Error::type_mismatch(t)
                .at(self.decoder.position())
                .with_message("unknown cbor type"))
        }
    }

    fn skip_byte(&mut self) {
        self.decoder.set_position(self.decoder.position() + 1)
    }
}

#[cfg(feature = "alloc")]
impl fmt::Display for Tokenizer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /// Control stack element.
        enum E {
            N,               // get next token
            T,               // tag
            A(Option<u64>),  // array
            M(Option<u64>),  // map
            B,               // indefinite bytes
            D,               // indefinite text
            S(&'static str), // display string
            X(&'static str)  // display string (unless next token is BREAK)
        }

        let mut iter = Tokenizer::from(self.decoder.clone()).peekable();
        let mut stack = alloc::vec::Vec::new();

        while iter.peek().is_some() {
            stack.push(E::N);
            while let Some(elt) = stack.pop() {
                match elt {
                    E::N => match iter.next() {
                        Some(Ok(Token::Array(n))) => {
                            stack.push(E::A(Some(n)));
                            f.write_str("[")?
                        }
                        Some(Ok(Token::Map(n))) => {
                            stack.push(E::M(Some(n)));
                            f.write_str("{")?
                        }
                        Some(Ok(Token::BeginArray)) => {
                            stack.push(E::A(None));
                            f.write_str("[_ ")?
                        }
                        Some(Ok(Token::BeginMap)) => {
                            stack.push(E::M(None));
                            f.write_str("{_ ")?
                        }
                        Some(Ok(Token::BeginBytes)) => if let Some(Ok(Token::Break)) = iter.peek() {
                            iter.next();
                            f.write_str("''_")?
                        } else {
                            stack.push(E::B);
                            f.write_str("(_ ")?
                        }
                        Some(Ok(Token::BeginString)) => if let Some(Ok(Token::Break)) = iter.peek() {
                            iter.next();
                            f.write_str("\"\"_")?
                        } else {
                            stack.push(E::D);
                            f.write_str("(_ ")?
                        }
                        Some(Ok(Token::Tag(t))) => {
                            stack.push(E::T);
                            write!(f, "{}(", t.numeric())?
                        }
                        Some(Ok(t))  => t.fmt(f)?,
                        Some(Err(e)) => {
                            write!(f, " !!! decoding error: {}", e)?;
                            return Ok(())
                        }
                        None => continue
                    }
                    E::S(s) => f.write_str(s)?,
                    E::X(s) => match iter.peek() {
                        Some(Ok(Token::Break)) | None => continue,
                        Some(Ok(_))  => f.write_str(s)?,
                        Some(Err(e)) => {
                            write!(f, " !!! decoding error: {}", e)?;
                            return Ok(())
                        }
                    }
                    E::T => {
                        stack.push(E::S(")"));
                        stack.push(E::N)
                    }
                    E::A(Some(0)) => f.write_str("]")?,
                    E::A(Some(1)) => {
                        stack.push(E::A(Some(0)));
                        stack.push(E::N)
                    }
                    E::A(Some(n)) => {
                        stack.push(E::A(Some(n - 1)));
                        stack.push(E::S(", "));
                        stack.push(E::N)
                    }
                    E::A(None) => match iter.peek() {
                        None => {
                            write!(f, " !!! indefinite array not closed")?;
                            return Ok(())
                        }
                        Some(Ok(Token::Break)) => {
                            iter.next();
                            f.write_str("]")?
                        }
                        _ => {
                            stack.push(E::A(None));
                            stack.push(E::X(", "));
                            stack.push(E::N)
                        }
                    }
                    E::M(Some(0)) => f.write_str("}")?,
                    E::M(Some(1)) => {
                        stack.push(E::M(Some(0)));
                        stack.push(E::N);
                        stack.push(E::S(": "));
                        stack.push(E::N)
                    }
                    E::M(Some(n)) => {
                        stack.push(E::M(Some(n - 1)));
                        stack.push(E::S(", "));
                        stack.push(E::N);
                        stack.push(E::S(": "));
                        stack.push(E::N)
                    }
                    E::M(None) => match iter.peek() {
                        None => {
                            write!(f, " !!! indefinite map not closed")?;
                            return Ok(())
                        }
                        Some(Ok(Token::Break)) => {
                            iter.next();
                            f.write_str("}")?
                        }
                        _ => {
                            stack.push(E::M(None));
                            stack.push(E::X(", "));
                            stack.push(E::N);
                            stack.push(E::S(": "));
                            stack.push(E::N)
                        }
                    }
                    E::B => match iter.peek() {
                        None => {
                            write!(f, " !!! indefinite byte string not closed")?;
                            return Ok(())
                        }
                        Some(Ok(Token::Break)) => {
                            iter.next();
                            f.write_str(")")?
                        }
                        _ => {
                            stack.push(E::B);
                            stack.push(E::X(", "));
                            stack.push(E::N)
                        }
                    }
                    E::D => match iter.peek() {
                        None => {
                            write!(f, " !!! indefinite string not closed")?;
                            return Ok(())
                        }
                        Some(Ok(Token::Break)) => {
                            iter.next();
                            f.write_str(")")?
                        }
                        _ => {
                            stack.push(E::D);
                            stack.push(E::X(", "));
                            stack.push(E::N)
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

/// Pretty print a token.
///
/// Since we only show a single token we can not use diagnostic notation
/// as in the `Display` impl of [`Tokenizer`]. Instead, the following
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
            Token::Tag(t)      => write!(f, "T({})", t.numeric()),
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

