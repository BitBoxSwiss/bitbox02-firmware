//! Hex encoding with `serde`.
//!
//! The functions in this module de/serialize as hex _only_ when the serializer is human readable.
//!
//! # Examples
//!
//! ```
//! # #[cfg(feature = "std")] {
//! use hex_conservative as hex;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct Foo {
//!     #[serde(with = "hex::serde")]
//!     bar: Vec<u8>,
//! }
//! # }
//! ```

use core::fmt;
use core::marker::PhantomData;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::prelude::*;

/// Serializes `data` as a hex string using lowercase characters.
///
/// We only serialize as hex if the serializer is human readable, if not we call through to the
/// `Serialize` implementation for `data`.
pub fn serialize<S, T>(data: T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + DisplayHex,
{
    serialize_lower(data, s)
}

/// Serializes `data` as a hex string using lowercase characters.
///
/// We only serialize as hex if the serializer is human readable, if not we call through to the
/// `Serialize` implementation for `data`.
pub fn serialize_lower<S, T>(data: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + DisplayHex,
{
    // Don't do anything special when not human readable.
    if !serializer.is_human_readable() {
        serde::Serialize::serialize(&data, serializer)
    } else {
        serializer.collect_str(&format_args!("{:x}", data.as_hex()))
    }
}

/// Serializes `data` as hex string using uppercase characters.
///
/// We only serialize as hex if the serializer is human readable, if not we call through to the
/// `Serialize` implementation for `data`.
pub fn serialize_upper<S, T>(data: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + DisplayHex,
{
    // Don't do anything special when not human readable.
    if !serializer.is_human_readable() {
        serde::Serialize::serialize(&data, serializer)
    } else {
        serializer.collect_str(&format_args!("{:X}", data.as_hex()))
    }
}

/// Deserializes a hex string into raw bytes.
///
/// Allows upper, lower, and mixed case characters (e.g. `a5b3c1`, `A5B3C1` and `A5b3C1`).
///
/// We only deserialize from hex if the serializer is human readable, if not we call through to the
/// `Deserialize` implementation for `T`.
pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromHex,
{
    struct HexVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for HexVisitor<T>
    where
        T: FromHex,
    {
        type Value = T;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an ASCII hex string")
        }

        fn visit_str<E: Error>(self, data: &str) -> Result<Self::Value, E> {
            FromHex::from_hex(data).map_err(Error::custom)
        }
    }

    // Don't do anything special when not human readable.
    if !d.is_human_readable() {
        serde::Deserialize::deserialize(d)
    } else {
        d.deserialize_map(HexVisitor(PhantomData))
    }
}
