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
    #[doc = "clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "clock interrupt register"]
    #[inline(always)]
    pub const fn cir(self) -> crate::common::Reg<regs::Cir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rstr(self) -> crate::common::Reg<regs::Apb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "AHB1 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1enr(self) -> crate::common::Reg<regs::Apb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "AHB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "AHB2 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "AHB3 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb3lpenr(self) -> crate::common::Reg<regs::Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn apb1lpenr(self) -> crate::common::Reg<regs::Apb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "APB2 peripheral clock enabled in low power mode register"]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "clock control & status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "spread spectrum clock generation register"]
    #[inline(always)]
    pub const fn sscgr(self) -> crate::common::Reg<regs::Sscgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "PLLI2S configuration register"]
    #[inline(always)]
    pub const fn plli2scfgr(self) -> crate::common::Reg<regs::Plli2scfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB1 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
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
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clock enable"]
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
        #[doc = "IO port F clock enable"]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clock enable"]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clock enable"]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clock enable"]
        #[inline(always)]
        pub fn set_gpioien(&mut self, val: bool) {
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
        #[doc = "Backup SRAM interface clock enable"]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Backup SRAM interface clock enable"]
        #[inline(always)]
        pub fn set_bkpsramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ethernet MAC clock enable"]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC clock enable"]
        #[inline(always)]
        pub fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Ethernet Transmission clock enable"]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission clock enable"]
        #[inline(always)]
        pub fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Ethernet Reception clock enable"]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception clock enable"]
        #[inline(always)]
        pub fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Ethernet PTP clock enable"]
        #[inline(always)]
        pub const fn ethptpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PTP clock enable"]
        #[inline(always)]
        pub fn set_ethptpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "USB OTG HS clock enable"]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "USB OTG HSULPI clock enable"]
        #[inline(always)]
        pub const fn usb_otg_hsulpien(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HSULPI clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_hsulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ahb1enr {
        #[inline(always)]
        fn default() -> Ahb1enr {
            Ahb1enr(0)
        }
    }
    impl core::fmt::Debug for Ahb1enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1enr")
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("gpioien", &self.gpioien())
                .field("crcen", &self.crcen())
                .field("bkpsramen", &self.bkpsramen())
                .field("dma1en", &self.dma1en())
                .field("dma2en", &self.dma2en())
                .field("ethen", &self.ethen())
                .field("ethtxen", &self.ethtxen())
                .field("ethrxen", &self.ethrxen())
                .field("ethptpen", &self.ethptpen())
                .field("usb_otg_hsen", &self.usb_otg_hsen())
                .field("usb_otg_hsulpien", &self.usb_otg_hsulpien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1enr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiofen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, gpioien: {=bool:?}, crcen: {=bool:?}, bkpsramen: {=bool:?}, dma1en: {=bool:?}, dma2en: {=bool:?}, ethen: {=bool:?}, ethtxen: {=bool:?}, ethrxen: {=bool:?}, ethptpen: {=bool:?}, usb_otg_hsen: {=bool:?}, usb_otg_hsulpien: {=bool:?} }}" , self . gpioaen () , self . gpioben () , self . gpiocen () , self . gpioden () , self . gpioeen () , self . gpiofen () , self . gpiogen () , self . gpiohen () , self . gpioien () , self . crcen () , self . bkpsramen () , self . dma1en () , self . dma2en () , self . ethen () , self . ethtxen () , self . ethrxen () , self . ethptpen () , self . usb_otg_hsen () , self . usb_otg_hsulpien ())
        }
    }
    #[doc = "AHB1 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "IO port A clock enable during sleep mode"]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioilpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Flash interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn flashlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_flashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SRAM 1interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM 1interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SRAM 2 interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM 2 interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Backup SRAM interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn bkpsramlpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Backup SRAM interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_bkpsramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DMA1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn dma1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn dma2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ethernet MAC clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ethlpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ethlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Ethernet transmission clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ethtxlpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet transmission clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ethtxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Ethernet reception clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ethrxlpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet reception clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ethrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Ethernet PTP clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ethptplpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PTP clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ethptplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "USB OTG HS clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hslpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "USB OTG HS ULPI clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hsulpilpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS ULPI clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hsulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ahb1lpenr {
        #[inline(always)]
        fn default() -> Ahb1lpenr {
            Ahb1lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb1lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1lpenr")
                .field("gpioalpen", &self.gpioalpen())
                .field("gpioblpen", &self.gpioblpen())
                .field("gpioclpen", &self.gpioclpen())
                .field("gpiodlpen", &self.gpiodlpen())
                .field("gpioelpen", &self.gpioelpen())
                .field("gpioflpen", &self.gpioflpen())
                .field("gpioglpen", &self.gpioglpen())
                .field("gpiohlpen", &self.gpiohlpen())
                .field("gpioilpen", &self.gpioilpen())
                .field("crclpen", &self.crclpen())
                .field("flashlpen", &self.flashlpen())
                .field("sram1lpen", &self.sram1lpen())
                .field("sram2lpen", &self.sram2lpen())
                .field("bkpsramlpen", &self.bkpsramlpen())
                .field("dma1lpen", &self.dma1lpen())
                .field("dma2lpen", &self.dma2lpen())
                .field("ethlpen", &self.ethlpen())
                .field("ethtxlpen", &self.ethtxlpen())
                .field("ethrxlpen", &self.ethrxlpen())
                .field("ethptplpen", &self.ethptplpen())
                .field("usb_otg_hslpen", &self.usb_otg_hslpen())
                .field("usb_otg_hsulpilpen", &self.usb_otg_hsulpilpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1lpenr {{ gpioalpen: {=bool:?}, gpioblpen: {=bool:?}, gpioclpen: {=bool:?}, gpiodlpen: {=bool:?}, gpioelpen: {=bool:?}, gpioflpen: {=bool:?}, gpioglpen: {=bool:?}, gpiohlpen: {=bool:?}, gpioilpen: {=bool:?}, crclpen: {=bool:?}, flashlpen: {=bool:?}, sram1lpen: {=bool:?}, sram2lpen: {=bool:?}, bkpsramlpen: {=bool:?}, dma1lpen: {=bool:?}, dma2lpen: {=bool:?}, ethlpen: {=bool:?}, ethtxlpen: {=bool:?}, ethrxlpen: {=bool:?}, ethptplpen: {=bool:?}, usb_otg_hslpen: {=bool:?}, usb_otg_hsulpilpen: {=bool:?} }}" , self . gpioalpen () , self . gpioblpen () , self . gpioclpen () , self . gpiodlpen () , self . gpioelpen () , self . gpioflpen () , self . gpioglpen () , self . gpiohlpen () , self . gpioilpen () , self . crclpen () , self . flashlpen () , self . sram1lpen () , self . sram2lpen () , self . bkpsramlpen () , self . dma1lpen () , self . dma2lpen () , self . ethlpen () , self . ethtxlpen () , self . ethrxlpen () , self . ethptplpen () , self . usb_otg_hslpen () , self . usb_otg_hsulpilpen ())
        }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I reset"]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I reset"]
        #[inline(always)]
        pub fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ethernet MAC reset"]
        #[inline(always)]
        pub const fn ethrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC reset"]
        #[inline(always)]
        pub fn set_ethrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB OTG HS module reset"]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS module reset"]
        #[inline(always)]
        pub fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ahb1rstr {
        #[inline(always)]
        fn default() -> Ahb1rstr {
            Ahb1rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb1rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1rstr")
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiofrst", &self.gpiofrst())
                .field("gpiogrst", &self.gpiogrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("gpioirst", &self.gpioirst())
                .field("crcrst", &self.crcrst())
                .field("dma1rst", &self.dma1rst())
                .field("dma2rst", &self.dma2rst())
                .field("ethrst", &self.ethrst())
                .field("usb_otg_hsrst", &self.usb_otg_hsrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1rstr {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiofrst: {=bool:?}, gpiogrst: {=bool:?}, gpiohrst: {=bool:?}, gpioirst: {=bool:?}, crcrst: {=bool:?}, dma1rst: {=bool:?}, dma2rst: {=bool:?}, ethrst: {=bool:?}, usb_otg_hsrst: {=bool:?} }}" , self . gpioarst () , self . gpiobrst () , self . gpiocrst () , self . gpiodrst () , self . gpioerst () , self . gpiofrst () , self . gpiogrst () , self . gpiohrst () , self . gpioirst () , self . crcrst () , self . dma1rst () , self . dma2rst () , self . ethrst () , self . usb_otg_hsrst ())
        }
    }
    #[doc = "AHB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "Camera interface enable"]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Camera interface enable"]
        #[inline(always)]
        pub fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Cryptographic modules clock enable"]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptographic modules clock enable"]
        #[inline(always)]
        pub fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash modules clock enable"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash modules clock enable"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random number generator clock enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator clock enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USB OTG FS clock enable"]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG FS clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ahb2enr {
        #[inline(always)]
        fn default() -> Ahb2enr {
            Ahb2enr(0)
        }
    }
    impl core::fmt::Debug for Ahb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2enr")
                .field("dcmien", &self.dcmien())
                .field("crypen", &self.crypen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("usb_otg_fsen", &self.usb_otg_fsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2enr {{ dcmien: {=bool:?}, crypen: {=bool:?}, hashen: {=bool:?}, rngen: {=bool:?}, usb_otg_fsen: {=bool:?} }}" , self . dcmien () , self . crypen () , self . hashen () , self . rngen () , self . usb_otg_fsen ())
        }
    }
    #[doc = "AHB2 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "Camera interface enable during Sleep mode"]
        #[inline(always)]
        pub const fn dcmilpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Camera interface enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dcmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Cryptography modules clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn cryplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptography modules clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_cryplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash modules clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash modules clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random number generator clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USB OTG FS clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usb_otg_fslpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG FS clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_fslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ahb2lpenr {
        #[inline(always)]
        fn default() -> Ahb2lpenr {
            Ahb2lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb2lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2lpenr")
                .field("dcmilpen", &self.dcmilpen())
                .field("cryplpen", &self.cryplpen())
                .field("hashlpen", &self.hashlpen())
                .field("rnglpen", &self.rnglpen())
                .field("usb_otg_fslpen", &self.usb_otg_fslpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2lpenr {{ dcmilpen: {=bool:?}, cryplpen: {=bool:?}, hashlpen: {=bool:?}, rnglpen: {=bool:?}, usb_otg_fslpen: {=bool:?} }}" , self . dcmilpen () , self . cryplpen () , self . hashlpen () , self . rnglpen () , self . usb_otg_fslpen ())
        }
    }
    #[doc = "AHB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "Camera interface reset"]
        #[inline(always)]
        pub const fn dcmirst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Camera interface reset"]
        #[inline(always)]
        pub fn set_dcmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Cryptographic module reset"]
        #[inline(always)]
        pub const fn cryprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptographic module reset"]
        #[inline(always)]
        pub fn set_cryprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash module reset"]
        #[inline(always)]
        pub const fn hsahrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash module reset"]
        #[inline(always)]
        pub fn set_hsahrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random number generator module reset"]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator module reset"]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USB OTG FS module reset"]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG FS module reset"]
        #[inline(always)]
        pub fn set_usb_otg_fsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ahb2rstr {
        #[inline(always)]
        fn default() -> Ahb2rstr {
            Ahb2rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2rstr")
                .field("dcmirst", &self.dcmirst())
                .field("cryprst", &self.cryprst())
                .field("hsahrst", &self.hsahrst())
                .field("rngrst", &self.rngrst())
                .field("usb_otg_fsrst", &self.usb_otg_fsrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2rstr {{ dcmirst: {=bool:?}, cryprst: {=bool:?}, hsahrst: {=bool:?}, rngrst: {=bool:?}, usb_otg_fsrst: {=bool:?} }}" , self . dcmirst () , self . cryprst () , self . hsahrst () , self . rngrst () , self . usb_otg_fsrst ())
        }
    }
    #[doc = "AHB3 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "Flexible static memory controller module clock enable"]
        #[inline(always)]
        pub const fn fsmcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible static memory controller module clock enable"]
        #[inline(always)]
        pub fn set_fsmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ahb3enr {
        #[inline(always)]
        fn default() -> Ahb3enr {
            Ahb3enr(0)
        }
    }
    impl core::fmt::Debug for Ahb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3enr").field("fsmcen", &self.fsmcen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb3enr {{ fsmcen: {=bool:?} }}", self.fsmcen())
        }
    }
    #[doc = "AHB3 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3lpenr(pub u32);
    impl Ahb3lpenr {
        #[doc = "Flexible static memory controller module clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn fsmclpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible static memory controller module clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_fsmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ahb3lpenr {
        #[inline(always)]
        fn default() -> Ahb3lpenr {
            Ahb3lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb3lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3lpenr").field("fsmclpen", &self.fsmclpen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb3lpenr {{ fsmclpen: {=bool:?} }}", self.fsmclpen())
        }
    }
    #[doc = "AHB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "Flexible static memory controller module reset"]
        #[inline(always)]
        pub const fn fsmcrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible static memory controller module reset"]
        #[inline(always)]
        pub fn set_fsmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ahb3rstr {
        #[inline(always)]
        fn default() -> Ahb3rstr {
            Ahb3rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb3rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3rstr").field("fsmcrst", &self.fsmcrst()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb3rstr {{ fsmcrst: {=bool:?} }}", self.fsmcrst())
        }
    }
    #[doc = "APB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr(pub u32);
    impl Apb1enr {
        #[doc = "TIM2 clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clock enable"]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable"]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 clock enable"]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable"]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 clock enable"]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 clock enable"]
        #[inline(always)]
        pub fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 clock enable"]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 clock enable"]
        #[inline(always)]
        pub fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 clock enable"]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 clock enable"]
        #[inline(always)]
        pub fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART 2 clock enable"]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 clock enable"]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable"]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable"]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable"]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable"]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable"]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable"]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
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
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CAN 1 clock enable"]
        #[inline(always)]
        pub const fn can1en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 1 clock enable"]
        #[inline(always)]
        pub fn set_can1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN 2 clock enable"]
        #[inline(always)]
        pub const fn can2en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 2 clock enable"]
        #[inline(always)]
        pub fn set_can2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("tim4en", &self.tim4en())
                .field("tim5en", &self.tim5en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("tim12en", &self.tim12en())
                .field("tim13en", &self.tim13en())
                .field("tim14en", &self.tim14en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i2c3en", &self.i2c3en())
                .field("can1en", &self.can1en())
                .field("can2en", &self.can2en())
                .field("pwren", &self.pwren())
                .field("dacen", &self.dacen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1enr {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, tim5en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, tim12en: {=bool:?}, tim13en: {=bool:?}, tim14en: {=bool:?}, wwdgen: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, i2c3en: {=bool:?}, can1en: {=bool:?}, can2en: {=bool:?}, pwren: {=bool:?}, dacen: {=bool:?} }}" , self . tim2en () , self . tim3en () , self . tim4en () , self . tim5en () , self . tim6en () , self . tim7en () , self . tim12en () , self . tim13en () , self . tim14en () , self . wwdgen () , self . spi2en () , self . spi3en () , self . usart2en () , self . usart3en () , self . uart4en () , self . uart5en () , self . i2c1en () , self . i2c2en () , self . i2c3en () , self . can1en () , self . can2en () , self . pwren () , self . dacen ())
        }
    }
    #[doc = "APB1 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lpenr(pub u32);
    impl Apb1lpenr {
        #[doc = "TIM2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Window watchdog clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn wwdglpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_wwdglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CAN 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn can1lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_can1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN 2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn can2lpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_can2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Power interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn pwrlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_pwrlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn daclpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_daclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb1lpenr {
        #[inline(always)]
        fn default() -> Apb1lpenr {
            Apb1lpenr(0)
        }
    }
    impl core::fmt::Debug for Apb1lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lpenr")
                .field("tim2lpen", &self.tim2lpen())
                .field("tim3lpen", &self.tim3lpen())
                .field("tim4lpen", &self.tim4lpen())
                .field("tim5lpen", &self.tim5lpen())
                .field("tim6lpen", &self.tim6lpen())
                .field("tim7lpen", &self.tim7lpen())
                .field("tim12lpen", &self.tim12lpen())
                .field("tim13lpen", &self.tim13lpen())
                .field("tim14lpen", &self.tim14lpen())
                .field("wwdglpen", &self.wwdglpen())
                .field("spi2lpen", &self.spi2lpen())
                .field("spi3lpen", &self.spi3lpen())
                .field("usart2lpen", &self.usart2lpen())
                .field("usart3lpen", &self.usart3lpen())
                .field("uart4lpen", &self.uart4lpen())
                .field("uart5lpen", &self.uart5lpen())
                .field("i2c1lpen", &self.i2c1lpen())
                .field("i2c2lpen", &self.i2c2lpen())
                .field("i2c3lpen", &self.i2c3lpen())
                .field("can1lpen", &self.can1lpen())
                .field("can2lpen", &self.can2lpen())
                .field("pwrlpen", &self.pwrlpen())
                .field("daclpen", &self.daclpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1lpenr {{ tim2lpen: {=bool:?}, tim3lpen: {=bool:?}, tim4lpen: {=bool:?}, tim5lpen: {=bool:?}, tim6lpen: {=bool:?}, tim7lpen: {=bool:?}, tim12lpen: {=bool:?}, tim13lpen: {=bool:?}, tim14lpen: {=bool:?}, wwdglpen: {=bool:?}, spi2lpen: {=bool:?}, spi3lpen: {=bool:?}, usart2lpen: {=bool:?}, usart3lpen: {=bool:?}, uart4lpen: {=bool:?}, uart5lpen: {=bool:?}, i2c1lpen: {=bool:?}, i2c2lpen: {=bool:?}, i2c3lpen: {=bool:?}, can1lpen: {=bool:?}, can2lpen: {=bool:?}, pwrlpen: {=bool:?}, daclpen: {=bool:?} }}" , self . tim2lpen () , self . tim3lpen () , self . tim4lpen () , self . tim5lpen () , self . tim6lpen () , self . tim7lpen () , self . tim12lpen () , self . tim13lpen () , self . tim14lpen () , self . wwdglpen () , self . spi2lpen () , self . spi3lpen () , self . usart2lpen () , self . usart3lpen () , self . uart4lpen () , self . uart5lpen () , self . i2c1lpen () , self . i2c2lpen () , self . i2c3lpen () , self . can1lpen () , self . can2lpen () , self . pwrlpen () , self . daclpen ())
        }
    }
    #[doc = "APB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr(pub u32);
    impl Apb1rstr {
        #[doc = "TIM2 reset"]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 reset"]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 reset"]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 reset"]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 reset"]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 reset"]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 reset"]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 reset"]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 reset"]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 reset"]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 reset"]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 reset"]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 reset"]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 reset"]
        #[inline(always)]
        pub fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 reset"]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 reset"]
        #[inline(always)]
        pub fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 reset"]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 reset"]
        #[inline(always)]
        pub fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
        #[doc = "SPI 2 reset"]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 2 reset"]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI 3 reset"]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 3 reset"]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART 2 reset"]
        #[inline(always)]
        pub const fn uart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 reset"]
        #[inline(always)]
        pub fn set_uart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART 3 reset"]
        #[inline(always)]
        pub const fn uart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART 3 reset"]
        #[inline(always)]
        pub fn set_uart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART 4 reset"]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART 4 reset"]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART 5 reset"]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USART 5 reset"]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C 1 reset"]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 1 reset"]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C 2 reset"]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 2 reset"]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CAN1 reset"]
        #[inline(always)]
        pub const fn can1rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 reset"]
        #[inline(always)]
        pub fn set_can1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN2 reset"]
        #[inline(always)]
        pub const fn can2rst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 reset"]
        #[inline(always)]
        pub fn set_can2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
        #[doc = "DAC reset"]
        #[inline(always)]
        pub const fn dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC reset"]
        #[inline(always)]
        pub fn set_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("tim4rst", &self.tim4rst())
                .field("tim5rst", &self.tim5rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("tim12rst", &self.tim12rst())
                .field("tim13rst", &self.tim13rst())
                .field("tim14rst", &self.tim14rst())
                .field("wwdgrst", &self.wwdgrst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("uart2rst", &self.uart2rst())
                .field("uart3rst", &self.uart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("can1rst", &self.can1rst())
                .field("can2rst", &self.can2rst())
                .field("pwrrst", &self.pwrrst())
                .field("dacrst", &self.dacrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1rstr {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim4rst: {=bool:?}, tim5rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, tim12rst: {=bool:?}, tim13rst: {=bool:?}, tim14rst: {=bool:?}, wwdgrst: {=bool:?}, spi2rst: {=bool:?}, spi3rst: {=bool:?}, uart2rst: {=bool:?}, uart3rst: {=bool:?}, uart4rst: {=bool:?}, uart5rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, i2c3rst: {=bool:?}, can1rst: {=bool:?}, can2rst: {=bool:?}, pwrrst: {=bool:?}, dacrst: {=bool:?} }}" , self . tim2rst () , self . tim3rst () , self . tim4rst () , self . tim5rst () , self . tim6rst () , self . tim7rst () , self . tim12rst () , self . tim13rst () , self . tim14rst () , self . wwdgrst () , self . spi2rst () , self . spi3rst () , self . uart2rst () , self . uart3rst () , self . uart4rst () , self . uart5rst () , self . i2c1rst () , self . i2c2rst () , self . i2c3rst () , self . can1rst () , self . can2rst () , self . pwrrst () , self . dacrst ())
        }
    }
    #[doc = "APB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 clock enable"]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable"]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 clock enable"]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable"]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 clock enable"]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable"]
        #[inline(always)]
        pub fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC1 clock enable"]
        #[inline(always)]
        pub const fn adc1en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 clock enable"]
        #[inline(always)]
        pub fn set_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC2 clock enable"]
        #[inline(always)]
        pub const fn adc2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2 clock enable"]
        #[inline(always)]
        pub fn set_adc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC3 clock enable"]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 clock enable"]
        #[inline(always)]
        pub fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SDIO clock enable"]
        #[inline(always)]
        pub const fn sdioen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO clock enable"]
        #[inline(always)]
        pub fn set_sdioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
        #[doc = "System configuration controller clock enable"]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller clock enable"]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM9 clock enable"]
        #[inline(always)]
        pub const fn tim9en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 clock enable"]
        #[inline(always)]
        pub fn set_tim9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM10 clock enable"]
        #[inline(always)]
        pub const fn tim10en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 clock enable"]
        #[inline(always)]
        pub fn set_tim10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM11 clock enable"]
        #[inline(always)]
        pub const fn tim11en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 clock enable"]
        #[inline(always)]
        pub fn set_tim11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("usart6en", &self.usart6en())
                .field("adc1en", &self.adc1en())
                .field("adc2en", &self.adc2en())
                .field("adc3en", &self.adc3en())
                .field("sdioen", &self.sdioen())
                .field("spi1en", &self.spi1en())
                .field("syscfgen", &self.syscfgen())
                .field("tim9en", &self.tim9en())
                .field("tim10en", &self.tim10en())
                .field("tim11en", &self.tim11en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2enr {{ tim1en: {=bool:?}, tim8en: {=bool:?}, usart1en: {=bool:?}, usart6en: {=bool:?}, adc1en: {=bool:?}, adc2en: {=bool:?}, adc3en: {=bool:?}, sdioen: {=bool:?}, spi1en: {=bool:?}, syscfgen: {=bool:?}, tim9en: {=bool:?}, tim10en: {=bool:?}, tim11en: {=bool:?} }}" , self . tim1en () , self . tim8en () , self . usart1en () , self . usart6en () , self . adc1en () , self . adc2en () , self . adc3en () , self . sdioen () , self . spi1en () , self . syscfgen () , self . tim9en () , self . tim10en () , self . tim11en ())
        }
    }
    #[doc = "APB2 peripheral clock enabled in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn adc1lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_adc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn adc2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_adc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC 3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SDIO clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn sdiolpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_sdiolpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "System configuration controller clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM9 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tim9lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tim9lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM10 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim10lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim10lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM11 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim11lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim11lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Apb2lpenr {
        #[inline(always)]
        fn default() -> Apb2lpenr {
            Apb2lpenr(0)
        }
    }
    impl core::fmt::Debug for Apb2lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2lpenr")
                .field("tim1lpen", &self.tim1lpen())
                .field("tim8lpen", &self.tim8lpen())
                .field("usart1lpen", &self.usart1lpen())
                .field("usart6lpen", &self.usart6lpen())
                .field("adc1lpen", &self.adc1lpen())
                .field("adc2lpen", &self.adc2lpen())
                .field("adc3lpen", &self.adc3lpen())
                .field("sdiolpen", &self.sdiolpen())
                .field("spi1lpen", &self.spi1lpen())
                .field("syscfglpen", &self.syscfglpen())
                .field("tim9lpen", &self.tim9lpen())
                .field("tim10lpen", &self.tim10lpen())
                .field("tim11lpen", &self.tim11lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2lpenr {{ tim1lpen: {=bool:?}, tim8lpen: {=bool:?}, usart1lpen: {=bool:?}, usart6lpen: {=bool:?}, adc1lpen: {=bool:?}, adc2lpen: {=bool:?}, adc3lpen: {=bool:?}, sdiolpen: {=bool:?}, spi1lpen: {=bool:?}, syscfglpen: {=bool:?}, tim9lpen: {=bool:?}, tim10lpen: {=bool:?}, tim11lpen: {=bool:?} }}" , self . tim1lpen () , self . tim8lpen () , self . usart1lpen () , self . usart6lpen () , self . adc1lpen () , self . adc2lpen () , self . adc3lpen () , self . sdiolpen () , self . spi1lpen () , self . syscfglpen () , self . tim9lpen () , self . tim10lpen () , self . tim11lpen ())
        }
    }
    #[doc = "APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 reset"]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 reset"]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 reset"]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 reset"]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 reset"]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 reset"]
        #[inline(always)]
        pub fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC interface reset (common to all ADCs)"]
        #[inline(always)]
        pub const fn adcrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC interface reset (common to all ADCs)"]
        #[inline(always)]
        pub fn set_adcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SDIO reset"]
        #[inline(always)]
        pub const fn sdiorst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO reset"]
        #[inline(always)]
        pub fn set_sdiorst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
        #[doc = "System configuration controller reset"]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller reset"]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM9 reset"]
        #[inline(always)]
        pub const fn tim9rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 reset"]
        #[inline(always)]
        pub fn set_tim9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM10 reset"]
        #[inline(always)]
        pub const fn tim10rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 reset"]
        #[inline(always)]
        pub fn set_tim10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM11 reset"]
        #[inline(always)]
        pub const fn tim11rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 reset"]
        #[inline(always)]
        pub fn set_tim11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("tim8rst", &self.tim8rst())
                .field("usart1rst", &self.usart1rst())
                .field("usart6rst", &self.usart6rst())
                .field("adcrst", &self.adcrst())
                .field("sdiorst", &self.sdiorst())
                .field("spi1rst", &self.spi1rst())
                .field("syscfgrst", &self.syscfgrst())
                .field("tim9rst", &self.tim9rst())
                .field("tim10rst", &self.tim10rst())
                .field("tim11rst", &self.tim11rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2rstr {{ tim1rst: {=bool:?}, tim8rst: {=bool:?}, usart1rst: {=bool:?}, usart6rst: {=bool:?}, adcrst: {=bool:?}, sdiorst: {=bool:?}, spi1rst: {=bool:?}, syscfgrst: {=bool:?}, tim9rst: {=bool:?}, tim10rst: {=bool:?}, tim11rst: {=bool:?} }}" , self . tim1rst () , self . tim8rst () , self . usart1rst () , self . usart6rst () , self . adcrst () , self . sdiorst () , self . spi1rst () , self . syscfgrst () , self . tim9rst () , self . tim10rst () , self . tim11rst ())
        }
    }
    #[doc = "Backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "External low-speed oscillator enable"]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator enable"]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External low-speed oscillator ready"]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator ready"]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External low-speed oscillator bypass"]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator bypass"]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup domain software reset"]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset"]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("rtcsel", &self.rtcsel())
                .field("rtcen", &self.rtcen())
                .field("bdrst", &self.bdrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, rtcsel: {:?}, rtcen: {=bool:?}, bdrst: {=bool:?} }}" , self . lseon () , self . lserdy () , self . lsebyp () , self . rtcsel () , self . rtcen () , self . bdrst ())
        }
    }
    #[doc = "clock configuration register"]
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
        #[doc = "APB Low speed prescaler (APB1)"]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB Low speed prescaler (APB1)"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Microcontroller clock output 1"]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 1"]
        #[inline(always)]
        pub fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "I2S clock selection"]
        #[inline(always)]
        pub const fn i2ssrc(&self) -> super::vals::Issrc {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Issrc::from_bits(val as u8)
        }
        #[doc = "I2S clock selection"]
        #[inline(always)]
        pub fn set_i2ssrc(&mut self, val: super::vals::Issrc) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 27usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
        }
        #[doc = "Microcontroller clock output 2"]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 2"]
        #[inline(always)]
        pub fn set_mco2sel(&mut self, val: super::vals::Mco2sel) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
                .field("rtcpre", &self.rtcpre())
                .field("mco1sel", &self.mco1sel())
                .field("i2ssrc", &self.i2ssrc())
                .field("mco1pre", &self.mco1pre())
                .field("mco2pre", &self.mco2pre())
                .field("mco2sel", &self.mco2sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ sw: {:?}, sws: {:?}, hpre: {:?}, ppre1: {:?}, ppre2: {:?}, rtcpre: {=u8:?}, mco1sel: {:?}, i2ssrc: {:?}, mco1pre: {:?}, mco2pre: {:?}, mco2sel: {:?} }}" , self . sw () , self . sws () , self . hpre () , self . ppre1 () , self . ppre2 () , self . rtcpre () , self . mco1sel () , self . i2ssrc () , self . mco1pre () , self . mco2pre () , self . mco2sel ())
        }
    }
    #[doc = "clock interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cir(pub u32);
    impl Cir {
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
        #[doc = "Main PLL (PLL) ready interrupt flag"]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL (PLL) ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLLI2S ready interrupt flag"]
        #[inline(always)]
        pub const fn plli2srdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S ready interrupt flag"]
        #[inline(always)]
        pub fn set_plli2srdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Main PLL (PLL) ready interrupt enable"]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL (PLL) ready interrupt enable"]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PLLI2S ready interrupt enable"]
        #[inline(always)]
        pub const fn plli2srdyie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S ready interrupt enable"]
        #[inline(always)]
        pub fn set_plli2srdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Main PLL(PLL) ready interrupt clear"]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL(PLL) ready interrupt clear"]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PLLI2S ready interrupt clear"]
        #[inline(always)]
        pub const fn plli2srdyc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S ready interrupt clear"]
        #[inline(always)]
        pub fn set_plli2srdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Cir {
        #[inline(always)]
        fn default() -> Cir {
            Cir(0)
        }
    }
    impl core::fmt::Debug for Cir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cir")
                .field("lsirdyf", &self.lsirdyf())
                .field("lserdyf", &self.lserdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("pllrdyf", &self.pllrdyf())
                .field("plli2srdyf", &self.plli2srdyf())
                .field("cssf", &self.cssf())
                .field("lsirdyie", &self.lsirdyie())
                .field("lserdyie", &self.lserdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("pllrdyie", &self.pllrdyie())
                .field("plli2srdyie", &self.plli2srdyie())
                .field("lsirdyc", &self.lsirdyc())
                .field("lserdyc", &self.lserdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("pllrdyc", &self.pllrdyc())
                .field("plli2srdyc", &self.plli2srdyc())
                .field("cssc", &self.cssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cir {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cir {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, pllrdyf: {=bool:?}, plli2srdyf: {=bool:?}, cssf: {=bool:?}, lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, pllrdyie: {=bool:?}, plli2srdyie: {=bool:?}, lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, pllrdyc: {=bool:?}, plli2srdyc: {=bool:?}, cssc: {=bool:?} }}" , self . lsirdyf () , self . lserdyf () , self . hsirdyf () , self . hserdyf () , self . pllrdyf () , self . plli2srdyf () , self . cssf () , self . lsirdyie () , self . lserdyie () , self . hsirdyie () , self . hserdyie () , self . pllrdyie () , self . plli2srdyie () , self . lsirdyc () , self . lserdyc () , self . hsirdyc () , self . hserdyc () , self . pllrdyc () , self . plli2srdyc () , self . cssc ())
        }
    }
    #[doc = "clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Internal high-speed clock enable"]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed clock enable"]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal high-speed clock ready flag"]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed clock ready flag"]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal high-speed clock trimming"]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal high-speed clock trimming"]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal high-speed clock calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Internal high-speed clock calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
        #[doc = "Clock security system enable"]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system enable"]
        #[inline(always)]
        pub fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Main PLL (PLL) enable"]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL (PLL) enable"]
        #[inline(always)]
        pub fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Main PLL (PLL) clock ready flag"]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL (PLL) clock ready flag"]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PLLI2S enable"]
        #[inline(always)]
        pub const fn plli2son(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S enable"]
        #[inline(always)]
        pub fn set_plli2son(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PLLI2S clock ready flag"]
        #[inline(always)]
        pub const fn plli2srdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S clock ready flag"]
        #[inline(always)]
        pub fn set_plli2srdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("hsirdy", &self.hsirdy())
                .field("hsitrim", &self.hsitrim())
                .field("hsical", &self.hsical())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("csson", &self.csson())
                .field("pllon", &self.pllon())
                .field("pllrdy", &self.pllrdy())
                .field("plli2son", &self.plli2son())
                .field("plli2srdy", &self.plli2srdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ hsion: {=bool:?}, hsirdy: {=bool:?}, hsitrim: {=u8:?}, hsical: {=u8:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, csson: {=bool:?}, pllon: {=bool:?}, pllrdy: {=bool:?}, plli2son: {=bool:?}, plli2srdy: {=bool:?} }}" , self . hsion () , self . hsirdy () , self . hsitrim () , self . hsical () , self . hseon () , self . hserdy () , self . hsebyp () , self . csson () , self . pllon () , self . pllrdy () , self . plli2son () , self . plli2srdy ())
        }
    }
    #[doc = "clock control & status register"]
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
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub const fn padrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub fn set_padrstf(&mut self, val: bool) {
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
        pub const fn wdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag"]
        #[inline(always)]
        pub fn set_wdgrstf(&mut self, val: bool) {
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
                .field("rmvf", &self.rmvf())
                .field("borrstf", &self.borrstf())
                .field("padrstf", &self.padrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("wdgrstf", &self.wdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ lsion: {=bool:?}, lsirdy: {=bool:?}, rmvf: {=bool:?}, borrstf: {=bool:?}, padrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, wdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}" , self . lsion () , self . lsirdy () , self . rmvf () , self . borrstf () , self . padrstf () , self . porrstf () , self . sftrstf () , self . wdgrstf () , self . wwdgrstf () , self . lpwrrstf ())
        }
    }
    #[doc = "PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 0usize) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
        #[inline(always)]
        pub fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
        }
        #[doc = "Main PLL (PLL) multiplication factor for VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 6usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Main PLL (PLL) multiplication factor for VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 6usize)) | (((val.to_bits() as u32) & 0x01ff) << 6usize);
        }
        #[doc = "Main PLL (PLL) division factor for main system clock"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Pllp {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "Main PLL (PLL) division factor for main system clock"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Pllp) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Pllq {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Pllq) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Pllcfgr {
        #[inline(always)]
        fn default() -> Pllcfgr {
            Pllcfgr(0)
        }
    }
    impl core::fmt::Debug for Pllcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllcfgr")
                .field("pllm", &self.pllm())
                .field("plln", &self.plln())
                .field("pllp", &self.pllp())
                .field("pllsrc", &self.pllsrc())
                .field("pllq", &self.pllq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pllcfgr {{ pllm: {:?}, plln: {:?}, pllp: {:?}, pllsrc: {:?}, pllq: {:?} }}",
                self.pllm(),
                self.plln(),
                self.pllp(),
                self.pllsrc(),
                self.pllq()
            )
        }
    }
    #[doc = "PLLI2S configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plli2scfgr(pub u32);
    impl Plli2scfgr {
        #[doc = "PLLI2S multiplication factor for VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 6usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "PLLI2S multiplication factor for VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 6usize)) | (((val.to_bits() as u32) & 0x01ff) << 6usize);
        }
        #[doc = "PLLI2S division factor for I2S clocks"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Pllr {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "PLLI2S division factor for I2S clocks"]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Pllr) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Plli2scfgr {
        #[inline(always)]
        fn default() -> Plli2scfgr {
            Plli2scfgr(0)
        }
    }
    impl core::fmt::Debug for Plli2scfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Plli2scfgr")
                .field("plln", &self.plln())
                .field("pllr", &self.pllr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Plli2scfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Plli2scfgr {{ plln: {:?}, pllr: {:?} }}", self.plln(), self.pllr())
        }
    }
    #[doc = "spread spectrum clock generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sscgr(pub u32);
    impl Sscgr {
        #[doc = "Modulation period"]
        #[inline(always)]
        pub const fn modper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Modulation period"]
        #[inline(always)]
        pub fn set_modper(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Incrementation step"]
        #[inline(always)]
        pub const fn incstep(&self) -> u16 {
            let val = (self.0 >> 13usize) & 0x7fff;
            val as u16
        }
        #[doc = "Incrementation step"]
        #[inline(always)]
        pub fn set_incstep(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 13usize)) | (((val as u32) & 0x7fff) << 13usize);
        }
        #[doc = "Spread Select"]
        #[inline(always)]
        pub const fn spreadsel(&self) -> super::vals::Spreadsel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Spreadsel::from_bits(val as u8)
        }
        #[doc = "Spread Select"]
        #[inline(always)]
        pub fn set_spreadsel(&mut self, val: super::vals::Spreadsel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Spread spectrum modulation enable"]
        #[inline(always)]
        pub const fn sscgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Spread spectrum modulation enable"]
        #[inline(always)]
        pub fn set_sscgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sscgr {
        #[inline(always)]
        fn default() -> Sscgr {
            Sscgr(0)
        }
    }
    impl core::fmt::Debug for Sscgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sscgr")
                .field("modper", &self.modper())
                .field("incstep", &self.incstep())
                .field("spreadsel", &self.spreadsel())
                .field("sscgen", &self.sscgen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sscgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sscgr {{ modper: {=u16:?}, incstep: {=u16:?}, spreadsel: {:?}, sscgen: {=bool:?} }}",
                self.modper(),
                self.incstep(),
                self.spreadsel(),
                self.sscgen()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "SYSCLK not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "SYSCLK divided by 2"]
        DIV2 = 0x08,
        #[doc = "SYSCLK divided by 4"]
        DIV4 = 0x09,
        #[doc = "SYSCLK divided by 8"]
        DIV8 = 0x0a,
        #[doc = "SYSCLK divided by 16"]
        DIV16 = 0x0b,
        #[doc = "SYSCLK divided by 64"]
        DIV64 = 0x0c,
        #[doc = "SYSCLK divided by 128"]
        DIV128 = 0x0d,
        #[doc = "SYSCLK divided by 256"]
        DIV256 = 0x0e,
        #[doc = "SYSCLK divided by 512"]
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
    pub enum Issrc {
        #[doc = "PLLI2S clock used as I2S clock source"]
        PLLI2S = 0x0,
        #[doc = "External clock mapped on the I2S_CKIN pin used as I2S clock source"]
        CKIN = 0x01,
    }
    impl Issrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Issrc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Issrc {
        #[inline(always)]
        fn from(val: u8) -> Issrc {
            Issrc::from_bits(val)
        }
    }
    impl From<Issrc> for u8 {
        #[inline(always)]
        fn from(val: Issrc) -> u8 {
            Issrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mco1sel {
        #[doc = "HSI clock selected"]
        HSI = 0x0,
        #[doc = "LSE oscillator selected"]
        LSE = 0x01,
        #[doc = "HSE oscillator clock selected"]
        HSE = 0x02,
        #[doc = "PLL clock selected"]
        PLL = 0x03,
    }
    impl Mco1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco1sel {
        #[inline(always)]
        fn from(val: u8) -> Mco1sel {
            Mco1sel::from_bits(val)
        }
    }
    impl From<Mco1sel> for u8 {
        #[inline(always)]
        fn from(val: Mco1sel) -> u8 {
            Mco1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mco2sel {
        #[doc = "System clock (SYSCLK) selected"]
        SYS = 0x0,
        #[doc = "PLLI2S clock selected"]
        PLLI2S = 0x01,
        #[doc = "HSE oscillator clock selected"]
        HSE = 0x02,
        #[doc = "PLL clock selected"]
        PLL = 0x03,
    }
    impl Mco2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco2sel {
            unsafe { core::mem::transmute(val & 0x03) }
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
        #[doc = "No division"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Division by 2"]
        DIV2 = 0x04,
        #[doc = "Division by 3"]
        DIV3 = 0x05,
        #[doc = "Division by 4"]
        DIV4 = 0x06,
        #[doc = "Division by 5"]
        DIV5 = 0x07,
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
    pub enum Pllm {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        DIV2 = 0x02,
        DIV3 = 0x03,
        DIV4 = 0x04,
        DIV5 = 0x05,
        DIV6 = 0x06,
        DIV7 = 0x07,
        DIV8 = 0x08,
        DIV9 = 0x09,
        DIV10 = 0x0a,
        DIV11 = 0x0b,
        DIV12 = 0x0c,
        DIV13 = 0x0d,
        DIV14 = 0x0e,
        DIV15 = 0x0f,
        DIV16 = 0x10,
        DIV17 = 0x11,
        DIV18 = 0x12,
        DIV19 = 0x13,
        DIV20 = 0x14,
        DIV21 = 0x15,
        DIV22 = 0x16,
        DIV23 = 0x17,
        DIV24 = 0x18,
        DIV25 = 0x19,
        DIV26 = 0x1a,
        DIV27 = 0x1b,
        DIV28 = 0x1c,
        DIV29 = 0x1d,
        DIV30 = 0x1e,
        DIV31 = 0x1f,
        DIV32 = 0x20,
        DIV33 = 0x21,
        DIV34 = 0x22,
        DIV35 = 0x23,
        DIV36 = 0x24,
        DIV37 = 0x25,
        DIV38 = 0x26,
        DIV39 = 0x27,
        DIV40 = 0x28,
        DIV41 = 0x29,
        DIV42 = 0x2a,
        DIV43 = 0x2b,
        DIV44 = 0x2c,
        DIV45 = 0x2d,
        DIV46 = 0x2e,
        DIV47 = 0x2f,
        DIV48 = 0x30,
        DIV49 = 0x31,
        DIV50 = 0x32,
        DIV51 = 0x33,
        DIV52 = 0x34,
        DIV53 = 0x35,
        DIV54 = 0x36,
        DIV55 = 0x37,
        DIV56 = 0x38,
        DIV57 = 0x39,
        DIV58 = 0x3a,
        DIV59 = 0x3b,
        DIV60 = 0x3c,
        DIV61 = 0x3d,
        DIV62 = 0x3e,
        DIV63 = 0x3f,
    }
    impl Pllm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllm {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllm {
        #[inline(always)]
        fn from(val: u8) -> Pllm {
            Pllm::from_bits(val)
        }
    }
    impl From<Pllm> for u8 {
        #[inline(always)]
        fn from(val: Pllm) -> u8 {
            Pllm::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plln(u16);
    impl Plln {
        pub const MUL192: Self = Self(0xc0);
        pub const MUL193: Self = Self(0xc1);
        pub const MUL194: Self = Self(0xc2);
        pub const MUL195: Self = Self(0xc3);
        pub const MUL196: Self = Self(0xc4);
        pub const MUL197: Self = Self(0xc5);
        pub const MUL198: Self = Self(0xc6);
        pub const MUL199: Self = Self(0xc7);
        pub const MUL200: Self = Self(0xc8);
        pub const MUL201: Self = Self(0xc9);
        pub const MUL202: Self = Self(0xca);
        pub const MUL203: Self = Self(0xcb);
        pub const MUL204: Self = Self(0xcc);
        pub const MUL205: Self = Self(0xcd);
        pub const MUL206: Self = Self(0xce);
        pub const MUL207: Self = Self(0xcf);
        pub const MUL208: Self = Self(0xd0);
        pub const MUL209: Self = Self(0xd1);
        pub const MUL210: Self = Self(0xd2);
        pub const MUL211: Self = Self(0xd3);
        pub const MUL212: Self = Self(0xd4);
        pub const MUL213: Self = Self(0xd5);
        pub const MUL214: Self = Self(0xd6);
        pub const MUL215: Self = Self(0xd7);
        pub const MUL216: Self = Self(0xd8);
        pub const MUL217: Self = Self(0xd9);
        pub const MUL218: Self = Self(0xda);
        pub const MUL219: Self = Self(0xdb);
        pub const MUL220: Self = Self(0xdc);
        pub const MUL221: Self = Self(0xdd);
        pub const MUL222: Self = Self(0xde);
        pub const MUL223: Self = Self(0xdf);
        pub const MUL224: Self = Self(0xe0);
        pub const MUL225: Self = Self(0xe1);
        pub const MUL226: Self = Self(0xe2);
        pub const MUL227: Self = Self(0xe3);
        pub const MUL228: Self = Self(0xe4);
        pub const MUL229: Self = Self(0xe5);
        pub const MUL230: Self = Self(0xe6);
        pub const MUL231: Self = Self(0xe7);
        pub const MUL232: Self = Self(0xe8);
        pub const MUL233: Self = Self(0xe9);
        pub const MUL234: Self = Self(0xea);
        pub const MUL235: Self = Self(0xeb);
        pub const MUL236: Self = Self(0xec);
        pub const MUL237: Self = Self(0xed);
        pub const MUL238: Self = Self(0xee);
        pub const MUL239: Self = Self(0xef);
        pub const MUL240: Self = Self(0xf0);
        pub const MUL241: Self = Self(0xf1);
        pub const MUL242: Self = Self(0xf2);
        pub const MUL243: Self = Self(0xf3);
        pub const MUL244: Self = Self(0xf4);
        pub const MUL245: Self = Self(0xf5);
        pub const MUL246: Self = Self(0xf6);
        pub const MUL247: Self = Self(0xf7);
        pub const MUL248: Self = Self(0xf8);
        pub const MUL249: Self = Self(0xf9);
        pub const MUL250: Self = Self(0xfa);
        pub const MUL251: Self = Self(0xfb);
        pub const MUL252: Self = Self(0xfc);
        pub const MUL253: Self = Self(0xfd);
        pub const MUL254: Self = Self(0xfe);
        pub const MUL255: Self = Self(0xff);
        pub const MUL256: Self = Self(0x0100);
        pub const MUL257: Self = Self(0x0101);
        pub const MUL258: Self = Self(0x0102);
        pub const MUL259: Self = Self(0x0103);
        pub const MUL260: Self = Self(0x0104);
        pub const MUL261: Self = Self(0x0105);
        pub const MUL262: Self = Self(0x0106);
        pub const MUL263: Self = Self(0x0107);
        pub const MUL264: Self = Self(0x0108);
        pub const MUL265: Self = Self(0x0109);
        pub const MUL266: Self = Self(0x010a);
        pub const MUL267: Self = Self(0x010b);
        pub const MUL268: Self = Self(0x010c);
        pub const MUL269: Self = Self(0x010d);
        pub const MUL270: Self = Self(0x010e);
        pub const MUL271: Self = Self(0x010f);
        pub const MUL272: Self = Self(0x0110);
        pub const MUL273: Self = Self(0x0111);
        pub const MUL274: Self = Self(0x0112);
        pub const MUL275: Self = Self(0x0113);
        pub const MUL276: Self = Self(0x0114);
        pub const MUL277: Self = Self(0x0115);
        pub const MUL278: Self = Self(0x0116);
        pub const MUL279: Self = Self(0x0117);
        pub const MUL280: Self = Self(0x0118);
        pub const MUL281: Self = Self(0x0119);
        pub const MUL282: Self = Self(0x011a);
        pub const MUL283: Self = Self(0x011b);
        pub const MUL284: Self = Self(0x011c);
        pub const MUL285: Self = Self(0x011d);
        pub const MUL286: Self = Self(0x011e);
        pub const MUL287: Self = Self(0x011f);
        pub const MUL288: Self = Self(0x0120);
        pub const MUL289: Self = Self(0x0121);
        pub const MUL290: Self = Self(0x0122);
        pub const MUL291: Self = Self(0x0123);
        pub const MUL292: Self = Self(0x0124);
        pub const MUL293: Self = Self(0x0125);
        pub const MUL294: Self = Self(0x0126);
        pub const MUL295: Self = Self(0x0127);
        pub const MUL296: Self = Self(0x0128);
        pub const MUL297: Self = Self(0x0129);
        pub const MUL298: Self = Self(0x012a);
        pub const MUL299: Self = Self(0x012b);
        pub const MUL300: Self = Self(0x012c);
        pub const MUL301: Self = Self(0x012d);
        pub const MUL302: Self = Self(0x012e);
        pub const MUL303: Self = Self(0x012f);
        pub const MUL304: Self = Self(0x0130);
        pub const MUL305: Self = Self(0x0131);
        pub const MUL306: Self = Self(0x0132);
        pub const MUL307: Self = Self(0x0133);
        pub const MUL308: Self = Self(0x0134);
        pub const MUL309: Self = Self(0x0135);
        pub const MUL310: Self = Self(0x0136);
        pub const MUL311: Self = Self(0x0137);
        pub const MUL312: Self = Self(0x0138);
        pub const MUL313: Self = Self(0x0139);
        pub const MUL314: Self = Self(0x013a);
        pub const MUL315: Self = Self(0x013b);
        pub const MUL316: Self = Self(0x013c);
        pub const MUL317: Self = Self(0x013d);
        pub const MUL318: Self = Self(0x013e);
        pub const MUL319: Self = Self(0x013f);
        pub const MUL320: Self = Self(0x0140);
        pub const MUL321: Self = Self(0x0141);
        pub const MUL322: Self = Self(0x0142);
        pub const MUL323: Self = Self(0x0143);
        pub const MUL324: Self = Self(0x0144);
        pub const MUL325: Self = Self(0x0145);
        pub const MUL326: Self = Self(0x0146);
        pub const MUL327: Self = Self(0x0147);
        pub const MUL328: Self = Self(0x0148);
        pub const MUL329: Self = Self(0x0149);
        pub const MUL330: Self = Self(0x014a);
        pub const MUL331: Self = Self(0x014b);
        pub const MUL332: Self = Self(0x014c);
        pub const MUL333: Self = Self(0x014d);
        pub const MUL334: Self = Self(0x014e);
        pub const MUL335: Self = Self(0x014f);
        pub const MUL336: Self = Self(0x0150);
        pub const MUL337: Self = Self(0x0151);
        pub const MUL338: Self = Self(0x0152);
        pub const MUL339: Self = Self(0x0153);
        pub const MUL340: Self = Self(0x0154);
        pub const MUL341: Self = Self(0x0155);
        pub const MUL342: Self = Self(0x0156);
        pub const MUL343: Self = Self(0x0157);
        pub const MUL344: Self = Self(0x0158);
        pub const MUL345: Self = Self(0x0159);
        pub const MUL346: Self = Self(0x015a);
        pub const MUL347: Self = Self(0x015b);
        pub const MUL348: Self = Self(0x015c);
        pub const MUL349: Self = Self(0x015d);
        pub const MUL350: Self = Self(0x015e);
        pub const MUL351: Self = Self(0x015f);
        pub const MUL352: Self = Self(0x0160);
        pub const MUL353: Self = Self(0x0161);
        pub const MUL354: Self = Self(0x0162);
        pub const MUL355: Self = Self(0x0163);
        pub const MUL356: Self = Self(0x0164);
        pub const MUL357: Self = Self(0x0165);
        pub const MUL358: Self = Self(0x0166);
        pub const MUL359: Self = Self(0x0167);
        pub const MUL360: Self = Self(0x0168);
        pub const MUL361: Self = Self(0x0169);
        pub const MUL362: Self = Self(0x016a);
        pub const MUL363: Self = Self(0x016b);
        pub const MUL364: Self = Self(0x016c);
        pub const MUL365: Self = Self(0x016d);
        pub const MUL366: Self = Self(0x016e);
        pub const MUL367: Self = Self(0x016f);
        pub const MUL368: Self = Self(0x0170);
        pub const MUL369: Self = Self(0x0171);
        pub const MUL370: Self = Self(0x0172);
        pub const MUL371: Self = Self(0x0173);
        pub const MUL372: Self = Self(0x0174);
        pub const MUL373: Self = Self(0x0175);
        pub const MUL374: Self = Self(0x0176);
        pub const MUL375: Self = Self(0x0177);
        pub const MUL376: Self = Self(0x0178);
        pub const MUL377: Self = Self(0x0179);
        pub const MUL378: Self = Self(0x017a);
        pub const MUL379: Self = Self(0x017b);
        pub const MUL380: Self = Self(0x017c);
        pub const MUL381: Self = Self(0x017d);
        pub const MUL382: Self = Self(0x017e);
        pub const MUL383: Self = Self(0x017f);
        pub const MUL384: Self = Self(0x0180);
        pub const MUL385: Self = Self(0x0181);
        pub const MUL386: Self = Self(0x0182);
        pub const MUL387: Self = Self(0x0183);
        pub const MUL388: Self = Self(0x0184);
        pub const MUL389: Self = Self(0x0185);
        pub const MUL390: Self = Self(0x0186);
        pub const MUL391: Self = Self(0x0187);
        pub const MUL392: Self = Self(0x0188);
        pub const MUL393: Self = Self(0x0189);
        pub const MUL394: Self = Self(0x018a);
        pub const MUL395: Self = Self(0x018b);
        pub const MUL396: Self = Self(0x018c);
        pub const MUL397: Self = Self(0x018d);
        pub const MUL398: Self = Self(0x018e);
        pub const MUL399: Self = Self(0x018f);
        pub const MUL400: Self = Self(0x0190);
        pub const MUL401: Self = Self(0x0191);
        pub const MUL402: Self = Self(0x0192);
        pub const MUL403: Self = Self(0x0193);
        pub const MUL404: Self = Self(0x0194);
        pub const MUL405: Self = Self(0x0195);
        pub const MUL406: Self = Self(0x0196);
        pub const MUL407: Self = Self(0x0197);
        pub const MUL408: Self = Self(0x0198);
        pub const MUL409: Self = Self(0x0199);
        pub const MUL410: Self = Self(0x019a);
        pub const MUL411: Self = Self(0x019b);
        pub const MUL412: Self = Self(0x019c);
        pub const MUL413: Self = Self(0x019d);
        pub const MUL414: Self = Self(0x019e);
        pub const MUL415: Self = Self(0x019f);
        pub const MUL416: Self = Self(0x01a0);
        pub const MUL417: Self = Self(0x01a1);
        pub const MUL418: Self = Self(0x01a2);
        pub const MUL419: Self = Self(0x01a3);
        pub const MUL420: Self = Self(0x01a4);
        pub const MUL421: Self = Self(0x01a5);
        pub const MUL422: Self = Self(0x01a6);
        pub const MUL423: Self = Self(0x01a7);
        pub const MUL424: Self = Self(0x01a8);
        pub const MUL425: Self = Self(0x01a9);
        pub const MUL426: Self = Self(0x01aa);
        pub const MUL427: Self = Self(0x01ab);
        pub const MUL428: Self = Self(0x01ac);
        pub const MUL429: Self = Self(0x01ad);
        pub const MUL430: Self = Self(0x01ae);
        pub const MUL431: Self = Self(0x01af);
        pub const MUL432: Self = Self(0x01b0);
    }
    impl Plln {
        pub const fn from_bits(val: u16) -> Plln {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Plln {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xc0 => f.write_str("MUL192"),
                0xc1 => f.write_str("MUL193"),
                0xc2 => f.write_str("MUL194"),
                0xc3 => f.write_str("MUL195"),
                0xc4 => f.write_str("MUL196"),
                0xc5 => f.write_str("MUL197"),
                0xc6 => f.write_str("MUL198"),
                0xc7 => f.write_str("MUL199"),
                0xc8 => f.write_str("MUL200"),
                0xc9 => f.write_str("MUL201"),
                0xca => f.write_str("MUL202"),
                0xcb => f.write_str("MUL203"),
                0xcc => f.write_str("MUL204"),
                0xcd => f.write_str("MUL205"),
                0xce => f.write_str("MUL206"),
                0xcf => f.write_str("MUL207"),
                0xd0 => f.write_str("MUL208"),
                0xd1 => f.write_str("MUL209"),
                0xd2 => f.write_str("MUL210"),
                0xd3 => f.write_str("MUL211"),
                0xd4 => f.write_str("MUL212"),
                0xd5 => f.write_str("MUL213"),
                0xd6 => f.write_str("MUL214"),
                0xd7 => f.write_str("MUL215"),
                0xd8 => f.write_str("MUL216"),
                0xd9 => f.write_str("MUL217"),
                0xda => f.write_str("MUL218"),
                0xdb => f.write_str("MUL219"),
                0xdc => f.write_str("MUL220"),
                0xdd => f.write_str("MUL221"),
                0xde => f.write_str("MUL222"),
                0xdf => f.write_str("MUL223"),
                0xe0 => f.write_str("MUL224"),
                0xe1 => f.write_str("MUL225"),
                0xe2 => f.write_str("MUL226"),
                0xe3 => f.write_str("MUL227"),
                0xe4 => f.write_str("MUL228"),
                0xe5 => f.write_str("MUL229"),
                0xe6 => f.write_str("MUL230"),
                0xe7 => f.write_str("MUL231"),
                0xe8 => f.write_str("MUL232"),
                0xe9 => f.write_str("MUL233"),
                0xea => f.write_str("MUL234"),
                0xeb => f.write_str("MUL235"),
                0xec => f.write_str("MUL236"),
                0xed => f.write_str("MUL237"),
                0xee => f.write_str("MUL238"),
                0xef => f.write_str("MUL239"),
                0xf0 => f.write_str("MUL240"),
                0xf1 => f.write_str("MUL241"),
                0xf2 => f.write_str("MUL242"),
                0xf3 => f.write_str("MUL243"),
                0xf4 => f.write_str("MUL244"),
                0xf5 => f.write_str("MUL245"),
                0xf6 => f.write_str("MUL246"),
                0xf7 => f.write_str("MUL247"),
                0xf8 => f.write_str("MUL248"),
                0xf9 => f.write_str("MUL249"),
                0xfa => f.write_str("MUL250"),
                0xfb => f.write_str("MUL251"),
                0xfc => f.write_str("MUL252"),
                0xfd => f.write_str("MUL253"),
                0xfe => f.write_str("MUL254"),
                0xff => f.write_str("MUL255"),
                0x0100 => f.write_str("MUL256"),
                0x0101 => f.write_str("MUL257"),
                0x0102 => f.write_str("MUL258"),
                0x0103 => f.write_str("MUL259"),
                0x0104 => f.write_str("MUL260"),
                0x0105 => f.write_str("MUL261"),
                0x0106 => f.write_str("MUL262"),
                0x0107 => f.write_str("MUL263"),
                0x0108 => f.write_str("MUL264"),
                0x0109 => f.write_str("MUL265"),
                0x010a => f.write_str("MUL266"),
                0x010b => f.write_str("MUL267"),
                0x010c => f.write_str("MUL268"),
                0x010d => f.write_str("MUL269"),
                0x010e => f.write_str("MUL270"),
                0x010f => f.write_str("MUL271"),
                0x0110 => f.write_str("MUL272"),
                0x0111 => f.write_str("MUL273"),
                0x0112 => f.write_str("MUL274"),
                0x0113 => f.write_str("MUL275"),
                0x0114 => f.write_str("MUL276"),
                0x0115 => f.write_str("MUL277"),
                0x0116 => f.write_str("MUL278"),
                0x0117 => f.write_str("MUL279"),
                0x0118 => f.write_str("MUL280"),
                0x0119 => f.write_str("MUL281"),
                0x011a => f.write_str("MUL282"),
                0x011b => f.write_str("MUL283"),
                0x011c => f.write_str("MUL284"),
                0x011d => f.write_str("MUL285"),
                0x011e => f.write_str("MUL286"),
                0x011f => f.write_str("MUL287"),
                0x0120 => f.write_str("MUL288"),
                0x0121 => f.write_str("MUL289"),
                0x0122 => f.write_str("MUL290"),
                0x0123 => f.write_str("MUL291"),
                0x0124 => f.write_str("MUL292"),
                0x0125 => f.write_str("MUL293"),
                0x0126 => f.write_str("MUL294"),
                0x0127 => f.write_str("MUL295"),
                0x0128 => f.write_str("MUL296"),
                0x0129 => f.write_str("MUL297"),
                0x012a => f.write_str("MUL298"),
                0x012b => f.write_str("MUL299"),
                0x012c => f.write_str("MUL300"),
                0x012d => f.write_str("MUL301"),
                0x012e => f.write_str("MUL302"),
                0x012f => f.write_str("MUL303"),
                0x0130 => f.write_str("MUL304"),
                0x0131 => f.write_str("MUL305"),
                0x0132 => f.write_str("MUL306"),
                0x0133 => f.write_str("MUL307"),
                0x0134 => f.write_str("MUL308"),
                0x0135 => f.write_str("MUL309"),
                0x0136 => f.write_str("MUL310"),
                0x0137 => f.write_str("MUL311"),
                0x0138 => f.write_str("MUL312"),
                0x0139 => f.write_str("MUL313"),
                0x013a => f.write_str("MUL314"),
                0x013b => f.write_str("MUL315"),
                0x013c => f.write_str("MUL316"),
                0x013d => f.write_str("MUL317"),
                0x013e => f.write_str("MUL318"),
                0x013f => f.write_str("MUL319"),
                0x0140 => f.write_str("MUL320"),
                0x0141 => f.write_str("MUL321"),
                0x0142 => f.write_str("MUL322"),
                0x0143 => f.write_str("MUL323"),
                0x0144 => f.write_str("MUL324"),
                0x0145 => f.write_str("MUL325"),
                0x0146 => f.write_str("MUL326"),
                0x0147 => f.write_str("MUL327"),
                0x0148 => f.write_str("MUL328"),
                0x0149 => f.write_str("MUL329"),
                0x014a => f.write_str("MUL330"),
                0x014b => f.write_str("MUL331"),
                0x014c => f.write_str("MUL332"),
                0x014d => f.write_str("MUL333"),
                0x014e => f.write_str("MUL334"),
                0x014f => f.write_str("MUL335"),
                0x0150 => f.write_str("MUL336"),
                0x0151 => f.write_str("MUL337"),
                0x0152 => f.write_str("MUL338"),
                0x0153 => f.write_str("MUL339"),
                0x0154 => f.write_str("MUL340"),
                0x0155 => f.write_str("MUL341"),
                0x0156 => f.write_str("MUL342"),
                0x0157 => f.write_str("MUL343"),
                0x0158 => f.write_str("MUL344"),
                0x0159 => f.write_str("MUL345"),
                0x015a => f.write_str("MUL346"),
                0x015b => f.write_str("MUL347"),
                0x015c => f.write_str("MUL348"),
                0x015d => f.write_str("MUL349"),
                0x015e => f.write_str("MUL350"),
                0x015f => f.write_str("MUL351"),
                0x0160 => f.write_str("MUL352"),
                0x0161 => f.write_str("MUL353"),
                0x0162 => f.write_str("MUL354"),
                0x0163 => f.write_str("MUL355"),
                0x0164 => f.write_str("MUL356"),
                0x0165 => f.write_str("MUL357"),
                0x0166 => f.write_str("MUL358"),
                0x0167 => f.write_str("MUL359"),
                0x0168 => f.write_str("MUL360"),
                0x0169 => f.write_str("MUL361"),
                0x016a => f.write_str("MUL362"),
                0x016b => f.write_str("MUL363"),
                0x016c => f.write_str("MUL364"),
                0x016d => f.write_str("MUL365"),
                0x016e => f.write_str("MUL366"),
                0x016f => f.write_str("MUL367"),
                0x0170 => f.write_str("MUL368"),
                0x0171 => f.write_str("MUL369"),
                0x0172 => f.write_str("MUL370"),
                0x0173 => f.write_str("MUL371"),
                0x0174 => f.write_str("MUL372"),
                0x0175 => f.write_str("MUL373"),
                0x0176 => f.write_str("MUL374"),
                0x0177 => f.write_str("MUL375"),
                0x0178 => f.write_str("MUL376"),
                0x0179 => f.write_str("MUL377"),
                0x017a => f.write_str("MUL378"),
                0x017b => f.write_str("MUL379"),
                0x017c => f.write_str("MUL380"),
                0x017d => f.write_str("MUL381"),
                0x017e => f.write_str("MUL382"),
                0x017f => f.write_str("MUL383"),
                0x0180 => f.write_str("MUL384"),
                0x0181 => f.write_str("MUL385"),
                0x0182 => f.write_str("MUL386"),
                0x0183 => f.write_str("MUL387"),
                0x0184 => f.write_str("MUL388"),
                0x0185 => f.write_str("MUL389"),
                0x0186 => f.write_str("MUL390"),
                0x0187 => f.write_str("MUL391"),
                0x0188 => f.write_str("MUL392"),
                0x0189 => f.write_str("MUL393"),
                0x018a => f.write_str("MUL394"),
                0x018b => f.write_str("MUL395"),
                0x018c => f.write_str("MUL396"),
                0x018d => f.write_str("MUL397"),
                0x018e => f.write_str("MUL398"),
                0x018f => f.write_str("MUL399"),
                0x0190 => f.write_str("MUL400"),
                0x0191 => f.write_str("MUL401"),
                0x0192 => f.write_str("MUL402"),
                0x0193 => f.write_str("MUL403"),
                0x0194 => f.write_str("MUL404"),
                0x0195 => f.write_str("MUL405"),
                0x0196 => f.write_str("MUL406"),
                0x0197 => f.write_str("MUL407"),
                0x0198 => f.write_str("MUL408"),
                0x0199 => f.write_str("MUL409"),
                0x019a => f.write_str("MUL410"),
                0x019b => f.write_str("MUL411"),
                0x019c => f.write_str("MUL412"),
                0x019d => f.write_str("MUL413"),
                0x019e => f.write_str("MUL414"),
                0x019f => f.write_str("MUL415"),
                0x01a0 => f.write_str("MUL416"),
                0x01a1 => f.write_str("MUL417"),
                0x01a2 => f.write_str("MUL418"),
                0x01a3 => f.write_str("MUL419"),
                0x01a4 => f.write_str("MUL420"),
                0x01a5 => f.write_str("MUL421"),
                0x01a6 => f.write_str("MUL422"),
                0x01a7 => f.write_str("MUL423"),
                0x01a8 => f.write_str("MUL424"),
                0x01a9 => f.write_str("MUL425"),
                0x01aa => f.write_str("MUL426"),
                0x01ab => f.write_str("MUL427"),
                0x01ac => f.write_str("MUL428"),
                0x01ad => f.write_str("MUL429"),
                0x01ae => f.write_str("MUL430"),
                0x01af => f.write_str("MUL431"),
                0x01b0 => f.write_str("MUL432"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Plln {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xc0 => defmt::write!(f, "MUL192"),
                0xc1 => defmt::write!(f, "MUL193"),
                0xc2 => defmt::write!(f, "MUL194"),
                0xc3 => defmt::write!(f, "MUL195"),
                0xc4 => defmt::write!(f, "MUL196"),
                0xc5 => defmt::write!(f, "MUL197"),
                0xc6 => defmt::write!(f, "MUL198"),
                0xc7 => defmt::write!(f, "MUL199"),
                0xc8 => defmt::write!(f, "MUL200"),
                0xc9 => defmt::write!(f, "MUL201"),
                0xca => defmt::write!(f, "MUL202"),
                0xcb => defmt::write!(f, "MUL203"),
                0xcc => defmt::write!(f, "MUL204"),
                0xcd => defmt::write!(f, "MUL205"),
                0xce => defmt::write!(f, "MUL206"),
                0xcf => defmt::write!(f, "MUL207"),
                0xd0 => defmt::write!(f, "MUL208"),
                0xd1 => defmt::write!(f, "MUL209"),
                0xd2 => defmt::write!(f, "MUL210"),
                0xd3 => defmt::write!(f, "MUL211"),
                0xd4 => defmt::write!(f, "MUL212"),
                0xd5 => defmt::write!(f, "MUL213"),
                0xd6 => defmt::write!(f, "MUL214"),
                0xd7 => defmt::write!(f, "MUL215"),
                0xd8 => defmt::write!(f, "MUL216"),
                0xd9 => defmt::write!(f, "MUL217"),
                0xda => defmt::write!(f, "MUL218"),
                0xdb => defmt::write!(f, "MUL219"),
                0xdc => defmt::write!(f, "MUL220"),
                0xdd => defmt::write!(f, "MUL221"),
                0xde => defmt::write!(f, "MUL222"),
                0xdf => defmt::write!(f, "MUL223"),
                0xe0 => defmt::write!(f, "MUL224"),
                0xe1 => defmt::write!(f, "MUL225"),
                0xe2 => defmt::write!(f, "MUL226"),
                0xe3 => defmt::write!(f, "MUL227"),
                0xe4 => defmt::write!(f, "MUL228"),
                0xe5 => defmt::write!(f, "MUL229"),
                0xe6 => defmt::write!(f, "MUL230"),
                0xe7 => defmt::write!(f, "MUL231"),
                0xe8 => defmt::write!(f, "MUL232"),
                0xe9 => defmt::write!(f, "MUL233"),
                0xea => defmt::write!(f, "MUL234"),
                0xeb => defmt::write!(f, "MUL235"),
                0xec => defmt::write!(f, "MUL236"),
                0xed => defmt::write!(f, "MUL237"),
                0xee => defmt::write!(f, "MUL238"),
                0xef => defmt::write!(f, "MUL239"),
                0xf0 => defmt::write!(f, "MUL240"),
                0xf1 => defmt::write!(f, "MUL241"),
                0xf2 => defmt::write!(f, "MUL242"),
                0xf3 => defmt::write!(f, "MUL243"),
                0xf4 => defmt::write!(f, "MUL244"),
                0xf5 => defmt::write!(f, "MUL245"),
                0xf6 => defmt::write!(f, "MUL246"),
                0xf7 => defmt::write!(f, "MUL247"),
                0xf8 => defmt::write!(f, "MUL248"),
                0xf9 => defmt::write!(f, "MUL249"),
                0xfa => defmt::write!(f, "MUL250"),
                0xfb => defmt::write!(f, "MUL251"),
                0xfc => defmt::write!(f, "MUL252"),
                0xfd => defmt::write!(f, "MUL253"),
                0xfe => defmt::write!(f, "MUL254"),
                0xff => defmt::write!(f, "MUL255"),
                0x0100 => defmt::write!(f, "MUL256"),
                0x0101 => defmt::write!(f, "MUL257"),
                0x0102 => defmt::write!(f, "MUL258"),
                0x0103 => defmt::write!(f, "MUL259"),
                0x0104 => defmt::write!(f, "MUL260"),
                0x0105 => defmt::write!(f, "MUL261"),
                0x0106 => defmt::write!(f, "MUL262"),
                0x0107 => defmt::write!(f, "MUL263"),
                0x0108 => defmt::write!(f, "MUL264"),
                0x0109 => defmt::write!(f, "MUL265"),
                0x010a => defmt::write!(f, "MUL266"),
                0x010b => defmt::write!(f, "MUL267"),
                0x010c => defmt::write!(f, "MUL268"),
                0x010d => defmt::write!(f, "MUL269"),
                0x010e => defmt::write!(f, "MUL270"),
                0x010f => defmt::write!(f, "MUL271"),
                0x0110 => defmt::write!(f, "MUL272"),
                0x0111 => defmt::write!(f, "MUL273"),
                0x0112 => defmt::write!(f, "MUL274"),
                0x0113 => defmt::write!(f, "MUL275"),
                0x0114 => defmt::write!(f, "MUL276"),
                0x0115 => defmt::write!(f, "MUL277"),
                0x0116 => defmt::write!(f, "MUL278"),
                0x0117 => defmt::write!(f, "MUL279"),
                0x0118 => defmt::write!(f, "MUL280"),
                0x0119 => defmt::write!(f, "MUL281"),
                0x011a => defmt::write!(f, "MUL282"),
                0x011b => defmt::write!(f, "MUL283"),
                0x011c => defmt::write!(f, "MUL284"),
                0x011d => defmt::write!(f, "MUL285"),
                0x011e => defmt::write!(f, "MUL286"),
                0x011f => defmt::write!(f, "MUL287"),
                0x0120 => defmt::write!(f, "MUL288"),
                0x0121 => defmt::write!(f, "MUL289"),
                0x0122 => defmt::write!(f, "MUL290"),
                0x0123 => defmt::write!(f, "MUL291"),
                0x0124 => defmt::write!(f, "MUL292"),
                0x0125 => defmt::write!(f, "MUL293"),
                0x0126 => defmt::write!(f, "MUL294"),
                0x0127 => defmt::write!(f, "MUL295"),
                0x0128 => defmt::write!(f, "MUL296"),
                0x0129 => defmt::write!(f, "MUL297"),
                0x012a => defmt::write!(f, "MUL298"),
                0x012b => defmt::write!(f, "MUL299"),
                0x012c => defmt::write!(f, "MUL300"),
                0x012d => defmt::write!(f, "MUL301"),
                0x012e => defmt::write!(f, "MUL302"),
                0x012f => defmt::write!(f, "MUL303"),
                0x0130 => defmt::write!(f, "MUL304"),
                0x0131 => defmt::write!(f, "MUL305"),
                0x0132 => defmt::write!(f, "MUL306"),
                0x0133 => defmt::write!(f, "MUL307"),
                0x0134 => defmt::write!(f, "MUL308"),
                0x0135 => defmt::write!(f, "MUL309"),
                0x0136 => defmt::write!(f, "MUL310"),
                0x0137 => defmt::write!(f, "MUL311"),
                0x0138 => defmt::write!(f, "MUL312"),
                0x0139 => defmt::write!(f, "MUL313"),
                0x013a => defmt::write!(f, "MUL314"),
                0x013b => defmt::write!(f, "MUL315"),
                0x013c => defmt::write!(f, "MUL316"),
                0x013d => defmt::write!(f, "MUL317"),
                0x013e => defmt::write!(f, "MUL318"),
                0x013f => defmt::write!(f, "MUL319"),
                0x0140 => defmt::write!(f, "MUL320"),
                0x0141 => defmt::write!(f, "MUL321"),
                0x0142 => defmt::write!(f, "MUL322"),
                0x0143 => defmt::write!(f, "MUL323"),
                0x0144 => defmt::write!(f, "MUL324"),
                0x0145 => defmt::write!(f, "MUL325"),
                0x0146 => defmt::write!(f, "MUL326"),
                0x0147 => defmt::write!(f, "MUL327"),
                0x0148 => defmt::write!(f, "MUL328"),
                0x0149 => defmt::write!(f, "MUL329"),
                0x014a => defmt::write!(f, "MUL330"),
                0x014b => defmt::write!(f, "MUL331"),
                0x014c => defmt::write!(f, "MUL332"),
                0x014d => defmt::write!(f, "MUL333"),
                0x014e => defmt::write!(f, "MUL334"),
                0x014f => defmt::write!(f, "MUL335"),
                0x0150 => defmt::write!(f, "MUL336"),
                0x0151 => defmt::write!(f, "MUL337"),
                0x0152 => defmt::write!(f, "MUL338"),
                0x0153 => defmt::write!(f, "MUL339"),
                0x0154 => defmt::write!(f, "MUL340"),
                0x0155 => defmt::write!(f, "MUL341"),
                0x0156 => defmt::write!(f, "MUL342"),
                0x0157 => defmt::write!(f, "MUL343"),
                0x0158 => defmt::write!(f, "MUL344"),
                0x0159 => defmt::write!(f, "MUL345"),
                0x015a => defmt::write!(f, "MUL346"),
                0x015b => defmt::write!(f, "MUL347"),
                0x015c => defmt::write!(f, "MUL348"),
                0x015d => defmt::write!(f, "MUL349"),
                0x015e => defmt::write!(f, "MUL350"),
                0x015f => defmt::write!(f, "MUL351"),
                0x0160 => defmt::write!(f, "MUL352"),
                0x0161 => defmt::write!(f, "MUL353"),
                0x0162 => defmt::write!(f, "MUL354"),
                0x0163 => defmt::write!(f, "MUL355"),
                0x0164 => defmt::write!(f, "MUL356"),
                0x0165 => defmt::write!(f, "MUL357"),
                0x0166 => defmt::write!(f, "MUL358"),
                0x0167 => defmt::write!(f, "MUL359"),
                0x0168 => defmt::write!(f, "MUL360"),
                0x0169 => defmt::write!(f, "MUL361"),
                0x016a => defmt::write!(f, "MUL362"),
                0x016b => defmt::write!(f, "MUL363"),
                0x016c => defmt::write!(f, "MUL364"),
                0x016d => defmt::write!(f, "MUL365"),
                0x016e => defmt::write!(f, "MUL366"),
                0x016f => defmt::write!(f, "MUL367"),
                0x0170 => defmt::write!(f, "MUL368"),
                0x0171 => defmt::write!(f, "MUL369"),
                0x0172 => defmt::write!(f, "MUL370"),
                0x0173 => defmt::write!(f, "MUL371"),
                0x0174 => defmt::write!(f, "MUL372"),
                0x0175 => defmt::write!(f, "MUL373"),
                0x0176 => defmt::write!(f, "MUL374"),
                0x0177 => defmt::write!(f, "MUL375"),
                0x0178 => defmt::write!(f, "MUL376"),
                0x0179 => defmt::write!(f, "MUL377"),
                0x017a => defmt::write!(f, "MUL378"),
                0x017b => defmt::write!(f, "MUL379"),
                0x017c => defmt::write!(f, "MUL380"),
                0x017d => defmt::write!(f, "MUL381"),
                0x017e => defmt::write!(f, "MUL382"),
                0x017f => defmt::write!(f, "MUL383"),
                0x0180 => defmt::write!(f, "MUL384"),
                0x0181 => defmt::write!(f, "MUL385"),
                0x0182 => defmt::write!(f, "MUL386"),
                0x0183 => defmt::write!(f, "MUL387"),
                0x0184 => defmt::write!(f, "MUL388"),
                0x0185 => defmt::write!(f, "MUL389"),
                0x0186 => defmt::write!(f, "MUL390"),
                0x0187 => defmt::write!(f, "MUL391"),
                0x0188 => defmt::write!(f, "MUL392"),
                0x0189 => defmt::write!(f, "MUL393"),
                0x018a => defmt::write!(f, "MUL394"),
                0x018b => defmt::write!(f, "MUL395"),
                0x018c => defmt::write!(f, "MUL396"),
                0x018d => defmt::write!(f, "MUL397"),
                0x018e => defmt::write!(f, "MUL398"),
                0x018f => defmt::write!(f, "MUL399"),
                0x0190 => defmt::write!(f, "MUL400"),
                0x0191 => defmt::write!(f, "MUL401"),
                0x0192 => defmt::write!(f, "MUL402"),
                0x0193 => defmt::write!(f, "MUL403"),
                0x0194 => defmt::write!(f, "MUL404"),
                0x0195 => defmt::write!(f, "MUL405"),
                0x0196 => defmt::write!(f, "MUL406"),
                0x0197 => defmt::write!(f, "MUL407"),
                0x0198 => defmt::write!(f, "MUL408"),
                0x0199 => defmt::write!(f, "MUL409"),
                0x019a => defmt::write!(f, "MUL410"),
                0x019b => defmt::write!(f, "MUL411"),
                0x019c => defmt::write!(f, "MUL412"),
                0x019d => defmt::write!(f, "MUL413"),
                0x019e => defmt::write!(f, "MUL414"),
                0x019f => defmt::write!(f, "MUL415"),
                0x01a0 => defmt::write!(f, "MUL416"),
                0x01a1 => defmt::write!(f, "MUL417"),
                0x01a2 => defmt::write!(f, "MUL418"),
                0x01a3 => defmt::write!(f, "MUL419"),
                0x01a4 => defmt::write!(f, "MUL420"),
                0x01a5 => defmt::write!(f, "MUL421"),
                0x01a6 => defmt::write!(f, "MUL422"),
                0x01a7 => defmt::write!(f, "MUL423"),
                0x01a8 => defmt::write!(f, "MUL424"),
                0x01a9 => defmt::write!(f, "MUL425"),
                0x01aa => defmt::write!(f, "MUL426"),
                0x01ab => defmt::write!(f, "MUL427"),
                0x01ac => defmt::write!(f, "MUL428"),
                0x01ad => defmt::write!(f, "MUL429"),
                0x01ae => defmt::write!(f, "MUL430"),
                0x01af => defmt::write!(f, "MUL431"),
                0x01b0 => defmt::write!(f, "MUL432"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Plln {
        #[inline(always)]
        fn from(val: u16) -> Plln {
            Plln::from_bits(val)
        }
    }
    impl From<Plln> for u16 {
        #[inline(always)]
        fn from(val: Plln) -> u16 {
            Plln::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllp {
        #[doc = "PLLP=2"]
        DIV2 = 0x0,
        #[doc = "PLLP=4"]
        DIV4 = 0x01,
        #[doc = "PLLP=6"]
        DIV6 = 0x02,
        #[doc = "PLLP=8"]
        DIV8 = 0x03,
    }
    impl Pllp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllp {
        #[inline(always)]
        fn from(val: u8) -> Pllp {
            Pllp::from_bits(val)
        }
    }
    impl From<Pllp> for u8 {
        #[inline(always)]
        fn from(val: Pllp) -> u8 {
            Pllp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllq {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        DIV2 = 0x02,
        DIV3 = 0x03,
        DIV4 = 0x04,
        DIV5 = 0x05,
        DIV6 = 0x06,
        DIV7 = 0x07,
        DIV8 = 0x08,
        DIV9 = 0x09,
        DIV10 = 0x0a,
        DIV11 = 0x0b,
        DIV12 = 0x0c,
        DIV13 = 0x0d,
        DIV14 = 0x0e,
        DIV15 = 0x0f,
    }
    impl Pllq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllq {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllq {
        #[inline(always)]
        fn from(val: u8) -> Pllq {
            Pllq::from_bits(val)
        }
    }
    impl From<Pllq> for u8 {
        #[inline(always)]
        fn from(val: Pllq) -> u8 {
            Pllq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllr {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        DIV2 = 0x02,
        DIV3 = 0x03,
        DIV4 = 0x04,
        DIV5 = 0x05,
        DIV6 = 0x06,
        DIV7 = 0x07,
    }
    impl Pllr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllr {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllr {
        #[inline(always)]
        fn from(val: u8) -> Pllr {
            Pllr::from_bits(val)
        }
    }
    impl From<Pllr> for u8 {
        #[inline(always)]
        fn from(val: Pllr) -> u8 {
            Pllr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "HSI clock selected as PLL and PLLI2S clock entry"]
        HSI = 0x0,
        #[doc = "HSE oscillator clock selected as PLL and PLLI2S clock entry"]
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
    pub enum Rtcsel {
        #[doc = "No clock"]
        DISABLE = 0x0,
        #[doc = "LSE oscillator clock used as RTC clock"]
        LSE = 0x01,
        #[doc = "LSI oscillator clock used as RTC clock"]
        LSI = 0x02,
        #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
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
    pub enum Spreadsel {
        #[doc = "Center spread"]
        CENTER = 0x0,
        #[doc = "Down spread"]
        DOWN = 0x01,
    }
    impl Spreadsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spreadsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spreadsel {
        #[inline(always)]
        fn from(val: u8) -> Spreadsel {
            Spreadsel::from_bits(val)
        }
    }
    impl From<Spreadsel> for u8 {
        #[inline(always)]
        fn from(val: Spreadsel) -> u8 {
            Spreadsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sw {
        #[doc = "HSI selected as system clock"]
        HSI = 0x0,
        #[doc = "HSE selected as system clock"]
        HSE = 0x01,
        #[doc = "PLL selected as system clock"]
        PLL1_P = 0x02,
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
}
