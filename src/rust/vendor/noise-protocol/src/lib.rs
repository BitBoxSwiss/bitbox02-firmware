//! Rust implementation of the [Noise Protocol
//! Framework](http://www.noiseprotocol.org/).
//!
//! # Basic Usage
//!
//! Initialize a [`HandshakeState`] with [`HandshakeState::new`] or
//! [`HandshakeStateBuilder`], call [`HandshakeState::write_message`] and
//! [`HandshakeState::read_message`] to complete the handshake, and finally call
//! [`HandshakeState::get_ciphers`] to get a pair of [`CipherState`] to
//! encrypt/decrypt further transport messages.
//!
//! # Crypto Primitives
//!
//! This crate only contains an abstract implementation of the protocol.
//! Concrete implementations of the crypto primitives, wrapping around some
//! popular libraries, are provided in sibling crates, e.g., `noise-ring`,
//! `noise-sodiumoxide` and `noise-rust-crypto`.
//!
//! Other implementations of the crypto primitives can be easily plugged in by
//! implementing the [`DH`], [`Cipher`] and [`Hash`] traits.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "use_std"), no_std)]

mod cipherstate;
mod handshakepattern;
mod handshakestate;
mod symmetricstate;
mod traits;

#[cfg(feature = "use_alloc")]
#[macro_use]
extern crate alloc;

pub use crate::cipherstate::CipherState;
pub use crate::traits::{Cipher, Hash, U8Array, DH};

/// Handshake patterns.
pub mod patterns {
    pub use crate::handshakepattern::*;
}

pub use crate::handshakestate::{Error, ErrorKind, HandshakeState, HandshakeStateBuilder};
