#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcc {
    ptr: *mut u8,
}
unsafe impl Send for Rcc {}
unsafe impl Sync for Rcc {}
impl Rcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CR register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CFGR register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CIER register."]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "CIFR register."]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "CSCMDR register."]
    #[inline(always)]
    pub const fn cscmdr(self) -> crate::common::Reg<regs::Cscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "AHBRSTR register."]
    #[inline(always)]
    pub const fn ahbrstr(self) -> crate::common::Reg<regs::Ahbrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "APB0RSTR register."]
    #[inline(always)]
    pub const fn apb0rstr(self) -> crate::common::Reg<regs::Apb0rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "APB1RSTR register."]
    #[inline(always)]
    pub const fn apb1rstr(self) -> crate::common::Reg<regs::Apb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "APB2RSTR register."]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "AHBENR register."]
    #[inline(always)]
    pub const fn ahbenr(self) -> crate::common::Reg<regs::Ahbenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "APB0ENR register."]
    #[inline(always)]
    pub const fn apb0enr(self) -> crate::common::Reg<regs::Apb0enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "APB1ENR register."]
    #[inline(always)]
    pub const fn apb1enr(self) -> crate::common::Reg<regs::Apb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "APB2ENR register."]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "CSR register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RFSWHSECR register."]
    #[inline(always)]
    pub const fn rfswhsecr(self) -> crate::common::Reg<regs::Rfswhsecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "RFHSECR register."]
    #[inline(always)]
    pub const fn rfhsecr(self) -> crate::common::Reg<regs::Rfhsecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
}
pub mod regs {
    #[doc = "AHBENR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbenr(pub u32);
    impl Ahbenr {
        #[doc = "DMA and DMAMUX enable Set and enable by software."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA and DMAMUX enable Set and enable by software."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOA enable. It must be enabled by default."]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA enable. It must be enabled by default."]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOB enable. It must be enabled by default."]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB enable. It must be enabled by default."]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC enable Set and enable by software."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC enable Set and enable by software."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PKA clock enable Set and enable by software."]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable Set and enable by software."]
        #[inline(always)]
        pub fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RNG clock enable Set and enable by software."]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable Set and enable by software."]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ahbenr {
        #[inline(always)]
        fn default() -> Ahbenr {
            Ahbenr(0)
        }
    }
    impl core::fmt::Debug for Ahbenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbenr")
                .field("dmaen", &self.dmaen())
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("crcen", &self.crcen())
                .field("pkaen", &self.pkaen())
                .field("rngen", &self.rngen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahbenr {{ dmaen: {=bool:?}, gpioaen: {=bool:?}, gpioben: {=bool:?}, crcen: {=bool:?}, pkaen: {=bool:?}, rngen: {=bool:?} }}" , self . dmaen () , self . gpioaen () , self . gpioben () , self . crcen () , self . pkaen () , self . rngen ())
        }
    }
    #[doc = "AHBRSTR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbrstr(pub u32);
    impl Ahbrstr {
        #[doc = "DMA and DMAMUX reset Set and reset by software."]
        #[inline(always)]
        pub const fn dmarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA and DMAMUX reset Set and reset by software."]
        #[inline(always)]
        pub fn set_dmarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOA reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOB reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC reset Set and reset by software."]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset Set and reset by software."]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PKA reset Set and reset by software."]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PKA reset Set and reset by software."]
        #[inline(always)]
        pub fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RNG reset Set and reset by software."]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG reset Set and reset by software."]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ahbrstr {
        #[inline(always)]
        fn default() -> Ahbrstr {
            Ahbrstr(0)
        }
    }
    impl core::fmt::Debug for Ahbrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbrstr")
                .field("dmarst", &self.dmarst())
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("crcrst", &self.crcrst())
                .field("pkarst", &self.pkarst())
                .field("rngrst", &self.rngrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahbrstr {{ dmarst: {=bool:?}, gpioarst: {=bool:?}, gpiobrst: {=bool:?}, crcrst: {=bool:?}, pkarst: {=bool:?}, rngrst: {=bool:?} }}" , self . dmarst () , self . gpioarst () , self . gpiobrst () , self . crcrst () , self . pkarst () , self . rngrst ())
        }
    }
    #[doc = "APB0ENR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb0enr(pub u32);
    impl Apb0enr {
        #[doc = "TIM1 enable."]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 enable."]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM2: Advanced Timer clock enable Set and enable by software."]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2: Advanced Timer clock enable Set and enable by software."]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM16 enable."]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 enable."]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM17 enable."]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 enable."]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SYSTEM CONFIG enable Set and enable by software."]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SYSTEM CONFIG enable Set and enable by software."]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RTC clock enable Set and enable by software. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable Set and enable by software. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Watchdog clock enable. Set and enable by software."]
        #[inline(always)]
        pub const fn wdgen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog clock enable. Set and enable by software."]
        #[inline(always)]
        pub fn set_wdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Apb0enr {
        #[inline(always)]
        fn default() -> Apb0enr {
            Apb0enr(0)
        }
    }
    impl core::fmt::Debug for Apb0enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb0enr")
                .field("tim1en", &self.tim1en())
                .field("tim2en", &self.tim2en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("syscfgen", &self.syscfgen())
                .field("rtcen", &self.rtcen())
                .field("wdgen", &self.wdgen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb0enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb0enr {{ tim1en: {=bool:?}, tim2en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, syscfgen: {=bool:?}, rtcen: {=bool:?}, wdgen: {=bool:?} }}" , self . tim1en () , self . tim2en () , self . tim16en () , self . tim17en () , self . syscfgen () , self . rtcen () , self . wdgen ())
        }
    }
    #[doc = "APB0RSTR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb0rstr(pub u32);
    impl Apb0rstr {
        #[doc = "TIM1: Advanced Timer reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1: Advanced Timer reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM16 reset."]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 reset."]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM17 reset."]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 reset."]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SYSTEM CONFIG reset Set and reset by software."]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SYSTEM CONFIG reset Set and reset by software."]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RTC reset Set and reset by software."]
        #[inline(always)]
        pub const fn rtcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RTC reset Set and reset by software."]
        #[inline(always)]
        pub fn set_rtcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "WATCHDOG reset Set and reset by software."]
        #[inline(always)]
        pub const fn wdgrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "WATCHDOG reset Set and reset by software."]
        #[inline(always)]
        pub fn set_wdgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "WATCHDOG reset Set and reset by software."]
        #[inline(always)]
        pub const fn wdrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "WATCHDOG reset Set and reset by software."]
        #[inline(always)]
        pub fn set_wdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Apb0rstr {
        #[inline(always)]
        fn default() -> Apb0rstr {
            Apb0rstr(0)
        }
    }
    impl core::fmt::Debug for Apb0rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb0rstr")
                .field("tim1rst", &self.tim1rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("syscfgrst", &self.syscfgrst())
                .field("rtcrst", &self.rtcrst())
                .field("wdgrst", &self.wdgrst())
                .field("wdrst", &self.wdrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb0rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb0rstr {{ tim1rst: {=bool:?}, tim16rst: {=bool:?}, tim17rst: {=bool:?}, syscfgrst: {=bool:?}, rtcrst: {=bool:?}, wdgrst: {=bool:?}, wdrst: {=bool:?} }}" , self . tim1rst () , self . tim16rst () , self . tim17rst () , self . syscfgrst () , self . rtcrst () , self . wdgrst () , self . wdrst ())
        }
    }
    #[doc = "APB1ENR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr(pub u32);
    impl Apb1enr {
        #[doc = "SPI1 enable."]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 enable."]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AUXADC clock enable for Aux-ADC digital clock Set and enable by software."]
        #[inline(always)]
        pub const fn adcdigen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AUXADC clock enable for Aux-ADC digital clock Set and enable by software."]
        #[inline(always)]
        pub fn set_adcdigen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC clock enable for Aux-ADC analog clock Set and enable by software."]
        #[inline(always)]
        pub const fn adcanaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clock enable for Aux-ADC analog clock Set and enable by software."]
        #[inline(always)]
        pub fn set_adcanaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART clock enable Set and enable by software."]
        #[inline(always)]
        pub const fn lpuarten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART clock enable Set and enable by software."]
        #[inline(always)]
        pub fn set_lpuarten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USART clock enable Set and enable by software."]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "USART clock enable Set and enable by software."]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SPI2 enable."]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 enable."]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI3 clock enable Set and enable by software."]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable Set and enable by software."]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "I2C1 clock enable Set and enable by software."]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable Set and enable by software."]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 enable."]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 enable."]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1enr {
        #[inline(always)]
        fn default() -> Apb1enr {
            Apb1enr(0)
        }
    }
    impl core::fmt::Debug for Apb1enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr")
                .field("spi1en", &self.spi1en())
                .field("adcdigen", &self.adcdigen())
                .field("adcanaen", &self.adcanaen())
                .field("lpuarten", &self.lpuarten())
                .field("usart1en", &self.usart1en())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1enr {{ spi1en: {=bool:?}, adcdigen: {=bool:?}, adcanaen: {=bool:?}, lpuarten: {=bool:?}, usart1en: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?} }}" , self . spi1en () , self . adcdigen () , self . adcanaen () , self . lpuarten () , self . usart1en () , self . spi2en () , self . spi3en () , self . i2c1en () , self . i2c2en ())
        }
    }
    #[doc = "APB1RSTR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr(pub u32);
    impl Apb1rstr {
        #[doc = "SPI1 reset."]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset."]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC reset."]
        #[inline(always)]
        pub const fn adcrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC reset."]
        #[inline(always)]
        pub fn set_adcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AUXADC reset for Aux-ADC digital clock Set and reset by software."]
        #[inline(always)]
        pub const fn auxadcrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AUXADC reset for Aux-ADC digital clock Set and reset by software."]
        #[inline(always)]
        pub fn set_auxadcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPUART reset Set and reset by software."]
        #[inline(always)]
        pub const fn lpuartrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lpuartrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USART reset Set and reset by software."]
        #[inline(always)]
        pub const fn usartrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "USART reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usartrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SPI2 reset."]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset."]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI3 reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "I2C1 reset Set and reset by software."]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C1 reset Set and reset by software."]
        #[inline(always)]
        pub const fn i2c21rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c21rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "2C2 reset."]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "2C2 reset."]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1rstr {
        #[inline(always)]
        fn default() -> Apb1rstr {
            Apb1rstr(0)
        }
    }
    impl core::fmt::Debug for Apb1rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr")
                .field("spi1rst", &self.spi1rst())
                .field("adcrst", &self.adcrst())
                .field("auxadcrst", &self.auxadcrst())
                .field("lpuartrst", &self.lpuartrst())
                .field("usartrst", &self.usartrst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c21rst", &self.i2c21rst())
                .field("i2c2rst", &self.i2c2rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1rstr {{ spi1rst: {=bool:?}, adcrst: {=bool:?}, auxadcrst: {=bool:?}, lpuartrst: {=bool:?}, usartrst: {=bool:?}, spi2rst: {=bool:?}, spi3rst: {=bool:?}, i2c1rst: {=bool:?}, i2c21rst: {=bool:?}, i2c2rst: {=bool:?} }}" , self . spi1rst () , self . adcrst () , self . auxadcrst () , self . lpuartrst () , self . usartrst () , self . spi2rst () , self . spi3rst () , self . i2c1rst () , self . i2c21rst () , self . i2c2rst ())
        }
    }
    #[doc = "APB2ENR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "MR_BLE enable."]
        #[inline(always)]
        pub const fn mrbleen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MR_BLE enable."]
        #[inline(always)]
        pub fn set_mrbleen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MR_BLE clock frequency selection when RCC_APB2ENR.MRBLEEN=1."]
        #[inline(always)]
        pub const fn clkblediv(&self) -> super::vals::Clkblediv {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Clkblediv::from_bits(val as u8)
        }
        #[doc = "MR_BLE clock frequency selection when RCC_APB2ENR.MRBLEEN=1."]
        #[inline(always)]
        pub fn set_clkblediv(&mut self, val: super::vals::Clkblediv) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    impl core::fmt::Debug for Apb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2enr")
                .field("mrbleen", &self.mrbleen())
                .field("clkblediv", &self.clkblediv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2enr {{ mrbleen: {=bool:?}, clkblediv: {:?} }}",
                self.mrbleen(),
                self.clkblediv()
            )
        }
    }
    #[doc = "APB2RSTR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "BLE reset."]
        #[inline(always)]
        pub const fn blerst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BLE reset."]
        #[inline(always)]
        pub fn set_blerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MR_BLE (Bluetooth radio) reset."]
        #[inline(always)]
        pub const fn mrblerst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MR_BLE (Bluetooth radio) reset."]
        #[inline(always)]
        pub fn set_mrblerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    impl core::fmt::Debug for Apb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2rstr")
                .field("blerst", &self.blerst())
                .field("mrblerst", &self.mrblerst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2rstr {{ blerst: {=bool:?}, mrblerst: {=bool:?} }}",
                self.blerst(),
                self.mrblerst()
            )
        }
    }
    #[doc = "CFGR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "bit to control inversion of the SMPS clock."]
        #[inline(always)]
        pub const fn smpsinv(&self) -> super::vals::Smpsinv {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Smpsinv::from_bits(val as u8)
        }
        #[doc = "bit to control inversion of the SMPS clock."]
        #[inline(always)]
        pub fn set_smpsinv(&mut self, val: super::vals::Smpsinv) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock source selection request:."]
        #[inline(always)]
        pub const fn hsesel(&self) -> super::vals::Hsesel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Hsesel::from_bits(val as u8)
        }
        #[doc = "Clock source selection request:."]
        #[inline(always)]
        pub fn set_hsesel(&mut self, val: super::vals::Hsesel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Stop HSI clock source request."]
        #[inline(always)]
        pub const fn stophsi(&self) -> super::vals::Stophsi {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Stophsi::from_bits(val as u8)
        }
        #[doc = "Stop HSI clock source request."]
        #[inline(always)]
        pub fn set_stophsi(&mut self, val: super::vals::Stophsi) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clock source selection Status."]
        #[inline(always)]
        pub const fn hsesel_status(&self) -> super::vals::HseselStatus {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::HseselStatus::from_bits(val as u8)
        }
        #[doc = "Clock source selection Status."]
        #[inline(always)]
        pub fn set_hsesel_status(&mut self, val: super::vals::HseselStatus) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "CLKSYSDIV: system clock divided factor from HSI_64M. 000: system clock frequency is 64 MHz (not available when HSESEL=1) 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz * 100: system clock frequency is 4 MHz * 101: system clock frequency is 2 MHz * 110: system clock frequency is 1 MHz * 111: not used. *: If RCC_APB2ENR.MRBLEEN bit is set, writing in CLKSYSDIV one of those values is replaced by a 010b = 16 MHz writing at hardware level. Warning: if the software programs the 64 MHz frequency target while the RCC_CFGR.HSESEL=1, the hardware will switch the system clock tree on HSI64MPLL again (and restart HSIPLL64M analog block if RCC_CFGR.STOPHSI=1) To switch the system frequency between 64 / 32 / 16 MHz without risk when the MR_BLE is used, prefer the RCC_CSCMDR register to change the system frequency. the MR_BLE frequency must always be equal or less than the CPU/system clock to have functional radio."]
        #[inline(always)]
        pub const fn clksysdiv(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "CLKSYSDIV: system clock divided factor from HSI_64M. 000: system clock frequency is 64 MHz (not available when HSESEL=1) 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz * 100: system clock frequency is 4 MHz * 101: system clock frequency is 2 MHz * 110: system clock frequency is 1 MHz * 111: not used. *: If RCC_APB2ENR.MRBLEEN bit is set, writing in CLKSYSDIV one of those values is replaced by a 010b = 16 MHz writing at hardware level. Warning: if the software programs the 64 MHz frequency target while the RCC_CFGR.HSESEL=1, the hardware will switch the system clock tree on HSI64MPLL again (and restart HSIPLL64M analog block if RCC_CFGR.STOPHSI=1) To switch the system frequency between 64 / 32 / 16 MHz without risk when the MR_BLE is used, prefer the RCC_CSCMDR register to change the system frequency. the MR_BLE frequency must always be equal or less than the CPU/system clock to have functional radio."]
        #[inline(always)]
        pub fn set_clksysdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "CLKSYSDIV_STATUS: system clock frequency status Set and cleared by hardware to indicate the actual system clock frequency. This register must be read to be sure that the new frequency, selected by CLKSYSDIV, has been applied. 000: system clock frequency is 64 MHz 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz 100: system clock frequency is 4 MHz 101: system clock frequency is 2 MHz 110: system clock frequency is 1 MHz 111: not used. The actual clock frequency switching can be delayed of up to 128 system clock cycles, depending on the RCC internal counter status at the moment the new CLKSYSDIV is applied."]
        #[inline(always)]
        pub const fn clksysdiv_status(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "CLKSYSDIV_STATUS: system clock frequency status Set and cleared by hardware to indicate the actual system clock frequency. This register must be read to be sure that the new frequency, selected by CLKSYSDIV, has been applied. 000: system clock frequency is 64 MHz 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz 100: system clock frequency is 4 MHz 101: system clock frequency is 2 MHz 110: system clock frequency is 1 MHz 111: not used. The actual clock frequency switching can be delayed of up to 128 system clock cycles, depending on the RCC internal counter status at the moment the new CLKSYSDIV is applied."]
        #[inline(always)]
        pub fn set_clksysdiv_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "SMPS clock prescaling factor to generate 4MHz or 8MHz."]
        #[inline(always)]
        pub const fn smpsdiv(&self) -> super::vals::Smpsdiv {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Smpsdiv::from_bits(val as u8)
        }
        #[doc = "SMPS clock prescaling factor to generate 4MHz or 8MHz."]
        #[inline(always)]
        pub fn set_smpsdiv(&mut self, val: super::vals::Smpsdiv) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Selection of LPUART clock:."]
        #[inline(always)]
        pub const fn lpuclksel(&self) -> super::vals::Lpuclksel {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Lpuclksel::from_bits(val as u8)
        }
        #[doc = "Selection of LPUART clock:."]
        #[inline(always)]
        pub fn set_lpuclksel(&mut self, val: super::vals::Lpuclksel) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub const fn clkslowsel(&self) -> super::vals::Clkslowsel {
            let val = (self.0 >> 15usize) & 0x03;
            super::vals::Clkslowsel::from_bits(val as u8)
        }
        #[doc = "slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub fn set_clkslowsel(&mut self, val: super::vals::Clkslowsel) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val.to_bits() as u32) & 0x03) << 15usize);
        }
        #[doc = "IO BOOSTER enable Set and reset by software."]
        #[inline(always)]
        pub const fn ioboosten(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "IO BOOSTER enable Set and reset by software."]
        #[inline(always)]
        pub fn set_ioboosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "IO BOOSTER clock enable as external clock Set and reset by software."]
        #[inline(always)]
        pub const fn ioboostclkexten(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IO BOOSTER clock enable as external clock Set and reset by software."]
        #[inline(always)]
        pub fn set_ioboostclkexten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LCO output enable."]
        #[inline(always)]
        pub const fn lcoen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LCO output enable."]
        #[inline(always)]
        pub fn set_lcoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Selection of I2S1 clock: 1x:64MHz peripheral clock."]
        #[inline(always)]
        pub const fn spi3i2sclksel(&self) -> super::vals::Spiisclksel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Spiisclksel::from_bits(val as u8)
        }
        #[doc = "Selection of I2S1 clock: 1x:64MHz peripheral clock."]
        #[inline(always)]
        pub fn set_spi3i2sclksel(&mut self, val: super::vals::Spiisclksel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Selection of I2S clock: 1x:64MHz peripheral clock."]
        #[inline(always)]
        pub const fn spi2i2sclksel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Selection of I2S clock: 1x:64MHz peripheral clock."]
        #[inline(always)]
        pub fn set_spi2i2sclksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub const fn lcosel(&self) -> super::vals::Lcosel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Lcosel::from_bits(val as u8)
        }
        #[doc = "Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub fn set_lcosel(&mut self, val: super::vals::Lcosel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible."]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 26usize) & 0x07;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible."]
        #[inline(always)]
        pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
        }
        #[doc = "Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. Others: not used."]
        #[inline(always)]
        pub const fn ccopre(&self) -> super::vals::Ccopre {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Ccopre::from_bits(val as u8)
        }
        #[doc = "Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. Others: not used."]
        #[inline(always)]
        pub fn set_ccopre(&mut self, val: super::vals::Ccopre) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("smpsinv", &self.smpsinv())
                .field("hsesel", &self.hsesel())
                .field("stophsi", &self.stophsi())
                .field("hsesel_status", &self.hsesel_status())
                .field("clksysdiv", &self.clksysdiv())
                .field("clksysdiv_status", &self.clksysdiv_status())
                .field("smpsdiv", &self.smpsdiv())
                .field("lpuclksel", &self.lpuclksel())
                .field("clkslowsel", &self.clkslowsel())
                .field("ioboosten", &self.ioboosten())
                .field("ioboostclkexten", &self.ioboostclkexten())
                .field("lcoen", &self.lcoen())
                .field("spi3i2sclksel", &self.spi3i2sclksel())
                .field("spi2i2sclksel", &self.spi2i2sclksel())
                .field("lcosel", &self.lcosel())
                .field("mcosel", &self.mcosel())
                .field("ccopre", &self.ccopre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ smpsinv: {:?}, hsesel: {:?}, stophsi: {:?}, hsesel_status: {:?}, clksysdiv: {=u8:?}, clksysdiv_status: {=u8:?}, smpsdiv: {:?}, lpuclksel: {:?}, clkslowsel: {:?}, ioboosten: {=bool:?}, ioboostclkexten: {=bool:?}, lcoen: {=bool:?}, spi3i2sclksel: {:?}, spi2i2sclksel: {=bool:?}, lcosel: {:?}, mcosel: {:?}, ccopre: {:?} }}" , self . smpsinv () , self . hsesel () , self . stophsi () , self . hsesel_status () , self . clksysdiv () , self . clksysdiv_status () , self . smpsdiv () , self . lpuclksel () , self . clkslowsel () , self . ioboosten () , self . ioboostclkexten () , self . lcoen () , self . spi3i2sclksel () , self . spi2i2sclksel () , self . lcosel () , self . mcosel () , self . ccopre ())
        }
    }
    #[doc = "CIER register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization."]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization."]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization."]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization."]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization."]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization."]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE."]
        #[inline(always)]
        pub const fn hsipllrdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE."]
        #[inline(always)]
        pub fn set_hsipllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock."]
        #[inline(always)]
        pub const fn hsipllunlockdetie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock."]
        #[inline(always)]
        pub fn set_hsipllunlockdetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end."]
        #[inline(always)]
        pub const fn rtcrstie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end."]
        #[inline(always)]
        pub fn set_rtcrstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end."]
        #[inline(always)]
        pub const fn wdgrstie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end."]
        #[inline(always)]
        pub fn set_wdgrstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPURSTIE: LPUART reset release interrupt enable."]
        #[inline(always)]
        pub const fn lpurstie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPURSTIE: LPUART reset release interrupt enable."]
        #[inline(always)]
        pub fn set_lpurstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cier {
        #[inline(always)]
        fn default() -> Cier {
            Cier(0)
        }
    }
    impl core::fmt::Debug for Cier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cier")
                .field("lsirdyie", &self.lsirdyie())
                .field("lserdyie", &self.lserdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("hsipllrdyie", &self.hsipllrdyie())
                .field("hsipllunlockdetie", &self.hsipllunlockdetie())
                .field("rtcrstie", &self.rtcrstie())
                .field("wdgrstie", &self.wdgrstie())
                .field("lpurstie", &self.lpurstie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cier {{ lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, hsipllrdyie: {=bool:?}, hsipllunlockdetie: {=bool:?}, rtcrstie: {=bool:?}, wdgrstie: {=bool:?}, lpurstie: {=bool:?} }}" , self . lsirdyie () , self . lserdyie () , self . hsirdyie () , self . hserdyie () , self . hsipllrdyie () , self . hsipllunlockdetie () , self . rtcrstie () , self . wdgrstie () , self . lpurstie ())
        }
    }
    #[doc = "CIFR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable."]
        #[inline(always)]
        pub const fn lsirdyif(&self) -> super::vals::Lsirdyif {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lsirdyif::from_bits(val as u8)
        }
        #[doc = "LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable."]
        #[inline(always)]
        pub fn set_lsirdyif(&mut self, val: super::vals::Lsirdyif) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable."]
        #[inline(always)]
        pub const fn lserdyif(&self) -> super::vals::Lserdyif {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Lserdyif::from_bits(val as u8)
        }
        #[doc = "LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable."]
        #[inline(always)]
        pub fn set_lserdyif(&mut self, val: super::vals::Lserdyif) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable."]
        #[inline(always)]
        pub const fn hsirdyif(&self) -> super::vals::Hsirdyif {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Hsirdyif::from_bits(val as u8)
        }
        #[doc = "HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable."]
        #[inline(always)]
        pub fn set_hsirdyif(&mut self, val: super::vals::Hsirdyif) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable."]
        #[inline(always)]
        pub const fn hserdyif(&self) -> super::vals::Hserdyif {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Hserdyif::from_bits(val as u8)
        }
        #[doc = "HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable."]
        #[inline(always)]
        pub fn set_hserdyif(&mut self, val: super::vals::Hserdyif) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable."]
        #[inline(always)]
        pub const fn hsipllrdyif(&self) -> super::vals::Hsipllrdyif {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Hsipllrdyif::from_bits(val as u8)
        }
        #[doc = "HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable."]
        #[inline(always)]
        pub fn set_hsipllrdyif(&mut self, val: super::vals::Hsipllrdyif) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag."]
        #[inline(always)]
        pub const fn hsipllunlockdetif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag."]
        #[inline(always)]
        pub fn set_hsipllunlockdetif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock."]
        #[inline(always)]
        pub const fn rtcrstif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock."]
        #[inline(always)]
        pub fn set_rtcrstif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock."]
        #[inline(always)]
        pub const fn wdgrstif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock."]
        #[inline(always)]
        pub fn set_wdgrstif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPUART reset release flag."]
        #[inline(always)]
        pub const fn lpurstf(&self) -> super::vals::Lpurstf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Lpurstf::from_bits(val as u8)
        }
        #[doc = "LPUART reset release flag."]
        #[inline(always)]
        pub fn set_lpurstf(&mut self, val: super::vals::Lpurstf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cifr {
        #[inline(always)]
        fn default() -> Cifr {
            Cifr(0)
        }
    }
    impl core::fmt::Debug for Cifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cifr")
                .field("lsirdyif", &self.lsirdyif())
                .field("lserdyif", &self.lserdyif())
                .field("hsirdyif", &self.hsirdyif())
                .field("hserdyif", &self.hserdyif())
                .field("hsipllrdyif", &self.hsipllrdyif())
                .field("hsipllunlockdetif", &self.hsipllunlockdetif())
                .field("rtcrstif", &self.rtcrstif())
                .field("wdgrstif", &self.wdgrstif())
                .field("lpurstf", &self.lpurstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cifr {{ lsirdyif: {:?}, lserdyif: {:?}, hsirdyif: {:?}, hserdyif: {:?}, hsipllrdyif: {:?}, hsipllunlockdetif: {=bool:?}, rtcrstif: {=bool:?}, wdgrstif: {=bool:?}, lpurstf: {:?} }}" , self . lsirdyif () , self . lserdyif () , self . hsirdyif () , self . hserdyif () , self . hsipllrdyif () , self . hsipllunlockdetif () , self . rtcrstif () , self . wdgrstif () , self . lpurstf ())
        }
    }
    #[doc = "CR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Internal Low Speed oscillator Ready Set and reset by hardware to indicate when the Low Speed Internal RC oscillator is stable. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub const fn lsirdy(&self) -> super::vals::Lsirdy {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Lsirdy::from_bits(val as u8)
        }
        #[doc = "Internal Low Speed oscillator Ready Set and reset by hardware to indicate when the Low Speed Internal RC oscillator is stable. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: super::vals::Lsirdy) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "External Low Speed Clock ready flag. Set by hardware to indicate that LSE oscillator is stable."]
        #[inline(always)]
        pub const fn lserdy(&self) -> super::vals::Lserdy {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Lserdy::from_bits(val as u8)
        }
        #[doc = "External Low Speed Clock ready flag. Set by hardware to indicate that LSE oscillator is stable."]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: super::vals::Lserdy) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub const fn lsebyp(&self) -> super::vals::Lsebyp {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Lsebyp::from_bits(val as u8)
        }
        #[doc = "External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn."]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: super::vals::Lsebyp) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0."]
        #[inline(always)]
        pub const fn lockdet_nstop(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x07;
            val as u8
        }
        #[doc = "Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0."]
        #[inline(always)]
        pub fn set_lockdet_nstop(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
        }
        #[doc = "Internal High Speed clock ready flag. Set by hardware to indicate that internal RC 64MHz oscillator is stable. This bit is activated only if the RC is enabled by HSION (it is not activated if the RC is enabled by an IP request)."]
        #[inline(always)]
        pub const fn hsirdy(&self) -> super::vals::Hsirdy {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Hsirdy::from_bits(val as u8)
        }
        #[doc = "Internal High Speed clock ready flag. Set by hardware to indicate that internal RC 64MHz oscillator is stable. This bit is activated only if the RC is enabled by HSION (it is not activated if the RC is enabled by an IP request)."]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: super::vals::Hsirdy) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "External High Speed Clock Buffer for PLL RF2G4 enable. Set and reset by software."]
        #[inline(always)]
        pub const fn hsepllbufon(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed Clock Buffer for PLL RF2G4 enable. Set and reset by software."]
        #[inline(always)]
        pub fn set_hsepllbufon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Internal High Speed Clock PLL enable."]
        #[inline(always)]
        pub const fn hsipllon(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed Clock PLL enable."]
        #[inline(always)]
        pub fn set_hsipllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Internal High Speed Clock PLL ready flag."]
        #[inline(always)]
        pub const fn hsipllrdy(&self) -> super::vals::Hsipllrdy {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Hsipllrdy::from_bits(val as u8)
        }
        #[doc = "Internal High Speed Clock PLL ready flag."]
        #[inline(always)]
        pub fn set_hsipllrdy(&mut self, val: super::vals::Hsipllrdy) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Force MR_BLE active transmission status (for debug purpose)."]
        #[inline(always)]
        pub const fn fmrat(&self) -> super::vals::Fmrat {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Fmrat::from_bits(val as u8)
        }
        #[doc = "Force MR_BLE active transmission status (for debug purpose)."]
        #[inline(always)]
        pub fn set_fmrat(&mut self, val: super::vals::Fmrat) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off."]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off."]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External High Speed Clock ready flag. Set by hardware to indicate that HSE oscillator is stable."]
        #[inline(always)]
        pub const fn hserdy(&self) -> super::vals::Hserdy {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Hserdy::from_bits(val as u8)
        }
        #[doc = "External High Speed Clock ready flag. Set by hardware to indicate that HSE oscillator is stable."]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: super::vals::Hserdy) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .field("lseon", &self.lseon())
                .field("lserdy", &self.lserdy())
                .field("lsebyp", &self.lsebyp())
                .field("lockdet_nstop", &self.lockdet_nstop())
                .field("hsirdy", &self.hsirdy())
                .field("hsepllbufon", &self.hsepllbufon())
                .field("hsipllon", &self.hsipllon())
                .field("hsipllrdy", &self.hsipllrdy())
                .field("fmrat", &self.fmrat())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ lsion: {=bool:?}, lsirdy: {:?}, lseon: {=bool:?}, lserdy: {:?}, lsebyp: {:?}, lockdet_nstop: {=u8:?}, hsirdy: {:?}, hsepllbufon: {=bool:?}, hsipllon: {=bool:?}, hsipllrdy: {:?}, fmrat: {:?}, hseon: {=bool:?}, hserdy: {:?} }}" , self . lsion () , self . lsirdy () , self . lseon () , self . lserdy () , self . lsebyp () , self . lockdet_nstop () , self . hsirdy () , self . hsepllbufon () , self . hsipllon () , self . hsipllrdy () , self . fmrat () , self . hseon () , self . hserdy ())
        }
    }
    #[doc = "CSCMDR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cscmdr(pub u32);
    impl Cscmdr {
        #[doc = "Request for system clock switching Cleared by hardware when system clock frequency switch is done."]
        #[inline(always)]
        pub const fn request(&self) -> super::vals::Request {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Request::from_bits(val as u8)
        }
        #[doc = "Request for system clock switching Cleared by hardware when system clock frequency switch is done."]
        #[inline(always)]
        pub fn set_request(&mut self, val: super::vals::Request) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "system clock dividing factor from HSI_64M requested Note: behavior depends on BLEEN in APB2ENR register."]
        #[inline(always)]
        pub const fn clksysdiv_req(&self) -> super::vals::ClksysdivReq {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::ClksysdivReq::from_bits(val as u8)
        }
        #[doc = "system clock dividing factor from HSI_64M requested Note: behavior depends on BLEEN in APB2ENR register."]
        #[inline(always)]
        pub fn set_clksysdiv_req(&mut self, val: super::vals::ClksysdivReq) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "Status of clock switch sequence."]
        #[inline(always)]
        pub const fn status(&self) -> super::vals::Status {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Status::from_bits(val as u8)
        }
        #[doc = "Status of clock switch sequence."]
        #[inline(always)]
        pub fn set_status(&mut self, val: super::vals::Status) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch."]
        #[inline(always)]
        pub const fn eofseq_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch."]
        #[inline(always)]
        pub fn set_eofseq_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "End of Sequence flag Set by hardware when clock system swtich is ended."]
        #[inline(always)]
        pub const fn eofseq_irq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "End of Sequence flag Set by hardware when clock system swtich is ended."]
        #[inline(always)]
        pub fn set_eofseq_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cscmdr {
        #[inline(always)]
        fn default() -> Cscmdr {
            Cscmdr(0)
        }
    }
    impl core::fmt::Debug for Cscmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cscmdr")
                .field("request", &self.request())
                .field("clksysdiv_req", &self.clksysdiv_req())
                .field("status", &self.status())
                .field("eofseq_ie", &self.eofseq_ie())
                .field("eofseq_irq", &self.eofseq_irq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cscmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cscmdr {{ request: {:?}, clksysdiv_req: {:?}, status: {:?}, eofseq_ie: {=bool:?}, eofseq_irq: {=bool:?} }}" , self . request () , self . clksysdiv_req () , self . status () , self . eofseq_ie () , self . eofseq_irq ())
        }
    }
    #[doc = "CSR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Remove reset flag Set by software to clear the value of the reset flags. It auto clears by HW after clearing reason flags."]
        #[inline(always)]
        pub const fn rmvf(&self) -> super::vals::Rmvf {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Rmvf::from_bits(val as u8)
        }
        #[doc = "Remove reset flag Set by software to clear the value of the reset flags. It auto clears by HW after clearing reason flags."]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: super::vals::Rmvf) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "SYSTEM reset flag Reset by software by writing the RMVF bit. Set by hardware when a reset from pad occurs."]
        #[inline(always)]
        pub const fn padrstf(&self) -> super::vals::Padrstf {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Padrstf::from_bits(val as u8)
        }
        #[doc = "SYSTEM reset flag Reset by software by writing the RMVF bit. Set by hardware when a reset from pad occurs."]
        #[inline(always)]
        pub fn set_padrstf(&mut self, val: super::vals::Padrstf) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "POWER reset flag Reset by software by writing the RMVF bit. Set by hardware when a power reset occurs from LPMURESET block."]
        #[inline(always)]
        pub const fn porrstf(&self) -> super::vals::Porrstf {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::Porrstf::from_bits(val as u8)
        }
        #[doc = "POWER reset flag Reset by software by writing the RMVF bit. Set by hardware when a power reset occurs from LPMURESET block."]
        #[inline(always)]
        pub fn set_porrstf(&mut self, val: super::vals::Porrstf) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag Reset by software by writing the RMVF bit. Set by hardware when a software reset occurs."]
        #[inline(always)]
        pub const fn sftrstf(&self) -> super::vals::Sftrstf {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Sftrstf::from_bits(val as u8)
        }
        #[doc = "Software reset flag Reset by software by writing the RMVF bit. Set by hardware when a software reset occurs."]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: super::vals::Sftrstf) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "Watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a watchdog reset from V33 domain occurs."]
        #[inline(always)]
        pub const fn wdgrstf(&self) -> super::vals::Wdgrstf {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Wdgrstf::from_bits(val as u8)
        }
        #[doc = "Watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a watchdog reset from V33 domain occurs."]
        #[inline(always)]
        pub fn set_wdgrstf(&mut self, val: super::vals::Wdgrstf) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "LOCK UP reset flag from CM0 Reset by software by writing the RMVF bit. Set by hardware from unrecoverable exception CPU. It reset V12i domain, FLASH controller and peripherals."]
        #[inline(always)]
        pub const fn lockuprstf(&self) -> super::vals::Lockuprstf {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Lockuprstf::from_bits(val as u8)
        }
        #[doc = "LOCK UP reset flag from CM0 Reset by software by writing the RMVF bit. Set by hardware from unrecoverable exception CPU. It reset V12i domain, FLASH controller and peripherals."]
        #[inline(always)]
        pub fn set_lockuprstf(&mut self, val: super::vals::Lockuprstf) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("rmvf", &self.rmvf())
                .field("padrstf", &self.padrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("wdgrstf", &self.wdgrstf())
                .field("lockuprstf", &self.lockuprstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ rmvf: {:?}, padrstf: {:?}, porrstf: {:?}, sftrstf: {:?}, wdgrstf: {:?}, lockuprstf: {:?} }}",
                self.rmvf(),
                self.padrstf(),
                self.porrstf(),
                self.sftrstf(),
                self.wdgrstf(),
                self.lockuprstf()
            )
        }
    }
    #[doc = "RFHSECR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfhsecr(pub u32);
    impl Rfhsecr {
        #[doc = "RF-HSE capacitor bank tuning Set by option byte loading soon after Power On Reset."]
        #[inline(always)]
        pub const fn xotune(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "RF-HSE capacitor bank tuning Set by option byte loading soon after Power On Reset."]
        #[inline(always)]
        pub fn set_xotune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Rfhsecr {
        #[inline(always)]
        fn default() -> Rfhsecr {
            Rfhsecr(0)
        }
    }
    impl core::fmt::Debug for Rfhsecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfhsecr").field("xotune", &self.xotune()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfhsecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rfhsecr {{ xotune: {=u8:?} }}", self.xotune())
        }
    }
    #[doc = "RFSWHSECR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfswhsecr(pub u32);
    impl Rfswhsecr {
        #[doc = "Sense Amplifier threshold Set by software."]
        #[inline(always)]
        pub const fn satrg(&self) -> super::vals::Satrg {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Satrg::from_bits(val as u8)
        }
        #[doc = "Sense Amplifier threshold Set by software."]
        #[inline(always)]
        pub fn set_satrg(&mut self, val: super::vals::Satrg) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "High Speed External XO current control Set by software."]
        #[inline(always)]
        pub const fn gmc(&self) -> super::vals::Gmc {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Gmc::from_bits(val as u8)
        }
        #[doc = "High Speed External XO current control Set by software."]
        #[inline(always)]
        pub fn set_gmc(&mut self, val: super::vals::Gmc) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "RF-HSE capacitor bank tuning by SW enable Set by software."]
        #[inline(always)]
        pub const fn swxotuneen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RF-HSE capacitor bank tuning by SW enable Set by software."]
        #[inline(always)]
        pub fn set_swxotuneen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RF-HSE capacitor bank tuning value by SW Set by software."]
        #[inline(always)]
        pub const fn swxotune(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "RF-HSE capacitor bank tuning value by SW Set by software."]
        #[inline(always)]
        pub fn set_swxotune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for Rfswhsecr {
        #[inline(always)]
        fn default() -> Rfswhsecr {
            Rfswhsecr(0)
        }
    }
    impl core::fmt::Debug for Rfswhsecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfswhsecr")
                .field("satrg", &self.satrg())
                .field("gmc", &self.gmc())
                .field("swxotuneen", &self.swxotuneen())
                .field("swxotune", &self.swxotune())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfswhsecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfswhsecr {{ satrg: {:?}, gmc: {:?}, swxotuneen: {=bool:?}, swxotune: {=u8:?} }}",
                self.satrg(),
                self.gmc(),
                self.swxotuneen(),
                self.swxotune()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ccopre {
        #[doc = "CCO clock is divided by 1."]
        B_0X0 = 0x0,
        #[doc = "CCO clock is divided by 2."]
        B_0X1 = 0x01,
        #[doc = "CCO clock is divided by 4."]
        B_0X2 = 0x02,
        #[doc = "CCO clock is divided by 8."]
        B_0X3 = 0x03,
        #[doc = "CCO clock is divided by 16."]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ccopre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccopre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccopre {
        #[inline(always)]
        fn from(val: u8) -> Ccopre {
            Ccopre::from_bits(val)
        }
    }
    impl From<Ccopre> for u8 {
        #[inline(always)]
        fn from(val: Ccopre) -> u8 {
            Ccopre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clkblediv {
        #[doc = "32MHz."]
        B_0X0 = 0x0,
        #[doc = "16MHz."]
        B_0X1 = 0x01,
    }
    impl Clkblediv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clkblediv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clkblediv {
        #[inline(always)]
        fn from(val: u8) -> Clkblediv {
            Clkblediv::from_bits(val)
        }
    }
    impl From<Clkblediv> for u8 {
        #[inline(always)]
        fn from(val: Clkblediv) -> u8 {
            Clkblediv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clkslowsel {
        #[doc = "LSILMPU oscillator clock (default)."]
        B_0X0 = 0x0,
        #[doc = "LSE oscillator clock used as slow clock."]
        B_0X1 = 0x01,
        #[doc = "LSI oscillator clock used as slow clock."]
        B_0X2 = 0x02,
        #[doc = "HSI_64M divided by 2048 used as slow clock."]
        B_0X3 = 0x03,
    }
    impl Clkslowsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clkslowsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clkslowsel {
        #[inline(always)]
        fn from(val: u8) -> Clkslowsel {
            Clkslowsel::from_bits(val)
        }
    }
    impl From<Clkslowsel> for u8 {
        #[inline(always)]
        fn from(val: Clkslowsel) -> u8 {
            Clkslowsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ClksysdivReq {
        #[doc = "div 1 (sys clock 64M)."]
        B_0X0 = 0x0,
        #[doc = "div 2 (sys clock 32M)."]
        B_0X1 = 0x01,
        #[doc = "div 4 (sys clock 16M)."]
        B_0X2 = 0x02,
        #[doc = "div 8 (sys clock 8M)."]
        B_0X3 = 0x03,
        #[doc = "div 16 (sys clock 4M)."]
        B_0X4 = 0x04,
        #[doc = "div 32 (sys clock 2M)."]
        B_0X5 = 0x05,
        #[doc = "div 64 (sys clock 1M)."]
        B_0X6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl ClksysdivReq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ClksysdivReq {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ClksysdivReq {
        #[inline(always)]
        fn from(val: u8) -> ClksysdivReq {
            ClksysdivReq::from_bits(val)
        }
    }
    impl From<ClksysdivReq> for u8 {
        #[inline(always)]
        fn from(val: ClksysdivReq) -> u8 {
            ClksysdivReq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fmrat {
        #[doc = "no effect."]
        B_0X0 = 0x0,
        #[doc = "active_transmission is force to '1' whatever the HSIPLLRDY status."]
        B_0X1 = 0x01,
    }
    impl Fmrat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmrat {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmrat {
        #[inline(always)]
        fn from(val: u8) -> Fmrat {
            Fmrat::from_bits(val)
        }
    }
    impl From<Fmrat> for u8 {
        #[inline(always)]
        fn from(val: Fmrat) -> u8 {
            Fmrat::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Gmc {
        #[doc = "max 0.0 001: max 0.57 mA/V."]
        B_0X0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "max 0.78 mA/V."]
        B_0X2 = 0x02,
        #[doc = "max 1.13 mA/V (Default)."]
        B_0X3 = 0x03,
        #[doc = "max 0.61 mA/V."]
        B_0X4 = 0x04,
        #[doc = "max 1.65 mA/V."]
        B_0X5 = 0x05,
        #[doc = "max 2.12 mA/V."]
        B_0X6 = 0x06,
        #[doc = "max 2.84 mA/V."]
        B_0X7 = 0x07,
    }
    impl Gmc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Gmc {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Gmc {
        #[inline(always)]
        fn from(val: u8) -> Gmc {
            Gmc::from_bits(val)
        }
    }
    impl From<Gmc> for u8 {
        #[inline(always)]
        fn from(val: Gmc) -> u8 {
            Gmc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hserdy {
        #[doc = "HSE oscillator not ready."]
        B_0X0 = 0x0,
        #[doc = "HSE oscillator ready."]
        B_0X1 = 0x01,
    }
    impl Hserdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hserdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hserdy {
        #[inline(always)]
        fn from(val: u8) -> Hserdy {
            Hserdy::from_bits(val)
        }
    }
    impl From<Hserdy> for u8 {
        #[inline(always)]
        fn from(val: Hserdy) -> u8 {
            Hserdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hserdyif {
        #[doc = "No clock ready interrupt caused by the HSE oscillator."]
        B_0X0 = 0x0,
        #[doc = "Clock ready interrupt caused by the HSE oscillator."]
        B_0X1 = 0x01,
    }
    impl Hserdyif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hserdyif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hserdyif {
        #[inline(always)]
        fn from(val: u8) -> Hserdyif {
            Hserdyif::from_bits(val)
        }
    }
    impl From<Hserdyif> for u8 {
        #[inline(always)]
        fn from(val: Hserdyif) -> u8 {
            Hserdyif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsesel {
        #[doc = "HSI clock source is requested (default)."]
        B_0X0 = 0x0,
        #[doc = "HSE clock source is requested."]
        B_0X1 = 0x01,
    }
    impl Hsesel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsesel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsesel {
        #[inline(always)]
        fn from(val: u8) -> Hsesel {
            Hsesel::from_bits(val)
        }
    }
    impl From<Hsesel> for u8 {
        #[inline(always)]
        fn from(val: Hsesel) -> u8 {
            Hsesel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HseselStatus {
        #[doc = "HSI clock source is requested (default)."]
        B_0X0 = 0x0,
        #[doc = "HSE clock source is requested."]
        B_0X1 = 0x01,
    }
    impl HseselStatus {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HseselStatus {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HseselStatus {
        #[inline(always)]
        fn from(val: u8) -> HseselStatus {
            HseselStatus::from_bits(val)
        }
    }
    impl From<HseselStatus> for u8 {
        #[inline(always)]
        fn from(val: HseselStatus) -> u8 {
            HseselStatus::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsipllrdy {
        #[doc = "PLL is unlocked."]
        B_0X0 = 0x0,
        #[doc = "PLL is locked."]
        B_0X1 = 0x01,
    }
    impl Hsipllrdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsipllrdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsipllrdy {
        #[inline(always)]
        fn from(val: u8) -> Hsipllrdy {
            Hsipllrdy::from_bits(val)
        }
    }
    impl From<Hsipllrdy> for u8 {
        #[inline(always)]
        fn from(val: Hsipllrdy) -> u8 {
            Hsipllrdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsipllrdyif {
        #[doc = "No clock ready interrupt caused by the HSI PLL64 MHz oscillator."]
        B_0X0 = 0x0,
        #[doc = "Clock ready interrupt caused by the HSI PLL64 MHz oscillator."]
        B_0X1 = 0x01,
    }
    impl Hsipllrdyif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsipllrdyif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsipllrdyif {
        #[inline(always)]
        fn from(val: u8) -> Hsipllrdyif {
            Hsipllrdyif::from_bits(val)
        }
    }
    impl From<Hsipllrdyif> for u8 {
        #[inline(always)]
        fn from(val: Hsipllrdyif) -> u8 {
            Hsipllrdyif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsirdy {
        #[doc = "internal RC 64 MHz oscillator not ready."]
        B_0X0 = 0x0,
        #[doc = "internal RC 64 MHz oscillator ready."]
        B_0X1 = 0x01,
    }
    impl Hsirdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsirdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsirdy {
        #[inline(always)]
        fn from(val: u8) -> Hsirdy {
            Hsirdy::from_bits(val)
        }
    }
    impl From<Hsirdy> for u8 {
        #[inline(always)]
        fn from(val: Hsirdy) -> u8 {
            Hsirdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsirdyif {
        #[doc = "No clock ready interrupt caused by the HSI oscillator."]
        B_0X0 = 0x0,
        #[doc = "Clock ready interrupt caused by the HSI oscillator."]
        B_0X1 = 0x01,
    }
    impl Hsirdyif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsirdyif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsirdyif {
        #[inline(always)]
        fn from(val: u8) -> Hsirdyif {
            Hsirdyif::from_bits(val)
        }
    }
    impl From<Hsirdyif> for u8 {
        #[inline(always)]
        fn from(val: Hsirdyif) -> u8 {
            Hsirdyif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lcosel {
        #[doc = "LCO output disabled, no clock on LCO."]
        B_0X0 = 0x0,
        #[doc = "internal 32 KHz (LSI_LPMU) oscillator clock selected."]
        B_0X1 = 0x01,
        #[doc = "internal 32 KHz (LSI) oscillator clock selected."]
        B_0X2 = 0x02,
        #[doc = "external 32 KHz (LSE) oscillator clock selected."]
        B_0X3 = 0x03,
    }
    impl Lcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lcosel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lcosel {
        #[inline(always)]
        fn from(val: u8) -> Lcosel {
            Lcosel::from_bits(val)
        }
    }
    impl From<Lcosel> for u8 {
        #[inline(always)]
        fn from(val: Lcosel) -> u8 {
            Lcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lockuprstf {
        #[doc = "No lockup reset occurred."]
        B_0X0 = 0x0,
        #[doc = "lockup reset occurred."]
        B_0X1 = 0x01,
    }
    impl Lockuprstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lockuprstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lockuprstf {
        #[inline(always)]
        fn from(val: u8) -> Lockuprstf {
            Lockuprstf::from_bits(val)
        }
    }
    impl From<Lockuprstf> for u8 {
        #[inline(always)]
        fn from(val: Lockuprstf) -> u8 {
            Lockuprstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpuclksel {
        #[doc = "16MHz peripheral clock (default)."]
        B_0X0 = 0x0,
        #[doc = "LSE clock."]
        B_0X1 = 0x01,
    }
    impl Lpuclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpuclksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpuclksel {
        #[inline(always)]
        fn from(val: u8) -> Lpuclksel {
            Lpuclksel::from_bits(val)
        }
    }
    impl From<Lpuclksel> for u8 {
        #[inline(always)]
        fn from(val: Lpuclksel) -> u8 {
            Lpuclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpurstf {
        #[doc = "no LPUART reset release event occurred."]
        B_0X0 = 0x0,
        #[doc = "LPUART reset release event occurred."]
        B_0X1 = 0x01,
    }
    impl Lpurstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpurstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpurstf {
        #[inline(always)]
        fn from(val: u8) -> Lpurstf {
            Lpurstf::from_bits(val)
        }
    }
    impl From<Lpurstf> for u8 {
        #[inline(always)]
        fn from(val: Lpurstf) -> u8 {
            Lpurstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsebyp {
        #[doc = "LSE oscillator bypass OFF."]
        B_0X0 = 0x0,
        #[doc = "LSE oscillator bypass ON."]
        B_0X1 = 0x01,
    }
    impl Lsebyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsebyp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsebyp {
        #[inline(always)]
        fn from(val: u8) -> Lsebyp {
            Lsebyp::from_bits(val)
        }
    }
    impl From<Lsebyp> for u8 {
        #[inline(always)]
        fn from(val: Lsebyp) -> u8 {
            Lsebyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lserdy {
        #[doc = "LSE oscillator not ready."]
        B_0X0 = 0x0,
        #[doc = "LSE oscillator ready."]
        B_0X1 = 0x01,
    }
    impl Lserdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lserdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lserdy {
        #[inline(always)]
        fn from(val: u8) -> Lserdy {
            Lserdy::from_bits(val)
        }
    }
    impl From<Lserdy> for u8 {
        #[inline(always)]
        fn from(val: Lserdy) -> u8 {
            Lserdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lserdyif {
        #[doc = "No clock ready interrupt caused by the LSE oscillator."]
        B_0X0 = 0x0,
        #[doc = "Clock ready interrupt caused by the LSE oscillator."]
        B_0X1 = 0x01,
    }
    impl Lserdyif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lserdyif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lserdyif {
        #[inline(always)]
        fn from(val: u8) -> Lserdyif {
            Lserdyif::from_bits(val)
        }
    }
    impl From<Lserdyif> for u8 {
        #[inline(always)]
        fn from(val: Lserdyif) -> u8 {
            Lserdyif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsirdy {
        #[doc = "LSI RC oscillator not ready."]
        B_0X0 = 0x0,
        #[doc = "LSI RC oscillator ready."]
        B_0X1 = 0x01,
    }
    impl Lsirdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsirdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsirdy {
        #[inline(always)]
        fn from(val: u8) -> Lsirdy {
            Lsirdy::from_bits(val)
        }
    }
    impl From<Lsirdy> for u8 {
        #[inline(always)]
        fn from(val: Lsirdy) -> u8 {
            Lsirdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsirdyif {
        #[doc = "No clock ready interrupt caused by the internal RC 32 KHz oscillator."]
        B_0X0 = 0x0,
        #[doc = "Clock ready interrupt caused by the internal RC 32 kHz oscillator."]
        B_0X1 = 0x01,
    }
    impl Lsirdyif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsirdyif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsirdyif {
        #[inline(always)]
        fn from(val: u8) -> Lsirdyif {
            Lsirdyif::from_bits(val)
        }
    }
    impl From<Lsirdyif> for u8 {
        #[inline(always)]
        fn from(val: Lsirdyif) -> u8 {
            Lsirdyif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcosel {
        #[doc = "MCO output disabled, no clock on MCO."]
        B_0X0 = 0x0,
        #[doc = "system clock selected."]
        B_0X1 = 0x01,
        #[doc = "na."]
        B_0X2 = 0x02,
        #[doc = "internal RC 64 MHz (HSI) oscillator clock selected."]
        B_0X3 = 0x03,
        #[doc = "external oscillator (HSE) clock selected."]
        B_0X4 = 0x04,
        #[doc = "internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected."]
        B_0X5 = 0x05,
        #[doc = "SMPS clock selected."]
        B_0X6 = 0x06,
        #[doc = "AUX ADC ANA clock selected."]
        B_0X7 = 0x07,
    }
    impl Mcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcosel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcosel {
        #[inline(always)]
        fn from(val: u8) -> Mcosel {
            Mcosel::from_bits(val)
        }
    }
    impl From<Mcosel> for u8 {
        #[inline(always)]
        fn from(val: Mcosel) -> u8 {
            Mcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Padrstf {
        #[doc = "No reset from pad occurred."]
        B_0X0 = 0x0,
        #[doc = "Reset from pad occurred."]
        B_0X1 = 0x01,
    }
    impl Padrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Padrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Padrstf {
        #[inline(always)]
        fn from(val: u8) -> Padrstf {
            Padrstf::from_bits(val)
        }
    }
    impl From<Padrstf> for u8 {
        #[inline(always)]
        fn from(val: Padrstf) -> u8 {
            Padrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Porrstf {
        #[doc = "No POWER reset occurred."]
        B_0X0 = 0x0,
        #[doc = "POWER reset occurred."]
        B_0X1 = 0x01,
    }
    impl Porrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Porrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Porrstf {
        #[inline(always)]
        fn from(val: u8) -> Porrstf {
            Porrstf::from_bits(val)
        }
    }
    impl From<Porrstf> for u8 {
        #[inline(always)]
        fn from(val: Porrstf) -> u8 {
            Porrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Request {
        #[doc = "To cancel an ongiong request - still possible until IRQ assertion."]
        B_0X0 = 0x0,
        #[doc = "To update the system clock frequency."]
        B_0X1 = 0x01,
    }
    impl Request {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Request {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Request {
        #[inline(always)]
        fn from(val: u8) -> Request {
            Request::from_bits(val)
        }
    }
    impl From<Request> for u8 {
        #[inline(always)]
        fn from(val: Request) -> u8 {
            Request::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rmvf {
        #[doc = "Nothing done."]
        B_0X0 = 0x0,
        #[doc = "Reset the value of the reset flags."]
        B_0X1 = 0x01,
    }
    impl Rmvf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rmvf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rmvf {
        #[inline(always)]
        fn from(val: u8) -> Rmvf {
            Rmvf::from_bits(val)
        }
    }
    impl From<Rmvf> for u8 {
        #[inline(always)]
        fn from(val: Rmvf) -> u8 {
            Rmvf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Satrg {
        #[doc = "the bias current is confronted to a reference current with a ratio of 1/2."]
        B_0X0 = 0x0,
        #[doc = "the bias current is confronted to a reference current with a ratio of 3/4."]
        B_0X1 = 0x01,
    }
    impl Satrg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Satrg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Satrg {
        #[inline(always)]
        fn from(val: u8) -> Satrg {
            Satrg::from_bits(val)
        }
    }
    impl From<Satrg> for u8 {
        #[inline(always)]
        fn from(val: Satrg) -> u8 {
            Satrg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sftrstf {
        #[doc = "No software reset occurred."]
        B_0X0 = 0x0,
        #[doc = "Software reset occurred."]
        B_0X1 = 0x01,
    }
    impl Sftrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sftrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sftrstf {
        #[inline(always)]
        fn from(val: u8) -> Sftrstf {
            Sftrstf::from_bits(val)
        }
    }
    impl From<Sftrstf> for u8 {
        #[inline(always)]
        fn from(val: Sftrstf) -> u8 {
            Sftrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpsdiv {
        #[doc = "div 2 when ANADIV=2 or 4 (default )."]
        B_0X0 = 0x0,
        #[doc = "div 4 when ANADIV=1 or 2."]
        B_0X1 = 0x01,
    }
    impl Smpsdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsdiv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsdiv {
        #[inline(always)]
        fn from(val: u8) -> Smpsdiv {
            Smpsdiv::from_bits(val)
        }
    }
    impl From<Smpsdiv> for u8 {
        #[inline(always)]
        fn from(val: Smpsdiv) -> u8 {
            Smpsdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpsinv {
        #[doc = "SMPS clock not inverted (default value)."]
        B_0X0 = 0x0,
        #[doc = "SMPS clock inverted (for debug)."]
        B_0X1 = 0x01,
    }
    impl Smpsinv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsinv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsinv {
        #[inline(always)]
        fn from(val: u8) -> Smpsinv {
            Smpsinv::from_bits(val)
        }
    }
    impl From<Smpsinv> for u8 {
        #[inline(always)]
        fn from(val: Smpsinv) -> u8 {
            Smpsinv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spiisclksel {
        #[doc = "16MHz peripheral clock (default)."]
        B_0X0 = 0x0,
        #[doc = "32MHz peripheral clock."]
        B_0X1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Spiisclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spiisclksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spiisclksel {
        #[inline(always)]
        fn from(val: u8) -> Spiisclksel {
            Spiisclksel::from_bits(val)
        }
    }
    impl From<Spiisclksel> for u8 {
        #[inline(always)]
        fn from(val: Spiisclksel) -> u8 {
            Spiisclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Status {
        #[doc = "IDLE no switch requested."]
        B_0X0 = 0x0,
        #[doc = "ONGOING clock frequency switch is ongoing."]
        B_0X1 = 0x01,
        #[doc = "DONE clock frequency switch done."]
        B_0X2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Status {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Status {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Status {
        #[inline(always)]
        fn from(val: u8) -> Status {
            Status::from_bits(val)
        }
    }
    impl From<Status> for u8 {
        #[inline(always)]
        fn from(val: Status) -> u8 {
            Status::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stophsi {
        #[doc = "HSI is enabled (default)."]
        B_0X0 = 0x0,
        #[doc = "disable HSI is requested."]
        B_0X1 = 0x01,
    }
    impl Stophsi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stophsi {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stophsi {
        #[inline(always)]
        fn from(val: u8) -> Stophsi {
            Stophsi::from_bits(val)
        }
    }
    impl From<Stophsi> for u8 {
        #[inline(always)]
        fn from(val: Stophsi) -> u8 {
            Stophsi::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wdgrstf {
        #[doc = "No watchdog reset occurred."]
        B_0X0 = 0x0,
        #[doc = "Watchdog reset occurred."]
        B_0X1 = 0x01,
    }
    impl Wdgrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wdgrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wdgrstf {
        #[inline(always)]
        fn from(val: u8) -> Wdgrstf {
            Wdgrstf::from_bits(val)
        }
    }
    impl From<Wdgrstf> for u8 {
        #[inline(always)]
        fn from(val: Wdgrstf) -> u8 {
            Wdgrstf::to_bits(val)
        }
    }
}
