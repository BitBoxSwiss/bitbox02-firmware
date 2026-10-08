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
    #[doc = "CCM SRAM protection register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register"]
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
    #[doc = "configuration register 4"]
    #[inline(always)]
    pub const fn cfgr4(self) -> crate::common::Reg<regs::Cfgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
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
        #[doc = "USB interrupt remap 0: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively 1: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
        #[inline(always)]
        pub const fn usb_it_rmp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB interrupt remap 0: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively 1: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
        #[inline(always)]
        pub fn set_usb_it_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer 1 ITR3 selection 0: Not remapped 1: TIM1_ITR3 = TIM17_OC"]
        #[inline(always)]
        pub const fn tim1_itr3_rmp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 ITR3 selection 0: Not remapped 1: TIM1_ITR3 = TIM17_OC"]
        #[inline(always)]
        pub fn set_tim1_itr3_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DAC trigger remap (when TSEL = 001) 0: DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices 1: DAC trigger is TIM3_TRGO"]
        #[inline(always)]
        pub const fn dac1_trig_rmp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DAC trigger remap (when TSEL = 001) 0: DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices 1: DAC trigger is TIM3_TRGO"]
        #[inline(always)]
        pub fn set_dac1_trig_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DAC trigger remap (when TSEL = 001) 0: Not remapped 1: DAC trigger is TIM3_TRGO"]
        #[inline(always)]
        pub const fn dac_trig_rmp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DAC trigger remap (when TSEL = 001) 0: Not remapped 1: DAC trigger is TIM3_TRGO"]
        #[inline(always)]
        pub fn set_dac_trig_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC24 DMA remapping bit 0: ADC24 DMA requests mapped on DMA2 channels 1 and 2 1: ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
        #[inline(always)]
        pub const fn adc2_dma_rmp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC24 DMA remapping bit 0: ADC24 DMA requests mapped on DMA2 channels 1 and 2 1: ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
        #[inline(always)]
        pub fn set_adc2_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM16 DMA request remapping bit 0: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3 1: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
        #[inline(always)]
        pub const fn tim16_dma_rmp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 DMA request remapping bit 0: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3 1: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
        #[inline(always)]
        pub fn set_tim16_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM17 DMA request remapping bit 0: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1 1: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
        #[inline(always)]
        pub const fn tim17_dma_rmp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 DMA request remapping bit 0: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1 1: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
        #[inline(always)]
        pub fn set_tim17_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM6 and DAC1 DMA request remapping bit 0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
        #[inline(always)]
        pub const fn tim6_dac1_ch1_dma_rmp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 and DAC1 DMA request remapping bit 0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
        #[inline(always)]
        pub fn set_tim6_dac1_ch1_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM6 and DAC1 DMA request remapping bit 0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
        #[inline(always)]
        pub const fn tim6_dac1_dma_rmp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 and DAC1 DMA request remapping bit 0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
        #[inline(always)]
        pub fn set_tim6_dac1_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM6 and DAC1 DMA request remapping bit 0: TIM7 and DAC1_OUT1 DMA requests mapped on DMA2 channel 3 1: TIM7 and DAC1_OUT1 DMA requests mapped on DMA1 channel 3"]
        #[inline(always)]
        pub const fn tim6_dac1_out1_dma_rmp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 and DAC1 DMA request remapping bit 0: TIM7 and DAC1_OUT1 DMA requests mapped on DMA2 channel 3 1: TIM7 and DAC1_OUT1 DMA requests mapped on DMA1 channel 3"]
        #[inline(always)]
        pub fn set_tim6_dac1_out1_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM7 and DAC2 DMA request remapping bit 0: Not remapped 1: TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
        #[inline(always)]
        pub const fn tim7_dac1_ch2_dma_rmp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 and DAC2 DMA request remapping bit 0: Not remapped 1: TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
        #[inline(always)]
        pub fn set_tim7_dac1_ch2_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM7 and DAC2 DMA request remapping bit 0: TIM7 and DAC1_OUT2 DMA requests mapped on DMA2 channel 4 1: TIM7 and DAC1_OUT2 DMA requests mapped on DMA1 channel 4"]
        #[inline(always)]
        pub const fn tim7_dac1_out2_dma_rmp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 and DAC2 DMA request remapping bit 0: TIM7 and DAC1_OUT2 DMA requests mapped on DMA2 channel 4 1: TIM7 and DAC1_OUT2 DMA requests mapped on DMA1 channel 4"]
        #[inline(always)]
        pub fn set_tim7_dac1_out2_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DAC2 channel1 DMA remap 0: Not remapped 1: DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
        #[inline(always)]
        pub const fn dac2_ch1_dma_rmp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 channel1 DMA remap 0: Not remapped 1: DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
        #[inline(always)]
        pub fn set_dac2_ch1_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TIM18 and DAC2_OUT1 DMA request remapping bit 0: TIM18 and DAC2_OUT1 DMA requests mapped on DMA2 channel 5 1: TIM18 and DAC2_OUT1 DMA requests mapped on DMA1 channel 5"]
        #[inline(always)]
        pub const fn tim18_dac2_out1_dma_rmp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "TIM18 and DAC2_OUT1 DMA request remapping bit 0: TIM18 and DAC2_OUT1 DMA requests mapped on DMA2 channel 5 1: TIM18 and DAC2_OUT1 DMA requests mapped on DMA1 channel 5"]
        #[inline(always)]
        pub fn set_tim18_dac2_out1_dma_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits. 0: PB6 pin operate in standard mode 1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits. 0: PB6 pin operate in standard mode 1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
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
        #[doc = "I2C1 Fast Mode Plus 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "I2C1 Fast Mode Plus 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C2 Fast Mode Plus 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "I2C2 Fast Mode Plus 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Encoder mode"]
        #[inline(always)]
        pub const fn encoder_mode(&self) -> super::vals::EncoderMode {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::EncoderMode::from_bits(val as u8)
        }
        #[doc = "Encoder mode"]
        #[inline(always)]
        pub fn set_encoder_mode(&mut self, val: super::vals::EncoderMode) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "I2C3 Fast Mode Plus 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C3 pins selected through selection trhough IOPORT control registers AF selection bits"]
        #[inline(always)]
        pub const fn i2c3_fmp(&self) -> super::vals::Fmp {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Fmp::from_bits(val as u8)
        }
        #[doc = "I2C3 Fast Mode Plus 0: FM+ mode is controlled by I2C_Pxx_FMP bits only 1: FM+ mode is enabled on all I2C3 pins selected through selection trhough IOPORT control registers AF selection bits"]
        #[inline(always)]
        pub fn set_i2c3_fmp(&mut self, val: super::vals::Fmp) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Enable the power switch to deliver VBAT voltage on ADC channel 18 input"]
        #[inline(always)]
        pub const fn vbat_mon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the power switch to deliver VBAT voltage on ADC channel 18 input"]
        #[inline(always)]
        pub fn set_vbat_mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Idx 0: Invalid operation interrupt enable; Idx 1: Devide-by-zero interrupt enable; Idx 2: Underflow interrupt enable; Idx 3: Overflow interrupt enable; Idx 4: Input denormal interrupt enable; Idx 5: Inexact interrupt enable"]
        #[inline(always)]
        pub const fn fpu_ie(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 26usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Idx 0: Invalid operation interrupt enable; Idx 1: Devide-by-zero interrupt enable; Idx 2: Underflow interrupt enable; Idx 3: Overflow interrupt enable; Idx 4: Input denormal interrupt enable; Idx 5: Inexact interrupt enable"]
        #[inline(always)]
        pub fn set_fpu_ie(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 26usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("usb_it_rmp", &self.usb_it_rmp())
                .field("tim1_itr3_rmp", &self.tim1_itr3_rmp())
                .field("dac1_trig_rmp", &self.dac1_trig_rmp())
                .field("dac_trig_rmp", &self.dac_trig_rmp())
                .field("adc2_dma_rmp", &self.adc2_dma_rmp())
                .field("tim16_dma_rmp", &self.tim16_dma_rmp())
                .field("tim17_dma_rmp", &self.tim17_dma_rmp())
                .field("tim6_dac1_ch1_dma_rmp", &self.tim6_dac1_ch1_dma_rmp())
                .field("tim6_dac1_dma_rmp", &self.tim6_dac1_dma_rmp())
                .field("tim6_dac1_out1_dma_rmp", &self.tim6_dac1_out1_dma_rmp())
                .field("tim7_dac1_ch2_dma_rmp", &self.tim7_dac1_ch2_dma_rmp())
                .field("tim7_dac1_out2_dma_rmp", &self.tim7_dac1_out2_dma_rmp())
                .field("dac2_ch1_dma_rmp", &self.dac2_ch1_dma_rmp())
                .field("tim18_dac2_out1_dma_rmp", &self.tim18_dac2_out1_dma_rmp())
                .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
                .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
                .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
                .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
                .field("i2c1_fmp", &self.i2c1_fmp())
                .field("i2c2_fmp", &self.i2c2_fmp())
                .field("encoder_mode", &self.encoder_mode())
                .field("i2c3_fmp", &self.i2c3_fmp())
                .field("vbat_mon", &self.vbat_mon())
                .field("fpu_ie[0]", &self.fpu_ie(0usize))
                .field("fpu_ie[1]", &self.fpu_ie(1usize))
                .field("fpu_ie[2]", &self.fpu_ie(2usize))
                .field("fpu_ie[3]", &self.fpu_ie(3usize))
                .field("fpu_ie[4]", &self.fpu_ie(4usize))
                .field("fpu_ie[5]", &self.fpu_ie(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ mem_mode: {:?}, usb_it_rmp: {=bool:?}, tim1_itr3_rmp: {=bool:?}, dac1_trig_rmp: {=bool:?}, dac_trig_rmp: {=bool:?}, adc2_dma_rmp: {=bool:?}, tim16_dma_rmp: {=bool:?}, tim17_dma_rmp: {=bool:?}, tim6_dac1_ch1_dma_rmp: {=bool:?}, tim6_dac1_dma_rmp: {=bool:?}, tim6_dac1_out1_dma_rmp: {=bool:?}, tim7_dac1_ch2_dma_rmp: {=bool:?}, tim7_dac1_out2_dma_rmp: {=bool:?}, dac2_ch1_dma_rmp: {=bool:?}, tim18_dac2_out1_dma_rmp: {=bool:?}, i2c_pb6_fmp: {:?}, i2c_pb7_fmp: {:?}, i2c_pb8_fmp: {:?}, i2c_pb9_fmp: {:?}, i2c1_fmp: {:?}, i2c2_fmp: {:?}, encoder_mode: {:?}, i2c3_fmp: {:?}, vbat_mon: {=bool:?}, fpu_ie[0]: {=bool:?}, fpu_ie[1]: {=bool:?}, fpu_ie[2]: {=bool:?}, fpu_ie[3]: {=bool:?}, fpu_ie[4]: {=bool:?}, fpu_ie[5]: {=bool:?} }}" , self . mem_mode () , self . usb_it_rmp () , self . tim1_itr3_rmp () , self . dac1_trig_rmp () , self . dac_trig_rmp () , self . adc2_dma_rmp () , self . tim16_dma_rmp () , self . tim17_dma_rmp () , self . tim6_dac1_ch1_dma_rmp () , self . tim6_dac1_dma_rmp () , self . tim6_dac1_out1_dma_rmp () , self . tim7_dac1_ch2_dma_rmp () , self . tim7_dac1_out2_dma_rmp () , self . dac2_ch1_dma_rmp () , self . tim18_dac2_out1_dma_rmp () , self . i2c_pb6_fmp () , self . i2c_pb7_fmp () , self . i2c_pb8_fmp () , self . i2c_pb9_fmp () , self . i2c1_fmp () , self . i2c2_fmp () , self . encoder_mode () , self . i2c3_fmp () , self . vbat_mon () , self . fpu_ie (0usize) , self . fpu_ie (1usize) , self . fpu_ie (2usize) , self . fpu_ie (3usize) , self . fpu_ie (4usize) , self . fpu_ie (5usize))
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
        #[doc = "Bypass address bit 29 in parity calculation"]
        #[inline(always)]
        pub const fn byp_addr_par(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass address bit 29 in parity calculation"]
        #[inline(always)]
        pub fn set_byp_addr_par(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("byp_addr_par", &self.byp_addr_par())
                .field("sram_pef", &self.sram_pef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr2 {{ lockup_lock: {=bool:?}, sram_parity_lock: {=bool:?}, pvd_lock: {=bool:?}, byp_addr_par: {=bool:?}, sram_pef: {=bool:?} }}" , self . lockup_lock () , self . sram_parity_lock () , self . pvd_lock () , self . byp_addr_par () , self . sram_pef ())
        }
    }
    #[doc = "configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "SPI1_RX DMA remapping bit"]
        #[inline(always)]
        pub const fn spi1_rx_dma_rmp(&self) -> super::vals::Spi1RxDmaRmp {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Spi1RxDmaRmp::from_bits(val as u8)
        }
        #[doc = "SPI1_RX DMA remapping bit"]
        #[inline(always)]
        pub fn set_spi1_rx_dma_rmp(&mut self, val: super::vals::Spi1RxDmaRmp) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "SPI1_TX DMA remapping bit"]
        #[inline(always)]
        pub const fn spi1_tx_dma_rmp(&self) -> super::vals::Spi1TxDmaRmp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Spi1TxDmaRmp::from_bits(val as u8)
        }
        #[doc = "SPI1_TX DMA remapping bit"]
        #[inline(always)]
        pub fn set_spi1_tx_dma_rmp(&mut self, val: super::vals::Spi1TxDmaRmp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "I2C1_RX DMA remapping bit"]
        #[inline(always)]
        pub const fn i2c1_rx_dma_rmp(&self) -> super::vals::I2c1RxDmaRmp {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::I2c1RxDmaRmp::from_bits(val as u8)
        }
        #[doc = "I2C1_RX DMA remapping bit"]
        #[inline(always)]
        pub fn set_i2c1_rx_dma_rmp(&mut self, val: super::vals::I2c1RxDmaRmp) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "I2C1_TX DMA remapping bit"]
        #[inline(always)]
        pub const fn i2c1_tx_dma_rmp(&self) -> super::vals::I2c1TxDmaRmp {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::I2c1TxDmaRmp::from_bits(val as u8)
        }
        #[doc = "I2C1_TX DMA remapping bit"]
        #[inline(always)]
        pub fn set_i2c1_tx_dma_rmp(&mut self, val: super::vals::I2c1TxDmaRmp) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "ADC2 DMA remapping bit"]
        #[inline(always)]
        pub const fn adc2_dma_rmp(&self) -> super::vals::Adc2DmaRmpCfgr3 {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Adc2DmaRmpCfgr3::from_bits(val as u8)
        }
        #[doc = "ADC2 DMA remapping bit"]
        #[inline(always)]
        pub fn set_adc2_dma_rmp(&mut self, val: super::vals::Adc2DmaRmpCfgr3) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "DAC1_CH1 / DAC1_CH2 Trigger remap"]
        #[inline(always)]
        pub const fn dac1_trig3_rmp(&self) -> super::vals::Dac1Trig3Rmp {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Dac1Trig3Rmp::from_bits(val as u8)
        }
        #[doc = "DAC1_CH1 / DAC1_CH2 Trigger remap"]
        #[inline(always)]
        pub fn set_dac1_trig3_rmp(&mut self, val: super::vals::Dac1Trig3Rmp) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "DAC1_CH1 / DAC1_CH2 Trigger remap 0: Not remapped 1: DAC trigger is HRTIM1_DAC1_TRIG2"]
        #[inline(always)]
        pub const fn dac1_trig5_rmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1_CH1 / DAC1_CH2 Trigger remap 0: Not remapped 1: DAC trigger is HRTIM1_DAC1_TRIG2"]
        #[inline(always)]
        pub fn set_dac1_trig5_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Cfgr3 {
        #[inline(always)]
        fn default() -> Cfgr3 {
            Cfgr3(0)
        }
    }
    impl core::fmt::Debug for Cfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr3")
                .field("spi1_rx_dma_rmp", &self.spi1_rx_dma_rmp())
                .field("spi1_tx_dma_rmp", &self.spi1_tx_dma_rmp())
                .field("i2c1_rx_dma_rmp", &self.i2c1_rx_dma_rmp())
                .field("i2c1_tx_dma_rmp", &self.i2c1_tx_dma_rmp())
                .field("adc2_dma_rmp", &self.adc2_dma_rmp())
                .field("dac1_trig3_rmp", &self.dac1_trig3_rmp())
                .field("dac1_trig5_rmp", &self.dac1_trig5_rmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr3 {{ spi1_rx_dma_rmp: {:?}, spi1_tx_dma_rmp: {:?}, i2c1_rx_dma_rmp: {:?}, i2c1_tx_dma_rmp: {:?}, adc2_dma_rmp: {:?}, dac1_trig3_rmp: {:?}, dac1_trig5_rmp: {=bool:?} }}" , self . spi1_rx_dma_rmp () , self . spi1_tx_dma_rmp () , self . i2c1_rx_dma_rmp () , self . i2c1_tx_dma_rmp () , self . adc2_dma_rmp () , self . dac1_trig3_rmp () , self . dac1_trig5_rmp ())
        }
    }
    #[doc = "configuration register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr4(pub u32);
    impl Cfgr4 {
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT2"]
        #[inline(always)]
        pub const fn adc12_ext2_rmp(&self) -> super::vals::Adc12Ext2Rmp {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Adc12Ext2Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT2"]
        #[inline(always)]
        pub fn set_adc12_ext2_rmp(&mut self, val: super::vals::Adc12Ext2Rmp) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT3"]
        #[inline(always)]
        pub const fn adc12_ext3_rmp(&self) -> super::vals::Adc12Ext3Rmp {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Adc12Ext3Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT3"]
        #[inline(always)]
        pub fn set_adc12_ext3_rmp(&mut self, val: super::vals::Adc12Ext3Rmp) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT5"]
        #[inline(always)]
        pub const fn adc12_ext5_rmp(&self) -> super::vals::Adc12Ext5Rmp {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Adc12Ext5Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT5"]
        #[inline(always)]
        pub fn set_adc12_ext5_rmp(&mut self, val: super::vals::Adc12Ext5Rmp) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT13"]
        #[inline(always)]
        pub const fn adc12_ext13_rmp(&self) -> super::vals::Adc12Ext13Rmp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Adc12Ext13Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT13"]
        #[inline(always)]
        pub fn set_adc12_ext13_rmp(&mut self, val: super::vals::Adc12Ext13Rmp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT15"]
        #[inline(always)]
        pub const fn adc12_ext15_rmp(&self) -> super::vals::Adc12Ext15Rmp {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Adc12Ext15Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC12 regular channel EXT15"]
        #[inline(always)]
        pub fn set_adc12_ext15_rmp(&mut self, val: super::vals::Adc12Ext15Rmp) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Controls the Input trigger of ADC12 injected channel JEXT3"]
        #[inline(always)]
        pub const fn adc12_jext3_rmp(&self) -> super::vals::Adc12Jext3Rmp {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Adc12Jext3Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC12 injected channel JEXT3"]
        #[inline(always)]
        pub fn set_adc12_jext3_rmp(&mut self, val: super::vals::Adc12Jext3Rmp) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Controls the Input trigger of ADC12 injected channel JEXT6"]
        #[inline(always)]
        pub const fn adc12_jext6_rmp(&self) -> super::vals::Adc12Jext6Rmp {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Adc12Jext6Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC12 injected channel JEXT6"]
        #[inline(always)]
        pub fn set_adc12_jext6_rmp(&mut self, val: super::vals::Adc12Jext6Rmp) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Controls the Input trigger of ADC12 injected channel JEXT13"]
        #[inline(always)]
        pub const fn adc12_jext13_rmp(&self) -> super::vals::Adc12Jext13Rmp {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Adc12Jext13Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC12 injected channel JEXT13"]
        #[inline(always)]
        pub fn set_adc12_jext13_rmp(&mut self, val: super::vals::Adc12Jext13Rmp) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Controls the Input trigger of ADC34 regular channel EXT5"]
        #[inline(always)]
        pub const fn adc34_ext5_rmp(&self) -> super::vals::Adc34Ext5Rmp {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Adc34Ext5Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC34 regular channel EXT5"]
        #[inline(always)]
        pub fn set_adc34_ext5_rmp(&mut self, val: super::vals::Adc34Ext5Rmp) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Controls the Input trigger of ADC34 regular channel EXT6"]
        #[inline(always)]
        pub const fn adc34_ext6_rmp(&self) -> super::vals::Adc34Ext6Rmp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Adc34Ext6Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC34 regular channel EXT6"]
        #[inline(always)]
        pub fn set_adc34_ext6_rmp(&mut self, val: super::vals::Adc34Ext6Rmp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Controls the Input trigger of ADC34 regular channel EXT15"]
        #[inline(always)]
        pub const fn adc34_ext15_rmp(&self) -> super::vals::Adc34Ext15Rmp {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Adc34Ext15Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC34 regular channel EXT15"]
        #[inline(always)]
        pub fn set_adc34_ext15_rmp(&mut self, val: super::vals::Adc34Ext15Rmp) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Controls the Input trigger of ADC34 injected channel JEXT5"]
        #[inline(always)]
        pub const fn adc34_jext5_rmp(&self) -> super::vals::Adc34Jext5Rmp {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Adc34Jext5Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC34 injected channel JEXT5"]
        #[inline(always)]
        pub fn set_adc34_jext5_rmp(&mut self, val: super::vals::Adc34Jext5Rmp) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Controls the Input trigger of ADC34 injected channel JEXT11"]
        #[inline(always)]
        pub const fn adc34_jext11_rmp(&self) -> super::vals::Adc34Jext11Rmp {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Adc34Jext11Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC34 injected channel JEXT11"]
        #[inline(always)]
        pub fn set_adc34_jext11_rmp(&mut self, val: super::vals::Adc34Jext11Rmp) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Controls the Input trigger of ADC34 injected channel JEXT14"]
        #[inline(always)]
        pub const fn adc34_jext14_rmp(&self) -> super::vals::Adc34Jext14Rmp {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Adc34Jext14Rmp::from_bits(val as u8)
        }
        #[doc = "Controls the Input trigger of ADC34 injected channel JEXT14"]
        #[inline(always)]
        pub fn set_adc34_jext14_rmp(&mut self, val: super::vals::Adc34Jext14Rmp) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Cfgr4 {
        #[inline(always)]
        fn default() -> Cfgr4 {
            Cfgr4(0)
        }
    }
    impl core::fmt::Debug for Cfgr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr4")
                .field("adc12_ext2_rmp", &self.adc12_ext2_rmp())
                .field("adc12_ext3_rmp", &self.adc12_ext3_rmp())
                .field("adc12_ext5_rmp", &self.adc12_ext5_rmp())
                .field("adc12_ext13_rmp", &self.adc12_ext13_rmp())
                .field("adc12_ext15_rmp", &self.adc12_ext15_rmp())
                .field("adc12_jext3_rmp", &self.adc12_jext3_rmp())
                .field("adc12_jext6_rmp", &self.adc12_jext6_rmp())
                .field("adc12_jext13_rmp", &self.adc12_jext13_rmp())
                .field("adc34_ext5_rmp", &self.adc34_ext5_rmp())
                .field("adc34_ext6_rmp", &self.adc34_ext6_rmp())
                .field("adc34_ext15_rmp", &self.adc34_ext15_rmp())
                .field("adc34_jext5_rmp", &self.adc34_jext5_rmp())
                .field("adc34_jext11_rmp", &self.adc34_jext11_rmp())
                .field("adc34_jext14_rmp", &self.adc34_jext14_rmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr4 {{ adc12_ext2_rmp: {:?}, adc12_ext3_rmp: {:?}, adc12_ext5_rmp: {:?}, adc12_ext13_rmp: {:?}, adc12_ext15_rmp: {:?}, adc12_jext3_rmp: {:?}, adc12_jext6_rmp: {:?}, adc12_jext13_rmp: {:?}, adc34_ext5_rmp: {:?}, adc34_ext6_rmp: {:?}, adc34_ext15_rmp: {:?}, adc34_jext5_rmp: {:?}, adc34_jext11_rmp: {:?}, adc34_jext14_rmp: {:?} }}" , self . adc12_ext2_rmp () , self . adc12_ext3_rmp () , self . adc12_ext5_rmp () , self . adc12_ext13_rmp () , self . adc12_ext15_rmp () , self . adc12_jext3_rmp () , self . adc12_jext6_rmp () , self . adc12_jext13_rmp () , self . adc34_ext5_rmp () , self . adc34_ext6_rmp () , self . adc34_ext15_rmp () , self . adc34_jext5_rmp () , self . adc34_jext11_rmp () , self . adc34_jext14_rmp ())
        }
    }
    #[doc = "external interrupt configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI x configuration"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI x configuration"]
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
    #[doc = "CCM SRAM protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr(pub u32);
    impl Rcr {
        #[doc = "CCM SRAM page x write protection enabled"]
        #[inline(always)]
        pub const fn page_wp(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "CCM SRAM page x write protection enabled"]
        #[inline(always)]
        pub fn set_page_wp(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Rcr {
        #[inline(always)]
        fn default() -> Rcr {
            Rcr(0)
        }
    }
    impl core::fmt::Debug for Rcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcr")
                .field("page_wp[0]", &self.page_wp(0usize))
                .field("page_wp[1]", &self.page_wp(1usize))
                .field("page_wp[2]", &self.page_wp(2usize))
                .field("page_wp[3]", &self.page_wp(3usize))
                .field("page_wp[4]", &self.page_wp(4usize))
                .field("page_wp[5]", &self.page_wp(5usize))
                .field("page_wp[6]", &self.page_wp(6usize))
                .field("page_wp[7]", &self.page_wp(7usize))
                .field("page_wp[8]", &self.page_wp(8usize))
                .field("page_wp[9]", &self.page_wp(9usize))
                .field("page_wp[10]", &self.page_wp(10usize))
                .field("page_wp[11]", &self.page_wp(11usize))
                .field("page_wp[12]", &self.page_wp(12usize))
                .field("page_wp[13]", &self.page_wp(13usize))
                .field("page_wp[14]", &self.page_wp(14usize))
                .field("page_wp[15]", &self.page_wp(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rcr {{ page_wp[0]: {=bool:?}, page_wp[1]: {=bool:?}, page_wp[2]: {=bool:?}, page_wp[3]: {=bool:?}, page_wp[4]: {=bool:?}, page_wp[5]: {=bool:?}, page_wp[6]: {=bool:?}, page_wp[7]: {=bool:?}, page_wp[8]: {=bool:?}, page_wp[9]: {=bool:?}, page_wp[10]: {=bool:?}, page_wp[11]: {=bool:?}, page_wp[12]: {=bool:?}, page_wp[13]: {=bool:?}, page_wp[14]: {=bool:?}, page_wp[15]: {=bool:?} }}" , self . page_wp (0usize) , self . page_wp (1usize) , self . page_wp (2usize) , self . page_wp (3usize) , self . page_wp (4usize) , self . page_wp (5usize) , self . page_wp (6usize) , self . page_wp (7usize) , self . page_wp (8usize) , self . page_wp (9usize) , self . page_wp (10usize) , self . page_wp (11usize) , self . page_wp (12usize) , self . page_wp (13usize) , self . page_wp (14usize) , self . page_wp (15usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc12Ext13Rmp {
        #[doc = "Trigger source is TIM6_TRGO"]
        TIM6 = 0x0,
        #[doc = "Trigger source is TIM20_CC2"]
        TIM20 = 0x01,
    }
    impl Adc12Ext13Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc12Ext13Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc12Ext13Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc12Ext13Rmp {
            Adc12Ext13Rmp::from_bits(val)
        }
    }
    impl From<Adc12Ext13Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc12Ext13Rmp) -> u8 {
            Adc12Ext13Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc12Ext15Rmp {
        #[doc = "Trigger source is TIM3_CC4"]
        TIM3 = 0x0,
        #[doc = "Trigger source is TIM20_CC3"]
        TIM20 = 0x01,
    }
    impl Adc12Ext15Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc12Ext15Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc12Ext15Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc12Ext15Rmp {
            Adc12Ext15Rmp::from_bits(val)
        }
    }
    impl From<Adc12Ext15Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc12Ext15Rmp) -> u8 {
            Adc12Ext15Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc12Ext2Rmp {
        #[doc = "Trigger source is TIM3_CC3"]
        TIM1 = 0x0,
        #[doc = "rigger source is TIM20_TRGO"]
        TIM20 = 0x01,
    }
    impl Adc12Ext2Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc12Ext2Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc12Ext2Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc12Ext2Rmp {
            Adc12Ext2Rmp::from_bits(val)
        }
    }
    impl From<Adc12Ext2Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc12Ext2Rmp) -> u8 {
            Adc12Ext2Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc12Ext3Rmp {
        #[doc = "Trigger source is TIM2_CC2"]
        TIM2 = 0x0,
        #[doc = "rigger source is TIM20_TRGO2"]
        TIM20 = 0x01,
    }
    impl Adc12Ext3Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc12Ext3Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc12Ext3Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc12Ext3Rmp {
            Adc12Ext3Rmp::from_bits(val)
        }
    }
    impl From<Adc12Ext3Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc12Ext3Rmp) -> u8 {
            Adc12Ext3Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc12Ext5Rmp {
        #[doc = "Trigger source is TIM4_CC4"]
        TIM4 = 0x0,
        #[doc = "Trigger source is TIM20_CC1"]
        TIM20 = 0x01,
    }
    impl Adc12Ext5Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc12Ext5Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc12Ext5Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc12Ext5Rmp {
            Adc12Ext5Rmp::from_bits(val)
        }
    }
    impl From<Adc12Ext5Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc12Ext5Rmp) -> u8 {
            Adc12Ext5Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc12Jext13Rmp {
        #[doc = "Trigger source is TIM3_CC1"]
        TIM3 = 0x0,
        #[doc = "Trigger source is TIM20_CC4"]
        TIM20 = 0x01,
    }
    impl Adc12Jext13Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc12Jext13Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc12Jext13Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc12Jext13Rmp {
            Adc12Jext13Rmp::from_bits(val)
        }
    }
    impl From<Adc12Jext13Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc12Jext13Rmp) -> u8 {
            Adc12Jext13Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc12Jext3Rmp {
        #[doc = "Trigger source is TIM2_CC1"]
        TIM2 = 0x0,
        #[doc = "Trigger source is TIM20_TRGO"]
        TIM20 = 0x01,
    }
    impl Adc12Jext3Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc12Jext3Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc12Jext3Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc12Jext3Rmp {
            Adc12Jext3Rmp::from_bits(val)
        }
    }
    impl From<Adc12Jext3Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc12Jext3Rmp) -> u8 {
            Adc12Jext3Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc12Jext6Rmp {
        #[doc = "Trigger source is EXTI line 15"]
        EXTI15 = 0x0,
        #[doc = "Trigger source is TIM20_TRGO2"]
        TIM20 = 0x01,
    }
    impl Adc12Jext6Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc12Jext6Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc12Jext6Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc12Jext6Rmp {
            Adc12Jext6Rmp::from_bits(val)
        }
    }
    impl From<Adc12Jext6Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc12Jext6Rmp) -> u8 {
            Adc12Jext6Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc2DmaRmpCfgr3 {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "ADC2 mapped on DMA1 channel 2"]
        MAP_DMA1CH2 = 0x02,
        #[doc = "ADC2 mapped on DMA1 channel 4"]
        MAP_DMA1CH4 = 0x03,
    }
    impl Adc2DmaRmpCfgr3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc2DmaRmpCfgr3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc2DmaRmpCfgr3 {
        #[inline(always)]
        fn from(val: u8) -> Adc2DmaRmpCfgr3 {
            Adc2DmaRmpCfgr3::from_bits(val)
        }
    }
    impl From<Adc2DmaRmpCfgr3> for u8 {
        #[inline(always)]
        fn from(val: Adc2DmaRmpCfgr3) -> u8 {
            Adc2DmaRmpCfgr3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc34Ext15Rmp {
        #[doc = "Trigger source is TIM2_CC1"]
        TIM2 = 0x0,
        #[doc = "Trigger source is TIM20_CC1"]
        TIM20 = 0x01,
    }
    impl Adc34Ext15Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc34Ext15Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc34Ext15Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc34Ext15Rmp {
            Adc34Ext15Rmp::from_bits(val)
        }
    }
    impl From<Adc34Ext15Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc34Ext15Rmp) -> u8 {
            Adc34Ext15Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc34Ext5Rmp {
        #[doc = "Trigger source is EXTI line 2 when reset at 0"]
        EXTI2 = 0x0,
        #[doc = "Trigger source is TIM20_TRGO"]
        TIM20 = 0x01,
    }
    impl Adc34Ext5Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc34Ext5Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc34Ext5Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc34Ext5Rmp {
            Adc34Ext5Rmp::from_bits(val)
        }
    }
    impl From<Adc34Ext5Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc34Ext5Rmp) -> u8 {
            Adc34Ext5Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc34Ext6Rmp {
        #[doc = "Trigger source is TIM4_CC1"]
        TIM4 = 0x0,
        #[doc = "Trigger source is TIM20_TRGO2"]
        TIM20 = 0x01,
    }
    impl Adc34Ext6Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc34Ext6Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc34Ext6Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc34Ext6Rmp {
            Adc34Ext6Rmp::from_bits(val)
        }
    }
    impl From<Adc34Ext6Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc34Ext6Rmp) -> u8 {
            Adc34Ext6Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc34Jext11Rmp {
        #[doc = "Trigger source is TIM1_CC3"]
        TIM1 = 0x0,
        #[doc = "Trigger source is TIM20_TRGO2"]
        TIM20 = 0x01,
    }
    impl Adc34Jext11Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc34Jext11Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc34Jext11Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc34Jext11Rmp {
            Adc34Jext11Rmp::from_bits(val)
        }
    }
    impl From<Adc34Jext11Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc34Jext11Rmp) -> u8 {
            Adc34Jext11Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc34Jext14Rmp {
        #[doc = "Trigger source is TIM7_TRGO"]
        TIM7 = 0x0,
        #[doc = "Trigger source is TIM20_CC2"]
        TIM20 = 0x01,
    }
    impl Adc34Jext14Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc34Jext14Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc34Jext14Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc34Jext14Rmp {
            Adc34Jext14Rmp::from_bits(val)
        }
    }
    impl From<Adc34Jext14Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc34Jext14Rmp) -> u8 {
            Adc34Jext14Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc34Jext5Rmp {
        #[doc = "Trigger source is TIM4_CC3"]
        TIM4 = 0x0,
        #[doc = "Trigger source is TIM20_TRGO"]
        TIM20 = 0x01,
    }
    impl Adc34Jext5Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc34Jext5Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc34Jext5Rmp {
        #[inline(always)]
        fn from(val: u8) -> Adc34Jext5Rmp {
            Adc34Jext5Rmp::from_bits(val)
        }
    }
    impl From<Adc34Jext5Rmp> for u8 {
        #[inline(always)]
        fn from(val: Adc34Jext5Rmp) -> u8 {
            Adc34Jext5Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dac1Trig3Rmp {
        #[doc = "DAC trigger is TIM15_TRGO"]
        TIM15 = 0x0,
        #[doc = "DAC trigger is HRTIM1_DAC1_TRIG1"]
        HR_TIM1 = 0x01,
    }
    impl Dac1Trig3Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dac1Trig3Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dac1Trig3Rmp {
        #[inline(always)]
        fn from(val: u8) -> Dac1Trig3Rmp {
            Dac1Trig3Rmp::from_bits(val)
        }
    }
    impl From<Dac1Trig3Rmp> for u8 {
        #[inline(always)]
        fn from(val: Dac1Trig3Rmp) -> u8 {
            Dac1Trig3Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EncoderMode {
        #[doc = "No redirection"]
        NO_REDIRECTION = 0x0,
        #[doc = "TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
        MAP_TIM2TIM15 = 0x01,
        #[doc = "TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
        MAP_TIM3TIM15 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl EncoderMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EncoderMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EncoderMode {
        #[inline(always)]
        fn from(val: u8) -> EncoderMode {
            EncoderMode::from_bits(val)
        }
    }
    impl From<EncoderMode> for u8 {
        #[inline(always)]
        fn from(val: EncoderMode) -> u8 {
            EncoderMode::to_bits(val)
        }
    }
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
    pub enum I2c1RxDmaRmp {
        #[doc = "I2C1_RX mapped on DMA1 CH7"]
        MAP_DMA1CH7 = 0x0,
        #[doc = "I2C1_RX mapped on DMA1 CH3"]
        MAP_DMA1CH3 = 0x01,
        #[doc = "I2C1_RX mapped on DMA1 CH5"]
        MAP_DMA1CH5 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl I2c1RxDmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1RxDmaRmp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1RxDmaRmp {
        #[inline(always)]
        fn from(val: u8) -> I2c1RxDmaRmp {
            I2c1RxDmaRmp::from_bits(val)
        }
    }
    impl From<I2c1RxDmaRmp> for u8 {
        #[inline(always)]
        fn from(val: I2c1RxDmaRmp) -> u8 {
            I2c1RxDmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c1TxDmaRmp {
        #[doc = "I2C1_TX mapped on DMA1 CH6"]
        MAP_DMA1CH6 = 0x0,
        #[doc = "I2C1_TX mapped on DMA1 CH2"]
        MAP_DMA1CH2 = 0x01,
        #[doc = "I2C1_TX mapped on DMA1 CH4"]
        MAP_DMA1CH4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl I2c1TxDmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1TxDmaRmp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1TxDmaRmp {
        #[inline(always)]
        fn from(val: u8) -> I2c1TxDmaRmp {
            I2c1TxDmaRmp::from_bits(val)
        }
    }
    impl From<I2c1TxDmaRmp> for u8 {
        #[inline(always)]
        fn from(val: I2c1TxDmaRmp) -> u8 {
            I2c1TxDmaRmp::to_bits(val)
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi1RxDmaRmp {
        #[doc = "SPI1_RX mapped on DMA1 CH2"]
        MAP_DMA1CH3 = 0x0,
        #[doc = "SPI1_RX mapped on DMA1 CH4"]
        MAP_DMA1CH5 = 0x01,
        #[doc = "SPI1_RX mapped on DMA1 CH6"]
        MAP_DMA1CH7 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Spi1RxDmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi1RxDmaRmp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi1RxDmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Spi1RxDmaRmp {
            Spi1RxDmaRmp::from_bits(val)
        }
    }
    impl From<Spi1RxDmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Spi1RxDmaRmp) -> u8 {
            Spi1RxDmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi1TxDmaRmp {
        #[doc = "SPI1_TX mapped on DMA1 CH3"]
        MAP_DMA1CH3 = 0x0,
        #[doc = "SPI1_TX mapped on DMA1 CH5"]
        MAP_DMA1CH5 = 0x01,
        #[doc = "SPI1_TX mapped on DMA1 CH7"]
        MAP_DMA1CH7 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Spi1TxDmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi1TxDmaRmp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi1TxDmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Spi1TxDmaRmp {
            Spi1TxDmaRmp::from_bits(val)
        }
    }
    impl From<Spi1TxDmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Spi1TxDmaRmp) -> u8 {
            Spi1TxDmaRmp::to_bits(val)
        }
    }
}
