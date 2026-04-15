// SPDX-License-Identifier: Apache-2.0

//! This crate implements the state machine for establishing and using a noise channel.
//!
//! The BitBox02 uses the following noise protocol config: `Noise_XX_25519_ChaChaPoly_SHA256`.
//! [noiseexplorer.com/patterns/XX](https://noiseexplorer.com/patterns/XX/).

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

mod noise_xx;
pub mod testing;
mod x25519;

pub use noise_xx::{Error, HandshakeHash, HandshakeResult, State};
pub use x25519::{X25519, genkey};

pub use noise_rust_crypto::sensitive::Sensitive;
