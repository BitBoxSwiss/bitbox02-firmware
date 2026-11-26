#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD_AVD"]
    PVD_AVD = 1,
    #[doc = "2 - TAMP_STAMP"]
    TAMP_STAMP = 2,
    #[doc = "3 - RTC_WKUP"]
    RTC_WKUP = 3,
    #[doc = "4 - FLASH"]
    FLASH = 4,
    #[doc = "5 - RCC"]
    RCC = 5,
    #[doc = "6 - EXTI0"]
    EXTI0 = 6,
    #[doc = "7 - EXTI1"]
    EXTI1 = 7,
    #[doc = "8 - EXTI2"]
    EXTI2 = 8,
    #[doc = "9 - EXTI3"]
    EXTI3 = 9,
    #[doc = "10 - EXTI4"]
    EXTI4 = 10,
    #[doc = "11 - DMA1_STREAM0"]
    DMA1_STREAM0 = 11,
    #[doc = "12 - DMA1_STREAM1"]
    DMA1_STREAM1 = 12,
    #[doc = "13 - DMA1_STREAM2"]
    DMA1_STREAM2 = 13,
    #[doc = "14 - DMA1_STREAM3"]
    DMA1_STREAM3 = 14,
    #[doc = "15 - DMA1_STREAM4"]
    DMA1_STREAM4 = 15,
    #[doc = "16 - DMA1_STREAM5"]
    DMA1_STREAM5 = 16,
    #[doc = "17 - DMA1_STREAM6"]
    DMA1_STREAM6 = 17,
    #[doc = "18 - ADC"]
    ADC = 18,
    #[doc = "19 - FDCAN1_IT0"]
    FDCAN1_IT0 = 19,
    #[doc = "20 - FDCAN2_IT0"]
    FDCAN2_IT0 = 20,
    #[doc = "21 - FDCAN1_IT1"]
    FDCAN1_IT1 = 21,
    #[doc = "22 - FDCAN2_IT1"]
    FDCAN2_IT1 = 22,
    #[doc = "23 - EXTI9_5"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1_BRK"]
    TIM1_BRK = 24,
    #[doc = "25 - TIM1_UP"]
    TIM1_UP = 25,
    #[doc = "26 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 26,
    #[doc = "27 - TIM1_CC"]
    TIM1_CC = 27,
    #[doc = "28 - TIM2"]
    TIM2 = 28,
    #[doc = "29 - TIM3"]
    TIM3 = 29,
    #[doc = "30 - TIM4"]
    TIM4 = 30,
    #[doc = "31 - I2C1_EV"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1_ER"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2_EV"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2_ER"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1"]
    SPI1 = 35,
    #[doc = "36 - SPI2"]
    SPI2 = 36,
    #[doc = "37 - USART1"]
    USART1 = 37,
    #[doc = "38 - USART2"]
    USART2 = 38,
    #[doc = "39 - USART3"]
    USART3 = 39,
    #[doc = "40 - EXTI15_10"]
    EXTI15_10 = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "43 - TIM8_BRK_TIM12"]
    TIM8_BRK_TIM12 = 43,
    #[doc = "44 - TIM8_UP_TIM13"]
    TIM8_UP_TIM13 = 44,
    #[doc = "45 - TIM8_TRG_COM_TIM14"]
    TIM8_TRG_COM_TIM14 = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "47 - DMA1_STREAM7"]
    DMA1_STREAM7 = 47,
    #[doc = "48 - FMC"]
    FMC = 48,
    #[doc = "49 - SDMMC1"]
    SDMMC1 = 49,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
    #[doc = "52 - UART4"]
    UART4 = 52,
    #[doc = "53 - UART5"]
    UART5 = 53,
    #[doc = "54 - TIM6_DAC"]
    TIM6_DAC = 54,
    #[doc = "55 - TIM7"]
    TIM7 = 55,
    #[doc = "56 - DMA2_STREAM0"]
    DMA2_STREAM0 = 56,
    #[doc = "57 - DMA2_STREAM1"]
    DMA2_STREAM1 = 57,
    #[doc = "58 - DMA2_STREAM2"]
    DMA2_STREAM2 = 58,
    #[doc = "59 - DMA2_STREAM3"]
    DMA2_STREAM3 = 59,
    #[doc = "60 - DMA2_STREAM4"]
    DMA2_STREAM4 = 60,
    #[doc = "61 - ETH"]
    ETH = 61,
    #[doc = "62 - ETH_WKUP"]
    ETH_WKUP = 62,
    #[doc = "63 - FDCAN_CAL"]
    FDCAN_CAL = 63,
    #[doc = "64 - CM7_SEV"]
    CM7_SEV = 64,
    #[doc = "65 - CM4_SEV"]
    CM4_SEV = 65,
    #[doc = "68 - DMA2_STREAM5"]
    DMA2_STREAM5 = 68,
    #[doc = "69 - DMA2_STREAM6"]
    DMA2_STREAM6 = 69,
    #[doc = "70 - DMA2_STREAM7"]
    DMA2_STREAM7 = 70,
    #[doc = "71 - USART6"]
    USART6 = 71,
    #[doc = "72 - I2C3_EV"]
    I2C3_EV = 72,
    #[doc = "73 - I2C3_ER"]
    I2C3_ER = 73,
    #[doc = "74 - OTG_HS_EP1_OUT"]
    OTG_HS_EP1_OUT = 74,
    #[doc = "75 - OTG_HS_EP1_IN"]
    OTG_HS_EP1_IN = 75,
    #[doc = "76 - OTG_HS_WKUP"]
    OTG_HS_WKUP = 76,
    #[doc = "77 - OTG_HS"]
    OTG_HS = 77,
    #[doc = "78 - DCMI"]
    DCMI = 78,
    #[doc = "80 - RNG"]
    RNG = 80,
    #[doc = "81 - FPU"]
    FPU = 81,
    #[doc = "82 - UART7"]
    UART7 = 82,
    #[doc = "83 - UART8"]
    UART8 = 83,
    #[doc = "84 - SPI4"]
    SPI4 = 84,
    #[doc = "85 - SPI5"]
    SPI5 = 85,
    #[doc = "86 - SPI6"]
    SPI6 = 86,
    #[doc = "87 - SAI1"]
    SAI1 = 87,
    #[doc = "88 - LTDC"]
    LTDC = 88,
    #[doc = "89 - LTDC_ER"]
    LTDC_ER = 89,
    #[doc = "90 - DMA2D"]
    DMA2D = 90,
    #[doc = "91 - SAI2"]
    SAI2 = 91,
    #[doc = "92 - QUADSPI"]
    QUADSPI = 92,
    #[doc = "93 - LPTIM1"]
    LPTIM1 = 93,
    #[doc = "94 - CEC"]
    CEC = 94,
    #[doc = "95 - I2C4_EV"]
    I2C4_EV = 95,
    #[doc = "96 - I2C4_ER"]
    I2C4_ER = 96,
    #[doc = "97 - SPDIF_RX"]
    SPDIF_RX = 97,
    #[doc = "98 - OTG_FS_EP1_OUT"]
    OTG_FS_EP1_OUT = 98,
    #[doc = "99 - OTG_FS_EP1_IN"]
    OTG_FS_EP1_IN = 99,
    #[doc = "100 - OTG_FS_WKUP"]
    OTG_FS_WKUP = 100,
    #[doc = "101 - OTG_FS"]
    OTG_FS = 101,
    #[doc = "102 - DMAMUX1_OVR"]
    DMAMUX1_OVR = 102,
    #[doc = "103 - HRTIM1_MASTER"]
    HRTIM1_MASTER = 103,
    #[doc = "104 - HRTIM1_TIMA"]
    HRTIM1_TIMA = 104,
    #[doc = "105 - HRTIM1_TIMB"]
    HRTIM1_TIMB = 105,
    #[doc = "106 - HRTIM1_TIMC"]
    HRTIM1_TIMC = 106,
    #[doc = "107 - HRTIM1_TIMD"]
    HRTIM1_TIMD = 107,
    #[doc = "108 - HRTIM1_TIME"]
    HRTIM1_TIME = 108,
    #[doc = "109 - HRTIM1_FLT"]
    HRTIM1_FLT = 109,
    #[doc = "110 - DFSDM1_FLT0"]
    DFSDM1_FLT0 = 110,
    #[doc = "111 - DFSDM1_FLT1"]
    DFSDM1_FLT1 = 111,
    #[doc = "112 - DFSDM1_FLT2"]
    DFSDM1_FLT2 = 112,
    #[doc = "113 - DFSDM1_FLT3"]
    DFSDM1_FLT3 = 113,
    #[doc = "114 - SAI3"]
    SAI3 = 114,
    #[doc = "115 - SWPMI1"]
    SWPMI1 = 115,
    #[doc = "116 - TIM15"]
    TIM15 = 116,
    #[doc = "117 - TIM16"]
    TIM16 = 117,
    #[doc = "118 - TIM17"]
    TIM17 = 118,
    #[doc = "119 - MDIOS_WKUP"]
    MDIOS_WKUP = 119,
    #[doc = "120 - MDIOS"]
    MDIOS = 120,
    #[doc = "121 - JPEG"]
    JPEG = 121,
    #[doc = "122 - MDMA"]
    MDMA = 122,
    #[doc = "123 - DSI"]
    DSI = 123,
    #[doc = "124 - SDMMC2"]
    SDMMC2 = 124,
    #[doc = "125 - HSEM1"]
    HSEM1 = 125,
    #[doc = "126 - HSEM2"]
    HSEM2 = 126,
    #[doc = "127 - ADC3"]
    ADC3 = 127,
    #[doc = "128 - DMAMUX2_OVR"]
    DMAMUX2_OVR = 128,
    #[doc = "129 - BDMA_CHANNEL0"]
    BDMA_CHANNEL0 = 129,
    #[doc = "130 - BDMA_CHANNEL1"]
    BDMA_CHANNEL1 = 130,
    #[doc = "131 - BDMA_CHANNEL2"]
    BDMA_CHANNEL2 = 131,
    #[doc = "132 - BDMA_CHANNEL3"]
    BDMA_CHANNEL3 = 132,
    #[doc = "133 - BDMA_CHANNEL4"]
    BDMA_CHANNEL4 = 133,
    #[doc = "134 - BDMA_CHANNEL5"]
    BDMA_CHANNEL5 = 134,
    #[doc = "135 - BDMA_CHANNEL6"]
    BDMA_CHANNEL6 = 135,
    #[doc = "136 - BDMA_CHANNEL7"]
    BDMA_CHANNEL7 = 136,
    #[doc = "137 - COMP"]
    COMP = 137,
    #[doc = "138 - LPTIM2"]
    LPTIM2 = 138,
    #[doc = "139 - LPTIM3"]
    LPTIM3 = 139,
    #[doc = "140 - LPTIM4"]
    LPTIM4 = 140,
    #[doc = "141 - LPTIM5"]
    LPTIM5 = 141,
    #[doc = "142 - LPUART1"]
    LPUART1 = 142,
    #[doc = "143 - WWDG_RST"]
    WWDG_RST = 143,
    #[doc = "144 - CRS"]
    CRS = 144,
    #[doc = "145 - ECC"]
    ECC = 145,
    #[doc = "146 - SAI4"]
    SAI4 = 146,
    #[doc = "148 - HOLD_CORE"]
    HOLD_CORE = 148,
    #[doc = "149 - WAKEUP_PIN"]
    WAKEUP_PIN = 149,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn WWDG();
        fn PVD_AVD();
        fn TAMP_STAMP();
        fn RTC_WKUP();
        fn FLASH();
        fn RCC();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_STREAM0();
        fn DMA1_STREAM1();
        fn DMA1_STREAM2();
        fn DMA1_STREAM3();
        fn DMA1_STREAM4();
        fn DMA1_STREAM5();
        fn DMA1_STREAM6();
        fn ADC();
        fn FDCAN1_IT0();
        fn FDCAN2_IT0();
        fn FDCAN1_IT1();
        fn FDCAN2_IT1();
        fn EXTI9_5();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn USART3();
        fn EXTI15_10();
        fn RTC_ALARM();
        fn TIM8_BRK_TIM12();
        fn TIM8_UP_TIM13();
        fn TIM8_TRG_COM_TIM14();
        fn TIM8_CC();
        fn DMA1_STREAM7();
        fn FMC();
        fn SDMMC1();
        fn TIM5();
        fn SPI3();
        fn UART4();
        fn UART5();
        fn TIM6_DAC();
        fn TIM7();
        fn DMA2_STREAM0();
        fn DMA2_STREAM1();
        fn DMA2_STREAM2();
        fn DMA2_STREAM3();
        fn DMA2_STREAM4();
        fn ETH();
        fn ETH_WKUP();
        fn FDCAN_CAL();
        fn CM7_SEV();
        fn CM4_SEV();
        fn DMA2_STREAM5();
        fn DMA2_STREAM6();
        fn DMA2_STREAM7();
        fn USART6();
        fn I2C3_EV();
        fn I2C3_ER();
        fn OTG_HS_EP1_OUT();
        fn OTG_HS_EP1_IN();
        fn OTG_HS_WKUP();
        fn OTG_HS();
        fn DCMI();
        fn RNG();
        fn FPU();
        fn UART7();
        fn UART8();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn SAI1();
        fn LTDC();
        fn LTDC_ER();
        fn DMA2D();
        fn SAI2();
        fn QUADSPI();
        fn LPTIM1();
        fn CEC();
        fn I2C4_EV();
        fn I2C4_ER();
        fn SPDIF_RX();
        fn OTG_FS_EP1_OUT();
        fn OTG_FS_EP1_IN();
        fn OTG_FS_WKUP();
        fn OTG_FS();
        fn DMAMUX1_OVR();
        fn HRTIM1_MASTER();
        fn HRTIM1_TIMA();
        fn HRTIM1_TIMB();
        fn HRTIM1_TIMC();
        fn HRTIM1_TIMD();
        fn HRTIM1_TIME();
        fn HRTIM1_FLT();
        fn DFSDM1_FLT0();
        fn DFSDM1_FLT1();
        fn DFSDM1_FLT2();
        fn DFSDM1_FLT3();
        fn SAI3();
        fn SWPMI1();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn MDIOS_WKUP();
        fn MDIOS();
        fn JPEG();
        fn MDMA();
        fn DSI();
        fn SDMMC2();
        fn HSEM1();
        fn HSEM2();
        fn ADC3();
        fn DMAMUX2_OVR();
        fn BDMA_CHANNEL0();
        fn BDMA_CHANNEL1();
        fn BDMA_CHANNEL2();
        fn BDMA_CHANNEL3();
        fn BDMA_CHANNEL4();
        fn BDMA_CHANNEL5();
        fn BDMA_CHANNEL6();
        fn BDMA_CHANNEL7();
        fn COMP();
        fn LPTIM2();
        fn LPTIM3();
        fn LPTIM4();
        fn LPTIM5();
        fn LPUART1();
        fn WWDG_RST();
        fn CRS();
        fn ECC();
        fn SAI4();
        fn HOLD_CORE();
        fn WAKEUP_PIN();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 150] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD_AVD },
        Vector { _handler: TAMP_STAMP },
        Vector { _handler: RTC_WKUP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector { _handler: DMA1_STREAM0 },
        Vector { _handler: DMA1_STREAM1 },
        Vector { _handler: DMA1_STREAM2 },
        Vector { _handler: DMA1_STREAM3 },
        Vector { _handler: DMA1_STREAM4 },
        Vector { _handler: DMA1_STREAM5 },
        Vector { _handler: DMA1_STREAM6 },
        Vector { _handler: ADC },
        Vector { _handler: FDCAN1_IT0 },
        Vector { _handler: FDCAN2_IT0 },
        Vector { _handler: FDCAN1_IT1 },
        Vector { _handler: FDCAN2_IT1 },
        Vector { _handler: EXTI9_5 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: EXTI15_10 },
        Vector { _handler: RTC_ALARM },
        Vector { _reserved: 0 },
        Vector {
            _handler: TIM8_BRK_TIM12,
        },
        Vector {
            _handler: TIM8_UP_TIM13,
        },
        Vector {
            _handler: TIM8_TRG_COM_TIM14,
        },
        Vector { _handler: TIM8_CC },
        Vector { _handler: DMA1_STREAM7 },
        Vector { _handler: FMC },
        Vector { _handler: SDMMC1 },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: TIM6_DAC },
        Vector { _handler: TIM7 },
        Vector { _handler: DMA2_STREAM0 },
        Vector { _handler: DMA2_STREAM1 },
        Vector { _handler: DMA2_STREAM2 },
        Vector { _handler: DMA2_STREAM3 },
        Vector { _handler: DMA2_STREAM4 },
        Vector { _handler: ETH },
        Vector { _handler: ETH_WKUP },
        Vector { _handler: FDCAN_CAL },
        Vector { _handler: CM7_SEV },
        Vector { _handler: CM4_SEV },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMA2_STREAM5 },
        Vector { _handler: DMA2_STREAM6 },
        Vector { _handler: DMA2_STREAM7 },
        Vector { _handler: USART6 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector {
            _handler: OTG_HS_EP1_OUT,
        },
        Vector {
            _handler: OTG_HS_EP1_IN,
        },
        Vector { _handler: OTG_HS_WKUP },
        Vector { _handler: OTG_HS },
        Vector { _handler: DCMI },
        Vector { _reserved: 0 },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector { _handler: SAI1 },
        Vector { _handler: LTDC },
        Vector { _handler: LTDC_ER },
        Vector { _handler: DMA2D },
        Vector { _handler: SAI2 },
        Vector { _handler: QUADSPI },
        Vector { _handler: LPTIM1 },
        Vector { _handler: CEC },
        Vector { _handler: I2C4_EV },
        Vector { _handler: I2C4_ER },
        Vector { _handler: SPDIF_RX },
        Vector {
            _handler: OTG_FS_EP1_OUT,
        },
        Vector {
            _handler: OTG_FS_EP1_IN,
        },
        Vector { _handler: OTG_FS_WKUP },
        Vector { _handler: OTG_FS },
        Vector { _handler: DMAMUX1_OVR },
        Vector {
            _handler: HRTIM1_MASTER,
        },
        Vector { _handler: HRTIM1_TIMA },
        Vector { _handler: HRTIM1_TIMB },
        Vector { _handler: HRTIM1_TIMC },
        Vector { _handler: HRTIM1_TIMD },
        Vector { _handler: HRTIM1_TIME },
        Vector { _handler: HRTIM1_FLT },
        Vector { _handler: DFSDM1_FLT0 },
        Vector { _handler: DFSDM1_FLT1 },
        Vector { _handler: DFSDM1_FLT2 },
        Vector { _handler: DFSDM1_FLT3 },
        Vector { _handler: SAI3 },
        Vector { _handler: SWPMI1 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: MDIOS_WKUP },
        Vector { _handler: MDIOS },
        Vector { _handler: JPEG },
        Vector { _handler: MDMA },
        Vector { _handler: DSI },
        Vector { _handler: SDMMC2 },
        Vector { _handler: HSEM1 },
        Vector { _handler: HSEM2 },
        Vector { _handler: ADC3 },
        Vector { _handler: DMAMUX2_OVR },
        Vector {
            _handler: BDMA_CHANNEL0,
        },
        Vector {
            _handler: BDMA_CHANNEL1,
        },
        Vector {
            _handler: BDMA_CHANNEL2,
        },
        Vector {
            _handler: BDMA_CHANNEL3,
        },
        Vector {
            _handler: BDMA_CHANNEL4,
        },
        Vector {
            _handler: BDMA_CHANNEL5,
        },
        Vector {
            _handler: BDMA_CHANNEL6,
        },
        Vector {
            _handler: BDMA_CHANNEL7,
        },
        Vector { _handler: COMP },
        Vector { _handler: LPTIM2 },
        Vector { _handler: LPTIM3 },
        Vector { _handler: LPTIM4 },
        Vector { _handler: LPTIM5 },
        Vector { _handler: LPUART1 },
        Vector { _handler: WWDG_RST },
        Vector { _handler: CRS },
        Vector { _handler: ECC },
        Vector { _handler: SAI4 },
        Vector { _reserved: 0 },
        Vector { _handler: HOLD_CORE },
        Vector { _handler: WAKEUP_PIN },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1ff1_e800usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM12: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4000_1800usize as _) };
