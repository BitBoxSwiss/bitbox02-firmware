// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod ffi {
    pub use bitbox_platform_stm32u5_sys::*;

    unsafe extern "C" {
        pub fn board_init() -> i32;
        pub fn board_init_essentials() -> i32;
        pub fn GPU2D_IRQHandler();
        pub fn GPU2D_ER_IRQHandler();
        pub fn LTDC_IRQHandler();
    }
}
