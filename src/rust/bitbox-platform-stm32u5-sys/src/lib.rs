// SPDX-License-Identifier: Apache-2.0

//! Low-level STM32U5 HAL bindings and CMSIS compatibility symbols.
//!
//! The ST/CMSIS startup model expects `system_stm32u5xx.c` to be linked. In the
//! Cube-generated board project this file is copied from ST's STM32U5 system
//! template. We do not compile that C file directly: the Rust runtime already
//! provides the reset handler, vector table, stack setup, `.data`/`.bss`
//! initialization and the call into Rust `main`.
//!
//! The remaining `SystemInit()` work that must happen before normal runtime
//! startup is implemented in `bitbox-platform-stm32u5` using
//! `cortex_m_rt::pre_init`. This crate only exports the CMSIS clock globals and
//! lookup tables that the ST HAL still references. It intentionally does not
//! provide `SystemInit()`, `SystemCoreClockUpdate()` or any reset/startup
//! handler from ST's C template.

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// These CMSIS clock symbols mirror ST's STM32U5 system template definitions in
// `system_stm32u5xx.c`.
#[unsafe(export_name = "SystemCoreClock")]
static mut SYSTEM_CORE_CLOCK: u32 = 4_000_000;

#[unsafe(export_name = "AHBPrescTable")]
static AHB_PRESC_TABLE: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9];

#[unsafe(export_name = "APBPrescTable")]
static APB_PRESC_TABLE: [u8; 8] = [0, 0, 0, 0, 1, 2, 3, 4];

#[unsafe(export_name = "MSIRangeTable")]
static MSI_RANGE_TABLE: [u32; 16] = [
    48_000_000, 24_000_000, 16_000_000, 12_000_000, 4_000_000, 2_000_000, 1_330_000, 1_000_000,
    3_072_000, 1_536_000, 1_024_000, 768_000, 400_000, 200_000, 133_000, 100_000,
];
