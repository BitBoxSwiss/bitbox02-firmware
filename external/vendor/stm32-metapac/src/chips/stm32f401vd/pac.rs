#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD"]
    PVD = 1,
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
    #[doc = "23 - EXTI9_5"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1_BRK_TIM9"]
    TIM1_BRK_TIM9 = 24,
    #[doc = "25 - TIM1_UP_TIM10"]
    TIM1_UP_TIM10 = 25,
    #[doc = "26 - TIM1_TRG_COM_TIM11"]
    TIM1_TRG_COM_TIM11 = 26,
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
    #[doc = "40 - EXTI15_10"]
    EXTI15_10 = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "42 - OTG_FS_WKUP"]
    OTG_FS_WKUP = 42,
    #[doc = "47 - DMA1_STREAM7"]
    DMA1_STREAM7 = 47,
    #[doc = "49 - SDIO"]
    SDIO = 49,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
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
    #[doc = "67 - OTG_FS"]
    OTG_FS = 67,
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
    #[doc = "81 - FPU"]
    FPU = 81,
    #[doc = "84 - SPI4"]
    SPI4 = 84,
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
        fn PVD();
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
        fn EXTI9_5();
        fn TIM1_BRK_TIM9();
        fn TIM1_UP_TIM10();
        fn TIM1_TRG_COM_TIM11();
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
        fn EXTI15_10();
        fn RTC_ALARM();
        fn OTG_FS_WKUP();
        fn DMA1_STREAM7();
        fn SDIO();
        fn TIM5();
        fn SPI3();
        fn DMA2_STREAM0();
        fn DMA2_STREAM1();
        fn DMA2_STREAM2();
        fn DMA2_STREAM3();
        fn DMA2_STREAM4();
        fn OTG_FS();
        fn DMA2_STREAM5();
        fn DMA2_STREAM6();
        fn DMA2_STREAM7();
        fn USART6();
        fn I2C3_EV();
        fn I2C3_ER();
        fn FPU();
        fn SPI4();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 85] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD },
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
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: EXTI9_5 },
        Vector {
            _handler: TIM1_BRK_TIM9,
        },
        Vector {
            _handler: TIM1_UP_TIM10,
        },
        Vector {
            _handler: TIM1_TRG_COM_TIM11,
        },
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
        Vector { _reserved: 0 },
        Vector { _handler: EXTI15_10 },
        Vector { _handler: RTC_ALARM },
        Vector { _handler: OTG_FS_WKUP },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMA1_STREAM7 },
        Vector { _reserved: 0 },
        Vector { _handler: SDIO },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMA2_STREAM0 },
        Vector { _handler: DMA2_STREAM1 },
        Vector { _handler: DMA2_STREAM2 },
        Vector { _handler: DMA2_STREAM3 },
        Vector { _handler: DMA2_STREAM4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: OTG_FS },
        Vector { _handler: DMA2_STREAM5 },
        Vector { _handler: DMA2_STREAM6 },
        Vector { _handler: DMA2_STREAM7 },
        Vector { _handler: USART6 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FPU },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI4 },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1fff_7a10usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_0000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_1000usize as _) };
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_1400usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2000usize as _) };
pub const ADC1_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4001_2300usize as _) };
pub const SDIO: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3400usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_3800usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_3c00usize as _) };
pub const TIM9: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4001_4000usize as _) };
pub const TIM10: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_4400usize as _) };
pub const TIM11: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_4800usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1000usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1c00usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_3800usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_3c00usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_6000usize as _) };
pub const DMA2: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_6400usize as _) };
pub const USB_OTG_FS: otg::Otg = unsafe { otg::Otg::from_ptr(0x5000_0000usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_v2.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_v2.rs"]
pub mod adccommon;
#[path = "../../peripherals/crc_v1.rs"]
pub mod crc;
#[path = "../../peripherals/dbgmcu_f4.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dma_v2.rs"]
pub mod dma;
#[path = "../../peripherals/exti_v1.rs"]
pub mod exti;
#[path = "../../peripherals/flash_f4.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v1.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../../peripherals/otg_v1.rs"]
pub mod otg;
#[path = "../../peripherals/pwr_f4.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_f4.rs"]
pub mod rcc;
#[path = "../../peripherals/rtc_v2_f4.rs"]
pub mod rtc;
#[path = "../../peripherals/sdmmc_v1.rs"]
pub mod sdmmc;
#[path = "../../peripherals/spi_v2_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_f4.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v2.rs"]
pub mod usart;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
