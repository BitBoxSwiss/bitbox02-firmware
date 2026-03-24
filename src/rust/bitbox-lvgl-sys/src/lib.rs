// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// bindgen fails to generate LV_SIZE_CONTENT
pub const LV_SIZE_CONTENT: u32 = LV_COORD_MAX | (1 << LV_COORD_TYPE_SHIFT);
