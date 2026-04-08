// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod ffi {
    pub use bitbox_stm32u5_sys::*;
}

pub mod uart;
