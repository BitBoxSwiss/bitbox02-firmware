// Copyright 2020 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
