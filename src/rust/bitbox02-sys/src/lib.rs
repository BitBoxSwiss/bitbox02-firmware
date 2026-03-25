// SPDX-License-Identifier: Apache-2.0

#![no_std]
// allow non-idiomatic names for generated code
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Can be removed once https://github.com/rust-lang/rust-bindgen/issues/1651 is resolved.
#![allow(deref_nullptr)]

#[allow(unused_extern_crates)]
extern crate bitbox_samd52;
#[allow(unused_extern_crates)]
extern crate cryptoauthlib_sys;
#[allow(unused_extern_crates)]
extern crate fatfs_sys;

// Host targets link fatfs and bitbox02 static archives separately. Keep one explicit reference to
// a diskio symbol so the fake diskio object is pulled from libbitbox02 even if fatfs appears later
// in archive processing order.
#[cfg(not(target_arch = "arm"))]
unsafe extern "C" {
    fn disk_status(pdrv: u8) -> u8;
}

#[cfg(not(target_arch = "arm"))]
#[used]
static DISK_STATUS_LINK_ANCHOR: unsafe extern "C" fn(u8) -> u8 = disk_status;

// include our generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
