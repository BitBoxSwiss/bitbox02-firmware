// SPDX-License-Identifier: Apache-2.0

use bitbox_mcu_stm32u5 as _;
#[cfg(feature = "stm32u5a9")]
use bitbox_mcu_stm32u5::pac::{RCC, rcc::regs};
use cortex_m_rt::exception;
#[cfg(feature = "stm32u5a9")]
use cortex_m_rt::pre_init;

mod ffi {
    pub use bitbox_platform_stm32u5_sys::*;
}

#[exception]
fn SysTick() {
    unsafe {
        ffi::HAL_IncTick();
    }
}

/// Rust port of the ST `SystemInit()` startup routine.
///
/// This intentionally keeps only the CMSIS/HAL compatibility work that must run
/// before regular Rust runtime startup: reset the RCC clock configuration to
/// the default state expected by ST HAL.
///
/// The following parts of ST's `system_stm32u5xx.c` are intentionally left out:
/// - FPU access setup: handled by the Rust target/runtime configuration.
/// - Vector table relocation: handled by `cortex-m-rt` with the `set-vtor`
///   feature and the linker script.
/// - Reset handler, stack setup, `.data`/`.bss` initialization and calling
///   Rust `main`: handled by `cortex-m-rt`.
/// - `SystemCoreClock`, `AHBPrescTable`, `APBPrescTable` and `MSIRangeTable`:
///   provided by `bitbox-platform-stm32u5-sys` as CMSIS compatibility symbols.
/// - `SystemCoreClockUpdate()`: not currently needed; HAL clock configuration
///   updates `SystemCoreClock` through the HAL RCC code we link.
#[cfg(feature = "stm32u5a9")]
#[pre_init]
unsafe fn system_init() {
    // Reset the RCC clock configuration to the default reset state.
    // RCC_CR resets to 0x35 with both MSIS and MSIK enabled.
    RCC.cr().write_value(regs::Cr(0x35));
    RCC.cfgr1().write_value(regs::Cfgr1(0));
    RCC.cfgr2().write_value(regs::Cfgr2(0));
    RCC.cfgr3().write_value(regs::Cfgr3(0));
    RCC.cr().modify(|w| {
        w.set_hseon(false);
        w.set_csson(false);
        w.set_hsion(false);
        w.set_pllon(0, false);
        w.set_pllon(1, false);
        w.set_pllon(2, false);
    });
    RCC.pll1cfgr().write_value(regs::Pll1cfgr(0));
    RCC.cr().modify(|w| w.set_hsebyp(false));
    RCC.cier().write_value(regs::Cier(0));
}
