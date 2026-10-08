#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Reset and clock control."]
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
    #[doc = "RCC clock control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RCC internal clock source calibration register 1."]
    #[inline(always)]
    pub const fn icscr1(self) -> crate::common::Reg<regs::Icscr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RCC internal clock source calibration register 2."]
    #[inline(always)]
    pub const fn icscr2(self) -> crate::common::Reg<regs::Icscr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RCC internal clock source calibration register 3."]
    #[inline(always)]
    pub const fn icscr3(self) -> crate::common::Reg<regs::Icscr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RCC clock recovery RC register."]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "RCC clock configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "RCC clock configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RCC clock configuration register 3."]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RCC clock configuration register 4."]
    #[inline(always)]
    pub const fn cfgr4(self) -> crate::common::Reg<regs::Cfgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RCC clock interrupt enable register."]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RCC clock interrupt flag register."]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RCC clock interrupt clear register."]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral reset register 1."]
    #[inline(always)]
    pub const fn ahb1rstr1(self) -> crate::common::Reg<regs::Ahb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register 1."]
    #[inline(always)]
    pub const fn ahb2rstr1(self) -> crate::common::Reg<regs::Ahb2rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register 2."]
    #[inline(always)]
    pub const fn ahb2rstr2(self) -> crate::common::Reg<regs::Ahb2rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 1."]
    #[inline(always)]
    pub const fn apb1rstr1(self) -> crate::common::Reg<regs::Apb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 2."]
    #[inline(always)]
    pub const fn apb1rstr2(self) -> crate::common::Reg<regs::Apb1rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "RCC APB2 peripheral reset register."]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "RCC APB3 peripheral reset register."]
    #[inline(always)]
    pub const fn apb3rstr(self) -> crate::common::Reg<regs::Apb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable register 1."]
    #[inline(always)]
    pub const fn ahb1enr1(self) -> crate::common::Reg<regs::Ahb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 1."]
    #[inline(always)]
    pub const fn ahb2enr1(self) -> crate::common::Reg<regs::Ahb2enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 2."]
    #[inline(always)]
    pub const fn ahb2enr2(self) -> crate::common::Reg<regs::Ahb2enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable register 2."]
    #[inline(always)]
    pub const fn ahb1enr2(self) -> crate::common::Reg<regs::Ahb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register 1."]
    #[inline(always)]
    pub const fn apb1enr1(self) -> crate::common::Reg<regs::Apb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register 2."]
    #[inline(always)]
    pub const fn apb1enr2(self) -> crate::common::Reg<regs::Apb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock enable register."]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock enable register."]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable in Sleep mode register."]
    #[inline(always)]
    pub const fn ahb1slpenr1(self) -> crate::common::Reg<regs::Ahb1slpenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable in Sleep mode register 1."]
    #[inline(always)]
    pub const fn ahb2slpenr1(self) -> crate::common::Reg<regs::Ahb2slpenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable in Sleep mode register 2."]
    #[inline(always)]
    pub const fn ahb2slpenr2(self) -> crate::common::Reg<regs::Ahb2slpenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable in Sleep mode register 2."]
    #[inline(always)]
    pub const fn ahb1slpenr2(self) -> crate::common::Reg<regs::Ahb1slpenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable in Sleep mode register 1."]
    #[inline(always)]
    pub const fn apb1slpenr1(self) -> crate::common::Reg<regs::Apb1slpenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable in Sleep mode register 2."]
    #[inline(always)]
    pub const fn apb1slpenr2(self) -> crate::common::Reg<regs::Apb1slpenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock enable in Sleep mode register."]
    #[inline(always)]
    pub const fn apb2slpenr(self) -> crate::common::Reg<regs::Apb2slpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock enable in Sleep mode register."]
    #[inline(always)]
    pub const fn apb3slpenr(self) -> crate::common::Reg<regs::Apb3slpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable in Stop mode register."]
    #[inline(always)]
    pub const fn ahb1stpenr1(self) -> crate::common::Reg<regs::Ahb1stpenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable in Stop mode register 1."]
    #[inline(always)]
    pub const fn ahb2stpenr1(self) -> crate::common::Reg<regs::Ahb2stpenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable in Stop mode register 1."]
    #[inline(always)]
    pub const fn apb1stpenr1(self) -> crate::common::Reg<regs::Apb1stpenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable in Stop mode register 2."]
    #[inline(always)]
    pub const fn apb1stpenr2(self) -> crate::common::Reg<regs::Apb1stpenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock enable in Stop mode register."]
    #[inline(always)]
    pub const fn apb2stpenr(self) -> crate::common::Reg<regs::Apb2stpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock enable in Stop mode register."]
    #[inline(always)]
    pub const fn apb3stpenr(self) -> crate::common::Reg<regs::Apb3stpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "RCC peripheral independent clock configuration register 1."]
    #[inline(always)]
    pub const fn ccipr1(self) -> crate::common::Reg<regs::Ccipr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "RCC peripheral independent clock configuration register 2."]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "RCC peripheral independent clock configuration register 3."]
    #[inline(always)]
    pub const fn ccipr3(self) -> crate::common::Reg<regs::Ccipr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "RCC backup domain control register."]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "RCC control/status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "RCC secure configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "RCC privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 peripheral clock enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr1(pub u32);
    impl Ahb1enr1 {
        #[doc = "GPDMA1 clock enable."]
        #[inline(always)]
        pub const fn gpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable."]
        #[inline(always)]
        pub fn set_gpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADF1 clock enable."]
        #[inline(always)]
        pub const fn adf1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 clock enable."]
        #[inline(always)]
        pub fn set_adf1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FLASH clock enable."]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clock enable."]
        #[inline(always)]
        pub fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch sensing controller clock enable."]
        #[inline(always)]
        pub const fn tscen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch sensing controller clock enable."]
        #[inline(always)]
        pub fn set_tscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG clock enable."]
        #[inline(always)]
        pub const fn ramcfgen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable."]
        #[inline(always)]
        pub fn set_ramcfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "GTZC1 clock enable."]
        #[inline(always)]
        pub const fn gtzc1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 clock enable."]
        #[inline(always)]
        pub fn set_gtzc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM1 clock enable."]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable."]
        #[inline(always)]
        pub fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb1enr1 {
        #[inline(always)]
        fn default() -> Ahb1enr1 {
            Ahb1enr1(0)
        }
    }
    impl core::fmt::Debug for Ahb1enr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1enr1")
                .field("gpdma1en", &self.gpdma1en())
                .field("adf1en", &self.adf1en())
                .field("flashen", &self.flashen())
                .field("crcen", &self.crcen())
                .field("tscen", &self.tscen())
                .field("ramcfgen", &self.ramcfgen())
                .field("gtzc1en", &self.gtzc1en())
                .field("sram1en", &self.sram1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1enr1 {{ gpdma1en: {=bool:?}, adf1en: {=bool:?}, flashen: {=bool:?}, crcen: {=bool:?}, tscen: {=bool:?}, ramcfgen: {=bool:?}, gtzc1en: {=bool:?}, sram1en: {=bool:?} }}" , self . gpdma1en () , self . adf1en () , self . flashen () , self . crcen () , self . tscen () , self . ramcfgen () , self . gtzc1en () , self . sram1en ())
        }
    }
    #[doc = "RCC AHB1 peripheral clock enable register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr2(pub u32);
    impl Ahb1enr2 {
        #[doc = "PWR clock enable."]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWR clock enable."]
        #[inline(always)]
        pub fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Ahb1enr2 {
        #[inline(always)]
        fn default() -> Ahb1enr2 {
            Ahb1enr2(0)
        }
    }
    impl core::fmt::Debug for Ahb1enr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1enr2").field("pwren", &self.pwren()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb1enr2 {{ pwren: {=bool:?} }}", self.pwren())
        }
    }
    #[doc = "RCC AHB1 peripheral reset register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr1(pub u32);
    impl Ahb1rstr1 {
        #[doc = "GPDMA1 reset."]
        #[inline(always)]
        pub const fn gpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 reset."]
        #[inline(always)]
        pub fn set_gpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADF1 reset."]
        #[inline(always)]
        pub const fn adf1rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 reset."]
        #[inline(always)]
        pub fn set_adf1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC reset."]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset."]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TSC reset."]
        #[inline(always)]
        pub const fn tscrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSC reset."]
        #[inline(always)]
        pub fn set_tscrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG reset."]
        #[inline(always)]
        pub const fn ramcfgrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG reset."]
        #[inline(always)]
        pub fn set_ramcfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb1rstr1 {
        #[inline(always)]
        fn default() -> Ahb1rstr1 {
            Ahb1rstr1(0)
        }
    }
    impl core::fmt::Debug for Ahb1rstr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1rstr1")
                .field("gpdma1rst", &self.gpdma1rst())
                .field("adf1rst", &self.adf1rst())
                .field("crcrst", &self.crcrst())
                .field("tscrst", &self.tscrst())
                .field("ramcfgrst", &self.ramcfgrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1rstr1 {{ gpdma1rst: {=bool:?}, adf1rst: {=bool:?}, crcrst: {=bool:?}, tscrst: {=bool:?}, ramcfgrst: {=bool:?} }}" , self . gpdma1rst () , self . adf1rst () , self . crcrst () , self . tscrst () , self . ramcfgrst ())
        }
    }
    #[doc = "RCC AHB1 peripheral clock enable in Sleep mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1slpenr1(pub u32);
    impl Ahb1slpenr1 {
        #[doc = "GPDMA1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn gpdma1slpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_gpdma1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADF1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn adf1slpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_adf1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FLASH clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn flashslpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_flashslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn crcslpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_crcslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TSC clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tscslpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSC clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tscslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn ramcfgslpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_ramcfgslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "GTZC1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn gtzc1slpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_gtzc1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ICACHE clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn icacheslpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_icacheslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn sram1slpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_sram1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb1slpenr1 {
        #[inline(always)]
        fn default() -> Ahb1slpenr1 {
            Ahb1slpenr1(0)
        }
    }
    impl core::fmt::Debug for Ahb1slpenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1slpenr1")
                .field("gpdma1slpen", &self.gpdma1slpen())
                .field("adf1slpen", &self.adf1slpen())
                .field("flashslpen", &self.flashslpen())
                .field("crcslpen", &self.crcslpen())
                .field("tscslpen", &self.tscslpen())
                .field("ramcfgslpen", &self.ramcfgslpen())
                .field("gtzc1slpen", &self.gtzc1slpen())
                .field("icacheslpen", &self.icacheslpen())
                .field("sram1slpen", &self.sram1slpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1slpenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1slpenr1 {{ gpdma1slpen: {=bool:?}, adf1slpen: {=bool:?}, flashslpen: {=bool:?}, crcslpen: {=bool:?}, tscslpen: {=bool:?}, ramcfgslpen: {=bool:?}, gtzc1slpen: {=bool:?}, icacheslpen: {=bool:?}, sram1slpen: {=bool:?} }}" , self . gpdma1slpen () , self . adf1slpen () , self . flashslpen () , self . crcslpen () , self . tscslpen () , self . ramcfgslpen () , self . gtzc1slpen () , self . icacheslpen () , self . sram1slpen ())
        }
    }
    #[doc = "RCC AHB1 peripheral clock enable in Sleep mode register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1slpenr2(pub u32);
    impl Ahb1slpenr2 {
        #[doc = "PWR clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn pwrslpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWR clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_pwrslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Ahb1slpenr2 {
        #[inline(always)]
        fn default() -> Ahb1slpenr2 {
            Ahb1slpenr2(0)
        }
    }
    impl core::fmt::Debug for Ahb1slpenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1slpenr2")
                .field("pwrslpen", &self.pwrslpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1slpenr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb1slpenr2 {{ pwrslpen: {=bool:?} }}", self.pwrslpen())
        }
    }
    #[doc = "RCC AHB1 peripheral clock enable in Stop mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1stpenr1(pub u32);
    impl Ahb1stpenr1 {
        #[doc = "GPDMA1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn gpdma1stpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_gpdma1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADF1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn adf1stpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_adf1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FLASH clock enable during Stop mode."]
        #[inline(always)]
        pub const fn flashstpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_flashstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RAMCFG clock enable during Stop mode."]
        #[inline(always)]
        pub const fn ramcfgstpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_ramcfgstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "GTZC1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn gtzc1stpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_gtzc1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn sram1stpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_sram1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb1stpenr1 {
        #[inline(always)]
        fn default() -> Ahb1stpenr1 {
            Ahb1stpenr1(0)
        }
    }
    impl core::fmt::Debug for Ahb1stpenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1stpenr1")
                .field("gpdma1stpen", &self.gpdma1stpen())
                .field("adf1stpen", &self.adf1stpen())
                .field("flashstpen", &self.flashstpen())
                .field("ramcfgstpen", &self.ramcfgstpen())
                .field("gtzc1stpen", &self.gtzc1stpen())
                .field("sram1stpen", &self.sram1stpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1stpenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1stpenr1 {{ gpdma1stpen: {=bool:?}, adf1stpen: {=bool:?}, flashstpen: {=bool:?}, ramcfgstpen: {=bool:?}, gtzc1stpen: {=bool:?}, sram1stpen: {=bool:?} }}" , self . gpdma1stpen () , self . adf1stpen () , self . flashstpen () , self . ramcfgstpen () , self . gtzc1stpen () , self . sram1stpen ())
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr1(pub u32);
    impl Ahb2enr1 {
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port i clock enable (i = H to G)."]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable (i = H to G)."]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O port i clock enable (i = H to G)."]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable (i = H to G)."]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC12 clock enable."]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC12 clock enable."]
        #[inline(always)]
        pub fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC1 clock enable."]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable."]
        #[inline(always)]
        pub fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "AES clock enable."]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable."]
        #[inline(always)]
        pub fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable."]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable."]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG clock enable."]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable."]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA clock enable."]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable."]
        #[inline(always)]
        pub fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES clock enable."]
        #[inline(always)]
        pub const fn saesen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES clock enable."]
        #[inline(always)]
        pub fn set_saesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CCB clock enable."]
        #[inline(always)]
        pub const fn ccben(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CCB clock enable."]
        #[inline(always)]
        pub fn set_ccben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDMMC1 clock enable."]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clock enable."]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SRAM2 clock enable."]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable."]
        #[inline(always)]
        pub fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ahb2enr1 {
        #[inline(always)]
        fn default() -> Ahb2enr1 {
            Ahb2enr1(0)
        }
    }
    impl core::fmt::Debug for Ahb2enr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2enr1")
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("adc12en", &self.adc12en())
                .field("dac1en", &self.dac1en())
                .field("aesen", &self.aesen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("pkaen", &self.pkaen())
                .field("saesen", &self.saesen())
                .field("ccben", &self.ccben())
                .field("sdmmc1en", &self.sdmmc1en())
                .field("sram2en", &self.sram2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2enr1 {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, adc12en: {=bool:?}, dac1en: {=bool:?}, aesen: {=bool:?}, hashen: {=bool:?}, rngen: {=bool:?}, pkaen: {=bool:?}, saesen: {=bool:?}, ccben: {=bool:?}, sdmmc1en: {=bool:?}, sram2en: {=bool:?} }}" , self . gpioaen () , self . gpioben () , self . gpiocen () , self . gpioden () , self . gpioeen () , self . gpiogen () , self . gpiohen () , self . adc12en () , self . dac1en () , self . aesen () , self . hashen () , self . rngen () , self . pkaen () , self . saesen () , self . ccben () , self . sdmmc1en () , self . sram2en ())
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr2(pub u32);
    impl Ahb2enr2 {
        #[doc = "OCTOSPI1 clock enable."]
        #[inline(always)]
        pub const fn octospi1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 clock enable."]
        #[inline(always)]
        pub fn set_octospi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Ahb2enr2 {
        #[inline(always)]
        fn default() -> Ahb2enr2 {
            Ahb2enr2(0)
        }
    }
    impl core::fmt::Debug for Ahb2enr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2enr2")
                .field("octospi1en", &self.octospi1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb2enr2 {{ octospi1en: {=bool:?} }}", self.octospi1en())
        }
    }
    #[doc = "RCC AHB2 peripheral reset register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr1(pub u32);
    impl Ahb2rstr1 {
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i reset (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port i reset (i = H to G)."]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i reset (i = H to G)."]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O port i reset (i = H to G)."]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i reset (i = H to G)."]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC12 reset."]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC12 reset."]
        #[inline(always)]
        pub fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC1 reset."]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 reset."]
        #[inline(always)]
        pub fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "AES hardware accelerator reset."]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES hardware accelerator reset."]
        #[inline(always)]
        pub fn set_aesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH reset."]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH reset."]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator reset."]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator reset."]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA reset."]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA reset."]
        #[inline(always)]
        pub fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES hardware accelerator reset."]
        #[inline(always)]
        pub const fn saesrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES hardware accelerator reset."]
        #[inline(always)]
        pub fn set_saesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CCB reset."]
        #[inline(always)]
        pub const fn ccbrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CCB reset."]
        #[inline(always)]
        pub fn set_ccbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDMMC1 reset."]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 reset."]
        #[inline(always)]
        pub fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Ahb2rstr1 {
        #[inline(always)]
        fn default() -> Ahb2rstr1 {
            Ahb2rstr1(0)
        }
    }
    impl core::fmt::Debug for Ahb2rstr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2rstr1")
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiogrst", &self.gpiogrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("adc12rst", &self.adc12rst())
                .field("dac1rst", &self.dac1rst())
                .field("aesrst", &self.aesrst())
                .field("hashrst", &self.hashrst())
                .field("rngrst", &self.rngrst())
                .field("pkarst", &self.pkarst())
                .field("saesrst", &self.saesrst())
                .field("ccbrst", &self.ccbrst())
                .field("sdmmc1rst", &self.sdmmc1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2rstr1 {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiogrst: {=bool:?}, gpiohrst: {=bool:?}, adc12rst: {=bool:?}, dac1rst: {=bool:?}, aesrst: {=bool:?}, hashrst: {=bool:?}, rngrst: {=bool:?}, pkarst: {=bool:?}, saesrst: {=bool:?}, ccbrst: {=bool:?}, sdmmc1rst: {=bool:?} }}" , self . gpioarst () , self . gpiobrst () , self . gpiocrst () , self . gpiodrst () , self . gpioerst () , self . gpiogrst () , self . gpiohrst () , self . adc12rst () , self . dac1rst () , self . aesrst () , self . hashrst () , self . rngrst () , self . pkarst () , self . saesrst () , self . ccbrst () , self . sdmmc1rst ())
        }
    }
    #[doc = "RCC AHB2 peripheral reset register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr2(pub u32);
    impl Ahb2rstr2 {
        #[doc = "OCTOSPI1 reset."]
        #[inline(always)]
        pub const fn octospi1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 reset."]
        #[inline(always)]
        pub fn set_octospi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Ahb2rstr2 {
        #[inline(always)]
        fn default() -> Ahb2rstr2 {
            Ahb2rstr2(0)
        }
    }
    impl core::fmt::Debug for Ahb2rstr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2rstr2")
                .field("octospi1rst", &self.octospi1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb2rstr2 {{ octospi1rst: {=bool:?} }}", self.octospi1rst())
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable in Sleep mode register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2slpenr1(pub u32);
    impl Ahb2slpenr1 {
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpioaslpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioaslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpiobslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiobslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpiocslpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiocslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpiodslpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiodslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpioeslpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioeslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = H to G)."]
        #[inline(always)]
        pub const fn gpiogslpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = H to G)."]
        #[inline(always)]
        pub fn set_gpiogslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = H to G)."]
        #[inline(always)]
        pub const fn gpiohslpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Sleep mode (i = H to G)."]
        #[inline(always)]
        pub fn set_gpiohslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC12 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn adc12slpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC12 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_adc12slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn dac1slpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_dac1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "AES clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn aesslpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_aesslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn hashslpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_hashslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn rngslpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_rngslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn pkaslpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_pkaslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES accelerator clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn saesslpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES accelerator clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_saesslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CCB accelerator clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn ccbslpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CCB accelerator clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_ccbslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDMMC1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn sdmmc1slpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_sdmmc1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SRAM2 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn sram2slpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_sram2slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ahb2slpenr1 {
        #[inline(always)]
        fn default() -> Ahb2slpenr1 {
            Ahb2slpenr1(0)
        }
    }
    impl core::fmt::Debug for Ahb2slpenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2slpenr1")
                .field("gpioaslpen", &self.gpioaslpen())
                .field("gpiobslpen", &self.gpiobslpen())
                .field("gpiocslpen", &self.gpiocslpen())
                .field("gpiodslpen", &self.gpiodslpen())
                .field("gpioeslpen", &self.gpioeslpen())
                .field("gpiogslpen", &self.gpiogslpen())
                .field("gpiohslpen", &self.gpiohslpen())
                .field("adc12slpen", &self.adc12slpen())
                .field("dac1slpen", &self.dac1slpen())
                .field("aesslpen", &self.aesslpen())
                .field("hashslpen", &self.hashslpen())
                .field("rngslpen", &self.rngslpen())
                .field("pkaslpen", &self.pkaslpen())
                .field("saesslpen", &self.saesslpen())
                .field("ccbslpen", &self.ccbslpen())
                .field("sdmmc1slpen", &self.sdmmc1slpen())
                .field("sram2slpen", &self.sram2slpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2slpenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2slpenr1 {{ gpioaslpen: {=bool:?}, gpiobslpen: {=bool:?}, gpiocslpen: {=bool:?}, gpiodslpen: {=bool:?}, gpioeslpen: {=bool:?}, gpiogslpen: {=bool:?}, gpiohslpen: {=bool:?}, adc12slpen: {=bool:?}, dac1slpen: {=bool:?}, aesslpen: {=bool:?}, hashslpen: {=bool:?}, rngslpen: {=bool:?}, pkaslpen: {=bool:?}, saesslpen: {=bool:?}, ccbslpen: {=bool:?}, sdmmc1slpen: {=bool:?}, sram2slpen: {=bool:?} }}" , self . gpioaslpen () , self . gpiobslpen () , self . gpiocslpen () , self . gpiodslpen () , self . gpioeslpen () , self . gpiogslpen () , self . gpiohslpen () , self . adc12slpen () , self . dac1slpen () , self . aesslpen () , self . hashslpen () , self . rngslpen () , self . pkaslpen () , self . saesslpen () , self . ccbslpen () , self . sdmmc1slpen () , self . sram2slpen ())
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable in Sleep mode register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2slpenr2(pub u32);
    impl Ahb2slpenr2 {
        #[doc = "OCTOSPI1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn octospi1slpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_octospi1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Ahb2slpenr2 {
        #[inline(always)]
        fn default() -> Ahb2slpenr2 {
            Ahb2slpenr2(0)
        }
    }
    impl core::fmt::Debug for Ahb2slpenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2slpenr2")
                .field("octospi1slpen", &self.octospi1slpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2slpenr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb2slpenr2 {{ octospi1slpen: {=bool:?} }}", self.octospi1slpen())
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable in Stop mode register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2stpenr1(pub u32);
    impl Ahb2stpenr1 {
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpioastpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioastpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpiobstpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiobstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpiocstpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiocstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpiodstpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpiodstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub const fn gpioestpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Stop mode (i = E to A)."]
        #[inline(always)]
        pub fn set_gpioestpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port i clock enable during Stop mode (i = H to G)."]
        #[inline(always)]
        pub const fn gpiogstpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Stop mode (i = H to G)."]
        #[inline(always)]
        pub fn set_gpiogstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O port i clock enable during Stop mode (i = H to G)."]
        #[inline(always)]
        pub const fn gpiohstpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port i clock enable during Stop mode (i = H to G)."]
        #[inline(always)]
        pub fn set_gpiohstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DAC1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn dac1stpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_dac1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SRAM2 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn sram2stpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_sram2stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ahb2stpenr1 {
        #[inline(always)]
        fn default() -> Ahb2stpenr1 {
            Ahb2stpenr1(0)
        }
    }
    impl core::fmt::Debug for Ahb2stpenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2stpenr1")
                .field("gpioastpen", &self.gpioastpen())
                .field("gpiobstpen", &self.gpiobstpen())
                .field("gpiocstpen", &self.gpiocstpen())
                .field("gpiodstpen", &self.gpiodstpen())
                .field("gpioestpen", &self.gpioestpen())
                .field("gpiogstpen", &self.gpiogstpen())
                .field("gpiohstpen", &self.gpiohstpen())
                .field("dac1stpen", &self.dac1stpen())
                .field("sram2stpen", &self.sram2stpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2stpenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2stpenr1 {{ gpioastpen: {=bool:?}, gpiobstpen: {=bool:?}, gpiocstpen: {=bool:?}, gpiodstpen: {=bool:?}, gpioestpen: {=bool:?}, gpiogstpen: {=bool:?}, gpiohstpen: {=bool:?}, dac1stpen: {=bool:?}, sram2stpen: {=bool:?} }}" , self . gpioastpen () , self . gpiobstpen () , self . gpiocstpen () , self . gpiodstpen () , self . gpioestpen () , self . gpiogstpen () , self . gpiohstpen () , self . dac1stpen () , self . sram2stpen ())
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr1(pub u32);
    impl Apb1enr1 {
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable."]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI3 clock enable."]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable."]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "WWDG clock enable."]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable."]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable."]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable."]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART3 clock enable."]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable."]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable."]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable."]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable."]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable."]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable."]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable."]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable."]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable."]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 clock enable."]
        #[inline(always)]
        pub const fn i3c1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable."]
        #[inline(always)]
        pub fn set_i3c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable."]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable."]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "OPAMP clock enable."]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable."]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "VREFBUF clock enable."]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clock enable."]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "RTC and TAMP APB clock enable."]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP APB clock enable."]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb1enr1 {
        #[inline(always)]
        fn default() -> Apb1enr1 {
            Apb1enr1(0)
        }
    }
    impl core::fmt::Debug for Apb1enr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr1")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim4en", &self.tim4en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("spi3en", &self.spi3en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i3c1en", &self.i3c1en())
                .field("crsen", &self.crsen())
                .field("opampen", &self.opampen())
                .field("vrefen", &self.vrefen())
                .field("rtcapben", &self.rtcapben())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1enr1 {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, spi3en: {=bool:?}, wwdgen: {=bool:?}, spi2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, i3c1en: {=bool:?}, crsen: {=bool:?}, opampen: {=bool:?}, vrefen: {=bool:?}, rtcapben: {=bool:?} }}" , self . tim2en () , self . tim3en () , self . tim4en () , self . tim6en () , self . tim7en () , self . spi3en () , self . wwdgen () , self . spi2en () , self . usart3en () , self . uart4en () , self . uart5en () , self . i2c1en () , self . i2c2en () , self . i3c1en () , self . crsen () , self . opampen () , self . vrefen () , self . rtcapben ())
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr2(pub u32);
    impl Apb1enr2 {
        #[doc = "LPTIM2 clock enable."]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clock enable."]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN1 clock enable."]
        #[inline(always)]
        pub const fn fdcan1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 clock enable."]
        #[inline(always)]
        pub fn set_fdcan1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Apb1enr2 {
        #[inline(always)]
        fn default() -> Apb1enr2 {
            Apb1enr2(0)
        }
    }
    impl core::fmt::Debug for Apb1enr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr2")
                .field("lptim2en", &self.lptim2en())
                .field("fdcan1en", &self.fdcan1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1enr2 {{ lptim2en: {=bool:?}, fdcan1en: {=bool:?} }}",
                self.lptim2en(),
                self.fdcan1en()
            )
        }
    }
    #[doc = "RCC APB1 peripheral reset register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr1(pub u32);
    impl Apb1rstr1 {
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj reset."]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI3 reset."]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset."]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SPI2 reset."]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset."]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART3 reset."]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 reset."]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 reset."]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 reset."]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 reset."]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 reset."]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset."]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset."]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset."]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset."]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 reset."]
        #[inline(always)]
        pub const fn i3c1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 reset."]
        #[inline(always)]
        pub fn set_i3c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS reset."]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS reset."]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "OPAMP reset."]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP reset."]
        #[inline(always)]
        pub fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "VREFBUF reset."]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF reset."]
        #[inline(always)]
        pub fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb1rstr1 {
        #[inline(always)]
        fn default() -> Apb1rstr1 {
            Apb1rstr1(0)
        }
    }
    impl core::fmt::Debug for Apb1rstr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr1")
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim4rst", &self.tim4rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("spi3rst", &self.spi3rst())
                .field("spi2rst", &self.spi2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("i3c1rst", &self.i3c1rst())
                .field("crsrst", &self.crsrst())
                .field("opamprst", &self.opamprst())
                .field("vrefrst", &self.vrefrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1rstr1 {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim4rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, spi3rst: {=bool:?}, spi2rst: {=bool:?}, usart3rst: {=bool:?}, uart4rst: {=bool:?}, uart5rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, i3c1rst: {=bool:?}, crsrst: {=bool:?}, opamprst: {=bool:?}, vrefrst: {=bool:?} }}" , self . tim2rst () , self . tim3rst () , self . tim4rst () , self . tim6rst () , self . tim7rst () , self . spi3rst () , self . spi2rst () , self . usart3rst () , self . uart4rst () , self . uart5rst () , self . i2c1rst () , self . i2c2rst () , self . i3c1rst () , self . crsrst () , self . opamprst () , self . vrefrst ())
        }
    }
    #[doc = "RCC APB1 peripheral reset register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr2(pub u32);
    impl Apb1rstr2 {
        #[doc = "LPTIM2 reset."]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 reset."]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN1 reset."]
        #[inline(always)]
        pub const fn fdcan1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 reset."]
        #[inline(always)]
        pub fn set_fdcan1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Apb1rstr2 {
        #[inline(always)]
        fn default() -> Apb1rstr2 {
            Apb1rstr2(0)
        }
    }
    impl core::fmt::Debug for Apb1rstr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr2")
                .field("lptim2rst", &self.lptim2rst())
                .field("fdcan1rst", &self.fdcan1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1rstr2 {{ lptim2rst: {=bool:?}, fdcan1rst: {=bool:?} }}",
                self.lptim2rst(),
                self.fdcan1rst()
            )
        }
    }
    #[doc = "RCC APB1 peripheral clock enable in Sleep mode register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1slpenr1(pub u32);
    impl Apb1slpenr1 {
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim2slpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim2slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim3slpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim3slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim4slpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim4slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim6slpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim6slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim7slpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIMj clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim7slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI3 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn spi3slpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_spi3slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "WWDG clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn wwdgslpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_wwdgslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn spi2slpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_spi2slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART3 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn usart3slpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_usart3slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn uart4slpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_uart4slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn uart5slpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_uart5slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn i2c1slpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_i2c1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn i2c2slpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_i2c2slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn i3c1slpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_i3c1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn crsslpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_crsslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "OPAMP clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn opampslpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_opampslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "VREFBUF clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn vrefslpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_vrefslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "RTC and TAMP APB clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn rtcapbslpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP APB clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_rtcapbslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb1slpenr1 {
        #[inline(always)]
        fn default() -> Apb1slpenr1 {
            Apb1slpenr1(0)
        }
    }
    impl core::fmt::Debug for Apb1slpenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1slpenr1")
                .field("tim2slpen", &self.tim2slpen())
                .field("tim3slpen", &self.tim3slpen())
                .field("tim4slpen", &self.tim4slpen())
                .field("tim6slpen", &self.tim6slpen())
                .field("tim7slpen", &self.tim7slpen())
                .field("spi3slpen", &self.spi3slpen())
                .field("wwdgslpen", &self.wwdgslpen())
                .field("spi2slpen", &self.spi2slpen())
                .field("usart3slpen", &self.usart3slpen())
                .field("uart4slpen", &self.uart4slpen())
                .field("uart5slpen", &self.uart5slpen())
                .field("i2c1slpen", &self.i2c1slpen())
                .field("i2c2slpen", &self.i2c2slpen())
                .field("i3c1slpen", &self.i3c1slpen())
                .field("crsslpen", &self.crsslpen())
                .field("opampslpen", &self.opampslpen())
                .field("vrefslpen", &self.vrefslpen())
                .field("rtcapbslpen", &self.rtcapbslpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1slpenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1slpenr1 {{ tim2slpen: {=bool:?}, tim3slpen: {=bool:?}, tim4slpen: {=bool:?}, tim6slpen: {=bool:?}, tim7slpen: {=bool:?}, spi3slpen: {=bool:?}, wwdgslpen: {=bool:?}, spi2slpen: {=bool:?}, usart3slpen: {=bool:?}, uart4slpen: {=bool:?}, uart5slpen: {=bool:?}, i2c1slpen: {=bool:?}, i2c2slpen: {=bool:?}, i3c1slpen: {=bool:?}, crsslpen: {=bool:?}, opampslpen: {=bool:?}, vrefslpen: {=bool:?}, rtcapbslpen: {=bool:?} }}" , self . tim2slpen () , self . tim3slpen () , self . tim4slpen () , self . tim6slpen () , self . tim7slpen () , self . spi3slpen () , self . wwdgslpen () , self . spi2slpen () , self . usart3slpen () , self . uart4slpen () , self . uart5slpen () , self . i2c1slpen () , self . i2c2slpen () , self . i3c1slpen () , self . crsslpen () , self . opampslpen () , self . vrefslpen () , self . rtcapbslpen ())
        }
    }
    #[doc = "RCC APB1 peripheral clock enable in Sleep mode register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1slpenr2(pub u32);
    impl Apb1slpenr2 {
        #[doc = "LPTIM2 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn lptim2slpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_lptim2slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn fdcan1slpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_fdcan1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Apb1slpenr2 {
        #[inline(always)]
        fn default() -> Apb1slpenr2 {
            Apb1slpenr2(0)
        }
    }
    impl core::fmt::Debug for Apb1slpenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1slpenr2")
                .field("lptim2slpen", &self.lptim2slpen())
                .field("fdcan1slpen", &self.fdcan1slpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1slpenr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1slpenr2 {{ lptim2slpen: {=bool:?}, fdcan1slpen: {=bool:?} }}",
                self.lptim2slpen(),
                self.fdcan1slpen()
            )
        }
    }
    #[doc = "RCC APB1 peripheral clock enable in Stop mode register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1stpenr1(pub u32);
    impl Apb1stpenr1 {
        #[doc = "SPI3 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn spi3stpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_spi3stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SPI2 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn spi2stpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_spi2stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART3 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn usart3stpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_usart3stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn uart4stpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_uart4stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn uart5stpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_uart5stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn i2c1stpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_i2c1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn i2c2stpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_i2c2stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn i3c1stpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_i3c1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OPAMP clock enable during Stop mode."]
        #[inline(always)]
        pub const fn opampstpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_opampstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "VREFBUF clock enable during Stop mode."]
        #[inline(always)]
        pub const fn vrefstpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_vrefstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "RTC and TAMP APB clock enable during Stop mode."]
        #[inline(always)]
        pub const fn rtcapbstpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP APB clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_rtcapbstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb1stpenr1 {
        #[inline(always)]
        fn default() -> Apb1stpenr1 {
            Apb1stpenr1(0)
        }
    }
    impl core::fmt::Debug for Apb1stpenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1stpenr1")
                .field("spi3stpen", &self.spi3stpen())
                .field("spi2stpen", &self.spi2stpen())
                .field("usart3stpen", &self.usart3stpen())
                .field("uart4stpen", &self.uart4stpen())
                .field("uart5stpen", &self.uart5stpen())
                .field("i2c1stpen", &self.i2c1stpen())
                .field("i2c2stpen", &self.i2c2stpen())
                .field("i3c1stpen", &self.i3c1stpen())
                .field("opampstpen", &self.opampstpen())
                .field("vrefstpen", &self.vrefstpen())
                .field("rtcapbstpen", &self.rtcapbstpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1stpenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1stpenr1 {{ spi3stpen: {=bool:?}, spi2stpen: {=bool:?}, usart3stpen: {=bool:?}, uart4stpen: {=bool:?}, uart5stpen: {=bool:?}, i2c1stpen: {=bool:?}, i2c2stpen: {=bool:?}, i3c1stpen: {=bool:?}, opampstpen: {=bool:?}, vrefstpen: {=bool:?}, rtcapbstpen: {=bool:?} }}" , self . spi3stpen () , self . spi2stpen () , self . usart3stpen () , self . uart4stpen () , self . uart5stpen () , self . i2c1stpen () , self . i2c2stpen () , self . i3c1stpen () , self . opampstpen () , self . vrefstpen () , self . rtcapbstpen ())
        }
    }
    #[doc = "RCC APB1 peripheral clock enable in Stop mode register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1stpenr2(pub u32);
    impl Apb1stpenr2 {
        #[doc = "LPTIM2 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn lptim2stpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_lptim2stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Apb1stpenr2 {
        #[inline(always)]
        fn default() -> Apb1stpenr2 {
            Apb1stpenr2(0)
        }
    }
    impl core::fmt::Debug for Apb1stpenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1stpenr2")
                .field("lptim2stpen", &self.lptim2stpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1stpenr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb1stpenr2 {{ lptim2stpen: {=bool:?} }}", self.lptim2stpen())
        }
    }
    #[doc = "RCC APB2 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 clock enable."]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable."]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable."]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable."]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1clock enable."]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clock enable."]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIMi clock enable."]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi clock enable."]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIMi clock enable."]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi clock enable."]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIMi clock enable."]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi clock enable."]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clock enable."]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clock enable."]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "USB clock enable."]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable."]
        #[inline(always)]
        pub fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "I3C2 clock enable."]
        #[inline(always)]
        pub const fn i3c2en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "I3C2 clock enable."]
        #[inline(always)]
        pub fn set_i3c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("tim1en", &self.tim1en())
                .field("spi1en", &self.spi1en())
                .field("usart1en", &self.usart1en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("sai1en", &self.sai1en())
                .field("usben", &self.usben())
                .field("i3c2en", &self.i3c2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2enr {{ tim1en: {=bool:?}, spi1en: {=bool:?}, usart1en: {=bool:?}, tim15en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, sai1en: {=bool:?}, usben: {=bool:?}, i3c2en: {=bool:?} }}" , self . tim1en () , self . spi1en () , self . usart1en () , self . tim15en () , self . tim16en () , self . tim17en () , self . sai1en () , self . usben () , self . i3c2en ())
        }
    }
    #[doc = "RCC APB2 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 reset."]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 reset."]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 reset."]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset."]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 reset."]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset."]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIMi reset."]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi reset."]
        #[inline(always)]
        pub fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIMi reset."]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi reset."]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIMi reset."]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi reset."]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 reset."]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 reset."]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "USB reset."]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset."]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "I3C2 reset."]
        #[inline(always)]
        pub const fn i3c2rst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "I3C2 reset."]
        #[inline(always)]
        pub fn set_i3c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("tim1rst", &self.tim1rst())
                .field("spi1rst", &self.spi1rst())
                .field("usart1rst", &self.usart1rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("sai1rst", &self.sai1rst())
                .field("usbrst", &self.usbrst())
                .field("i3c2rst", &self.i3c2rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2rstr {{ tim1rst: {=bool:?}, spi1rst: {=bool:?}, usart1rst: {=bool:?}, tim15rst: {=bool:?}, tim16rst: {=bool:?}, tim17rst: {=bool:?}, sai1rst: {=bool:?}, usbrst: {=bool:?}, i3c2rst: {=bool:?} }}" , self . tim1rst () , self . spi1rst () , self . usart1rst () , self . tim15rst () , self . tim16rst () , self . tim17rst () , self . sai1rst () , self . usbrst () , self . i3c2rst ())
        }
    }
    #[doc = "RCC APB2 peripheral clock enable in Sleep mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2slpenr(pub u32);
    impl Apb2slpenr {
        #[doc = "TIM1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim1slpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn spi1slpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_spi1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn usart1slpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_usart1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim15slpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim15slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim16slpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim16slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn tim17slpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_tim17slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn sai1slpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_sai1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "USB clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn usbslpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_usbslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "I3C2 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn i3c2slpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "I3C2 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_i3c2slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Apb2slpenr {
        #[inline(always)]
        fn default() -> Apb2slpenr {
            Apb2slpenr(0)
        }
    }
    impl core::fmt::Debug for Apb2slpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2slpenr")
                .field("tim1slpen", &self.tim1slpen())
                .field("spi1slpen", &self.spi1slpen())
                .field("usart1slpen", &self.usart1slpen())
                .field("tim15slpen", &self.tim15slpen())
                .field("tim16slpen", &self.tim16slpen())
                .field("tim17slpen", &self.tim17slpen())
                .field("sai1slpen", &self.sai1slpen())
                .field("usbslpen", &self.usbslpen())
                .field("i3c2slpen", &self.i3c2slpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2slpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2slpenr {{ tim1slpen: {=bool:?}, spi1slpen: {=bool:?}, usart1slpen: {=bool:?}, tim15slpen: {=bool:?}, tim16slpen: {=bool:?}, tim17slpen: {=bool:?}, sai1slpen: {=bool:?}, usbslpen: {=bool:?}, i3c2slpen: {=bool:?} }}" , self . tim1slpen () , self . spi1slpen () , self . usart1slpen () , self . tim15slpen () , self . tim16slpen () , self . tim17slpen () , self . sai1slpen () , self . usbslpen () , self . i3c2slpen ())
        }
    }
    #[doc = "RCC APB2 peripheral clock enable in Stop mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2stpenr(pub u32);
    impl Apb2stpenr {
        #[doc = "SPI1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn spi1stpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_spi1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1clock enable during Stop mode."]
        #[inline(always)]
        pub const fn usart1stpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_usart1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USB clock enable during Stop mode."]
        #[inline(always)]
        pub const fn usbstpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_usbstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "I3C2 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn i3c2stpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "I3C2 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_i3c2stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Apb2stpenr {
        #[inline(always)]
        fn default() -> Apb2stpenr {
            Apb2stpenr(0)
        }
    }
    impl core::fmt::Debug for Apb2stpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2stpenr")
                .field("spi1stpen", &self.spi1stpen())
                .field("usart1stpen", &self.usart1stpen())
                .field("usbstpen", &self.usbstpen())
                .field("i3c2stpen", &self.i3c2stpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2stpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2stpenr {{ spi1stpen: {=bool:?}, usart1stpen: {=bool:?}, usbstpen: {=bool:?}, i3c2stpen: {=bool:?} }}" , self . spi1stpen () , self . usart1stpen () , self . usbstpen () , self . i3c2stpen ())
        }
    }
    #[doc = "RCC APB3 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "SYSCFG clock enable."]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clock enable."]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 clock enable."]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable."]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clock enable."]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable."]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 clock enable."]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable."]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIMi clock enable."]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIMi clock enable."]
        #[inline(always)]
        pub fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIMi clock enable."]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIMi clock enable."]
        #[inline(always)]
        pub fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP clock enable."]
        #[inline(always)]
        pub const fn compen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable."]
        #[inline(always)]
        pub fn set_compen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Apb3enr {
        #[inline(always)]
        fn default() -> Apb3enr {
            Apb3enr(0)
        }
    }
    impl core::fmt::Debug for Apb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3enr")
                .field("syscfgen", &self.syscfgen())
                .field("lpuart1en", &self.lpuart1en())
                .field("i2c3en", &self.i2c3en())
                .field("lptim1en", &self.lptim1en())
                .field("lptim3en", &self.lptim3en())
                .field("lptim4en", &self.lptim4en())
                .field("compen", &self.compen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb3enr {{ syscfgen: {=bool:?}, lpuart1en: {=bool:?}, i2c3en: {=bool:?}, lptim1en: {=bool:?}, lptim3en: {=bool:?}, lptim4en: {=bool:?}, compen: {=bool:?} }}" , self . syscfgen () , self . lpuart1en () , self . i2c3en () , self . lptim1en () , self . lptim3en () , self . lptim4en () , self . compen ())
        }
    }
    #[doc = "RCC APB3 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
        #[doc = "SYSCFG reset."]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG reset."]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 reset."]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 reset."]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 reset."]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset."]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 reset."]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 reset."]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIMi reset."]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIMi reset."]
        #[inline(always)]
        pub fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIMi reset."]
        #[inline(always)]
        pub const fn lptim4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIMi reset."]
        #[inline(always)]
        pub fn set_lptim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP reset."]
        #[inline(always)]
        pub const fn comprst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP reset."]
        #[inline(always)]
        pub fn set_comprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Apb3rstr {
        #[inline(always)]
        fn default() -> Apb3rstr {
            Apb3rstr(0)
        }
    }
    impl core::fmt::Debug for Apb3rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3rstr")
                .field("syscfgrst", &self.syscfgrst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("lptim1rst", &self.lptim1rst())
                .field("lptim3rst", &self.lptim3rst())
                .field("lptim4rst", &self.lptim4rst())
                .field("comprst", &self.comprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb3rstr {{ syscfgrst: {=bool:?}, lpuart1rst: {=bool:?}, i2c3rst: {=bool:?}, lptim1rst: {=bool:?}, lptim3rst: {=bool:?}, lptim4rst: {=bool:?}, comprst: {=bool:?} }}" , self . syscfgrst () , self . lpuart1rst () , self . i2c3rst () , self . lptim1rst () , self . lptim3rst () , self . lptim4rst () , self . comprst ())
        }
    }
    #[doc = "RCC APB3 peripheral clock enable in Sleep mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3slpenr(pub u32);
    impl Apb3slpenr {
        #[doc = "SYSCFG clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn syscfgslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_syscfgslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn lpuart1slpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_lpuart1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn i2c3slpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_i2c3slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn lptim1slpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_lptim1slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn lptim3slpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_lptim3slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn lptim4slpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIMi clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_lptim4slpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP clock enable during Sleep mode."]
        #[inline(always)]
        pub const fn compslpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable during Sleep mode."]
        #[inline(always)]
        pub fn set_compslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Apb3slpenr {
        #[inline(always)]
        fn default() -> Apb3slpenr {
            Apb3slpenr(0)
        }
    }
    impl core::fmt::Debug for Apb3slpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3slpenr")
                .field("syscfgslpen", &self.syscfgslpen())
                .field("lpuart1slpen", &self.lpuart1slpen())
                .field("i2c3slpen", &self.i2c3slpen())
                .field("lptim1slpen", &self.lptim1slpen())
                .field("lptim3slpen", &self.lptim3slpen())
                .field("lptim4slpen", &self.lptim4slpen())
                .field("compslpen", &self.compslpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3slpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb3slpenr {{ syscfgslpen: {=bool:?}, lpuart1slpen: {=bool:?}, i2c3slpen: {=bool:?}, lptim1slpen: {=bool:?}, lptim3slpen: {=bool:?}, lptim4slpen: {=bool:?}, compslpen: {=bool:?} }}" , self . syscfgslpen () , self . lpuart1slpen () , self . i2c3slpen () , self . lptim1slpen () , self . lptim3slpen () , self . lptim4slpen () , self . compslpen ())
        }
    }
    #[doc = "RCC APB3 peripheral clock enable in Stop mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3stpenr(pub u32);
    impl Apb3stpenr {
        #[doc = "LPUART1 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn lpuart1stpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_lpuart1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clock enable during Stop mode."]
        #[inline(always)]
        pub const fn i2c3stpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_i2c3stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1clock enable during Stop mode."]
        #[inline(always)]
        pub const fn lptim1stpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_lptim1stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIMi clock enable during Stop mode."]
        #[inline(always)]
        pub const fn lptim3stpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIMi clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_lptim3stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIMi clock enable during Stop mode."]
        #[inline(always)]
        pub const fn lptim4stpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIMi clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_lptim4stpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP clock enable during Stop mode."]
        #[inline(always)]
        pub const fn compstpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable during Stop mode."]
        #[inline(always)]
        pub fn set_compstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Apb3stpenr {
        #[inline(always)]
        fn default() -> Apb3stpenr {
            Apb3stpenr(0)
        }
    }
    impl core::fmt::Debug for Apb3stpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3stpenr")
                .field("lpuart1stpen", &self.lpuart1stpen())
                .field("i2c3stpen", &self.i2c3stpen())
                .field("lptim1stpen", &self.lptim1stpen())
                .field("lptim3stpen", &self.lptim3stpen())
                .field("lptim4stpen", &self.lptim4stpen())
                .field("compstpen", &self.compstpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3stpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb3stpenr {{ lpuart1stpen: {=bool:?}, i2c3stpen: {=bool:?}, lptim1stpen: {=bool:?}, lptim3stpen: {=bool:?}, lptim4stpen: {=bool:?}, compstpen: {=bool:?} }}" , self . lpuart1stpen () , self . i2c3stpen () , self . lptim1stpen () , self . lptim3stpen () , self . lptim4stpen () , self . compstpen ())
        }
    }
    #[doc = "RCC backup domain control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enable."]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enable."]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready."]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready."]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass."]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass."]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator drive capability."]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator drive capability."]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "CSS on LSE enable."]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CSS on LSE enable."]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSS on LSE failure detection."]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CSS on LSE failure detection."]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LSE system clock (LSESYS) enable."]
        #[inline(always)]
        pub const fn lsesysen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LSE system clock (LSESYS) enable."]
        #[inline(always)]
        pub fn set_lsesysen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTC and TAMP clock source selection."]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC and TAMP clock source selection."]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LSE system clock (LSESYS) ready."]
        #[inline(always)]
        pub const fn lsesysrdy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LSE system clock (LSESYS) ready."]
        #[inline(always)]
        pub fn set_lsesysrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSE clock glitch filter enable."]
        #[inline(always)]
        pub const fn lsegfon(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock glitch filter enable."]
        #[inline(always)]
        pub fn set_lsegfon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RTC and TAMP clock enable."]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP clock enable."]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup domain software reset."]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset."]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Low-speed clock output (LSCO) enable."]
        #[inline(always)]
        pub const fn lscoen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Low-speed clock output (LSCO) enable."]
        #[inline(always)]
        pub fn set_lscoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Low-speed clock output selection."]
        #[inline(always)]
        pub const fn lscosel(&self) -> super::vals::Lscosel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Lscosel::from_bits(val as u8)
        }
        #[doc = "Low-speed clock output selection."]
        #[inline(always)]
        pub fn set_lscosel(&mut self, val: super::vals::Lscosel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    impl core::fmt::Debug for Bdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr")
                .field("lseon", &self.lseon())
                .field("lserdy", &self.lserdy())
                .field("lsebyp", &self.lsebyp())
                .field("lsedrv", &self.lsedrv())
                .field("lsecsson", &self.lsecsson())
                .field("lsecssd", &self.lsecssd())
                .field("lsesysen", &self.lsesysen())
                .field("rtcsel", &self.rtcsel())
                .field("lsesysrdy", &self.lsesysrdy())
                .field("lsegfon", &self.lsegfon())
                .field("rtcen", &self.rtcen())
                .field("bdrst", &self.bdrst())
                .field("lscoen", &self.lscoen())
                .field("lscosel", &self.lscosel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, lsedrv: {:?}, lsecsson: {=bool:?}, lsecssd: {=bool:?}, lsesysen: {=bool:?}, rtcsel: {:?}, lsesysrdy: {=bool:?}, lsegfon: {=bool:?}, rtcen: {=bool:?}, bdrst: {=bool:?}, lscoen: {=bool:?}, lscosel: {:?} }}" , self . lseon () , self . lserdy () , self . lsebyp () , self . lsedrv () , self . lsecsson () , self . lsecssd () , self . lsesysen () , self . rtcsel () , self . lsesysrdy () , self . lsegfon () , self . rtcen () , self . bdrst () , self . lscoen () , self . lscosel ())
        }
    }
    #[doc = "RCC peripheral independent clock configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr1(pub u32);
    impl Ccipr1 {
        #[doc = "USART1 kernel clock source selection."]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "USART3 kernel clock source selection."]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usart3sel {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Usart3sel::from_bits(val as u8)
        }
        #[doc = "USART3 kernel clock source selection."]
        #[inline(always)]
        pub fn set_usart3sel(&mut self, val: super::vals::Usart3sel) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "UART4 kernel clock source selection."]
        #[inline(always)]
        pub const fn uart4sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART4 kernel clock source selection."]
        #[inline(always)]
        pub fn set_uart4sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "UART5 kernel clock source selection."]
        #[inline(always)]
        pub const fn uart5sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART5 kernel clock source selection."]
        #[inline(always)]
        pub fn set_uart5sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "I3C1 kernel clock source selection."]
        #[inline(always)]
        pub const fn i3c1sel(&self) -> super::vals::I3csel {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::I3csel::from_bits(val as u8)
        }
        #[doc = "I3C1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_i3c1sel(&mut self, val: super::vals::I3csel) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "I2C1 kernel clock source selection."]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_i2c1sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "I2C2 kernel clock source selection."]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C2 kernel clock source selection."]
        #[inline(always)]
        pub fn set_i2c2sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "I3C2 kernel clock source selection."]
        #[inline(always)]
        pub const fn i3c2sel(&self) -> super::vals::I3c2sel {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::I3c2sel::from_bits(val as u8)
        }
        #[doc = "I3C2 kernel clock source selection."]
        #[inline(always)]
        pub fn set_i3c2sel(&mut self, val: super::vals::I3c2sel) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI2 kernel clock source selection."]
        #[inline(always)]
        pub const fn spi2sel(&self) -> super::vals::Spi2sel {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Spi2sel::from_bits(val as u8)
        }
        #[doc = "SPI2 kernel clock source selection."]
        #[inline(always)]
        pub fn set_spi2sel(&mut self, val: super::vals::Spi2sel) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Low-power timer 2 kernel clock source selection."]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "Low-power timer 2 kernel clock source selection."]
        #[inline(always)]
        pub fn set_lptim2sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "SPI1 kernel clock source selection."]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_spi1sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "SysTick clock source selection."]
        #[inline(always)]
        pub const fn systicksel(&self) -> super::vals::Systicksel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Systicksel::from_bits(val as u8)
        }
        #[doc = "SysTick clock source selection."]
        #[inline(always)]
        pub fn set_systicksel(&mut self, val: super::vals::Systicksel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "FDCAN1 kernel clock source selection."]
        #[inline(always)]
        pub const fn fdcan1sel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_fdcan1sel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Intermediate clock source selection."]
        #[inline(always)]
        pub const fn iclksel(&self) -> super::vals::Iclksel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Iclksel::from_bits(val as u8)
        }
        #[doc = "Intermediate clock source selection."]
        #[inline(always)]
        pub fn set_iclksel(&mut self, val: super::vals::Iclksel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "USB kernel clock prescaler selection."]
        #[inline(always)]
        pub const fn usbsel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB kernel clock prescaler selection."]
        #[inline(always)]
        pub fn set_usbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Clock sources for TIM16,TIM17, and LPTIM2 internal input capture."]
        #[inline(always)]
        pub const fn timicsel(&self) -> super::vals::Timicsel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Timicsel::from_bits(val as u8)
        }
        #[doc = "Clock sources for TIM16,TIM17, and LPTIM2 internal input capture."]
        #[inline(always)]
        pub fn set_timicsel(&mut self, val: super::vals::Timicsel) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Ccipr1 {
        #[inline(always)]
        fn default() -> Ccipr1 {
            Ccipr1(0)
        }
    }
    impl core::fmt::Debug for Ccipr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr1")
                .field("usart1sel", &self.usart1sel())
                .field("usart3sel", &self.usart3sel())
                .field("uart4sel", &self.uart4sel())
                .field("uart5sel", &self.uart5sel())
                .field("i3c1sel", &self.i3c1sel())
                .field("i2c1sel", &self.i2c1sel())
                .field("i2c2sel", &self.i2c2sel())
                .field("i3c2sel", &self.i3c2sel())
                .field("spi2sel", &self.spi2sel())
                .field("lptim2sel", &self.lptim2sel())
                .field("spi1sel", &self.spi1sel())
                .field("systicksel", &self.systicksel())
                .field("fdcan1sel", &self.fdcan1sel())
                .field("iclksel", &self.iclksel())
                .field("usbsel", &self.usbsel())
                .field("timicsel", &self.timicsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccipr1 {{ usart1sel: {:?}, usart3sel: {:?}, uart4sel: {:?}, uart5sel: {:?}, i3c1sel: {:?}, i2c1sel: {:?}, i2c2sel: {:?}, i3c2sel: {:?}, spi2sel: {:?}, lptim2sel: {:?}, spi1sel: {:?}, systicksel: {:?}, fdcan1sel: {:?}, iclksel: {:?}, usbsel: {=bool:?}, timicsel: {:?} }}" , self . usart1sel () , self . usart3sel () , self . uart4sel () , self . uart5sel () , self . i3c1sel () , self . i2c1sel () , self . i2c2sel () , self . i3c2sel () , self . spi2sel () , self . lptim2sel () , self . spi1sel () , self . systicksel () , self . fdcan1sel () , self . iclksel () , self . usbsel () , self . timicsel ())
        }
    }
    #[doc = "RCC peripheral independent clock configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "ADF1 kernel clock source selection."]
        #[inline(always)]
        pub const fn adf1sel(&self) -> super::vals::Adfsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Adfsel::from_bits(val as u8)
        }
        #[doc = "ADF1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_adf1sel(&mut self, val: super::vals::Adfsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "SPI3 kernel clock source selection."]
        #[inline(always)]
        pub const fn spi3sel(&self) -> super::vals::Spi3sel {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Spi3sel::from_bits(val as u8)
        }
        #[doc = "SPI3 kernel clock source selection."]
        #[inline(always)]
        pub fn set_spi3sel(&mut self, val: super::vals::Spi3sel) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "SAI1 kernel clock source selection."]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "RNG kernel clock source selection."]
        #[inline(always)]
        pub const fn rngsel(&self) -> super::vals::Rngsel {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Rngsel::from_bits(val as u8)
        }
        #[doc = "RNG kernel clock source selection."]
        #[inline(always)]
        pub fn set_rngsel(&mut self, val: super::vals::Rngsel) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "ADC12 and DAC1 kernel clock prescaler."]
        #[inline(always)]
        pub const fn adcdacpre(&self) -> super::vals::Adcdacpre {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Adcdacpre::from_bits(val as u8)
        }
        #[doc = "ADC12 and DAC1 kernel clock prescaler."]
        #[inline(always)]
        pub fn set_adcdacpre(&mut self, val: super::vals::Adcdacpre) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "ADC12 and DAC1 intermediate kernel clock source selection."]
        #[inline(always)]
        pub const fn adcdacsel(&self) -> super::vals::Adcdacsel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Adcdacsel::from_bits(val as u8)
        }
        #[doc = "ADC12 and DAC1 intermediate kernel clock source selection."]
        #[inline(always)]
        pub fn set_adcdacsel(&mut self, val: super::vals::Adcdacsel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "DAC1 sample and hold clock source selection."]
        #[inline(always)]
        pub const fn dac1shsel(&self) -> super::vals::Dacshsel {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Dacshsel::from_bits(val as u8)
        }
        #[doc = "DAC1 sample and hold clock source selection."]
        #[inline(always)]
        pub fn set_dac1shsel(&mut self, val: super::vals::Dacshsel) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI1 kernel clock source selection."]
        #[inline(always)]
        pub const fn octospisel(&self) -> super::vals::Octospisel {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Octospisel::from_bits(val as u8)
        }
        #[doc = "OCTOSPI1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_octospisel(&mut self, val: super::vals::Octospisel) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ccipr2 {
        #[inline(always)]
        fn default() -> Ccipr2 {
            Ccipr2(0)
        }
    }
    impl core::fmt::Debug for Ccipr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr2")
                .field("adf1sel", &self.adf1sel())
                .field("spi3sel", &self.spi3sel())
                .field("sai1sel", &self.sai1sel())
                .field("rngsel", &self.rngsel())
                .field("adcdacpre", &self.adcdacpre())
                .field("adcdacsel", &self.adcdacsel())
                .field("dac1shsel", &self.dac1shsel())
                .field("octospisel", &self.octospisel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccipr2 {{ adf1sel: {:?}, spi3sel: {:?}, sai1sel: {:?}, rngsel: {:?}, adcdacpre: {:?}, adcdacsel: {:?}, dac1shsel: {:?}, octospisel: {:?} }}" , self . adf1sel () , self . spi3sel () , self . sai1sel () , self . rngsel () , self . adcdacpre () , self . adcdacsel () , self . dac1shsel () , self . octospisel ())
        }
    }
    #[doc = "RCC peripheral independent clock configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr3(pub u32);
    impl Ccipr3 {
        #[doc = "LPUART1 kernel clock source selection."]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "I2C3 kernel clock source selection."]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> super::vals::I2c3sel {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::I2c3sel::from_bits(val as u8)
        }
        #[doc = "I2C3 kernel clock source selection."]
        #[inline(always)]
        pub fn set_i2c3sel(&mut self, val: super::vals::I2c3sel) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "LPTIM3 and LPTIM4 kernel clock source selection."]
        #[inline(always)]
        pub const fn lptim34sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM3 and LPTIM4 kernel clock source selection."]
        #[inline(always)]
        pub fn set_lptim34sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LPTIM1 kernel clock source selection."]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection."]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
    }
    impl Default for Ccipr3 {
        #[inline(always)]
        fn default() -> Ccipr3 {
            Ccipr3(0)
        }
    }
    impl core::fmt::Debug for Ccipr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr3")
                .field("lpuart1sel", &self.lpuart1sel())
                .field("i2c3sel", &self.i2c3sel())
                .field("lptim34sel", &self.lptim34sel())
                .field("lptim1sel", &self.lptim1sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr3 {{ lpuart1sel: {:?}, i2c3sel: {:?}, lptim34sel: {:?}, lptim1sel: {:?} }}",
                self.lpuart1sel(),
                self.i2c3sel(),
                self.lptim34sel(),
                self.lptim1sel()
            )
        }
    }
    #[doc = "RCC clock configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "System clock switch."]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch."]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status."]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sws {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sws::from_bits(val as u8)
        }
        #[doc = "System clock switch status."]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sws) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Wake-up from Stop and CSS backup clock selection."]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "Wake-up from Stop and CSS backup clock selection."]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Wake-up from Stop kernel clock automatic enable selection."]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up from Stop kernel clock automatic enable selection."]
        #[inline(always)]
        pub fn set_stopkerwuck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Microcontroller clock output 2."]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 2."]
        #[inline(always)]
        pub fn set_mco2sel(&mut self, val: super::vals::Mco2sel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Microcontroller clock output 2 prescaler."]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 2 prescaler."]
        #[inline(always)]
        pub fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "Microcontroller clock output."]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output."]
        #[inline(always)]
        pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Microcontroller clock output prescaler."]
        #[inline(always)]
        pub const fn mcopre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output prescaler."]
        #[inline(always)]
        pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
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
                .field("sw", &self.sw())
                .field("sws", &self.sws())
                .field("stopwuck", &self.stopwuck())
                .field("stopkerwuck", &self.stopkerwuck())
                .field("mco2sel", &self.mco2sel())
                .field("mco2pre", &self.mco2pre())
                .field("mcosel", &self.mcosel())
                .field("mcopre", &self.mcopre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ sw: {:?}, sws: {:?}, stopwuck: {:?}, stopkerwuck: {=bool:?}, mco2sel: {:?}, mco2pre: {:?}, mcosel: {:?}, mcopre: {:?} }}" , self . sw () , self . sws () , self . stopwuck () , self . stopkerwuck () , self . mco2sel () , self . mco2pre () , self . mcosel () , self . mcopre ())
        }
    }
    #[doc = "RCC clock configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "AHB prescaler."]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler."]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "APB1 prescaler."]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB1 prescaler."]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "APB2 prescaler."]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB2 prescaler."]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
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
                .field("hpre", &self.hpre())
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ hpre: {:?}, ppre1: {:?}, ppre2: {:?} }}",
                self.hpre(),
                self.ppre1(),
                self.ppre2()
            )
        }
    }
    #[doc = "RCC clock configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "APB3 prescaler."]
        #[inline(always)]
        pub const fn ppre3(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB3 prescaler."]
        #[inline(always)]
        pub fn set_ppre3(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
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
            f.debug_struct("Cfgr3").field("ppre3", &self.ppre3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfgr3 {{ ppre3: {:?} }}", self.ppre3())
        }
    }
    #[doc = "RCC clock configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr4(pub u32);
    impl Cfgr4 {
        #[doc = "EPOD booster input clock source selection."]
        #[inline(always)]
        pub const fn boostsel(&self) -> super::vals::Boostsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Boostsel::from_bits(val as u8)
        }
        #[doc = "EPOD booster input clock source selection."]
        #[inline(always)]
        pub fn set_boostsel(&mut self, val: super::vals::Boostsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "EPOD booster input clock prescaler."]
        #[inline(always)]
        pub const fn boostdiv(&self) -> super::vals::Boostdiv {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Boostdiv::from_bits(val as u8)
        }
        #[doc = "EPOD booster input clock prescaler."]
        #[inline(always)]
        pub fn set_boostdiv(&mut self, val: super::vals::Boostdiv) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
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
                .field("boostsel", &self.boostsel())
                .field("boostdiv", &self.boostdiv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr4 {{ boostsel: {:?}, boostdiv: {:?} }}",
                self.boostsel(),
                self.boostdiv()
            )
        }
    }
    #[doc = "RCC clock interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear."]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear."]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear."]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear."]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt clear."]
        #[inline(always)]
        pub const fn msisrdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt clear."]
        #[inline(always)]
        pub fn set_msisrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI16 ready interrupt clear."]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI16 ready interrupt clear."]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear."]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear."]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt clear."]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt clear."]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MSIRC1 PLL mode ready interrupt clear."]
        #[inline(always)]
        pub const fn msipll1rdyc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC1 PLL mode ready interrupt clear."]
        #[inline(always)]
        pub fn set_msipll1rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MSIRC0 PLL mode ready interrupt clear."]
        #[inline(always)]
        pub const fn msipll0rdyc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC0 PLL mode ready interrupt clear."]
        #[inline(always)]
        pub fn set_msipll0rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MSI PLL mode with LSE unlock interrupt clear."]
        #[inline(always)]
        pub const fn msiplluc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MSI PLL mode with LSE unlock interrupt clear."]
        #[inline(always)]
        pub fn set_msiplluc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MSI PLL mode with HSE unlock interrupt clear."]
        #[inline(always)]
        pub const fn msipllhsuc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MSI PLL mode with HSE unlock interrupt clear."]
        #[inline(always)]
        pub fn set_msipllhsuc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock security system interrupt clear."]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear."]
        #[inline(always)]
        pub fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MSIK oscillator ready interrupt clear."]
        #[inline(always)]
        pub const fn msikrdyc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK oscillator ready interrupt clear."]
        #[inline(always)]
        pub fn set_msikrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSE CSS interrupt clear."]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LSE CSS interrupt clear."]
        #[inline(always)]
        pub fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Cicr {
        #[inline(always)]
        fn default() -> Cicr {
            Cicr(0)
        }
    }
    impl core::fmt::Debug for Cicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cicr")
                .field("lsirdyc", &self.lsirdyc())
                .field("lserdyc", &self.lserdyc())
                .field("msisrdyc", &self.msisrdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("hsi48rdyc", &self.hsi48rdyc())
                .field("msipll1rdyc", &self.msipll1rdyc())
                .field("msipll0rdyc", &self.msipll0rdyc())
                .field("msiplluc", &self.msiplluc())
                .field("msipllhsuc", &self.msipllhsuc())
                .field("cssc", &self.cssc())
                .field("msikrdyc", &self.msikrdyc())
                .field("lsecssc", &self.lsecssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cicr {{ lsirdyc: {=bool:?}, lserdyc: {=bool:?}, msisrdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, hsi48rdyc: {=bool:?}, msipll1rdyc: {=bool:?}, msipll0rdyc: {=bool:?}, msiplluc: {=bool:?}, msipllhsuc: {=bool:?}, cssc: {=bool:?}, msikrdyc: {=bool:?}, lsecssc: {=bool:?} }}" , self . lsirdyc () , self . lserdyc () , self . msisrdyc () , self . hsirdyc () , self . hserdyc () , self . hsi48rdyc () , self . msipll1rdyc () , self . msipll0rdyc () , self . msiplluc () , self . msipllhsuc () , self . cssc () , self . msikrdyc () , self . lsecssc ())
        }
    }
    #[doc = "RCC clock interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable."]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable."]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable."]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable."]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt enable."]
        #[inline(always)]
        pub const fn msisrdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt enable."]
        #[inline(always)]
        pub fn set_msisrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI16 ready interrupt enable."]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI16 ready interrupt enable."]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable."]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable."]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt enable."]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt enable."]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MSIRC1 PLL mode ready interrupt enable."]
        #[inline(always)]
        pub const fn msipll1rdyie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC1 PLL mode ready interrupt enable."]
        #[inline(always)]
        pub fn set_msipll1rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MSIRC0 PLL mode ready interrupt enable."]
        #[inline(always)]
        pub const fn msipll0rdyie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC0 PLL mode ready interrupt enable."]
        #[inline(always)]
        pub fn set_msipll0rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MSI PLL mode with LSE unlock interrupt enable."]
        #[inline(always)]
        pub const fn msiplluie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MSI PLL mode with LSE unlock interrupt enable."]
        #[inline(always)]
        pub fn set_msiplluie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MSI PLL mode with HSE unlock interrupt enable."]
        #[inline(always)]
        pub const fn msipllhsuie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MSI PLL mode with HSE unlock interrupt enable."]
        #[inline(always)]
        pub fn set_msipllhsuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MSIK ready interrupt enable."]
        #[inline(always)]
        pub const fn msikrdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK ready interrupt enable."]
        #[inline(always)]
        pub fn set_msikrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSE clock security system interrupt enable."]
        #[inline(always)]
        pub const fn lsecssie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt enable."]
        #[inline(always)]
        pub fn set_lsecssie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("msisrdyie", &self.msisrdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("hsi48rdyie", &self.hsi48rdyie())
                .field("msipll1rdyie", &self.msipll1rdyie())
                .field("msipll0rdyie", &self.msipll0rdyie())
                .field("msiplluie", &self.msiplluie())
                .field("msipllhsuie", &self.msipllhsuie())
                .field("msikrdyie", &self.msikrdyie())
                .field("lsecssie", &self.lsecssie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cier {{ lsirdyie: {=bool:?}, lserdyie: {=bool:?}, msisrdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, hsi48rdyie: {=bool:?}, msipll1rdyie: {=bool:?}, msipll0rdyie: {=bool:?}, msiplluie: {=bool:?}, msipllhsuie: {=bool:?}, msikrdyie: {=bool:?}, lsecssie: {=bool:?} }}" , self . lsirdyie () , self . lserdyie () , self . msisrdyie () , self . hsirdyie () , self . hserdyie () , self . hsi48rdyie () , self . msipll1rdyie () , self . msipll0rdyie () , self . msiplluie () , self . msipllhsuie () , self . msikrdyie () , self . lsecssie ())
        }
    }
    #[doc = "RCC clock interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag."]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag."]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag."]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag."]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt flag."]
        #[inline(always)]
        pub const fn msisrdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt flag."]
        #[inline(always)]
        pub fn set_msisrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI16 ready interrupt flag."]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI16 ready interrupt flag."]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag."]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag."]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt flag."]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt flag."]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MSIRC1 PLL mode ready interrupt enable."]
        #[inline(always)]
        pub const fn msipll1rdyf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC1 PLL mode ready interrupt enable."]
        #[inline(always)]
        pub fn set_msipll1rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MSIRC0 PLL mode ready interrupt flag."]
        #[inline(always)]
        pub const fn msipll0rdyf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC0 PLL mode ready interrupt flag."]
        #[inline(always)]
        pub fn set_msipll0rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MSI PLL mode with LSE unlock interrupt flag."]
        #[inline(always)]
        pub const fn msiplluf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MSI PLL mode with LSE unlock interrupt flag."]
        #[inline(always)]
        pub fn set_msiplluf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MSI PLL mode with HSE unlock interrupt flag."]
        #[inline(always)]
        pub const fn msipllhsuf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MSI PLL mode with HSE unlock interrupt flag."]
        #[inline(always)]
        pub fn set_msipllhsuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock security system interrupt flag."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt flag."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MSIK ready interrupt flag."]
        #[inline(always)]
        pub const fn msikrdyf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK ready interrupt flag."]
        #[inline(always)]
        pub fn set_msikrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSE clock security system interrupt flag."]
        #[inline(always)]
        pub const fn lsecssf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt flag."]
        #[inline(always)]
        pub fn set_lsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("lsirdyf", &self.lsirdyf())
                .field("lserdyf", &self.lserdyf())
                .field("msisrdyf", &self.msisrdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("hsi48rdyf", &self.hsi48rdyf())
                .field("msipll1rdyf", &self.msipll1rdyf())
                .field("msipll0rdyf", &self.msipll0rdyf())
                .field("msiplluf", &self.msiplluf())
                .field("msipllhsuf", &self.msipllhsuf())
                .field("cssf", &self.cssf())
                .field("msikrdyf", &self.msikrdyf())
                .field("lsecssf", &self.lsecssf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cifr {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, msisrdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, hsi48rdyf: {=bool:?}, msipll1rdyf: {=bool:?}, msipll0rdyf: {=bool:?}, msiplluf: {=bool:?}, msipllhsuf: {=bool:?}, cssf: {=bool:?}, msikrdyf: {=bool:?}, lsecssf: {=bool:?} }}" , self . lsirdyf () , self . lserdyf () , self . msisrdyf () , self . hsirdyf () , self . hserdyf () , self . hsi48rdyf () , self . msipll1rdyf () , self . msipll0rdyf () , self . msiplluf () , self . msipllhsuf () , self . cssf () , self . msikrdyf () , self . lsecssf ())
        }
    }
    #[doc = "RCC clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "MSIS clock enable."]
        #[inline(always)]
        pub const fn msison(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS clock enable."]
        #[inline(always)]
        pub fn set_msison(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MSI enable for some peripheral kernels."]
        #[inline(always)]
        pub const fn msikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MSI enable for some peripheral kernels."]
        #[inline(always)]
        pub fn set_msikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS clock ready flag."]
        #[inline(always)]
        pub const fn msisrdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS clock ready flag."]
        #[inline(always)]
        pub fn set_msisrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MSIK clock enable."]
        #[inline(always)]
        pub const fn msikon(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK clock enable."]
        #[inline(always)]
        pub fn set_msikon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MSIK clock ready flag."]
        #[inline(always)]
        pub const fn msikrdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK clock ready flag."]
        #[inline(always)]
        pub fn set_msikrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MSIRC1 PLL mode enable."]
        #[inline(always)]
        pub const fn msipll1en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC1 PLL mode enable."]
        #[inline(always)]
        pub fn set_msipll1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MSIRC0 PLL mode enable."]
        #[inline(always)]
        pub const fn msipll0en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC0 PLL mode enable."]
        #[inline(always)]
        pub fn set_msipll0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MSIRC1 PLL mode fast startup."]
        #[inline(always)]
        pub const fn msipll1fast(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC1 PLL mode fast startup."]
        #[inline(always)]
        pub fn set_msipll1fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MSIRC0 PLL mode fast startup."]
        #[inline(always)]
        pub const fn msipll0fast(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC0 PLL mode fast startup."]
        #[inline(always)]
        pub fn set_msipll0fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MSIRC1 PLL mode ready flag."]
        #[inline(always)]
        pub const fn msipll1rdy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC1 PLL mode ready flag."]
        #[inline(always)]
        pub fn set_msipll1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MSIRC0 PLL mode ready flag."]
        #[inline(always)]
        pub const fn msipll0rdy(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MSIRC0 PLL mode ready flag."]
        #[inline(always)]
        pub fn set_msipll0rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI16 clock enable."]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSI16 clock enable."]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HSI16 enable for some peripheral kernels."]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSI16 enable for some peripheral kernels."]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HSI16 clock ready flag."]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HSI16 clock ready flag."]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HSI48 clock enable."]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock enable."]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "HSI48 clock ready flag."]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock ready flag."]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HSE clock enable."]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable."]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag."]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag."]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE crystal oscillator bypass."]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE crystal oscillator bypass."]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock security system enable."]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system enable."]
        #[inline(always)]
        pub fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HSE external clock bypass mode."]
        #[inline(always)]
        pub const fn hseext(&self) -> super::vals::Hseext {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Hseext::from_bits(val as u8)
        }
        #[doc = "HSE external clock bypass mode."]
        #[inline(always)]
        pub fn set_hseext(&mut self, val: super::vals::Hseext) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
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
                .field("msison", &self.msison())
                .field("msikeron", &self.msikeron())
                .field("msisrdy", &self.msisrdy())
                .field("msikon", &self.msikon())
                .field("msikrdy", &self.msikrdy())
                .field("msipll1en", &self.msipll1en())
                .field("msipll0en", &self.msipll0en())
                .field("msipll1fast", &self.msipll1fast())
                .field("msipll0fast", &self.msipll0fast())
                .field("msipll1rdy", &self.msipll1rdy())
                .field("msipll0rdy", &self.msipll0rdy())
                .field("hsion", &self.hsion())
                .field("hsikeron", &self.hsikeron())
                .field("hsirdy", &self.hsirdy())
                .field("hsi48on", &self.hsi48on())
                .field("hsi48rdy", &self.hsi48rdy())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("hsecsson", &self.hsecsson())
                .field("hseext", &self.hseext())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ msison: {=bool:?}, msikeron: {=bool:?}, msisrdy: {=bool:?}, msikon: {=bool:?}, msikrdy: {=bool:?}, msipll1en: {=bool:?}, msipll0en: {=bool:?}, msipll1fast: {=bool:?}, msipll0fast: {=bool:?}, msipll1rdy: {=bool:?}, msipll0rdy: {=bool:?}, hsion: {=bool:?}, hsikeron: {=bool:?}, hsirdy: {=bool:?}, hsi48on: {=bool:?}, hsi48rdy: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, hsecsson: {=bool:?}, hseext: {:?} }}" , self . msison () , self . msikeron () , self . msisrdy () , self . msikon () , self . msikrdy () , self . msipll1en () , self . msipll0en () , self . msipll1fast () , self . msipll0fast () , self . msipll1rdy () , self . msipll0rdy () , self . hsion () , self . hsikeron () , self . hsirdy () , self . hsi48on () , self . hsi48rdy () , self . hseon () , self . hserdy () , self . hsebyp () , self . hsecsson () , self . hseext ())
        }
    }
    #[doc = "RCC clock recovery RC register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "HSI48 clock calibration."]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSI48 clock calibration."]
        #[inline(always)]
        pub fn set_hsi48cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Crrcr {
        #[inline(always)]
        fn default() -> Crrcr {
            Crrcr(0)
        }
    }
    impl core::fmt::Debug for Crrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crrcr").field("hsi48cal", &self.hsi48cal()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crrcr {{ hsi48cal: {=u16:?} }}", self.hsi48cal())
        }
    }
    #[doc = "RCC control/status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "LSI oscillator enable."]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable."]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSI oscillator ready."]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready."]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Low-speed clock divider configuration."]
        #[inline(always)]
        pub const fn lsiprediv(&self) -> super::vals::Lsiprediv {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Lsiprediv::from_bits(val as u8)
        }
        #[doc = "Low-speed clock divider configuration."]
        #[inline(always)]
        pub fn set_lsiprediv(&mut self, val: super::vals::Lsiprediv) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "MSIK oscillator division after Standby mode."]
        #[inline(always)]
        pub const fn msikdivs(&self) -> super::vals::Msikdivs {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Msikdivs::from_bits(val as u8)
        }
        #[doc = "MSIK oscillator division after Standby mode."]
        #[inline(always)]
        pub fn set_msikdivs(&mut self, val: super::vals::Msikdivs) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "MSIS oscillator division after Standby mode."]
        #[inline(always)]
        pub const fn msisdivs(&self) -> super::vals::Msisdivs {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Msisdivs::from_bits(val as u8)
        }
        #[doc = "MSIS oscillator division after Standby mode."]
        #[inline(always)]
        pub fn set_msisdivs(&mut self, val: super::vals::Msisdivs) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Remove reset flag."]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag."]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Option-byte loader reset flag."]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Option-byte loader reset flag."]
        #[inline(always)]
        pub fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "NRST pin reset flag."]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "NRST pin reset flag."]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BOR flag."]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BOR flag."]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag."]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag."]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag."]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag."]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag."]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag."]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag."]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag."]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .field("lsiprediv", &self.lsiprediv())
                .field("msikdivs", &self.msikdivs())
                .field("msisdivs", &self.msisdivs())
                .field("rmvf", &self.rmvf())
                .field("oblrstf", &self.oblrstf())
                .field("pinrstf", &self.pinrstf())
                .field("borrstf", &self.borrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdgrstf", &self.iwdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ lsion: {=bool:?}, lsirdy: {=bool:?}, lsiprediv: {:?}, msikdivs: {:?}, msisdivs: {:?}, rmvf: {=bool:?}, oblrstf: {=bool:?}, pinrstf: {=bool:?}, borrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}" , self . lsion () , self . lsirdy () , self . lsiprediv () , self . msikdivs () , self . msisdivs () , self . rmvf () , self . oblrstf () , self . pinrstf () , self . borrstf () , self . sftrstf () , self . iwdgrstf () , self . wwdgrstf () , self . lpwrrstf ())
        }
    }
    #[doc = "RCC internal clock source calibration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr1(pub u32);
    impl Icscr1 {
        #[doc = "MSIRC1 clock calibration for MSI ranges 4 to 7."]
        #[inline(always)]
        pub const fn msical1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "MSIRC1 clock calibration for MSI ranges 4 to 7."]
        #[inline(always)]
        pub fn set_msical1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "MSIRC0 clock calibration for MSI ranges 0 to 3."]
        #[inline(always)]
        pub const fn msical0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "MSIRC0 clock calibration for MSI ranges 0 to 3."]
        #[inline(always)]
        pub fn set_msical0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "MSIRCx (x = 0, 1) PLL mode HSE input division."]
        #[inline(always)]
        pub const fn msihsindiv(&self) -> super::vals::Msihsindiv {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Msihsindiv::from_bits(val as u8)
        }
        #[doc = "MSIRCx (x = 0, 1) PLL mode HSE input division."]
        #[inline(always)]
        pub fn set_msihsindiv(&mut self, val: super::vals::Msihsindiv) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "MSIRC1 PLL mode input clock selection."]
        #[inline(always)]
        pub const fn msipll1sel(&self) -> super::vals::Msipllsel {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Msipllsel::from_bits(val as u8)
        }
        #[doc = "MSIRC1 PLL mode input clock selection."]
        #[inline(always)]
        pub fn set_msipll1sel(&mut self, val: super::vals::Msipllsel) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "MSIRC0 PLL mode input clock selection."]
        #[inline(always)]
        pub const fn msipll0sel(&self) -> super::vals::Msipllsel {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Msipllsel::from_bits(val as u8)
        }
        #[doc = "MSIRC0 PLL mode input clock selection."]
        #[inline(always)]
        pub fn set_msipll0sel(&mut self, val: super::vals::Msipllsel) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "MSI bias mode selection."]
        #[inline(always)]
        pub const fn msibias(&self) -> super::vals::Msibias {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Msibias::from_bits(val as u8)
        }
        #[doc = "MSI bias mode selection."]
        #[inline(always)]
        pub fn set_msibias(&mut self, val: super::vals::Msibias) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "MSI clock range selection."]
        #[inline(always)]
        pub const fn msirgsel(&self) -> super::vals::Msirgsel {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Msirgsel::from_bits(val as u8)
        }
        #[doc = "MSI clock range selection."]
        #[inline(always)]
        pub fn set_msirgsel(&mut self, val: super::vals::Msirgsel) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "MSIRC1 PLL mode with LSE multiplication factor."]
        #[inline(always)]
        pub const fn msipll1n(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "MSIRC1 PLL mode with LSE multiplication factor."]
        #[inline(always)]
        pub fn set_msipll1n(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "MSIK oscillator division."]
        #[inline(always)]
        pub const fn msikdiv(&self) -> super::vals::Msikdiv {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Msikdiv::from_bits(val as u8)
        }
        #[doc = "MSIK oscillator division."]
        #[inline(always)]
        pub fn set_msikdiv(&mut self, val: super::vals::Msikdiv) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "MSIK clock source selection."]
        #[inline(always)]
        pub const fn msiksel(&self) -> super::vals::Msiksel {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Msiksel::from_bits(val as u8)
        }
        #[doc = "MSIK clock source selection."]
        #[inline(always)]
        pub fn set_msiksel(&mut self, val: super::vals::Msiksel) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "MSIS oscillator division."]
        #[inline(always)]
        pub const fn msisdiv(&self) -> super::vals::Msisdiv {
            let val = (self.0 >> 29usize) & 0x03;
            super::vals::Msisdiv::from_bits(val as u8)
        }
        #[doc = "MSIS oscillator division."]
        #[inline(always)]
        pub fn set_msisdiv(&mut self, val: super::vals::Msisdiv) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
        }
        #[doc = "MSIS clock source selection."]
        #[inline(always)]
        pub const fn msissel(&self) -> super::vals::Msissel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Msissel::from_bits(val as u8)
        }
        #[doc = "MSIS clock source selection."]
        #[inline(always)]
        pub fn set_msissel(&mut self, val: super::vals::Msissel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icscr1 {
        #[inline(always)]
        fn default() -> Icscr1 {
            Icscr1(0)
        }
    }
    impl core::fmt::Debug for Icscr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr1")
                .field("msical1", &self.msical1())
                .field("msical0", &self.msical0())
                .field("msihsindiv", &self.msihsindiv())
                .field("msipll1sel", &self.msipll1sel())
                .field("msipll0sel", &self.msipll0sel())
                .field("msibias", &self.msibias())
                .field("msirgsel", &self.msirgsel())
                .field("msipll1n", &self.msipll1n())
                .field("msikdiv", &self.msikdiv())
                .field("msiksel", &self.msiksel())
                .field("msisdiv", &self.msisdiv())
                .field("msissel", &self.msissel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icscr1 {{ msical1: {=u8:?}, msical0: {=u8:?}, msihsindiv: {:?}, msipll1sel: {:?}, msipll0sel: {:?}, msibias: {:?}, msirgsel: {:?}, msipll1n: {=u8:?}, msikdiv: {:?}, msiksel: {:?}, msisdiv: {:?}, msissel: {:?} }}" , self . msical1 () , self . msical0 () , self . msihsindiv () , self . msipll1sel () , self . msipll0sel () , self . msibias () , self . msirgsel () , self . msipll1n () , self . msikdiv () , self . msiksel () , self . msisdiv () , self . msissel ())
        }
    }
    #[doc = "RCC internal clock source calibration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr2(pub u32);
    impl Icscr2 {
        #[doc = "MSIRC1 clock trimming for ranges 4 to 7."]
        #[inline(always)]
        pub const fn msitrim1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "MSIRC1 clock trimming for ranges 4 to 7."]
        #[inline(always)]
        pub fn set_msitrim1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "MSIRC0 clock trimming for ranges 0 to 3."]
        #[inline(always)]
        pub const fn msitrim0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "MSIRC0 clock trimming for ranges 0 to 3."]
        #[inline(always)]
        pub fn set_msitrim0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
    }
    impl Default for Icscr2 {
        #[inline(always)]
        fn default() -> Icscr2 {
            Icscr2(0)
        }
    }
    impl core::fmt::Debug for Icscr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr2")
                .field("msitrim1", &self.msitrim1())
                .field("msitrim0", &self.msitrim0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icscr2 {{ msitrim1: {=u8:?}, msitrim0: {=u8:?} }}",
                self.msitrim1(),
                self.msitrim0()
            )
        }
    }
    #[doc = "RCC internal clock source calibration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr3(pub u32);
    impl Icscr3 {
        #[doc = "HSI clock calibration."]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration."]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming."]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HSI clock trimming."]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Icscr3 {
        #[inline(always)]
        fn default() -> Icscr3 {
            Icscr3(0)
        }
    }
    impl core::fmt::Debug for Icscr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr3")
                .field("hsical", &self.hsical())
                .field("hsitrim", &self.hsitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icscr3 {{ hsical: {=u16:?}, hsitrim: {=u8:?} }}",
                self.hsical(),
                self.hsitrim()
            )
        }
    }
    #[doc = "RCC privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "RCC secure function privilege configuration."]
        #[inline(always)]
        pub const fn spriv(&self) -> super::vals::Security {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "RCC secure function privilege configuration."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "RCC nonsecure function privilege configuration."]
        #[inline(always)]
        pub const fn nspriv(&self) -> super::vals::Security {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "RCC nonsecure function privilege configuration."]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("spriv", &self.spriv())
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {:?}, nspriv: {:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[doc = "RCC secure configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "HSI clock configuration and status bit security."]
        #[inline(always)]
        pub const fn hsisec(&self) -> super::vals::Security {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSI clock configuration and status bit security."]
        #[inline(always)]
        pub fn set_hsisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "HSE clock configuration, status bits, and HSE_CSS security."]
        #[inline(always)]
        pub const fn hsesec(&self) -> super::vals::Security {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSE clock configuration, status bits, and HSE_CSS security."]
        #[inline(always)]
        pub fn set_hsesec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI clock configuration and status bit security."]
        #[inline(always)]
        pub const fn msisec(&self) -> super::vals::Security {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "MSI clock configuration and status bit security."]
        #[inline(always)]
        pub fn set_msisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "LSI clock configuration and status bit security."]
        #[inline(always)]
        pub const fn lsisec(&self) -> super::vals::Security {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "LSI clock configuration and status bit security."]
        #[inline(always)]
        pub fn set_lsisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "LSE clock configuration and status bit security."]
        #[inline(always)]
        pub const fn lsesec(&self) -> super::vals::Security {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "LSE clock configuration and status bit security."]
        #[inline(always)]
        pub fn set_lsesec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security."]
        #[inline(always)]
        pub const fn sysclksec(&self) -> super::vals::Security {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security."]
        #[inline(always)]
        pub fn set_sysclksec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "AHBx/APBx prescaler configuration bit security."]
        #[inline(always)]
        pub const fn prescsec(&self) -> super::vals::Security {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "AHBx/APBx prescaler configuration bit security."]
        #[inline(always)]
        pub fn set_prescsec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "EPOD booster configuration bit security."]
        #[inline(always)]
        pub const fn boostsec(&self) -> super::vals::Security {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "EPOD booster configuration bit security."]
        #[inline(always)]
        pub fn set_boostsec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Intermediate clock source selection security."]
        #[inline(always)]
        pub const fn iclksec(&self) -> super::vals::Security {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "Intermediate clock source selection security."]
        #[inline(always)]
        pub fn set_iclksec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI48 clock configuration and status bit security."]
        #[inline(always)]
        pub const fn hsi48sec(&self) -> super::vals::Security {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSI48 clock configuration and status bit security."]
        #[inline(always)]
        pub fn set_hsi48sec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Remove reset flag security."]
        #[inline(always)]
        pub const fn rmvfsec(&self) -> super::vals::Security {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "Remove reset flag security."]
        #[inline(always)]
        pub fn set_rmvfsec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("hsisec", &self.hsisec())
                .field("hsesec", &self.hsesec())
                .field("msisec", &self.msisec())
                .field("lsisec", &self.lsisec())
                .field("lsesec", &self.lsesec())
                .field("sysclksec", &self.sysclksec())
                .field("prescsec", &self.prescsec())
                .field("boostsec", &self.boostsec())
                .field("iclksec", &self.iclksec())
                .field("hsi48sec", &self.hsi48sec())
                .field("rmvfsec", &self.rmvfsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr {{ hsisec: {:?}, hsesec: {:?}, msisec: {:?}, lsisec: {:?}, lsesec: {:?}, sysclksec: {:?}, prescsec: {:?}, boostsec: {:?}, iclksec: {:?}, hsi48sec: {:?}, rmvfsec: {:?} }}" , self . hsisec () , self . hsesec () , self . msisec () , self . lsisec () , self . lsesec () , self . sysclksec () , self . prescsec () , self . boostsec () , self . iclksec () , self . hsi48sec () , self . rmvfsec ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcdacpre {
        #[doc = "adcdac_iclk."]
        DIV1 = 0x0,
        #[doc = "adcdac_iclk/2."]
        DIV2 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "adcdac_iclk/4."]
        DIV4 = 0x08,
        #[doc = "adcdac_iclk/8."]
        DIV8 = 0x09,
        #[doc = "adcdac_iclk/16."]
        DIV16 = 0x0a,
        #[doc = "adcdac_iclk/32."]
        DIV32 = 0x0b,
        #[doc = "adcdac_iclk/64."]
        DIV64 = 0x0c,
        #[doc = "adcdac_iclk/128."]
        DIV128 = 0x0d,
        #[doc = "adcdac_iclk/256."]
        DIV256 = 0x0e,
        #[doc = "adcdac_iclk/512."]
        DIV512 = 0x0f,
    }
    impl Adcdacpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcdacpre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcdacpre {
        #[inline(always)]
        fn from(val: u8) -> Adcdacpre {
            Adcdacpre::from_bits(val)
        }
    }
    impl From<Adcdacpre> for u8 {
        #[inline(always)]
        fn from(val: Adcdacpre) -> u8 {
            Adcdacpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcdacsel {
        #[doc = "HCLK selected."]
        HCLK1 = 0x0,
        #[doc = "HSE selected."]
        HSE = 0x01,
        #[doc = "MSIK selected."]
        MSIK = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Adcdacsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcdacsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcdacsel {
        #[inline(always)]
        fn from(val: u8) -> Adcdacsel {
            Adcdacsel::from_bits(val)
        }
    }
    impl From<Adcdacsel> for u8 {
        #[inline(always)]
        fn from(val: Adcdacsel) -> u8 {
            Adcdacsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adfsel {
        #[doc = "HCLK."]
        HCLK1 = 0x0,
        #[doc = "Input pin AUDIOCLK selected."]
        AUDIOCLK = 0x01,
        #[doc = "MSIK clock selected."]
        MSIK = 0x02,
        #[doc = "SAI1 kernel clock selected."]
        SAI1 = 0x03,
    }
    impl Adfsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adfsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adfsel {
        #[inline(always)]
        fn from(val: u8) -> Adfsel {
            Adfsel::from_bits(val)
        }
    }
    impl From<Adfsel> for u8 {
        #[inline(always)]
        fn from(val: Adfsel) -> u8 {
            Adfsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Boostdiv {
        #[doc = "Divided by 1 (bypass)."]
        DIV1 = 0x0,
        #[doc = "Divided by 2."]
        DIV2 = 0x01,
        #[doc = "Divided by 4."]
        DIV4 = 0x02,
        #[doc = "Divided by 6."]
        DIV6 = 0x03,
        #[doc = "Divided by 8."]
        DIV8 = 0x04,
        #[doc = "Divided by 10."]
        DIV10 = 0x05,
        #[doc = "Divided by 12."]
        DIV12 = 0x06,
        #[doc = "Divided by 14."]
        DIV14 = 0x07,
        #[doc = "Divided by 16."]
        DIV16 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Boostdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Boostdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Boostdiv {
        #[inline(always)]
        fn from(val: u8) -> Boostdiv {
            Boostdiv::from_bits(val)
        }
    }
    impl From<Boostdiv> for u8 {
        #[inline(always)]
        fn from(val: Boostdiv) -> u8 {
            Boostdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Boostsel {
        #[doc = "No clock."]
        DISABLE = 0x0,
        #[doc = "MSIS selected as EPOD booster clock source."]
        MSIS = 0x01,
        #[doc = "HSI16 selected as EPOD booster clock source."]
        HSI16 = 0x02,
        #[doc = "HSE selected as EPOD booster clock source."]
        HSE = 0x03,
    }
    impl Boostsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Boostsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Boostsel {
        #[inline(always)]
        fn from(val: u8) -> Boostsel {
            Boostsel::from_bits(val)
        }
    }
    impl From<Boostsel> for u8 {
        #[inline(always)]
        fn from(val: Boostsel) -> u8 {
            Boostsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacshsel {
        #[doc = "LSE selected."]
        LSE = 0x0,
        #[doc = "LSI selected."]
        LSI = 0x01,
    }
    impl Dacshsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacshsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacshsel {
        #[inline(always)]
        fn from(val: u8) -> Dacshsel {
            Dacshsel::from_bits(val)
        }
    }
    impl From<Dacshsel> for u8 {
        #[inline(always)]
        fn from(val: Dacshsel) -> u8 {
            Dacshsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fdcansel {
        #[doc = "SYSCLK selected."]
        SYS = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl Fdcansel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fdcansel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fdcansel {
        #[inline(always)]
        fn from(val: u8) -> Fdcansel {
            Fdcansel::from_bits(val)
        }
    }
    impl From<Fdcansel> for u8 {
        #[inline(always)]
        fn from(val: Fdcansel) -> u8 {
            Fdcansel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "HCLK = SYSCLK not divided."]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "HCLK = SYSCLK divided by 2."]
        DIV2 = 0x08,
        #[doc = "HCLK = SYSCLK divided by 4."]
        DIV4 = 0x09,
        #[doc = "HCLK = SYSCLK divided by 8."]
        DIV8 = 0x0a,
        #[doc = "HCLK = SYSCLK divided by 16."]
        DIV16 = 0x0b,
        #[doc = "HCLK = SYSCLK divided by 64."]
        DIV64 = 0x0c,
        #[doc = "HCLK = SYSCLK divided by 128."]
        DIV128 = 0x0d,
        #[doc = "HCLK = SYSCLK divided by 256."]
        DIV256 = 0x0e,
        #[doc = "HCLK = SYSCLK divided by 512."]
        DIV512 = 0x0f,
    }
    impl Hpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpre {
        #[inline(always)]
        fn from(val: u8) -> Hpre {
            Hpre::from_bits(val)
        }
    }
    impl From<Hpre> for u8 {
        #[inline(always)]
        fn from(val: Hpre) -> u8 {
            Hpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hseext {
        #[doc = "External HSE clock analog mode."]
        ANALOG = 0x0,
        #[doc = "External HSE clock digital mode (through I/O Schmitt trigger)."]
        DIGITAL = 0x01,
    }
    impl Hseext {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hseext {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hseext {
        #[inline(always)]
        fn from(val: u8) -> Hseext {
            Hseext::from_bits(val)
        }
    }
    impl From<Hseext> for u8 {
        #[inline(always)]
        fn from(val: Hseext) -> u8 {
            Hseext::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c3sel {
        #[doc = "PCLK1 selected."]
        PCLK3 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl I2c3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c3sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c3sel {
        #[inline(always)]
        fn from(val: u8) -> I2c3sel {
            I2c3sel::from_bits(val)
        }
    }
    impl From<I2c3sel> for u8 {
        #[inline(always)]
        fn from(val: I2c3sel) -> u8 {
            I2c3sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2csel {
        #[doc = "PCLK1 selected."]
        PCLK1 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl I2csel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2csel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2csel {
        #[inline(always)]
        fn from(val: u8) -> I2csel {
            I2csel::from_bits(val)
        }
    }
    impl From<I2csel> for u8 {
        #[inline(always)]
        fn from(val: I2csel) -> u8 {
            I2csel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I3c2sel {
        #[doc = "PCLK2 selected."]
        PCLK2 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl I3c2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I3c2sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I3c2sel {
        #[inline(always)]
        fn from(val: u8) -> I3c2sel {
            I3c2sel::from_bits(val)
        }
    }
    impl From<I3c2sel> for u8 {
        #[inline(always)]
        fn from(val: I3c2sel) -> u8 {
            I3c2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I3csel {
        #[doc = "PCLK1 selected."]
        PCLK1 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl I3csel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I3csel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I3csel {
        #[inline(always)]
        fn from(val: u8) -> I3csel {
            I3csel::from_bits(val)
        }
    }
    impl From<I3csel> for u8 {
        #[inline(always)]
        fn from(val: I3csel) -> u8 {
            I3csel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Iclksel {
        #[doc = "HSI48 selected."]
        HSI48 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
        #[doc = "HSE selected."]
        HSE = 0x02,
        #[doc = "SYSCLK selected."]
        SYS = 0x03,
    }
    impl Iclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Iclksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Iclksel {
        #[inline(always)]
        fn from(val: u8) -> Iclksel {
            Iclksel::from_bits(val)
        }
    }
    impl From<Iclksel> for u8 {
        #[inline(always)]
        fn from(val: Iclksel) -> u8 {
            Iclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptimsel {
        #[doc = "MSIK clock selected."]
        MSIK = 0x0,
        #[doc = "LSI selected."]
        LSI = 0x01,
        #[doc = "HSI16 selected."]
        HSI = 0x02,
        #[doc = "LSE selected."]
        LSE = 0x03,
    }
    impl Lptimsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptimsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptimsel {
        #[inline(always)]
        fn from(val: u8) -> Lptimsel {
            Lptimsel::from_bits(val)
        }
    }
    impl From<Lptimsel> for u8 {
        #[inline(always)]
        fn from(val: Lptimsel) -> u8 {
            Lptimsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpuartsel {
        #[doc = "PCLK3 selected."]
        PCLK3 = 0x0,
        #[doc = "HSI16 selected."]
        HSI = 0x01,
        #[doc = "LSE selected."]
        LSE = 0x02,
        #[doc = "MSIK selected."]
        MSIK = 0x03,
    }
    impl Lpuartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpuartsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpuartsel {
        #[inline(always)]
        fn from(val: u8) -> Lpuartsel {
            Lpuartsel::from_bits(val)
        }
    }
    impl From<Lpuartsel> for u8 {
        #[inline(always)]
        fn from(val: Lpuartsel) -> u8 {
            Lpuartsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lscosel {
        #[doc = "LSI selected."]
        LSI = 0x0,
        #[doc = "LSE selected."]
        LSE = 0x01,
    }
    impl Lscosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lscosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lscosel {
        #[inline(always)]
        fn from(val: u8) -> Lscosel {
            Lscosel::from_bits(val)
        }
    }
    impl From<Lscosel> for u8 {
        #[inline(always)]
        fn from(val: Lscosel) -> u8 {
            Lscosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsedrv {
        #[doc = "Low driving capability"]
        LOW = 0x0,
        #[doc = "Medium-low driving capability"]
        MEDIUM_LOW = 0x01,
        #[doc = "Medium-high driving capability"]
        MEDIUM_HIGH = 0x02,
        #[doc = "High driving capability"]
        HIGH = 0x03,
    }
    impl Lsedrv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsedrv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsedrv {
        #[inline(always)]
        fn from(val: u8) -> Lsedrv {
            Lsedrv::from_bits(val)
        }
    }
    impl From<Lsedrv> for u8 {
        #[inline(always)]
        fn from(val: Lsedrv) -> u8 {
            Lsedrv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsiprediv {
        #[doc = "LSI not divided."]
        DIV1 = 0x0,
        #[doc = "LSI divided by 128."]
        DIV128 = 0x01,
    }
    impl Lsiprediv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsiprediv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsiprediv {
        #[inline(always)]
        fn from(val: u8) -> Lsiprediv {
            Lsiprediv::from_bits(val)
        }
    }
    impl From<Lsiprediv> for u8 {
        #[inline(always)]
        fn from(val: Lsiprediv) -> u8 {
            Lsiprediv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mco2sel {
        #[doc = "MCO2 output disabled, no clock on MCO2."]
        DISABLE = 0x0,
        #[doc = "SYSCLK system clock selected."]
        SYS = 0x01,
        #[doc = "MSIS clock selected."]
        MSIS = 0x02,
        #[doc = "HSI16 clock selected."]
        HSI16 = 0x03,
        #[doc = "HSE clock selected 0101: LSI clock selected."]
        HSE = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "LSE clock selected."]
        LSE = 0x06,
        #[doc = "HSI48 clock selected."]
        HSI48 = 0x07,
        #[doc = "MSIK clock selected."]
        MSIK = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Mco2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco2sel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco2sel {
        #[inline(always)]
        fn from(val: u8) -> Mco2sel {
            Mco2sel::from_bits(val)
        }
    }
    impl From<Mco2sel> for u8 {
        #[inline(always)]
        fn from(val: Mco2sel) -> u8 {
            Mco2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcopre {
        #[doc = "MCO2 divided by 1."]
        DIV1 = 0x0,
        #[doc = "MCO2 divided by 2."]
        DIV2 = 0x01,
        #[doc = "MCO2 divided by 4."]
        DIV4 = 0x02,
        #[doc = "MCO2 divided by 8."]
        DIV8 = 0x03,
        #[doc = "MCO2 divided by 16."]
        DIV16 = 0x04,
        #[doc = "MCO2 divided by 32."]
        DIV32 = 0x05,
        #[doc = "MCO2 divided by 64."]
        DIV64 = 0x06,
        #[doc = "MCO2 divided by 128."]
        DIV128 = 0x07,
    }
    impl Mcopre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcopre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcopre {
        #[inline(always)]
        fn from(val: u8) -> Mcopre {
            Mcopre::from_bits(val)
        }
    }
    impl From<Mcopre> for u8 {
        #[inline(always)]
        fn from(val: Mcopre) -> u8 {
            Mcopre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcosel {
        #[doc = "MCO output disabled, no clock on MCO."]
        DISABLE = 0x0,
        #[doc = "SYSCLK system clock selected."]
        SYS = 0x01,
        #[doc = "MSIS clock selected."]
        MSIS = 0x02,
        #[doc = "HSI16 clock selected."]
        HSI16 = 0x03,
        #[doc = "HSE clock selected."]
        HSE = 0x04,
        #[doc = "LSI clock selected."]
        LSI = 0x05,
        #[doc = "LSE clock selected."]
        LSE = 0x06,
        #[doc = "HSI48 clock selected."]
        HSI48 = 0x07,
        #[doc = "MSIK clock selected."]
        MSIK = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Mcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcosel {
            unsafe { core::mem::transmute(val & 0x0f) }
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
    pub enum Msibias {
        #[doc = "MSI bias continuous mode (clock accuracy fast settling time)."]
        CONTINUOUS = 0x0,
        #[doc = "MSI bias sampling mode when the device is in Stop 1 or Stop 2 mode, or when the regulator is in range 2."]
        SAMPLING = 0x01,
    }
    impl Msibias {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msibias {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msibias {
        #[inline(always)]
        fn from(val: u8) -> Msibias {
            Msibias::from_bits(val)
        }
    }
    impl From<Msibias> for u8 {
        #[inline(always)]
        fn from(val: Msibias) -> u8 {
            Msibias::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msihsindiv {
        #[doc = "HSE (16 MHz) is used as MSI input clock when PLL mode with high-speed clock is selected."]
        DIV1 = 0x0,
        #[doc = "HSE (32 MHz)/2 is used as MSI input clock when PLL mode with high-speed clock is selected."]
        DIV2 = 0x01,
    }
    impl Msihsindiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msihsindiv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msihsindiv {
        #[inline(always)]
        fn from(val: u8) -> Msihsindiv {
            Msihsindiv::from_bits(val)
        }
    }
    impl From<Msihsindiv> for u8 {
        #[inline(always)]
        fn from(val: Msihsindiv) -> u8 {
            Msihsindiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msikdiv {
        #[doc = "MSIRC0/1 is selected for MSIK (range 0 around 96 MHz)."]
        DIV1 = 0x0,
        #[doc = "MSIRC0/2 is selected for MSIK (range 1 around 48 MHz)."]
        DIV2 = 0x01,
        #[doc = "MSIRC0/4 is selected for MSIK (range 2 around 24 MHz)."]
        DIV4 = 0x02,
        #[doc = "MSIRC0/8 is selected for MSIK (range 3 around 12 MHz)."]
        DIV8 = 0x03,
    }
    impl Msikdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msikdiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msikdiv {
        #[inline(always)]
        fn from(val: u8) -> Msikdiv {
            Msikdiv::from_bits(val)
        }
    }
    impl From<Msikdiv> for u8 {
        #[inline(always)]
        fn from(val: Msikdiv) -> u8 {
            Msikdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msikdivs {
        _RESERVED_0 = 0x0,
        #[doc = "Range 5 around 12 MHz (reset value)."]
        RANGE5_12MHZ = 0x01,
        #[doc = "Range 6 around 6 MHz."]
        RANGE6_6MHZ = 0x02,
        #[doc = "Range 7 around 3 MHz."]
        RANGE7_3MHZ = 0x03,
    }
    impl Msikdivs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msikdivs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msikdivs {
        #[inline(always)]
        fn from(val: u8) -> Msikdivs {
            Msikdivs::from_bits(val)
        }
    }
    impl From<Msikdivs> for u8 {
        #[inline(always)]
        fn from(val: Msikdivs) -> u8 {
            Msikdivs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msiksel {
        #[doc = "MSIRC0 (96 MHz) is selected as source to generate MSIK."]
        MSIRC0_96MHZ = 0x0,
        #[doc = "MSIRC1 (24 MHz) is selected as source to generate MSIK."]
        MSIRC1_24MHZ = 0x01,
    }
    impl Msiksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msiksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msiksel {
        #[inline(always)]
        fn from(val: u8) -> Msiksel {
            Msiksel::from_bits(val)
        }
    }
    impl From<Msiksel> for u8 {
        #[inline(always)]
        fn from(val: Msiksel) -> u8 {
            Msiksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msipllsel {
        #[doc = "LSE is used as MSIRC0 input clock when PLL mode is selected."]
        LSE = 0x0,
        #[doc = "HSE or HSE/2 is used as MSIRC0 input clock when PLL mode is selected."]
        HSE = 0x01,
    }
    impl Msipllsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msipllsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msipllsel {
        #[inline(always)]
        fn from(val: u8) -> Msipllsel {
            Msipllsel::from_bits(val)
        }
    }
    impl From<Msipllsel> for u8 {
        #[inline(always)]
        fn from(val: Msipllsel) -> u8 {
            Msipllsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msirgsel {
        #[doc = "MSIS/MSIK ranges provided by MSISDIVS\\[1:0\\]
and MSIKDIVS\\[1:0\\]
in RCC_CSR."]
        RCC_CSR = 0x0,
        #[doc = "MSIS/MSIK ranges provided by MSISDIV\\[1:0\\]
and MSIKDIV\\[1:0\\]
in RCC_ICSCR1."]
        RCC_ICSCR1 = 0x01,
    }
    impl Msirgsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msirgsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msirgsel {
        #[inline(always)]
        fn from(val: u8) -> Msirgsel {
            Msirgsel::from_bits(val)
        }
    }
    impl From<Msirgsel> for u8 {
        #[inline(always)]
        fn from(val: Msirgsel) -> u8 {
            Msirgsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msisdiv {
        #[doc = "MSIRC0/1 is selected for MSIS (range 0 around 96 MHz)."]
        DIV1 = 0x0,
        #[doc = "MSIRC0/2 is selected for MSIS (range 1 around 48 MHz)."]
        DIV2 = 0x01,
        #[doc = "MSIRC0/4 is selected for MSIS (range 2 around 24 MHz)."]
        DIV4 = 0x02,
        #[doc = "MSIRC0/8 is selected for MSIS (range 3 around 12 MHz)."]
        DIV8 = 0x03,
    }
    impl Msisdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msisdiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msisdiv {
        #[inline(always)]
        fn from(val: u8) -> Msisdiv {
            Msisdiv::from_bits(val)
        }
    }
    impl From<Msisdiv> for u8 {
        #[inline(always)]
        fn from(val: Msisdiv) -> u8 {
            Msisdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msisdivs {
        _RESERVED_0 = 0x0,
        #[doc = "Range 5 around 12 MHz (reset value)."]
        RANGE5_12MHZ = 0x01,
        #[doc = "Range 6 around 6 MHz."]
        RANGE6_6MHZ = 0x02,
        #[doc = "Range 7 around 3 MHz."]
        RANGE7_3MHZ = 0x03,
    }
    impl Msisdivs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msisdivs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msisdivs {
        #[inline(always)]
        fn from(val: u8) -> Msisdivs {
            Msisdivs::from_bits(val)
        }
    }
    impl From<Msisdivs> for u8 {
        #[inline(always)]
        fn from(val: Msisdivs) -> u8 {
            Msisdivs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msissel {
        #[doc = "MSIRC0 (96 MHz) is selected as source to generate MSIS."]
        MSIRC0_96MHZ = 0x0,
        #[doc = "MSIRC1 (24 MHz) is selected as source to generate MSIS."]
        MSIRC1_24MHZ = 0x01,
    }
    impl Msissel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msissel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msissel {
        #[inline(always)]
        fn from(val: u8) -> Msissel {
            Msissel::from_bits(val)
        }
    }
    impl From<Msissel> for u8 {
        #[inline(always)]
        fn from(val: Msissel) -> u8 {
            Msissel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Octospisel {
        #[doc = "SYSCLK selected."]
        SYS = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl Octospisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Octospisel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Octospisel {
        #[inline(always)]
        fn from(val: u8) -> Octospisel {
            Octospisel::from_bits(val)
        }
    }
    impl From<Octospisel> for u8 {
        #[inline(always)]
        fn from(val: Octospisel) -> u8 {
            Octospisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre {
        #[doc = "PCLK1 = HCLK not divided."]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "PCLK1 = HCLK divided by 2."]
        DIV2 = 0x04,
        #[doc = "PCLK1 = HCLK divided by 4."]
        DIV4 = 0x05,
        #[doc = "PCLK1 = HCLK divided by 8."]
        DIV8 = 0x06,
        #[doc = "PCLK1 = HCLK divided by 16."]
        DIV16 = 0x07,
    }
    impl Ppre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre {
        #[inline(always)]
        fn from(val: u8) -> Ppre {
            Ppre::from_bits(val)
        }
    }
    impl From<Ppre> for u8 {
        #[inline(always)]
        fn from(val: Ppre) -> u8 {
            Ppre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rngsel {
        #[doc = "HSI48 selected."]
        HSI48 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl Rngsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rngsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rngsel {
        #[inline(always)]
        fn from(val: u8) -> Rngsel {
            Rngsel::from_bits(val)
        }
    }
    impl From<Rngsel> for u8 {
        #[inline(always)]
        fn from(val: Rngsel) -> u8 {
            Rngsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcsel {
        #[doc = "No clock selected."]
        DISABLE = 0x0,
        #[doc = "LSE selected."]
        LSE = 0x01,
        #[doc = "LSI selected."]
        LSI = 0x02,
        #[doc = "HSE/32 selected."]
        HSE = 0x03,
    }
    impl Rtcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtcsel {
        #[inline(always)]
        fn from(val: u8) -> Rtcsel {
            Rtcsel::from_bits(val)
        }
    }
    impl From<Rtcsel> for u8 {
        #[inline(always)]
        fn from(val: Rtcsel) -> u8 {
            Rtcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Saisel {
        #[doc = "MSIK selected."]
        MSIK = 0x0,
        #[doc = "input pin AUDIOCLK selected."]
        AUDIOCLK = 0x01,
        #[doc = "HSE clock selected."]
        HSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Saisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saisel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Saisel {
        #[inline(always)]
        fn from(val: u8) -> Saisel {
            Saisel::from_bits(val)
        }
    }
    impl From<Saisel> for u8 {
        #[inline(always)]
        fn from(val: Saisel) -> u8 {
            Saisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Security {
        #[doc = "Nonsecure."]
        NON_SECURE = 0x0,
        #[doc = "Secure."]
        SECURE = 0x01,
    }
    impl Security {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Security {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Security {
        #[inline(always)]
        fn from(val: u8) -> Security {
            Security::from_bits(val)
        }
    }
    impl From<Security> for u8 {
        #[inline(always)]
        fn from(val: Security) -> u8 {
            Security::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi2sel {
        #[doc = "PCLK1 selected."]
        PCLK1 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl Spi2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi2sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi2sel {
        #[inline(always)]
        fn from(val: u8) -> Spi2sel {
            Spi2sel::from_bits(val)
        }
    }
    impl From<Spi2sel> for u8 {
        #[inline(always)]
        fn from(val: Spi2sel) -> u8 {
            Spi2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi3sel {
        #[doc = "PCLK1 selected."]
        PCLK1 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl Spi3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi3sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi3sel {
        #[inline(always)]
        fn from(val: u8) -> Spi3sel {
            Spi3sel::from_bits(val)
        }
    }
    impl From<Spi3sel> for u8 {
        #[inline(always)]
        fn from(val: Spi3sel) -> u8 {
            Spi3sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spisel {
        #[doc = "PCLK2 selected."]
        PCLK2 = 0x0,
        #[doc = "MSIK selected."]
        MSIK = 0x01,
    }
    impl Spisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spisel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spisel {
        #[inline(always)]
        fn from(val: u8) -> Spisel {
            Spisel::from_bits(val)
        }
    }
    impl From<Spisel> for u8 {
        #[inline(always)]
        fn from(val: Spisel) -> u8 {
            Spisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopwuck {
        #[doc = "MSIS oscillator selected as wake-up from stop clock and CSS backup clock."]
        MSIS = 0x0,
        #[doc = "HSI16 oscillator selected as wake-up from stop clock and CSS backup clock."]
        HSI16 = 0x01,
    }
    impl Stopwuck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopwuck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopwuck {
        #[inline(always)]
        fn from(val: u8) -> Stopwuck {
            Stopwuck::from_bits(val)
        }
    }
    impl From<Stopwuck> for u8 {
        #[inline(always)]
        fn from(val: Stopwuck) -> u8 {
            Stopwuck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sw {
        #[doc = "MSIS selected as system clock."]
        MSIS = 0x0,
        #[doc = "HSI16 selected as system clock."]
        HSI16 = 0x01,
        #[doc = "HSE selected as system clock."]
        HSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sw {
        #[inline(always)]
        fn from(val: u8) -> Sw {
            Sw::from_bits(val)
        }
    }
    impl From<Sw> for u8 {
        #[inline(always)]
        fn from(val: Sw) -> u8 {
            Sw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sws {
        #[doc = "MSIS oscillator used as system clock."]
        MSIS = 0x0,
        #[doc = "HSI16 oscillator used as system clock."]
        HSI16 = 0x01,
        #[doc = "HSE used as system clock."]
        HSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Sws {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sws {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sws {
        #[inline(always)]
        fn from(val: u8) -> Sws {
            Sws::from_bits(val)
        }
    }
    impl From<Sws> for u8 {
        #[inline(always)]
        fn from(val: Sws) -> u8 {
            Sws::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Systicksel {
        #[doc = "HCLK/8 selected."]
        HCLK1_DIV_8 = 0x0,
        #[doc = "LSI selected."]
        LSI = 0x01,
        #[doc = "LSE selected."]
        LSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Systicksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Systicksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Systicksel {
        #[inline(always)]
        fn from(val: u8) -> Systicksel {
            Systicksel::from_bits(val)
        }
    }
    impl From<Systicksel> for u8 {
        #[inline(always)]
        fn from(val: Systicksel) -> u8 {
            Systicksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Timicsel {
        #[doc = "HSI, MSIK and MSIS dividers disabled."]
        DISABLE = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HSI/256, MSIS/1024 and MSIS/4 are generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture."]
        HSI256_MSIS1024_MSIS4 = 0x04,
        #[doc = "HSI/256, MSIS/1024 and MSIK/4 are generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture."]
        HSI256_MSIS1024_MSIK4 = 0x05,
        #[doc = "HSI/256, MSIK/1024 and MSIS/4 are generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture."]
        HSI256_MSIK1024_MSIS4 = 0x06,
        #[doc = "HSI/256, MSIK/1024 and MSIK/4 are generated and can be selected by TIM16, TIM17, and LPTIM2 as internal input capture."]
        HSI256_MSIK1024_MSIK4 = 0x07,
    }
    impl Timicsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timicsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timicsel {
        #[inline(always)]
        fn from(val: u8) -> Timicsel {
            Timicsel::from_bits(val)
        }
    }
    impl From<Timicsel> for u8 {
        #[inline(always)]
        fn from(val: Timicsel) -> u8 {
            Timicsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Uartsel {
        #[doc = "PCLK1 selected."]
        PCLK1 = 0x0,
        #[doc = "HSI16 selected."]
        HSI = 0x01,
    }
    impl Uartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Uartsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Uartsel {
        #[inline(always)]
        fn from(val: u8) -> Uartsel {
            Uartsel::from_bits(val)
        }
    }
    impl From<Uartsel> for u8 {
        #[inline(always)]
        fn from(val: Uartsel) -> u8 {
            Uartsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart3sel {
        #[doc = "PCLK1 selected."]
        PCLK1 = 0x0,
        #[doc = "HSI16 selected."]
        HSI = 0x01,
    }
    impl Usart3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart3sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart3sel {
        #[inline(always)]
        fn from(val: u8) -> Usart3sel {
            Usart3sel::from_bits(val)
        }
    }
    impl From<Usart3sel> for u8 {
        #[inline(always)]
        fn from(val: Usart3sel) -> u8 {
            Usart3sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usartsel {
        #[doc = "PCLK2 selected."]
        PCLK2 = 0x0,
        #[doc = "HSI16 selected."]
        HSI = 0x01,
    }
    impl Usartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usartsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usartsel {
        #[inline(always)]
        fn from(val: u8) -> Usartsel {
            Usartsel::from_bits(val)
        }
    }
    impl From<Usartsel> for u8 {
        #[inline(always)]
        fn from(val: Usartsel) -> u8 {
            Usartsel::to_bits(val)
        }
    }
}
