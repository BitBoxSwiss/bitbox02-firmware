//! The ChaCha20 stream cipher ([RFC 8439])
//!
//! ChaCha20 is a lightweight stream cipher which is amenable to fast,
//! constant-time implementations in software. It improves upon the previous
//! [Salsa20] stream cipher, providing increased per-round diffusion
//! with no cost to performance.
//!
//! Cipher functionality is accessed using traits from re-exported
//! [`cipher`](https://docs.rs/cipher) crate.
//!
//! This crate contains the following variants of the ChaCha20 core algorithm:
//!
//! - [`ChaCha20`]: standard IETF variant with 96-bit nonce
//! - [`ChaCha20Legacy`]: (gated under the `legacy` feature) "djb" variant with 64-bit nonce
//! - [`ChaCha8`] / [`ChaCha12`]: reduced round variants of ChaCha20
//! - [`XChaCha20`]: 192-bit extended nonce variant
//! - [`XChaCha8`] / [`XChaCha12`]: reduced round variants of XChaCha20
//!
//! # ⚠️ Security Warning: [Hazmat!]
//!
//! This crate does not ensure ciphertexts are authentic, which can lead to
//! serious vulnerabilities if used incorrectly!
//!
//! If in doubt, use the [`chacha20poly1305`](https://docs.rs/chacha20poly1305)
//! crate instead, which provides an authenticated mode on top of ChaCha20.
//!
//! **USE AT YOUR OWN RISK!**
//!
//! # Diagram
//!
//! This diagram illustrates the ChaCha quarter round function.
//! Each round consists of four quarter-rounds:
//!
//! <img src="https://raw.githubusercontent.com/RustCrypto/meta/master/img/stream-ciphers/chacha20.png" width="300px">
//!
//! Legend:
//!
//! - ⊞ add
//! - ‹‹‹ rotate
//! - ⊕ xor
//!
//! # Usage
//!
//! ```
//! # #[cfg(feature = "cipher")]
//! # {
//! use chacha20::{ChaCha20, Key, Nonce};
//! use chacha20::cipher::{NewCipher, StreamCipher, StreamCipherSeek};
//!
//! let mut data = [1, 2, 3, 4, 5, 6, 7];
//!
//! let key = Key::from_slice(b"an example very very secret key.");
//! let nonce = Nonce::from_slice(b"secret nonce");
//!
//! // create cipher instance
//! let mut cipher = ChaCha20::new(&key, &nonce);
//!
//! // apply keystream (encrypt)
//! cipher.apply_keystream(&mut data);
//! assert_eq!(data, [73, 98, 234, 202, 73, 143, 0]);
//!
//! // seek to the keystream beginning and apply it again to the `data` (decrypt)
//! cipher.seek(0);
//! cipher.apply_keystream(&mut data);
//! assert_eq!(data, [1, 2, 3, 4, 5, 6, 7]);
//! # }
//! ```
//!
//! [RFC 8439]: https://tools.ietf.org/html/rfc8439
//! [Salsa20]: https://docs.rs/salsa20
//! [Hazmat!]: https://github.com/RustCrypto/meta/blob/master/HAZMAT.md

#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg",
    html_root_url = "https://docs.rs/chacha20/0.8.1"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    all(feature = "neon", target_arch = "aarch64", target_feature = "neon"),
    feature(stdsimd, aarch64_target_feature)
)]
#![warn(missing_docs, rust_2018_idioms, trivial_casts, unused_qualifications)]

mod backend;
#[cfg(feature = "cipher")]
mod chacha;
#[cfg(feature = "legacy")]
mod legacy;
mod max_blocks;
#[cfg(feature = "rng")]
mod rng;
mod rounds;
#[cfg(feature = "cipher")]
mod xchacha;

#[cfg(feature = "cipher")]
pub use cipher;

#[cfg(feature = "cipher")]
pub use crate::{
    chacha::{ChaCha, ChaCha12, ChaCha20, ChaCha8, Key, Nonce},
    xchacha::{XChaCha, XChaCha12, XChaCha20, XChaCha8, XNonce},
};

#[cfg(feature = "expose-core")]
pub use crate::{
    backend::Core,
    rounds::{R12, R20, R8},
};

#[cfg(feature = "hchacha")]
pub use crate::xchacha::hchacha;

#[cfg(feature = "legacy")]
pub use crate::legacy::{ChaCha20Legacy, LegacyNonce};

#[cfg(feature = "rng")]
pub use rng::{
    ChaCha12Rng, ChaCha12RngCore, ChaCha20Rng, ChaCha20RngCore, ChaCha8Rng, ChaCha8RngCore,
};

/// Size of a ChaCha20 block in bytes
pub const BLOCK_SIZE: usize = 64;

/// Size of a ChaCha20 key in bytes
pub const KEY_SIZE: usize = 32;

/// Number of bytes in the core (non-extended) ChaCha20 IV
const IV_SIZE: usize = 8;

/// State initialization constant ("expand 32-byte k")
const CONSTANTS: [u32; 4] = [0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574];
