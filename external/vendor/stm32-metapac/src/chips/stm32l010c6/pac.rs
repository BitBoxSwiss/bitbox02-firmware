#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - FLASH"]
    FLASH = 3,
    #[doc = "4 - RCC"]
    RCC = 4,
    #[doc = "5 - EXTI0_1"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI2_3"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI4_15"]
    EXTI4_15 = 7,
    #[doc = "9 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 9,
    #[doc = "10 - DMA1_CHANNEL2_3"]
    DMA1_CHANNEL2_3 = 10,
    #[doc = "11 - DMA1_CHANNEL4_5_6_7"]
    DMA1_CHANNEL4_5_6_7 = 11,
    #[doc = "12 - ADC1"]
    ADC1 = 12,
    #[doc = "13 - LPTIM1"]
    LPTIM1 = 13,
    #[doc = "15 - TIM2"]
    TIM2 = 15,
    #[doc = "20 - TIM21"]
    TIM21 = 20,
    #[doc = "23 - I2C1"]
    I2C1 = 23,
    #[doc = "25 - SPI1"]
    SPI1 = 25,
    #[doc = "28 - USART2"]
    USART2 = 28,
    #[doc = "29 - LPUART1"]
    LPUART1 = 29,
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
        fn RTC();
        fn FLASH();
        fn RCC();
        fn EXTI0_1();
        fn EXTI2_3();
        fn EXTI4_15();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2_3();
        fn DMA1_CHANNEL4_5_6_7();
        fn ADC1();
        fn LPTIM1();
        fn TIM2();
        fn TIM21();
        fn I2C1();
        fn SPI1();
        fn USART2();
        fn LPUART1();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 30] = [
        Vector { _handler: WWDG },
        Vector { _reserved: 0 },
        Vector { _handler: RTC },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0_1 },
        Vector { _handler: EXTI2_3 },
        Vector { _handler: EXTI4_15 },
        Vector { _reserved: 0 },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2_3,
        },
        Vector {
            _handler: DMA1_CHANNEL4_5_6_7,
        },
        Vector { _handler: ADC1 },
        Vector { _handler: LPTIM1 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM21 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2C1 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: USART2 },
        Vector { _handler: LPUART1 },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1ff8_0050usize as _) };
pub const VREFINTCAL: vrefintcal::Vrefintcal = unsafe { vrefintcal::Vrefintcal::from_ptr(0x1ff8_0078usize as _) };
pub const TIM2: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4000_4800usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_7c00usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const TIM21: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4001_0800usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const ADC1_COMMON: *mut () = 0x4001_2708usize as _;
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0x4001_5800usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_1c00usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_l0.rs"]
pub mod adc;
#[path = "../../peripherals/bdma_v2.rs"]
pub mod bdma;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dbgmcu_l0.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/exti_v1.rs"]
pub mod exti;
#[path = "../../peripherals/flash_l0.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/lptim_v1.rs"]
pub mod lptim;
#[path = "../../peripherals/pwr_l0.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_l0.rs"]
pub mod rcc;
#[path = "../../peripherals/rtc_v2_l0.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v2.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_l0.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_l0.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v3.rs"]
pub mod usart;
#[path = "../../peripherals/vrefintcal_v1.rs"]
pub mod vrefintcal;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
