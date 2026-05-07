// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod memory;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn init() {
    unsafe {
        let _ = ffi::HAL_Init();
        ffi::SystemPower_Config();
        ffi::SystemClock_Config();
        ffi::PeriphCommonClock_Config();
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub fn init() {}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub mod ffi {
    #[allow(unused_imports)]
    pub use bitbox_platform_stm32u5_sys::*;

    unsafe extern "C" {
        pub fn GPU2D_IRQHandler();
        pub fn GPU2D_ER_IRQHandler();
        pub fn LTDC_IRQHandler();
    }
}
