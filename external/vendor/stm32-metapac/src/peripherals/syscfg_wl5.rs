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
    #[doc = "memory remap register"]
    #[inline(always)]
    pub const fn memrmp(self) -> crate::common::Reg<regs::Memrmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "SCSR"]
    #[inline(always)]
    pub const fn scsr(self) -> crate::common::Reg<regs::Scsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SWPR"]
    #[inline(always)]
    pub const fn swpr(self) -> crate::common::Reg<regs::Swpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SKR"]
    #[inline(always)]
    pub const fn skr(self) -> crate::common::Reg<regs::Skr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SYSCFG CPU1 interrupt mask register 1"]
    #[inline(always)]
    pub const fn imr1(self) -> crate::common::Reg<regs::Imr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "SYSCFG CPU1 interrupt mask register 2"]
    #[inline(always)]
    pub const fn imr2(self) -> crate::common::Reg<regs::Imr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "SYSCFG CPU2 interrupt mask register 1"]
    #[inline(always)]
    pub const fn c2imr1(self) -> crate::common::Reg<regs::C2imr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "SYSCFG CPU2 interrupt mask register 2"]
    #[inline(always)]
    pub const fn c2imr2(self) -> crate::common::Reg<regs::C2imr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "radio debug control register"]
    #[inline(always)]
    pub const fn rfdcr(self) -> crate::common::Reg<regs::Rfdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
}
pub mod regs {
    #[doc = "SYSCFG CPU2 interrupt mask register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2imr1(pub u32);
    impl C2imr1 {
        #[doc = "RTCSTAMPTAMPLSECSSIM"]
        #[inline(always)]
        pub const fn rtcstamptamplsecssim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RTCSTAMPTAMPLSECSSIM"]
        #[inline(always)]
        pub fn set_rtcstamptamplsecssim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTCALARMIM"]
        #[inline(always)]
        pub const fn rtcalarmim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTCALARMIM"]
        #[inline(always)]
        pub fn set_rtcalarmim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RTCSSRUIM"]
        #[inline(always)]
        pub const fn rtcssruim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RTCSSRUIM"]
        #[inline(always)]
        pub fn set_rtcssruim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RTCWKUPIM"]
        #[inline(always)]
        pub const fn rtcwkupim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RTCWKUPIM"]
        #[inline(always)]
        pub fn set_rtcwkupim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RCCIM"]
        #[inline(always)]
        pub const fn rccim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RCCIM"]
        #[inline(always)]
        pub fn set_rccim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FLASHIM"]
        #[inline(always)]
        pub const fn flashim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FLASHIM"]
        #[inline(always)]
        pub fn set_flashim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PKAIM"]
        #[inline(always)]
        pub const fn pkaim(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PKAIM"]
        #[inline(always)]
        pub fn set_pkaim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "AESIM"]
        #[inline(always)]
        pub const fn aesim(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "AESIM"]
        #[inline(always)]
        pub fn set_aesim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "COMPIM"]
        #[inline(always)]
        pub const fn compim(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "COMPIM"]
        #[inline(always)]
        pub fn set_compim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ADCIM"]
        #[inline(always)]
        pub const fn adcim(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ADCIM"]
        #[inline(always)]
        pub fn set_adcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DACIM"]
        #[inline(always)]
        pub const fn dacim(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DACIM"]
        #[inline(always)]
        pub fn set_dacim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "EXTI0IM"]
        #[inline(always)]
        pub const fn exti0im(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI0IM"]
        #[inline(always)]
        pub fn set_exti0im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "EXTI1IM"]
        #[inline(always)]
        pub const fn exti1im(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI1IM"]
        #[inline(always)]
        pub fn set_exti1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EXTI2IM"]
        #[inline(always)]
        pub const fn exti2im(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI2IM"]
        #[inline(always)]
        pub fn set_exti2im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "EXTI3IM"]
        #[inline(always)]
        pub const fn exti3im(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI3IM"]
        #[inline(always)]
        pub fn set_exti3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "EXTI4IM"]
        #[inline(always)]
        pub const fn exti4im(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI4IM"]
        #[inline(always)]
        pub fn set_exti4im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "EXTI5IM"]
        #[inline(always)]
        pub const fn exti5im(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI5IM"]
        #[inline(always)]
        pub fn set_exti5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "EXTI6IM"]
        #[inline(always)]
        pub const fn exti6im(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI6IM"]
        #[inline(always)]
        pub fn set_exti6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "EXTI7IM"]
        #[inline(always)]
        pub const fn exti7im(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI7IM"]
        #[inline(always)]
        pub fn set_exti7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "EXTI8IM"]
        #[inline(always)]
        pub const fn exti8im(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI8IM"]
        #[inline(always)]
        pub fn set_exti8im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "EXTI9IM"]
        #[inline(always)]
        pub const fn exti9im(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI9IM"]
        #[inline(always)]
        pub fn set_exti9im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "EXTI10IM"]
        #[inline(always)]
        pub const fn exti10im(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI10IM"]
        #[inline(always)]
        pub fn set_exti10im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "EXTI11IM"]
        #[inline(always)]
        pub const fn exti11im(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI11IM"]
        #[inline(always)]
        pub fn set_exti11im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "EXTI12IM"]
        #[inline(always)]
        pub const fn exti12im(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI12IM"]
        #[inline(always)]
        pub fn set_exti12im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "EXTI13IM"]
        #[inline(always)]
        pub const fn exti13im(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI13IM"]
        #[inline(always)]
        pub fn set_exti13im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "EXTI14IM"]
        #[inline(always)]
        pub const fn exti14im(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI14IM"]
        #[inline(always)]
        pub fn set_exti14im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "EXTI15IM"]
        #[inline(always)]
        pub const fn exti15im(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI15IM"]
        #[inline(always)]
        pub fn set_exti15im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C2imr1 {
        #[inline(always)]
        fn default() -> C2imr1 {
            C2imr1(0)
        }
    }
    impl core::fmt::Debug for C2imr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C2imr1")
                .field("rtcstamptamplsecssim", &self.rtcstamptamplsecssim())
                .field("rtcalarmim", &self.rtcalarmim())
                .field("rtcssruim", &self.rtcssruim())
                .field("rtcwkupim", &self.rtcwkupim())
                .field("rccim", &self.rccim())
                .field("flashim", &self.flashim())
                .field("pkaim", &self.pkaim())
                .field("aesim", &self.aesim())
                .field("compim", &self.compim())
                .field("adcim", &self.adcim())
                .field("dacim", &self.dacim())
                .field("exti0im", &self.exti0im())
                .field("exti1im", &self.exti1im())
                .field("exti2im", &self.exti2im())
                .field("exti3im", &self.exti3im())
                .field("exti4im", &self.exti4im())
                .field("exti5im", &self.exti5im())
                .field("exti6im", &self.exti6im())
                .field("exti7im", &self.exti7im())
                .field("exti8im", &self.exti8im())
                .field("exti9im", &self.exti9im())
                .field("exti10im", &self.exti10im())
                .field("exti11im", &self.exti11im())
                .field("exti12im", &self.exti12im())
                .field("exti13im", &self.exti13im())
                .field("exti14im", &self.exti14im())
                .field("exti15im", &self.exti15im())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C2imr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "C2imr1 {{ rtcstamptamplsecssim: {=bool:?}, rtcalarmim: {=bool:?}, rtcssruim: {=bool:?}, rtcwkupim: {=bool:?}, rccim: {=bool:?}, flashim: {=bool:?}, pkaim: {=bool:?}, aesim: {=bool:?}, compim: {=bool:?}, adcim: {=bool:?}, dacim: {=bool:?}, exti0im: {=bool:?}, exti1im: {=bool:?}, exti2im: {=bool:?}, exti3im: {=bool:?}, exti4im: {=bool:?}, exti5im: {=bool:?}, exti6im: {=bool:?}, exti7im: {=bool:?}, exti8im: {=bool:?}, exti9im: {=bool:?}, exti10im: {=bool:?}, exti11im: {=bool:?}, exti12im: {=bool:?}, exti13im: {=bool:?}, exti14im: {=bool:?}, exti15im: {=bool:?} }}" , self . rtcstamptamplsecssim () , self . rtcalarmim () , self . rtcssruim () , self . rtcwkupim () , self . rccim () , self . flashim () , self . pkaim () , self . aesim () , self . compim () , self . adcim () , self . dacim () , self . exti0im () , self . exti1im () , self . exti2im () , self . exti3im () , self . exti4im () , self . exti5im () , self . exti6im () , self . exti7im () , self . exti8im () , self . exti9im () , self . exti10im () , self . exti11im () , self . exti12im () , self . exti13im () , self . exti14im () , self . exti15im ())
        }
    }
    #[doc = "SYSCFG CPU2 interrupt mask register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2imr2(pub u32);
    impl C2imr2 {
        #[doc = "DMA1CH1IM"]
        #[inline(always)]
        pub const fn dma1ch1im(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH1IM"]
        #[inline(always)]
        pub fn set_dma1ch1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA1CH2IM"]
        #[inline(always)]
        pub const fn dma1ch2im(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH2IM"]
        #[inline(always)]
        pub fn set_dma1ch2im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA1CH3IM"]
        #[inline(always)]
        pub const fn dma1ch3im(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH3IM"]
        #[inline(always)]
        pub fn set_dma1ch3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA1CH4IM"]
        #[inline(always)]
        pub const fn dma1ch4im(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH4IM"]
        #[inline(always)]
        pub fn set_dma1ch4im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA1CH5IM"]
        #[inline(always)]
        pub const fn dma1ch5im(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH5IM"]
        #[inline(always)]
        pub fn set_dma1ch5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA1CH6IM"]
        #[inline(always)]
        pub const fn dma1ch6im(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH6IM"]
        #[inline(always)]
        pub fn set_dma1ch6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DMA1CH7IM"]
        #[inline(always)]
        pub const fn dma1ch7im(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH7IM"]
        #[inline(always)]
        pub fn set_dma1ch7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA2CH1IM"]
        #[inline(always)]
        pub const fn dma2ch1im(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH1IM"]
        #[inline(always)]
        pub fn set_dma2ch1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DMA2CH2IM"]
        #[inline(always)]
        pub const fn dma2ch2im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH2IM"]
        #[inline(always)]
        pub fn set_dma2ch2im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DMA2CH3IM"]
        #[inline(always)]
        pub const fn dma2ch3im(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH3IM"]
        #[inline(always)]
        pub fn set_dma2ch3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DMA2CH4IM"]
        #[inline(always)]
        pub const fn dma2ch4im(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH4IM"]
        #[inline(always)]
        pub fn set_dma2ch4im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "DMA2CH5IM"]
        #[inline(always)]
        pub const fn dma2ch5im(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH5IM"]
        #[inline(always)]
        pub fn set_dma2ch5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA2CH6IM"]
        #[inline(always)]
        pub const fn dma2ch6im(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH6IM"]
        #[inline(always)]
        pub fn set_dma2ch6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DMA2CH7IM"]
        #[inline(always)]
        pub const fn dma2ch7im(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH7IM"]
        #[inline(always)]
        pub fn set_dma2ch7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DMAMUX1IM"]
        #[inline(always)]
        pub const fn dmamux1im(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX1IM"]
        #[inline(always)]
        pub fn set_dmamux1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PVM3IM"]
        #[inline(always)]
        pub const fn pvm3im(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PVM3IM"]
        #[inline(always)]
        pub fn set_pvm3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PVDIM"]
        #[inline(always)]
        pub const fn pvdim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PVDIM"]
        #[inline(always)]
        pub fn set_pvdim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for C2imr2 {
        #[inline(always)]
        fn default() -> C2imr2 {
            C2imr2(0)
        }
    }
    impl core::fmt::Debug for C2imr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C2imr2")
                .field("dma1ch1im", &self.dma1ch1im())
                .field("dma1ch2im", &self.dma1ch2im())
                .field("dma1ch3im", &self.dma1ch3im())
                .field("dma1ch4im", &self.dma1ch4im())
                .field("dma1ch5im", &self.dma1ch5im())
                .field("dma1ch6im", &self.dma1ch6im())
                .field("dma1ch7im", &self.dma1ch7im())
                .field("dma2ch1im", &self.dma2ch1im())
                .field("dma2ch2im", &self.dma2ch2im())
                .field("dma2ch3im", &self.dma2ch3im())
                .field("dma2ch4im", &self.dma2ch4im())
                .field("dma2ch5im", &self.dma2ch5im())
                .field("dma2ch6im", &self.dma2ch6im())
                .field("dma2ch7im", &self.dma2ch7im())
                .field("dmamux1im", &self.dmamux1im())
                .field("pvm3im", &self.pvm3im())
                .field("pvdim", &self.pvdim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C2imr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "C2imr2 {{ dma1ch1im: {=bool:?}, dma1ch2im: {=bool:?}, dma1ch3im: {=bool:?}, dma1ch4im: {=bool:?}, dma1ch5im: {=bool:?}, dma1ch6im: {=bool:?}, dma1ch7im: {=bool:?}, dma2ch1im: {=bool:?}, dma2ch2im: {=bool:?}, dma2ch3im: {=bool:?}, dma2ch4im: {=bool:?}, dma2ch5im: {=bool:?}, dma2ch6im: {=bool:?}, dma2ch7im: {=bool:?}, dmamux1im: {=bool:?}, pvm3im: {=bool:?}, pvdim: {=bool:?} }}" , self . dma1ch1im () , self . dma1ch2im () , self . dma1ch3im () , self . dma1ch4im () , self . dma1ch5im () , self . dma1ch6im () , self . dma1ch7im () , self . dma2ch1im () , self . dma2ch2im () , self . dma2ch3im () , self . dma2ch4im () , self . dma2ch5im () , self . dma2ch6im () , self . dma2ch7im () , self . dmamux1im () , self . pvm3im () , self . pvdim ())
        }
    }
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "I/O analog switch voltage booster enable"]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O analog switch voltage booster enable"]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6"]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6"]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7"]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7"]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8"]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8"]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9"]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9"]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I2C1 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C2 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C3 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c3_fmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c3_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("boosten", &self.boosten())
                .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
                .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
                .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
                .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
                .field("i2c1_fmp", &self.i2c1_fmp())
                .field("i2c2_fmp", &self.i2c2_fmp())
                .field("i2c3_fmp", &self.i2c3_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ boosten: {=bool:?}, i2c_pb6_fmp: {=bool:?}, i2c_pb7_fmp: {=bool:?}, i2c_pb8_fmp: {=bool:?}, i2c_pb9_fmp: {=bool:?}, i2c1_fmp: {=bool:?}, i2c2_fmp: {=bool:?}, i2c3_fmp: {=bool:?} }}" , self . boosten () , self . i2c_pb6_fmp () , self . i2c_pb7_fmp () , self . i2c_pb8_fmp () , self . i2c_pb9_fmp () , self . i2c1_fmp () , self . i2c2_fmp () , self . i2c3_fmp ())
        }
    }
    #[doc = "CFGR2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "CPU1 LOCKUP (Hardfault) output enable bit"]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CPU1 LOCKUP (Hardfault) output enable bit"]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 parity lock bit"]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity lock bit"]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC Lock"]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Lock"]
        #[inline(always)]
        pub fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SRAM2 parity error flag"]
        #[inline(always)]
        pub const fn spf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity error flag"]
        #[inline(always)]
        pub fn set_spf(&mut self, val: bool) {
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
                .field("cll", &self.cll())
                .field("spl", &self.spl())
                .field("pvdl", &self.pvdl())
                .field("eccl", &self.eccl())
                .field("spf", &self.spf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ cll: {=bool:?}, spl: {=bool:?}, pvdl: {=bool:?}, eccl: {=bool:?}, spf: {=bool:?} }}",
                self.cll(),
                self.spl(),
                self.pvdl(),
                self.eccl(),
                self.spf()
            )
        }
    }
    #[doc = "external interrupt configuration register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI12 configuration bits"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "EXTI12 configuration bits"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
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
    #[doc = "SYSCFG CPU1 interrupt mask register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr1(pub u32);
    impl Imr1 {
        #[doc = "RTCSTAMPTAMPLSECSSIM"]
        #[inline(always)]
        pub const fn rtcstamptamplsecssim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RTCSTAMPTAMPLSECSSIM"]
        #[inline(always)]
        pub fn set_rtcstamptamplsecssim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTCSSRUIM"]
        #[inline(always)]
        pub const fn rtcssruim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RTCSSRUIM"]
        #[inline(always)]
        pub fn set_rtcssruim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EXTI5IM"]
        #[inline(always)]
        pub const fn exti5im(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI5IM"]
        #[inline(always)]
        pub fn set_exti5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "EXTI6IM"]
        #[inline(always)]
        pub const fn exti6im(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI6IM"]
        #[inline(always)]
        pub fn set_exti6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "EXTI7IM"]
        #[inline(always)]
        pub const fn exti7im(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI7IM"]
        #[inline(always)]
        pub fn set_exti7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "EXTI8IM"]
        #[inline(always)]
        pub const fn exti8im(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI8IM"]
        #[inline(always)]
        pub fn set_exti8im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "EXTI9IM"]
        #[inline(always)]
        pub const fn exti9im(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI9IM"]
        #[inline(always)]
        pub fn set_exti9im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "EXTI10IM"]
        #[inline(always)]
        pub const fn exti10im(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI10IM"]
        #[inline(always)]
        pub fn set_exti10im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "EXTI11IM"]
        #[inline(always)]
        pub const fn exti11im(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI11IM"]
        #[inline(always)]
        pub fn set_exti11im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "EXTI12IM"]
        #[inline(always)]
        pub const fn exti12im(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI12IM"]
        #[inline(always)]
        pub fn set_exti12im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "EXTI13IM"]
        #[inline(always)]
        pub const fn exti13im(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI13IM"]
        #[inline(always)]
        pub fn set_exti13im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "EXTI14IM"]
        #[inline(always)]
        pub const fn exti14im(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI14IM"]
        #[inline(always)]
        pub fn set_exti14im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "EXTI15IM"]
        #[inline(always)]
        pub const fn exti15im(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI15IM"]
        #[inline(always)]
        pub fn set_exti15im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Imr1 {
        #[inline(always)]
        fn default() -> Imr1 {
            Imr1(0)
        }
    }
    impl core::fmt::Debug for Imr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imr1")
                .field("rtcstamptamplsecssim", &self.rtcstamptamplsecssim())
                .field("rtcssruim", &self.rtcssruim())
                .field("exti5im", &self.exti5im())
                .field("exti6im", &self.exti6im())
                .field("exti7im", &self.exti7im())
                .field("exti8im", &self.exti8im())
                .field("exti9im", &self.exti9im())
                .field("exti10im", &self.exti10im())
                .field("exti11im", &self.exti11im())
                .field("exti12im", &self.exti12im())
                .field("exti13im", &self.exti13im())
                .field("exti14im", &self.exti14im())
                .field("exti15im", &self.exti15im())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Imr1 {{ rtcstamptamplsecssim: {=bool:?}, rtcssruim: {=bool:?}, exti5im: {=bool:?}, exti6im: {=bool:?}, exti7im: {=bool:?}, exti8im: {=bool:?}, exti9im: {=bool:?}, exti10im: {=bool:?}, exti11im: {=bool:?}, exti12im: {=bool:?}, exti13im: {=bool:?}, exti14im: {=bool:?}, exti15im: {=bool:?} }}" , self . rtcstamptamplsecssim () , self . rtcssruim () , self . exti5im () , self . exti6im () , self . exti7im () , self . exti8im () , self . exti9im () , self . exti10im () , self . exti11im () , self . exti12im () , self . exti13im () , self . exti14im () , self . exti15im ())
        }
    }
    #[doc = "SYSCFG CPU1 interrupt mask register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr2(pub u32);
    impl Imr2 {
        #[doc = "PVM3IM"]
        #[inline(always)]
        pub const fn pvm3im(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PVM3IM"]
        #[inline(always)]
        pub fn set_pvm3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PVDIM"]
        #[inline(always)]
        pub const fn pvdim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PVDIM"]
        #[inline(always)]
        pub fn set_pvdim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Imr2 {
        #[inline(always)]
        fn default() -> Imr2 {
            Imr2(0)
        }
    }
    impl core::fmt::Debug for Imr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imr2")
                .field("pvm3im", &self.pvm3im())
                .field("pvdim", &self.pvdim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Imr2 {{ pvm3im: {=bool:?}, pvdim: {=bool:?} }}",
                self.pvm3im(),
                self.pvdim()
            )
        }
    }
    #[doc = "memory remap register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memrmp(pub u32);
    impl Memrmp {
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Memrmp {
        #[inline(always)]
        fn default() -> Memrmp {
            Memrmp(0)
        }
    }
    impl core::fmt::Debug for Memrmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Memrmp").field("mem_mode", &self.mem_mode()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Memrmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Memrmp {{ mem_mode: {=u8:?} }}", self.mem_mode())
        }
    }
    #[doc = "radio debug control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfdcr(pub u32);
    impl Rfdcr {
        #[doc = "radio debug test bus selection"]
        #[inline(always)]
        pub const fn rftbsel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "radio debug test bus selection"]
        #[inline(always)]
        pub fn set_rftbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Rfdcr {
        #[inline(always)]
        fn default() -> Rfdcr {
            Rfdcr(0)
        }
    }
    impl core::fmt::Debug for Rfdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfdcr").field("rftbsel", &self.rftbsel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rfdcr {{ rftbsel: {=bool:?} }}", self.rftbsel())
        }
    }
    #[doc = "SCSR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scsr(pub u32);
    impl Scsr {
        #[doc = "SRAM2 erase"]
        #[inline(always)]
        pub const fn sram2er(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase"]
        #[inline(always)]
        pub fn set_sram2er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
        #[inline(always)]
        pub const fn srambsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
        #[inline(always)]
        pub fn set_srambsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PKA SRAM busy by erase operation"]
        #[inline(always)]
        pub const fn pkasrambsy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PKA SRAM busy by erase operation"]
        #[inline(always)]
        pub fn set_pkasrambsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Scsr {
        #[inline(always)]
        fn default() -> Scsr {
            Scsr(0)
        }
    }
    impl core::fmt::Debug for Scsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scsr")
                .field("sram2er", &self.sram2er())
                .field("srambsy", &self.srambsy())
                .field("pkasrambsy", &self.pkasrambsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scsr {{ sram2er: {=bool:?}, srambsy: {=bool:?}, pkasrambsy: {=bool:?} }}",
                self.sram2er(),
                self.srambsy(),
                self.pkasrambsy()
            )
        }
    }
    #[doc = "SKR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Skr(pub u32);
    impl Skr {
        #[doc = "SRAM2 write protection key for software erase"]
        #[inline(always)]
        pub const fn key(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SRAM2 write protection key for software erase"]
        #[inline(always)]
        pub fn set_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Skr {
        #[inline(always)]
        fn default() -> Skr {
            Skr(0)
        }
    }
    impl core::fmt::Debug for Skr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Skr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Skr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Skr {{ key: {=u8:?} }}", self.key())
        }
    }
    #[doc = "SWPR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swpr(pub u32);
    impl Swpr {
        #[doc = "SRAM2 1Kbyte page 0 write protection"]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1Kbyte page 0 write protection"]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swpr {
        #[inline(always)]
        fn default() -> Swpr {
            Swpr(0)
        }
    }
    impl core::fmt::Debug for Swpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swpr")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swpr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Swpr {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}" , self . pwp (0usize) , self . pwp (1usize) , self . pwp (2usize) , self . pwp (3usize) , self . pwp (4usize) , self . pwp (5usize) , self . pwp (6usize) , self . pwp (7usize) , self . pwp (8usize) , self . pwp (9usize) , self . pwp (10usize) , self . pwp (11usize) , self . pwp (12usize) , self . pwp (13usize) , self . pwp (14usize) , self . pwp (15usize) , self . pwp (16usize) , self . pwp (17usize) , self . pwp (18usize) , self . pwp (19usize) , self . pwp (20usize) , self . pwp (21usize) , self . pwp (22usize) , self . pwp (23usize) , self . pwp (24usize) , self . pwp (25usize) , self . pwp (26usize) , self . pwp (27usize) , self . pwp (28usize) , self . pwp (29usize) , self . pwp (30usize) , self . pwp (31usize))
        }
    }
}
