#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD_PVM"]
    PVD_PVM = 1,
    #[doc = "2 - TAMP_STAMP_LSECSS"]
    TAMP_STAMP_LSECSS = 2,
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
    #[doc = "11 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 11,
    #[doc = "12 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 12,
    #[doc = "13 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 13,
    #[doc = "14 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 14,
    #[doc = "15 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 15,
    #[doc = "16 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 16,
    #[doc = "17 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 17,
    #[doc = "18 - ADC1"]
    ADC1 = 18,
    #[doc = "21 - C2SEV_PWR_C2H"]
    C2SEV_PWR_C2H = 21,
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
    #[doc = "29 - PKA"]
    PKA = 29,
    #[doc = "30 - I2C1_EV"]
    I2C1_EV = 30,
    #[doc = "31 - I2C1_ER"]
    I2C1_ER = 31,
    #[doc = "34 - SPI1"]
    SPI1 = 34,
    #[doc = "36 - USART1"]
    USART1 = 36,
    #[doc = "39 - TSC"]
    TSC = 39,
    #[doc = "40 - EXTI15_10"]
    EXTI15_10 = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "43 - PWR_SOTF_BLEACT_RFPHASE"]
    PWR_SOTF_BLEACT_RFPHASE = 43,
    #[doc = "44 - IPCC_C1_RX"]
    IPCC_C1_RX = 44,
    #[doc = "45 - IPCC_C1_TX"]
    IPCC_C1_TX = 45,
    #[doc = "46 - HSEM"]
    HSEM = 46,
    #[doc = "47 - LPTIM1"]
    LPTIM1 = 47,
    #[doc = "48 - LPTIM2"]
    LPTIM2 = 48,
    #[doc = "52 - AES2"]
    AES2 = 52,
    #[doc = "53 - RNG"]
    RNG = 53,
    #[doc = "54 - FPU"]
    FPU = 54,
    #[doc = "62 - DMAMUX1_OVR"]
    DMAMUX1_OVR = 62,
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
        fn PVD_PVM();
        fn TAMP_STAMP_LSECSS();
        fn RTC_WKUP();
        fn FLASH();
        fn RCC();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn ADC1();
        fn C2SEV_PWR_C2H();
        fn EXTI9_5();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn PKA();
        fn I2C1_EV();
        fn I2C1_ER();
        fn SPI1();
        fn USART1();
        fn TSC();
        fn EXTI15_10();
        fn RTC_ALARM();
        fn PWR_SOTF_BLEACT_RFPHASE();
        fn IPCC_C1_RX();
        fn IPCC_C1_TX();
        fn HSEM();
        fn LPTIM1();
        fn LPTIM2();
        fn AES2();
        fn RNG();
        fn FPU();
        fn DMAMUX1_OVR();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 63] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD_PVM },
        Vector {
            _handler: TAMP_STAMP_LSECSS,
        },
        Vector { _handler: RTC_WKUP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2,
        },
        Vector {
            _handler: DMA1_CHANNEL3,
        },
        Vector {
            _handler: DMA1_CHANNEL4,
        },
        Vector {
            _handler: DMA1_CHANNEL5,
        },
        Vector {
            _handler: DMA1_CHANNEL6,
        },
        Vector {
            _handler: DMA1_CHANNEL7,
        },
        Vector { _handler: ADC1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: C2SEV_PWR_C2H,
        },
        Vector { _reserved: 0 },
        Vector { _handler: EXTI9_5 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: PKA },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI1 },
        Vector { _reserved: 0 },
        Vector { _handler: USART1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TSC },
        Vector { _handler: EXTI15_10 },
        Vector { _handler: RTC_ALARM },
        Vector { _reserved: 0 },
        Vector {
            _handler: PWR_SOTF_BLEACT_RFPHASE,
        },
        Vector { _handler: IPCC_C1_RX },
        Vector { _handler: IPCC_C1_TX },
        Vector { _handler: HSEM },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: AES2 },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMAMUX1_OVR },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1fff_7590usize as _) };
pub const VREFINTCAL: vrefintcal::Vrefintcal = unsafe { vrefintcal::Vrefintcal::from_ptr(0x1fff_75aausize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_7c00usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_9400usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000usize as _) };
pub const ADC1: *mut () = 0x4001_2400usize as _;
pub const ADC1_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4001_2700usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4002_0800usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const TSC: *mut () = 0x4002_4000usize as _;
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0800usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1000usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1c00usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x5800_0000usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x5800_0400usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x5800_0800usize as _) };
pub const IPCC: ipcc::Ipcc = unsafe { ipcc::Ipcc::from_ptr(0x5800_0c00usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x5800_1000usize as _) };
pub const HSEM: *mut () = 0x5800_1400usize as _;
pub const AES2: *mut () = 0x5800_1800usize as _;
pub const PKA: pka::Pka = unsafe { pka::Pka::from_ptr(0x5800_2000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x5800_4000usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adccommon_v3.rs"]
pub mod adccommon;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dbgmcu_wb.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../../peripherals/exti_w.rs"]
pub mod exti;
#[path = "../../peripherals/flash_wb.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/ipcc_v1.rs"]
pub mod ipcc;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/lptim_v1b.rs"]
pub mod lptim;
#[path = "../../peripherals/pka_v1c.rs"]
pub mod pka;
#[path = "../../peripherals/pwr_wb.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_wb.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v1.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v2_wb.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v3.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_wb.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/vrefintcal_v1.rs"]
pub mod vrefintcal;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
