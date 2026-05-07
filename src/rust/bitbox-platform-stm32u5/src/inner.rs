// SPDX-License-Identifier: Apache-2.0

use bitbox_mcu_stm32u5::pac::RCC;
use cortex_m_rt::{exception, pre_init};

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
#[pre_init]
unsafe fn system_init() {
    let rcc = unsafe { &*RCC::PTR };

    // Reset the RCC clock configuration to the default reset state.
    rcc.cr().write(|w| w.msison().enabled());
    rcc.cfgr1().write(|w| unsafe { w.bits(0) });
    rcc.cfgr2().write(|w| unsafe { w.bits(0) });
    rcc.cfgr3().write(|w| unsafe { w.bits(0) });
    rcc.cr().modify(|_, w| {
        w.hseon()
            .disabled()
            .csson()
            .disabled()
            .hsion()
            .disabled()
            .pll1on()
            .disabled()
            .pll2on()
            .disabled()
            .pll3on()
            .disabled()
    });
    rcc.pll1cfgr().write(|w| unsafe { w.bits(0) });
    rcc.cr().modify(|_, w| w.hsebyp().not_bypassed());
    rcc.cier().write(|w| unsafe { w.bits(0) });
}
