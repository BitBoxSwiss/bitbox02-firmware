// SPDX-License-Identifier: Apache-2.0

#![no_std]
// allow non-idiomatic names for generated code
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Can be removed once https://github.com/rust-lang/rust-bindgen/issues/1651 is resolved.
#![allow(deref_nullptr)]

// Force link `util` for tests
#[cfg(test)]
extern crate util;
// Force link `bitbox-framed-serial-olink` for tests
#[cfg(test)]
extern crate bitbox_framed_serial_link;
// Force link `rust-bytequeue` for tests
#[cfg(test)]
extern crate bitbox_bytequeue;

// include our generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
