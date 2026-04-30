// SPDX-License-Identifier: Apache-2.0

#![no_std]

use bitbox_mcu_stm32u5::pac::RCC;
use cortex_m_rt::exception;
use cortex_m_rt::pre_init;

pub mod ffi {
    pub use bitbox_platform_stm32u5_sys::*;

    unsafe extern "C" {
        pub static mut huart1: UART_HandleTypeDef;
    }
}

pub mod flash;
pub mod hash;
pub mod otp;
pub mod timer;
pub mod uart;
pub mod usbx;

#[exception]
fn SysTick() {
    unsafe {
        ffi::HAL_IncTick();
    }
}

/// Rust port of the ST `SystemInit()` startup routine.
///
/// This intentionally keeps only the CMSIS/HAL compatibility work that must
/// happen before regular Rust runtime startup.
#[pre_init]
unsafe fn system_init() {
    let rcc = unsafe { &*RCC::PTR };

    // Reset the RCC clock configuration to the default reset state.
    rcc.cr().write(|w| w.msison().enabled());
    rcc.cfgr1().write(|w| unsafe { w.bits(0) });
    rcc.cfgr2().write(|w| unsafe { w.bits(0) });
    rcc.cfgr3().write(|w| unsafe { w.bits(0) });
    rcc.cr().modify(|r, w| unsafe {
        w.bits(
            r.bits()
                & !((1_u32 << 16) | (1_u32 << 19) | (1_u32 << 24) | (1_u32 << 26) | (1_u32 << 28)),
        )
    });
    rcc.pll1cfgr().write(|w| unsafe { w.bits(0) });
    rcc.cr()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << 18)) });
    rcc.cier().write(|w| unsafe { w.bits(0) });
}
