//! Utilities for pinning
//!
//! **Note:** Since Rust 1.68, all APIs in this crate are now soft-deprecated or deprecated:
//!
//! - `pin_utils::pin_mut!` is soft-deprecated in favor of [`pin!` macro in the standard library](https://doc.rust-lang.org/std/pin/macro.pin.html) that stabilized in Rust 1.68.
//! - `pin_utils::{unsafe_pinned,unsafe_unpinned}` are **deprecated** in favor of safe alternatives: [pin-project](https://crates.io/crates/pin-project), [pin-project-lite](https://crates.io/crates/pin-project-lite)

#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

#[macro_use]
mod stack_pin;
#[macro_use]
mod projection;

// Not public API.
#[doc(hidden)]
pub mod __private {
    pub use core::pin::Pin;
}
