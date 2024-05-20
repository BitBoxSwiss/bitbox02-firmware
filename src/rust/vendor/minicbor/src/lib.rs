//! A small [CBOR] codec suitable for `no_std` environments.
//!
//! The crate is organised around the following entities:
//!
//! - [`Encoder`] and [`Decoder`] for type-directed encoding and decoding
//! of values.
//!
//! - [`Encode`] and [`Decode`] traits which can be implemented for any
//! type that should be encoded to or decoded from CBOR. They are similar
//! to [serde]'s `Serialize` and `Deserialize` traits but do not abstract
//! over the encoder/decoder.
//!
//! Encoding and decoding proceeds in a type-directed way, i.e.  by calling
//! methods for expected data item types, e.g. [`Decoder::u32`] or
//! [`Encoder::str`]. In addition there is support for data type inspection.
//! The `Decoder` can be queried for the current data type which returns a
//! [`data::Type`] that can represent every possible CBOR type and decoding
//! can thus proceed based on this information. It is also possible to just
//! tokenize the input bytes using a [`Tokenizer`](decode::Tokenizer), i.e.
//! an `Iterator` over CBOR [`Token`](data::Token)s. Finally, the length
//! in bytes of a value's CBOR representation can be calculated if the
//! value's type implements the [`CborLen`] trait.
//!
//! Optionally, `Encode` and `Decode` can be derived for structs and enums
//! using the respective derive macros (*requires feature* `"derive"`).
//! See [`minicbor_derive`] for details.
//!
//! For I/O support see [`minicbor-io`][1].
//!
//! [1]: https://twittner.gitlab.io/minicbor/minicbor_io/
//!
//! # Feature flags
//!
//! The following feature flags are supported:
//!
//! - `"alloc"`: Enables most collection types in a `no_std` environment.
//!
//! - `"std"`: Implies `"alloc"` and enables more functionality that depends
//!   on the `std` crate.
//!
//! - `"derive"`: Allows deriving [`Encode`] and [`Decode`] traits.
//!
//! # Example: generic encoding and decoding
//!
//! ```
//! use minicbor::{Encode, Decode};
//!
//! let input = ["hello", "world"];
//! let mut buffer = [0u8; 128];
//!
//! minicbor::encode(&input, buffer.as_mut())?;
//! let output: [&str; 2] = minicbor::decode(buffer.as_ref())?;
//! assert_eq!(input, output);
//!
//! # Ok::<_, Box<dyn std::error::Error>>(())
//! ```
//!
//! # Example: ad-hoc encoding
//!
//! ```
//! use minicbor::Encoder;
//!
//! let mut buffer = [0u8; 128];
//! let mut encoder = Encoder::new(&mut buffer[..]);
//!
//! encoder.begin_map()? // using an indefinite map here
//!     .str("hello")?.str("world")?
//!     .str("submap")?.map(2)?
//!         .u8(1)?.bool(true)?
//!         .u8(2)?.bool(false)?
//!     .u16(34234)?.array(3)?.u8(1)?.u8(2)?.u8(3)?
//!     .bool(true)?.null()?
//! .end()?;
//!
//! # Ok::<_, Box<dyn std::error::Error>>(())
//! ```
//!
//! # Example: ad-hoc decoding
//!
//! ```
//! use minicbor::Decoder;
//! use minicbor::data::IanaTag;
//!
//! let input = [
//!     0xc0, 0x74, 0x32, 0x30, 0x31, 0x33, 0x2d, 0x30,
//!     0x33, 0x2d, 0x32, 0x31, 0x54, 0x32, 0x30, 0x3a,
//!     0x30, 0x34, 0x3a, 0x30, 0x30, 0x5a
//! ];
//!
//! let mut decoder = Decoder::new(&input);
//! assert_eq!(IanaTag::DateTime.tag(), decoder.tag()?);
//! assert_eq!("2013-03-21T20:04:00Z", decoder.str()?);
//! # Ok::<_, Box<dyn std::error::Error>>(())
//! ```
//!
//! # Example: tokenization
//!
//! ```
//! use minicbor::display;
//! use minicbor::{Encoder, Decoder};
//! use minicbor::data::Token;
//!
//! let input  = [0x83, 0x01, 0x9f, 0x02, 0x03, 0xff, 0x82, 0x04, 0x05];
//!
//! assert_eq!("[1, [_ 2, 3], [4, 5]]", format!("{}", display(&input)));
//!
//! let tokens = Decoder::new(&input).tokens().collect::<Result<Vec<Token>, _>>()?;
//!
//! assert_eq! { &tokens[..],
//!     &[Token::Array(3),
//!       Token::U8(1),
//!       Token::BeginArray,
//!       Token::U8(2),
//!       Token::U8(3),
//!       Token::Break,
//!       Token::Array(2),
//!       Token::U8(4),
//!       Token::U8(5)]
//! };
//!
//! let mut buffer = [0u8; 9];
//! Encoder::new(buffer.as_mut()).tokens(&tokens)?;
//!
//! assert_eq!(input, buffer);
//!
//! # Ok::<_, Box<dyn std::error::Error>>(())
//! ```
//!
//! [CBOR]: https://datatracker.ietf.org/doc/html/rfc8949
//! [serde]: https://serde.rs

