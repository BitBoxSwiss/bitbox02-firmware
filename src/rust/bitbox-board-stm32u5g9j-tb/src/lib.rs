// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[path = "../../bitbox-platform-stm32u5/src/bitbox03_memory.rs"]
pub mod memory;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn init() -> embassy_stm32::Peripherals {
    use bitbox_platform_stm32u5_sys as ffi;
    use embassy_stm32::rcc::{
        AHBPrescaler, APBPrescaler, MSIRange, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk,
        VoltageScale,
    };

    let mut config = embassy_stm32::Config::default();
    config.rcc.msis = Some(MSIRange::RANGE_48MHZ);
    config.rcc.pll1 = Some(Pll {
        source: PllSource::MSIS,
        prediv: PllPreDiv::DIV3,
        mul: PllMul::MUL10,
        divp: Some(PllDiv::DIV8),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV1),
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.apb3_pre = APBPrescaler::DIV1;
    config.rcc.voltage_range = VoltageScale::RANGE1;

    // Configure clocks before HAL_Init reads them to update SystemCoreClock and the HAL tick.
    let peripherals = embassy_stm32::init(config);
    assert_eq!(unsafe { ffi::HAL_Init() }, ffi::HAL_StatusTypeDef::HAL_OK);
    peripherals
}
