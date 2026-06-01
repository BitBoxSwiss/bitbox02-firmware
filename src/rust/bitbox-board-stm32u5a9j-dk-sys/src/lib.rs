// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
