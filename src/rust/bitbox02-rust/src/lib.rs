// SPDX-License-Identifier: Apache-2.0

// Since we are targeting embedded we exclude the standard library by default
#![no_std]
// When compiling for testing we allow certain warnings.
#![cfg_attr(test, allow(unused_imports, dead_code))]

pub use bitbox_proto::{pb, pb_backup};

pub mod async_usb;
pub mod attestation;
pub mod backup;
mod bip32;
pub mod bip39;
pub mod communication_mode;
pub mod general;
pub mod hal;
pub mod hash;
pub mod hww;
pub mod keystore;
#[cfg(all(
    feature = "firmware",
    not(any(feature = "c-unit-testing", feature = "simulator-graphical"))
))]
pub mod main_loop;
pub mod reset;
pub mod secp256k1;
#[cfg(feature = "app-u2f")]
mod u2f;
mod version;
pub mod workflow;
#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
mod xpubcache;

// for `format!`
#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate bitbox_aes;
