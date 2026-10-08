#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD"]
    PVD = 1,
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
    #[doc = "27 - IWDG"]
    IWDG = 27,
    #[doc = "28 - SAES"]
    SAES = 28,
    #[doc = "29 - GPDMA1_CHANNEL0"]
    GPDMA1_CHANNEL0 = 29,
    #[doc = "30 - GPDMA1_CHANNEL1"]
    GPDMA1_CHANNEL1 = 30,
    #[doc = "31 - GPDMA1_CHANNEL2"]
    GPDMA1_CHANNEL2 = 31,
    #[doc = "32 - GPDMA1_CHANNEL3"]
    GPDMA1_CHANNEL3 = 32,
    #[doc = "33 - GPDMA1_CHANNEL4"]
    GPDMA1_CHANNEL4 = 33,
    #[doc = "34 - GPDMA1_CHANNEL5"]
    GPDMA1_CHANNEL5 = 34,
    #[doc = "35 - GPDMA1_CHANNEL6"]
    GPDMA1_CHANNEL6 = 35,
    #[doc = "36 - GPDMA1_CHANNEL7"]
    GPDMA1_CHANNEL7 = 36,
    #[doc = "37 - TIM1_BRK"]
    TIM1_BRK = 37,
    #[doc = "38 - TIM1_UP"]
    TIM1_UP = 38,
    #[doc = "39 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 39,
    #[doc = "40 - TIM1_CC"]
    TIM1_CC = 40,
    #[doc = "41 - TIM2"]
    TIM2 = 41,
    #[doc = "42 - TIM3"]
    TIM3 = 42,
    #[doc = "43 - I2C1_EV"]
    I2C1_EV = 43,
    #[doc = "44 - I2C1_ER"]
    I2C1_ER = 44,
    #[doc = "45 - SPI1"]
    SPI1 = 45,
    #[doc = "46 - USART1"]
    USART1 = 46,
    #[doc = "47 - USART2"]
    USART2 = 47,
    #[doc = "48 - LPUART1"]
    LPUART1 = 48,
    #[doc = "49 - LPTIM1"]
    LPTIM1 = 49,
    #[doc = "50 - LPTIM2"]
    LPTIM2 = 50,
    #[doc = "51 - TIM16"]
    TIM16 = 51,
    #[doc = "52 - TIM17"]
    TIM17 = 52,
    #[doc = "53 - COMP"]
    COMP = 53,
    #[doc = "54 - I2C3_EV"]
    I2C3_EV = 54,
    #[doc = "55 - I2C3_ER"]
    I2C3_ER = 55,
    #[doc = "56 - SAI1"]
    SAI1 = 56,
    #[doc = "57 - TSC"]
    TSC = 57,
    #[doc = "58 - AES"]
    AES = 58,
    #[doc = "59 - RNG"]
    RNG = 59,
    #[doc = "60 - FPU"]
    FPU = 60,
    #[doc = "61 - HASH"]
    HASH = 61,
    #[doc = "62 - PKA"]
    PKA = 62,
    #[doc = "63 - SPI3"]
    SPI3 = 63,
    #[doc = "64 - ICACHE"]
    ICACHE = 64,
    #[doc = "65 - ADC4"]
    ADC4 = 65,
    #[doc = "66 - RADIO"]
    RADIO = 66,
    #[doc = "67 - WKUP"]
    WKUP = 67,
    #[doc = "68 - HSEM"]
    HSEM = 68,
    #[doc = "69 - HSEM_S"]
    HSEM_S = 69,
    #[doc = "70 - WKUP_S"]
    WKUP_S = 70,
    #[doc = "71 - RCC_AUDIOSYNC"]
    RCC_AUDIOSYNC = 71,
    #[doc = "80 - EXTI19_RADIO_IO"]
    EXTI19_RADIO_IO = 80,
    #[doc = "81 - EXTI20_RADIO_IO"]
    EXTI20_RADIO_IO = 81,
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
        fn IWDG();
        fn SAES();
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
        fn I2C1_EV();
        fn I2C1_ER();
        fn SPI1();
        fn USART1();
        fn USART2();
        fn LPUART1();
        fn LPTIM1();
        fn LPTIM2();
        fn TIM16();
        fn TIM17();
        fn COMP();
        fn I2C3_EV();
        fn I2C3_ER();
        fn SAI1();
        fn TSC();
        fn AES();
        fn RNG();
        fn FPU();
        fn HASH();
        fn PKA();
        fn SPI3();
        fn ICACHE();
        fn ADC4();
        fn RADIO();
        fn WKUP();
        fn HSEM();
        fn HSEM_S();
        fn WKUP_S();
        fn RCC_AUDIOSYNC();
        fn EXTI19_RADIO_IO();
        fn EXTI20_RADIO_IO();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 82] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD },
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
        Vector { _handler: IWDG },
        Vector { _handler: SAES },
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
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: COMP },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: SAI1 },
        Vector { _handler: TSC },
        Vector { _handler: AES },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
        Vector { _handler: HASH },
        Vector { _handler: PKA },
        Vector { _handler: SPI3 },
        Vector { _handler: ICACHE },
        Vector { _handler: ADC4 },
        Vector { _handler: RADIO },
        Vector { _handler: WKUP },
        Vector { _handler: HSEM },
        Vector { _handler: HSEM_S },
        Vector { _handler: WKUP_S },
        Vector {
            _handler: RCC_AUDIOSYNC,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: EXTI19_RADIO_IO,
        },
        Vector {
            _handler: EXTI20_RADIO_IO,
        },
    ];
}
pub const DESIG: desig::Desig = unsafe { desig::Desig::from_ptr(0x0bfa_0500usize as _) };
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x0bfa_0700usize as _) };
pub const VREFINTCAL: vrefintcal::Vrefintcal = unsafe { vrefintcal::Vrefintcal::from_ptr(0x0bfa_07a4usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_9400usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4800usize as _) };
pub const SAI1: sai::Sai = unsafe { sai::Sai::from_ptr(0x4001_5400usize as _) };
pub const GPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_0000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const TSC: tsc::Tsc = unsafe { tsc::Tsc::from_ptr(0x4002_4000usize as _) };
pub const RAMCFG: ramcfg::Ramcfg = unsafe { ramcfg::Ramcfg::from_ptr(0x4002_6000usize as _) };
pub const ICACHE: icache::Icache = unsafe { icache::Icache::from_ptr(0x4003_0400usize as _) };
pub const GTZC: gtzc::GtzcTzsc = unsafe { gtzc::GtzcTzsc::from_ptr(0x4003_2400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1c00usize as _) };
pub const AES: aes::Aes = unsafe { aes::Aes::from_ptr(0x420c_0000usize as _) };
pub const HASH: hash::Hash = unsafe { hash::Hash::from_ptr(0x420c_0400usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x420c_0800usize as _) };
pub const SAES: saes::Saes = unsafe { saes::Saes::from_ptr(0x420c_0c00usize as _) };
pub const HSEM: *mut () = 0x420c_1c00usize as _;
pub const PKA: pka::Pka = unsafe { pka::Pka::from_ptr(0x420c_2000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4600_0400usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4600_2000usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4600_2400usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4600_2800usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4600_4400usize as _) };
pub const COMP1: comp::Comp = unsafe { comp::Comp::from_ptr(0x4600_5400usize as _) };
pub const COMP2: comp::Comp = unsafe { comp::Comp::from_ptr(0x4600_5404usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4600_7800usize as _) };
pub const TAMP: tamp::Tamp = unsafe { tamp::Tamp::from_ptr(0x4600_7c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4602_0800usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4602_0c00usize as _) };
pub const ADC4: adc::Adc4 = unsafe { adc::Adc4::from_ptr(0x4602_1000usize as _) };
pub const ADC4_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4602_1308usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4602_2000usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0xe004_4000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_wba.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_v3.rs"]
pub mod adccommon;
#[path = "../../peripherals/aes_v3b.rs"]
pub mod aes;
#[path = "../../peripherals/comp_u5.rs"]
pub mod comp;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dbgmcu_wba.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/desig_wba.rs"]
pub mod desig;
#[path = "../../peripherals/exti_l5.rs"]
pub mod exti;
#[path = "../../peripherals/flash_wba.rs"]
pub mod flash;
#[path = "../../peripherals/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/gtzc_wba.rs"]
pub mod gtzc;
#[path = "../../peripherals/hash_v4.rs"]
pub mod hash;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/icache_v1_4crr.rs"]
pub mod icache;
#[path = "../../peripherals/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../../peripherals/lptim_v2a.rs"]
pub mod lptim;
#[path = "../../peripherals/pka_v1a.rs"]
pub mod pka;
#[path = "../../peripherals/pwr_wba.rs"]
pub mod pwr;
#[path = "../../peripherals/ramcfg_wba.rs"]
pub mod ramcfg;
#[path = "../../peripherals/rcc_wba.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_wba6.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v3_u5.rs"]
pub mod rtc;
#[path = "../../peripherals/saes_v1a.rs"]
pub mod saes;
#[path = "../../peripherals/sai_v4_2pdm.rs"]
pub mod sai;
#[path = "../../peripherals/spi_v6.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_wba.rs"]
pub mod syscfg;
#[path = "../../peripherals/tamp_wba.rs"]
pub mod tamp;
#[path = "../../peripherals/timer_v2.rs"]
pub mod timer;
#[path = "../../peripherals/tsc_v1.rs"]
pub mod tsc;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/vrefintcal_v2.rs"]
pub mod vrefintcal;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
