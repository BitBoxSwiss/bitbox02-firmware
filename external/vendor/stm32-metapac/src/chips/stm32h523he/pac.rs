#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD_AVD"]
    PVD_AVD = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - RTC_S"]
    RTC_S = 3,
    #[doc = "4 - TAMP"]
    TAMP = 4,
    #[doc = "5 - RAMCFG"]
    RAMCFG = 5,
    #[doc = "6 - FLASH"]
    FLASH = 6,
    #[doc = "7 - FLASH_S"]
    FLASH_S = 7,
    #[doc = "8 - GTZC"]
    GTZC = 8,
    #[doc = "9 - RCC"]
    RCC = 9,
    #[doc = "10 - RCC_S"]
    RCC_S = 10,
    #[doc = "11 - EXTI0"]
    EXTI0 = 11,
    #[doc = "12 - EXTI1"]
    EXTI1 = 12,
    #[doc = "13 - EXTI2"]
    EXTI2 = 13,
    #[doc = "14 - EXTI3"]
    EXTI3 = 14,
    #[doc = "15 - EXTI4"]
    EXTI4 = 15,
    #[doc = "16 - EXTI5"]
    EXTI5 = 16,
    #[doc = "17 - EXTI6"]
    EXTI6 = 17,
    #[doc = "18 - EXTI7"]
    EXTI7 = 18,
    #[doc = "19 - EXTI8"]
    EXTI8 = 19,
    #[doc = "20 - EXTI9"]
    EXTI9 = 20,
    #[doc = "21 - EXTI10"]
    EXTI10 = 21,
    #[doc = "22 - EXTI11"]
    EXTI11 = 22,
    #[doc = "23 - EXTI12"]
    EXTI12 = 23,
    #[doc = "24 - EXTI13"]
    EXTI13 = 24,
    #[doc = "25 - EXTI14"]
    EXTI14 = 25,
    #[doc = "26 - EXTI15"]
    EXTI15 = 26,
    #[doc = "27 - GPDMA1_CHANNEL0"]
    GPDMA1_CHANNEL0 = 27,
    #[doc = "28 - GPDMA1_CHANNEL1"]
    GPDMA1_CHANNEL1 = 28,
    #[doc = "29 - GPDMA1_CHANNEL2"]
    GPDMA1_CHANNEL2 = 29,
    #[doc = "30 - GPDMA1_CHANNEL3"]
    GPDMA1_CHANNEL3 = 30,
    #[doc = "31 - GPDMA1_CHANNEL4"]
    GPDMA1_CHANNEL4 = 31,
    #[doc = "32 - GPDMA1_CHANNEL5"]
    GPDMA1_CHANNEL5 = 32,
    #[doc = "33 - GPDMA1_CHANNEL6"]
    GPDMA1_CHANNEL6 = 33,
    #[doc = "34 - GPDMA1_CHANNEL7"]
    GPDMA1_CHANNEL7 = 34,
    #[doc = "35 - IWDG"]
    IWDG = 35,
    #[doc = "37 - ADC1"]
    ADC1 = 37,
    #[doc = "38 - DAC1"]
    DAC1 = 38,
    #[doc = "39 - FDCAN1_IT0"]
    FDCAN1_IT0 = 39,
    #[doc = "40 - FDCAN1_IT1"]
    FDCAN1_IT1 = 40,
    #[doc = "41 - TIM1_BRK"]
    TIM1_BRK = 41,
    #[doc = "42 - TIM1_UP"]
    TIM1_UP = 42,
    #[doc = "43 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 43,
    #[doc = "44 - TIM1_CC"]
    TIM1_CC = 44,
    #[doc = "45 - TIM2"]
    TIM2 = 45,
    #[doc = "46 - TIM3"]
    TIM3 = 46,
    #[doc = "47 - TIM4"]
    TIM4 = 47,
    #[doc = "48 - TIM5"]
    TIM5 = 48,
    #[doc = "49 - TIM6"]
    TIM6 = 49,
    #[doc = "50 - TIM7"]
    TIM7 = 50,
    #[doc = "51 - I2C1_EV"]
    I2C1_EV = 51,
    #[doc = "52 - I2C1_ER"]
    I2C1_ER = 52,
    #[doc = "53 - I2C2_EV"]
    I2C2_EV = 53,
    #[doc = "54 - I2C2_ER"]
    I2C2_ER = 54,
    #[doc = "55 - SPI1"]
    SPI1 = 55,
    #[doc = "56 - SPI2"]
    SPI2 = 56,
    #[doc = "57 - SPI3"]
    SPI3 = 57,
    #[doc = "58 - USART1"]
    USART1 = 58,
    #[doc = "59 - USART2"]
    USART2 = 59,
    #[doc = "60 - USART3"]
    USART3 = 60,
    #[doc = "61 - UART4"]
    UART4 = 61,
    #[doc = "62 - UART5"]
    UART5 = 62,
    #[doc = "63 - LPUART1"]
    LPUART1 = 63,
    #[doc = "64 - LPTIM1"]
    LPTIM1 = 64,
    #[doc = "65 - TIM8_BRK"]
    TIM8_BRK = 65,
    #[doc = "66 - TIM8_UP"]
    TIM8_UP = 66,
    #[doc = "67 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 67,
    #[doc = "68 - TIM8_CC"]
    TIM8_CC = 68,
    #[doc = "69 - ADC2"]
    ADC2 = 69,
    #[doc = "70 - LPTIM2"]
    LPTIM2 = 70,
    #[doc = "71 - TIM15"]
    TIM15 = 71,
    #[doc = "74 - USB_DRD_FS"]
    USB_DRD_FS = 74,
    #[doc = "75 - CRS"]
    CRS = 75,
    #[doc = "76 - UCPD1"]
    UCPD1 = 76,
    #[doc = "77 - FMC"]
    FMC = 77,
    #[doc = "78 - OCTOSPI1"]
    OCTOSPI1 = 78,
    #[doc = "79 - SDMMC1"]
    SDMMC1 = 79,
    #[doc = "80 - I2C3_EV"]
    I2C3_EV = 80,
    #[doc = "81 - I2C3_ER"]
    I2C3_ER = 81,
    #[doc = "82 - SPI4"]
    SPI4 = 82,
    #[doc = "85 - USART6"]
    USART6 = 85,
    #[doc = "90 - GPDMA2_CHANNEL0"]
    GPDMA2_CHANNEL0 = 90,
    #[doc = "91 - GPDMA2_CHANNEL1"]
    GPDMA2_CHANNEL1 = 91,
    #[doc = "92 - GPDMA2_CHANNEL2"]
    GPDMA2_CHANNEL2 = 92,
    #[doc = "93 - GPDMA2_CHANNEL3"]
    GPDMA2_CHANNEL3 = 93,
    #[doc = "94 - GPDMA2_CHANNEL4"]
    GPDMA2_CHANNEL4 = 94,
    #[doc = "95 - GPDMA2_CHANNEL5"]
    GPDMA2_CHANNEL5 = 95,
    #[doc = "96 - GPDMA2_CHANNEL6"]
    GPDMA2_CHANNEL6 = 96,
    #[doc = "97 - GPDMA2_CHANNEL7"]
    GPDMA2_CHANNEL7 = 97,
    #[doc = "103 - FPU"]
    FPU = 103,
    #[doc = "104 - ICACHE"]
    ICACHE = 104,
    #[doc = "105 - DCACHE1"]
    DCACHE1 = 105,
    #[doc = "108 - DCMI_PSSI"]
    DCMI_PSSI = 108,
    #[doc = "109 - FDCAN2_IT0"]
    FDCAN2_IT0 = 109,
    #[doc = "110 - FDCAN2_IT1"]
    FDCAN2_IT1 = 110,
    #[doc = "113 - DTS"]
    DTS = 113,
    #[doc = "114 - RNG"]
    RNG = 114,
    #[doc = "117 - HASH"]
    HASH = 117,
    #[doc = "118 - PKA"]
    PKA = 118,
    #[doc = "119 - CEC"]
    CEC = 119,
    #[doc = "120 - TIM12"]
    TIM12 = 120,
    #[doc = "123 - I3C1_EV"]
    I3C1_EV = 123,
    #[doc = "124 - I3C1_ER"]
    I3C1_ER = 124,
    #[doc = "131 - I3C2_EV"]
    I3C2_EV = 131,
    #[doc = "132 - I3C2_ER"]
    I3C2_ER = 132,
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
        fn RTC();
        fn RTC_S();
        fn TAMP();
        fn RAMCFG();
        fn FLASH();
        fn FLASH_S();
        fn GTZC();
        fn RCC();
        fn RCC_S();
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
        fn GPDMA1_CHANNEL0();
        fn GPDMA1_CHANNEL1();
        fn GPDMA1_CHANNEL2();
        fn GPDMA1_CHANNEL3();
        fn GPDMA1_CHANNEL4();
        fn GPDMA1_CHANNEL5();
        fn GPDMA1_CHANNEL6();
        fn GPDMA1_CHANNEL7();
        fn IWDG();
        fn ADC1();
        fn DAC1();
        fn FDCAN1_IT0();
        fn FDCAN1_IT1();
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
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn USART1();
        fn USART2();
        fn USART3();
        fn UART4();
        fn UART5();
        fn LPUART1();
        fn LPTIM1();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn ADC2();
        fn LPTIM2();
        fn TIM15();
        fn USB_DRD_FS();
        fn CRS();
        fn UCPD1();
        fn FMC();
        fn OCTOSPI1();
        fn SDMMC1();
        fn I2C3_EV();
        fn I2C3_ER();
        fn SPI4();
        fn USART6();
        fn GPDMA2_CHANNEL0();
        fn GPDMA2_CHANNEL1();
        fn GPDMA2_CHANNEL2();
        fn GPDMA2_CHANNEL3();
        fn GPDMA2_CHANNEL4();
        fn GPDMA2_CHANNEL5();
        fn GPDMA2_CHANNEL6();
        fn GPDMA2_CHANNEL7();
        fn FPU();
        fn ICACHE();
        fn DCACHE1();
        fn DCMI_PSSI();
        fn FDCAN2_IT0();
        fn FDCAN2_IT1();
        fn DTS();
        fn RNG();
        fn HASH();
        fn PKA();
        fn CEC();
        fn TIM12();
        fn I3C1_EV();
        fn I3C1_ER();
        fn I3C2_EV();
        fn I3C2_ER();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 133] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD_AVD },
        Vector { _handler: RTC },
        Vector { _handler: RTC_S },
        Vector { _handler: TAMP },
        Vector { _handler: RAMCFG },
        Vector { _handler: FLASH },
        Vector { _handler: FLASH_S },
        Vector { _handler: GTZC },
        Vector { _handler: RCC },
        Vector { _handler: RCC_S },
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
        Vector { _handler: IWDG },
        Vector { _reserved: 0 },
        Vector { _handler: ADC1 },
        Vector { _handler: DAC1 },
        Vector { _handler: FDCAN1_IT0 },
        Vector { _handler: FDCAN1_IT1 },
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
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: ADC2 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: TIM15 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: USB_DRD_FS },
        Vector { _handler: CRS },
        Vector { _handler: UCPD1 },
        Vector { _handler: FMC },
        Vector { _handler: OCTOSPI1 },
        Vector { _handler: SDMMC1 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: SPI4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: USART6 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: GPDMA2_CHANNEL0,
        },
        Vector {
            _handler: GPDMA2_CHANNEL1,
        },
        Vector {
            _handler: GPDMA2_CHANNEL2,
        },
        Vector {
            _handler: GPDMA2_CHANNEL3,
        },
        Vector {
            _handler: GPDMA2_CHANNEL4,
        },
        Vector {
            _handler: GPDMA2_CHANNEL5,
        },
        Vector {
            _handler: GPDMA2_CHANNEL6,
        },
        Vector {
            _handler: GPDMA2_CHANNEL7,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FPU },
        Vector { _handler: ICACHE },
        Vector { _handler: DCACHE1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DCMI_PSSI },
        Vector { _handler: FDCAN2_IT0 },
        Vector { _handler: FDCAN2_IT1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DTS },
        Vector { _handler: RNG },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: HASH },
        Vector { _handler: PKA },
        Vector { _handler: CEC },
        Vector { _handler: TIM12 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I3C1_EV },
        Vector { _handler: I3C1_ER },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I3C2_EV },
        Vector { _handler: I3C2_ER },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x08ff_f800usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM12: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4000_1800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I3C1: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4000_5c00usize as _) };
