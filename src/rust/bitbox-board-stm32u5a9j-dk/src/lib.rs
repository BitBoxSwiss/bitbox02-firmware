// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub mod ffi {
    pub use bitbox_board_stm32u5a9j_dk_sys::*;
    #[allow(unused_imports)]
    pub use bitbox_platform_stm32u5_sys::*;
}

pub mod memory;
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub mod storage;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn init() {
    unsafe {
        let _ = ffi::HAL_Init();
        ffi::SystemPower_Config();
        ffi::SystemClock_Config();
        ffi::MX_FLASH_Init();
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub fn init() {}
