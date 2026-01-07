// SPDX-License-Identifier: CC0-1.0

//! Hex encoding and decoding.
//!
//! General purpose hex encoding/decoding library with a conservative MSRV and dependency policy.
//!
//! ## Stabilization strategy
//!
//! Because downstream crates may need to return hex errors in their APIs and they need to be
//! stabilized soon, this crate only exposes the errors and two basic decoding functions. This
//! should already help with the vast majority of the cases and we're sufficiently confident that
//! these errors won't have a breaking change any time soon (possibly never).
//!
//! If you're writing a binary you don't need to worry about any of this and just use the unstable
//! version for now. If you're writing a library you should use these stable errors in the API but
//! you may internally depend on the unstable crate version to get the advanced features that won't
//! affect your API. This way your API can stabilize before all features in this crate are fully
//! stable and you still can use all of them.
//!
//! ## Crate feature flags
//!
//! * `std` - enables the standard library, on by default.
//! * `alloc` - enables features that require allocation such as decoding into `Vec<u8>`, implied
//!   by `std`.
//! * `newer-rust-version` - enables Rust version detection and thus newer features, may add
//!   dependency on a feature detection crate to reduce compile times. This feature is expected to
//!   do nothing once the native detection is in Rust and our MSRV is at least that version. We may
//!   also remove the feature gate in 2.0 with semver trick once that happens.
//!
//! ## MSRV policy
//!
//! The MSRV of the crate is currently 1.63.0 and we don't intend to bump it until the newer Rust
//! version is at least two years old and also included in Debian stable (1.63 is in Debian 12 at
//! the moment).
//!
//! Note though that the dependencies may have looser policy. This is not considered
//! breaking/wrong - you would just need to pin them in `Cargo.lock` (not `.toml`).

#![no_std]
// Experimental features we need.
#![cfg_attr(docsrs, feature(doc_cfg))]
// Coding conventions
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
#[allow(unused_imports)] // false positive regarding macro
#[macro_use]
extern crate alloc;

#[doc(hidden)]
pub mod _export {
    /// A re-export of `core::*`.
    pub mod _core {
        pub use core::*;
    }
}

pub mod error;
mod iter;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::iter::HexToBytesIter;

#[rustfmt::skip]                // Keep public re-exports separate.
#[doc(inline)]
pub use self::{
    error::{
        DecodeFixedLengthBytesError, DecodeVariableLengthBytesError,
    },
};

/// Decodes a hex string with variable length.
///
/// The length of the returned `Vec` is determined by the length of the input, meaning all even
/// lengths of the input string are allowed. If you know the required length at compile time using
/// [`decode_to_array`] is most likely a better choice.
///
/// # Errors
///
/// Returns an error if `hex` contains invalid characters or doesn't have even length.
#[cfg(feature = "alloc")]
pub fn decode_to_vec(hex: &str) -> Result<Vec<u8>, DecodeVariableLengthBytesError> {
    Ok(HexToBytesIter::new(hex)?.drain_to_vec()?)
}

/// Decodes a hex string with an expected length known at compile time.
///
/// If you don't know the required length at compile time you need to use [`decode_to_vec`]
/// instead.
///
/// # Errors
///
/// Returns an error if `hex` contains invalid characters or has incorrect length. (Should be
/// `N * 2`.)
pub fn decode_to_array<const N: usize>(hex: &str) -> Result<[u8; N], DecodeFixedLengthBytesError> {
    if hex.len() == N * 2 {
        let mut ret = [0u8; N];
        // checked above
        HexToBytesIter::new_unchecked(hex).drain_to_slice(&mut ret)?;
        Ok(ret)
    } else {
        Err(error::InvalidLengthError { invalid: hex.len(), expected: 2 * N }.into())
    }
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    #[test]
    fn parse_hex_into_vector() {
        let got = crate::decode_to_vec("deadbeef").unwrap();
        let want = vec![0xde, 0xad, 0xbe, 0xef];
        assert_eq!(got, want);
    }
}
