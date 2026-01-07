// SPDX-License-Identifier: CC0-1.0

//! Hex encoding and decoding.
//!
//! General purpose hex encoding/decoding library with a conservative MSRV and dependency policy.
//!
//! ## Basic Usage
//! ```
//! # #[cfg(feature = "alloc")] {
//! // In your manifest use the `package` key to improve import ergonomics.
//! // hex = { package = "hex-conservative", version = "*" }
//! # use hex_conservative as hex; // No need for this if using `package` as above.
//! use hex::prelude::*;
//!
//! // Decode an arbitrary length hex string into a vector.
//! let v = Vec::from_hex("deadbeef").expect("valid hex digits");
//! // Or a known length hex string into a fixed size array.
//! let a = <[u8; 4]>::from_hex("deadbeef").expect("valid length and valid hex digits");
//!
//! // We support `LowerHex` and `UpperHex` out of the box for `[u8]` slices.
//! println!("An array as lower hex: {:x}", a.as_hex());
//! // And for vecs since `Vec` derefs to byte slice.
//! println!("A vector as upper hex: {:X}", v.as_hex());
//!
//! // Allocate a new string (also `to_upper_hex_string`).
//! let s = v.to_lower_hex_string();
//!
//! // Please note, mixed case strings will still parse successfully but we only
//! // support displaying hex in a single case.
//! assert_eq!(
//!     Vec::from_hex("dEaDbEeF").expect("valid mixed case hex digits"),
//!     Vec::from_hex("deadbeef").expect("valid hex digits"),
//! );
//! # }
//! ```

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
// Experimental features we need.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Coding conventions
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(hidden)]
pub mod _export {
    /// A re-export of core::*
    pub mod _core {
        pub use core::*;
    }
}

pub mod buf_encoder;
pub mod display;
pub mod error;
mod iter;
pub mod parse;
#[cfg(feature = "serde")]
pub mod serde;

/// Re-exports of the common crate traits.
pub mod prelude {
    #[doc(inline)]
    pub use crate::{display::DisplayHex, parse::FromHex};
}

pub(crate) use table::Table;

#[rustfmt::skip]                // Keep public re-exports separate.
#[doc(inline)]
pub use self::{
    display::DisplayHex,
    error::{OddLengthStringError, HexToBytesError, HexToArrayError, InvalidCharError},
    iter::{BytesToHexIter, HexToBytesIter, HexSliceToBytesIter},
    parse::FromHex,
};

/// Possible case of hex.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Case {
    /// Produce lower-case chars (`[0-9a-f]`).
    ///
    /// This is the default.
    Lower,

    /// Produce upper-case chars (`[0-9A-F]`).
    Upper,
}

impl Default for Case {
    #[inline]
    fn default() -> Self { Case::Lower }
}

impl Case {
    /// Returns the encoding table.
    ///
    /// The returned table may only contain displayable ASCII chars.
    #[inline]
    #[rustfmt::skip]
    pub(crate) fn table(self) -> &'static Table {
        match self {
            Case::Lower => &Table::LOWER,
            Case::Upper => &Table::UPPER,
        }
    }
}

/// Correctness boundary for `Table`.
mod table {
    use arrayvec::ArrayString;

    /// Table of hex chars.
    //
    // Correctness invariant: each byte in the table must be ASCII.
    pub(crate) struct Table([u8; 16]);

    impl Table {
        pub(crate) const LOWER: Self = Table([
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd',
            b'e', b'f',
        ]);
        pub(crate) const UPPER: Self = Table([
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D',
            b'E', b'F',
        ]);

        /// Encodes single byte as two ASCII chars using the given table.
        ///
        /// The function guarantees only returning values from the provided table.
        #[inline]
        pub(crate) fn byte_to_hex(&self, byte: u8) -> ArrayString<2> {
            let left = self.0[usize::from(byte.wrapping_shr(4))];
            let right = self.0[usize::from(byte & 0x0F)];

            ArrayString::from_byte_string(&[left, right]).expect("Table only contains valid ASCII")
        }
    }
}

/// Quick and dirty macro for parsing hex in tests.
///
/// For improved ergonomics import with: `use hex_conservative::test_hex_unwrap as hex;`
#[macro_export]
macro_rules! test_hex_unwrap (($hex:expr) => (<Vec<u8> as $crate::FromHex>::from_hex($hex).unwrap()));

#[cfg(test)]
mod tests {
    use crate::test_hex_unwrap as hex;

    #[test]
    fn parse_hex_into_vector() {
        let got = hex!("deadbeef");
        let want = vec![0xde, 0xad, 0xbe, 0xef];
        assert_eq!(got, want)
    }
}
