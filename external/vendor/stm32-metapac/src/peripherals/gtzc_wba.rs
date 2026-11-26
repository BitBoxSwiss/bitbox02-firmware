#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "GTZC_TZSC address block description."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtzcTzsc {
    ptr: *mut u8,
}
unsafe impl Send for GtzcTzsc {}
unsafe impl Sync for GtzcTzsc {}
impl GtzcTzsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GTZC1 TZSC control register."]
    #[inline(always)]
    pub const fn tzsc_cr(self) -> crate::common::Reg<regs::TzscCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GTZC1 TZSC secure configuration register 1."]
    #[inline(always)]
    pub const fn tzsc_seccfgr1(self) -> crate::common::Reg<regs::TzscSeccfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GTZC1 TZSC secure configuration register 2."]
    #[inline(always)]
    pub const fn tzsc_seccfgr2(self) -> crate::common::Reg<regs::TzscSeccfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "GTZC1 TZSC secure configuration register 3."]
    #[inline(always)]
    pub const fn tzsc_seccfgr3(self) -> crate::common::Reg<regs::TzscSeccfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 1."]
    #[inline(always)]
    pub const fn tzsc_privcfgr1(self) -> crate::common::Reg<regs::TzscPrivcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 2."]
    #[inline(always)]
    pub const fn tzsc_privcfgr2(self) -> crate::common::Reg<regs::TzscPrivcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 3."]
    #[inline(always)]
    pub const fn tzsc_privcfgr3(self) -> crate::common::Reg<regs::TzscPrivcfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "GTZC1 TZSC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscCr(pub u32);
    impl TzscCr {
        #[doc = "Lock the configuration of TZSC_SECCFGRn and TZSC_PRIVCFGRn registers until next reset."]
        #[inline(always)]
        pub const fn lck(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Lock the configuration of TZSC_SECCFGRn and TZSC_PRIVCFGRn registers until next reset."]
        #[inline(always)]
        pub fn set_lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TzscCr {
        #[inline(always)]
        fn default() -> TzscCr {
            TzscCr(0)
        }
    }
    impl core::fmt::Debug for TzscCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscCr").field("lck", &self.lck()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TzscCr {{ lck: {=bool:?} }}", self.lck())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscPrivcfgr1(pub u32);
    impl TzscPrivcfgr1 {
        #[doc = "Privileged access mode for TIM2."]
        #[inline(always)]
        pub const fn tim2priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM2."]
        #[inline(always)]
        pub fn set_tim2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged access mode for TIM3."]
        #[inline(always)]
        pub const fn tim3priv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM3."]
        #[inline(always)]
        pub fn set_tim3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Privileged access mode for TIM4."]
        #[inline(always)]
        pub const fn tim4priv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM4."]
        #[inline(always)]
        pub fn set_tim4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Privileged access mode for WWDG."]
        #[inline(always)]
        pub const fn wwdgpriv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for WWDG."]
        #[inline(always)]
        pub fn set_wwdgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Privileged access mode for IWDG."]
        #[inline(always)]
        pub const fn iwdgpriv(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for IWDG."]
        #[inline(always)]
        pub fn set_iwdgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Privileged access mode for SPI2."]
        #[inline(always)]
        pub const fn spi2priv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SPI2."]
        #[inline(always)]
        pub fn set_spi2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged access mode for USART2."]
        #[inline(always)]
        pub const fn usart2priv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for USART2."]
        #[inline(always)]
        pub fn set_usart2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Privileged access mode for USART3."]
        #[inline(always)]
        pub const fn usart3priv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for USART3."]
        #[inline(always)]
        pub fn set_usart3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Privileged access mode for I2C1."]
        #[inline(always)]
        pub const fn i2c1priv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for I2C1."]
        #[inline(always)]
        pub fn set_i2c1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Privileged access mode for I2C2."]
        #[inline(always)]
        pub const fn i2c2priv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for I2C2."]
        #[inline(always)]
        pub fn set_i2c2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Privileged access mode for I2C4."]
        #[inline(always)]
        pub const fn i2c4priv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for I2C4."]
        #[inline(always)]
        pub fn set_i2c4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Privileged access mode for LPTIM2."]
        #[inline(always)]
        pub const fn lptim2priv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for LPTIM2."]
        #[inline(always)]
        pub fn set_lptim2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for TzscPrivcfgr1 {
        #[inline(always)]
        fn default() -> TzscPrivcfgr1 {
            TzscPrivcfgr1(0)
        }
    }
    impl core::fmt::Debug for TzscPrivcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscPrivcfgr1")
                .field("tim2priv", &self.tim2priv())
                .field("tim3priv", &self.tim3priv())
                .field("tim4priv", &self.tim4priv())
                .field("wwdgpriv", &self.wwdgpriv())
                .field("iwdgpriv", &self.iwdgpriv())
                .field("spi2priv", &self.spi2priv())
                .field("usart2priv", &self.usart2priv())
                .field("usart3priv", &self.usart3priv())
                .field("i2c1priv", &self.i2c1priv())
                .field("i2c2priv", &self.i2c2priv())
                .field("i2c4priv", &self.i2c4priv())
                .field("lptim2priv", &self.lptim2priv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscPrivcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscPrivcfgr1 {{ tim2priv: {=bool:?}, tim3priv: {=bool:?}, tim4priv: {=bool:?}, wwdgpriv: {=bool:?}, iwdgpriv: {=bool:?}, spi2priv: {=bool:?}, usart2priv: {=bool:?}, usart3priv: {=bool:?}, i2c1priv: {=bool:?}, i2c2priv: {=bool:?}, i2c4priv: {=bool:?}, lptim2priv: {=bool:?} }}" , self . tim2priv () , self . tim3priv () , self . tim4priv () , self . wwdgpriv () , self . iwdgpriv () , self . spi2priv () , self . usart2priv () , self . usart3priv () , self . i2c1priv () , self . i2c2priv () , self . i2c4priv () , self . lptim2priv ())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscPrivcfgr2(pub u32);
    impl TzscPrivcfgr2 {
        #[doc = "Privileged access mode for TIM1."]
        #[inline(always)]
        pub const fn tim1priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM1."]
        #[inline(always)]
        pub fn set_tim1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged access mode for SPI1PRIV."]
        #[inline(always)]
        pub const fn spi1priv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SPI1PRIV."]
        #[inline(always)]
        pub fn set_spi1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Privileged access mode for USART1."]
        #[inline(always)]
        pub const fn usart1priv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for USART1."]
        #[inline(always)]
        pub fn set_usart1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Privileged access mode for TIM16."]
        #[inline(always)]
        pub const fn tim16priv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM16."]
        #[inline(always)]
        pub fn set_tim16priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Privileged access mode for TIM17."]
        #[inline(always)]
        pub const fn tim17priv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM17."]
        #[inline(always)]
        pub fn set_tim17priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Privileged access mode for SAI1."]
        #[inline(always)]
        pub const fn sai1priv(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SAI1."]
        #[inline(always)]
        pub fn set_sai1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Privileged access mode for SPI3."]
        #[inline(always)]
        pub const fn spi3priv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SPI3."]
        #[inline(always)]
        pub fn set_spi3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Privileged access mode for LPUART1."]
        #[inline(always)]
        pub const fn lpuart1priv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for LPUART1."]
        #[inline(always)]
        pub fn set_lpuart1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Privileged access mode for I2C3."]
        #[inline(always)]
        pub const fn i2c3priv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for I2C3."]
        #[inline(always)]
        pub fn set_i2c3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Privileged access mode for LPTIM1."]
        #[inline(always)]
        pub const fn lptim1priv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for LPTIM1."]
        #[inline(always)]
        pub fn set_lptim1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Privileged access mode for COMP."]
        #[inline(always)]
        pub const fn comppriv(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for COMP."]
        #[inline(always)]
        pub fn set_comppriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Privileged access mode for ADC4."]
        #[inline(always)]
        pub const fn adc4priv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for ADC4."]
        #[inline(always)]
        pub fn set_adc4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Privileged access mode for VREFBUF."]
        #[inline(always)]
        pub const fn vrefbufpriv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for VREFBUF."]
        #[inline(always)]
        pub fn set_vrefbufpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for TzscPrivcfgr2 {
        #[inline(always)]
        fn default() -> TzscPrivcfgr2 {
            TzscPrivcfgr2(0)
        }
    }
    impl core::fmt::Debug for TzscPrivcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscPrivcfgr2")
                .field("tim1priv", &self.tim1priv())
                .field("spi1priv", &self.spi1priv())
                .field("usart1priv", &self.usart1priv())
                .field("tim16priv", &self.tim16priv())
                .field("tim17priv", &self.tim17priv())
                .field("sai1priv", &self.sai1priv())
                .field("spi3priv", &self.spi3priv())
                .field("lpuart1priv", &self.lpuart1priv())
                .field("i2c3priv", &self.i2c3priv())
                .field("lptim1priv", &self.lptim1priv())
                .field("comppriv", &self.comppriv())
                .field("adc4priv", &self.adc4priv())
                .field("vrefbufpriv", &self.vrefbufpriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscPrivcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscPrivcfgr2 {{ tim1priv: {=bool:?}, spi1priv: {=bool:?}, usart1priv: {=bool:?}, tim16priv: {=bool:?}, tim17priv: {=bool:?}, sai1priv: {=bool:?}, spi3priv: {=bool:?}, lpuart1priv: {=bool:?}, i2c3priv: {=bool:?}, lptim1priv: {=bool:?}, comppriv: {=bool:?}, adc4priv: {=bool:?}, vrefbufpriv: {=bool:?} }}" , self . tim1priv () , self . spi1priv () , self . usart1priv () , self . tim16priv () , self . tim17priv () , self . sai1priv () , self . spi3priv () , self . lpuart1priv () , self . i2c3priv () , self . lptim1priv () , self . comppriv () , self . adc4priv () , self . vrefbufpriv ())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscPrivcfgr3(pub u32);
    impl TzscPrivcfgr3 {
        #[doc = "Privileged access mode for CRC."]
        #[inline(always)]
        pub const fn crcpriv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for CRC."]
        #[inline(always)]
        pub fn set_crcpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Privileged access mode for TSC."]
        #[inline(always)]
        pub const fn tscpriv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TSC."]
        #[inline(always)]
        pub fn set_tscpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Privileged access mode for ICACHE registers."]
        #[inline(always)]
        pub const fn icache_regpriv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for ICACHE registers."]
        #[inline(always)]
        pub fn set_icache_regpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Privileged access mode for USB OTG_HS."]
        #[inline(always)]
        pub const fn otgpriv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for USB OTG_HS."]
        #[inline(always)]
        pub fn set_otgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Privileged access mode for AES."]
        #[inline(always)]
        pub const fn aespriv(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for AES."]
        #[inline(always)]
        pub fn set_aespriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Privileged access mode for HASH."]
        #[inline(always)]
        pub const fn hashpriv(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for HASH."]
        #[inline(always)]
        pub fn set_hashpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Privileged access mode for RNG."]
        #[inline(always)]
        pub const fn rngpriv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for RNG."]
        #[inline(always)]
        pub fn set_rngpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Privileged access mode for SAES."]
        #[inline(always)]
        pub const fn saespriv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SAES."]
        #[inline(always)]
        pub fn set_saespriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Privileged access mode for PKA."]
        #[inline(always)]
        pub const fn pkapriv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for PKA."]
        #[inline(always)]
        pub fn set_pkapriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Privileged access mode for RAMCFG."]
        #[inline(always)]
        pub const fn ramcfgpriv(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for RAMCFG."]
        #[inline(always)]
        pub fn set_ramcfgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Privileged access mode for 2."]
        #[inline(always)]
        pub const fn radiopriv(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for 2."]
        #[inline(always)]
        pub fn set_radiopriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Privileged access mode for PTACONV."]
        #[inline(always)]
        pub const fn ptaconvpriv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for PTACONV."]
        #[inline(always)]
        pub fn set_ptaconvpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for TzscPrivcfgr3 {
        #[inline(always)]
        fn default() -> TzscPrivcfgr3 {
            TzscPrivcfgr3(0)
        }
    }
    impl core::fmt::Debug for TzscPrivcfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscPrivcfgr3")
                .field("crcpriv", &self.crcpriv())
                .field("tscpriv", &self.tscpriv())
                .field("icache_regpriv", &self.icache_regpriv())
                .field("otgpriv", &self.otgpriv())
                .field("aespriv", &self.aespriv())
                .field("hashpriv", &self.hashpriv())
                .field("rngpriv", &self.rngpriv())
                .field("saespriv", &self.saespriv())
                .field("pkapriv", &self.pkapriv())
                .field("ramcfgpriv", &self.ramcfgpriv())
                .field("radiopriv", &self.radiopriv())
                .field("ptaconvpriv", &self.ptaconvpriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscPrivcfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscPrivcfgr3 {{ crcpriv: {=bool:?}, tscpriv: {=bool:?}, icache_regpriv: {=bool:?}, otgpriv: {=bool:?}, aespriv: {=bool:?}, hashpriv: {=bool:?}, rngpriv: {=bool:?}, saespriv: {=bool:?}, pkapriv: {=bool:?}, ramcfgpriv: {=bool:?}, radiopriv: {=bool:?}, ptaconvpriv: {=bool:?} }}" , self . crcpriv () , self . tscpriv () , self . icache_regpriv () , self . otgpriv () , self . aespriv () , self . hashpriv () , self . rngpriv () , self . saespriv () , self . pkapriv () , self . ramcfgpriv () , self . radiopriv () , self . ptaconvpriv ())
        }
    }
    #[doc = "GTZC1 TZSC secure configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscSeccfgr1(pub u32);
    impl TzscSeccfgr1 {
        #[doc = "Secure access mode for TIM2."]
        #[inline(always)]
        pub const fn tim2sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM2."]
        #[inline(always)]
        pub fn set_tim2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure access mode for TIM3."]
        #[inline(always)]
        pub const fn tim3sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM3."]
        #[inline(always)]
        pub fn set_tim3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure access mode for TIM4."]
        #[inline(always)]
        pub const fn tim4sec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM4."]
        #[inline(always)]
        pub fn set_tim4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secure access mode for WWDG."]
        #[inline(always)]
        pub const fn wwdgsec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for WWDG."]
        #[inline(always)]
        pub fn set_wwdgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Secure access mode for IWDG."]
        #[inline(always)]
        pub const fn iwdgsec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for IWDG."]
        #[inline(always)]
        pub fn set_iwdgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure access mode for SPI2."]
        #[inline(always)]
        pub const fn spi2sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SPI2."]
        #[inline(always)]
        pub fn set_spi2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Secure access mode for USART2."]
        #[inline(always)]
        pub const fn usart2sec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for USART2."]
        #[inline(always)]
        pub fn set_usart2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Secure access mode for USART3."]
        #[inline(always)]
        pub const fn usart3sec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for USART3."]
        #[inline(always)]
        pub fn set_usart3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Secure access mode for I2C1."]
        #[inline(always)]
        pub const fn i2c1sec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for I2C1."]
        #[inline(always)]
        pub fn set_i2c1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Secure access mode for I2C2."]
        #[inline(always)]
        pub const fn i2c2sec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for I2C2."]
        #[inline(always)]
        pub fn set_i2c2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Secure access mode for I2C4."]
        #[inline(always)]
        pub const fn i2c4sec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for I2C4."]
        #[inline(always)]
        pub fn set_i2c4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure access mode for LPTIM2."]
        #[inline(always)]
        pub const fn lptim2sec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for LPTIM2."]
        #[inline(always)]
        pub fn set_lptim2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for TzscSeccfgr1 {
        #[inline(always)]
        fn default() -> TzscSeccfgr1 {
            TzscSeccfgr1(0)
        }
    }
    impl core::fmt::Debug for TzscSeccfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscSeccfgr1")
                .field("tim2sec", &self.tim2sec())
                .field("tim3sec", &self.tim3sec())
                .field("tim4sec", &self.tim4sec())
                .field("wwdgsec", &self.wwdgsec())
                .field("iwdgsec", &self.iwdgsec())
                .field("spi2sec", &self.spi2sec())
                .field("usart2sec", &self.usart2sec())
                .field("usart3sec", &self.usart3sec())
                .field("i2c1sec", &self.i2c1sec())
                .field("i2c2sec", &self.i2c2sec())
                .field("i2c4sec", &self.i2c4sec())
                .field("lptim2sec", &self.lptim2sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscSeccfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscSeccfgr1 {{ tim2sec: {=bool:?}, tim3sec: {=bool:?}, tim4sec: {=bool:?}, wwdgsec: {=bool:?}, iwdgsec: {=bool:?}, spi2sec: {=bool:?}, usart2sec: {=bool:?}, usart3sec: {=bool:?}, i2c1sec: {=bool:?}, i2c2sec: {=bool:?}, i2c4sec: {=bool:?}, lptim2sec: {=bool:?} }}" , self . tim2sec () , self . tim3sec () , self . tim4sec () , self . wwdgsec () , self . iwdgsec () , self . spi2sec () , self . usart2sec () , self . usart3sec () , self . i2c1sec () , self . i2c2sec () , self . i2c4sec () , self . lptim2sec ())
        }
    }
    #[doc = "GTZC1 TZSC secure configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscSeccfgr2(pub u32);
    impl TzscSeccfgr2 {
        #[doc = "Secure access mode for TIM1."]
        #[inline(always)]
        pub const fn tim1sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM1."]
        #[inline(always)]
        pub fn set_tim1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure access mode for SPI1."]
        #[inline(always)]
        pub const fn spi1sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SPI1."]
        #[inline(always)]
        pub fn set_spi1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure access mode for USART1."]
        #[inline(always)]
        pub const fn usart1sec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for USART1."]
        #[inline(always)]
        pub fn set_usart1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Secure access mode for TIM16."]
        #[inline(always)]
        pub const fn tim16sec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM16."]
        #[inline(always)]
        pub fn set_tim16sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Secure access mode for TIM17."]
        #[inline(always)]
        pub const fn tim17sec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM17."]
        #[inline(always)]
        pub fn set_tim17sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Secure access mode for SAI1."]
        #[inline(always)]
        pub const fn sai1sec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SAI1."]
        #[inline(always)]
        pub fn set_sai1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure access mode for SPI3."]
        #[inline(always)]
        pub const fn spi3sec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SPI3."]
        #[inline(always)]
        pub fn set_spi3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure access mode for LPUART1."]
        #[inline(always)]
        pub const fn lpuart1sec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for LPUART1."]
        #[inline(always)]
        pub fn set_lpuart1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Secure access mode for I2C3."]
        #[inline(always)]
        pub const fn i2c3sec(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for I2C3."]
        #[inline(always)]
        pub fn set_i2c3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Secure access mode for LPTIM1."]
        #[inline(always)]
        pub const fn lptim1sec(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for LPTIM1."]
        #[inline(always)]
        pub fn set_lptim1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Secure access mode for COMP."]
        #[inline(always)]
        pub const fn compsec(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for COMP."]
        #[inline(always)]
        pub fn set_compsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Secure access mode for ADC4."]
        #[inline(always)]
        pub const fn adc4sec(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for ADC4."]
        #[inline(always)]
        pub fn set_adc4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Secure access mode for VREFBUF."]
        #[inline(always)]
        pub const fn vrefbufsec(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for VREFBUF."]
        #[inline(always)]
        pub fn set_vrefbufsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for TzscSeccfgr2 {
        #[inline(always)]
        fn default() -> TzscSeccfgr2 {
            TzscSeccfgr2(0)
        }
    }
    impl core::fmt::Debug for TzscSeccfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscSeccfgr2")
                .field("tim1sec", &self.tim1sec())
                .field("spi1sec", &self.spi1sec())
                .field("usart1sec", &self.usart1sec())
                .field("tim16sec", &self.tim16sec())
                .field("tim17sec", &self.tim17sec())
                .field("sai1sec", &self.sai1sec())
                .field("spi3sec", &self.spi3sec())
                .field("lpuart1sec", &self.lpuart1sec())
                .field("i2c3sec", &self.i2c3sec())
                .field("lptim1sec", &self.lptim1sec())
                .field("compsec", &self.compsec())
                .field("adc4sec", &self.adc4sec())
                .field("vrefbufsec", &self.vrefbufsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscSeccfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscSeccfgr2 {{ tim1sec: {=bool:?}, spi1sec: {=bool:?}, usart1sec: {=bool:?}, tim16sec: {=bool:?}, tim17sec: {=bool:?}, sai1sec: {=bool:?}, spi3sec: {=bool:?}, lpuart1sec: {=bool:?}, i2c3sec: {=bool:?}, lptim1sec: {=bool:?}, compsec: {=bool:?}, adc4sec: {=bool:?}, vrefbufsec: {=bool:?} }}" , self . tim1sec () , self . spi1sec () , self . usart1sec () , self . tim16sec () , self . tim17sec () , self . sai1sec () , self . spi3sec () , self . lpuart1sec () , self . i2c3sec () , self . lptim1sec () , self . compsec () , self . adc4sec () , self . vrefbufsec ())
        }
    }
    #[doc = "GTZC1 TZSC secure configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscSeccfgr3(pub u32);
    impl TzscSeccfgr3 {
        #[doc = "Secure access mode for CRC."]
        #[inline(always)]
        pub const fn crcsec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for CRC."]
        #[inline(always)]
        pub fn set_crcsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Secure access mode for TSC."]
        #[inline(always)]
        pub const fn tscsec(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TSC."]
        #[inline(always)]
        pub fn set_tscsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Secure access mode for ICACHE registers."]
        #[inline(always)]
        pub const fn icache_regsec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for ICACHE registers."]
        #[inline(always)]
        pub fn set_icache_regsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Secure access mode for USB OTG_HS."]
        #[inline(always)]
        pub const fn otgsec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for USB OTG_HS."]
        #[inline(always)]
        pub fn set_otgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Secure access mode for AES."]
        #[inline(always)]
        pub const fn aessec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for AES."]
        #[inline(always)]
        pub fn set_aessec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Secure access mode for HASH."]
        #[inline(always)]
        pub const fn hashsec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for HASH."]
        #[inline(always)]
        pub fn set_hashsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Secure access mode for RNG."]
        #[inline(always)]
        pub const fn rngsec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for RNG."]
        #[inline(always)]
        pub fn set_rngsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Secure access mode for SAES."]
        #[inline(always)]
        pub const fn saessec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SAES."]
        #[inline(always)]
        pub fn set_saessec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Secure access mode for PKA."]
        #[inline(always)]
        pub const fn pkasec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for PKA."]
        #[inline(always)]
        pub fn set_pkasec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure access mode for RAMCFG."]
        #[inline(always)]
        pub const fn ramcfgsec(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for RAMCFG."]
        #[inline(always)]
        pub fn set_ramcfgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Secure access mode for 2."]
        #[inline(always)]
        pub const fn radiosec(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for 2."]
        #[inline(always)]
        pub fn set_radiosec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Secure access mode for PTACONV."]
        #[inline(always)]
        pub const fn ptaconvsec(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for PTACONV."]
        #[inline(always)]
        pub fn set_ptaconvsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for TzscSeccfgr3 {
        #[inline(always)]
        fn default() -> TzscSeccfgr3 {
            TzscSeccfgr3(0)
        }
    }
    impl core::fmt::Debug for TzscSeccfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscSeccfgr3")
                .field("crcsec", &self.crcsec())
                .field("tscsec", &self.tscsec())
                .field("icache_regsec", &self.icache_regsec())
                .field("otgsec", &self.otgsec())
                .field("aessec", &self.aessec())
                .field("hashsec", &self.hashsec())
                .field("rngsec", &self.rngsec())
                .field("saessec", &self.saessec())
                .field("pkasec", &self.pkasec())
                .field("ramcfgsec", &self.ramcfgsec())
                .field("radiosec", &self.radiosec())
                .field("ptaconvsec", &self.ptaconvsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscSeccfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscSeccfgr3 {{ crcsec: {=bool:?}, tscsec: {=bool:?}, icache_regsec: {=bool:?}, otgsec: {=bool:?}, aessec: {=bool:?}, hashsec: {=bool:?}, rngsec: {=bool:?}, saessec: {=bool:?}, pkasec: {=bool:?}, ramcfgsec: {=bool:?}, radiosec: {=bool:?}, ptaconvsec: {=bool:?} }}" , self . crcsec () , self . tscsec () , self . icache_regsec () , self . otgsec () , self . aessec () , self . hashsec () , self . rngsec () , self . saessec () , self . pkasec () , self . ramcfgsec () , self . radiosec () , self . ptaconvsec ())
        }
    }
}
