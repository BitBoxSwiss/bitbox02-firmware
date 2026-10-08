#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - PVD_PVM"]
    PVD_PVM = 0,
    #[doc = "2 - DTS"]
    DTS = 2,
    #[doc = "3 - IWDG"]
    IWDG = 3,
    #[doc = "4 - WWDG"]
    WWDG = 4,
    #[doc = "5 - RCC"]
    RCC = 5,
    #[doc = "8 - FLASH"]
    FLASH = 8,
    #[doc = "9 - RAMECC"]
    RAMECC = 9,
    #[doc = "10 - FPU"]
    FPU = 10,
    #[doc = "13 - TAMP"]
    TAMP = 13,
    #[doc = "16 - EXTI0"]
    EXTI0 = 16,
    #[doc = "17 - EXTI1"]
    EXTI1 = 17,
    #[doc = "18 - EXTI2"]
    EXTI2 = 18,
    #[doc = "19 - EXTI3"]
    EXTI3 = 19,
    #[doc = "20 - EXTI4"]
    EXTI4 = 20,
    #[doc = "21 - EXTI5"]
    EXTI5 = 21,
    #[doc = "22 - EXTI6"]
    EXTI6 = 22,
    #[doc = "23 - EXTI7"]
    EXTI7 = 23,
    #[doc = "24 - EXTI8"]
    EXTI8 = 24,
    #[doc = "25 - EXTI9"]
    EXTI9 = 25,
    #[doc = "26 - EXTI10"]
    EXTI10 = 26,
    #[doc = "27 - EXTI11"]
    EXTI11 = 27,
    #[doc = "28 - EXTI12"]
    EXTI12 = 28,
    #[doc = "29 - EXTI13"]
    EXTI13 = 29,
    #[doc = "30 - EXTI14"]
    EXTI14 = 30,
    #[doc = "31 - EXTI15"]
    EXTI15 = 31,
    #[doc = "32 - RTC"]
    RTC = 32,
    #[doc = "35 - PKA"]
    PKA = 35,
    #[doc = "36 - HASH"]
    HASH = 36,
    #[doc = "37 - RNG"]
    RNG = 37,
    #[doc = "38 - ADC1_2"]
    ADC1_2 = 38,
    #[doc = "39 - GPDMA1_CHANNEL0"]
    GPDMA1_CHANNEL0 = 39,
    #[doc = "40 - GPDMA1_CHANNEL1"]
    GPDMA1_CHANNEL1 = 40,
    #[doc = "41 - GPDMA1_CHANNEL2"]
    GPDMA1_CHANNEL2 = 41,
    #[doc = "42 - GPDMA1_CHANNEL3"]
    GPDMA1_CHANNEL3 = 42,
    #[doc = "43 - GPDMA1_CHANNEL4"]
    GPDMA1_CHANNEL4 = 43,
    #[doc = "44 - GPDMA1_CHANNEL5"]
    GPDMA1_CHANNEL5 = 44,
    #[doc = "45 - GPDMA1_CHANNEL6"]
    GPDMA1_CHANNEL6 = 45,
    #[doc = "46 - GPDMA1_CHANNEL7"]
    GPDMA1_CHANNEL7 = 46,
    #[doc = "47 - TIM1_BRK"]
    TIM1_BRK = 47,
    #[doc = "48 - TIM1_UP"]
    TIM1_UP = 48,
    #[doc = "49 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 49,
    #[doc = "50 - TIM1_CC"]
    TIM1_CC = 50,
    #[doc = "51 - TIM2"]
    TIM2 = 51,
    #[doc = "52 - TIM3"]
    TIM3 = 52,
    #[doc = "53 - TIM4"]
    TIM4 = 53,
    #[doc = "54 - TIM5"]
    TIM5 = 54,
    #[doc = "55 - TIM6"]
    TIM6 = 55,
    #[doc = "56 - TIM7"]
    TIM7 = 56,
    #[doc = "57 - TIM9"]
    TIM9 = 57,
    #[doc = "58 - SPI1"]
    SPI1 = 58,
    #[doc = "59 - SPI2"]
    SPI2 = 59,
    #[doc = "60 - SPI3"]
    SPI3 = 60,
    #[doc = "61 - SPI4"]
    SPI4 = 61,
    #[doc = "62 - SPI5"]
    SPI5 = 62,
    #[doc = "63 - SPI6"]
    SPI6 = 63,
    #[doc = "64 - HPDMA1_CHANNEL0"]
    HPDMA1_CHANNEL0 = 64,
    #[doc = "65 - HPDMA1_CHANNEL1"]
    HPDMA1_CHANNEL1 = 65,
    #[doc = "66 - HPDMA1_CHANNEL2"]
    HPDMA1_CHANNEL2 = 66,
    #[doc = "67 - HPDMA1_CHANNEL3"]
    HPDMA1_CHANNEL3 = 67,
    #[doc = "68 - HPDMA1_CHANNEL4"]
    HPDMA1_CHANNEL4 = 68,
    #[doc = "69 - HPDMA1_CHANNEL5"]
    HPDMA1_CHANNEL5 = 69,
    #[doc = "70 - HPDMA1_CHANNEL6"]
    HPDMA1_CHANNEL6 = 70,
    #[doc = "71 - HPDMA1_CHANNEL7"]
    HPDMA1_CHANNEL7 = 71,
    #[doc = "72 - SAI1_A"]
    SAI1_A = 72,
    #[doc = "73 - SAI1_B"]
    SAI1_B = 73,
    #[doc = "74 - SAI2_A"]
    SAI2_A = 74,
    #[doc = "75 - SAI2_B"]
    SAI2_B = 75,
    #[doc = "76 - I2C1_EV"]
    I2C1_EV = 76,
    #[doc = "77 - I2C1_ER"]
    I2C1_ER = 77,
    #[doc = "78 - I2C2_EV"]
    I2C2_EV = 78,
    #[doc = "79 - I2C2_ER"]
    I2C2_ER = 79,
    #[doc = "80 - I2C3_EV"]
    I2C3_EV = 80,
    #[doc = "81 - I2C3_ER"]
    I2C3_ER = 81,
    #[doc = "82 - USART1"]
    USART1 = 82,
    #[doc = "83 - USART2"]
    USART2 = 83,
    #[doc = "84 - USART3"]
    USART3 = 84,
    #[doc = "85 - UART4"]
    UART4 = 85,
    #[doc = "86 - UART5"]
    UART5 = 86,
    #[doc = "87 - UART7"]
    UART7 = 87,
    #[doc = "88 - UART8"]
    UART8 = 88,
    #[doc = "89 - I3C1_EV"]
    I3C1_EV = 89,
    #[doc = "90 - I3C1_ER"]
    I3C1_ER = 90,
    #[doc = "91 - OTG_HS"]
    OTG_HS = 91,
    #[doc = "92 - ETH"]
    ETH = 92,
    #[doc = "93 - CORDIC"]
    CORDIC = 93,
    #[doc = "94 - GFXTIM"]
    GFXTIM = 94,
    #[doc = "95 - DCMIPP"]
    DCMIPP = 95,
    #[doc = "98 - DMA2D"]
    DMA2D = 98,
    #[doc = "99 - JPEG"]
    JPEG = 99,
    #[doc = "100 - GFXMMU"]
    GFXMMU = 100,
    #[doc = "101 - I3C1_WKUP"]
    I3C1_WKUP = 101,
    #[doc = "105 - XSPI1"]
    XSPI1 = 105,
    #[doc = "106 - XSPI2"]
    XSPI2 = 106,
    #[doc = "107 - FMC"]
    FMC = 107,
    #[doc = "108 - SDMMC1"]
    SDMMC1 = 108,
    #[doc = "109 - SDMMC2"]
    SDMMC2 = 109,
    #[doc = "112 - OTG_FS"]
    OTG_FS = 112,
    #[doc = "113 - TIM12"]
    TIM12 = 113,
    #[doc = "114 - TIM13"]
    TIM13 = 114,
    #[doc = "115 - TIM14"]
    TIM14 = 115,
    #[doc = "116 - TIM15"]
    TIM15 = 116,
    #[doc = "117 - TIM16"]
    TIM16 = 117,
    #[doc = "118 - TIM17"]
    TIM17 = 118,
    #[doc = "119 - LPTIM1"]
    LPTIM1 = 119,
    #[doc = "120 - LPTIM2"]
    LPTIM2 = 120,
    #[doc = "121 - LPTIM3"]
    LPTIM3 = 121,
    #[doc = "122 - LPTIM4"]
    LPTIM4 = 122,
    #[doc = "123 - LPTIM5"]
    LPTIM5 = 123,
    #[doc = "124 - SPDIF_RX"]
    SPDIF_RX = 124,
    #[doc = "125 - MDIOS"]
    MDIOS = 125,
    #[doc = "126 - ADF1_FLT0"]
    ADF1_FLT0 = 126,
    #[doc = "127 - CRS"]
    CRS = 127,
    #[doc = "128 - UCPD1"]
    UCPD1 = 128,
    #[doc = "129 - CEC"]
    CEC = 129,
    #[doc = "130 - PSSI"]
    PSSI = 130,
    #[doc = "131 - LPUART1"]
    LPUART1 = 131,
    #[doc = "132 - WAKEUP_PIN"]
    WAKEUP_PIN = 132,
    #[doc = "133 - GPDMA1_CHANNEL8"]
    GPDMA1_CHANNEL8 = 133,
    #[doc = "134 - GPDMA1_CHANNEL9"]
    GPDMA1_CHANNEL9 = 134,
    #[doc = "135 - GPDMA1_CHANNEL10"]
    GPDMA1_CHANNEL10 = 135,
    #[doc = "136 - GPDMA1_CHANNEL11"]
    GPDMA1_CHANNEL11 = 136,
    #[doc = "137 - GPDMA1_CHANNEL12"]
    GPDMA1_CHANNEL12 = 137,
    #[doc = "138 - GPDMA1_CHANNEL13"]
    GPDMA1_CHANNEL13 = 138,
    #[doc = "139 - GPDMA1_CHANNEL14"]
    GPDMA1_CHANNEL14 = 139,
    #[doc = "140 - GPDMA1_CHANNEL15"]
    GPDMA1_CHANNEL15 = 140,
    #[doc = "141 - HPDMA1_CHANNEL8"]
    HPDMA1_CHANNEL8 = 141,
    #[doc = "142 - HPDMA1_CHANNEL9"]
    HPDMA1_CHANNEL9 = 142,
    #[doc = "143 - HPDMA1_CHANNEL10"]
    HPDMA1_CHANNEL10 = 143,
    #[doc = "144 - HPDMA1_CHANNEL11"]
    HPDMA1_CHANNEL11 = 144,
    #[doc = "145 - HPDMA1_CHANNEL12"]
    HPDMA1_CHANNEL12 = 145,
    #[doc = "146 - HPDMA1_CHANNEL13"]
    HPDMA1_CHANNEL13 = 146,
    #[doc = "147 - HPDMA1_CHANNEL14"]
    HPDMA1_CHANNEL14 = 147,
    #[doc = "148 - HPDMA1_CHANNEL15"]
    HPDMA1_CHANNEL15 = 148,
    #[doc = "152 - FDCAN1_IT0"]
    FDCAN1_IT0 = 152,
    #[doc = "153 - FDCAN1_IT1"]
    FDCAN1_IT1 = 153,
    #[doc = "154 - FDCAN2_IT0"]
    FDCAN2_IT0 = 154,
    #[doc = "155 - FDCAN2_IT1"]
    FDCAN2_IT1 = 155,
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
        fn PVD_PVM();
        fn DTS();
        fn IWDG();
        fn WWDG();
        fn RCC();
        fn FLASH();
        fn RAMECC();
        fn FPU();
        fn TAMP();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn EXTI5();
        fn EXTI6();
        fn EXTI7();
        fn EXTI8();
        fn EXTI9();
        fn EXTI10();
        fn EXTI11();
        fn EXTI12();
        fn EXTI13();
        fn EXTI14();
        fn EXTI15();
        fn RTC();
        fn PKA();
        fn HASH();
        fn RNG();
        fn ADC1_2();
        fn GPDMA1_CHANNEL0();
        fn GPDMA1_CHANNEL1();
        fn GPDMA1_CHANNEL2();
        fn GPDMA1_CHANNEL3();
        fn GPDMA1_CHANNEL4();
        fn GPDMA1_CHANNEL5();
        fn GPDMA1_CHANNEL6();
        fn GPDMA1_CHANNEL7();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn TIM5();
        fn TIM6();
        fn TIM7();
        fn TIM9();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn HPDMA1_CHANNEL0();
        fn HPDMA1_CHANNEL1();
        fn HPDMA1_CHANNEL2();
        fn HPDMA1_CHANNEL3();
        fn HPDMA1_CHANNEL4();
        fn HPDMA1_CHANNEL5();
        fn HPDMA1_CHANNEL6();
        fn HPDMA1_CHANNEL7();
        fn SAI1_A();
        fn SAI1_B();
        fn SAI2_A();
        fn SAI2_B();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn I2C3_EV();
        fn I2C3_ER();
        fn USART1();
        fn USART2();
        fn USART3();
        fn UART4();
        fn UART5();
        fn UART7();
        fn UART8();
        fn I3C1_EV();
        fn I3C1_ER();
        fn OTG_HS();
        fn ETH();
        fn CORDIC();
        fn GFXTIM();
        fn DCMIPP();
        fn DMA2D();
        fn JPEG();
        fn GFXMMU();
        fn I3C1_WKUP();
        fn XSPI1();
        fn XSPI2();
        fn FMC();
        fn SDMMC1();
        fn SDMMC2();
        fn OTG_FS();
        fn TIM12();
        fn TIM13();
        fn TIM14();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn LPTIM1();
        fn LPTIM2();
        fn LPTIM3();
        fn LPTIM4();
        fn LPTIM5();
        fn SPDIF_RX();
        fn MDIOS();
        fn ADF1_FLT0();
        fn CRS();
        fn UCPD1();
        fn CEC();
        fn PSSI();
        fn LPUART1();
        fn WAKEUP_PIN();
        fn GPDMA1_CHANNEL8();
        fn GPDMA1_CHANNEL9();
        fn GPDMA1_CHANNEL10();
        fn GPDMA1_CHANNEL11();
        fn GPDMA1_CHANNEL12();
        fn GPDMA1_CHANNEL13();
        fn GPDMA1_CHANNEL14();
        fn GPDMA1_CHANNEL15();
        fn HPDMA1_CHANNEL8();
        fn HPDMA1_CHANNEL9();
        fn HPDMA1_CHANNEL10();
        fn HPDMA1_CHANNEL11();
        fn HPDMA1_CHANNEL12();
        fn HPDMA1_CHANNEL13();
        fn HPDMA1_CHANNEL14();
        fn HPDMA1_CHANNEL15();
        fn FDCAN1_IT0();
        fn FDCAN1_IT1();
        fn FDCAN2_IT0();
        fn FDCAN2_IT1();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 156] = [
        Vector { _handler: PVD_PVM },
        Vector { _reserved: 0 },
        Vector { _handler: DTS },
        Vector { _handler: IWDG },
        Vector { _handler: WWDG },
        Vector { _handler: RCC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FLASH },
        Vector { _handler: RAMECC },
        Vector { _handler: FPU },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TAMP },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector { _handler: EXTI5 },
        Vector { _handler: EXTI6 },
        Vector { _handler: EXTI7 },
        Vector { _handler: EXTI8 },
        Vector { _handler: EXTI9 },
        Vector { _handler: EXTI10 },
        Vector { _handler: EXTI11 },
        Vector { _handler: EXTI12 },
        Vector { _handler: EXTI13 },
        Vector { _handler: EXTI14 },
        Vector { _handler: EXTI15 },
        Vector { _handler: RTC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: PKA },
        Vector { _handler: HASH },
        Vector { _handler: RNG },
        Vector { _handler: ADC1_2 },
        Vector {
            _handler: GPDMA1_CHANNEL0,
        },
        Vector {
            _handler: GPDMA1_CHANNEL1,
        },
        Vector {
            _handler: GPDMA1_CHANNEL2,
        },
        Vector {
            _handler: GPDMA1_CHANNEL3,
        },
        Vector {
            _handler: GPDMA1_CHANNEL4,
        },
        Vector {
            _handler: GPDMA1_CHANNEL5,
        },
        Vector {
            _handler: GPDMA1_CHANNEL6,
        },
        Vector {
            _handler: GPDMA1_CHANNEL7,
        },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: TIM5 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector { _handler: TIM9 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector {
            _handler: HPDMA1_CHANNEL0,
        },
        Vector {
            _handler: HPDMA1_CHANNEL1,
        },
        Vector {
            _handler: HPDMA1_CHANNEL2,
        },
        Vector {
            _handler: HPDMA1_CHANNEL3,
        },
        Vector {
            _handler: HPDMA1_CHANNEL4,
        },
        Vector {
            _handler: HPDMA1_CHANNEL5,
        },
        Vector {
            _handler: HPDMA1_CHANNEL6,
        },
        Vector {
            _handler: HPDMA1_CHANNEL7,
        },
        Vector { _handler: SAI1_A },
        Vector { _handler: SAI1_B },
        Vector { _handler: SAI2_A },
        Vector { _handler: SAI2_B },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: I3C1_EV },
        Vector { _handler: I3C1_ER },
        Vector { _handler: OTG_HS },
        Vector { _handler: ETH },
        Vector { _handler: CORDIC },
        Vector { _handler: GFXTIM },
        Vector { _handler: DCMIPP },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMA2D },
        Vector { _handler: JPEG },
        Vector { _handler: GFXMMU },
        Vector { _handler: I3C1_WKUP },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: XSPI1 },
        Vector { _handler: XSPI2 },
        Vector { _handler: FMC },
        Vector { _handler: SDMMC1 },
        Vector { _handler: SDMMC2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: OTG_FS },
        Vector { _handler: TIM12 },
        Vector { _handler: TIM13 },
        Vector { _handler: TIM14 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: LPTIM3 },
        Vector { _handler: LPTIM4 },
        Vector { _handler: LPTIM5 },
        Vector { _handler: SPDIF_RX },
        Vector { _handler: MDIOS },
        Vector { _handler: ADF1_FLT0 },
        Vector { _handler: CRS },
        Vector { _handler: UCPD1 },
        Vector { _handler: CEC },
        Vector { _handler: PSSI },
        Vector { _handler: LPUART1 },
        Vector { _handler: WAKEUP_PIN },
        Vector {
            _handler: GPDMA1_CHANNEL8,
        },
        Vector {
            _handler: GPDMA1_CHANNEL9,
        },
        Vector {
            _handler: GPDMA1_CHANNEL10,
        },
        Vector {
            _handler: GPDMA1_CHANNEL11,
        },
        Vector {
            _handler: GPDMA1_CHANNEL12,
        },
        Vector {
            _handler: GPDMA1_CHANNEL13,
        },
        Vector {
            _handler: GPDMA1_CHANNEL14,
        },
        Vector {
            _handler: GPDMA1_CHANNEL15,
        },
        Vector {
            _handler: HPDMA1_CHANNEL8,
        },
        Vector {
            _handler: HPDMA1_CHANNEL9,
        },
        Vector {
            _handler: HPDMA1_CHANNEL10,
        },
        Vector {
            _handler: HPDMA1_CHANNEL11,
        },
        Vector {
            _handler: HPDMA1_CHANNEL12,
        },
        Vector {
            _handler: HPDMA1_CHANNEL13,
        },
        Vector {
            _handler: HPDMA1_CHANNEL14,
        },
        Vector {
            _handler: HPDMA1_CHANNEL15,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FDCAN1_IT0 },
        Vector { _handler: FDCAN1_IT1 },
        Vector { _handler: FDCAN2_IT0 },
        Vector { _handler: FDCAN2_IT1 },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x08ff_f800usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM12: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4000_1800usize as _) };
