#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - FLASH"]
    FLASH = 0,
    #[doc = "1 - RCC"]
    RCC = 1,
    #[doc = "2 - PVD"]
    PVD = 2,
    #[doc = "3 - I2C1"]
    I2C1 = 3,
    #[doc = "7 - SPI3"]
    SPI3 = 7,
    #[doc = "8 - USART1"]
    USART1 = 8,
    #[doc = "9 - LPUART1"]
    LPUART1 = 9,
    #[doc = "10 - TIM2"]
    TIM2 = 10,
    #[doc = "11 - RTC"]
    RTC = 11,
    #[doc = "12 - ADC"]
    ADC = 12,
    #[doc = "13 - PKA"]
    PKA = 13,
    #[doc = "14 - UPCONV"]
    UPCONV = 14,
    #[doc = "15 - GPIOA"]
    GPIOA = 15,
    #[doc = "16 - GPIOB"]
    GPIOB = 16,
    #[doc = "17 - DMA"]
    DMA = 17,
    #[doc = "18 - RADIO_TXRX"]
    RADIO_TXRX = 18,
    #[doc = "20 - RADIO_TIMER_ERROR"]
    RADIO_TIMER_ERROR = 20,
    #[doc = "23 - RADIO_TIMER_CPU_WKUP"]
    RADIO_TIMER_CPU_WKUP = 23,
    #[doc = "24 - RADIO_TIMER_TXRX_WKUP"]
    RADIO_TIMER_TXRX_WKUP = 24,
    #[doc = "25 - RADIO_TXRX_SEQ"]
    RADIO_TXRX_SEQ = 25,
    #[doc = "26 - TIM16"]
    TIM16 = 26,
    #[doc = "27 - TIM17"]
    TIM17 = 27,
    #[doc = "28 - RNG"]
    RNG = 28,
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
        fn FLASH();
        fn RCC();
        fn PVD();
        fn I2C1();
        fn SPI3();
        fn USART1();
        fn LPUART1();
        fn TIM2();
        fn RTC();
        fn ADC();
        fn PKA();
        fn UPCONV();
        fn GPIOA();
        fn GPIOB();
        fn DMA();
        fn RADIO_TXRX();
        fn RADIO_TIMER_ERROR();
        fn RADIO_TIMER_CPU_WKUP();
        fn RADIO_TIMER_TXRX_WKUP();
        fn RADIO_TXRX_SEQ();
        fn TIM16();
        fn TIM17();
        fn RNG();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 29] = [
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: PVD },
        Vector { _handler: I2C1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI3 },
        Vector { _handler: USART1 },
        Vector { _handler: LPUART1 },
        Vector { _handler: TIM2 },
        Vector { _handler: RTC },
        Vector { _handler: ADC },
        Vector { _handler: PKA },
        Vector { _handler: UPCONV },
        Vector { _handler: GPIOA },
        Vector { _handler: GPIOB },
        Vector { _handler: DMA },
        Vector { _handler: RADIO_TXRX },
        Vector { _reserved: 0 },
        Vector {
            _handler: RADIO_TIMER_ERROR,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: RADIO_TIMER_CPU_WKUP,
        },
        Vector {
            _handler: RADIO_TIMER_TXRX_WKUP,
        },
        Vector {
            _handler: RADIO_TXRX_SEQ,
        },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: RNG },
    ];
}
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4000_0000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4000_1000usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_2000usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_4000usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4000_5000usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4000_6000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4100_0000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4100_4000usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4100_5000usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4100_6000usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4100_7000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4810_0000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4820_0000usize as _) };
pub const PKA: pka::Pka = unsafe { pka::Pka::from_ptr(0x4830_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4840_0000usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4850_0000usize as _) };
pub const RNG: trng::Trng = unsafe { trng::Trng::from_ptr(0x4860_0000usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4870_0000usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4880_0000usize as _) };
pub const RADIO: radio::Radio = unsafe { radio::Radio::from_ptr(0x6000_0000usize as _) };
pub const RRM: rrm::Rrm = unsafe { rrm::Rrm::from_ptr(0x6000_1400usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_wb0.rs"]
pub mod adc;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../../peripherals/flash_wb0.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v1.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/pka_v1c.rs"]
pub mod pka;
#[path = "../../peripherals/pwr_wb0.rs"]
pub mod pwr;
#[path = "../../peripherals/radio_v1.rs"]
pub mod radio;
#[path = "../../peripherals/rcc_wb0.rs"]
pub mod rcc;
#[path = "../../peripherals/rrm_v1.rs"]
pub mod rrm;
#[path = "../../peripherals/rtc_v3_base.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v3.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_wb0.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/trng_v1.rs"]
pub mod trng;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
