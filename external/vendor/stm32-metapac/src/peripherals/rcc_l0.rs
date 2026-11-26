#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Reset and clock control"]
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
    #[doc = "Clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Internal clock sources calibration register"]
    #[inline(always)]
    pub const fn icscr(self) -> crate::common::Reg<regs::Icscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "GPIO reset register"]
    #[inline(always)]
    pub const fn gpiorstr(self) -> crate::common::Reg<regs::Gpiorstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "AHB peripheral reset register"]
    #[inline(always)]
    pub const fn ahbrstr(self) -> crate::common::Reg<regs::Ahbrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rstr(self) -> crate::common::Reg<regs::Apb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "GPIO clock enable register"]
    #[inline(always)]
    pub const fn gpioenr(self) -> crate::common::Reg<regs::Gpioenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "AHB peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahbenr(self) -> crate::common::Reg<regs::Ahbenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1enr(self) -> crate::common::Reg<regs::Apb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "GPIO clock enable in sleep mode register"]
    #[inline(always)]
    pub const fn gpiosmen(self) -> crate::common::Reg<regs::Gpiosmen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "AHB peripheral clock enable in sleep mode register"]
    #[inline(always)]
    pub const fn ahbsmenr(self) -> crate::common::Reg<regs::Ahbsmenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "APB2 peripheral clock enable in sleep mode register"]
    #[inline(always)]
    pub const fn apb2smenr(self) -> crate::common::Reg<regs::Apb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable in sleep mode register"]
    #[inline(always)]
    pub const fn apb1smenr(self) -> crate::common::Reg<regs::Apb1smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Clock configuration register"]
    #[inline(always)]
    pub const fn ccipr(self) -> crate::common::Reg<regs::Ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbenr(pub u32);
    impl Ahbenr {
        #[doc = "DMA clock enable"]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA clock enable"]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "NVM interface clock enable"]
        #[inline(always)]
        pub const fn mifen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "NVM interface clock enable"]
        #[inline(always)]
        pub fn set_mifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing clock enable"]
        #[inline(always)]
        pub const fn tscen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing clock enable"]
        #[inline(always)]
        pub fn set_tscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Random Number Generator clock enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator clock enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Crypto clock enable"]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Crypto clock enable"]
        #[inline(always)]
        pub fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("dma1en", &self.dma1en())
                .field("mifen", &self.mifen())
                .field("crcen", &self.crcen())
                .field("tscen", &self.tscen())
                .field("rngen", &self.rngen())
                .field("crypen", &self.crypen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahbenr {{ dma1en: {=bool:?}, mifen: {=bool:?}, crcen: {=bool:?}, tscen: {=bool:?}, rngen: {=bool:?}, crypen: {=bool:?} }}" , self . dma1en () , self . mifen () , self . crcen () , self . tscen () , self . rngen () , self . crypen ())
        }
    }
    #[doc = "AHB peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbrstr(pub u32);
    impl Ahbrstr {
        #[doc = "DMA reset"]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA reset"]
        #[inline(always)]
        pub fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Memory interface reset"]
        #[inline(always)]
        pub const fn mifrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Memory interface reset"]
        #[inline(always)]
        pub fn set_mifrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Test integration module reset"]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Test integration module reset"]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing reset"]
        #[inline(always)]
        pub const fn tscrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing reset"]
        #[inline(always)]
        pub fn set_tscrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Random Number Generator module reset"]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator module reset"]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Crypto module reset"]
        #[inline(always)]
        pub const fn cryprst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Crypto module reset"]
        #[inline(always)]
        pub fn set_cryprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("dma1rst", &self.dma1rst())
                .field("mifrst", &self.mifrst())
                .field("crcrst", &self.crcrst())
                .field("tscrst", &self.tscrst())
                .field("rngrst", &self.rngrst())
                .field("cryprst", &self.cryprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahbrstr {{ dma1rst: {=bool:?}, mifrst: {=bool:?}, crcrst: {=bool:?}, tscrst: {=bool:?}, rngrst: {=bool:?}, cryprst: {=bool:?} }}" , self . dma1rst () , self . mifrst () , self . crcrst () , self . tscrst () , self . rngrst () , self . cryprst ())
        }
    }
    #[doc = "AHB peripheral clock enable in sleep mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbsmenr(pub u32);
    impl Ahbsmenr {
        #[doc = "DMA clock enable during sleep mode"]
        #[inline(always)]
        pub const fn dma1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_dma1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "NVM interface clock enable during sleep mode"]
        #[inline(always)]
        pub const fn mifsmen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "NVM interface clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_mifsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM interface clock enable during sleep mode"]
        #[inline(always)]
        pub const fn sramsmen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM interface clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_sramsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CRC clock enable during sleep mode"]
        #[inline(always)]
        pub const fn crcsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_crcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tscsmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tscsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Random Number Generator clock enable during sleep mode"]
        #[inline(always)]
        pub const fn rngsmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_rngsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Crypto clock enable during sleep mode"]
        #[inline(always)]
        pub const fn crypsmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Crypto clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_crypsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ahbsmenr {
        #[inline(always)]
        fn default() -> Ahbsmenr {
            Ahbsmenr(0)
        }
    }
    impl core::fmt::Debug for Ahbsmenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbsmenr")
                .field("dma1smen", &self.dma1smen())
                .field("mifsmen", &self.mifsmen())
                .field("sramsmen", &self.sramsmen())
                .field("crcsmen", &self.crcsmen())
                .field("tscsmen", &self.tscsmen())
                .field("rngsmen", &self.rngsmen())
                .field("crypsmen", &self.crypsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbsmenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahbsmenr {{ dma1smen: {=bool:?}, mifsmen: {=bool:?}, sramsmen: {=bool:?}, crcsmen: {=bool:?}, tscsmen: {=bool:?}, rngsmen: {=bool:?}, crypsmen: {=bool:?} }}" , self . dma1smen () , self . mifsmen () , self . sramsmen () , self . crcsmen () , self . tscsmen () , self . rngsmen () , self . crypsmen ())
        }
    }
    #[doc = "APB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr(pub u32);
    impl Apb1enr {
        #[doc = "Timer2 clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer2 clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 clock enbale"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 clock enbale"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 6 clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Window watchdog clock enable"]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable"]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "UART2 clock enable"]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "UART2 clock enable"]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "LPUART1 clock enable"]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable"]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART4 clock enable"]
        #[inline(always)]
        pub const fn usart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 clock enable"]
        #[inline(always)]
        pub fn set_usart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART5 clock enable"]
        #[inline(always)]
        pub const fn usart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USART5 clock enable"]
        #[inline(always)]
        pub fn set_usart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB clock enable"]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable"]
        #[inline(always)]
        pub fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Clock recovery system clock enable"]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Clock recovery system clock enable"]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power interface clock enable"]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable"]
        #[inline(always)]
        pub fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface clock enable"]
        #[inline(always)]
        pub const fn dacen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable"]
        #[inline(always)]
        pub fn set_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low power timer clock enable"]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer clock enable"]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("usart2en", &self.usart2en())
                .field("lpuart1en", &self.lpuart1en())
                .field("usart4en", &self.usart4en())
                .field("usart5en", &self.usart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("usben", &self.usben())
                .field("crsen", &self.crsen())
                .field("pwren", &self.pwren())
                .field("dacen", &self.dacen())
                .field("i2c3en", &self.i2c3en())
                .field("lptim1en", &self.lptim1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1enr {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, wwdgen: {=bool:?}, spi2en: {=bool:?}, usart2en: {=bool:?}, lpuart1en: {=bool:?}, usart4en: {=bool:?}, usart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, usben: {=bool:?}, crsen: {=bool:?}, pwren: {=bool:?}, dacen: {=bool:?}, i2c3en: {=bool:?}, lptim1en: {=bool:?} }}" , self . tim2en () , self . tim3en () , self . tim6en () , self . tim7en () , self . wwdgen () , self . spi2en () , self . usart2en () , self . lpuart1en () , self . usart4en () , self . usart5en () , self . i2c1en () , self . i2c2en () , self . usben () , self . crsen () , self . pwren () , self . dacen () , self . i2c3en () , self . lptim1en ())
        }
    }
    #[doc = "APB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr(pub u32);
    impl Apb1rstr {
        #[doc = "Timer 2 reset"]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 reset"]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 reset"]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 reset"]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 6 reset"]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 reset"]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 reset"]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 reset"]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Window watchdog reset"]
        #[inline(always)]
        pub const fn wwdgrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset"]
        #[inline(always)]
        pub fn set_wwdgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 reset"]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 reset"]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "LPUART1 reset"]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 reset"]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART4 reset"]
        #[inline(always)]
        pub const fn usart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 reset"]
        #[inline(always)]
        pub fn set_usart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART5 reset"]
        #[inline(always)]
        pub const fn usart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USART5 reset"]
        #[inline(always)]
        pub fn set_usart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset"]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset"]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB reset"]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset"]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Clock recovery system reset"]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Clock recovery system reset"]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power interface reset"]
        #[inline(always)]
        pub const fn pwrrst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface reset"]
        #[inline(always)]
        pub fn set_pwrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface reset"]
        #[inline(always)]
        pub const fn dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface reset"]
        #[inline(always)]
        pub fn set_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low power timer reset"]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer reset"]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("wwdgrst", &self.wwdgrst())
                .field("spi2rst", &self.spi2rst())
                .field("usart2rst", &self.usart2rst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("usart4rst", &self.usart4rst())
                .field("usart5rst", &self.usart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("usbrst", &self.usbrst())
                .field("crsrst", &self.crsrst())
                .field("pwrrst", &self.pwrrst())
                .field("dacrst", &self.dacrst())
                .field("i2c3rst", &self.i2c3rst())
                .field("lptim1rst", &self.lptim1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1rstr {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, wwdgrst: {=bool:?}, spi2rst: {=bool:?}, usart2rst: {=bool:?}, lpuart1rst: {=bool:?}, usart4rst: {=bool:?}, usart5rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, usbrst: {=bool:?}, crsrst: {=bool:?}, pwrrst: {=bool:?}, dacrst: {=bool:?}, i2c3rst: {=bool:?}, lptim1rst: {=bool:?} }}" , self . tim2rst () , self . tim3rst () , self . tim6rst () , self . tim7rst () , self . wwdgrst () , self . spi2rst () , self . usart2rst () , self . lpuart1rst () , self . usart4rst () , self . usart5rst () , self . i2c1rst () , self . i2c2rst () , self . usbrst () , self . crsrst () , self . pwrrst () , self . dacrst () , self . i2c3rst () , self . lptim1rst ())
        }
    }
    #[doc = "APB1 peripheral clock enable in sleep mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr(pub u32);
    impl Apb1smenr {
        #[doc = "Timer2 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tim2smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer2 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tim3smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 6 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tim6smen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tim6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tim7smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tim7smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Window watchdog clock enable during sleep mode"]
        #[inline(always)]
        pub const fn wwdgsmen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_wwdgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn spi2smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_spi2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "UART2 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn usart2smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "UART2 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_usart2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "LPUART1 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn lpuart1smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_lpuart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART4 clock enabe during sleep mode"]
        #[inline(always)]
        pub const fn usart4smen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 clock enabe during sleep mode"]
        #[inline(always)]
        pub fn set_usart4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART5 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn usart5smen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USART5 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_usart5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn i2c1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_i2c1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn i2c2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_i2c2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB clock enable during sleep mode"]
        #[inline(always)]
        pub const fn usbsmen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_usbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Clock recovery system clock enable during sleep mode"]
        #[inline(always)]
        pub const fn crssmen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Clock recovery system clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_crssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power interface clock enable during sleep mode"]
        #[inline(always)]
        pub const fn pwrsmen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_pwrsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface clock enable during sleep mode"]
        #[inline(always)]
        pub const fn dacsmen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_dacsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "I2C3 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn i2c3smen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_i2c3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low power timer clock enable during sleep mode"]
        #[inline(always)]
        pub const fn lptim1smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_lptim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1smenr {
        #[inline(always)]
        fn default() -> Apb1smenr {
            Apb1smenr(0)
        }
    }
    impl core::fmt::Debug for Apb1smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1smenr")
                .field("tim2smen", &self.tim2smen())
                .field("tim3smen", &self.tim3smen())
                .field("tim6smen", &self.tim6smen())
                .field("tim7smen", &self.tim7smen())
                .field("wwdgsmen", &self.wwdgsmen())
                .field("spi2smen", &self.spi2smen())
                .field("usart2smen", &self.usart2smen())
                .field("lpuart1smen", &self.lpuart1smen())
                .field("usart4smen", &self.usart4smen())
                .field("usart5smen", &self.usart5smen())
                .field("i2c1smen", &self.i2c1smen())
                .field("i2c2smen", &self.i2c2smen())
                .field("usbsmen", &self.usbsmen())
                .field("crssmen", &self.crssmen())
                .field("pwrsmen", &self.pwrsmen())
                .field("dacsmen", &self.dacsmen())
                .field("i2c3smen", &self.i2c3smen())
                .field("lptim1smen", &self.lptim1smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1smenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1smenr {{ tim2smen: {=bool:?}, tim3smen: {=bool:?}, tim6smen: {=bool:?}, tim7smen: {=bool:?}, wwdgsmen: {=bool:?}, spi2smen: {=bool:?}, usart2smen: {=bool:?}, lpuart1smen: {=bool:?}, usart4smen: {=bool:?}, usart5smen: {=bool:?}, i2c1smen: {=bool:?}, i2c2smen: {=bool:?}, usbsmen: {=bool:?}, crssmen: {=bool:?}, pwrsmen: {=bool:?}, dacsmen: {=bool:?}, i2c3smen: {=bool:?}, lptim1smen: {=bool:?} }}" , self . tim2smen () , self . tim3smen () , self . tim6smen () , self . tim7smen () , self . wwdgsmen () , self . spi2smen () , self . usart2smen () , self . lpuart1smen () , self . usart4smen () , self . usart5smen () , self . i2c1smen () , self . i2c2smen () , self . usbsmen () , self . crssmen () , self . pwrsmen () , self . dacsmen () , self . i2c3smen () , self . lptim1smen ())
        }
    }
    #[doc = "APB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "System configuration controller clock enable"]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller clock enable"]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM21 timer clock enable"]
        #[inline(always)]
        pub const fn tim21en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM21 timer clock enable"]
        #[inline(always)]
        pub fn set_tim21en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM22 timer clock enable"]
        #[inline(always)]
        pub const fn tim22en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM22 timer clock enable"]
        #[inline(always)]
        pub fn set_tim22en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Firewall clock enable"]
        #[inline(always)]
        pub const fn fwen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Firewall clock enable"]
        #[inline(always)]
        pub fn set_fwen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC clock enable"]
        #[inline(always)]
        pub const fn adcen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clock enable"]
        #[inline(always)]
        pub fn set_adcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DBG clock enable"]
        #[inline(always)]
        pub const fn dbgen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DBG clock enable"]
        #[inline(always)]
        pub fn set_dbgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("syscfgen", &self.syscfgen())
                .field("tim21en", &self.tim21en())
                .field("tim22en", &self.tim22en())
                .field("fwen", &self.fwen())
                .field("adcen", &self.adcen())
                .field("spi1en", &self.spi1en())
                .field("usart1en", &self.usart1en())
                .field("dbgen", &self.dbgen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2enr {{ syscfgen: {=bool:?}, tim21en: {=bool:?}, tim22en: {=bool:?}, fwen: {=bool:?}, adcen: {=bool:?}, spi1en: {=bool:?}, usart1en: {=bool:?}, dbgen: {=bool:?} }}" , self . syscfgen () , self . tim21en () , self . tim22en () , self . fwen () , self . adcen () , self . spi1en () , self . usart1en () , self . dbgen ())
        }
    }
    #[doc = "APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "System configuration controller reset"]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller reset"]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM21 timer reset"]
        #[inline(always)]
        pub const fn tim21rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM21 timer reset"]
        #[inline(always)]
        pub fn set_tim21rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM22 timer reset"]
        #[inline(always)]
        pub const fn tim22rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM22 timer reset"]
        #[inline(always)]
        pub fn set_tim22rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC interface reset"]
        #[inline(always)]
        pub const fn adcrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC interface reset"]
        #[inline(always)]
        pub fn set_adcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SPI 1 reset"]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 reset"]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DBG reset"]
        #[inline(always)]
        pub const fn dbgrst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DBG reset"]
        #[inline(always)]
        pub fn set_dbgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("syscfgrst", &self.syscfgrst())
                .field("tim21rst", &self.tim21rst())
                .field("tim22rst", &self.tim22rst())
                .field("adcrst", &self.adcrst())
                .field("spi1rst", &self.spi1rst())
                .field("usart1rst", &self.usart1rst())
                .field("dbgrst", &self.dbgrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2rstr {{ syscfgrst: {=bool:?}, tim21rst: {=bool:?}, tim22rst: {=bool:?}, adcrst: {=bool:?}, spi1rst: {=bool:?}, usart1rst: {=bool:?}, dbgrst: {=bool:?} }}" , self . syscfgrst () , self . tim21rst () , self . tim22rst () , self . adcrst () , self . spi1rst () , self . usart1rst () , self . dbgrst ())
        }
    }
    #[doc = "APB2 peripheral clock enable in sleep mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2smenr(pub u32);
    impl Apb2smenr {
        #[doc = "System configuration controller clock enable during sleep mode"]
        #[inline(always)]
        pub const fn syscfgsmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_syscfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM21 timer clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tim21smen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM21 timer clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tim21smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM22 timer clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tim22smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM22 timer clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tim22smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC clock enable during sleep mode"]
        #[inline(always)]
        pub const fn adcsmen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_adcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SPI1 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn spi1smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_spi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn usart1smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_usart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DBG clock enable during sleep mode"]
        #[inline(always)]
        pub const fn dbgsmen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DBG clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_dbgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Apb2smenr {
        #[inline(always)]
        fn default() -> Apb2smenr {
            Apb2smenr(0)
        }
    }
    impl core::fmt::Debug for Apb2smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2smenr")
                .field("syscfgsmen", &self.syscfgsmen())
                .field("tim21smen", &self.tim21smen())
                .field("tim22smen", &self.tim22smen())
                .field("adcsmen", &self.adcsmen())
                .field("spi1smen", &self.spi1smen())
                .field("usart1smen", &self.usart1smen())
                .field("dbgsmen", &self.dbgsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2smenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2smenr {{ syscfgsmen: {=bool:?}, tim21smen: {=bool:?}, tim22smen: {=bool:?}, adcsmen: {=bool:?}, spi1smen: {=bool:?}, usart1smen: {=bool:?}, dbgsmen: {=bool:?} }}" , self . syscfgsmen () , self . tim21smen () , self . tim22smen () , self . adcsmen () , self . spi1smen () , self . usart1smen () , self . dbgsmen ())
        }
    }
    #[doc = "Clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr(pub u32);
    impl Ccipr {
        #[doc = "USART1 clock source selection"]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usart1sel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usart1sel::from_bits(val as u8)
        }
        #[doc = "USART1 clock source selection"]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: super::vals::Usart1sel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "USART2 clock source selection"]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "USART2 clock source selection"]
        #[inline(always)]
        pub fn set_usart2sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "LPUART1 clock source selection"]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 clock source selection"]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub fn set_i2c1sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub fn set_i2c3sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Low Power Timer clock source selection"]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "Low Power Timer clock source selection"]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
    }
    impl Default for Ccipr {
        #[inline(always)]
        fn default() -> Ccipr {
            Ccipr(0)
        }
    }
    impl core::fmt::Debug for Ccipr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr")
                .field("usart1sel", &self.usart1sel())
                .field("usart2sel", &self.usart2sel())
                .field("lpuart1sel", &self.lpuart1sel())
                .field("i2c1sel", &self.i2c1sel())
                .field("i2c3sel", &self.i2c3sel())
                .field("lptim1sel", &self.lptim1sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccipr {{ usart1sel: {:?}, usart2sel: {:?}, lpuart1sel: {:?}, i2c1sel: {:?}, i2c3sel: {:?}, lptim1sel: {:?} }}" , self . usart1sel () , self . usart2sel () , self . lpuart1sel () , self . i2c1sel () , self . i2c3sel () , self . lptim1sel ())
        }
    }
    #[doc = "Clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "APB low-speed prescaler (APB1)"]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB1)"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 11usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
        }
        #[doc = "Wake-up from stop clock selection"]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "Wake-up from stop clock selection"]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "PLL entry clock source"]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL entry clock source"]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL multiplication factor"]
        #[inline(always)]
        pub const fn pllmul(&self) -> super::vals::Pllmul {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Pllmul::from_bits(val as u8)
        }
        #[doc = "PLL multiplication factor"]
        #[inline(always)]
        pub fn set_pllmul(&mut self, val: super::vals::Pllmul) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "PLL output division"]
        #[inline(always)]
        pub const fn plldiv(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL output division"]
        #[inline(always)]
        pub fn set_plldiv(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Microcontroller clock output selection"]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output selection"]
        #[inline(always)]
        pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Microcontroller clock output prescaler"]
        #[inline(always)]
        pub const fn mcopre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output prescaler"]
        #[inline(always)]
        pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
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
                .field("sw", &self.sw())
                .field("sws", &self.sws())
                .field("hpre", &self.hpre())
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("stopwuck", &self.stopwuck())
                .field("pllsrc", &self.pllsrc())
                .field("pllmul", &self.pllmul())
                .field("plldiv", &self.plldiv())
                .field("mcosel", &self.mcosel())
                .field("mcopre", &self.mcopre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ sw: {:?}, sws: {:?}, hpre: {:?}, ppre1: {:?}, ppre2: {:?}, stopwuck: {:?}, pllsrc: {:?}, pllmul: {:?}, plldiv: {:?}, mcosel: {:?}, mcopre: {:?} }}" , self . sw () , self . sws () , self . hpre () , self . ppre1 () , self . ppre2 () , self . stopwuck () , self . pllsrc () , self . pllmul () , self . plldiv () , self . mcosel () , self . mcopre ())
        }
    }
    #[doc = "Clock interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready Interrupt clear"]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt clear"]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt clear"]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt clear"]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt clear"]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt clear"]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt clear"]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt clear"]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PLL ready Interrupt clear"]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready Interrupt clear"]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MSI ready Interrupt clear"]
        #[inline(always)]
        pub const fn msirdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready Interrupt clear"]
        #[inline(always)]
        pub fn set_msirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE Clock Security System Interrupt clear"]
        #[inline(always)]
        pub const fn csslsec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Clock Security System Interrupt clear"]
        #[inline(always)]
        pub fn set_csslsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock Security System Interrupt clear"]
        #[inline(always)]
        pub const fn csshsec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System Interrupt clear"]
        #[inline(always)]
        pub fn set_csshsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("pllrdyc", &self.pllrdyc())
                .field("msirdyc", &self.msirdyc())
                .field("csslsec", &self.csslsec())
                .field("csshsec", &self.csshsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cicr {{ lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, pllrdyc: {=bool:?}, msirdyc: {=bool:?}, csslsec: {=bool:?}, csshsec: {=bool:?} }}" , self . lsirdyc () , self . lserdyc () , self . hsirdyc () , self . hserdyc () , self . pllrdyc () , self . msirdyc () , self . csslsec () , self . csshsec ())
        }
    }
    #[doc = "Clock interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MSI ready interrupt flag"]
        #[inline(always)]
        pub const fn msirdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_msirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE CSS interrupt flag"]
        #[inline(always)]
        pub const fn csslse(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LSE CSS interrupt flag"]
        #[inline(always)]
        pub fn set_csslse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("pllrdyie", &self.pllrdyie())
                .field("msirdyie", &self.msirdyie())
                .field("csslse", &self.csslse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cier {{ lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, pllrdyie: {=bool:?}, msirdyie: {=bool:?}, csslse: {=bool:?} }}" , self . lsirdyie () , self . lserdyie () , self . hsirdyie () , self . hserdyie () , self . pllrdyie () , self . msirdyie () , self . csslse ())
        }
    }
    #[doc = "Clock interrupt flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MSI ready interrupt flag"]
        #[inline(always)]
        pub const fn msirdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_msirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE Clock Security System Interrupt flag"]
        #[inline(always)]
        pub const fn csslsef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Clock Security System Interrupt flag"]
        #[inline(always)]
        pub fn set_csslsef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock Security System Interrupt flag"]
        #[inline(always)]
        pub const fn csshsef(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System Interrupt flag"]
        #[inline(always)]
        pub fn set_csshsef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("pllrdyf", &self.pllrdyf())
                .field("msirdyf", &self.msirdyf())
                .field("csslsef", &self.csslsef())
                .field("csshsef", &self.csshsef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cifr {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, pllrdyf: {=bool:?}, msirdyf: {=bool:?}, csslsef: {=bool:?}, csshsef: {=bool:?} }}" , self . lsirdyf () , self . lserdyf () , self . hsirdyf () , self . hserdyf () , self . pllrdyf () , self . msirdyf () , self . csslsef () , self . csshsef ())
        }
    }
    #[doc = "Clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "16 MHz high-speed internal clock enable"]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "16 MHz high-speed internal clock enable"]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "High-speed internal clock enable bit for some IP kernels"]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed internal clock enable bit for some IP kernels"]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal high-speed clock ready flag"]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed clock ready flag"]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSIDIVEN"]
        #[inline(always)]
        pub const fn hsidiven(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSIDIVEN"]
        #[inline(always)]
        pub fn set_hsidiven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSIDIVF"]
        #[inline(always)]
        pub const fn hsidivf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSIDIVF"]
        #[inline(always)]
        pub fn set_hsidivf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "16 MHz high-speed internal clock output enable"]
        #[inline(always)]
        pub const fn hsiouten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "16 MHz high-speed internal clock output enable"]
        #[inline(always)]
        pub fn set_hsiouten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MSI clock enable"]
        #[inline(always)]
        pub const fn msion(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock enable"]
        #[inline(always)]
        pub fn set_msion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MSI clock ready flag"]
        #[inline(always)]
        pub const fn msirdy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock ready flag"]
        #[inline(always)]
        pub fn set_msirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE clock bypass"]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock bypass"]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock security system on HSE enable"]
        #[inline(always)]
        pub const fn csshseon(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system on HSE enable"]
        #[inline(always)]
        pub fn set_csshseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TC/LCD prescaler"]
        #[inline(always)]
        pub const fn rtcpre(&self) -> super::vals::Rtcpre {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Rtcpre::from_bits(val as u8)
        }
        #[doc = "TC/LCD prescaler"]
        #[inline(always)]
        pub fn set_rtcpre(&mut self, val: super::vals::Rtcpre) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "PLL enable"]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL enable"]
        #[inline(always)]
        pub fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL clock ready flag"]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PLL clock ready flag"]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
                .field("hsion", &self.hsion())
                .field("hsikeron", &self.hsikeron())
                .field("hsirdy", &self.hsirdy())
                .field("hsidiven", &self.hsidiven())
                .field("hsidivf", &self.hsidivf())
                .field("hsiouten", &self.hsiouten())
                .field("msion", &self.msion())
                .field("msirdy", &self.msirdy())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("csshseon", &self.csshseon())
                .field("rtcpre", &self.rtcpre())
                .field("pllon", &self.pllon())
                .field("pllrdy", &self.pllrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ hsion: {=bool:?}, hsikeron: {=bool:?}, hsirdy: {=bool:?}, hsidiven: {=bool:?}, hsidivf: {=bool:?}, hsiouten: {=bool:?}, msion: {=bool:?}, msirdy: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, csshseon: {=bool:?}, rtcpre: {:?}, pllon: {=bool:?}, pllrdy: {=bool:?} }}" , self . hsion () , self . hsikeron () , self . hsirdy () , self . hsidiven () , self . hsidivf () , self . hsiouten () , self . msion () , self . msirdy () , self . hseon () , self . hserdy () , self . hsebyp () , self . csshseon () , self . rtcpre () , self . pllon () , self . pllrdy ())
        }
    }
    #[doc = "Control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Internal low-speed oscillator enable"]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low-speed oscillator enable"]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal low-speed oscillator ready"]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low-speed oscillator ready"]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External low-speed oscillator enable"]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator enable"]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External low-speed oscillator ready"]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator ready"]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "External low-speed oscillator bypass"]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator bypass"]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LSEDRV"]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 11usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSEDRV"]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
        }
        #[doc = "CSSLSEON"]
        #[inline(always)]
        pub const fn csslseon(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CSSLSEON"]
        #[inline(always)]
        pub fn set_csslseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CSS on LSE failure detection flag"]
        #[inline(always)]
        pub const fn csslsed(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CSS on LSE failure detection flag"]
        #[inline(always)]
        pub fn set_csslsed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "RTC and LCD clock source selection"]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC and LCD clock source selection"]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "RTC software reset"]
        #[inline(always)]
        pub const fn rtcrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "RTC software reset"]
        #[inline(always)]
        pub fn set_rtcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Firewall reset flag"]
        #[inline(always)]
        pub const fn fwrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Firewall reset flag"]
        #[inline(always)]
        pub fn set_fwrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "OBLRSTF"]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "OBLRSTF"]
        #[inline(always)]
        pub fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag"]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag"]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag"]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag"]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag"]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag"]
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
                .field("lseon", &self.lseon())
                .field("lserdy", &self.lserdy())
                .field("lsebyp", &self.lsebyp())
                .field("lsedrv", &self.lsedrv())
                .field("csslseon", &self.csslseon())
                .field("csslsed", &self.csslsed())
                .field("rtcsel", &self.rtcsel())
                .field("rtcen", &self.rtcen())
                .field("rtcrst", &self.rtcrst())
                .field("rmvf", &self.rmvf())
                .field("fwrstf", &self.fwrstf())
                .field("oblrstf", &self.oblrstf())
                .field("pinrstf", &self.pinrstf())
                .field("porrstf", &self.porrstf())
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
            defmt :: write ! (f , "Csr {{ lsion: {=bool:?}, lsirdy: {=bool:?}, lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, lsedrv: {:?}, csslseon: {=bool:?}, csslsed: {=bool:?}, rtcsel: {:?}, rtcen: {=bool:?}, rtcrst: {=bool:?}, rmvf: {=bool:?}, fwrstf: {=bool:?}, oblrstf: {=bool:?}, pinrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}" , self . lsion () , self . lsirdy () , self . lseon () , self . lserdy () , self . lsebyp () , self . lsedrv () , self . csslseon () , self . csslsed () , self . rtcsel () , self . rtcen () , self . rtcrst () , self . rmvf () , self . fwrstf () , self . oblrstf () , self . pinrstf () , self . porrstf () , self . sftrstf () , self . iwdgrstf () , self . wwdgrstf () , self . lpwrrstf ())
        }
    }
    #[doc = "GPIO clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpioenr(pub u32);
    impl Gpioenr {
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port D clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D clock enable"]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clock enable"]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clock enable"]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port H clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port H clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Gpioenr {
        #[inline(always)]
        fn default() -> Gpioenr {
            Gpioenr(0)
        }
    }
    impl core::fmt::Debug for Gpioenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpioenr")
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiohen", &self.gpiohen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpioenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gpioenr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiohen: {=bool:?} }}" , self . gpioaen () , self . gpioben () , self . gpiocen () , self . gpioden () , self . gpioeen () , self . gpiohen ())
        }
    }
    #[doc = "GPIO reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpiorstr(pub u32);
    impl Gpiorstr {
        #[doc = "I/O port A reset"]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A reset"]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I/O port B reset"]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port B reset"]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I/O port A reset"]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A reset"]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port D reset"]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D reset"]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O port E reset"]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port E reset"]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port H reset"]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port H reset"]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Gpiorstr {
        #[inline(always)]
        fn default() -> Gpiorstr {
            Gpiorstr(0)
        }
    }
    impl core::fmt::Debug for Gpiorstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpiorstr")
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiohrst", &self.gpiohrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpiorstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gpiorstr {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiohrst: {=bool:?} }}" , self . gpioarst () , self . gpiobrst () , self . gpiocrst () , self . gpiodrst () , self . gpioerst () , self . gpiohrst ())
        }
    }
    #[doc = "GPIO clock enable in sleep mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpiosmen(pub u32);
    impl Gpiosmen {
        #[doc = "Port A clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioasmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port A clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port B clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiobsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port B clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiobsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port C clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiocsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port C clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiocsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port D clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiodsmen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port D clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiodsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port E clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioesmen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port E clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port H clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiohsmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port H clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiohsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Gpiosmen {
        #[inline(always)]
        fn default() -> Gpiosmen {
            Gpiosmen(0)
        }
    }
    impl core::fmt::Debug for Gpiosmen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpiosmen")
                .field("gpioasmen", &self.gpioasmen())
                .field("gpiobsmen", &self.gpiobsmen())
                .field("gpiocsmen", &self.gpiocsmen())
                .field("gpiodsmen", &self.gpiodsmen())
                .field("gpioesmen", &self.gpioesmen())
                .field("gpiohsmen", &self.gpiohsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpiosmen {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gpiosmen {{ gpioasmen: {=bool:?}, gpiobsmen: {=bool:?}, gpiocsmen: {=bool:?}, gpiodsmen: {=bool:?}, gpioesmen: {=bool:?}, gpiohsmen: {=bool:?} }}" , self . gpioasmen () , self . gpiobsmen () , self . gpiocsmen () , self . gpiodsmen () , self . gpioesmen () , self . gpiohsmen ())
        }
    }
    #[doc = "Internal clock sources calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr(pub u32);
    impl Icscr {
        #[doc = "nternal high speed clock calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "nternal high speed clock calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "High speed internal clock trimming"]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "High speed internal clock trimming"]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "MSI clock ranges"]
        #[inline(always)]
        pub const fn msirange(&self) -> super::vals::Msirange {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Msirange::from_bits(val as u8)
        }
        #[doc = "MSI clock ranges"]
        #[inline(always)]
        pub fn set_msirange(&mut self, val: super::vals::Msirange) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "MSI clock calibration"]
        #[inline(always)]
        pub const fn msical(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "MSI clock calibration"]
        #[inline(always)]
        pub fn set_msical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "MSI clock trimming"]
        #[inline(always)]
        pub const fn msitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "MSI clock trimming"]
        #[inline(always)]
        pub fn set_msitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Icscr {
        #[inline(always)]
        fn default() -> Icscr {
            Icscr(0)
        }
    }
    impl core::fmt::Debug for Icscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr")
                .field("hsical", &self.hsical())
                .field("hsitrim", &self.hsitrim())
                .field("msirange", &self.msirange())
                .field("msical", &self.msical())
                .field("msitrim", &self.msitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icscr {{ hsical: {=u8:?}, hsitrim: {=u8:?}, msirange: {:?}, msical: {=u8:?}, msitrim: {=u8:?} }}",
                self.hsical(),
                self.hsitrim(),
                self.msirange(),
                self.msical(),
                self.msitrim()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "system clock not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "system clock divided by 2"]
        DIV2 = 0x08,
        #[doc = "system clock divided by 4"]
        DIV4 = 0x09,
        #[doc = "system clock divided by 8"]
        DIV8 = 0x0a,
        #[doc = "system clock divided by 16"]
        DIV16 = 0x0b,
        #[doc = "system clock divided by 64"]
        DIV64 = 0x0c,
        #[doc = "system clock divided by 128"]
        DIV128 = 0x0d,
        #[doc = "system clock divided by 256"]
        DIV256 = 0x0e,
        #[doc = "system clock divided by 512"]
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
    pub enum I2csel {
        #[doc = "APB clock selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "System clock selected as peripheral clock"]
        SYS = 0x01,
        #[doc = "HSI clock selected as peripheral clock"]
        HSI = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl I2csel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2csel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Lptimsel {
        #[doc = "APB clock selected as Timer clock"]
        PCLK1 = 0x0,
        #[doc = "LSI clock selected as Timer clock"]
        LSI = 0x01,
        #[doc = "HSI clock selected as Timer clock"]
        HSI = 0x02,
        #[doc = "LSE clock selected as Timer clock"]
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
    pub enum Lsedrv {
        #[doc = "Low driving capability"]
        LOW = 0x0,
        #[doc = "Medium low driving capability"]
        MEDIUM_LOW = 0x01,
        #[doc = "Medium high driving capability"]
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
    pub enum Mcopre {
        #[doc = "No division"]
        DIV1 = 0x0,
        #[doc = "Division by 2"]
        DIV2 = 0x01,
        #[doc = "Division by 4"]
        DIV4 = 0x02,
        #[doc = "Division by 8"]
        DIV8 = 0x03,
        #[doc = "Division by 16"]
        DIV16 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
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
        #[doc = "No clock"]
        DISABLE = 0x0,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x01,
        #[doc = "HSI oscillator clock selected"]
        HSI = 0x02,
        #[doc = "MSI oscillator clock selected"]
        MSI = 0x03,
        #[doc = "HSE oscillator clock selected"]
        HSE = 0x04,
        #[doc = "PLL clock selected"]
        PLL = 0x05,
        #[doc = "LSI oscillator clock selected"]
        LSI = 0x06,
        #[doc = "LSE oscillator clock selected"]
        LSE = 0x07,
        _RESERVED_8 = 0x08,
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
    pub enum Msirange {
        #[doc = "range 0 around 65.536 kHz"]
        RANGE66K = 0x0,
        #[doc = "range 1 around 131.072 kHz"]
        RANGE131K = 0x01,
        #[doc = "range 2 around 262.144 kHz"]
        RANGE262K = 0x02,
        #[doc = "range 3 around 524.288 kHz"]
        RANGE524K = 0x03,
        #[doc = "range 4 around 1.048 MHz"]
        RANGE1M = 0x04,
        #[doc = "range 5 around 2.097 MHz (reset value)"]
        RANGE2M = 0x05,
        #[doc = "range 6 around 4.194 MHz"]
        RANGE4M = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Msirange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msirange {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msirange {
        #[inline(always)]
        fn from(val: u8) -> Msirange {
            Msirange::from_bits(val)
        }
    }
    impl From<Msirange> for u8 {
        #[inline(always)]
        fn from(val: Msirange) -> u8 {
            Msirange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plldiv {
        _RESERVED_0 = 0x0,
        #[doc = "PLLVCO / 2"]
        DIV2 = 0x01,
        #[doc = "PLLVCO / 3"]
        DIV3 = 0x02,
        #[doc = "PLLVCO / 4"]
        DIV4 = 0x03,
    }
    impl Plldiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plldiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Plldiv {
        #[inline(always)]
        fn from(val: u8) -> Plldiv {
            Plldiv::from_bits(val)
        }
    }
    impl From<Plldiv> for u8 {
        #[inline(always)]
        fn from(val: Plldiv) -> u8 {
            Plldiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllmul {
        #[doc = "PLL clock entry x 3"]
        MUL3 = 0x0,
        #[doc = "PLL clock entry x 4"]
        MUL4 = 0x01,
        #[doc = "PLL clock entry x 6"]
        MUL6 = 0x02,
        #[doc = "PLL clock entry x 8"]
        MUL8 = 0x03,
        #[doc = "PLL clock entry x 12"]
        MUL12 = 0x04,
        #[doc = "PLL clock entry x 16"]
        MUL16 = 0x05,
        #[doc = "PLL clock entry x 24"]
        MUL24 = 0x06,
        #[doc = "PLL clock entry x 32"]
        MUL32 = 0x07,
        #[doc = "PLL clock entry x 48"]
        MUL48 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Pllmul {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllmul {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllmul {
        #[inline(always)]
        fn from(val: u8) -> Pllmul {
            Pllmul::from_bits(val)
        }
    }
    impl From<Pllmul> for u8 {
        #[inline(always)]
        fn from(val: Pllmul) -> u8 {
            Pllmul::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "HSI selected as PLL input clock"]
        HSI = 0x0,
        #[doc = "HSE selected as PLL input clock"]
        HSE = 0x01,
    }
    impl Pllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsrc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsrc {
        #[inline(always)]
        fn from(val: u8) -> Pllsrc {
            Pllsrc::from_bits(val)
        }
    }
    impl From<Pllsrc> for u8 {
        #[inline(always)]
        fn from(val: Pllsrc) -> u8 {
            Pllsrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre {
        #[doc = "HCLK not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HCLK divided by 2"]
        DIV2 = 0x04,
        #[doc = "HCLK divided by 4"]
        DIV4 = 0x05,
        #[doc = "HCLK divided by 8"]
        DIV8 = 0x06,
        #[doc = "HCLK divided by 16"]
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
    pub enum Rtcpre {
        #[doc = "HSE divided by 2"]
        DIV2 = 0x0,
        #[doc = "HSE divided by 4"]
        DIV4 = 0x01,
        #[doc = "HSE divided by 8"]
        DIV8 = 0x02,
        #[doc = "HSE divided by 16"]
        DIV16 = 0x03,
    }
    impl Rtcpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtcpre {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtcpre {
        #[inline(always)]
        fn from(val: u8) -> Rtcpre {
            Rtcpre::from_bits(val)
        }
    }
    impl From<Rtcpre> for u8 {
        #[inline(always)]
        fn from(val: Rtcpre) -> u8 {
            Rtcpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcsel {
        #[doc = "No clock"]
        DISABLE = 0x0,
        #[doc = "LSE oscillator clock used as RTC clock"]
        LSE = 0x01,
        #[doc = "LSI oscillator clock used as RTC clock"]
        LSI = 0x02,
        #[doc = "HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\\[1:0\\]
bits in the RCC clock control register (RCC_CR)) used as the RTC clock"]
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
    pub enum Stopwuck {
        #[doc = "Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock"]
        MSI = 0x0,
        #[doc = "Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI/4 if HSIDIVEN=1)"]
        HSI = 0x01,
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
        #[doc = "MSI oscillator used as system clock"]
        MSI = 0x0,
        #[doc = "HSI oscillator used as system clock"]
        HSI = 0x01,
        #[doc = "HSE oscillator used as system clock"]
        HSE = 0x02,
        #[doc = "PLL used as system clock"]
        PLL1_R = 0x03,
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
    pub enum Uartsel {
        #[doc = "APB clock selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "System clock selected as peripheral clock"]
        SYS = 0x01,
        #[doc = "HSI clock selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "LSE clock selected as peripheral clock"]
        LSE = 0x03,
    }
    impl Uartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Uartsel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Usart1sel {
        #[doc = "APB clock selected as peripheral clock"]
        PCLK2 = 0x0,
        #[doc = "System clock selected as peripheral clock"]
        SYS = 0x01,
        #[doc = "HSI clock selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "LSE clock selected as peripheral clock"]
        LSE = 0x03,
    }
    impl Usart1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart1sel {
        #[inline(always)]
        fn from(val: u8) -> Usart1sel {
            Usart1sel::from_bits(val)
        }
    }
    impl From<Usart1sel> for u8 {
        #[inline(always)]
        fn from(val: Usart1sel) -> u8 {
            Usart1sel::to_bits(val)
        }
    }
}