pub const TIM13: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_1c00usize as _) };
pub const TIM14: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_2000usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_2400usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const SPDIFRX1: spdifrx::Spdifrx = unsafe { spdifrx::Spdifrx::from_ptr(0x4000_4000usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I3C1: *mut () = 0x4000_5400usize as _;
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00usize as _) };
pub const CEC: cec::Cec = unsafe { cec::Cec::from_ptr(0x4000_6c00usize as _) };
pub const UART7: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_7800usize as _) };
pub const CRS: crs::Crs = unsafe { crs::Crs::from_ptr(0x4000_8400usize as _) };
pub const MDIOS: mdios::Mdios = unsafe { mdios::Mdios::from_ptr(0x4000_9400usize as _) };
pub const FDCAN1: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_a000usize as _) };
pub const FDCAN2: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_a400usize as _) };
pub const FDCANRAM1: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_ac00usize as _) };
pub const FDCANRAM2: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_af50usize as _) };
pub const UCPD1: ucpd::Ucpd = unsafe { ucpd::Ucpd::from_ptr(0x4000_ec00usize as _) };
pub const GPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_1000usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4002_2000usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x4002_2100usize as _) };
pub const ADC12_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4002_2300usize as _) };
pub const USB_OTG_HS: otg::Otg = unsafe { otg::Otg::from_ptr(0x4004_0000usize as _) };
pub const USB_OTG_FS: otg::Otg = unsafe { otg::Otg::from_ptr(0x4008_0000usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4200_0000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4200_1000usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4200_3000usize as _) };
pub const TIM15: timer::Tim2chCmp = unsafe { timer::Tim2chCmp::from_ptr(0x4200_4000usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4200_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4200_4800usize as _) };
pub const TIM9: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4200_4c00usize as _) };
pub const SPI5: spi::Spi = unsafe { spi::Spi::from_ptr(0x4200_5000usize as _) };
pub const SAI2: sai::Sai = unsafe { sai::Sai::from_ptr(0x4200_5c00usize as _) };
pub const SDMMC2: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x4800_2400usize as _) };
pub const DLYB_SDMMC2: dlyb::Dlyb = unsafe { dlyb::Dlyb::from_ptr(0x4800_2800usize as _) };
pub const CORDIC: cordic::Cordic = unsafe { cordic::Cordic::from_ptr(0x4800_4400usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4802_0000usize as _) };
pub const HASH: hash::Hash = unsafe { hash::Hash::from_ptr(0x4802_0400usize as _) };
pub const GFXTIM: *mut () = 0x5000_4000usize as _;
pub const HPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x5200_0000usize as _) };
pub const DMA2D: dma2d::Dma2d = unsafe { dma2d::Dma2d::from_ptr(0x5200_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x5200_2000usize as _) };
pub const JPEG: jpeg::Jpeg = unsafe { jpeg::Jpeg::from_ptr(0x5200_3000usize as _) };
pub const FMC: fmc::Fmc = unsafe { fmc::Fmc::from_ptr(0x5200_4000usize as _) };
pub const XSPI1: xspi::Xspi = unsafe { xspi::Xspi::from_ptr(0x5200_5000usize as _) };
pub const DLYB_SDMMC1: dlyb::Dlyb = unsafe { dlyb::Dlyb::from_ptr(0x5200_8000usize as _) };
pub const XSPIM: xspim::Xspim = unsafe { xspim::Xspim::from_ptr(0x5200_b400usize as _) };
pub const GFXMMU: *mut () = 0x5201_0000usize as _;
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x5800_0000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x5800_0400usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x5800_0c00usize as _) };
pub const SPI6: spi::Spi = unsafe { spi::Spi::from_ptr(0x5800_1400usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2400usize as _) };
pub const LPTIM3: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2800usize as _) };
pub const LPTIM4: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2c00usize as _) };
pub const VREFBUF: vrefbuf::Vrefbuf = unsafe { vrefbuf::Vrefbuf::from_ptr(0x5800_3c00usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x5800_4000usize as _) };
pub const TAMP: *mut () = 0x5800_4400usize as _;
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x5800_4800usize as _) };
pub const DTS: dts::Dts = unsafe { dts::Dts::from_ptr(0x5800_6800usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1c00usize as _) };
pub const GPIOM: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_3000usize as _) };
pub const GPION: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_3400usize as _) };
pub const GPIOO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_3800usize as _) };
pub const GPIOP: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_3c00usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x5802_4400usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x5802_4800usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x5802_4c00usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0x5c00_1000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_h7rs.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_h5.rs"]
pub mod adccommon;
#[path = "../../peripherals/can_fdcan_v1.rs"]
pub mod can;
#[path = "../../peripherals/cec_v2.rs"]
pub mod cec;
#[path = "../../peripherals/cordic_v1.rs"]
pub mod cordic;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/crs_v1.rs"]
pub mod crs;
#[path = "../../peripherals/dbgmcu_h7.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dlyb_v1.rs"]
pub mod dlyb;
#[path = "../../peripherals/dma2d_v2.rs"]
pub mod dma2d;
#[path = "../../peripherals/dts_v1.rs"]
pub mod dts;
#[path = "../../peripherals/exti_h7.rs"]
pub mod exti;
#[path = "../../peripherals/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../../peripherals/flash_h7rs.rs"]
pub mod flash;
#[path = "../../peripherals/fmc_v3x1.rs"]
pub mod fmc;
#[path = "../../peripherals/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/hash_v3.rs"]
pub mod hash;
#[path = "../../peripherals/i2c_v3.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../../peripherals/jpeg_v1.rs"]
pub mod jpeg;
#[path = "../../peripherals/lptim_v1b_h7.rs"]
pub mod lptim;
#[path = "../../peripherals/mdios_v1.rs"]
pub mod mdios;
#[path = "../../peripherals/otg_v1.rs"]
pub mod otg;
#[path = "../../peripherals/pwr_h7rs.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_h7rs.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v3.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v3_h7rs.rs"]
pub mod rtc;
#[path = "../../peripherals/sai_v4_4pdm.rs"]
pub mod sai;
#[path = "../../peripherals/sdmmc_v2.rs"]
pub mod sdmmc;
#[path = "../../peripherals/spdifrx_h7.rs"]
pub mod spdifrx;
#[path = "../../peripherals/spi_v5_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_h7rs.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/ucpd_v1.rs"]
pub mod ucpd;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/vrefbuf_v2a1.rs"]
pub mod vrefbuf;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
#[path = "../../peripherals/xspi_v1.rs"]
pub mod xspi;
#[path = "../../peripherals/xspim_v1.rs"]
pub mod xspim;
