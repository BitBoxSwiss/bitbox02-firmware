// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub mod ffi {
    pub use bitbox_board_stm32u5a9j_dk_sys::*;
    #[allow(unused_imports)]
    pub use bitbox_platform_stm32u5_sys::*;
}

#[path = "../../bitbox-platform-stm32u5/src/bitbox03_memory.rs"]
pub mod memory;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn init() {
    let mut config = bitbox_platform_stm32u5::Config::default();
    config.enable_independent_io_supply = false;
    config.enable_independent_analog_supply = false;
    bitbox_platform_stm32u5::init(config);
    unsafe {
        ffi::MX_FLASH_Init();
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub fn init() {}
