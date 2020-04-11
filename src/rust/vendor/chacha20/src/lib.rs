//! The ChaCha20 stream cipher ([RFC 8439])
//!
//! ChaCha20 is a lightweight stream cipher which is amenable to fast,
//! constant-time implementations in software. It improves upon the previous
//! [Salsa20] stream cipher, providing increased per-round diffusion
//! with no cost to performance.
//!
//! Cipher functionality is accessed using traits from re-exported
//! [`stream-cipher`](https://docs.rs/stream-cipher) crate.
//!
//! This crate contains three variants of ChaCha20:
//!
//! - `ChaCha20`: standard IETF variant with 96-bit nonce
//! - `ChaCha20Legacy`: (gated under the `legacy` feature) "djb" variant with 64-bit nonce
//! - `ChaCha8` / `ChaCha12`: reduced round variants of ChaCha20
//! - `XChaCha20`: (gated under the `xchacha20` feature) 192-bit extended nonce variant
//!
//! # Security Warning
//!
//! This crate does not ensure ciphertexts are authentic, which can lead to
//! serious vulnerabilities if used incorrectly!
//!
//! USE AT YOUR OWN RISK!
//!
//! # Usage
//!
//! ```
//! use chacha20::ChaCha20;
//! use chacha20::stream_cipher::generic_array::GenericArray;
//! use chacha20::stream_cipher::{NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek};
//!
//! let mut data = [1, 2, 3, 4, 5, 6, 7];
//!
//! let key = GenericArray::from_slice(b"an example very very secret key.");
//! let nonce = GenericArray::from_slice(b"secret nonce");
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
//! ```
//!
//! [RFC 8439]: https://tools.ietf.org/html/rfc8439
//! [Salsa20]: https://docs.rs/salsa20

#![no_std]
#![doc(html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png")]
#![warn(missing_docs, rust_2018_idioms, trivial_casts, unused_qualifications)]

#[cfg(feature = "stream-cipher")]
pub use stream_cipher;

mod block;
#[cfg(feature = "stream-cipher")]
pub(crate) mod cipher;
#[cfg(feature = "legacy")]
mod legacy;
mod rounds;
#[cfg(feature = "xchacha20")]
mod xchacha20;

#[cfg(feature = "rng")]
mod rng;

#[cfg(feature = "stream-cipher")]
pub use self::cipher::{ChaCha12, ChaCha20, ChaCha8, Cipher};

#[cfg(feature = "legacy")]
pub use self::legacy::ChaCha20Legacy;

#[cfg(feature = "rng")]
pub use rng::{
    ChaCha12Rng, ChaCha12RngCore, ChaCha20Rng, ChaCha20RngCore, ChaCha8Rng, ChaCha8RngCore,
};

#[cfg(feature = "xchacha20")]
pub use self::xchacha20::XChaCha20;

/// Size of a ChaCha20 block in bytes
pub const BLOCK_SIZE: usize = 64;

/// Size of a ChaCha20 key in bytes
pub const KEY_SIZE: usize = 32;

/// Maximum number of blocks that can be encrypted with ChaCha20 before the
/// counter overflows.
pub const MAX_BLOCKS: usize = core::u32::MAX as usize;

/// Number of bytes in the core (non-extended) ChaCha20 IV
const IV_SIZE: usize = 8;

/// Number of 32-bit words in the ChaCha20 state
const STATE_WORDS: usize = 16;

/// State initialization constant ("expand 32-byte k")
const CONSTANTS: [u32; 4] = [0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574];
