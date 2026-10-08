#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - PVD_PVM"]
    PVD_PVM = 0,
    #[doc = "2 - DTS"]
    DTS = 2,
    #[doc = "3 - RCC"]
    RCC = 3,
    #[doc = "4 - LOCKUP"]
    LOCKUP = 4,
    #[doc = "5 - CACHE_ECC"]
    CACHE_ECC = 5,
    #[doc = "6 - TCM_ECC"]
    TCM_ECC = 6,
    #[doc = "7 - BKP_ECC"]
    BKP_ECC = 7,
    #[doc = "8 - FPU"]
    FPU = 8,
    #[doc = "10 - RTC_S"]
    RTC_S = 10,
    #[doc = "11 - TAMP"]
    TAMP = 11,
    #[doc = "12 - RIFSC_TAMPER"]
    RIFSC_TAMPER = 12,
    #[doc = "13 - IAC"]
    IAC = 13,
    #[doc = "14 - RCC_S"]
    RCC_S = 14,
    #[doc = "16 - RTC"]
    RTC = 16,
    #[doc = "18 - IWDG"]
    IWDG = 18,
    #[doc = "19 - WWDG"]
    WWDG = 19,
    #[doc = "20 - EXTI0"]
    EXTI0 = 20,
    #[doc = "21 - EXTI1"]
    EXTI1 = 21,
    #[doc = "22 - EXTI2"]
    EXTI2 = 22,
    #[doc = "23 - EXTI3"]
    EXTI3 = 23,
    #[doc = "24 - EXTI4"]
    EXTI4 = 24,
    #[doc = "25 - EXTI5"]
    EXTI5 = 25,
    #[doc = "26 - EXTI6"]
    EXTI6 = 26,
    #[doc = "27 - EXTI7"]
    EXTI7 = 27,
    #[doc = "28 - EXTI8"]
    EXTI8 = 28,
    #[doc = "29 - EXTI9"]
    EXTI9 = 29,
    #[doc = "30 - EXTI10"]
    EXTI10 = 30,
    #[doc = "31 - EXTI11"]
    EXTI11 = 31,
    #[doc = "32 - EXTI12"]
    EXTI12 = 32,
    #[doc = "33 - EXTI13"]
    EXTI13 = 33,
    #[doc = "34 - EXTI14"]
    EXTI14 = 34,
    #[doc = "35 - EXTI15"]
    EXTI15 = 35,
    #[doc = "38 - PKA"]
    PKA = 38,
    #[doc = "39 - HASH"]
    HASH = 39,
    #[doc = "40 - RNG"]
    RNG = 40,
    #[doc = "46 - ADC1_2"]
    ADC1_2 = 46,
    #[doc = "47 - CSI"]
    CSI = 47,
    #[doc = "48 - DCMIPP"]
    DCMIPP = 48,
    #[doc = "52 - PAHB_ERR"]
    PAHB_ERR = 52,
    #[doc = "53 - NPU0"]
    NPU0 = 53,
    #[doc = "54 - NPU1"]
    NPU1 = 54,
    #[doc = "55 - NPU2"]
    NPU2 = 55,
    #[doc = "56 - NPU3"]
    NPU3 = 56,
    #[doc = "57 - CACHEAXI"]
    CACHEAXI = 57,
    #[doc = "58 - LTDC_LO"]
    LTDC_LO = 58,
    #[doc = "59 - LTDC_LO_ERR"]
    LTDC_LO_ERR = 59,
    #[doc = "60 - DMA2D"]
    DMA2D = 60,
    #[doc = "61 - JPEG"]
    JPEG = 61,
    #[doc = "62 - VENC"]
    VENC = 62,
    #[doc = "63 - GFXMMU"]
    GFXMMU = 63,
    #[doc = "64 - GFXTIM"]
    GFXTIM = 64,
    #[doc = "65 - GPU2D"]
    GPU2D = 65,
    #[doc = "66 - GPU2D_ER"]
    GPU2D_ER = 66,
    #[doc = "67 - ICACHE"]
    ICACHE = 67,
    #[doc = "68 - HPDMA1_CHANNEL0"]
    HPDMA1_CHANNEL0 = 68,
    #[doc = "69 - HPDMA1_CHANNEL1"]
    HPDMA1_CHANNEL1 = 69,
    #[doc = "70 - HPDMA1_CHANNEL2"]
    HPDMA1_CHANNEL2 = 70,
    #[doc = "71 - HPDMA1_CHANNEL3"]
    HPDMA1_CHANNEL3 = 71,
    #[doc = "72 - HPDMA1_CHANNEL4"]
    HPDMA1_CHANNEL4 = 72,
    #[doc = "73 - HPDMA1_CHANNEL5"]
    HPDMA1_CHANNEL5 = 73,
    #[doc = "74 - HPDMA1_CHANNEL6"]
    HPDMA1_CHANNEL6 = 74,
    #[doc = "75 - HPDMA1_CHANNEL7"]
    HPDMA1_CHANNEL7 = 75,
    #[doc = "76 - HPDMA1_CHANNEL8"]
    HPDMA1_CHANNEL8 = 76,
    #[doc = "77 - HPDMA1_CHANNEL9"]
    HPDMA1_CHANNEL9 = 77,
    #[doc = "78 - HPDMA1_CHANNEL10"]
    HPDMA1_CHANNEL10 = 78,
    #[doc = "79 - HPDMA1_CHANNEL11"]
    HPDMA1_CHANNEL11 = 79,
    #[doc = "80 - HPDMA1_CHANNEL12"]
    HPDMA1_CHANNEL12 = 80,
    #[doc = "81 - HPDMA1_CHANNEL13"]
    HPDMA1_CHANNEL13 = 81,
    #[doc = "82 - HPDMA1_CHANNEL14"]
    HPDMA1_CHANNEL14 = 82,
    #[doc = "83 - HPDMA1_CHANNEL15"]
    HPDMA1_CHANNEL15 = 83,
    #[doc = "84 - GPDMA1_CHANNEL0"]
    GPDMA1_CHANNEL0 = 84,
    #[doc = "85 - GPDMA1_CHANNEL1"]
    GPDMA1_CHANNEL1 = 85,
    #[doc = "86 - GPDMA1_CHANNEL2"]
    GPDMA1_CHANNEL2 = 86,
    #[doc = "87 - GPDMA1_CHANNEL3"]
    GPDMA1_CHANNEL3 = 87,
    #[doc = "88 - GPDMA1_CHANNEL4"]
    GPDMA1_CHANNEL4 = 88,
    #[doc = "89 - GPDMA1_CHANNEL5"]
    GPDMA1_CHANNEL5 = 89,
    #[doc = "90 - GPDMA1_CHANNEL6"]
    GPDMA1_CHANNEL6 = 90,
    #[doc = "91 - GPDMA1_CHANNEL7"]
    GPDMA1_CHANNEL7 = 91,
    #[doc = "92 - GPDMA1_CHANNEL8"]
    GPDMA1_CHANNEL8 = 92,
    #[doc = "93 - GPDMA1_CHANNEL9"]
    GPDMA1_CHANNEL9 = 93,
    #[doc = "94 - GPDMA1_CHANNEL10"]
    GPDMA1_CHANNEL10 = 94,
    #[doc = "95 - GPDMA1_CHANNEL11"]
    GPDMA1_CHANNEL11 = 95,
    #[doc = "96 - GPDMA1_CHANNEL12"]
    GPDMA1_CHANNEL12 = 96,
    #[doc = "97 - GPDMA1_CHANNEL13"]
    GPDMA1_CHANNEL13 = 97,
    #[doc = "98 - GPDMA1_CHANNEL14"]
    GPDMA1_CHANNEL14 = 98,
    #[doc = "99 - GPDMA1_CHANNEL15"]
    GPDMA1_CHANNEL15 = 99,
    #[doc = "100 - I2C1_EV"]
    I2C1_EV = 100,
    #[doc = "101 - I2C1_ER"]
    I2C1_ER = 101,
    #[doc = "102 - I2C2_EV"]
    I2C2_EV = 102,
    #[doc = "103 - I2C2_ER"]
    I2C2_ER = 103,
    #[doc = "104 - I2C3_EV"]
    I2C3_EV = 104,
    #[doc = "105 - I2C3_ER"]
    I2C3_ER = 105,
    #[doc = "106 - I2C4_EV"]
    I2C4_EV = 106,
    #[doc = "107 - I2C4_ER"]
    I2C4_ER = 107,
    #[doc = "108 - I3C1_EV"]
    I3C1_EV = 108,
    #[doc = "109 - I3C1_ER"]
    I3C1_ER = 109,
    #[doc = "110 - I3C2_EV"]
    I3C2_EV = 110,
    #[doc = "111 - I3C2_ER"]
    I3C2_ER = 111,
    #[doc = "112 - TIM1_BRK"]
    TIM1_BRK = 112,
    #[doc = "113 - TIM1_UP"]
    TIM1_UP = 113,
    #[doc = "114 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 114,
    #[doc = "115 - TIM1_CC"]
    TIM1_CC = 115,
    #[doc = "116 - TIM2"]
    TIM2 = 116,
    #[doc = "117 - TIM3"]
    TIM3 = 117,
    #[doc = "118 - TIM4"]
    TIM4 = 118,
    #[doc = "119 - TIM5"]
    TIM5 = 119,
    #[doc = "120 - TIM6"]
    TIM6 = 120,
    #[doc = "121 - TIM7"]
    TIM7 = 121,
    #[doc = "122 - TIM8_BRK"]
    TIM8_BRK = 122,
    #[doc = "123 - TIM8_UP"]
    TIM8_UP = 123,
    #[doc = "124 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 124,
    #[doc = "125 - TIM8_CC"]
    TIM8_CC = 125,
    #[doc = "126 - TIM9"]
    TIM9 = 126,
    #[doc = "127 - TIM10"]
    TIM10 = 127,
    #[doc = "128 - TIM11"]
    TIM11 = 128,
    #[doc = "129 - TIM12"]
    TIM12 = 129,
    #[doc = "130 - TIM13"]
    TIM13 = 130,
    #[doc = "131 - TIM14"]
    TIM14 = 131,
    #[doc = "132 - TIM15"]
    TIM15 = 132,
    #[doc = "133 - TIM16"]
    TIM16 = 133,
    #[doc = "134 - TIM17"]
    TIM17 = 134,
    #[doc = "135 - TIM18"]
    TIM18 = 135,
    #[doc = "136 - LPTIM1"]
    LPTIM1 = 136,
    #[doc = "137 - LPTIM2"]
    LPTIM2 = 137,
    #[doc = "138 - LPTIM3"]
    LPTIM3 = 138,
    #[doc = "139 - LPTIM4"]
    LPTIM4 = 139,
    #[doc = "140 - LPTIM5"]
    LPTIM5 = 140,
    #[doc = "141 - ADF1_FLT0"]
    ADF1_FLT0 = 141,
    #[doc = "142 - MDF1_FLT0"]
    MDF1_FLT0 = 142,
    #[doc = "143 - MDF1_FLT1"]
    MDF1_FLT1 = 143,
    #[doc = "144 - MDF1_FLT2"]
    MDF1_FLT2 = 144,
    #[doc = "145 - MDF1_FLT3"]
    MDF1_FLT3 = 145,
    #[doc = "146 - MDF1_FLT4"]
    MDF1_FLT4 = 146,
    #[doc = "147 - MDF1_FLT5"]
    MDF1_FLT5 = 147,
    #[doc = "148 - SAI1_A"]
    SAI1_A = 148,
    #[doc = "149 - SAI1_B"]
    SAI1_B = 149,
    #[doc = "150 - SAI2_A"]
    SAI2_A = 150,
    #[doc = "151 - SAI2_B"]
    SAI2_B = 151,
    #[doc = "152 - SPDIFRX1"]
    SPDIFRX1 = 152,
    #[doc = "153 - SPI1"]
    SPI1 = 153,
    #[doc = "154 - SPI2"]
    SPI2 = 154,
    #[doc = "155 - SPI3"]
    SPI3 = 155,
    #[doc = "156 - SPI4"]
    SPI4 = 156,
    #[doc = "157 - SPI5"]
    SPI5 = 157,
    #[doc = "158 - SPI6"]
    SPI6 = 158,
    #[doc = "159 - USART1"]
    USART1 = 159,
    #[doc = "160 - USART2"]
    USART2 = 160,
    #[doc = "161 - USART3"]
    USART3 = 161,
    #[doc = "162 - UART4"]
    UART4 = 162,
    #[doc = "163 - UART5"]
    UART5 = 163,
    #[doc = "164 - USART6"]
    USART6 = 164,
    #[doc = "165 - UART7"]
    UART7 = 165,
    #[doc = "166 - UART8"]
    UART8 = 166,
    #[doc = "167 - UART9"]
    UART9 = 167,
    #[doc = "168 - USART10"]
    USART10 = 168,
    #[doc = "169 - LPUART1"]
    LPUART1 = 169,
    #[doc = "170 - XSPI1"]
    XSPI1 = 170,
    #[doc = "171 - XSPI2"]
    XSPI2 = 171,
    #[doc = "172 - XSPI3"]
    XSPI3 = 172,
    #[doc = "173 - FMC"]
    FMC = 173,
    #[doc = "174 - SDMMC1"]
    SDMMC1 = 174,
    #[doc = "175 - SDMMC2"]
    SDMMC2 = 175,
    #[doc = "176 - UCPD1"]
    UCPD1 = 176,
    #[doc = "177 - USB1_OTG_HS"]
    USB1_OTG_HS = 177,
    #[doc = "178 - USB2_OTG_HS"]
    USB2_OTG_HS = 178,
    #[doc = "179 - ETH1"]
    ETH1 = 179,
    #[doc = "180 - FDCAN1_IT0"]
    FDCAN1_IT0 = 180,
    #[doc = "181 - FDCAN1_IT1"]
    FDCAN1_IT1 = 181,
    #[doc = "182 - FDCAN2_IT0"]
    FDCAN2_IT0 = 182,
    #[doc = "183 - FDCAN2_IT1"]
    FDCAN2_IT1 = 183,
    #[doc = "184 - FDCAN3_IT0"]
    FDCAN3_IT0 = 184,
    #[doc = "185 - FDCAN3_IT1"]
    FDCAN3_IT1 = 185,
    #[doc = "186 - FDCAN_CU"]
    FDCAN_CU = 186,
    #[doc = "187 - MDIOS"]
    MDIOS = 187,
    #[doc = "188 - DCMI_PSSI"]
    DCMI_PSSI = 188,
    #[doc = "189 - WAKEUP_PIN"]
    WAKEUP_PIN = 189,
    #[doc = "190 - CTI_INT0"]
    CTI_INT0 = 190,
    #[doc = "191 - CTI_INT1"]
    CTI_INT1 = 191,
    #[doc = "193 - LTDC_UP"]
    LTDC_UP = 193,
    #[doc = "194 - LTDC_UP_ERR"]
    LTDC_UP_ERR = 194,
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
        fn RCC();
        fn LOCKUP();
        fn CACHE_ECC();
        fn TCM_ECC();
        fn BKP_ECC();
        fn FPU();
        fn RTC_S();
        fn TAMP();
        fn RIFSC_TAMPER();
        fn IAC();
        fn RCC_S();
        fn RTC();
        fn IWDG();
        fn WWDG();
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
        fn PKA();
        fn HASH();
        fn RNG();
        fn ADC1_2();
        fn CSI();
        fn DCMIPP();
        fn PAHB_ERR();
        fn NPU0();
        fn NPU1();
        fn NPU2();
        fn NPU3();
        fn CACHEAXI();
        fn LTDC_LO();
        fn LTDC_LO_ERR();
        fn DMA2D();
        fn JPEG();
        fn VENC();
        fn GFXMMU();
        fn GFXTIM();
        fn GPU2D();
        fn GPU2D_ER();
        fn ICACHE();
        fn HPDMA1_CHANNEL0();
        fn HPDMA1_CHANNEL1();
        fn HPDMA1_CHANNEL2();
        fn HPDMA1_CHANNEL3();
        fn HPDMA1_CHANNEL4();
        fn HPDMA1_CHANNEL5();
        fn HPDMA1_CHANNEL6();
        fn HPDMA1_CHANNEL7();
        fn HPDMA1_CHANNEL8();
        fn HPDMA1_CHANNEL9();
        fn HPDMA1_CHANNEL10();
        fn HPDMA1_CHANNEL11();
        fn HPDMA1_CHANNEL12();
        fn HPDMA1_CHANNEL13();
        fn HPDMA1_CHANNEL14();
        fn HPDMA1_CHANNEL15();
        fn GPDMA1_CHANNEL0();
        fn GPDMA1_CHANNEL1();
        fn GPDMA1_CHANNEL2();
        fn GPDMA1_CHANNEL3();
        fn GPDMA1_CHANNEL4();
        fn GPDMA1_CHANNEL5();
        fn GPDMA1_CHANNEL6();
        fn GPDMA1_CHANNEL7();
        fn GPDMA1_CHANNEL8();
        fn GPDMA1_CHANNEL9();
        fn GPDMA1_CHANNEL10();
        fn GPDMA1_CHANNEL11();
        fn GPDMA1_CHANNEL12();
        fn GPDMA1_CHANNEL13();
        fn GPDMA1_CHANNEL14();
        fn GPDMA1_CHANNEL15();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn I2C3_EV();
        fn I2C3_ER();
        fn I2C4_EV();
        fn I2C4_ER();
        fn I3C1_EV();
        fn I3C1_ER();
        fn I3C2_EV();
        fn I3C2_ER();
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
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn TIM9();
        fn TIM10();
        fn TIM11();
        fn TIM12();
        fn TIM13();
        fn TIM14();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn TIM18();
        fn LPTIM1();
        fn LPTIM2();
        fn LPTIM3();
        fn LPTIM4();
        fn LPTIM5();
        fn ADF1_FLT0();
        fn MDF1_FLT0();
        fn MDF1_FLT1();
        fn MDF1_FLT2();
        fn MDF1_FLT3();
        fn MDF1_FLT4();
        fn MDF1_FLT5();
        fn SAI1_A();
        fn SAI1_B();
        fn SAI2_A();
        fn SAI2_B();
        fn SPDIFRX1();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn USART1();
        fn USART2();
        fn USART3();
        fn UART4();
        fn UART5();
        fn USART6();
        fn UART7();
        fn UART8();
        fn UART9();
        fn USART10();
        fn LPUART1();
        fn XSPI1();
        fn XSPI2();
        fn XSPI3();
        fn FMC();
        fn SDMMC1();
        fn SDMMC2();
        fn UCPD1();
        fn USB1_OTG_HS();
        fn USB2_OTG_HS();
        fn ETH1();
        fn FDCAN1_IT0();
        fn FDCAN1_IT1();
        fn FDCAN2_IT0();
        fn FDCAN2_IT1();
        fn FDCAN3_IT0();
        fn FDCAN3_IT1();
        fn FDCAN_CU();
        fn MDIOS();
        fn DCMI_PSSI();
        fn WAKEUP_PIN();
        fn CTI_INT0();
        fn CTI_INT1();
        fn LTDC_UP();
        fn LTDC_UP_ERR();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 195] = [
        Vector { _handler: PVD_PVM },
        Vector { _reserved: 0 },
        Vector { _handler: DTS },
        Vector { _handler: RCC },
        Vector { _handler: LOCKUP },
        Vector { _handler: CACHE_ECC },
        Vector { _handler: TCM_ECC },
        Vector { _handler: BKP_ECC },
        Vector { _handler: FPU },
        Vector { _reserved: 0 },
        Vector { _handler: RTC_S },
        Vector { _handler: TAMP },
        Vector { _handler: RIFSC_TAMPER },
        Vector { _handler: IAC },
        Vector { _handler: RCC_S },
        Vector { _reserved: 0 },
        Vector { _handler: RTC },
        Vector { _reserved: 0 },
        Vector { _handler: IWDG },
        Vector { _handler: WWDG },
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
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: PKA },
        Vector { _handler: HASH },
        Vector { _handler: RNG },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC1_2 },
        Vector { _handler: CSI },
        Vector { _handler: DCMIPP },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: PAHB_ERR },
        Vector { _handler: NPU0 },
        Vector { _handler: NPU1 },
        Vector { _handler: NPU2 },
        Vector { _handler: NPU3 },
        Vector { _handler: CACHEAXI },
        Vector { _handler: LTDC_LO },
        Vector { _handler: LTDC_LO_ERR },
        Vector { _handler: DMA2D },
        Vector { _handler: JPEG },
        Vector { _handler: VENC },
        Vector { _handler: GFXMMU },
        Vector { _handler: GFXTIM },
        Vector { _handler: GPU2D },
        Vector { _handler: GPU2D_ER },
        Vector { _handler: ICACHE },
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
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: I2C4_EV },
        Vector { _handler: I2C4_ER },
        Vector { _handler: I3C1_EV },
        Vector { _handler: I3C1_ER },
        Vector { _handler: I3C2_EV },
        Vector { _handler: I3C2_ER },
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
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: TIM9 },
        Vector { _handler: TIM10 },
        Vector { _handler: TIM11 },
        Vector { _handler: TIM12 },
        Vector { _handler: TIM13 },
        Vector { _handler: TIM14 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: TIM18 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: LPTIM3 },
        Vector { _handler: LPTIM4 },
        Vector { _handler: LPTIM5 },
        Vector { _handler: ADF1_FLT0 },
        Vector { _handler: MDF1_FLT0 },
        Vector { _handler: MDF1_FLT1 },
        Vector { _handler: MDF1_FLT2 },
        Vector { _handler: MDF1_FLT3 },
        Vector { _handler: MDF1_FLT4 },
        Vector { _handler: MDF1_FLT5 },
        Vector { _handler: SAI1_A },
        Vector { _handler: SAI1_B },
        Vector { _handler: SAI2_A },
        Vector { _handler: SAI2_B },
        Vector { _handler: SPDIFRX1 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: USART6 },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: UART9 },
        Vector { _handler: USART10 },
        Vector { _handler: LPUART1 },
        Vector { _handler: XSPI1 },
        Vector { _handler: XSPI2 },
        Vector { _handler: XSPI3 },
        Vector { _handler: FMC },
        Vector { _handler: SDMMC1 },
        Vector { _handler: SDMMC2 },
        Vector { _handler: UCPD1 },
        Vector { _handler: USB1_OTG_HS },
        Vector { _handler: USB2_OTG_HS },
        Vector { _handler: ETH1 },
        Vector { _handler: FDCAN1_IT0 },
        Vector { _handler: FDCAN1_IT1 },
        Vector { _handler: FDCAN2_IT0 },
        Vector { _handler: FDCAN2_IT1 },
        Vector { _handler: FDCAN3_IT0 },
        Vector { _handler: FDCAN3_IT1 },
        Vector { _handler: FDCAN_CU },
        Vector { _handler: MDIOS },
        Vector { _handler: DCMI_PSSI },
        Vector { _handler: WAKEUP_PIN },
        Vector { _handler: CTI_INT0 },
        Vector { _handler: CTI_INT1 },
        Vector { _reserved: 0 },
        Vector { _handler: LTDC_UP },
        Vector { _handler: LTDC_UP_ERR },
    ];
}
pub const TIM2: *mut () = 0x4000_0000usize as _;
pub const TIM3: *mut () = 0x4000_0400usize as _;
pub const TIM6: *mut () = 0x4000_1000usize as _;
pub const TIM7: *mut () = 0x4000_1400usize as _;
pub const TIM13: *mut () = 0x4000_1c00usize as _;
pub const TIM14: *mut () = 0x4000_2000usize as _;
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const TIM10: *mut () = 0x4000_3000usize as _;
pub const TIM11: *mut () = 0x4000_3400usize as _;
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00usize as _) };
pub const I3C1: *mut () = 0x4000_6000usize as _;
pub const I3C2: *mut () = 0x4000_6400usize as _;
pub const UART7: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_7800usize as _) };
pub const MDIOS: mdios::Mdios = unsafe { mdios::Mdios::from_ptr(0x4000_9400usize as _) };
pub const FDCAN1: *mut () = 0x4000_a000usize as _;
pub const FDCAN2: *mut () = 0x4000_a400usize as _;
pub const FDCANRAM1: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_c000usize as _) };
pub const FDCANRAM2: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_c350usize as _) };
pub const FDCANRAM3: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_c6a0usize as _) };
pub const FDCAN3: *mut () = 0x4000_e800usize as _;
pub const UCPD1: ucpd::Ucpd = unsafe { ucpd::Ucpd::from_ptr(0x4000_fc00usize as _) };
pub const GPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_1000usize as _) };
pub const ADC1: *mut () = 0x4002_2000usize as _;
pub const ADC2: *mut () = 0x4002_2100usize as _;
pub const ADC12_COMMON: *mut () = 0x4002_2300usize as _;
pub const TIM1: *mut () = 0x4200_0000usize as _;
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4200_1000usize as _) };
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4200_1400usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4200_3000usize as _) };
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4200_3400usize as _) };
pub const TIM18: *mut () = 0x4200_3c00usize as _;
pub const TIM15: *mut () = 0x4200_4000usize as _;
pub const TIM16: *mut () = 0x4200_4400usize as _;
pub const TIM9: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4200_4c00usize as _) };
pub const SPI5: spi::Spi = unsafe { spi::Spi::from_ptr(0x4200_5000usize as _) };
pub const SAI1: *mut () = 0x4200_5800usize as _;
pub const SAI2: *mut () = 0x4200_5c00usize as _;
pub const RAMCFG: ramcfg::Ramcfg = unsafe { ramcfg::Ramcfg::from_ptr(0x4202_3000usize as _) };
pub const MDF1: *mut () = 0x4202_5000usize as _;
pub const ADF1: *mut () = 0x4202_6000usize as _;
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0x4400_1000usize as _) };
pub const RNG: *mut () = 0x4402_0000usize as _;
pub const HASH: hash::Hash = unsafe { hash::Hash::from_ptr(0x4402_0400usize as _) };
pub const RIFSC: rifsc::Rifsc = unsafe { rifsc::Rifsc::from_ptr(0x4402_4000usize as _) };
pub const RISAF1: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_6000usize as _) };
pub const RISAF2: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_7000usize as _) };
pub const RISAF3: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_8000usize as _) };
pub const RISAF4: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_9000usize as _) };
pub const RISAF5: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_a000usize as _) };
pub const RISAF6: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_b000usize as _) };
pub const RISAF7: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_c000usize as _) };
pub const RISAF8: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_d000usize as _) };
pub const RISAF9: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4402_e000usize as _) };
pub const RISAF11: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4403_0000usize as _) };
pub const RISAF12: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4403_1000usize as _) };
pub const RISAF13: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4403_2000usize as _) };
pub const RISAF14: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4403_3000usize as _) };
pub const RISAF15: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4403_4000usize as _) };
pub const RISAF21: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4403_5000usize as _) };
pub const RISAF22: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4403_6000usize as _) };
pub const RISAF23: risaf::Risaf = unsafe { risaf::Risaf::from_ptr(0x4403_7000usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4600_0c00usize as _) };
pub const SPI6: spi::Spi = unsafe { spi::Spi::from_ptr(0x4600_1400usize as _) };
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4600_1c00usize as _) };
pub const LPTIM2: *mut () = 0x4600_2400usize as _;
pub const VREFBUF: *mut () = 0x4600_3c00usize as _;
pub const RTC: *mut () = 0x4600_4000usize as _;
pub const TAMP: *mut () = 0x4600_4400usize as _;
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4600_4800usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4600_8000usize as _) };
pub const BSEC: bsec::Bsec = unsafe { bsec::Bsec::from_ptr(0x4600_9000usize as _) };
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x4600_9014usize as _) };
pub const DTS: dts::Dts = unsafe { dts::Dts::from_ptr(0x4600_a000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_1c00usize as _) };
pub const GPION: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_3400usize as _) };
pub const GPIOO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_3800usize as _) };
pub const GPIOP: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_3c00usize as _) };
pub const GPIOQ: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4602_4000usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4602_4800usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4602_4c00usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4602_5000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4602_8000usize as _) };
pub const LTDC: ltdc::Ltdc = unsafe { ltdc::Ltdc::from_ptr(0x4800_1000usize as _) };
pub const DCMIPP: *mut () = 0x4800_2000usize as _;
pub const GFXTIM: *mut () = 0x4800_4000usize as _;
pub const VENC: *mut () = 0x4800_5000usize as _;
pub const CSI: *mut () = 0x4800_6000usize as _;
pub const HPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4802_0000usize as _) };
pub const DMA2D: dma2d::Dma2d = unsafe { dma2d::Dma2d::from_ptr(0x4802_1000usize as _) };
pub const JPEG: jpeg::Jpeg = unsafe { jpeg::Jpeg::from_ptr(0x4802_3000usize as _) };
pub const XSPI1: xspi::Xspi = unsafe { xspi::Xspi::from_ptr(0x4802_5000usize as _) };
pub const PSSI: *mut () = 0x4802_6400usize as _;
pub const DCMI: dcmi::Dcmi = unsafe { dcmi::Dcmi::from_ptr(0x4802_8400usize as _) };
pub const XSPI2: xspi::Xspi = unsafe { xspi::Xspi::from_ptr(0x4802_a000usize as _) };
pub const XSPIM: xspim::Xspim = unsafe { xspim::Xspim::from_ptr(0x4802_b400usize as _) };
pub const XSPI3: xspi::Xspi = unsafe { xspi::Xspi::from_ptr(0x4802_d000usize as _) };
pub const GFXMMU: *mut () = 0x4803_0000usize as _;
pub const GPU2D: *mut () = 0x4803_4000usize as _;
pub const ICACHE: icache::Icache = unsafe { icache::Icache::from_ptr(0x4803_5000usize as _) };
pub const ETH1: *mut () = 0x4803_6000usize as _;
pub const USB1_OTG_HS: otg::Otg = unsafe { otg::Otg::from_ptr(0x4804_0000usize as _) };
pub const USB2_OTG_HS: otg::Otg = unsafe { otg::Otg::from_ptr(0x4808_0000usize as _) };
pub const CACHEAXI: *mut () = 0x480d_fc00usize as _;
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/bsec_v2.rs"]
pub mod bsec;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dbgmcu_n6.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../../peripherals/dma2d_v1.rs"]
pub mod dma2d;
#[path = "../../peripherals/dts_v1.rs"]
pub mod dts;
#[path = "../../peripherals/exti_n6.rs"]
pub mod exti;
#[path = "../../peripherals/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../../peripherals/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/hash_v3.rs"]
pub mod hash;
#[path = "../../peripherals/i2c_v3.rs"]
pub mod i2c;
#[path = "../../peripherals/icache_v1_0crr.rs"]
pub mod icache;
#[path = "../../peripherals/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../../peripherals/jpeg_v1.rs"]
pub mod jpeg;
#[path = "../../peripherals/ltdc_v1_3.rs"]
pub mod ltdc;
#[path = "../../peripherals/mdios_v1.rs"]
pub mod mdios;
#[path = "../../peripherals/otg_v1.rs"]
pub mod otg;
#[path = "../../peripherals/pwr_n6.rs"]
pub mod pwr;
#[path = "../../peripherals/ramcfg_h5.rs"]
pub mod ramcfg;
#[path = "../../peripherals/rcc_n6.rs"]
pub mod rcc;
#[path = "../../peripherals/rifsc_n6.rs"]
pub mod rifsc;
#[path = "../../peripherals/risaf_n6.rs"]
pub mod risaf;
#[path = "../../peripherals/spi_v5.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_n6.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/ucpd_v1.rs"]
pub mod ucpd;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
#[path = "../../peripherals/xspi_v1.rs"]
pub mod xspi;
#[path = "../../peripherals/xspim_v1.rs"]
pub mod xspim;