#![forbid(unused_variables)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod bytes;
pub mod data;
pub mod decode;
pub mod encode;

const UNSIGNED: u8 = 0x00;
const SIGNED: u8   = 0x20;
const BYTES: u8    = 0x40;
const TEXT: u8     = 0x60;
const ARRAY: u8    = 0x80;
const MAP: u8      = 0xa0;
const TAGGED: u8   = 0xc0;
const SIMPLE: u8   = 0xe0;
const BREAK: u8    = 0xff;

pub use decode::{Decode, Decoder};
pub use encode::{Encode, Encoder, CborLen};

#[cfg(feature = "derive")]
pub use minicbor_derive::*;

#[cfg(feature = "alloc")]
use core::convert::Infallible;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Decode a type implementing [`Decode`] from the given byte slice.
pub fn decode<'b, T>(b: &'b [u8]) -> Result<T, decode::Error>
where
    T: Decode<'b, ()>
{
    Decoder::new(b).decode()
}

/// Decode a type implementing [`Decode`] from the given byte slice.
pub fn decode_with<'b, C, T>(b: &'b [u8], ctx: &mut C) -> Result<T, decode::Error>
where
    T: Decode<'b, C>
{
    Decoder::new(b).decode_with(ctx)
}

/// Encode a type implementing [`Encode`] to the given [`encode::Write`] impl.
pub fn encode<T, W>(x: T, w: W) -> Result<(), encode::Error<W::Error>>
where
    T: Encode<()>,
    W: encode::Write
{
    Encoder::new(w).encode(x)?.ok()
}

/// Encode a type implementing [`Encode`] to the given [`encode::Write`] impl.
pub fn encode_with<C, T, W>(x: T, w: W, ctx: &mut C) -> Result<(), encode::Error<W::Error>>
where
    T: Encode<C>,
    W: encode::Write
{
    Encoder::new(w).encode_with(x, ctx)?.ok()
}

/// Encode a type implementing [`Encode`] and return the encoded byte vector.
///
/// *Requires feature* `"alloc"`.
#[cfg(feature = "alloc")]
pub fn to_vec<T>(x: T) -> Result<Vec<u8>, encode::Error<Infallible>>
where
    T: Encode<()>
{
    let mut e = Encoder::new(Vec::new());
    x.encode(&mut e, &mut ())?;
    Ok(e.into_writer())
}

/// Encode a type implementing [`Encode`] and return the encoded byte vector.
///
/// *Requires feature* `"alloc"`.
#[cfg(feature = "alloc")]
pub fn to_vec_with<C, T>(x: T, ctx: &mut C) -> Result<Vec<u8>, encode::Error<Infallible>>
where
    T: Encode<C>
{
    let mut e = Encoder::new(Vec::new());
    x.encode(&mut e, ctx)?;
    Ok(e.into_writer())
}

/// Display the given CBOR bytes in [diagnostic notation][1].
///
/// *Requires features* `"alloc"` *and* `"half"`.
///
/// Quick syntax summary:
///
/// - Maps are enclosed in curly braces: `{` and `}`.
/// - Arrays are enclosed in brackets: `[` and `]`.
/// - Indefinite maps start with `{_` instead of `{`.
/// - Indefinite arrays start with `[_` instead of `[`.
/// - Bytes are hex encoded and enclosed in `h'` and `'`.
/// - Strings are enclosed in double quotes.
/// - Numbers and booleans are displayed as in Rust but floats are always
///   shown in scientific notation (this differs slightly from the RFC
///   format).
/// - Indefinite bytes are enclosed in `(_` and `)` except for the empty
///   sequence which is shown as `''_`.
/// - Indefinite strings are enclosed in `(_` and `)` except for the empty
///   sequence which is shown as `""_`.
/// - Tagged values are enclosed in `t(` and `)` where `t` is the numeric
///   tag value.
/// - Simple values are shown as `simple(n)` where `n` is the numeric
///   simple value.
/// - Undefined and null are shown as `undefined` and `null`.
///
/// No error is produced should decoding fail, the error message
/// becomes part of the display.
///
/// [1]: https://www.rfc-editor.org/rfc/rfc8949.html#section-8
#[cfg(all(feature = "alloc", feature = "half"))]
pub fn display<'b>(cbor: &'b [u8]) -> impl core::fmt::Display + 'b {
    decode::Tokenizer::new(cbor)
}

/// Calculate the length in bytes of the given value's CBOR representation.
pub fn len<T>(x: T) -> usize
where
    T: CborLen<()>
{
    x.cbor_len(&mut ())
}

/// Calculate the length in bytes of the given value's CBOR representation.
pub fn len_with<C, T>(x: T, ctx: &mut C) -> usize
where
    T: CborLen<C>
{
    x.cbor_len(ctx)
}

// Ensure we can safely cast a `usize` to a `u64`.
const __USIZE_FITS_INTO_U64: () =
    assert!(core::mem::size_of::<usize>() <= core::mem::size_of::<u64>());

