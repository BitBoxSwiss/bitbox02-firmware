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
pub use x25519::{Random32, X25519};

pub use noise_rust_crypto::sensitive::Sensitive;

use noise_protocol::DH;

/// Generate a x25519 private key.
pub fn generate_static_private_key<R: Random32>() -> Sensitive<x25519::PrivateKey> {
    X25519::<R>::genkey()
}