pub const CRS: crs::Crs = unsafe { crs::Crs::from_ptr(0x4000_6000usize as _) };
pub const CEC: cec::Cec = unsafe { cec::Cec::from_ptr(0x4000_7000usize as _) };
pub const DTS: dts::Dts = unsafe { dts::Dts::from_ptr(0x4000_8c00usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_9400usize as _) };
pub const FDCAN1: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_a400usize as _) };
pub const FDCAN2: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_a800usize as _) };
pub const FDCANRAM1: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_ac00usize as _) };
pub const FDCANRAM2: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_af50usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_3400usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM15: timer::Tim2chCmp = unsafe { timer::Tim2chCmp::from_ptr(0x4001_4000usize as _) };
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0x4001_6000usize as _) };
pub const USBRAM: usbram::Usbram = unsafe { usbram::Usbram::from_ptr(0x4001_6400usize as _) };
pub const GPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_0000usize as _) };
pub const GPDMA2: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const RAMCFG: ramcfg::Ramcfg = unsafe { ramcfg::Ramcfg::from_ptr(0x4002_6000usize as _) };
pub const ICACHE: *mut () = 0x4003_0400usize as _;
pub const DCACHE1: dcache::Dcache = unsafe { dcache::Dcache::from_ptr(0x4003_1400usize as _) };
pub const GTZC: gtzc::Gtzc1Tzsc = unsafe { gtzc::Gtzc1Tzsc::from_ptr(0x4003_2400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1c00usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4202_8000usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x4202_8100usize as _) };
pub const ADC12_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4202_8300usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4202_8400usize as _) };
pub const HASH: hash::Hash = unsafe { hash::Hash::from_ptr(0x420c_0400usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x420c_0800usize as _) };
pub const PKA: pka::Pka = unsafe { pka::Pka::from_ptr(0x420c_2000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4400_0400usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4400_2400usize as _) };
pub const I3C2: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4400_3000usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4400_4400usize as _) };
pub const VREFBUF: vrefbuf::Vrefbuf = unsafe { vrefbuf::Vrefbuf::from_ptr(0x4400_7400usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4400_7800usize as _) };
pub const TAMP: tamp::Tamp = unsafe { tamp::Tamp::from_ptr(0x4400_7c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4402_0800usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4402_0c00usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4402_2000usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0x4402_4000usize as _) };
pub const DLYB_SDMMC1: dlyb::Dlyb = unsafe { dlyb::Dlyb::from_ptr(0x4600_8400usize as _) };
pub const DLYB_OCTOSPI1: dlyb::Dlyb = unsafe { dlyb::Dlyb::from_ptr(0x4600_f000usize as _) };
pub const OCTOSPI1: octospi::Octospi = unsafe { octospi::Octospi::from_ptr(0x4700_1400usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_h5.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_h5.rs"]
pub mod adccommon;
#[path = "../../peripherals/can_fdcan_v1.rs"]
pub mod can;
#[path = "../../peripherals/cec_v2.rs"]
pub mod cec;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/crs_v1.rs"]
pub mod crs;
#[path = "../../peripherals/dac_v6.rs"]
pub mod dac;
#[path = "../../peripherals/dbgmcu_h5.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dcache_v1.rs"]
pub mod dcache;
#[path = "../../peripherals/dlyb_v1.rs"]
pub mod dlyb;
#[path = "../../peripherals/dts_v1.rs"]
pub mod dts;
#[path = "../../peripherals/exti_h5.rs"]
pub mod exti;
#[path = "../../peripherals/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../../peripherals/flash_h5.rs"]
pub mod flash;
#[path = "../../peripherals/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/gtzc_v1.rs"]
pub mod gtzc;
#[path = "../../peripherals/hash_v3.rs"]
pub mod hash;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/i3c_v1.rs"]
pub mod i3c;
#[path = "../../peripherals/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../../peripherals/lptim_v2a.rs"]
pub mod lptim;
#[path = "../../peripherals/octospi_v2.rs"]
pub mod octospi;
#[path = "../../peripherals/pka_v1a.rs"]
pub mod pka;
#[path = "../../peripherals/pwr_h5.rs"]
pub mod pwr;
#[path = "../../peripherals/ramcfg_h5.rs"]
pub mod ramcfg;
#[path = "../../peripherals/rcc_h5.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v3.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v3_u5.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v5_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_h5.rs"]
pub mod syscfg;
#[path = "../../peripherals/tamp_h5.rs"]
pub mod tamp;
#[path = "../../peripherals/timer_v2.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/usb_v4.rs"]
pub mod usb;
#[path = "../../peripherals/usbram_32_2048.rs"]
pub mod usbram;
#[path = "../../peripherals/vrefbuf_v2a2.rs"]
pub mod vrefbuf;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
