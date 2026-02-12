// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cortex_m_rt::exception]
fn SysTick() {
    unsafe {
        HAL_IncTick();
    }
}

#[cortex_m_rt::pre_init]
unsafe fn call_st_system_init() {
    // Typically called in Reset_Handler
    unsafe { SystemInit() };
}

// pub union Vector {
//     handler: unsafe extern "C" fn(),
//     reserved: usize,
// }
//
// #[unsafe(link_section = ".Vector_table.interrupts")]
// #[unsafe(no_mangle)]
// pub static __INTERRUPTS: [Vector; 139] = [
//     Vector {
//         handler: WWDG_IRQHandler,
//     },
//     Vector {
//         handler: PVD_PVM_IRQHandler,
//     },
//     Vector {
//         handler: RTC_IRQHandler,
//     },
//     Vector {
//         handler: RTC_S_IRQHandler,
//     },
//     Vector {
//         handler: TAMP_IRQHandler,
//     },
//     Vector {
//         handler: RAMCFG_IRQHandler,
//     },
//     Vector {
//         handler: FLASH_IRQHandler,
//     },
//     Vector {
//         handler: FLASH_S_IRQHandler,
//     },
//     Vector {
//         handler: GTZC_IRQHandler,
//     },
//     Vector {
//         handler: RCC_IRQHandler,
//     },
//     Vector {
//         handler: RCC_S_IRQHandler,
//     },
//     Vector {
//         handler: EXTI0_IRQHandler,
//     },
//     Vector {
//         handler: EXTI1_IRQHandler,
//     },
//     Vector {
//         handler: EXTI2_IRQHandler,
//     },
//     Vector {
//         handler: EXTI3_IRQHandler,
//     },
//     Vector {
//         handler: EXTI4_IRQHandler,
//     },
//     Vector {
//         handler: EXTI5_IRQHandler,
//     },
//     Vector {
//         handler: EXTI6_IRQHandler,
//     },
//     Vector {
//         handler: EXTI7_IRQHandler,
//     },
//     Vector {
//         handler: EXTI8_IRQHandler,
//     },
//     Vector {
//         handler: EXTI9_IRQHandler,
//     },
//     Vector {
//         handler: EXTI10_IRQHandler,
//     },
//     Vector {
//         handler: EXTI11_IRQHandler,
//     },
//     Vector {
//         handler: EXTI12_IRQHandler,
//     },
//     Vector {
//         handler: EXTI13_IRQHandler,
//     },
//     Vector {
//         handler: EXTI14_IRQHandler,
//     },
//     Vector {
//         handler: EXTI15_IRQHandler,
//     },
//     Vector {
//         handler: IWDG_IRQHandler,
//     },
//     Vector {
//         handler: SAES_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel0_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel1_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel2_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel3_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel4_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel5_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel6_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel7_IRQHandler,
//     },
//     Vector {
//         handler: ADC1_2_IRQHandler,
//     },
//     Vector {
//         handler: DAC1_IRQHandler,
//     },
//     Vector {
//         handler: FDCAN1_IT0_IRQHandler,
//     },
//     Vector {
//         handler: FDCAN1_IT1_IRQHandler,
//     },
//     Vector {
//         handler: TIM1_BRK_IRQHandler,
//     },
//     Vector {
//         handler: TIM1_UP_IRQHandler,
//     },
//     Vector {
//         handler: TIM1_TRG_COM_IRQHandler,
//     },
//     Vector {
//         handler: TIM1_CC_IRQHandler,
//     },
//     Vector {
//         handler: TIM2_IRQHandler,
//     },
//     Vector {
//         handler: TIM3_IRQHandler,
//     },
//     Vector {
//         handler: TIM4_IRQHandler,
//     },
//     Vector {
//         handler: TIM5_IRQHandler,
//     },
//     Vector {
//         handler: TIM6_IRQHandler,
//     },
//     Vector {
//         handler: TIM7_IRQHandler,
//     },
//     Vector {
//         handler: TIM8_BRK_IRQHandler,
//     },
//     Vector {
//         handler: TIM8_UP_IRQHandler,
//     },
//     Vector {
//         handler: TIM8_TRG_COM_IRQHandler,
//     },
//     Vector {
//         handler: TIM8_CC_IRQHandler,
//     },
//     Vector {
//         handler: I2C1_EV_IRQHandler,
//     },
//     Vector {
//         handler: I2C1_ER_IRQHandler,
//     },
//     Vector {
//         handler: I2C2_EV_IRQHandler,
//     },
//     Vector {
//         handler: I2C2_ER_IRQHandler,
//     },
//     Vector {
//         handler: SPI1_IRQHandler,
//     },
//     Vector {
//         handler: SPI2_IRQHandler,
//     },
//     Vector {
//         handler: USART1_IRQHandler,
//     },
//     Vector {
//         handler: USART2_IRQHandler,
//     },
//     Vector {
//         handler: USART3_IRQHandler,
//     },
//     Vector {
//         handler: UART4_IRQHandler,
//     },
//     Vector {
//         handler: UART5_IRQHandler,
//     },
//     Vector {
//         handler: LPUART1_IRQHandler,
//     },
//     Vector {
//         handler: LPTIM1_IRQHandler,
//     },
//     Vector {
//         handler: LPTIM2_IRQHandler,
//     },
//     Vector {
//         handler: TIM15_IRQHandler,
//     },
//     Vector {
//         handler: TIM16_IRQHandler,
//     },
//     Vector {
//         handler: TIM17_IRQHandler,
//     },
//     Vector {
//         handler: COMP_IRQHandler,
//     },
//     Vector {
//         handler: OTG_HS_IRQHandler,
//     },
//     Vector {
//         handler: CRS_IRQHandler,
//     },
//     Vector {
//         handler: FMC_IRQHandler,
//     },
//     Vector {
//         handler: OCTOSPI1_IRQHandler,
//     },
//     Vector {
//         handler: PWR_S3WU_IRQHandler,
//     },
//     Vector {
//         handler: SDMMC1_IRQHandler,
//     },
//     Vector {
//         handler: SDMMC2_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel8_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel9_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel10_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel11_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel12_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel13_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel14_IRQHandler,
//     },
//     Vector {
//         handler: GPDMA1_Channel15_IRQHandler,
//     },
//     Vector {
//         handler: I2C3_EV_IRQHandler,
//     },
//     Vector {
//         handler: I2C3_ER_IRQHandler,
//     },
//     Vector {
//         handler: SAI1_IRQHandler,
//     },
//     Vector {
//         handler: SAI2_IRQHandler,
//     },
//     Vector {
//         handler: TSC_IRQHandler,
//     },
//     Vector {
//         handler: AES_IRQHandler,
//     },
//     Vector {
//         handler: RNG_IRQHandler,
//     },
//     Vector {
//         handler: FPU_IRQHandler,
//     },
//     Vector {
//         handler: HASH_IRQHandler,
//     },
//     Vector {
//         handler: PKA_IRQHandler,
//     },
//     Vector {
//         handler: LPTIM3_IRQHandler,
//     },
//     Vector {
//         handler: SPI3_IRQHandler,
//     },
//     Vector {
//         handler: I2C4_ER_IRQHandler,
//     },
//     Vector {
//         handler: I2C4_EV_IRQHandler,
//     },
//     Vector {
//         handler: MDF1_FLT0_IRQHandler,
//     },
//     Vector {
//         handler: MDF1_FLT1_IRQHandler,
//     },
//     Vector {
//         handler: MDF1_FLT2_IRQHandler,
//     },
//     Vector {
//         handler: MDF1_FLT3_IRQHandler,
//     },
//     Vector {
//         handler: UCPD1_IRQHandler,
//     },
//     Vector {
//         handler: ICACHE_IRQHandler,
//     },
//     Vector {
//         handler: OTFDEC1_IRQHandler,
//     },
//     Vector {
//         handler: OTFDEC2_IRQHandler,
//     },
//     Vector {
//         handler: LPTIM4_IRQHandler,
//     },
//     Vector {
//         handler: DCACHE1_IRQHandler,
//     },
//     Vector {
//         handler: ADF1_IRQHandler,
//     },
//     Vector {
//         handler: ADC4_IRQHandler,
//     },
//     Vector {
//         handler: LPDMA1_Channel0_IRQHandler,
//     },
//     Vector {
//         handler: LPDMA1_Channel1_IRQHandler,
//     },
//     Vector {
//         handler: LPDMA1_Channel2_IRQHandler,
//     },
//     Vector {
//         handler: LPDMA1_Channel3_IRQHandler,
//     },
//     Vector {
//         handler: DMA2D_IRQHandler,
//     },
//     Vector {
//         handler: DCMI_PSSI_IRQHandler,
//     },
//     Vector {
//         handler: OCTOSPI2_IRQHandler,
//     },
//     Vector {
//         handler: MDF1_FLT4_IRQHandler,
//     },
//     Vector {
//         handler: MDF1_FLT5_IRQHandler,
//     },
//     Vector {
//         handler: CORDIC_IRQHandler,
//     },
//     Vector {
//         handler: FMAC_IRQHandler,
//     },
//     Vector {
//         handler: LSECSSD_IRQHandler,
//     },
//     Vector {
//         handler: USART6_IRQHandler,
//     },
//     Vector {
//         handler: I2C5_ER_IRQHandler,
//     },
//     Vector {
//         handler: I2C5_EV_IRQHandler,
//     },
//     Vector {
//         handler: I2C6_ER_IRQHandler,
//     },
//     Vector {
//         handler: I2C6_EV_IRQHandler,
//     },
//     Vector {
//         handler: HSPI1_IRQHandler,
//     },
//     Vector {
//         handler: GPU2D_IRQHandler,
//     },
//     Vector {
//         handler: GPU2D_ER_IRQHandler,
//     },
//     Vector {
//         handler: GFXMMU_IRQHandler,
//     },
//     Vector {
//         handler: LTDC_IRQHandler,
//     },
//     Vector {
//         handler: LTDC_ER_IRQHandler,
//     },
//     Vector {
//         handler: DSI_IRQHandler,
//     },
//     Vector {
//         handler: DCACHE2_IRQHandler,
//     },
// ];
