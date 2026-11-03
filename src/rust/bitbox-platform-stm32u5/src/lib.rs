// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub mod flash;

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod inner;

/// Milliseconds since HAL startup, wrapping after `u32::MAX`.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn millis() -> u32 {
    // HAL_GetTick reads the counter maintained by the platform's SysTick handler.
    unsafe { bitbox_platform_stm32u5_sys::HAL_GetTick() }
}

#[cfg(all(feature = "usb", target_arch = "arm", target_os = "none"))]
pub mod usb;
