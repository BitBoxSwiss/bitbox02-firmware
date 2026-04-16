// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod ffi {
    pub use bitbox_platform_stm32u5_sys::*;

    unsafe extern "C" {
        pub static mut huart1: UART_HandleTypeDef;
    }
}

pub mod uart;
pub mod usbx;
