#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - TZIC_ILA"]
    TZIC_ILA = 0,
    #[doc = "1 - PVD_PVM"]
    PVD_PVM = 1,
    #[doc = "2 - RTC_LSECSS"]
    RTC_LSECSS = 2,
    #[doc = "3 - RCC_FLASH_C1SEV"]
    RCC_FLASH_C1SEV = 3,
    #[doc = "4 - EXTI1_0"]
    EXTI1_0 = 4,
    #[doc = "5 - EXTI3_2"]
    EXTI3_2 = 5,
    #[doc = "6 - EXTI15_4"]
    EXTI15_4 = 6,
    #[doc = "7 - ADC_COMP_DAC"]
    ADC_COMP_DAC = 7,
    #[doc = "8 - DMA1_CHANNEL1_2_3"]
    DMA1_CHANNEL1_2_3 = 8,
    #[doc = "9 - DMA1_CHANNEL4_5_6_7"]
    DMA1_CHANNEL4_5_6_7 = 9,
    #[doc = "10 - DMA2_DMAMUX1_OVR"]
    DMA2_DMAMUX1_OVR = 10,
    #[doc = "11 - LPTIM1"]
    LPTIM1 = 11,
    #[doc = "12 - LPTIM2"]
    LPTIM2 = 12,
    #[doc = "13 - LPTIM3"]
    LPTIM3 = 13,
    #[doc = "14 - TIM1"]
    TIM1 = 14,
    #[doc = "15 - TIM2"]
    TIM2 = 15,
    #[doc = "16 - TIM16"]
    TIM16 = 16,
    #[doc = "17 - TIM17"]
    TIM17 = 17,
    #[doc = "18 - IPCC_C2_RX_C2_TX"]
    IPCC_C2_RX_C2_TX = 18,
    #[doc = "19 - HSEM"]
    HSEM = 19,
    #[doc = "20 - RNG"]
    RNG = 20,
    #[doc = "21 - AES_PKA"]
    AES_PKA = 21,
    #[doc = "22 - I2C1"]
    I2C1 = 22,
    #[doc = "23 - I2C2"]
    I2C2 = 23,
    #[doc = "24 - I2C3"]
    I2C3 = 24,
    #[doc = "25 - SPI1"]
    SPI1 = 25,
    #[doc = "26 - SPI2"]
    SPI2 = 26,
    #[doc = "27 - USART1"]
    USART1 = 27,
    #[doc = "28 - USART2"]
    USART2 = 28,
    #[doc = "29 - LPUART1"]
    LPUART1 = 29,
    #[doc = "30 - SUBGHZSPI"]
    SUBGHZSPI = 30,
    #[doc = "31 - SUBGHZ_RADIO"]
    SUBGHZ_RADIO = 31,
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
        fn TZIC_ILA();
        fn PVD_PVM();
        fn RTC_LSECSS();
        fn RCC_FLASH_C1SEV();
        fn EXTI1_0();
        fn EXTI3_2();
        fn EXTI15_4();
        fn ADC_COMP_DAC();
        fn DMA1_CHANNEL1_2_3();
        fn DMA1_CHANNEL4_5_6_7();
        fn DMA2_DMAMUX1_OVR();
        fn LPTIM1();
        fn LPTIM2();
        fn LPTIM3();
        fn TIM1();
        fn TIM2();
        fn TIM16();
        fn TIM17();
        fn IPCC_C2_RX_C2_TX();
        fn HSEM();
        fn RNG();
        fn AES_PKA();
        fn I2C1();
        fn I2C2();
        fn I2C3();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn LPUART1();
        fn SUBGHZSPI();
        fn SUBGHZ_RADIO();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 32] = [
        Vector { _handler: TZIC_ILA },
        Vector { _handler: PVD_PVM },
        Vector { _handler: RTC_LSECSS },
        Vector {
            _handler: RCC_FLASH_C1SEV,
        },
        Vector { _handler: EXTI1_0 },
        Vector { _handler: EXTI3_2 },
        Vector { _handler: EXTI15_4 },
        Vector { _handler: ADC_COMP_DAC },
        Vector {
            _handler: DMA1_CHANNEL1_2_3,
        },
        Vector {
            _handler: DMA1_CHANNEL4_5_6_7,
        },
        Vector {
            _handler: DMA2_DMAMUX1_OVR,
        },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: LPTIM3 },
        Vector { _handler: TIM1 },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector {
            _handler: IPCC_C2_RX_C2_TX,
        },
        Vector { _handler: HSEM },
        Vector { _handler: RNG },
        Vector { _handler: AES_PKA },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2 },
        Vector { _handler: I2C3 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: LPUART1 },
        Vector { _handler: SUBGHZSPI },
        Vector { _handler: SUBGHZ_RADIO },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1fff_7590usize as _) };
pub const VREFINTCAL: vrefintcal::Vrefintcal = unsafe { vrefintcal::Vrefintcal::from_ptr(0x1fff_75aausize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_7c00usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4000_8000usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_9400usize as _) };
pub const LPTIM3: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_9800usize as _) };
pub const TAMP: tamp::Tamp = unsafe { tamp::Tamp::from_ptr(0x4000_b000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000usize as _) };
pub const VREFBUF: vrefbuf::Vrefbuf = unsafe { vrefbuf::Vrefbuf::from_ptr(0x4001_0030usize as _) };
pub const COMP1: comp::Comp = unsafe { comp::Comp::from_ptr(0x4001_0200usize as _) };
pub const COMP2: comp::Comp = unsafe { comp::Comp::from_ptr(0x4001_0204usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const ADC1_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4001_2708usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4800usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0400usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4002_0800usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1c00usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x5800_0000usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x5800_0400usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x5800_0800usize as _) };
pub const IPCC: ipcc::Ipcc = unsafe { ipcc::Ipcc::from_ptr(0x5800_0c00usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x5800_1000usize as _) };
pub const HSEM: hsem::Hsem = unsafe { hsem::Hsem::from_ptr(0x5800_1400usize as _) };
pub const AES: aes::Aes = unsafe { aes::Aes::from_ptr(0x5800_1800usize as _) };
pub const PKA: pka::Pka = unsafe { pka::Pka::from_ptr(0x5800_2000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x5800_4000usize as _) };
pub const SUBGHZSPI: spi::Spi = unsafe { spi::Spi::from_ptr(0x5801_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_g0.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_v3.rs"]
pub mod adccommon;
#[path = "../../peripherals/aes_v2.rs"]
pub mod aes;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/comp_v3.rs"]
pub mod comp;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dac_v4.rs"]
pub mod dac;
#[path = "../../peripherals/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../../peripherals/exti_w.rs"]
pub mod exti;
#[path = "../../peripherals/flash_wl.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/hsem_v3.rs"]
pub mod hsem;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/ipcc_v1.rs"]
pub mod ipcc;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/lptim_v1c.rs"]
pub mod lptim;
#[path = "../../peripherals/pka_v1c.rs"]
pub mod pka;
#[path = "../../peripherals/pwr_wl5.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_wl5.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v2.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v3_base.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v3_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_wl5.rs"]
pub mod syscfg;
#[path = "../../peripherals/tamp_wl.rs"]
pub mod tamp;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/vrefbuf_v1.rs"]
pub mod vrefbuf;
#[path = "../../peripherals/vrefintcal_v1.rs"]
pub mod vrefintcal;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
