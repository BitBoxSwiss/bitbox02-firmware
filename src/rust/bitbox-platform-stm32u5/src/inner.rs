// SPDX-License-Identifier: Apache-2.0

use bitbox_mcu_stm32u5 as _;
use bitbox_platform_stm32u5_sys as ffi;
use cortex_m_rt::exception;

#[exception]
fn SysTick() {
    unsafe {
        ffi::HAL_IncTick();
    }
}

pub fn init(mut config: embassy_stm32::Config) -> embassy_stm32::Peripherals {
    use embassy_stm32::rcc::{
        AHBPrescaler, APBPrescaler, MSIRange, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk,
        VoltageScale,
    };

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

    embassy_stm32::rcc::enable_and_reset::<embassy_stm32::peripherals::PWR>();
    if unsafe { ffi::HAL_PWREx_GetSupplyConfig() } == ffi::PWR_LDO_SUPPLY {
        // AN5766 requires SYSCLK below 100 MHz for the LDO-to-SMPS switch at 85–125 °C.
        assert!(unsafe { ffi::HAL_RCC_GetSysClockFreq() } < 100_000_000);
        // Refresh SystemCoreClock for the HAL supply-switch timeout.
        unsafe { ffi::HAL_RCC_GetHCLKFreq() };
        assert_eq!(
            unsafe { ffi::HAL_PWREx_ConfigSupply(ffi::PWR_SMPS_SUPPLY) },
            ffi::HAL_StatusTypeDef::HAL_OK
        );
    }

    // Configure clocks before HAL_Init reads them to update SystemCoreClock and the HAL tick.
    let peripherals = embassy_stm32::init(config);
    assert_eq!(unsafe { ffi::HAL_Init() }, ffi::HAL_StatusTypeDef::HAL_OK);
    peripherals
}
