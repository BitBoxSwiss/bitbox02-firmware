#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Memory mapping selection bits"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> super::vals::MemMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::MemMode::from_bits(val as u8)
        }
        #[doc = "Memory mapping selection bits"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: super::vals::MemMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PA11 and PA12 remapping bit for small packages (28 and 20 pins) 0: Pin pair PA9/PA10 mapped on the pins 1: Pin pair PA11/PA12 mapped instead of PA9/PA10"]
        #[inline(always)]
        pub const fn pa11_pa12_rmp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA11 and PA12 remapping bit for small packages (28 and 20 pins) 0: Pin pair PA9/PA10 mapped on the pins 1: Pin pair PA11/PA12 mapped instead of PA9/PA10"]
        #[inline(always)]
        pub fn set_pa11_pa12_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IR Modulation Envelope signal selection"]
        #[inline(always)]
        pub const fn ir_mod(&self) -> super::vals::IrMod {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::IrMod::from_bits(val as u8)
        }
        #[doc = "IR Modulation Envelope signal selection"]
        #[inline(always)]
        pub fn set_ir_mod(&mut self, val: super::vals::IrMod) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "ADC DMA remapping bit 0: ADC DMA request mapped on DMA channel 1 1: ADC DMA request mapped on DMA channel 2"]
        #[inline(always)]
        pub const fn adc_dma_rmp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC DMA remapping bit 0: ADC DMA request mapped on DMA channel 1 1: ADC DMA request mapped on DMA channel 2"]
        #[inline(always)]
        pub fn set_adc_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USART1_TX DMA remapping bit 0: USART1_TX DMA request mapped on DMA channel 2 1: USART1_TX DMA request mapped on DMA channel 4"]
        #[inline(always)]
        pub const fn usart1_tx_dma_rmp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "USART1_TX DMA remapping bit 0: USART1_TX DMA request mapped on DMA channel 2 1: USART1_TX DMA request mapped on DMA channel 4"]
        #[inline(always)]
        pub fn set_usart1_tx_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "USART1_RX DMA request remapping bit 0: USART1_RX DMA request mapped on DMA channel 3 1: USART1_RX DMA request mapped on DMA channel 5"]
        #[inline(always)]
        pub const fn usart1_rx_dma_rmp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "USART1_RX DMA request remapping bit 0: USART1_RX DMA request mapped on DMA channel 3 1: USART1_RX DMA request mapped on DMA channel 5"]
        #[inline(always)]
        pub fn set_usart1_rx_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM16 DMA request remapping bit 0: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3 1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4"]
        #[inline(always)]
        pub const fn tim16_dma_rmp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 DMA request remapping bit 0: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3 1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4"]
        #[inline(always)]
        pub fn set_tim16_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM17 DMA request remapping bit 0: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1 1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2"]
        #[inline(always)]
        pub const fn tim17_dma_rmp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 DMA request remapping bit 0: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1 1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2"]
        #[inline(always)]
        pub fn set_tim17_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM16 alternate DMA request remapping bit 0: TIM16 DMA request mapped according to TIM16_DMA_RMP bit 1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6"]
        #[inline(always)]
        pub const fn tim16_dma_rmp2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 alternate DMA request remapping bit 0: TIM16 DMA request mapped according to TIM16_DMA_RMP bit 1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6"]
        #[inline(always)]
        pub fn set_tim16_dma_rmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM17 alternate DMA request remapping bit 0: TIM17 DMA request mapped according to TIM16_DMA_RMP bit 1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7"]
        #[inline(always)]
        pub const fn tim17_dma_rmp2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 alternate DMA request remapping bit 0: TIM17 DMA request mapped according to TIM16_DMA_RMP bit 1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7"]
        #[inline(always)]
        pub fn set_tim17_dma_rmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Fast Mode Plus (FM plus) driving capability activation bits. 0: PB6 pin operate in standard mode 1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM plus) driving capability activation bits. 0: PB6 pin operate in standard mode 1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits. 0: PB7 pin operate in standard mode 1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits. 0: PB7 pin operate in standard mode 1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits. 0: PB8 pin operate in standard mode 1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits. 0: PB8 pin operate in standard mode 1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits. 0: PB9 pin operate in standard mode 1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits. 0: PB9 pin operate in standard mode 1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "FM+ driving capability activation for I2C1 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "FM+ driving capability activation for I2C1 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "FM+ driving capability activation for I2C2 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "FM+ driving capability activation for I2C2 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits 0: PA9 pin operate in standard mode 1: I2C FM+ mode enabled on PA9 and the Speed control is bypassed"]
        #[inline(always)]
        pub const fn i2c_pa9_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits 0: PA9 pin operate in standard mode 1: I2C FM+ mode enabled on PA9 and the Speed control is bypassed"]
        #[inline(always)]
        pub fn set_i2c_pa9_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits 0: PA10 pin operate in standard mode 1: I2C FM+ mode enabled on PA10 and the Speed control is bypassed"]
        #[inline(always)]
        pub const fn i2c_pa10_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits 0: PA10 pin operate in standard mode 1: I2C FM+ mode enabled on PA10 and the Speed control is bypassed"]
        #[inline(always)]
        pub fn set_i2c_pa10_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "SPI2 DMA request remapping bit 0: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively 1: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
        #[inline(always)]
        pub const fn spi2_dma_rmp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 DMA request remapping bit 0: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively 1: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
        #[inline(always)]
        pub fn set_spi2_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART2 DMA request remapping bit 0: USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively 1: USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
        #[inline(always)]
        pub const fn usart2_dma_rmp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 DMA request remapping bit 0: USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively 1: USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
        #[inline(always)]
        pub fn set_usart2_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USART3 DMA request remapping bit 0: USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0) 1: USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
        #[inline(always)]
        pub const fn usart3_dma_rmp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 DMA request remapping bit 0: USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0) 1: USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
        #[inline(always)]
        pub fn set_usart3_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "I2C1 DMA request remapping bit 0: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively 1: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively"]
        #[inline(always)]
        pub const fn i2c1_dma_rmp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 DMA request remapping bit 0: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively 1: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively"]
        #[inline(always)]
        pub fn set_i2c1_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "TIM1 DMA request remapping bit 0: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively 1: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6"]
        #[inline(always)]
        pub const fn tim1_dma_rmp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 DMA request remapping bit 0: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively 1: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6"]
        #[inline(always)]
        pub fn set_tim1_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "TIM2 DMA request remapping bit 0: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively 1: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7"]
        #[inline(always)]
        pub const fn tim2_dma_rmp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 DMA request remapping bit 0: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively 1: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7"]
        #[inline(always)]
        pub fn set_tim2_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "TIM3 DMA request remapping bit 0: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4 1: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6"]
        #[inline(always)]
        pub const fn tim3_dma_rmp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 DMA request remapping bit 0: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4 1: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6"]
        #[inline(always)]
        pub fn set_tim3_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("mem_mode", &self.mem_mode())
                .field("pa11_pa12_rmp", &self.pa11_pa12_rmp())
                .field("ir_mod", &self.ir_mod())
                .field("adc_dma_rmp", &self.adc_dma_rmp())
                .field("usart1_tx_dma_rmp", &self.usart1_tx_dma_rmp())
                .field("usart1_rx_dma_rmp", &self.usart1_rx_dma_rmp())
                .field("tim16_dma_rmp", &self.tim16_dma_rmp())
                .field("tim17_dma_rmp", &self.tim17_dma_rmp())
                .field("tim16_dma_rmp2", &self.tim16_dma_rmp2())
                .field("tim17_dma_rmp2", &self.tim17_dma_rmp2())
                .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
                .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
                .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
                .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
                .field("i2c1_fmp", &self.i2c1_fmp())
                .field("i2c2_fmp", &self.i2c2_fmp())
                .field("i2c_pa9_fmp", &self.i2c_pa9_fmp())
                .field("i2c_pa10_fmp", &self.i2c_pa10_fmp())
                .field("spi2_dma_rmp", &self.spi2_dma_rmp())
                .field("usart2_dma_rmp", &self.usart2_dma_rmp())
                .field("usart3_dma_rmp", &self.usart3_dma_rmp())
                .field("i2c1_dma_rmp", &self.i2c1_dma_rmp())
                .field("tim1_dma_rmp", &self.tim1_dma_rmp())
                .field("tim2_dma_rmp", &self.tim2_dma_rmp())
                .field("tim3_dma_rmp", &self.tim3_dma_rmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ mem_mode: {:?}, pa11_pa12_rmp: {=bool:?}, ir_mod: {:?}, adc_dma_rmp: {=bool:?}, usart1_tx_dma_rmp: {=bool:?}, usart1_rx_dma_rmp: {=bool:?}, tim16_dma_rmp: {=bool:?}, tim17_dma_rmp: {=bool:?}, tim16_dma_rmp2: {=bool:?}, tim17_dma_rmp2: {=bool:?}, i2c_pb6_fmp: {:?}, i2c_pb7_fmp: {:?}, i2c_pb8_fmp: {:?}, i2c_pb9_fmp: {:?}, i2c1_fmp: {:?}, i2c2_fmp: {:?}, i2c_pa9_fmp: {:?}, i2c_pa10_fmp: {:?}, spi2_dma_rmp: {=bool:?}, usart2_dma_rmp: {=bool:?}, usart3_dma_rmp: {=bool:?}, i2c1_dma_rmp: {=bool:?}, tim1_dma_rmp: {=bool:?}, tim2_dma_rmp: {=bool:?}, tim3_dma_rmp: {=bool:?} }}" , self . mem_mode () , self . pa11_pa12_rmp () , self . ir_mod () , self . adc_dma_rmp () , self . usart1_tx_dma_rmp () , self . usart1_rx_dma_rmp () , self . tim16_dma_rmp () , self . tim17_dma_rmp () , self . tim16_dma_rmp2 () , self . tim17_dma_rmp2 () , self . i2c_pb6_fmp () , self . i2c_pb7_fmp () , self . i2c_pb8_fmp () , self . i2c_pb9_fmp () , self . i2c1_fmp () , self . i2c2_fmp () , self . i2c_pa9_fmp () , self . i2c_pa10_fmp () , self . spi2_dma_rmp () , self . usart2_dma_rmp () , self . usart3_dma_rmp () , self . i2c1_dma_rmp () , self . tim1_dma_rmp () , self . tim2_dma_rmp () , self . tim3_dma_rmp ())
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex-M0 LOCKUP bit enable bit"]
        #[inline(always)]
        pub const fn lockup_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex-M0 LOCKUP bit enable bit"]
        #[inline(always)]
        pub fn set_lockup_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM parity lock bit"]
        #[inline(always)]
        pub const fn sram_parity_lock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM parity lock bit"]
        #[inline(always)]
        pub fn set_sram_parity_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub const fn pvd_lock(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub fn set_pvd_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SRAM parity flag"]
        #[inline(always)]
        pub const fn sram_pef(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM parity flag"]
        #[inline(always)]
        pub fn set_sram_pef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("lockup_lock", &self.lockup_lock())
                .field("sram_parity_lock", &self.sram_parity_lock())
                .field("pvd_lock", &self.pvd_lock())
                .field("sram_pef", &self.sram_pef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr2 {{ lockup_lock: {=bool:?}, sram_parity_lock: {=bool:?}, pvd_lock: {=bool:?}, sram_pef: {=bool:?} }}" , self . lockup_lock () , self . sram_parity_lock () , self . pvd_lock () , self . sram_pef ())
        }
    }
    #[doc = "external interrupt configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI configuration bits"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI configuration bits"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fmp {
        #[doc = "Standard"]
        STANDARD = 0x0,
        #[doc = "FM+"]
        FMP = 0x01,
    }
    impl Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmp {
        #[inline(always)]
        fn from(val: u8) -> Fmp {
            Fmp::from_bits(val)
        }
    }
    impl From<Fmp> for u8 {
        #[inline(always)]
        fn from(val: Fmp) -> u8 {
            Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IrMod {
        #[doc = "TIM16 selected"]
        TIM16 = 0x0,
        #[doc = "USART1 selected"]
        USART1 = 0x01,
        #[doc = "USART4 selected"]
        USART4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl IrMod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IrMod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IrMod {
        #[inline(always)]
        fn from(val: u8) -> IrMod {
            IrMod::from_bits(val)
        }
    }
    impl From<IrMod> for u8 {
        #[inline(always)]
        fn from(val: IrMod) -> u8 {
            IrMod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MemMode {
        #[doc = "Main Flash memory mapped at 0x0000_0000"]
        MAIN_FLASH = 0x0,
        #[doc = "System Flash memory mapped at 0x0000_0000"]
        SYSTEM_FLASH = 0x01,
        #[doc = "Main Flash memory mapped at 0x0000_0000"]
        MAIN_FLASH2 = 0x02,
        #[doc = "Embedded SRAM mapped at 0x0000_0000"]
        SRAM = 0x03,
    }
    impl MemMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemMode {
        #[inline(always)]
        fn from(val: u8) -> MemMode {
            MemMode::from_bits(val)
        }
    }
    impl From<MemMode> for u8 {
        #[inline(always)]
        fn from(val: MemMode) -> u8 {
            MemMode::to_bits(val)
        }
    }
}
