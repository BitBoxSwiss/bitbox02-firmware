// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use cortex_m_rt::exception;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[exception]
fn SysTick() {
    unsafe {
        HAL_IncTick();
    }
}

#[cortex_m_rt::pre_init]
unsafe fn init_safe_clocks_and_heap() {
    // Typically called in Reset_Handler
    unsafe { SystemInit() };
    {}
}
