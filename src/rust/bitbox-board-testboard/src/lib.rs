// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[path = "../../bitbox-platform-stm32u5/src/bitbox03_memory.rs"]
pub mod memory;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[unsafe(no_mangle)]
extern "C" fn assert_failed(_file: *mut u8, line: u32) -> ! {
    panic!("ST HAL assertion failed at line {line}");
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn init() -> bitbox_platform_stm32u5::Peripherals {
    bitbox_platform_stm32u5::init(Default::default())
}
