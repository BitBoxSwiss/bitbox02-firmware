// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Can be removed once https://github.com/rust-lang/rust-bindgen/issues/1651 is resolved.
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