pub const TIM13: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_1c00usize as _) };
pub const TIM14: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_2000usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_2400usize as _) };
pub const WWDG2: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const SPDIFRX1: spdifrx::Spdifrx = unsafe { spdifrx::Spdifrx::from_ptr(0x4000_4000usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00usize as _) };
pub const CEC: cec::Cec = unsafe { cec::Cec::from_ptr(0x4000_6c00usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const UART7: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_7800usize as _) };
pub const UART8: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_7c00usize as _) };
pub const CRS: crs::Crs = unsafe { crs::Crs::from_ptr(0x4000_8400usize as _) };
pub const SWPMI1: *mut () = 0x4000_8800usize as _;
pub const OPAMP1: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4000_9000usize as _) };
pub const OPAMP2: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4000_9010usize as _) };
pub const MDIOS: mdios::Mdios = unsafe { mdios::Mdios::from_ptr(0x4000_9400usize as _) };
pub const FDCAN1: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_a000usize as _) };
pub const FDCAN2: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_a400usize as _) };
pub const FDCANRAM: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_ac00usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_0000usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_0400usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_1000usize as _) };
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_1400usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3400usize as _) };
pub const TIM15: timer::Tim2chCmp = unsafe { timer::Tim2chCmp::from_ptr(0x4001_4000usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4800usize as _) };
pub const SAI1: sai::Sai = unsafe { sai::Sai::from_ptr(0x4001_5800usize as _) };
pub const SAI2: sai::Sai = unsafe { sai::Sai::from_ptr(0x4001_5c00usize as _) };
pub const SAI3: sai::Sai = unsafe { sai::Sai::from_ptr(0x4001_6000usize as _) };
pub const DFSDM1: *mut () = 0x4001_7000usize as _;
pub const HRTIM1: hrtim::Hrtim = unsafe { hrtim::Hrtim::from_ptr(0x4001_7400usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0400usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4002_0800usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4002_2000usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x4002_2100usize as _) };
pub const ADC12_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4002_2300usize as _) };
pub const ETH: eth::Eth = unsafe { eth::Eth::from_ptr(0x4002_8000usize as _) };
pub const USB_OTG_HS: otg::Otg = unsafe { otg::Otg::from_ptr(0x4004_0000usize as _) };
pub const USB_OTG_FS: otg::Otg = unsafe { otg::Otg::from_ptr(0x4008_0000usize as _) };
pub const DCMI: dcmi::Dcmi = unsafe { dcmi::Dcmi::from_ptr(0x4802_0000usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4802_1800usize as _) };
pub const SDMMC2: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x4802_2400usize as _) };
pub const DLYB_SDMMC2: dlyb::Dlyb = unsafe { dlyb::Dlyb::from_ptr(0x4802_2800usize as _) };
pub const DSIHOST: dsihost::Dsihost = unsafe { dsihost::Dsihost::from_ptr(0x5000_0000usize as _) };
pub const LTDC: ltdc::Ltdc = unsafe { ltdc::Ltdc::from_ptr(0x5000_1000usize as _) };
pub const WWDG1: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x5000_3000usize as _) };
pub const MDMA: mdma::Mdma = unsafe { mdma::Mdma::from_ptr(0x5200_0000usize as _) };
pub const DMA2D: dma2d::Dma2d = unsafe { dma2d::Dma2d::from_ptr(0x5200_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x5200_2000usize as _) };
pub const JPEG: jpeg::Jpeg = unsafe { jpeg::Jpeg::from_ptr(0x5200_3000usize as _) };
pub const FMC: fmc::Fmc = unsafe { fmc::Fmc::from_ptr(0x5200_4000usize as _) };
pub const QUADSPI: quadspi::Quadspi = unsafe { quadspi::Quadspi::from_ptr(0x5200_5000usize as _) };
pub const DLYB_QUADSPI: dlyb::Dlyb = unsafe { dlyb::Dlyb::from_ptr(0x5200_6000usize as _) };
pub const SDMMC1: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x5200_7000usize as _) };
pub const DLYB_SDMMC1: dlyb::Dlyb = unsafe { dlyb::Dlyb::from_ptr(0x5200_8000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x5800_0000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x5800_0400usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x5800_0c00usize as _) };
pub const SPI6: spi::Spi = unsafe { spi::Spi::from_ptr(0x5800_1400usize as _) };
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x5800_1c00usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2400usize as _) };
pub const LPTIM3: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2800usize as _) };
pub const LPTIM4: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2c00usize as _) };
pub const LPTIM5: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_3000usize as _) };
pub const COMP1: comp::Comp = unsafe { comp::Comp::from_ptr(0x5800_380cusize as _) };
pub const COMP2: comp::Comp = unsafe { comp::Comp::from_ptr(0x5800_3810usize as _) };
pub const VREFBUF: vrefbuf::Vrefbuf = unsafe { vrefbuf::Vrefbuf::from_ptr(0x5800_3c00usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x5800_4000usize as _) };
pub const IWDG1: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x5800_4800usize as _) };
pub const IWDG2: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x5800_4c00usize as _) };
pub const SAI4: sai::Sai = unsafe { sai::Sai::from_ptr(0x5800_5400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1c00usize as _) };
pub const GPIOI: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_2000usize as _) };
pub const GPIOJ: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_2400usize as _) };
pub const GPIOK: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_2800usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x5802_4400usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x5802_4800usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x5802_4c00usize as _) };
pub const BDMA: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x5802_5400usize as _) };
pub const DMAMUX2: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x5802_5800usize as _) };
pub const ADC3: adc::Adc = unsafe { adc::Adc::from_ptr(0x5802_6000usize as _) };
pub const ADC3_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x5802_6300usize as _) };
pub const HSEM: hsem::Hsem = unsafe { hsem::Hsem::from_ptr(0x5802_6400usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0x5c00_1000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_v4.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_v4.rs"]
pub mod adccommon;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/can_fdcan_h7.rs"]
pub mod can;
#[path = "../../peripherals/cec_v2.rs"]
pub mod cec;
#[path = "../../peripherals/comp_h7_b.rs"]
pub mod comp;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/crs_v1.rs"]
pub mod crs;
#[path = "../../peripherals/dac_v4.rs"]
pub mod dac;
#[path = "../../peripherals/dbgmcu_h7.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../../peripherals/dlyb_v1.rs"]
pub mod dlyb;
#[path = "../../peripherals/dma_v1.rs"]
pub mod dma;
#[path = "../../peripherals/dma2d_v2.rs"]
pub mod dma2d;
#[path = "../../peripherals/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../../peripherals/dsihost_v1.rs"]
pub mod dsihost;
#[path = "../../peripherals/eth_v2.rs"]
pub mod eth;
#[path = "../../peripherals/exti_h7.rs"]
pub mod exti;
#[path = "../../peripherals/fdcanram_h7.rs"]
pub mod fdcanram;
#[path = "../../peripherals/flash_h7.rs"]
pub mod flash;
#[path = "../../peripherals/fmc_v3x1.rs"]
pub mod fmc;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/hrtim_v1.rs"]
pub mod hrtim;
#[path = "../../peripherals/hsem_v1.rs"]
pub mod hsem;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/jpeg_v1.rs"]
pub mod jpeg;
#[path = "../../peripherals/lptim_v1b_h7.rs"]
pub mod lptim;
#[path = "../../peripherals/ltdc_v1.rs"]
pub mod ltdc;
#[path = "../../peripherals/mdios_v1.rs"]
pub mod mdios;
#[path = "../../peripherals/mdma_v1.rs"]
pub mod mdma;
#[path = "../../peripherals/opamp_v4.rs"]
pub mod opamp;
#[path = "../../peripherals/otg_v1.rs"]
pub mod otg;
#[path = "../../peripherals/pwr_h7rm0399.rs"]
pub mod pwr;
#[path = "../../peripherals/quadspi_v1.rs"]
pub mod quadspi;
#[path = "../../peripherals/rcc_h7.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v1.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v2_h7.rs"]
pub mod rtc;
#[path = "../../peripherals/sai_v3_4pdm.rs"]
pub mod sai;
#[path = "../../peripherals/sdmmc_v2.rs"]
pub mod sdmmc;
#[path = "../../peripherals/spdifrx_h7.rs"]
pub mod spdifrx;
#[path = "../../peripherals/spi_v4_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_h7od.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/vrefbuf_v2a1.rs"]
pub mod vrefbuf;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
