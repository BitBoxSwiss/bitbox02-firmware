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
    #[doc = "RCC source control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RCC HSI calibration register."]
    #[inline(always)]
    pub const fn hsicfgr(self) -> crate::common::Reg<regs::Hsicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RCC clock recovery RC register."]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RCC CSI calibration register."]
    #[inline(always)]
    pub const fn csicfgr(self) -> crate::common::Reg<regs::Csicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RCC clock configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RCC CPU domain clock configuration register."]
    #[inline(always)]
    pub const fn cdcfgr(self) -> crate::common::Reg<regs::Cdcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "RCC AHB clock configuration register."]
    #[inline(always)]
    pub const fn bmcfgr(self) -> crate::common::Reg<regs::Bmcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "RCC APB clocks configuration register."]
    #[inline(always)]
    pub const fn apbcfgr(self) -> crate::common::Reg<regs::Apbcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RCC PLLs clock source selection register."]
    #[inline(always)]
    pub const fn pllckselr(self) -> crate::common::Reg<regs::Pllckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RCC PLLs configuration register."]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "RCC PLL dividers configuration register 1."]
    #[inline(always)]
    pub const fn plldivr(self, n: usize) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 8usize) as _) }
    }
    #[doc = "RCC PLL fractional divider register."]
    #[inline(always)]
    pub const fn pllfracr(self, n: usize) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 8usize) as _) }
    }
    #[doc = "RCC AHB peripheral kernel clock selection register."]
    #[inline(always)]
    pub const fn ahbperckselr(self) -> crate::common::Reg<regs::Ahbperckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "RCC APB1 peripherals kernel clock selection register."]
    #[inline(always)]
    pub const fn apb1perckselr(self) -> crate::common::Reg<regs::Apb1perckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RCC APB2 peripherals kernel clock selection register."]
    #[inline(always)]
    pub const fn apb2perckselr(self) -> crate::common::Reg<regs::Apb2perckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RCC APB4,5 peripherals kernel clock selection register."]
    #[inline(always)]
    pub const fn apb45perckselr(self) -> crate::common::Reg<regs::Apb45perckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RCC clock source interrupt enable register."]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "RCC clock source interrupt flag register."]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RCC clock source interrupt clear register."]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "RCC Backup domain control register."]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "RCC clock control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "RCC AHB5 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb5rstr(self) -> crate::common::Reg<regs::Ahb5rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "RCC AHB1 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "RCC AHB4 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb4rstr(self) -> crate::common::Reg<regs::Ahb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "RCC APB5 peripheral reset register."]
    #[inline(always)]
    pub const fn apb5rstr(self) -> crate::common::Reg<regs::Apb5rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 1."]
    #[inline(always)]
    pub const fn apb1rstr1(self) -> crate::common::Reg<regs::Apb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 2."]
    #[inline(always)]
    pub const fn apb1rstr2(self) -> crate::common::Reg<regs::Apb1rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RCC APB2 peripheral reset register."]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "RCC APB4 peripheral reset register."]
    #[inline(always)]
    pub const fn apb4rstr(self) -> crate::common::Reg<regs::Apb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "RCC AHB3 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "RCC AXI clocks gating disable register."]
    #[inline(always)]
    pub const fn ckgdisr(self) -> crate::common::Reg<regs::Ckgdisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "RCC PLL dividers configuration register 2."]
    #[inline(always)]
    pub const fn plldivr2(self, n: usize) -> crate::common::Reg<regs::Plldivr2, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "RCC PLL Spread Spectrum Clock Generator register."]
    #[inline(always)]
    pub const fn pllsscgr(self, n: usize) -> crate::common::Reg<regs::Pllsscgr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize + n * 4usize) as _) }
    }
    #[doc = "RCC clock protection register."]
    #[inline(always)]
    pub const fn ckprotr(self) -> crate::common::Reg<regs::Ckprotr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "RCC Reset status register."]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "RCC AHB5 clock enable register."]
    #[inline(always)]
    pub const fn ahb5enr(self) -> crate::common::Reg<regs::Ahb5enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "RCC AHB1 clock enable register."]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "RCC AHB2 clock enable register."]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "RCC AHB4 clock enable register."]
    #[inline(always)]
    pub const fn ahb4enr(self) -> crate::common::Reg<regs::Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "RCC APB5 clock enable register."]
    #[inline(always)]
    pub const fn apb5enr(self) -> crate::common::Reg<regs::Apb5enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "RCC APB1 clock enable register 1."]
    #[inline(always)]
    pub const fn apb1enr1(self) -> crate::common::Reg<regs::Apb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "RCC APB1 clock enable register 2."]
    #[inline(always)]
    pub const fn apb1enr2(self) -> crate::common::Reg<regs::Apb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "RCC APB2 clock enable register."]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "RCC APB4 clock enable register."]
    #[inline(always)]
    pub const fn apb4enr(self) -> crate::common::Reg<regs::Apb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "RCC AHB3 clock enable register."]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "RCC AHB5 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb5lpenr(self) -> crate::common::Reg<regs::Ahb5lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "RCC AHB1 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "RCC AHB2 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "RCC AHB4 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb4lpenr(self) -> crate::common::Reg<regs::Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "RCC AHB3 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb3lpenr(self) -> crate::common::Reg<regs::Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "RCC APB1 low-power clock enable register 1."]
    #[inline(always)]
    pub const fn apb1lpenr1(self) -> crate::common::Reg<regs::Apb1lpenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "RCC APB1 low-power clock enable register 2."]
    #[inline(always)]
    pub const fn apb1lpenr2(self) -> crate::common::Reg<regs::Apb1lpenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "RCC APB2 low-power clock enable register."]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "RCC APB4 low-power clock enable register."]
    #[inline(always)]
    pub const fn apb4lpenr(self) -> crate::common::Reg<regs::Apb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[doc = "RCC APB5 sleep clock register."]
    #[inline(always)]
    pub const fn apb5lpenr(self) -> crate::common::Reg<regs::Apb5lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "GPDMA1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC1 and 2 peripheral clocks enable Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to ADCx_CK input, and the hclk1 bus interface clock."]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and 2 peripheral clocks enable Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to ADCx_CK input, and the hclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ETH1 MAC peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ETH1 MAC peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ETH1 transmission clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ETH1 transmission clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ETH1 reception clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ETH1 reception clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OTGHS clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "OTGHS clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USBPHYC clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn usbphycen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USBPHYC clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_usbphycen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "OTGFS peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "OTGFS peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADF clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn adfen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADF clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_adfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("gpdma1en", &self.gpdma1en())
                .field("adc12en", &self.adc12en())
                .field("ethen", &self.ethen())
                .field("ethtxen", &self.ethtxen())
                .field("ethrxen", &self.ethrxen())
                .field("usb_otg_hsen", &self.usb_otg_hsen())
                .field("usbphycen", &self.usbphycen())
                .field("usb_otg_fsen", &self.usb_otg_fsen())
                .field("adfen", &self.adfen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1enr {{ gpdma1en: {=bool:?}, adc12en: {=bool:?}, ethen: {=bool:?}, ethtxen: {=bool:?}, ethrxen: {=bool:?}, usb_otg_hsen: {=bool:?}, usbphycen: {=bool:?}, usb_otg_fsen: {=bool:?}, adfen: {=bool:?} }}" , self . gpdma1en () , self . adc12en () , self . ethen () , self . ethtxen () , self . ethrxen () , self . usb_otg_hsen () , self . usbphycen () , self . usb_otg_fsen () , self . adfen ())
        }
    }
    #[doc = "RCC AHB1 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "GPDMA1 clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC1 and 2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to ADCx_CK input, and the rcc_hclk1 bus interface clock."]
        #[inline(always)]
        pub const fn adc12lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and 2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to ADCx_CK input, and the rcc_hclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_adc12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ETH1 MAC peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn ethlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ETH1 MAC peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_ethlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ETH1 transmission peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn ethtxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ETH1 transmission peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_ethtxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ETH1 reception peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn ethrxlpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ETH1 reception peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_ethrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USBPHYC common block power-down control Set and reset by software."]
        #[inline(always)]
        pub const fn usbpdctrl(&self) -> super::vals::Usbpdctrl {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Usbpdctrl::from_bits(val as u8)
        }
        #[doc = "USBPHYC common block power-down control Set and reset by software."]
        #[inline(always)]
        pub fn set_usbpdctrl(&mut self, val: super::vals::Usbpdctrl) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "OTGHS peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn usb_otg_hslpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "OTGHS peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usb_otg_hslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USBPHYC peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn usbphyclpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USBPHYC peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usbphyclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "OTGFS clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn usb_otg_fslpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "OTGFS clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usb_otg_fslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADF clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn adflpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADF clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_adflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("gpdma1lpen", &self.gpdma1lpen())
                .field("adc12lpen", &self.adc12lpen())
                .field("ethlpen", &self.ethlpen())
                .field("ethtxlpen", &self.ethtxlpen())
                .field("ethrxlpen", &self.ethrxlpen())
                .field("usbpdctrl", &self.usbpdctrl())
                .field("usb_otg_hslpen", &self.usb_otg_hslpen())
                .field("usbphyclpen", &self.usbphyclpen())
                .field("usb_otg_fslpen", &self.usb_otg_fslpen())
                .field("adflpen", &self.adflpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1lpenr {{ gpdma1lpen: {=bool:?}, adc12lpen: {=bool:?}, ethlpen: {=bool:?}, ethtxlpen: {=bool:?}, ethrxlpen: {=bool:?}, usbpdctrl: {:?}, usb_otg_hslpen: {=bool:?}, usbphyclpen: {=bool:?}, usb_otg_fslpen: {=bool:?}, adflpen: {=bool:?} }}" , self . gpdma1lpen () , self . adc12lpen () , self . ethlpen () , self . ethtxlpen () , self . ethrxlpen () , self . usbpdctrl () , self . usb_otg_hslpen () , self . usbphyclpen () , self . usb_otg_fslpen () , self . adflpen ())
        }
    }
    #[doc = "RCC AHB1 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "GPDMA1 blocks reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 blocks reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC1 and 2 blocks reset Set and reset by software."]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and 2 blocks reset Set and reset by software."]
        #[inline(always)]
        pub fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ETH1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn ethrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ETH1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_ethrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "OTGHS block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "OTGHS block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USBPHYC block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usbphycrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USBPHYC block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usbphycrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "OTGFS block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "OTGFS block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usb_otg_fsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADF block reset Set and reset by software."]
        #[inline(always)]
        pub const fn adfrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADF block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_adfrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("gpdma1rst", &self.gpdma1rst())
                .field("adc12rst", &self.adc12rst())
                .field("ethrst", &self.ethrst())
                .field("usb_otg_hsrst", &self.usb_otg_hsrst())
                .field("usbphycrst", &self.usbphycrst())
                .field("usb_otg_fsrst", &self.usb_otg_fsrst())
                .field("adfrst", &self.adfrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1rstr {{ gpdma1rst: {=bool:?}, adc12rst: {=bool:?}, ethrst: {=bool:?}, usb_otg_hsrst: {=bool:?}, usbphycrst: {=bool:?}, usb_otg_fsrst: {=bool:?}, adfrst: {=bool:?} }}" , self . gpdma1rst () , self . adc12rst () , self . ethrst () , self . usb_otg_hsrst () , self . usbphycrst () , self . usb_otg_fsrst () , self . adfrst ())
        }
    }
    #[doc = "RCC AHB2 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "PSSI peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn pssien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PSSI peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_pssien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CORDIC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn cordicen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_cordicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SRAM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("pssien", &self.pssien())
                .field("sdmmc2en", &self.sdmmc2en())
                .field("cordicen", &self.cordicen())
                .field("sram1en", &self.sram1en())
                .field("sram2en", &self.sram2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2enr {{ pssien: {=bool:?}, sdmmc2en: {=bool:?}, cordicen: {=bool:?}, sram1en: {=bool:?}, sram2en: {=bool:?} }}" , self . pssien () , self . sdmmc2en () , self . cordicen () , self . sram1en () , self . sram2en ())
        }
    }
    #[doc = "RCC AHB2 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "PSSI peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn pssilpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PSSI peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_pssilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn sdmmc2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_sdmmc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CORDIC clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn cordiclpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_cordiclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SRAM1 clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("pssilpen", &self.pssilpen())
                .field("sdmmc2lpen", &self.sdmmc2lpen())
                .field("cordiclpen", &self.cordiclpen())
                .field("sram1lpen", &self.sram1lpen())
                .field("sram2lpen", &self.sram2lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2lpenr {{ pssilpen: {=bool:?}, sdmmc2lpen: {=bool:?}, cordiclpen: {=bool:?}, sram1lpen: {=bool:?}, sram2lpen: {=bool:?} }}" , self . pssilpen () , self . sdmmc2lpen () , self . cordiclpen () , self . sram1lpen () , self . sram2lpen ())
        }
    }
    #[doc = "RCC AHB2 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "PSSI block reset Set and reset by software."]
        #[inline(always)]
        pub const fn pssirst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PSSI block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_pssirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
        #[inline(always)]
        pub const fn sdmmc2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
        #[inline(always)]
        pub fn set_sdmmc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CORDIC reset Set and reset by software."]
        #[inline(always)]
        pub const fn cordicrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC reset Set and reset by software."]
        #[inline(always)]
        pub fn set_cordicrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("pssirst", &self.pssirst())
                .field("sdmmc2rst", &self.sdmmc2rst())
                .field("cordicrst", &self.cordicrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2rstr {{ pssirst: {=bool:?}, sdmmc2rst: {=bool:?}, cordicrst: {=bool:?} }}",
                self.pssirst(),
                self.sdmmc2rst(),
                self.cordicrst()
            )
        }
    }
    #[doc = "RCC AHB3 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "RNG peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HASH peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CRYP peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SAES peripheral clock enable Set and reset by software. This bit controls the enable of the clock delivered to the SAES."]
        #[inline(always)]
        pub const fn saesen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SAES peripheral clock enable Set and reset by software. This bit controls the enable of the clock delivered to the SAES."]
        #[inline(always)]
        pub fn set_saesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PKA peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PKA peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
            f.debug_struct("Ahb3enr")
                .field("rngen", &self.rngen())
                .field("hashen", &self.hashen())
                .field("crypen", &self.crypen())
                .field("saesen", &self.saesen())
                .field("pkaen", &self.pkaen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb3enr {{ rngen: {=bool:?}, hashen: {=bool:?}, crypen: {=bool:?}, saesen: {=bool:?}, pkaen: {=bool:?} }}" , self . rngen () , self . hashen () , self . crypen () , self . saesen () , self . pkaen ())
        }
    }
    #[doc = "RCC AHB3 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3lpenr(pub u32);
    impl Ahb3lpenr {
        #[doc = "RNG peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HASH peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CRYP peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn cryplpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_cryplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SAES peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn saeslpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SAES peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_saeslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PKA peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn pkalpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PKA peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_pkalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
            f.debug_struct("Ahb3lpenr")
                .field("rnglpen", &self.rnglpen())
                .field("hashlpen", &self.hashlpen())
                .field("cryplpen", &self.cryplpen())
                .field("saeslpen", &self.saeslpen())
                .field("pkalpen", &self.pkalpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb3lpenr {{ rnglpen: {=bool:?}, hashlpen: {=bool:?}, cryplpen: {=bool:?}, saeslpen: {=bool:?}, pkalpen: {=bool:?} }}" , self . rnglpen () , self . hashlpen () , self . cryplpen () , self . saeslpen () , self . pkalpen ())
        }
    }
    #[doc = "RCC AHB3 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "random number generator block reset Set and reset by software."]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "random number generator block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HASH block reset Set and reset by software."]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HASH block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CRYP block reset Set and reset by software."]
        #[inline(always)]
        pub const fn cryprst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_cryprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SAES block reset Set and reset by software."]
        #[inline(always)]
        pub const fn saesrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SAES block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_saesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PKA block reset Set and reset by software."]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PKA block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
            f.debug_struct("Ahb3rstr")
                .field("rngrst", &self.rngrst())
                .field("hashrst", &self.hashrst())
                .field("cryprst", &self.cryprst())
                .field("saesrst", &self.saesrst())
                .field("pkarst", &self.pkarst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb3rstr {{ rngrst: {=bool:?}, hashrst: {=bool:?}, cryprst: {=bool:?}, saesrst: {=bool:?}, pkarst: {=bool:?} }}" , self . rngrst () , self . hashrst () , self . cryprst () , self . saesrst () , self . pkarst ())
        }
    }
    #[doc = "RCC AHB4 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4enr(pub u32);
    impl Ahb4enr {
        #[doc = "GPIOA peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOM peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiomen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOM peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiomen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPION peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpionen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPION peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpionen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIOO peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiooen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOO peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiooen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPIOP peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiopen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOP peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiopen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Backup RAM clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn bkpramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_bkpramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb4enr {
        #[inline(always)]
        fn default() -> Ahb4enr {
            Ahb4enr(0)
        }
    }
    impl core::fmt::Debug for Ahb4enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4enr")
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("gpiomen", &self.gpiomen())
                .field("gpionen", &self.gpionen())
                .field("gpiooen", &self.gpiooen())
                .field("gpiopen", &self.gpiopen())
                .field("crcen", &self.crcen())
                .field("bkpramen", &self.bkpramen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb4enr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiofen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, gpiomen: {=bool:?}, gpionen: {=bool:?}, gpiooen: {=bool:?}, gpiopen: {=bool:?}, crcen: {=bool:?}, bkpramen: {=bool:?} }}" , self . gpioaen () , self . gpioben () , self . gpiocen () , self . gpioden () , self . gpioeen () , self . gpiofen () , self . gpiogen () , self . gpiohen () , self . gpiomen () , self . gpionen () , self . gpiooen () , self . gpiopen () , self . crcen () , self . bkpramen ())
        }
    }
    #[doc = "RCC AHB4 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4lpenr(pub u32);
    impl Ahb4lpenr {
        #[doc = "GPIOA peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOM peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpiomlpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOM peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiomlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPION peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpionlpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPION peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpionlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIOO peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioolpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOO peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioolpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPIOP peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioplpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOP peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRC clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Backup RAM clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn bkpramlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_bkpramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb4lpenr {
        #[inline(always)]
        fn default() -> Ahb4lpenr {
            Ahb4lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb4lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4lpenr")
                .field("gpioalpen", &self.gpioalpen())
                .field("gpioblpen", &self.gpioblpen())
                .field("gpioclpen", &self.gpioclpen())
                .field("gpiodlpen", &self.gpiodlpen())
                .field("gpioelpen", &self.gpioelpen())
                .field("gpioflpen", &self.gpioflpen())
                .field("gpioglpen", &self.gpioglpen())
                .field("gpiohlpen", &self.gpiohlpen())
                .field("gpiomlpen", &self.gpiomlpen())
                .field("gpionlpen", &self.gpionlpen())
                .field("gpioolpen", &self.gpioolpen())
                .field("gpioplpen", &self.gpioplpen())
                .field("crclpen", &self.crclpen())
                .field("bkpramlpen", &self.bkpramlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb4lpenr {{ gpioalpen: {=bool:?}, gpioblpen: {=bool:?}, gpioclpen: {=bool:?}, gpiodlpen: {=bool:?}, gpioelpen: {=bool:?}, gpioflpen: {=bool:?}, gpioglpen: {=bool:?}, gpiohlpen: {=bool:?}, gpiomlpen: {=bool:?}, gpionlpen: {=bool:?}, gpioolpen: {=bool:?}, gpioplpen: {=bool:?}, crclpen: {=bool:?}, bkpramlpen: {=bool:?} }}" , self . gpioalpen () , self . gpioblpen () , self . gpioclpen () , self . gpiodlpen () , self . gpioelpen () , self . gpioflpen () , self . gpioglpen () , self . gpiohlpen () , self . gpiomlpen () , self . gpionlpen () , self . gpioolpen () , self . gpioplpen () , self . crclpen () , self . bkpramlpen ())
        }
    }
    #[doc = "RCC AHB4 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4rstr(pub u32);
    impl Ahb4rstr {
        #[doc = "GPIOA block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOM block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiomrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOM block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiomrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPION block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpionrst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPION block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpionrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIOO block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpioorst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOO block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioorst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPIOP block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpioprst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOP block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRC block reset Set and reset by software."]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Ahb4rstr {
        #[inline(always)]
        fn default() -> Ahb4rstr {
            Ahb4rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb4rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4rstr")
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiofrst", &self.gpiofrst())
                .field("gpiogrst", &self.gpiogrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("gpiomrst", &self.gpiomrst())
                .field("gpionrst", &self.gpionrst())
                .field("gpioorst", &self.gpioorst())
                .field("gpioprst", &self.gpioprst())
                .field("crcrst", &self.crcrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb4rstr {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiofrst: {=bool:?}, gpiogrst: {=bool:?}, gpiohrst: {=bool:?}, gpiomrst: {=bool:?}, gpionrst: {=bool:?}, gpioorst: {=bool:?}, gpioprst: {=bool:?}, crcrst: {=bool:?} }}" , self . gpioarst () , self . gpiobrst () , self . gpiocrst () , self . gpiodrst () , self . gpioerst () , self . gpiofrst () , self . gpiogrst () , self . gpiohrst () , self . gpiomrst () , self . gpionrst () , self . gpioorst () , self . gpioprst () , self . crcrst ())
        }
    }
    #[doc = "RCC AHB5 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5enr(pub u32);
    impl Ahb5enr {
        #[doc = "HPDMA1 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn hpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA1 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_hpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "JPEG peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn jpegen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_jpegen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FMC and MCE3 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock."]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FMC and MCE3 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock."]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "XSPI1 and MCE1 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub const fn xspi1en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 and MCE1 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_xspi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC1 and DB_SDMMC1 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and DB_SDMMC1 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "XSPI2 and MCE2 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub const fn xspi2en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI2 and MCE2 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_xspi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "XSPIM peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn iomngren(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_iomngren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GFXMMU peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gfxmmuen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gfxmmuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpuen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ahb5enr {
        #[inline(always)]
        fn default() -> Ahb5enr {
            Ahb5enr(0)
        }
    }
    impl core::fmt::Debug for Ahb5enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb5enr")
                .field("hpdma1en", &self.hpdma1en())
                .field("dma2den", &self.dma2den())
                .field("jpegen", &self.jpegen())
                .field("fmcen", &self.fmcen())
                .field("xspi1en", &self.xspi1en())
                .field("sdmmc1en", &self.sdmmc1en())
                .field("xspi2en", &self.xspi2en())
                .field("iomngren", &self.iomngren())
                .field("gfxmmuen", &self.gfxmmuen())
                .field("gpuen", &self.gpuen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb5enr {{ hpdma1en: {=bool:?}, dma2den: {=bool:?}, jpegen: {=bool:?}, fmcen: {=bool:?}, xspi1en: {=bool:?}, sdmmc1en: {=bool:?}, xspi2en: {=bool:?}, iomngren: {=bool:?}, gfxmmuen: {=bool:?}, gpuen: {=bool:?} }}" , self . hpdma1en () , self . dma2den () , self . jpegen () , self . fmcen () , self . xspi1en () , self . sdmmc1en () , self . xspi2en () , self . iomngren () , self . gfxmmuen () , self . gpuen ())
        }
    }
    #[doc = "RCC AHB5 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5lpenr(pub u32);
    impl Ahb5lpenr {
        #[doc = "HPDMA1 low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn hpdma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA1 low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_hpdma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn dma2dlpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_dma2dlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FLITF low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn flitflpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FLITF low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_flitflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "JPEG clock enable during Sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn jpeglpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG clock enable during Sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_jpeglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FMC and MCE3 peripheral clocks enable during Sleep mode Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock."]
        #[inline(always)]
        pub const fn fmclpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FMC and MCE3 peripheral clocks enable during Sleep mode Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock."]
        #[inline(always)]
        pub fn set_fmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "XSPI1 and MCE1 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub const fn xspi1lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 and MCE1 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_xspi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC1 and SDMMC1 delay low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sdmmc1lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 delay low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sdmmc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "XSPI2 and MCE2 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub const fn xspi2lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI2 and MCE2 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_xspi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "XSPIM low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn xspimlpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_xspimlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GFXMMU low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gfxmmulpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gfxmmulpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpulpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpulpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DTCM1 low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn dtcm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DTCM1 low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_dtcm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DTCM2 low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn dtcm2lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DTCM2 low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_dtcm2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ITCM low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn itcmlpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ITCM low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_itcmlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM\\[4:1\\]
low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn axisramlpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM\\[4:1\\]
low-power peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_axisramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb5lpenr {
        #[inline(always)]
        fn default() -> Ahb5lpenr {
            Ahb5lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb5lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb5lpenr")
                .field("hpdma1lpen", &self.hpdma1lpen())
                .field("dma2dlpen", &self.dma2dlpen())
                .field("flitflpen", &self.flitflpen())
                .field("jpeglpen", &self.jpeglpen())
                .field("fmclpen", &self.fmclpen())
                .field("xspi1lpen", &self.xspi1lpen())
                .field("sdmmc1lpen", &self.sdmmc1lpen())
                .field("xspi2lpen", &self.xspi2lpen())
                .field("xspimlpen", &self.xspimlpen())
                .field("gfxmmulpen", &self.gfxmmulpen())
                .field("gpulpen", &self.gpulpen())
                .field("dtcm1lpen", &self.dtcm1lpen())
                .field("dtcm2lpen", &self.dtcm2lpen())
                .field("itcmlpen", &self.itcmlpen())
                .field("axisramlpen", &self.axisramlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb5lpenr {{ hpdma1lpen: {=bool:?}, dma2dlpen: {=bool:?}, flitflpen: {=bool:?}, jpeglpen: {=bool:?}, fmclpen: {=bool:?}, xspi1lpen: {=bool:?}, sdmmc1lpen: {=bool:?}, xspi2lpen: {=bool:?}, xspimlpen: {=bool:?}, gfxmmulpen: {=bool:?}, gpulpen: {=bool:?}, dtcm1lpen: {=bool:?}, dtcm2lpen: {=bool:?}, itcmlpen: {=bool:?}, axisramlpen: {=bool:?} }}" , self . hpdma1lpen () , self . dma2dlpen () , self . flitflpen () , self . jpeglpen () , self . fmclpen () , self . xspi1lpen () , self . sdmmc1lpen () , self . xspi2lpen () , self . xspimlpen () , self . gfxmmulpen () , self . gpulpen () , self . dtcm1lpen () , self . dtcm2lpen () , self . itcmlpen () , self . axisramlpen ())
        }
    }
    #[doc = "RCC AHB5 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5rstr(pub u32);
    impl Ahb5rstr {
        #[doc = "HPDMA1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn hpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_hpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D block reset Set and reset by software."]
        #[inline(always)]
        pub const fn dma2drst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_dma2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "JPEG block reset Set and reset by software."]
        #[inline(always)]
        pub const fn jpegrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_jpegrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FMC and MCE3 blocks reset Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1."]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FMC and MCE3 blocks reset Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1."]
        #[inline(always)]
        pub fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "XSPI1 and MCE1 blocks reset Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub const fn xspi1rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 and MCE1 blocks reset Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_xspi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC1 and DB_SDMMC1 blocks reset Set and reset by software."]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and DB_SDMMC1 blocks reset Set and reset by software."]
        #[inline(always)]
        pub fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "XSPI2 and MCE2 blocks reset Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub const fn xspi2rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI2 and MCE2 blocks reset Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_xspi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "XSPIM reset Set and reset by software."]
        #[inline(always)]
        pub const fn iomngrrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM reset Set and reset by software."]
        #[inline(always)]
        pub fn set_iomngrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GFXMMU block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gfxmmurst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gfxmmurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpurst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ahb5rstr {
        #[inline(always)]
        fn default() -> Ahb5rstr {
            Ahb5rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb5rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb5rstr")
                .field("hpdma1rst", &self.hpdma1rst())
                .field("dma2drst", &self.dma2drst())
                .field("jpegrst", &self.jpegrst())
                .field("fmcrst", &self.fmcrst())
                .field("xspi1rst", &self.xspi1rst())
                .field("sdmmc1rst", &self.sdmmc1rst())
                .field("xspi2rst", &self.xspi2rst())
                .field("iomngrrst", &self.iomngrrst())
                .field("gfxmmurst", &self.gfxmmurst())
                .field("gpurst", &self.gpurst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb5rstr {{ hpdma1rst: {=bool:?}, dma2drst: {=bool:?}, jpegrst: {=bool:?}, fmcrst: {=bool:?}, xspi1rst: {=bool:?}, sdmmc1rst: {=bool:?}, xspi2rst: {=bool:?}, iomngrrst: {=bool:?}, gfxmmurst: {=bool:?}, gpurst: {=bool:?} }}" , self . hpdma1rst () , self . dma2drst () , self . jpegrst () , self . fmcrst () , self . xspi1rst () , self . sdmmc1rst () , self . xspi2rst () , self . iomngrrst () , self . gfxmmurst () , self . gpurst ())
        }
    }
    #[doc = "RCC AHB peripheral kernel clock selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbperckselr(pub u32);
    impl Ahbperckselr {
        #[doc = "FMC kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub const fn fmcsel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "FMC kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_fmcsel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "SDMMC1 and SDMMC2 kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub const fn sdmmcsel(&self) -> super::vals::Sdmmcsel {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Sdmmcsel::from_bits(val as u8)
        }
        #[doc = "SDMMC1 and SDMMC2 kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_sdmmcsel(&mut self, val: super::vals::Sdmmcsel) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "XSPI1 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock."]
        #[inline(always)]
        pub const fn xspi1sel(&self) -> super::vals::Xspisel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Xspisel::from_bits(val as u8)
        }
        #[doc = "XSPI1 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock."]
        #[inline(always)]
        pub fn set_xspi1sel(&mut self, val: super::vals::Xspisel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "XSPI2 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock."]
        #[inline(always)]
        pub const fn xspi2sel(&self) -> super::vals::Xspisel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Xspisel::from_bits(val as u8)
        }
        #[doc = "XSPI2 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock."]
        #[inline(always)]
        pub fn set_xspi2sel(&mut self, val: super::vals::Xspisel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "USBPHYC kernel clock frequency selection Set and reset by software. This field is used to indicate to the USBPHYC, the frequency of the reference kernel clock provided to the USBPHYC. others: reserved."]
        #[inline(always)]
        pub const fn usbrefcksel(&self) -> super::vals::Usbrefcksel {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Usbrefcksel::from_bits(val as u8)
        }
        #[doc = "USBPHYC kernel clock frequency selection Set and reset by software. This field is used to indicate to the USBPHYC, the frequency of the reference kernel clock provided to the USBPHYC. others: reserved."]
        #[inline(always)]
        pub fn set_usbrefcksel(&mut self, val: super::vals::Usbrefcksel) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "USBPHYC kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub const fn usbphycsel(&self) -> super::vals::Usbphycsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Usbphycsel::from_bits(val as u8)
        }
        #[doc = "USBPHYC kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_usbphycsel(&mut self, val: super::vals::Usbphycsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "OTGFS kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub const fn usb_otg_fssel(&self) -> super::vals::UsbOtgFssel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::UsbOtgFssel::from_bits(val as u8)
        }
        #[doc = "OTGFS kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_usb_otg_fssel(&mut self, val: super::vals::UsbOtgFssel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Ethernet reference clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn eth_ref_clk_sel(&self) -> super::vals::EthRefClkSel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::EthRefClkSel::from_bits(val as u8)
        }
        #[doc = "Ethernet reference clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_eth_ref_clk_sel(&mut self, val: super::vals::EthRefClkSel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Clock source selection for external Ethernet PHY Set and reset by software."]
        #[inline(always)]
        pub const fn ethphy_clk_sel(&self) -> super::vals::EthphyClkSel {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::EthphyClkSel::from_bits(val as u8)
        }
        #[doc = "Clock source selection for external Ethernet PHY Set and reset by software."]
        #[inline(always)]
        pub fn set_ethphy_clk_sel(&mut self, val: super::vals::EthphyClkSel) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "ADF kernel clock source selection Set and reset by software. Note: I2S_CKIN is an external clock taken from a pin."]
        #[inline(always)]
        pub const fn adfsel(&self) -> super::vals::Adfsel {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Adfsel::from_bits(val as u8)
        }
        #[doc = "ADF kernel clock source selection Set and reset by software. Note: I2S_CKIN is an external clock taken from a pin."]
        #[inline(always)]
        pub fn set_adfsel(&mut self, val: super::vals::Adfsel) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "PSSI kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub const fn pssisel(&self) -> super::vals::Pssisel {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::Pssisel::from_bits(val as u8)
        }
        #[doc = "PSSI kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_pssisel(&mut self, val: super::vals::Pssisel) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "per_ck clock source selection."]
        #[inline(always)]
        pub const fn persel(&self) -> super::vals::Persel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Persel::from_bits(val as u8)
        }
        #[doc = "per_ck clock source selection."]
        #[inline(always)]
        pub fn set_persel(&mut self, val: super::vals::Persel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ahbperckselr {
        #[inline(always)]
        fn default() -> Ahbperckselr {
            Ahbperckselr(0)
        }
    }
    impl core::fmt::Debug for Ahbperckselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbperckselr")
                .field("fmcsel", &self.fmcsel())
                .field("sdmmcsel", &self.sdmmcsel())
                .field("xspi1sel", &self.xspi1sel())
                .field("xspi2sel", &self.xspi2sel())
                .field("usbrefcksel", &self.usbrefcksel())
                .field("usbphycsel", &self.usbphycsel())
                .field("usb_otg_fssel", &self.usb_otg_fssel())
                .field("eth_ref_clk_sel", &self.eth_ref_clk_sel())
                .field("ethphy_clk_sel", &self.ethphy_clk_sel())
                .field("adfsel", &self.adfsel())
                .field("adcsel", &self.adcsel())
                .field("pssisel", &self.pssisel())
                .field("persel", &self.persel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbperckselr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahbperckselr {{ fmcsel: {:?}, sdmmcsel: {:?}, xspi1sel: {:?}, xspi2sel: {:?}, usbrefcksel: {:?}, usbphycsel: {:?}, usb_otg_fssel: {:?}, eth_ref_clk_sel: {:?}, ethphy_clk_sel: {:?}, adfsel: {:?}, adcsel: {:?}, pssisel: {:?}, persel: {:?} }}" , self . fmcsel () , self . sdmmcsel () , self . xspi1sel () , self . xspi2sel () , self . usbrefcksel () , self . usbphycsel () , self . usb_otg_fssel () , self . eth_ref_clk_sel () , self . ethphy_clk_sel () , self . adfsel () , self . adcsel () , self . pssisel () , self . persel ())
        }
    }
    #[doc = "RCC APB1 clock enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr1(pub u32);
    impl Apb1enr1 {
        #[doc = "TIM2 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 peripheral clocks enable Set and reset by software. The peripheral clocks of the LPTIM1 are the kernel clock selected by LPTIM1SEL and provided to clk_lpt input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 peripheral clocks enable Set and reset by software. The peripheral clocks of the LPTIM1 are the kernel clock selected by LPTIM1SEL and provided to clk_lpt input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG clock enable Set by software, and reset by hardware when a system reset occurs."]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable Set by software, and reset by hardware when a system reset occurs."]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI2 are the kernel clock selected by I2S123SRC and provided to com_clk input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI2 are the kernel clock selected by I2S123SRC and provided to com_clk input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI3 are the kernel clock selected by I2S123SRC and provided to com_clk input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI3 are the kernel clock selected by I2S123SRC and provided to com_clk input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX peripheral clocks enable Set and reset by software. The peripheral clocks of the SPDIFRX are the kernel clock selected by SPDIFRXSEL and provided to SPDIFRX_CLK input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub const fn spdifrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX peripheral clocks enable Set and reset by software. The peripheral clocks of the SPDIFRX are the kernel clock selected by SPDIFRXSEL and provided to SPDIFRX_CLK input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_spdifrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2peripheral clocks enable Set and reset by software. The peripheral clocks of the USART2 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2peripheral clocks enable Set and reset by software. The peripheral clocks of the USART2 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 peripheral clocks enable Set and reset by software. The peripheral clocks of the USART3 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 peripheral clocks enable Set and reset by software. The peripheral clocks of the USART3 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 peripheral clocks enable Set and reset by software. The peripheral clocks of the UART4 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 peripheral clocks enable Set and reset by software. The peripheral clocks of the UART4 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock."]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1/I3C1 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn i2c1_i3c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1/I3C1 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c1_i3c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "HDMI-CEC peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "UART7 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub const fn uart8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 peripheral clocks enable Set and reset by software."]
        #[inline(always)]
        pub fn set_uart8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("tim5en", &self.tim5en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("tim12en", &self.tim12en())
                .field("tim13en", &self.tim13en())
                .field("tim14en", &self.tim14en())
                .field("lptim1en", &self.lptim1en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("spdifrxen", &self.spdifrxen())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1_i3c1en", &self.i2c1_i3c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i2c3en", &self.i2c3en())
                .field("cecen", &self.cecen())
                .field("uart7en", &self.uart7en())
                .field("uart8en", &self.uart8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1enr1 {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, tim5en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, tim12en: {=bool:?}, tim13en: {=bool:?}, tim14en: {=bool:?}, lptim1en: {=bool:?}, wwdgen: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, spdifrxen: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1_i3c1en: {=bool:?}, i2c2en: {=bool:?}, i2c3en: {=bool:?}, cecen: {=bool:?}, uart7en: {=bool:?}, uart8en: {=bool:?} }}" , self . tim2en () , self . tim3en () , self . tim4en () , self . tim5en () , self . tim6en () , self . tim7en () , self . tim12en () , self . tim13en () , self . tim14en () , self . lptim1en () , self . wwdgen () , self . spi2en () , self . spi3en () , self . spdifrxen () , self . usart2en () , self . usart3en () , self . uart4en () , self . uart5en () , self . i2c1_i3c1en () , self . i2c2en () , self . i2c3en () , self . cecen () , self . uart7en () , self . uart8en ())
        }
    }
    #[doc = "RCC APB1 clock enable register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr2(pub u32);
    impl Apb1enr2 {
        #[doc = "clock recovery system peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock recovery system peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MDIOS peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn mdiosen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_mdiosen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_fdcanen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UCPD peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn ucpden(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_ucpden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("crsen", &self.crsen())
                .field("mdiosen", &self.mdiosen())
                .field("fdcanen", &self.fdcanen())
                .field("ucpden", &self.ucpden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1enr2 {{ crsen: {=bool:?}, mdiosen: {=bool:?}, fdcanen: {=bool:?}, ucpden: {=bool:?} }}",
                self.crsen(),
                self.mdiosen(),
                self.fdcanen(),
                self.ucpden()
            )
        }
    }
    #[doc = "RCC APB1 low-power clock enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lpenr1(pub u32);
    impl Apb1lpenr1 {
        #[doc = "TIM2 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn wwdglpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_wwdglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn spdifrxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_spdifrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1/I3C1 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn i2c1_i3c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1/I3C1 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c1_i3c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "HDMI-CEC peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn ceclpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_ceclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "UART7 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn uart8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 peripheral clocks enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_uart8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1lpenr1 {
        #[inline(always)]
        fn default() -> Apb1lpenr1 {
            Apb1lpenr1(0)
        }
    }
    impl core::fmt::Debug for Apb1lpenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lpenr1")
                .field("tim2lpen", &self.tim2lpen())
                .field("tim3lpen", &self.tim3lpen())
                .field("tim4lpen", &self.tim4lpen())
                .field("tim5lpen", &self.tim5lpen())
                .field("tim6lpen", &self.tim6lpen())
                .field("tim7lpen", &self.tim7lpen())
                .field("tim12lpen", &self.tim12lpen())
                .field("tim13lpen", &self.tim13lpen())
                .field("tim14lpen", &self.tim14lpen())
                .field("lptim1lpen", &self.lptim1lpen())
                .field("wwdglpen", &self.wwdglpen())
                .field("spi2lpen", &self.spi2lpen())
                .field("spi3lpen", &self.spi3lpen())
                .field("spdifrxlpen", &self.spdifrxlpen())
                .field("usart2lpen", &self.usart2lpen())
                .field("usart3lpen", &self.usart3lpen())
                .field("uart4lpen", &self.uart4lpen())
                .field("uart5lpen", &self.uart5lpen())
                .field("i2c1_i3c1lpen", &self.i2c1_i3c1lpen())
                .field("i2c2lpen", &self.i2c2lpen())
                .field("i2c3lpen", &self.i2c3lpen())
                .field("ceclpen", &self.ceclpen())
                .field("uart7lpen", &self.uart7lpen())
                .field("uart8lpen", &self.uart8lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lpenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1lpenr1 {{ tim2lpen: {=bool:?}, tim3lpen: {=bool:?}, tim4lpen: {=bool:?}, tim5lpen: {=bool:?}, tim6lpen: {=bool:?}, tim7lpen: {=bool:?}, tim12lpen: {=bool:?}, tim13lpen: {=bool:?}, tim14lpen: {=bool:?}, lptim1lpen: {=bool:?}, wwdglpen: {=bool:?}, spi2lpen: {=bool:?}, spi3lpen: {=bool:?}, spdifrxlpen: {=bool:?}, usart2lpen: {=bool:?}, usart3lpen: {=bool:?}, uart4lpen: {=bool:?}, uart5lpen: {=bool:?}, i2c1_i3c1lpen: {=bool:?}, i2c2lpen: {=bool:?}, i2c3lpen: {=bool:?}, ceclpen: {=bool:?}, uart7lpen: {=bool:?}, uart8lpen: {=bool:?} }}" , self . tim2lpen () , self . tim3lpen () , self . tim4lpen () , self . tim5lpen () , self . tim6lpen () , self . tim7lpen () , self . tim12lpen () , self . tim13lpen () , self . tim14lpen () , self . lptim1lpen () , self . wwdglpen () , self . spi2lpen () , self . spi3lpen () , self . spdifrxlpen () , self . usart2lpen () , self . usart3lpen () , self . uart4lpen () , self . uart5lpen () , self . i2c1_i3c1lpen () , self . i2c2lpen () , self . i2c3lpen () , self . ceclpen () , self . uart7lpen () , self . uart8lpen ())
        }
    }
    #[doc = "RCC APB1 low-power clock enable register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lpenr2(pub u32);
    impl Apb1lpenr2 {
        #[doc = "clock recovery system peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock recovery system peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MDIOS peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn mdioslpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_mdioslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_fdcanlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UCPD peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn ucpdlpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_ucpdlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Apb1lpenr2 {
        #[inline(always)]
        fn default() -> Apb1lpenr2 {
            Apb1lpenr2(0)
        }
    }
    impl core::fmt::Debug for Apb1lpenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lpenr2")
                .field("crslpen", &self.crslpen())
                .field("mdioslpen", &self.mdioslpen())
                .field("fdcanlpen", &self.fdcanlpen())
                .field("ucpdlpen", &self.ucpdlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lpenr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1lpenr2 {{ crslpen: {=bool:?}, mdioslpen: {=bool:?}, fdcanlpen: {=bool:?}, ucpdlpen: {=bool:?} }}",
                self.crslpen(),
                self.mdioslpen(),
                self.fdcanlpen(),
                self.ucpdlpen()
            )
        }
    }
    #[doc = "RCC APB1 peripherals kernel clock selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1perckselr(pub u32);
    impl Apb1perckselr {
        #[doc = "USART2,3, UART4,5,7,8 (APB1) kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn usart234578sel(&self) -> super::vals::Usart234578sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Usart234578sel::from_bits(val as u8)
        }
        #[doc = "USART2,3, UART4,5,7,8 (APB1) kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_usart234578sel(&mut self, val: super::vals::Usart234578sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI/I2S2 and SPI/I2S3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
        #[inline(always)]
        pub const fn spi23sel(&self) -> super::vals::Spi123sel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Spi123sel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S2 and SPI/I2S3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
        #[inline(always)]
        pub fn set_spi23sel(&mut self, val: super::vals::Spi123sel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "I2C2, I2C3 kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub const fn i2c23sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C2, I2C3 kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c23sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I2C1 or I3C1 kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub const fn i2c1_i3c1sel(&self) -> super::vals::I2c1I3c1sel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I2c1I3c1sel::from_bits(val as u8)
        }
        #[doc = "I2C1 or I3C1 kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c1_i3c1sel(&mut self, val: super::vals::I2c1I3c1sel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "LPTIM1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Lptim1sel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "FDCAN kernel clock source selection."]
        #[inline(always)]
        pub const fn fdcansel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN kernel clock source selection."]
        #[inline(always)]
        pub fn set_fdcansel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "SPDIFRX kernel clock source selection."]
        #[inline(always)]
        pub const fn spdifrxsel(&self) -> super::vals::Spdifrxsel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Spdifrxsel::from_bits(val as u8)
        }
        #[doc = "SPDIFRX kernel clock source selection."]
        #[inline(always)]
        pub fn set_spdifrxsel(&mut self, val: super::vals::Spdifrxsel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "HDMI-CEC kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub const fn cecsel(&self) -> super::vals::Cecsel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Cecsel::from_bits(val as u8)
        }
        #[doc = "HDMI-CEC kernel clock source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_cecsel(&mut self, val: super::vals::Cecsel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Apb1perckselr {
        #[inline(always)]
        fn default() -> Apb1perckselr {
            Apb1perckselr(0)
        }
    }
    impl core::fmt::Debug for Apb1perckselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1perckselr")
                .field("usart234578sel", &self.usart234578sel())
                .field("spi23sel", &self.spi23sel())
                .field("i2c23sel", &self.i2c23sel())
                .field("i2c1_i3c1sel", &self.i2c1_i3c1sel())
                .field("lptim1sel", &self.lptim1sel())
                .field("fdcansel", &self.fdcansel())
                .field("spdifrxsel", &self.spdifrxsel())
                .field("cecsel", &self.cecsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1perckselr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1perckselr {{ usart234578sel: {:?}, spi23sel: {:?}, i2c23sel: {:?}, i2c1_i3c1sel: {:?}, lptim1sel: {:?}, fdcansel: {:?}, spdifrxsel: {:?}, cecsel: {:?} }}" , self . usart234578sel () , self . spi23sel () , self . i2c23sel () , self . i2c1_i3c1sel () , self . lptim1sel () , self . fdcansel () , self . spdifrxsel () , self . cecsel ())
        }
    }
    #[doc = "RCC APB1 peripheral reset register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr1(pub u32);
    impl Apb1rstr1 {
        #[doc = "TIM2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SPI2S2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI2S3 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S3 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spdifrxrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spdifrxrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1/I3C1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn i2c1_i3c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1/I3C1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c1_i3c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "HDMI-CEC block reset Set and reset by software."]
        #[inline(always)]
        pub const fn cecrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_cecrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "UART7 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn uart7rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_uart7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn uart8rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_uart8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("tim5rst", &self.tim5rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("tim12rst", &self.tim12rst())
                .field("tim13rst", &self.tim13rst())
                .field("tim14rst", &self.tim14rst())
                .field("lptim1rst", &self.lptim1rst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("spdifrxrst", &self.spdifrxrst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1_i3c1rst", &self.i2c1_i3c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("cecrst", &self.cecrst())
                .field("uart7rst", &self.uart7rst())
                .field("uart8rst", &self.uart8rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1rstr1 {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim4rst: {=bool:?}, tim5rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, tim12rst: {=bool:?}, tim13rst: {=bool:?}, tim14rst: {=bool:?}, lptim1rst: {=bool:?}, spi2rst: {=bool:?}, spi3rst: {=bool:?}, spdifrxrst: {=bool:?}, usart2rst: {=bool:?}, usart3rst: {=bool:?}, uart4rst: {=bool:?}, uart5rst: {=bool:?}, i2c1_i3c1rst: {=bool:?}, i2c2rst: {=bool:?}, i2c3rst: {=bool:?}, cecrst: {=bool:?}, uart7rst: {=bool:?}, uart8rst: {=bool:?} }}" , self . tim2rst () , self . tim3rst () , self . tim4rst () , self . tim5rst () , self . tim6rst () , self . tim7rst () , self . tim12rst () , self . tim13rst () , self . tim14rst () , self . lptim1rst () , self . spi2rst () , self . spi3rst () , self . spdifrxrst () , self . usart2rst () , self . usart3rst () , self . uart4rst () , self . uart5rst () , self . i2c1_i3c1rst () , self . i2c2rst () , self . i2c3rst () , self . cecrst () , self . uart7rst () , self . uart8rst ())
        }
    }
    #[doc = "RCC APB1 peripheral reset register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr2(pub u32);
    impl Apb1rstr2 {
        #[doc = "clock recovery system reset Set and reset by software."]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock recovery system reset Set and reset by software."]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MDIOS block reset Set and reset by software."]
        #[inline(always)]
        pub const fn mdiosrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_mdiosrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN block reset Set and reset by software."]
        #[inline(always)]
        pub const fn fdcanrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_fdcanrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UCPD block reset Set and reset by software."]
        #[inline(always)]
        pub const fn ucpdrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_ucpdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("crsrst", &self.crsrst())
                .field("mdiosrst", &self.mdiosrst())
                .field("fdcanrst", &self.fdcanrst())
                .field("ucpdrst", &self.ucpdrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1rstr2 {{ crsrst: {=bool:?}, mdiosrst: {=bool:?}, fdcanrst: {=bool:?}, ucpdrst: {=bool:?} }}",
                self.crsrst(),
                self.mdiosrst(),
                self.fdcanrst(),
                self.ucpdrst()
            )
        }
    }
    #[doc = "RCC APB2 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USART1 peripheral clocks enable Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART1SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 peripheral clocks enable Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART1SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SPI2S1 Peripheral Clocks Enable Set and reset by software. The peripheral clocks of the SPI2S1 are: the kernel clock selected by SPI1SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S1 Peripheral Clocks Enable Set and reset by software. The peripheral clocks of the SPI2S1 are: the kernel clock selected by SPI1SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim9en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SPI5 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 peripheral clocks enable Set and reset by software. The peripheral clocks of the SAI1 are the kernel clock selected by SAI1SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 peripheral clocks enable Set and reset by software. The peripheral clocks of the SAI1 are the kernel clock selected by SAI1SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 peripheral clocks enable Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI2SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 peripheral clocks enable Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI2SEL, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("usart1en", &self.usart1en())
                .field("spi1en", &self.spi1en())
                .field("spi4en", &self.spi4en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("tim9en", &self.tim9en())
                .field("spi5en", &self.spi5en())
                .field("sai1en", &self.sai1en())
                .field("sai2en", &self.sai2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2enr {{ tim1en: {=bool:?}, usart1en: {=bool:?}, spi1en: {=bool:?}, spi4en: {=bool:?}, tim15en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, tim9en: {=bool:?}, spi5en: {=bool:?}, sai1en: {=bool:?}, sai2en: {=bool:?} }}" , self . tim1en () , self . usart1en () , self . spi1en () , self . spi4en () , self . tim15en () , self . tim16en () , self . tim17en () , self . tim9en () , self . spi5en () , self . sai1en () , self . sai2en ())
        }
    }
    #[doc = "RCC APB2 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USART1 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART169SEL and provided to UCLK inputs, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART169SEL and provided to UCLK inputs, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SPI2S1 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the SPI2S1 are: the kernel clock selected by I2S1SEL and provided to spi_ker_ck input, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S1 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the SPI2S1 are: the kernel clock selected by I2S1SEL and provided to spi_ker_ck input, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to com_clk input, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn spi4lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to com_clk input, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_spi4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim9lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim9lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SPI5 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to com_clk input, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to com_clk input, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to SAI_CK_A and SAI_CK_B inputs, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn sai1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to SAI_CK_A and SAI_CK_B inputs, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_sai1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SAI2 are: the kernel clock selected by SAI2SEL and provided to SAI_CK_A and SAI_CK_B inputs, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub const fn sai2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SAI2 are: the kernel clock selected by SAI2SEL and provided to SAI_CK_A and SAI_CK_B inputs, and the pclk2 bus interface clock."]
        #[inline(always)]
        pub fn set_sai2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("usart1lpen", &self.usart1lpen())
                .field("spi1lpen", &self.spi1lpen())
                .field("spi4lpen", &self.spi4lpen())
                .field("tim15lpen", &self.tim15lpen())
                .field("tim16lpen", &self.tim16lpen())
                .field("tim17lpen", &self.tim17lpen())
                .field("tim9lpen", &self.tim9lpen())
                .field("spi5lpen", &self.spi5lpen())
                .field("sai1lpen", &self.sai1lpen())
                .field("sai2lpen", &self.sai2lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2lpenr {{ tim1lpen: {=bool:?}, usart1lpen: {=bool:?}, spi1lpen: {=bool:?}, spi4lpen: {=bool:?}, tim15lpen: {=bool:?}, tim16lpen: {=bool:?}, tim17lpen: {=bool:?}, tim9lpen: {=bool:?}, spi5lpen: {=bool:?}, sai1lpen: {=bool:?}, sai2lpen: {=bool:?} }}" , self . tim1lpen () , self . usart1lpen () , self . spi1lpen () , self . spi4lpen () , self . tim15lpen () , self . tim16lpen () , self . tim17lpen () , self . tim9lpen () , self . spi5lpen () , self . sai1lpen () , self . sai2lpen ())
        }
    }
    #[doc = "RCC APB2 peripherals kernel clock selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2perckselr(pub u32);
    impl Apb2perckselr {
        #[doc = "USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usart1sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Usart1sel::from_bits(val as u8)
        }
        #[doc = "USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: super::vals::Usart1sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn spi45sel(&self) -> super::vals::Spi45sel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Spi45sel::from_bits(val as u8)
        }
        #[doc = "SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_spi45sel(&mut self, val: super::vals::Spi45sel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "SPI/I2S1 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spi123sel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Spi123sel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S1 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
        #[inline(always)]
        pub fn set_spi1sel(&mut self, val: super::vals::Spi123sel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "SAI1 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Sai1sel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Sai1sel::from_bits(val as u8)
        }
        #[doc = "SAI1 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: super::vals::Sai1sel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "SAI2 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see Figure 51)."]
        #[inline(always)]
        pub const fn sai2sel(&self) -> super::vals::Sai2sel {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Sai2sel::from_bits(val as u8)
        }
        #[doc = "SAI2 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see Figure 51)."]
        #[inline(always)]
        pub fn set_sai2sel(&mut self, val: super::vals::Sai2sel) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
    }
    impl Default for Apb2perckselr {
        #[inline(always)]
        fn default() -> Apb2perckselr {
            Apb2perckselr(0)
        }
    }
    impl core::fmt::Debug for Apb2perckselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2perckselr")
                .field("usart1sel", &self.usart1sel())
                .field("spi45sel", &self.spi45sel())
                .field("spi1sel", &self.spi1sel())
                .field("sai1sel", &self.sai1sel())
                .field("sai2sel", &self.sai2sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2perckselr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2perckselr {{ usart1sel: {:?}, spi45sel: {:?}, spi1sel: {:?}, sai1sel: {:?}, sai2sel: {:?} }}",
                self.usart1sel(),
                self.spi45sel(),
                self.spi1sel(),
                self.sai1sel(),
                self.sai2sel()
            )
        }
    }
    #[doc = "RCC APB2 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USART1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SPI2S1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim9rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SPI5 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("usart1rst", &self.usart1rst())
                .field("spi1rst", &self.spi1rst())
                .field("spi4rst", &self.spi4rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("tim9rst", &self.tim9rst())
                .field("spi5rst", &self.spi5rst())
                .field("sai1rst", &self.sai1rst())
                .field("sai2rst", &self.sai2rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2rstr {{ tim1rst: {=bool:?}, usart1rst: {=bool:?}, spi1rst: {=bool:?}, spi4rst: {=bool:?}, tim15rst: {=bool:?}, tim16rst: {=bool:?}, tim17rst: {=bool:?}, tim9rst: {=bool:?}, spi5rst: {=bool:?}, sai1rst: {=bool:?}, sai2rst: {=bool:?} }}" , self . tim1rst () , self . usart1rst () , self . spi1rst () , self . spi4rst () , self . tim15rst () , self . tim16rst () , self . tim17rst () , self . tim9rst () , self . spi5rst () , self . sai1rst () , self . sai2rst ())
        }
    }
    #[doc = "RCC APB4,5 peripherals kernel clock selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb45perckselr(pub u32);
    impl Apb45perckselr {
        #[doc = "LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI/I2S6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn spi6sel(&self) -> super::vals::Spi6sel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Spi6sel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_spi6sel(&mut self, val: super::vals::Spi6sel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "LPTIM2 and LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn lptim23sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM2 and LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_lptim23sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "LPTIM4, and LPTIM5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub const fn lptim45sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM4, and LPTIM5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled."]
        #[inline(always)]
        pub fn set_lptim45sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Apb45perckselr {
        #[inline(always)]
        fn default() -> Apb45perckselr {
            Apb45perckselr(0)
        }
    }
    impl core::fmt::Debug for Apb45perckselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb45perckselr")
                .field("lpuart1sel", &self.lpuart1sel())
                .field("spi6sel", &self.spi6sel())
                .field("lptim23sel", &self.lptim23sel())
                .field("lptim45sel", &self.lptim45sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb45perckselr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb45perckselr {{ lpuart1sel: {:?}, spi6sel: {:?}, lptim23sel: {:?}, lptim45sel: {:?} }}",
                self.lpuart1sel(),
                self.spi6sel(),
                self.lptim23sel(),
                self.lptim45sel()
            )
        }
    }
    #[doc = "RCC APB4 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4enr(pub u32);
    impl Apb4enr {
        #[doc = "SBS peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 peripheral clocks enable Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to UCLK input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 peripheral clocks enable Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to UCLK input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI/I2S6 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI/I2S6 are the kernel clock selected by SPI6SEL and provided to com_clk input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub const fn spi6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI/I2S6 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI/I2S6 are the kernel clock selected by SPI6SEL and provided to com_clk input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub fn set_spi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM2 peripheral clocks enable Set and reset by software. The LPTIM2 kernel clock can be selected by LPTIM23SEL."]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 peripheral clocks enable Set and reset by software. The LPTIM2 kernel clock can be selected by LPTIM23SEL."]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 peripheral clocks enable Set and reset by software. The LPTIM3 kernel clock can be selected by LPTIM23SEL."]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 peripheral clocks enable Set and reset by software. The LPTIM3 kernel clock can be selected by LPTIM23SEL."]
        #[inline(always)]
        pub fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 peripheral clocks enable Set and reset by software. The LPTIM4 kernel clock can be selected by LPTIM45SEL."]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 peripheral clocks enable Set and reset by software. The LPTIM4 kernel clock can be selected by LPTIM45SEL."]
        #[inline(always)]
        pub fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 peripheral clocks enable Set and reset by software. The LPTIM5 kernel clock can be selected by LPTIM45SEL."]
        #[inline(always)]
        pub const fn lptim5en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 peripheral clocks enable Set and reset by software. The LPTIM5 kernel clock can be selected by LPTIM45SEL."]
        #[inline(always)]
        pub fn set_lptim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VREF peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Temperature Sensor peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tmpsensen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Sensor peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tmpsensen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4enr {
        #[inline(always)]
        fn default() -> Apb4enr {
            Apb4enr(0)
        }
    }
    impl core::fmt::Debug for Apb4enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb4enr")
                .field("syscfgen", &self.syscfgen())
                .field("lpuart1en", &self.lpuart1en())
                .field("spi6en", &self.spi6en())
                .field("lptim2en", &self.lptim2en())
                .field("lptim3en", &self.lptim3en())
                .field("lptim4en", &self.lptim4en())
                .field("lptim5en", &self.lptim5en())
                .field("vrefen", &self.vrefen())
                .field("rtcapben", &self.rtcapben())
                .field("tmpsensen", &self.tmpsensen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb4enr {{ syscfgen: {=bool:?}, lpuart1en: {=bool:?}, spi6en: {=bool:?}, lptim2en: {=bool:?}, lptim3en: {=bool:?}, lptim4en: {=bool:?}, lptim5en: {=bool:?}, vrefen: {=bool:?}, rtcapben: {=bool:?}, tmpsensen: {=bool:?} }}" , self . syscfgen () , self . lpuart1en () , self . spi6en () , self . lptim2en () , self . lptim3en () , self . lptim4en () , self . lptim5en () , self . vrefen () , self . rtcapben () , self . tmpsensen ())
        }
    }
    #[doc = "RCC APB4 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4lpenr(pub u32);
    impl Apb4lpenr {
        #[doc = "SBS peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to UCLK input, and the rcc_pclk4 bus interface clock."]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to UCLK input, and the rcc_pclk4 bus interface clock."]
        #[inline(always)]
        pub fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI/I2S6 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SPI/I2S6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock."]
        #[inline(always)]
        pub const fn spi6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI/I2S6 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SPI/I2S6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock."]
        #[inline(always)]
        pub fn set_spi6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM23SEL and provided to clk_lpt input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub const fn lptim2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM23SEL and provided to clk_lpt input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub fn set_lptim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM23SEL and provided to clk_lpt input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub const fn lptim3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM23SEL and provided to clk_lpt input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub fn set_lptim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM4 are the kernel clock selected by LPTIM45SEL and provided to clk_lpt input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub const fn lptim4lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM4 are the kernel clock selected by LPTIM45SEL and provided to clk_lpt input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub fn set_lptim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM5 are the kernel clock selected by LPTIM45SEL and provided to clk_lpt input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub const fn lptim5lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM5 are the kernel clock selected by LPTIM45SEL and provided to clk_lpt input, and the pclk4 bus interface clock."]
        #[inline(always)]
        pub fn set_lptim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VREF peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn tmpsenslpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tmpsenslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4lpenr {
        #[inline(always)]
        fn default() -> Apb4lpenr {
            Apb4lpenr(0)
        }
    }
    impl core::fmt::Debug for Apb4lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb4lpenr")
                .field("syscfglpen", &self.syscfglpen())
                .field("lpuart1lpen", &self.lpuart1lpen())
                .field("spi6lpen", &self.spi6lpen())
                .field("lptim2lpen", &self.lptim2lpen())
                .field("lptim3lpen", &self.lptim3lpen())
                .field("lptim4lpen", &self.lptim4lpen())
                .field("lptim5lpen", &self.lptim5lpen())
                .field("vreflpen", &self.vreflpen())
                .field("rtcapblpen", &self.rtcapblpen())
                .field("tmpsenslpen", &self.tmpsenslpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb4lpenr {{ syscfglpen: {=bool:?}, lpuart1lpen: {=bool:?}, spi6lpen: {=bool:?}, lptim2lpen: {=bool:?}, lptim3lpen: {=bool:?}, lptim4lpen: {=bool:?}, lptim5lpen: {=bool:?}, vreflpen: {=bool:?}, rtcapblpen: {=bool:?}, tmpsenslpen: {=bool:?} }}" , self . syscfglpen () , self . lpuart1lpen () , self . spi6lpen () , self . lptim2lpen () , self . lptim3lpen () , self . lptim4lpen () , self . lptim5lpen () , self . vreflpen () , self . rtcapblpen () , self . tmpsenslpen ())
        }
    }
    #[doc = "RCC APB4 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4rstr(pub u32);
    impl Apb4rstr {
        #[doc = "SBS block reset Set and reset by software."]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI/I2S6 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI/I2S6 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lptim4rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lptim5rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VREF block reset Set and reset by software."]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TMPSENS block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tmpsensrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "TMPSENS block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tmpsensrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4rstr {
        #[inline(always)]
        fn default() -> Apb4rstr {
            Apb4rstr(0)
        }
    }
    impl core::fmt::Debug for Apb4rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb4rstr")
                .field("syscfgrst", &self.syscfgrst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("spi6rst", &self.spi6rst())
                .field("lptim2rst", &self.lptim2rst())
                .field("lptim3rst", &self.lptim3rst())
                .field("lptim4rst", &self.lptim4rst())
                .field("lptim5rst", &self.lptim5rst())
                .field("vrefrst", &self.vrefrst())
                .field("tmpsensrst", &self.tmpsensrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb4rstr {{ syscfgrst: {=bool:?}, lpuart1rst: {=bool:?}, spi6rst: {=bool:?}, lptim2rst: {=bool:?}, lptim3rst: {=bool:?}, lptim4rst: {=bool:?}, lptim5rst: {=bool:?}, vrefrst: {=bool:?}, tmpsensrst: {=bool:?} }}" , self . syscfgrst () , self . lpuart1rst () , self . spi6rst () , self . lptim2rst () , self . lptim3rst () , self . lptim4rst () , self . lptim5rst () , self . vrefrst () , self . tmpsensrst ())
        }
    }
    #[doc = "RCC APB5 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb5enr(pub u32);
    impl Apb5enr {
        #[doc = "LTDC peripheral clock enable Provides the pixel clock (ltdc_clk) to the LTDC block. Set and reset by software."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable Provides the pixel clock (ltdc_clk) to the LTDC block. Set and reset by software."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DCMIPP peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn dcmippen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DCMIPP peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_dcmippen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GFXTIM peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gfxtimen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gfxtimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb5enr {
        #[inline(always)]
        fn default() -> Apb5enr {
            Apb5enr(0)
        }
    }
    impl core::fmt::Debug for Apb5enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb5enr")
                .field("ltdcen", &self.ltdcen())
                .field("dcmippen", &self.dcmippen())
                .field("gfxtimen", &self.gfxtimen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb5enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb5enr {{ ltdcen: {=bool:?}, dcmippen: {=bool:?}, gfxtimen: {=bool:?} }}",
                self.ltdcen(),
                self.dcmippen(),
                self.gfxtimen()
            )
        }
    }
    #[doc = "RCC APB5 sleep clock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb5lpenr(pub u32);
    impl Apb5lpenr {
        #[doc = "LTDC peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn ltdclpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_ltdclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DCMIPP peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn dcmipplpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DCMIPP peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_dcmipplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GFXTIM peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub const fn gfxtimlpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM peripheral clock enable in low-power mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gfxtimlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb5lpenr {
        #[inline(always)]
        fn default() -> Apb5lpenr {
            Apb5lpenr(0)
        }
    }
    impl core::fmt::Debug for Apb5lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb5lpenr")
                .field("ltdclpen", &self.ltdclpen())
                .field("dcmipplpen", &self.dcmipplpen())
                .field("gfxtimlpen", &self.gfxtimlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb5lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb5lpenr {{ ltdclpen: {=bool:?}, dcmipplpen: {=bool:?}, gfxtimlpen: {=bool:?} }}",
                self.ltdclpen(),
                self.dcmipplpen(),
                self.gfxtimlpen()
            )
        }
    }
    #[doc = "RCC APB5 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb5rstr(pub u32);
    impl Apb5rstr {
        #[doc = "LTDC block reset Set and reset by software."]
        #[inline(always)]
        pub const fn ltdcrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_ltdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DCMIPP block reset Set and reset by software."]
        #[inline(always)]
        pub const fn dcmipprst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DCMIPP block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_dcmipprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GFXTIM block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gfxtimrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gfxtimrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb5rstr {
        #[inline(always)]
        fn default() -> Apb5rstr {
            Apb5rstr(0)
        }
    }
    impl core::fmt::Debug for Apb5rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb5rstr")
                .field("ltdcrst", &self.ltdcrst())
                .field("dcmipprst", &self.dcmipprst())
                .field("gfxtimrst", &self.gfxtimrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb5rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb5rstr {{ ltdcrst: {=bool:?}, dcmipprst: {=bool:?}, gfxtimrst: {=bool:?} }}",
                self.ltdcrst(),
                self.dcmipprst(),
                self.gfxtimrst()
            )
        }
    }
    #[doc = "RCC APB clocks configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apbcfgr(pub u32);
    impl Apbcfgr {
        #[doc = "CPU domain APB1 prescaler Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE1 write. 0xx: rcc_pclk1 = sys_bus_ck (default after reset)."]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "CPU domain APB1 prescaler Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE1 write. 0xx: rcc_pclk1 = sys_bus_ck (default after reset)."]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "CPU domain APB2 prescaler Set and reset by software to control the division factor of rcc_pclk2. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE2 write. 0xx: rcc_pclk2 = sys_bus_ck (default after reset)."]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "CPU domain APB2 prescaler Set and reset by software to control the division factor of rcc_pclk2. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE2 write. 0xx: rcc_pclk2 = sys_bus_ck (default after reset)."]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "CPU domain APB4 prescaler Set and reset by software to control the division factor of rcc_pclk4. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE4 write. 0xx: rcc_pclk4 = sys_bus_ck (default after reset)."]
        #[inline(always)]
        pub const fn ppre4(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "CPU domain APB4 prescaler Set and reset by software to control the division factor of rcc_pclk4. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE4 write. 0xx: rcc_pclk4 = sys_bus_ck (default after reset)."]
        #[inline(always)]
        pub fn set_ppre4(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "CPU domain APB5 prescaler Set and reset by software to control the division factor of rcc_pclk5. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE5 write. 0xx: rcc_pclk5 = sys_bus_ck (default after reset)."]
        #[inline(always)]
        pub const fn ppre5(&self) -> super::vals::Ppre {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "CPU domain APB5 prescaler Set and reset by software to control the division factor of rcc_pclk5. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE5 write. 0xx: rcc_pclk5 = sys_bus_ck (default after reset)."]
        #[inline(always)]
        pub fn set_ppre5(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Apbcfgr {
        #[inline(always)]
        fn default() -> Apbcfgr {
            Apbcfgr(0)
        }
    }
    impl core::fmt::Debug for Apbcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apbcfgr")
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("ppre4", &self.ppre4())
                .field("ppre5", &self.ppre5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apbcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apbcfgr {{ ppre1: {:?}, ppre2: {:?}, ppre4: {:?}, ppre5: {:?} }}",
                self.ppre1(),
                self.ppre2(),
                self.ppre4(),
                self.ppre5()
            )
        }
    }
    #[doc = "RCC Backup domain control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enabled Set and reset by software."]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enabled Set and reset by software."]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)."]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)."]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator."]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator."]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit can only be disabled, After a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON. After a back-up domain reset."]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit can only be disabled, After a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON. After a back-up domain reset."]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "low-speed external clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
        #[inline(always)]
        pub const fn lseext(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "low-speed external clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
        #[inline(always)]
        pub fn set_lseext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Re-Arm the LSECSS function Set by software. After a LSE failure detection, the software application can re-enable the LSECSS by writing this bit to 1. Reading this bit returns the written value. Prior to set this bit to 1, LSECSSON must be set to 0. Please refer to Section : CSS on LSE for details."]
        #[inline(always)]
        pub const fn lsecssra(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Re-Arm the LSECSS function Set by software. After a LSE failure detection, the software application can re-enable the LSECSS by writing this bit to 1. Reading this bit returns the written value. Prior to set this bit to 1, LSECSSON must be set to 0. Please refer to Section : CSS on LSE for details."]
        #[inline(always)]
        pub fn set_lsecssra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RTC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VSwitch domain software reset Set and reset by software. To generate a VSW reset, it is recommended to write this bit to 1, then back to 0."]
        #[inline(always)]
        pub const fn vswrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VSwitch domain software reset Set and reset by software. To generate a VSW reset, it is recommended to write this bit to 1, then back to 0."]
        #[inline(always)]
        pub fn set_vswrst(&mut self, val: bool) {
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
                .field("lsedrv", &self.lsedrv())
                .field("lsecsson", &self.lsecsson())
                .field("lsecssd", &self.lsecssd())
                .field("lseext", &self.lseext())
                .field("rtcsel", &self.rtcsel())
                .field("lsecssra", &self.lsecssra())
                .field("rtcen", &self.rtcen())
                .field("vswrst", &self.vswrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, lsedrv: {:?}, lsecsson: {=bool:?}, lsecssd: {=bool:?}, lseext: {=bool:?}, rtcsel: {:?}, lsecssra: {=bool:?}, rtcen: {=bool:?}, vswrst: {=bool:?} }}" , self . lseon () , self . lserdy () , self . lsebyp () , self . lsedrv () , self . lsecsson () , self . lsecssd () , self . lseext () , self . rtcsel () , self . lsecssra () , self . rtcen () , self . vswrst ())
        }
    }
    #[doc = "RCC AHB clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmcfgr(pub u32);
    impl Bmcfgr {
        #[doc = "Bus matrix clock prescaler Set and reset by software to control the division factor of rcc_hclk\\[5:1\\]
and rcc_aclk. This group of clocks is also named sys_bus_ck. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: sys_bus_ck= sys_cpu_ck (default after reset) Note: The clocks are divided by the new prescaler factor from 1 to 16 periods of the slowest APB clock among rcc_pclk1,2,4,5 after BMPRE update. Note: Note also that frequency of rcc_hclk\\[5:1\\]
= rcc_aclk = sys_bus_ck."]
        #[inline(always)]
        pub const fn bmpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "Bus matrix clock prescaler Set and reset by software to control the division factor of rcc_hclk\\[5:1\\]
and rcc_aclk. This group of clocks is also named sys_bus_ck. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: sys_bus_ck= sys_cpu_ck (default after reset) Note: The clocks are divided by the new prescaler factor from 1 to 16 periods of the slowest APB clock among rcc_pclk1,2,4,5 after BMPRE update. Note: Note also that frequency of rcc_hclk\\[5:1\\]
= rcc_aclk = sys_bus_ck."]
        #[inline(always)]
        pub fn set_bmpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Bmcfgr {
        #[inline(always)]
        fn default() -> Bmcfgr {
            Bmcfgr(0)
        }
    }
    impl core::fmt::Debug for Bmcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmcfgr").field("bmpre", &self.bmpre()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bmcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bmcfgr {{ bmpre: {:?} }}", self.bmpre())
        }
    }
    #[doc = "RCC CPU domain clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdcfgr(pub u32);
    impl Cdcfgr {
        #[doc = "CPU domain core prescaler Set and reset by software to control the CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)."]
        #[inline(always)]
        pub const fn cpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "CPU domain core prescaler Set and reset by software to control the CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)."]
        #[inline(always)]
        pub fn set_cpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Cdcfgr {
        #[inline(always)]
        fn default() -> Cdcfgr {
            Cdcfgr(0)
        }
    }
    impl core::fmt::Debug for Cdcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdcfgr").field("cpre", &self.cpre()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cdcfgr {{ cpre: {:?} }}", self.cpre())
        }
    }
    #[doc = "RCC clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "system clock switch Set and reset by software to select system clock source (sys_ck). Set by hardware in order to force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode or in case of failure of the HSE when used directly or indirectly as system clock. others: reserved."]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch Set and reset by software to select system clock source (sys_ck). Set by hardware in order to force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode or in case of failure of the HSE when used directly or indirectly as system clock. others: reserved."]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. others: reserved."]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. others: reserved."]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "system clock selection after a wake up from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. See Section 1.: Dividers values can be changed on-the-fly. All dividers provide have 50% duty-cycles. for details. STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "system clock selection after a wake up from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. See Section 1.: Dividers values can be changed on-the-fly. All dividers provide have 50% duty-cycles. for details. STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "kernel clock selection after a wake up from system Stop Set and reset by software to select the kernel wakeup clock from system Stop. See Section 1.: Dividers values can be changed on-the-fly. All dividers provide have 50% duty-cycles. for details."]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> super::vals::Stopkerwuck {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Stopkerwuck::from_bits(val as u8)
        }
        #[doc = "kernel clock selection after a wake up from system Stop Set and reset by software to select the kernel wakeup clock from system Stop. See Section 1.: Dividers values can be changed on-the-fly. All dividers provide have 50% duty-cycles. for details."]
        #[inline(always)]
        pub fn set_stopkerwuck(&mut self, val: super::vals::Stopkerwuck) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
        #[inline(always)]
        pub fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains. or 4, else it is equal to 4 x F<sub>rcc_pclkx_d2</sub> Refer to Table 64: Ratio between clock timer and pclk for more details."]
        #[inline(always)]
        pub const fn timpre(&self) -> super::vals::Timpre {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Timpre::from_bits(val as u8)
        }
        #[doc = "timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains. or 4, else it is equal to 4 x F<sub>rcc_pclkx_d2</sub> Refer to Table 64: Ratio between clock timer and pclk for more details."]
        #[inline(always)]
        pub fn set_timpre(&mut self, val: super::vals::Timpre) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
        #[inline(always)]
        pub fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved."]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 22usize) & 0x07;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved."]
        #[inline(always)]
        pub fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
        }
        #[doc = "MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 25usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
        #[inline(always)]
        pub fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
        }
        #[doc = "microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved."]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved."]
        #[inline(always)]
        pub fn set_mco2sel(&mut self, val: super::vals::Mco2sel) {
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
                .field("sw", &self.sw())
                .field("sws", &self.sws())
                .field("stopwuck", &self.stopwuck())
                .field("stopkerwuck", &self.stopkerwuck())
                .field("rtcpre", &self.rtcpre())
                .field("timpre", &self.timpre())
                .field("mco1pre", &self.mco1pre())
                .field("mco1sel", &self.mco1sel())
                .field("mco2pre", &self.mco2pre())
                .field("mco2sel", &self.mco2sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ sw: {:?}, sws: {:?}, stopwuck: {:?}, stopkerwuck: {:?}, rtcpre: {=u8:?}, timpre: {:?}, mco1pre: {:?}, mco1sel: {:?}, mco2pre: {:?}, mco2sel: {:?} }}" , self . sw () , self . sws () , self . stopwuck () , self . stopkerwuck () , self . rtcpre () , self . timpre () , self . mco1pre () , self . mco1sel () , self . mco2pre () , self . mco2sel ())
        }
    }
    #[doc = "RCC clock source interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn csirdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_csirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn pllrdyc(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn hsecssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_hsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("csirdyc", &self.csirdyc())
                .field("hsi48rdyc", &self.hsi48rdyc())
                .field("pllrdyc[0]", &self.pllrdyc(0usize))
                .field("pllrdyc[1]", &self.pllrdyc(1usize))
                .field("pllrdyc[2]", &self.pllrdyc(2usize))
                .field("lsecssc", &self.lsecssc())
                .field("hsecssc", &self.hsecssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cicr {{ lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, csirdyc: {=bool:?}, hsi48rdyc: {=bool:?}, pllrdyc[0]: {=bool:?}, pllrdyc[1]: {=bool:?}, pllrdyc[2]: {=bool:?}, lsecssc: {=bool:?}, hsecssc: {=bool:?} }}" , self . lsirdyc () , self . lserdyc () , self . hsirdyc () , self . hserdyc () , self . csirdyc () , self . hsi48rdyc () , self . pllrdyc (0usize) , self . pllrdyc (1usize) , self . pllrdyc (2usize) , self . lsecssc () , self . hsecssc ())
        }
    }
    #[doc = "RCC clock source interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
        #[inline(always)]
        pub const fn csirdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_csirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
        #[inline(always)]
        pub const fn pllrdyie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system interrupt enable Set and reset by software to enable/disable interrupt caused by the clock security system (CSS) on external 32 kHz oscillator."]
        #[inline(always)]
        pub const fn lsecssie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt enable Set and reset by software to enable/disable interrupt caused by the clock security system (CSS) on external 32 kHz oscillator."]
        #[inline(always)]
        pub fn set_lsecssie(&mut self, val: bool) {
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
                .field("csirdyie", &self.csirdyie())
                .field("hsi48rdyie", &self.hsi48rdyie())
                .field("pllrdyie[0]", &self.pllrdyie(0usize))
                .field("pllrdyie[1]", &self.pllrdyie(1usize))
                .field("pllrdyie[2]", &self.pllrdyie(2usize))
                .field("lsecssie", &self.lsecssie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cier {{ lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, csirdyie: {=bool:?}, hsi48rdyie: {=bool:?}, pllrdyie[0]: {=bool:?}, pllrdyie[1]: {=bool:?}, pllrdyie[2]: {=bool:?}, lsecssie: {=bool:?} }}" , self . lsirdyie () , self . lserdyie () , self . hsirdyie () , self . hserdyie () , self . csirdyie () , self . hsi48rdyie () , self . pllrdyie (0usize) , self . pllrdyie (1usize) , self . pllrdyie (2usize) , self . lsecssie ())
        }
    }
    #[doc = "RCC clock source interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
        #[inline(always)]
        pub const fn csirdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
        #[inline(always)]
        pub fn set_csirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
        #[inline(always)]
        pub const fn pllrdyf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system interrupt flag Reset by software by writing LSECSSC bit. Set by hardware when a failure is detected on the external 32 kHz oscillator and LSECSSIE is set."]
        #[inline(always)]
        pub const fn lsecssf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt flag Reset by software by writing LSECSSC bit. Set by hardware when a failure is detected on the external 32 kHz oscillator and LSECSSIE is set."]
        #[inline(always)]
        pub fn set_lsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
        #[inline(always)]
        pub const fn hsecssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
        #[inline(always)]
        pub fn set_hsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("csirdyf", &self.csirdyf())
                .field("hsi48rdyf", &self.hsi48rdyf())
                .field("pllrdyf[0]", &self.pllrdyf(0usize))
                .field("pllrdyf[1]", &self.pllrdyf(1usize))
                .field("pllrdyf[2]", &self.pllrdyf(2usize))
                .field("lsecssf", &self.lsecssf())
                .field("hsecssf", &self.hsecssf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cifr {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, csirdyf: {=bool:?}, hsi48rdyf: {=bool:?}, pllrdyf[0]: {=bool:?}, pllrdyf[1]: {=bool:?}, pllrdyf[2]: {=bool:?}, lsecssf: {=bool:?}, hsecssf: {=bool:?} }}" , self . lsirdyf () , self . lserdyf () , self . hsirdyf () , self . hserdyf () , self . csirdyf () , self . hsi48rdyf () , self . pllrdyf (0usize) , self . pllrdyf (1usize) , self . pllrdyf (2usize) , self . lsecssf () , self . hsecssf ())
        }
    }
    #[doc = "RCC AXI clocks gating disable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckgdisr(pub u32);
    impl Ckgdisr {
        #[doc = "AXI interconnect matrix clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn axickg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AXI interconnect matrix clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_axickg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AXI master AHB clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn ahbmckg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master AHB clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_ahbmckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AXI master SDMMC1 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn sdmmc1ckg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master SDMMC1 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_sdmmc1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AXI master HPDMA1 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn hpdma1ckg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master HPDMA1 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_hpdma1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AXI master CPU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn cpuckg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master CPU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_cpuckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AXI master 0 GPU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn gpus0ckg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master 0 GPU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_gpus0ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "AXI master 1 GPU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn gpus1ckg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master 1 GPU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_gpus1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AXI master cache GPU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn gpuclckg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master cache GPU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_gpuclckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "AXI master DCMIPP clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn dcmippckg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master DCMIPP clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_dcmippckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "AXI master DMA2D clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn dma2dckg(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master DMA2D clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_dma2dckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "AXI matrix slave GFXMMU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn gfxmmusckg(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "AXI matrix slave GFXMMU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_gfxmmusckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "AXI master LTDC clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn ltdcckg(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master LTDC clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_ltdcckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "AXI master GFXMMU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn gfxmmumckg(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master GFXMMU clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_gfxmmumckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "AXI slave AHB clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn ahbsckg(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "AXI slave AHB clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_ahbsckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AXI slave FMC and MCE3 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn fmcckg(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "AXI slave FMC and MCE3 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_fmcckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AXI slave XSPI1 and MCE1 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn xspi1ckg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "AXI slave XSPI1 and MCE1 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_xspi1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AXI slave XSPI2 and MCE2 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn xspi2ckg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AXI slave XSPI2 and MCE2 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_xspi2ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AXI matrix slave SRAM4 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn axiram4ckg(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "AXI matrix slave SRAM4 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_axiram4ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "AXI matrix slave SRAM3 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn axiram3ckg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "AXI matrix slave SRAM3 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_axiram3ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "AXI slave SRAM2 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn axiram2ckg(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "AXI slave SRAM2 clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_axiram2ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "AXI slave SRAM1 / error code correction (ECC) clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn axiram1ckg(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "AXI slave SRAM1 / error code correction (ECC) clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_axiram1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "AXI slave Flash interface (FLIFT) clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn flitfckg(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "AXI slave Flash interface (FLIFT) clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_flitfckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "EXTI clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub const fn extickg(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI clock gating disable This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_extickg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "JTAG automatic clock gating disabling This bit is set and reset by software."]
        #[inline(always)]
        pub const fn jtagckg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "JTAG automatic clock gating disabling This bit is set and reset by software."]
        #[inline(always)]
        pub fn set_jtagckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ckgdisr {
        #[inline(always)]
        fn default() -> Ckgdisr {
            Ckgdisr(0)
        }
    }
    impl core::fmt::Debug for Ckgdisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckgdisr")
                .field("axickg", &self.axickg())
                .field("ahbmckg", &self.ahbmckg())
                .field("sdmmc1ckg", &self.sdmmc1ckg())
                .field("hpdma1ckg", &self.hpdma1ckg())
                .field("cpuckg", &self.cpuckg())
                .field("gpus0ckg", &self.gpus0ckg())
                .field("gpus1ckg", &self.gpus1ckg())
                .field("gpuclckg", &self.gpuclckg())
                .field("dcmippckg", &self.dcmippckg())
                .field("dma2dckg", &self.dma2dckg())
                .field("gfxmmusckg", &self.gfxmmusckg())
                .field("ltdcckg", &self.ltdcckg())
                .field("gfxmmumckg", &self.gfxmmumckg())
                .field("ahbsckg", &self.ahbsckg())
                .field("fmcckg", &self.fmcckg())
                .field("xspi1ckg", &self.xspi1ckg())
                .field("xspi2ckg", &self.xspi2ckg())
                .field("axiram4ckg", &self.axiram4ckg())
                .field("axiram3ckg", &self.axiram3ckg())
                .field("axiram2ckg", &self.axiram2ckg())
                .field("axiram1ckg", &self.axiram1ckg())
                .field("flitfckg", &self.flitfckg())
                .field("extickg", &self.extickg())
                .field("jtagckg", &self.jtagckg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ckgdisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ckgdisr {{ axickg: {=bool:?}, ahbmckg: {=bool:?}, sdmmc1ckg: {=bool:?}, hpdma1ckg: {=bool:?}, cpuckg: {=bool:?}, gpus0ckg: {=bool:?}, gpus1ckg: {=bool:?}, gpuclckg: {=bool:?}, dcmippckg: {=bool:?}, dma2dckg: {=bool:?}, gfxmmusckg: {=bool:?}, ltdcckg: {=bool:?}, gfxmmumckg: {=bool:?}, ahbsckg: {=bool:?}, fmcckg: {=bool:?}, xspi1ckg: {=bool:?}, xspi2ckg: {=bool:?}, axiram4ckg: {=bool:?}, axiram3ckg: {=bool:?}, axiram2ckg: {=bool:?}, axiram1ckg: {=bool:?}, flitfckg: {=bool:?}, extickg: {=bool:?}, jtagckg: {=bool:?} }}" , self . axickg () , self . ahbmckg () , self . sdmmc1ckg () , self . hpdma1ckg () , self . cpuckg () , self . gpus0ckg () , self . gpus1ckg () , self . gpuclckg () , self . dcmippckg () , self . dma2dckg () , self . gfxmmusckg () , self . ltdcckg () , self . gfxmmumckg () , self . ahbsckg () , self . fmcckg () , self . xspi1ckg () , self . xspi2ckg () , self . axiram4ckg () , self . axiram3ckg () , self . axiram2ckg () , self . axiram1ckg () , self . flitfckg () , self . extickg () , self . jtagckg ())
        }
    }
    #[doc = "RCC clock protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckprotr(pub u32);
    impl Ckprotr {
        #[doc = "XSPI clock protection Set and cleared by software. When set to 1, this bit prevents disabling accidentally the XSPIs. The following fields cannot be modified when this bit is set to 1: PLL2ON, PLL2DIVSEN, PLL2DIVTEN, HSEON, HSION, CSION, XSPIxEN, XSPIxLPEN, XSPIxRST."]
        #[inline(always)]
        pub const fn xspickp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI clock protection Set and cleared by software. When set to 1, this bit prevents disabling accidentally the XSPIs. The following fields cannot be modified when this bit is set to 1: PLL2ON, PLL2DIVSEN, PLL2DIVTEN, HSEON, HSION, CSION, XSPIxEN, XSPIxLPEN, XSPIxRST."]
        #[inline(always)]
        pub fn set_xspickp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FMC clock protection Set and cleared by software. When set to 1, this bit prevents disabling accidentally the FMC. The following fields cannot be modified when this bit is set to 1: PLL1ON, PLL2ON, PLL1DIVQEN, PLL2DIVREN, HSEON, HSION, CSION, FMCEN, FMCLPEN, FMCRST."]
        #[inline(always)]
        pub const fn fmcckp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FMC clock protection Set and cleared by software. When set to 1, this bit prevents disabling accidentally the FMC. The following fields cannot be modified when this bit is set to 1: PLL1ON, PLL2ON, PLL1DIVQEN, PLL2DIVREN, HSEON, HSION, CSION, FMCEN, FMCLPEN, FMCRST."]
        #[inline(always)]
        pub fn set_fmcckp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "XSPI1 kernel clock switch position Set by hardware. This field can be used to verify the real position of XSPI2 kernel switch selector."]
        #[inline(always)]
        pub const fn xspi1swp(&self) -> super::vals::Xspiswp {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Xspiswp::from_bits(val as u8)
        }
        #[doc = "XSPI1 kernel clock switch position Set by hardware. This field can be used to verify the real position of XSPI2 kernel switch selector."]
        #[inline(always)]
        pub fn set_xspi1swp(&mut self, val: super::vals::Xspiswp) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "XSPI2 kernel clock switch position Set by hardware. This field can be used to verify the real position of XSPI2 kernel switch selector."]
        #[inline(always)]
        pub const fn xspi2swp(&self) -> super::vals::Xspiswp {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Xspiswp::from_bits(val as u8)
        }
        #[doc = "XSPI2 kernel clock switch position Set by hardware. This field can be used to verify the real position of XSPI2 kernel switch selector."]
        #[inline(always)]
        pub fn set_xspi2swp(&mut self, val: super::vals::Xspiswp) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "FMC kernel clock switch position Set by hardware. This field can be used to verify the real position of FMC kernel switch selector."]
        #[inline(always)]
        pub const fn fmcswp(&self) -> super::vals::Fmcswp {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Fmcswp::from_bits(val as u8)
        }
        #[doc = "FMC kernel clock switch position Set by hardware. This field can be used to verify the real position of FMC kernel switch selector."]
        #[inline(always)]
        pub fn set_fmcswp(&mut self, val: super::vals::Fmcswp) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Ckprotr {
        #[inline(always)]
        fn default() -> Ckprotr {
            Ckprotr(0)
        }
    }
    impl core::fmt::Debug for Ckprotr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckprotr")
                .field("xspickp", &self.xspickp())
                .field("fmcckp", &self.fmcckp())
                .field("xspi1swp", &self.xspi1swp())
                .field("xspi2swp", &self.xspi2swp())
                .field("fmcswp", &self.fmcswp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ckprotr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ckprotr {{ xspickp: {=bool:?}, fmcckp: {=bool:?}, xspi1swp: {:?}, xspi2swp: {:?}, fmcswp: {:?} }}",
                self.xspickp(),
                self.fmcckp(),
                self.xspi1swp(),
                self.xspi2swp(),
                self.fmcswp()
            )
        }
    }
    #[doc = "RCC source control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 0 or STOPKERWUCK = 0. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW switch) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1."]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 0 or STOPKERWUCK = 0. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW switch) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable."]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable."]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
        #[inline(always)]
        pub const fn hsidiv(&self) -> super::vals::Hsidiv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Hsidiv::from_bits(val as u8)
        }
        #[doc = "HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
        #[inline(always)]
        pub fn set_hsidiv(&mut self, val: super::vals::Hsidiv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV. clock setting is completed)."]
        #[inline(always)]
        pub const fn hsidivf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV. clock setting is completed)."]
        #[inline(always)]
        pub fn set_hsidivf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1."]
        #[inline(always)]
        pub const fn csion(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_csion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request)."]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request)."]
        #[inline(always)]
        pub fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
        #[inline(always)]
        pub const fn csikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
        #[inline(always)]
        pub fn set_csikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable."]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable."]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1."]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable."]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable."]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub const fn hseext(&self) -> super::vals::Hseext {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Hseext::from_bits(val as u8)
        }
        #[doc = "external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub fn set_hseext(&mut self, val: super::vals::Hseext) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "HSE clock security system enable Set by software to enable clock security system on HSE. This bit is set only (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system enable Set by software to enable clock security system on HSE. This bit is set only (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
        #[inline(always)]
        pub fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3) or if FMCCKP = 1, or if XSPICKP = 1."]
        #[inline(always)]
        pub const fn pllon(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3) or if FMCCKP = 1, or if XSPICKP = 1."]
        #[inline(always)]
        pub fn set_pllon(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
        #[inline(always)]
        pub const fn pllrdy(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("hsidiv", &self.hsidiv())
                .field("hsidivf", &self.hsidivf())
                .field("csion", &self.csion())
                .field("csirdy", &self.csirdy())
                .field("csikeron", &self.csikeron())
                .field("hsi48on", &self.hsi48on())
                .field("hsi48rdy", &self.hsi48rdy())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("hseext", &self.hseext())
                .field("hsecsson", &self.hsecsson())
                .field("pllon[0]", &self.pllon(0usize))
                .field("pllon[1]", &self.pllon(1usize))
                .field("pllon[2]", &self.pllon(2usize))
                .field("pllrdy[0]", &self.pllrdy(0usize))
                .field("pllrdy[1]", &self.pllrdy(1usize))
                .field("pllrdy[2]", &self.pllrdy(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ hsion: {=bool:?}, hsikeron: {=bool:?}, hsirdy: {=bool:?}, hsidiv: {:?}, hsidivf: {=bool:?}, csion: {=bool:?}, csirdy: {=bool:?}, csikeron: {=bool:?}, hsi48on: {=bool:?}, hsi48rdy: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, hseext: {:?}, hsecsson: {=bool:?}, pllon[0]: {=bool:?}, pllon[1]: {=bool:?}, pllon[2]: {=bool:?}, pllrdy[0]: {=bool:?}, pllrdy[1]: {=bool:?}, pllrdy[2]: {=bool:?} }}" , self . hsion () , self . hsikeron () , self . hsirdy () , self . hsidiv () , self . hsidivf () , self . csion () , self . csirdy () , self . csikeron () , self . hsi48on () , self . hsi48rdy () , self . hseon () , self . hserdy () , self . hsebyp () , self . hseext () , self . hsecsson () , self . pllon (0usize) , self . pllon (1usize) , self . pllon (2usize) , self . pllrdy (0usize) , self . pllrdy (1usize) , self . pllrdy (2usize))
        }
    }
    #[doc = "RCC clock recovery RC register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "Internal RC 48 MHz clock calibration Set by hardware by option byte loading. Read-only."]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Internal RC 48 MHz clock calibration Set by hardware by option byte loading. Read-only."]
        #[inline(always)]
        pub fn set_hsi48cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
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
    #[doc = "RCC CSI calibration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csicfgr(pub u32);
    impl Csicfgr {
        #[doc = "CSI clock calibration Set by hardware by option byte loading. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value."]
        #[inline(always)]
        pub const fn csical(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "CSI clock calibration Set by hardware by option byte loading. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value."]
        #[inline(always)]
        pub fn set_csical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_opt) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_opt. Note: The reset value of the field is 0x20."]
        #[inline(always)]
        pub const fn csitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_opt) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_opt. Note: The reset value of the field is 0x20."]
        #[inline(always)]
        pub fn set_csitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Csicfgr {
        #[inline(always)]
        fn default() -> Csicfgr {
            Csicfgr(0)
        }
    }
    impl core::fmt::Debug for Csicfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csicfgr")
                .field("csical", &self.csical())
                .field("csitrim", &self.csitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csicfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csicfgr {{ csical: {=u8:?}, csitrim: {=u8:?} }}",
                self.csical(),
                self.csitrim()
            )
        }
    }
    #[doc = "RCC clock control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "LSI oscillator enable Set and reset by software."]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable Set and reset by software."]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSI oscillator ready Set and reset by hardware to indicate when the low-speed internal RC oscillator is stable. This bit needs 3 cycles of lsi_ck clock to fall down after LSION has been set to 0. This bit can be set even when LSION is not enabled if there is a request for LSI clock by the clock security system on LSE or by the low-speed watchdog or by the RTC."]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready Set and reset by hardware to indicate when the low-speed internal RC oscillator is stable. This bit needs 3 cycles of lsi_ck clock to fall down after LSION has been set to 0. This bit can be set even when LSION is not enabled if there is a request for LSI clock by the clock security system on LSE or by the low-speed watchdog or by the RTC."]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ lsion: {=bool:?}, lsirdy: {=bool:?} }}",
                self.lsion(),
                self.lsirdy()
            )
        }
    }
    #[doc = "RCC HSI calibration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hsicfgr(pub u32);
    impl Hsicfgr {
        #[doc = "HSI clock calibration Set by hardware by option byte loading. Adjusted by software through trimming bits HSITRIM. This field represents the sum of engineering option byte calibration value and HSITRIM bits value."]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration Set by hardware by option byte loading. Adjusted by software through trimming bits HSITRIM. This field represents the sum of engineering option byte calibration value and HSITRIM bits value."]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_opt) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_opt. Note: The reset value of the field is 0x40."]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_opt) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_opt. Note: The reset value of the field is 0x40."]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Hsicfgr {
        #[inline(always)]
        fn default() -> Hsicfgr {
            Hsicfgr(0)
        }
    }
    impl core::fmt::Debug for Hsicfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hsicfgr")
                .field("hsical", &self.hsical())
                .field("hsitrim", &self.hsitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hsicfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hsicfgr {{ hsical: {=u16:?}, hsitrim: {=u8:?} }}",
                self.hsical(),
                self.hsitrim()
            )
        }
    }
    #[doc = "RCC PLLs configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "PLL1 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL1FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information."]
        #[inline(always)]
        pub const fn pllfracen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 11usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL1FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information."]
        #[inline(always)]
        pub fn set_pllfracen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 11usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref1_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref1_ck</sub> must be between 1 and 2 MHz)."]
        #[inline(always)]
        pub const fn pllvcosel(&self, n: usize) -> super::vals::Pllvcosel {
            assert!(n < 3usize);
            let offs = 1usize + n * 11usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllvcosel::from_bits(val as u8)
        }
        #[doc = "PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref1_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref1_ck</sub> must be between 1 and 2 MHz)."]
        #[inline(always)]
        pub fn set_pllvcosel(&mut self, n: usize, val: super::vals::Pllvcosel) {
            assert!(n < 3usize);
            let offs = 1usize + n * 11usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL1, in order to reduce the amount of EMI peaks."]
        #[inline(always)]
        pub const fn pllsscgen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 2usize + n * 11usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL1, in order to reduce the amount of EMI peaks."]
        #[inline(always)]
        pub fn set_pllsscgen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 2usize + n * 11usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
        #[inline(always)]
        pub const fn pllrge(&self, n: usize) -> super::vals::Pllrge {
            assert!(n < 3usize);
            let offs = 3usize + n * 11usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
        #[inline(always)]
        pub fn set_pllrge(&mut self, n: usize, val: super::vals::Pllrge) {
            assert!(n < 3usize);
            let offs = 3usize + n * 11usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. The hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
        #[inline(always)]
        pub const fn divpen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 5usize + n * 11usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. The hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
        #[inline(always)]
        pub fn set_divpen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 5usize + n * 11usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. The hardware prevents writing this bit if FMCCKP = 1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled."]
        #[inline(always)]
        pub const fn divqen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 11usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. The hardware prevents writing this bit if FMCCKP = 1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled."]
        #[inline(always)]
        pub fn set_divqen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 11usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1DIVREN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used."]
        #[inline(always)]
        pub const fn divren(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 11usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1DIVREN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used."]
        #[inline(always)]
        pub fn set_divren(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 11usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVS divider output enable Set and reset by software to enable the pll1_s_ck output of the PLL1. To save power, PLL1DIVSEN must be set to 0 when the pll1_s_ck is not used."]
        #[inline(always)]
        pub const fn divsen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 8usize + n * 11usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVS divider output enable Set and reset by software to enable the pll1_s_ck output of the PLL1. To save power, PLL1DIVSEN must be set to 0 when the pll1_s_ck is not used."]
        #[inline(always)]
        pub fn set_divsen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 8usize + n * 11usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVT divider output enable Set and reset by software to enable the pll1_t_ck output of the PLL1. To save power, PLL1DIVTEN must be set to 0 when the pll1_t_ck is not used."]
        #[inline(always)]
        pub const fn divten(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 9usize + n * 11usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVT divider output enable Set and reset by software to enable the pll1_t_ck output of the PLL1. To save power, PLL1DIVTEN must be set to 0 when the pll1_t_ck is not used."]
        #[inline(always)]
        pub fn set_divten(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 9usize + n * 11usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("pllfracen[0]", &self.pllfracen(0usize))
                .field("pllfracen[1]", &self.pllfracen(1usize))
                .field("pllfracen[2]", &self.pllfracen(2usize))
                .field("pllvcosel[0]", &self.pllvcosel(0usize))
                .field("pllvcosel[1]", &self.pllvcosel(1usize))
                .field("pllvcosel[2]", &self.pllvcosel(2usize))
                .field("pllsscgen[0]", &self.pllsscgen(0usize))
                .field("pllsscgen[1]", &self.pllsscgen(1usize))
                .field("pllsscgen[2]", &self.pllsscgen(2usize))
                .field("pllrge[0]", &self.pllrge(0usize))
                .field("pllrge[1]", &self.pllrge(1usize))
                .field("pllrge[2]", &self.pllrge(2usize))
                .field("divpen[0]", &self.divpen(0usize))
                .field("divpen[1]", &self.divpen(1usize))
                .field("divpen[2]", &self.divpen(2usize))
                .field("divqen[0]", &self.divqen(0usize))
                .field("divqen[1]", &self.divqen(1usize))
                .field("divqen[2]", &self.divqen(2usize))
                .field("divren[0]", &self.divren(0usize))
                .field("divren[1]", &self.divren(1usize))
                .field("divren[2]", &self.divren(2usize))
                .field("divsen[0]", &self.divsen(0usize))
                .field("divsen[1]", &self.divsen(1usize))
                .field("divsen[2]", &self.divsen(2usize))
                .field("divten[0]", &self.divten(0usize))
                .field("divten[1]", &self.divten(1usize))
                .field("divten[2]", &self.divten(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pllcfgr {{ pllfracen[0]: {=bool:?}, pllfracen[1]: {=bool:?}, pllfracen[2]: {=bool:?}, pllvcosel[0]: {:?}, pllvcosel[1]: {:?}, pllvcosel[2]: {:?}, pllsscgen[0]: {=bool:?}, pllsscgen[1]: {=bool:?}, pllsscgen[2]: {=bool:?}, pllrge[0]: {:?}, pllrge[1]: {:?}, pllrge[2]: {:?}, divpen[0]: {=bool:?}, divpen[1]: {=bool:?}, divpen[2]: {=bool:?}, divqen[0]: {=bool:?}, divqen[1]: {=bool:?}, divqen[2]: {=bool:?}, divren[0]: {=bool:?}, divren[1]: {=bool:?}, divren[2]: {=bool:?}, divsen[0]: {=bool:?}, divsen[1]: {=bool:?}, divsen[2]: {=bool:?}, divten[0]: {=bool:?}, divten[1]: {=bool:?}, divten[2]: {=bool:?} }}" , self . pllfracen (0usize) , self . pllfracen (1usize) , self . pllfracen (2usize) , self . pllvcosel (0usize) , self . pllvcosel (1usize) , self . pllvcosel (2usize) , self . pllsscgen (0usize) , self . pllsscgen (1usize) , self . pllsscgen (2usize) , self . pllrge (0usize) , self . pllrge (1usize) , self . pllrge (2usize) , self . divpen (0usize) , self . divpen (1usize) , self . divpen (2usize) , self . divqen (0usize) , self . divqen (1usize) , self . divqen (2usize) , self . divren (0usize) , self . divren (1usize) , self . divren (2usize) , self . divsen (0usize) , self . divsen (1usize) , self . divsen (2usize) , self . divten (0usize) , self . divten (1usize) , self . divten (2usize))
        }
    }
    #[doc = "RCC PLLs clock source selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllckselr(pub u32);
    impl Pllckselr {
        #[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, PLLSRC must be set to 11."]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, PLLSRC must be set to 11."]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
        #[inline(always)]
        pub const fn divm(&self, n: usize) -> super::vals::Pllm {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
        #[inline(always)]
        pub fn set_divm(&mut self, n: usize, val: super::vals::Pllm) {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x3f << offs)) | (((val.to_bits() as u32) & 0x3f) << offs);
        }
    }
    impl Default for Pllckselr {
        #[inline(always)]
        fn default() -> Pllckselr {
            Pllckselr(0)
        }
    }
    impl core::fmt::Debug for Pllckselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllckselr")
                .field("pllsrc", &self.pllsrc())
                .field("divm[0]", &self.divm(0usize))
                .field("divm[1]", &self.divm(1usize))
                .field("divm[2]", &self.divm(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllckselr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pllckselr {{ pllsrc: {:?}, divm[0]: {:?}, divm[1]: {:?}, divm[2]: {:?} }}",
                self.pllsrc(),
                self.divm(0usize),
                self.divm(1usize),
                self.divm(2usize)
            )
        }
    }
    #[doc = "RCC PLL1 dividers configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plldivr(pub u32);
    impl Plldivr {
        #[doc = "multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F<sub>ref1_ck</sub> x DIVN1, when fractional value 0 has been loaded into FRACN, with: DIVN1 between 8 and 420 The input frequency F<sub>ref1_ck</sub> must be between 1 and 16 MHz."]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F<sub>ref1_ck</sub> x DIVN1, when fractional value 0 has been loaded into FRACN, with: DIVN1 between 8 and 420 The input frequency F<sub>ref1_ck</sub> must be between 1 and 16 MHz."]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1DIVPEN = 0. ..."]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1DIVPEN = 0. ..."]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1DIVQEN = 0. ..."]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1DIVQEN = 0. ..."]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1DIVREN = 0. ..."]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1DIVREN = 0. ..."]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Plldivr {
        #[inline(always)]
        fn default() -> Plldivr {
            Plldivr(0)
        }
    }
    impl core::fmt::Debug for Plldivr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Plldivr")
                .field("plln", &self.plln())
                .field("pllp", &self.pllp())
                .field("pllq", &self.pllq())
                .field("pllr", &self.pllr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Plldivr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Plldivr {{ plln: {:?}, pllp: {:?}, pllq: {:?}, pllr: {:?} }}",
                self.plln(),
                self.pllp(),
                self.pllq(),
                self.pllr()
            )
        }
    }
    #[doc = "RCC PLL1 dividers configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plldivr2(pub u32);
    impl Plldivr2 {
        #[doc = "PLL1 DIVS division factor Set and reset by software to control the frequency of the pll1_s_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVS+1) is even, With VCOH, for all DIVS values These bits can be written only when the PLL1DIVSEN = 0."]
        #[inline(always)]
        pub const fn plls(&self) -> super::vals::Plldivst {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Plldivst::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVS division factor Set and reset by software to control the frequency of the pll1_s_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVS+1) is even, With VCOH, for all DIVS values These bits can be written only when the PLL1DIVSEN = 0."]
        #[inline(always)]
        pub fn set_plls(&mut self, val: super::vals::Plldivst) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "PLL1 DIVT division factor Set and reset by software to control the frequency of the pll1_t_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVT+1) is even, With VCOH, for all DIVT values These bits can be written only when the PLL1DIVTEN = 0."]
        #[inline(always)]
        pub const fn pllt(&self) -> super::vals::Plldivst {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Plldivst::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVT division factor Set and reset by software to control the frequency of the pll1_t_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVT+1) is even, With VCOH, for all DIVT values These bits can be written only when the PLL1DIVTEN = 0."]
        #[inline(always)]
        pub fn set_pllt(&mut self, val: super::vals::Plldivst) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Plldivr2 {
        #[inline(always)]
        fn default() -> Plldivr2 {
            Plldivr2(0)
        }
    }
    impl core::fmt::Debug for Plldivr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Plldivr2")
                .field("plls", &self.plls())
                .field("pllt", &self.pllt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Plldivr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Plldivr2 {{ plls: {:?}, pllt: {:?} }}", self.plls(), self.pllt())
        }
    }
    #[doc = "RCC PLL1 fractional divider register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllfracr(pub u32);
    impl Pllfracr {
        #[doc = "fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544 MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F<sub>ref1_ck</sub> x (DIVN1 + (FRACN / 2<sup>13</sup>)), with DIVN1 between 8 and 420 FRACN can be between 0 and 2<sup>13</sup>- 1 The input frequency F<sub>ref1_ck</sub> must be between 1 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACLE to 0. Write the new fractional value into FRACN. Set the bit PLL1FRACLE to 1."]
        #[inline(always)]
        pub const fn fracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544 MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F<sub>ref1_ck</sub> x (DIVN1 + (FRACN / 2<sup>13</sup>)), with DIVN1 between 8 and 420 FRACN can be between 0 and 2<sup>13</sup>- 1 The input frequency F<sub>ref1_ck</sub> must be between 1 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACLE to 0. Write the new fractional value into FRACN. Set the bit PLL1FRACLE to 1."]
        #[inline(always)]
        pub fn set_fracn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Pllfracr {
        #[inline(always)]
        fn default() -> Pllfracr {
            Pllfracr(0)
        }
    }
    impl core::fmt::Debug for Pllfracr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllfracr").field("fracn", &self.fracn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllfracr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pllfracr {{ fracn: {=u16:?} }}", self.fracn())
        }
    }
    #[doc = "RCC PLL1 Spread Spectrum Clock Generator register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllsscgr(pub u32);
    impl Pllsscgr {
        #[doc = "Modulation Period Adjustment for PLL1 Set and reset by software to adjust the modulation period of the clock spreading generator."]
        #[inline(always)]
        pub const fn mod_per(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Modulation Period Adjustment for PLL1 Set and reset by software to adjust the modulation period of the clock spreading generator."]
        #[inline(always)]
        pub fn set_mod_per(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Dithering TPDF noise control for PLL1 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a triangular probability density function."]
        #[inline(always)]
        pub const fn tpdfn_dis1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering TPDF noise control for PLL1 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a triangular probability density function."]
        #[inline(always)]
        pub fn set_tpdfn_dis1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Dithering RPDF noise control for PLL1 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a rectangular probability density function."]
        #[inline(always)]
        pub const fn rpdfn_dis1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering RPDF noise control for PLL1 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a rectangular probability density function."]
        #[inline(always)]
        pub fn set_rpdfn_dis1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Spread spectrum clock generator mode for PLL1 Set and reset by software to select the clock spreading mode."]
        #[inline(always)]
        pub const fn dwnspread1(&self) -> super::vals::Dwnspread {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Dwnspread::from_bits(val as u8)
        }
        #[doc = "Spread spectrum clock generator mode for PLL1 Set and reset by software to select the clock spreading mode."]
        #[inline(always)]
        pub fn set_dwnspread1(&mut self, val: super::vals::Dwnspread) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Modulation Depth Adjustment for PLL1 Set and reset by software to adjust the modulation depth of the clock spreading generator."]
        #[inline(always)]
        pub const fn inc_step(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Modulation Depth Adjustment for PLL1 Set and reset by software to adjust the modulation depth of the clock spreading generator."]
        #[inline(always)]
        pub fn set_inc_step(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Pllsscgr {
        #[inline(always)]
        fn default() -> Pllsscgr {
            Pllsscgr(0)
        }
    }
    impl core::fmt::Debug for Pllsscgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllsscgr")
                .field("mod_per", &self.mod_per())
                .field("tpdfn_dis1", &self.tpdfn_dis1())
                .field("rpdfn_dis1", &self.rpdfn_dis1())
                .field("dwnspread1", &self.dwnspread1())
                .field("inc_step", &self.inc_step())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllsscgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pllsscgr {{ mod_per: {=u16:?}, tpdfn_dis1: {=bool:?}, rpdfn_dis1: {=bool:?}, dwnspread1: {:?}, inc_step: {=u16:?} }}" , self . mod_per () , self . tpdfn_dis1 () , self . rpdfn_dis1 () , self . dwnspread1 () , self . inc_step ())
        }
    }
    #[doc = "RCC Reset status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsr(pub u32);
    impl Rsr {
        #[doc = "remove reset flag Set and reset by software to reset the value of the reset flags."]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "remove reset flag Set and reset by software to reset the value of the reset flags."]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Option byte loading reset flag <sup>(1)</sup> Reset by software by the RMVF bit. Set by hardware when a reset from the option byte loading occurs."]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loading reset flag <sup>(1)</sup> Reset by software by the RMVF bit. Set by hardware when a reset from the option byte loading occurs."]
        #[inline(always)]
        pub fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BOR reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "pin reset flag (NRST) <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "pin reset flag (NRST) <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "POR/PDR reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs."]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs."]
        #[inline(always)]
        pub fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "system reset from CPU reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7."]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "system reset from CPU reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7."]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "independent watchdog reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "independent watchdog reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "window watchdog reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "window watchdog reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "reset due to illegal Stop or Standby flag Reset by software by writing the RMVF bit. Set by hardware when the CPU goes erroneously in Stop or Standby mode,."]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "reset due to illegal Stop or Standby flag Reset by software by writing the RMVF bit. Set by hardware when the CPU goes erroneously in Stop or Standby mode,."]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Rsr {
        #[inline(always)]
        fn default() -> Rsr {
            Rsr(0)
        }
    }
    impl core::fmt::Debug for Rsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsr")
                .field("rmvf", &self.rmvf())
                .field("oblrstf", &self.oblrstf())
                .field("borrstf", &self.borrstf())
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
    impl defmt::Format for Rsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rsr {{ rmvf: {=bool:?}, oblrstf: {=bool:?}, borrstf: {=bool:?}, pinrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}" , self . rmvf () , self . oblrstf () , self . borrstf () , self . pinrstf () , self . porrstf () , self . sftrstf () , self . iwdgrstf () , self . wwdgrstf () , self . lpwrrstf ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcsel {
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Adcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcsel {
        #[inline(always)]
        fn from(val: u8) -> Adcsel {
            Adcsel::from_bits(val)
        }
    }
    impl From<Adcsel> for u8 {
        #[inline(always)]
        fn from(val: Adcsel) -> u8 {
            Adcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adfsel {
        #[doc = "hclk1 selected as ADF kernel clock (default after reset)."]
        HCLK1 = 0x0,
        #[doc = "pll2_p_ck selected as ADF kernel clock."]
        PLL2_P = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Adfsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adfsel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Cecsel {
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x0,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x01,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cecsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cecsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cecsel {
        #[inline(always)]
        fn from(val: u8) -> Cecsel {
            Cecsel::from_bits(val)
        }
    }
    impl From<Cecsel> for u8 {
        #[inline(always)]
        fn from(val: Cecsel) -> u8 {
            Cecsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dwnspread {
        #[doc = "Center-spread modulation selected (default after reset)."]
        CENTER_SPREAD = 0x0,
        #[doc = "Down-spread modulation selected."]
        DOWN_SPREAD = 0x01,
    }
    impl Dwnspread {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dwnspread {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dwnspread {
        #[inline(always)]
        fn from(val: u8) -> Dwnspread {
            Dwnspread::from_bits(val)
        }
    }
    impl From<Dwnspread> for u8 {
        #[inline(always)]
        fn from(val: Dwnspread) -> u8 {
            Dwnspread::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EthRefClkSel {
        #[doc = "PAD ETH_RMII_REF_CLK selected as kernel peripheral clock (default after reset)."]
        ETH_RMII_REF = 0x0,
        #[doc = "hse_ker_ck selected as kernel peripheral clock."]
        HSE = 0x01,
        #[doc = "eth_clk_fb selected as kernel peripheral clock."]
        ETH = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl EthRefClkSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EthRefClkSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EthRefClkSel {
        #[inline(always)]
        fn from(val: u8) -> EthRefClkSel {
            EthRefClkSel::from_bits(val)
        }
    }
    impl From<EthRefClkSel> for u8 {
        #[inline(always)]
        fn from(val: EthRefClkSel) -> u8 {
            EthRefClkSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EthphyClkSel {
        #[doc = "hse_ker_ck selected as clock source (default after reset)."]
        HSE = 0x0,
        #[doc = "pll3_s_ck selected clock source."]
        PLL3_S = 0x01,
    }
    impl EthphyClkSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EthphyClkSel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EthphyClkSel {
        #[inline(always)]
        fn from(val: u8) -> EthphyClkSel {
            EthphyClkSel::from_bits(val)
        }
    }
    impl From<EthphyClkSel> for u8 {
        #[inline(always)]
        fn from(val: EthphyClkSel) -> u8 {
            EthphyClkSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fdcansel {
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Fdcansel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fdcansel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Fmcsel {
        #[doc = "hclk5 selected as kernel peripheral clock (default after reset)."]
        HCLK5 = 0x0,
        #[doc = "pll1_q_ck selected as kernel peripheral clock."]
        PLL1_Q = 0x01,
        #[doc = "pll2_r_ck selected as kernel peripheral clock."]
        PLL2_R = 0x02,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        HSI = 0x03,
    }
    impl Fmcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmcsel {
        #[inline(always)]
        fn from(val: u8) -> Fmcsel {
            Fmcsel::from_bits(val)
        }
    }
    impl From<Fmcsel> for u8 {
        #[inline(always)]
        fn from(val: Fmcsel) -> u8 {
            Fmcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fmcswp {
        #[doc = "The switch is in neutral mode and output clock is gated (default after reset)."]
        B_0X0 = 0x0,
        #[doc = "The switch is selecting hclk5."]
        B_0X1 = 0x01,
        #[doc = "The switch is selecting pll1_q_ck."]
        B_0X2 = 0x02,
        #[doc = "The switch is selecting pll2_r_ck."]
        B_0X3 = 0x03,
        #[doc = "The switch is selecting hsi_ker_ck."]
        B_0X4 = 0x04,
        #[doc = "The switch is in recovery position (hclk5/4)."]
        B_0X5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fmcswp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmcswp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmcswp {
        #[inline(always)]
        fn from(val: u8) -> Fmcswp {
            Fmcswp::from_bits(val)
        }
    }
    impl From<Fmcswp> for u8 {
        #[inline(always)]
        fn from(val: Fmcswp) -> u8 {
            Fmcswp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        DIV2 = 0x08,
        DIV4 = 0x09,
        DIV8 = 0x0a,
        DIV16 = 0x0b,
        DIV64 = 0x0c,
        DIV128 = 0x0d,
        DIV256 = 0x0e,
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
        #[doc = "HSE in analog mode (default after reset)"]
        ANALOG = 0x0,
        #[doc = "HSE in digital mode"]
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
    pub enum Hsidiv {
        #[doc = "division by 1, hsi(_ker)_ck = 64 MHz (default after reset)."]
        DIV1 = 0x0,
        #[doc = "division by 2, hsi(_ker)_ck = 32 MHz."]
        DIV2 = 0x01,
        #[doc = "division by 4, hsi(_ker)_ck = 16 MHz."]
        DIV4 = 0x02,
        #[doc = "division by 8, hsi(_ker)_ck = 8 MHz."]
        DIV8 = 0x03,
    }
    impl Hsidiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsidiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsidiv {
        #[inline(always)]
        fn from(val: u8) -> Hsidiv {
            Hsidiv::from_bits(val)
        }
    }
    impl From<Hsidiv> for u8 {
        #[inline(always)]
        fn from(val: Hsidiv) -> u8 {
            Hsidiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c1I3c1sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x03,
    }
    impl I2c1I3c1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1I3c1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1I3c1sel {
        #[inline(always)]
        fn from(val: u8) -> I2c1I3c1sel {
            I2c1I3c1sel::from_bits(val)
        }
    }
    impl From<I2c1I3c1sel> for u8 {
        #[inline(always)]
        fn from(val: I2c1I3c1sel) -> u8 {
            I2c1I3c1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2csel {
        #[doc = "pclk1 selected as kernel clock (default after reset)."]
        PCLK1 = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x03,
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
    pub enum Lptim1sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x03,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x04,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptim1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim1sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim1sel {
            Lptim1sel::from_bits(val)
        }
    }
    impl From<Lptim1sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim1sel) -> u8 {
            Lptim1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptimsel {
        #[doc = "rcc_pclk4 selected as peripheral clock"]
        PCLK4 = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x03,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x04,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptimsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptimsel {
            unsafe { core::mem::transmute(val & 0x07) }
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
        #[doc = "rcc_pclk_d4 selected as peripheral clock"]
        PCLK4 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lpuartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpuartsel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Mco1sel {
        #[doc = "HSI selected for micro-controller clock output"]
        HSI = 0x0,
        #[doc = "LSE selected for micro-controller clock output"]
        LSE = 0x01,
        #[doc = "HSE selected for micro-controller clock output"]
        HSE = 0x02,
        #[doc = "pll1_q selected for micro-controller clock output"]
        PLL1_Q = 0x03,
        #[doc = "HSI48 selected for micro-controller clock output"]
        HSI48 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mco1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco1sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
        #[doc = "System clock selected for micro-controller clock output"]
        SYS = 0x0,
        #[doc = "pll2_p selected for micro-controller clock output"]
        PLL2_P = 0x01,
        #[doc = "HSE selected for micro-controller clock output"]
        HSE = 0x02,
        #[doc = "pll1_p selected for micro-controller clock output"]
        PLL1_P = 0x03,
        #[doc = "CSI selected for micro-controller clock output"]
        CSI = 0x04,
        #[doc = "LSI selected for micro-controller clock output"]
        LSI = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mco2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco2sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
        _RESERVED_0 = 0x0,
        #[doc = "Divide by 1"]
        DIV1 = 0x01,
        #[doc = "Divide by 2"]
        DIV2 = 0x02,
        #[doc = "Divide by 3"]
        DIV3 = 0x03,
        #[doc = "Divide by 4"]
        DIV4 = 0x04,
        #[doc = "Divide by 5"]
        DIV5 = 0x05,
        #[doc = "Divide by 6"]
        DIV6 = 0x06,
        #[doc = "Divide by 7"]
        DIV7 = 0x07,
        #[doc = "Divide by 8"]
        DIV8 = 0x08,
        #[doc = "Divide by 9"]
        DIV9 = 0x09,
        #[doc = "Divide by 10"]
        DIV10 = 0x0a,
        #[doc = "Divide by 11"]
        DIV11 = 0x0b,
        #[doc = "Divide by 12"]
        DIV12 = 0x0c,
        #[doc = "Divide by 13"]
        DIV13 = 0x0d,
        #[doc = "Divide by 14"]
        DIV14 = 0x0e,
        #[doc = "Divide by 15"]
        DIV15 = 0x0f,
    }
    impl Mcopre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcopre {
            unsafe { core::mem::transmute(val & 0x0f) }
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
    pub enum Persel {
        #[doc = "HSI selected as peripheral clock"]
        HSI = 0x0,
        #[doc = "CSI selected as peripheral clock"]
        CSI = 0x01,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Persel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Persel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Persel {
        #[inline(always)]
        fn from(val: u8) -> Persel {
            Persel::from_bits(val)
        }
    }
    impl From<Persel> for u8 {
        #[inline(always)]
        fn from(val: Persel) -> u8 {
            Persel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plldiv {
        DIV1 = 0x0,
        DIV2 = 0x01,
        DIV3 = 0x02,
        DIV4 = 0x03,
        DIV5 = 0x04,
        DIV6 = 0x05,
        DIV7 = 0x06,
        DIV8 = 0x07,
        DIV9 = 0x08,
        DIV10 = 0x09,
        DIV11 = 0x0a,
        DIV12 = 0x0b,
        DIV13 = 0x0c,
        DIV14 = 0x0d,
        DIV15 = 0x0e,
        DIV16 = 0x0f,
        DIV17 = 0x10,
        DIV18 = 0x11,
        DIV19 = 0x12,
        DIV20 = 0x13,
        DIV21 = 0x14,
        DIV22 = 0x15,
        DIV23 = 0x16,
        DIV24 = 0x17,
        DIV25 = 0x18,
        DIV26 = 0x19,
        DIV27 = 0x1a,
        DIV28 = 0x1b,
        DIV29 = 0x1c,
        DIV30 = 0x1d,
        DIV31 = 0x1e,
        DIV32 = 0x1f,
        DIV33 = 0x20,
        DIV34 = 0x21,
        DIV35 = 0x22,
        DIV36 = 0x23,
        DIV37 = 0x24,
        DIV38 = 0x25,
        DIV39 = 0x26,
        DIV40 = 0x27,
        DIV41 = 0x28,
        DIV42 = 0x29,
        DIV43 = 0x2a,
        DIV44 = 0x2b,
        DIV45 = 0x2c,
        DIV46 = 0x2d,
        DIV47 = 0x2e,
        DIV48 = 0x2f,
        DIV49 = 0x30,
        DIV50 = 0x31,
        DIV51 = 0x32,
        DIV52 = 0x33,
        DIV53 = 0x34,
        DIV54 = 0x35,
        DIV55 = 0x36,
        DIV56 = 0x37,
        DIV57 = 0x38,
        DIV58 = 0x39,
        DIV59 = 0x3a,
        DIV60 = 0x3b,
        DIV61 = 0x3c,
        DIV62 = 0x3d,
        DIV63 = 0x3e,
        DIV64 = 0x3f,
        DIV65 = 0x40,
        DIV66 = 0x41,
        DIV67 = 0x42,
        DIV68 = 0x43,
        DIV69 = 0x44,
        DIV70 = 0x45,
        DIV71 = 0x46,
        DIV72 = 0x47,
        DIV73 = 0x48,
        DIV74 = 0x49,
        DIV75 = 0x4a,
        DIV76 = 0x4b,
        DIV77 = 0x4c,
        DIV78 = 0x4d,
        DIV79 = 0x4e,
        DIV80 = 0x4f,
        DIV81 = 0x50,
        DIV82 = 0x51,
        DIV83 = 0x52,
        DIV84 = 0x53,
        DIV85 = 0x54,
        DIV86 = 0x55,
        DIV87 = 0x56,
        DIV88 = 0x57,
        DIV89 = 0x58,
        DIV90 = 0x59,
        DIV91 = 0x5a,
        DIV92 = 0x5b,
        DIV93 = 0x5c,
        DIV94 = 0x5d,
        DIV95 = 0x5e,
        DIV96 = 0x5f,
        DIV97 = 0x60,
        DIV98 = 0x61,
        DIV99 = 0x62,
        DIV100 = 0x63,
        DIV101 = 0x64,
        DIV102 = 0x65,
        DIV103 = 0x66,
        DIV104 = 0x67,
        DIV105 = 0x68,
        DIV106 = 0x69,
        DIV107 = 0x6a,
        DIV108 = 0x6b,
        DIV109 = 0x6c,
        DIV110 = 0x6d,
        DIV111 = 0x6e,
        DIV112 = 0x6f,
        DIV113 = 0x70,
        DIV114 = 0x71,
        DIV115 = 0x72,
        DIV116 = 0x73,
        DIV117 = 0x74,
        DIV118 = 0x75,
        DIV119 = 0x76,
        DIV120 = 0x77,
        DIV121 = 0x78,
        DIV122 = 0x79,
        DIV123 = 0x7a,
        DIV124 = 0x7b,
        DIV125 = 0x7c,
        DIV126 = 0x7d,
        DIV127 = 0x7e,
        DIV128 = 0x7f,
    }
    impl Plldiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plldiv {
            unsafe { core::mem::transmute(val & 0x7f) }
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
    pub enum Plldivst {
        DIV1 = 0x0,
        DIV2 = 0x01,
        DIV3 = 0x02,
        DIV4 = 0x03,
        DIV5 = 0x04,
        DIV6 = 0x05,
        DIV7 = 0x06,
        DIV8 = 0x07,
    }
    impl Plldivst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plldivst {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Plldivst {
        #[inline(always)]
        fn from(val: u8) -> Plldivst {
            Plldivst::from_bits(val)
        }
    }
    impl From<Plldivst> for u8 {
        #[inline(always)]
        fn from(val: Plldivst) -> u8 {
            Plldivst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllm {
        _RESERVED_0 = 0x0,
        DIV1 = 0x01,
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
    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plln {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        MUL8 = 0x07,
        MUL9 = 0x08,
        MUL10 = 0x09,
        MUL11 = 0x0a,
        MUL12 = 0x0b,
        MUL13 = 0x0c,
        MUL14 = 0x0d,
        MUL15 = 0x0e,
        MUL16 = 0x0f,
        MUL17 = 0x10,
        MUL18 = 0x11,
        MUL19 = 0x12,
        MUL20 = 0x13,
        MUL21 = 0x14,
        MUL22 = 0x15,
        MUL23 = 0x16,
        MUL24 = 0x17,
        MUL25 = 0x18,
        MUL26 = 0x19,
        MUL27 = 0x1a,
        MUL28 = 0x1b,
        MUL29 = 0x1c,
        MUL30 = 0x1d,
        MUL31 = 0x1e,
        MUL32 = 0x1f,
        MUL33 = 0x20,
        MUL34 = 0x21,
        MUL35 = 0x22,
        MUL36 = 0x23,
        MUL37 = 0x24,
        MUL38 = 0x25,
        MUL39 = 0x26,
        MUL40 = 0x27,
        MUL41 = 0x28,
        MUL42 = 0x29,
        MUL43 = 0x2a,
        MUL44 = 0x2b,
        MUL45 = 0x2c,
        MUL46 = 0x2d,
        MUL47 = 0x2e,
        MUL48 = 0x2f,
        MUL49 = 0x30,
        MUL50 = 0x31,
        MUL51 = 0x32,
        MUL52 = 0x33,
        MUL53 = 0x34,
        MUL54 = 0x35,
        MUL55 = 0x36,
        MUL56 = 0x37,
        MUL57 = 0x38,
        MUL58 = 0x39,
        MUL59 = 0x3a,
        MUL60 = 0x3b,
        MUL61 = 0x3c,
        MUL62 = 0x3d,
        MUL63 = 0x3e,
        MUL64 = 0x3f,
        MUL65 = 0x40,
        MUL66 = 0x41,
        MUL67 = 0x42,
        MUL68 = 0x43,
        MUL69 = 0x44,
        MUL70 = 0x45,
        MUL71 = 0x46,
        MUL72 = 0x47,
        MUL73 = 0x48,
        MUL74 = 0x49,
        MUL75 = 0x4a,
        MUL76 = 0x4b,
        MUL77 = 0x4c,
        MUL78 = 0x4d,
        MUL79 = 0x4e,
        MUL80 = 0x4f,
        MUL81 = 0x50,
        MUL82 = 0x51,
        MUL83 = 0x52,
        MUL84 = 0x53,
        MUL85 = 0x54,
        MUL86 = 0x55,
        MUL87 = 0x56,
        MUL88 = 0x57,
        MUL89 = 0x58,
        MUL90 = 0x59,
        MUL91 = 0x5a,
        MUL92 = 0x5b,
        MUL93 = 0x5c,
        MUL94 = 0x5d,
        MUL95 = 0x5e,
        MUL96 = 0x5f,
        MUL97 = 0x60,
        MUL98 = 0x61,
        MUL99 = 0x62,
        MUL100 = 0x63,
        MUL101 = 0x64,
        MUL102 = 0x65,
        MUL103 = 0x66,
        MUL104 = 0x67,
        MUL105 = 0x68,
        MUL106 = 0x69,
        MUL107 = 0x6a,
        MUL108 = 0x6b,
        MUL109 = 0x6c,
        MUL110 = 0x6d,
        MUL111 = 0x6e,
        MUL112 = 0x6f,
        MUL113 = 0x70,
        MUL114 = 0x71,
        MUL115 = 0x72,
        MUL116 = 0x73,
        MUL117 = 0x74,
        MUL118 = 0x75,
        MUL119 = 0x76,
        MUL120 = 0x77,
        MUL121 = 0x78,
        MUL122 = 0x79,
        MUL123 = 0x7a,
        MUL124 = 0x7b,
        MUL125 = 0x7c,
        MUL126 = 0x7d,
        MUL127 = 0x7e,
        MUL128 = 0x7f,
        MUL129 = 0x80,
        MUL130 = 0x81,
        MUL131 = 0x82,
        MUL132 = 0x83,
        MUL133 = 0x84,
        MUL134 = 0x85,
        MUL135 = 0x86,
        MUL136 = 0x87,
        MUL137 = 0x88,
        MUL138 = 0x89,
        MUL139 = 0x8a,
        MUL140 = 0x8b,
        MUL141 = 0x8c,
        MUL142 = 0x8d,
        MUL143 = 0x8e,
        MUL144 = 0x8f,
        MUL145 = 0x90,
        MUL146 = 0x91,
        MUL147 = 0x92,
        MUL148 = 0x93,
        MUL149 = 0x94,
        MUL150 = 0x95,
        MUL151 = 0x96,
        MUL152 = 0x97,
        MUL153 = 0x98,
        MUL154 = 0x99,
        MUL155 = 0x9a,
        MUL156 = 0x9b,
        MUL157 = 0x9c,
        MUL158 = 0x9d,
        MUL159 = 0x9e,
        MUL160 = 0x9f,
        MUL161 = 0xa0,
        MUL162 = 0xa1,
        MUL163 = 0xa2,
        MUL164 = 0xa3,
        MUL165 = 0xa4,
        MUL166 = 0xa5,
        MUL167 = 0xa6,
        MUL168 = 0xa7,
        MUL169 = 0xa8,
        MUL170 = 0xa9,
        MUL171 = 0xaa,
        MUL172 = 0xab,
        MUL173 = 0xac,
        MUL174 = 0xad,
        MUL175 = 0xae,
        MUL176 = 0xaf,
        MUL177 = 0xb0,
        MUL178 = 0xb1,
        MUL179 = 0xb2,
        MUL180 = 0xb3,
        MUL181 = 0xb4,
        MUL182 = 0xb5,
        MUL183 = 0xb6,
        MUL184 = 0xb7,
        MUL185 = 0xb8,
        MUL186 = 0xb9,
        MUL187 = 0xba,
        MUL188 = 0xbb,
        MUL189 = 0xbc,
        MUL190 = 0xbd,
        MUL191 = 0xbe,
        MUL192 = 0xbf,
        MUL193 = 0xc0,
        MUL194 = 0xc1,
        MUL195 = 0xc2,
        MUL196 = 0xc3,
        MUL197 = 0xc4,
        MUL198 = 0xc5,
        MUL199 = 0xc6,
        MUL200 = 0xc7,
        MUL201 = 0xc8,
        MUL202 = 0xc9,
        MUL203 = 0xca,
        MUL204 = 0xcb,
        MUL205 = 0xcc,
        MUL206 = 0xcd,
        MUL207 = 0xce,
        MUL208 = 0xcf,
        MUL209 = 0xd0,
        MUL210 = 0xd1,
        MUL211 = 0xd2,
        MUL212 = 0xd3,
        MUL213 = 0xd4,
        MUL214 = 0xd5,
        MUL215 = 0xd6,
        MUL216 = 0xd7,
        MUL217 = 0xd8,
        MUL218 = 0xd9,
        MUL219 = 0xda,
        MUL220 = 0xdb,
        MUL221 = 0xdc,
        MUL222 = 0xdd,
        MUL223 = 0xde,
        MUL224 = 0xdf,
        MUL225 = 0xe0,
        MUL226 = 0xe1,
        MUL227 = 0xe2,
        MUL228 = 0xe3,
        MUL229 = 0xe4,
        MUL230 = 0xe5,
        MUL231 = 0xe6,
        MUL232 = 0xe7,
        MUL233 = 0xe8,
        MUL234 = 0xe9,
        MUL235 = 0xea,
        MUL236 = 0xeb,
        MUL237 = 0xec,
        MUL238 = 0xed,
        MUL239 = 0xee,
        MUL240 = 0xef,
        MUL241 = 0xf0,
        MUL242 = 0xf1,
        MUL243 = 0xf2,
        MUL244 = 0xf3,
        MUL245 = 0xf4,
        MUL246 = 0xf5,
        MUL247 = 0xf6,
        MUL248 = 0xf7,
        MUL249 = 0xf8,
        MUL250 = 0xf9,
        MUL251 = 0xfa,
        MUL252 = 0xfb,
        MUL253 = 0xfc,
        MUL254 = 0xfd,
        MUL255 = 0xfe,
        MUL256 = 0xff,
        MUL257 = 0x0100,
        MUL258 = 0x0101,
        MUL259 = 0x0102,
        MUL260 = 0x0103,
        MUL261 = 0x0104,
        MUL262 = 0x0105,
        MUL263 = 0x0106,
        MUL264 = 0x0107,
        MUL265 = 0x0108,
        MUL266 = 0x0109,
        MUL267 = 0x010a,
        MUL268 = 0x010b,
        MUL269 = 0x010c,
        MUL270 = 0x010d,
        MUL271 = 0x010e,
        MUL272 = 0x010f,
        MUL273 = 0x0110,
        MUL274 = 0x0111,
        MUL275 = 0x0112,
        MUL276 = 0x0113,
        MUL277 = 0x0114,
        MUL278 = 0x0115,
        MUL279 = 0x0116,
        MUL280 = 0x0117,
        MUL281 = 0x0118,
        MUL282 = 0x0119,
        MUL283 = 0x011a,
        MUL284 = 0x011b,
        MUL285 = 0x011c,
        MUL286 = 0x011d,
        MUL287 = 0x011e,
        MUL288 = 0x011f,
        MUL289 = 0x0120,
        MUL290 = 0x0121,
        MUL291 = 0x0122,
        MUL292 = 0x0123,
        MUL293 = 0x0124,
        MUL294 = 0x0125,
        MUL295 = 0x0126,
        MUL296 = 0x0127,
        MUL297 = 0x0128,
        MUL298 = 0x0129,
        MUL299 = 0x012a,
        MUL300 = 0x012b,
        MUL301 = 0x012c,
        MUL302 = 0x012d,
        MUL303 = 0x012e,
        MUL304 = 0x012f,
        MUL305 = 0x0130,
        MUL306 = 0x0131,
        MUL307 = 0x0132,
        MUL308 = 0x0133,
        MUL309 = 0x0134,
        MUL310 = 0x0135,
        MUL311 = 0x0136,
        MUL312 = 0x0137,
        MUL313 = 0x0138,
        MUL314 = 0x0139,
        MUL315 = 0x013a,
        MUL316 = 0x013b,
        MUL317 = 0x013c,
        MUL318 = 0x013d,
        MUL319 = 0x013e,
        MUL320 = 0x013f,
        MUL321 = 0x0140,
        MUL322 = 0x0141,
        MUL323 = 0x0142,
        MUL324 = 0x0143,
        MUL325 = 0x0144,
        MUL326 = 0x0145,
        MUL327 = 0x0146,
        MUL328 = 0x0147,
        MUL329 = 0x0148,
        MUL330 = 0x0149,
        MUL331 = 0x014a,
        MUL332 = 0x014b,
        MUL333 = 0x014c,
        MUL334 = 0x014d,
        MUL335 = 0x014e,
        MUL336 = 0x014f,
        MUL337 = 0x0150,
        MUL338 = 0x0151,
        MUL339 = 0x0152,
        MUL340 = 0x0153,
        MUL341 = 0x0154,
        MUL342 = 0x0155,
        MUL343 = 0x0156,
        MUL344 = 0x0157,
        MUL345 = 0x0158,
        MUL346 = 0x0159,
        MUL347 = 0x015a,
        MUL348 = 0x015b,
        MUL349 = 0x015c,
        MUL350 = 0x015d,
        MUL351 = 0x015e,
        MUL352 = 0x015f,
        MUL353 = 0x0160,
        MUL354 = 0x0161,
        MUL355 = 0x0162,
        MUL356 = 0x0163,
        MUL357 = 0x0164,
        MUL358 = 0x0165,
        MUL359 = 0x0166,
        MUL360 = 0x0167,
        MUL361 = 0x0168,
        MUL362 = 0x0169,
        MUL363 = 0x016a,
        MUL364 = 0x016b,
        MUL365 = 0x016c,
        MUL366 = 0x016d,
        MUL367 = 0x016e,
        MUL368 = 0x016f,
        MUL369 = 0x0170,
        MUL370 = 0x0171,
        MUL371 = 0x0172,
        MUL372 = 0x0173,
        MUL373 = 0x0174,
        MUL374 = 0x0175,
        MUL375 = 0x0176,
        MUL376 = 0x0177,
        MUL377 = 0x0178,
        MUL378 = 0x0179,
        MUL379 = 0x017a,
        MUL380 = 0x017b,
        MUL381 = 0x017c,
        MUL382 = 0x017d,
        MUL383 = 0x017e,
        MUL384 = 0x017f,
        MUL385 = 0x0180,
        MUL386 = 0x0181,
        MUL387 = 0x0182,
        MUL388 = 0x0183,
        MUL389 = 0x0184,
        MUL390 = 0x0185,
        MUL391 = 0x0186,
        MUL392 = 0x0187,
        MUL393 = 0x0188,
        MUL394 = 0x0189,
        MUL395 = 0x018a,
        MUL396 = 0x018b,
        MUL397 = 0x018c,
        MUL398 = 0x018d,
        MUL399 = 0x018e,
        MUL400 = 0x018f,
        MUL401 = 0x0190,
        MUL402 = 0x0191,
        MUL403 = 0x0192,
        MUL404 = 0x0193,
        MUL405 = 0x0194,
        MUL406 = 0x0195,
        MUL407 = 0x0196,
        MUL408 = 0x0197,
        MUL409 = 0x0198,
        MUL410 = 0x0199,
        MUL411 = 0x019a,
        MUL412 = 0x019b,
        MUL413 = 0x019c,
        MUL414 = 0x019d,
        MUL415 = 0x019e,
        MUL416 = 0x019f,
        MUL417 = 0x01a0,
        MUL418 = 0x01a1,
        MUL419 = 0x01a2,
        MUL420 = 0x01a3,
        _RESERVED_1a4 = 0x01a4,
        _RESERVED_1a5 = 0x01a5,
        _RESERVED_1a6 = 0x01a6,
        _RESERVED_1a7 = 0x01a7,
        _RESERVED_1a8 = 0x01a8,
        _RESERVED_1a9 = 0x01a9,
        _RESERVED_1aa = 0x01aa,
        _RESERVED_1ab = 0x01ab,
        _RESERVED_1ac = 0x01ac,
        _RESERVED_1ad = 0x01ad,
        _RESERVED_1ae = 0x01ae,
        _RESERVED_1af = 0x01af,
        _RESERVED_1b0 = 0x01b0,
        _RESERVED_1b1 = 0x01b1,
        _RESERVED_1b2 = 0x01b2,
        _RESERVED_1b3 = 0x01b3,
        _RESERVED_1b4 = 0x01b4,
        _RESERVED_1b5 = 0x01b5,
        _RESERVED_1b6 = 0x01b6,
        _RESERVED_1b7 = 0x01b7,
        _RESERVED_1b8 = 0x01b8,
        _RESERVED_1b9 = 0x01b9,
        _RESERVED_1ba = 0x01ba,
        _RESERVED_1bb = 0x01bb,
        _RESERVED_1bc = 0x01bc,
        _RESERVED_1bd = 0x01bd,
        _RESERVED_1be = 0x01be,
        _RESERVED_1bf = 0x01bf,
        _RESERVED_1c0 = 0x01c0,
        _RESERVED_1c1 = 0x01c1,
        _RESERVED_1c2 = 0x01c2,
        _RESERVED_1c3 = 0x01c3,
        _RESERVED_1c4 = 0x01c4,
        _RESERVED_1c5 = 0x01c5,
        _RESERVED_1c6 = 0x01c6,
        _RESERVED_1c7 = 0x01c7,
        _RESERVED_1c8 = 0x01c8,
        _RESERVED_1c9 = 0x01c9,
        _RESERVED_1ca = 0x01ca,
        _RESERVED_1cb = 0x01cb,
        _RESERVED_1cc = 0x01cc,
        _RESERVED_1cd = 0x01cd,
        _RESERVED_1ce = 0x01ce,
        _RESERVED_1cf = 0x01cf,
        _RESERVED_1d0 = 0x01d0,
        _RESERVED_1d1 = 0x01d1,
        _RESERVED_1d2 = 0x01d2,
        _RESERVED_1d3 = 0x01d3,
        _RESERVED_1d4 = 0x01d4,
        _RESERVED_1d5 = 0x01d5,
        _RESERVED_1d6 = 0x01d6,
        _RESERVED_1d7 = 0x01d7,
        _RESERVED_1d8 = 0x01d8,
        _RESERVED_1d9 = 0x01d9,
        _RESERVED_1da = 0x01da,
        _RESERVED_1db = 0x01db,
        _RESERVED_1dc = 0x01dc,
        _RESERVED_1dd = 0x01dd,
        _RESERVED_1de = 0x01de,
        _RESERVED_1df = 0x01df,
        _RESERVED_1e0 = 0x01e0,
        _RESERVED_1e1 = 0x01e1,
        _RESERVED_1e2 = 0x01e2,
        _RESERVED_1e3 = 0x01e3,
        _RESERVED_1e4 = 0x01e4,
        _RESERVED_1e5 = 0x01e5,
        _RESERVED_1e6 = 0x01e6,
        _RESERVED_1e7 = 0x01e7,
        _RESERVED_1e8 = 0x01e8,
        _RESERVED_1e9 = 0x01e9,
        _RESERVED_1ea = 0x01ea,
        _RESERVED_1eb = 0x01eb,
        _RESERVED_1ec = 0x01ec,
        _RESERVED_1ed = 0x01ed,
        _RESERVED_1ee = 0x01ee,
        _RESERVED_1ef = 0x01ef,
        _RESERVED_1f0 = 0x01f0,
        _RESERVED_1f1 = 0x01f1,
        _RESERVED_1f2 = 0x01f2,
        _RESERVED_1f3 = 0x01f3,
        _RESERVED_1f4 = 0x01f4,
        _RESERVED_1f5 = 0x01f5,
        _RESERVED_1f6 = 0x01f6,
        _RESERVED_1f7 = 0x01f7,
        _RESERVED_1f8 = 0x01f8,
        _RESERVED_1f9 = 0x01f9,
        _RESERVED_1fa = 0x01fa,
        _RESERVED_1fb = 0x01fb,
        _RESERVED_1fc = 0x01fc,
        _RESERVED_1fd = 0x01fd,
        _RESERVED_1fe = 0x01fe,
        _RESERVED_1ff = 0x01ff,
    }
    impl Plln {
        #[inline(always)]
        pub const fn from_bits(val: u16) -> Plln {
            unsafe { core::mem::transmute(val & 0x01ff) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u16 {
            unsafe { core::mem::transmute(self) }
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
    pub enum Pllrge {
        #[doc = "Frequency is between 1 and 2 MHz"]
        RANGE1 = 0x0,
        #[doc = "Frequency is between 2 and 4 MHz"]
        RANGE2 = 0x01,
        #[doc = "Frequency is between 4 and 8 MHz"]
        RANGE4 = 0x02,
        #[doc = "Frequency is between 8 and 16 MHz"]
        RANGE8 = 0x03,
    }
    impl Pllrge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrge {
        #[inline(always)]
        fn from(val: u8) -> Pllrge {
            Pllrge::from_bits(val)
        }
    }
    impl From<Pllrge> for u8 {
        #[inline(always)]
        fn from(val: Pllrge) -> u8 {
            Pllrge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "HSI selected as PLL clock"]
        HSI = 0x0,
        #[doc = "CSI selected as PLL clock"]
        CSI = 0x01,
        #[doc = "HSE selected as PLL clock"]
        HSE = 0x02,
        #[doc = "No clock sent to DIVMx dividers and PLLs"]
        DISABLE = 0x03,
    }
    impl Pllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsrc {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Pllvcosel {
        #[doc = "VCOH selected (default after reset)."]
        WIDE_VCO = 0x0,
        #[doc = "VCOL selected."]
        MEDIUM_VCO = 0x01,
    }
    impl Pllvcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllvcosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllvcosel {
        #[inline(always)]
        fn from(val: u8) -> Pllvcosel {
            Pllvcosel::from_bits(val)
        }
    }
    impl From<Pllvcosel> for u8 {
        #[inline(always)]
        fn from(val: Pllvcosel) -> u8 {
            Pllvcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre {
        #[doc = "rcc_hclk not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "rcc_hclk divided by 2"]
        DIV2 = 0x04,
        #[doc = "rcc_hclk divided by 4"]
        DIV4 = 0x05,
        #[doc = "rcc_hclk divided by 8"]
        DIV8 = 0x06,
        #[doc = "rcc_hclk divided by 16"]
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
    pub enum Pssisel {
        #[doc = "pll3_r_ck selected as kernel peripheral clock (default after reset)."]
        PLL3_R = 0x0,
        #[doc = "per_ck selected as kernel peripheral clock."]
        PER = 0x01,
    }
    impl Pssisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pssisel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pssisel {
        #[inline(always)]
        fn from(val: u8) -> Pssisel {
            Pssisel::from_bits(val)
        }
    }
    impl From<Pssisel> for u8 {
        #[inline(always)]
        fn from(val: Pssisel) -> u8 {
            Pssisel::to_bits(val)
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
    pub enum Sai1sel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_p selected as peripheral clock"]
        PLL3_P = 0x02,
        #[doc = "I2S_CKIN selected as peripheral clock"]
        I2S_CKIN = 0x03,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Sai1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sai1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sai1sel {
        #[inline(always)]
        fn from(val: u8) -> Sai1sel {
            Sai1sel::from_bits(val)
        }
    }
    impl From<Sai1sel> for u8 {
        #[inline(always)]
        fn from(val: Sai1sel) -> u8 {
            Sai1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sai2sel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_p selected as peripheral clock"]
        PLL3_P = 0x02,
        #[doc = "I2S_CKIN selected as peripheral clock"]
        I2S_CKIN = 0x03,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x04,
        #[doc = "spdifrx_symb_ck selected as SAI2 kernel clock."]
        SPDIFRX_SYMB = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Sai2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sai2sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sai2sel {
        #[inline(always)]
        fn from(val: u8) -> Sai2sel {
            Sai2sel::from_bits(val)
        }
    }
    impl From<Sai2sel> for u8 {
        #[inline(always)]
        fn from(val: Sai2sel) -> u8 {
            Sai2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdmmcsel {
        #[doc = "pll2_s_ck selected as kernel peripheral clock (default after reset)."]
        PLL2_S = 0x0,
        #[doc = "pll2_t_ck selected as kernel peripheral clock."]
        PLL2_T = 0x01,
    }
    impl Sdmmcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdmmcsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdmmcsel {
        #[inline(always)]
        fn from(val: u8) -> Sdmmcsel {
            Sdmmcsel::from_bits(val)
        }
    }
    impl From<Sdmmcsel> for u8 {
        #[inline(always)]
        fn from(val: Sdmmcsel) -> u8 {
            Sdmmcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spdifrxsel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x0,
        #[doc = "pll2_r selected as peripheral clock"]
        PLL2_R = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
    }
    impl Spdifrxsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spdifrxsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spdifrxsel {
        #[inline(always)]
        fn from(val: u8) -> Spdifrxsel {
            Spdifrxsel::from_bits(val)
        }
    }
    impl From<Spdifrxsel> for u8 {
        #[inline(always)]
        fn from(val: Spdifrxsel) -> u8 {
            Spdifrxsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi123sel {
        #[doc = "pll1_q_ck selected as SPI/I2S1 and 7 kernel clock (default after reset)."]
        PLL1_Q = 0x0,
        #[doc = "pll2_p_ck selected as SPI/I2S1 and 7 kernel clock."]
        PLL2_P = 0x01,
        #[doc = "pll3_p_ck selected as SPI/I2S1 and 7 kernel clock."]
        PLL3_P = 0x02,
        #[doc = "I2S_CKIN selected as SPI/I2S1 and 7 kernel clock."]
        I2S_CKIN = 0x03,
        #[doc = "per_ck selected as SPI/I2S1,and 7 kernel clock."]
        PER = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi123sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi123sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi123sel {
        #[inline(always)]
        fn from(val: u8) -> Spi123sel {
            Spi123sel::from_bits(val)
        }
    }
    impl From<Spi123sel> for u8 {
        #[inline(always)]
        fn from(val: Spi123sel) -> u8 {
            Spi123sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi45sel {
        #[doc = "APB2 clock selected as peripheral clock"]
        PCLK2 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi45sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi45sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi45sel {
        #[inline(always)]
        fn from(val: u8) -> Spi45sel {
            Spi45sel::from_bits(val)
        }
    }
    impl From<Spi45sel> for u8 {
        #[inline(always)]
        fn from(val: Spi45sel) -> u8 {
            Spi45sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi6sel {
        #[doc = "rcc_pclk4 selected as peripheral clock"]
        PCLK4 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi6sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi6sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi6sel {
        #[inline(always)]
        fn from(val: u8) -> Spi6sel {
            Spi6sel::from_bits(val)
        }
    }
    impl From<Spi6sel> for u8 {
        #[inline(always)]
        fn from(val: Spi6sel) -> u8 {
            Spi6sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopkerwuck {
        #[doc = "HSI selected as wake up clock from system Stop (default after reset)."]
        HSI = 0x0,
        #[doc = "CSI selected as wake up clock from system Stop."]
        CSI = 0x01,
    }
    impl Stopkerwuck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopkerwuck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopkerwuck {
        #[inline(always)]
        fn from(val: u8) -> Stopkerwuck {
            Stopkerwuck::from_bits(val)
        }
    }
    impl From<Stopkerwuck> for u8 {
        #[inline(always)]
        fn from(val: Stopkerwuck) -> u8 {
            Stopkerwuck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopwuck {
        #[doc = "HSI selected as wake up clock from system Stop"]
        HSI = 0x0,
        #[doc = "CSI selected as wake up clock from system Stop"]
        CSI = 0x01,
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
        #[doc = "HSI selected as system clock"]
        HSI = 0x0,
        #[doc = "CSI selected as system clock"]
        CSI = 0x01,
        #[doc = "HSE selected as system clock"]
        HSE = 0x02,
        #[doc = "PLL1 selected as system clock"]
        PLL1_P = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Timpre {
        #[doc = "Timer kernel clock equal to 2x pclk by default"]
        DEFAULT_X2 = 0x0,
        #[doc = "Timer kernel clock equal to 4x pclk by default"]
        DEFAULT_X4 = 0x01,
    }
    impl Timpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timpre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timpre {
        #[inline(always)]
        fn from(val: u8) -> Timpre {
            Timpre::from_bits(val)
        }
    }
    impl From<Timpre> for u8 {
        #[inline(always)]
        fn from(val: Timpre) -> u8 {
            Timpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart1sel {
        #[doc = "rcc_pclk2 selected as peripheral clock"]
        PCLK2 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usart1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart1sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart234578sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usart234578sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart234578sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart234578sel {
        #[inline(always)]
        fn from(val: u8) -> Usart234578sel {
            Usart234578sel::from_bits(val)
        }
    }
    impl From<Usart234578sel> for u8 {
        #[inline(always)]
        fn from(val: Usart234578sel) -> u8 {
            Usart234578sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum UsbOtgFssel {
        #[doc = "hsi48_ker_ck (default after reset)."]
        HSI48 = 0x0,
        #[doc = "pll3_q_ck."]
        PLL3_Q = 0x01,
        #[doc = "hse_ker_ck."]
        HSE = 0x02,
        #[doc = "clk48mohci."]
        CLK48MOHCI = 0x03,
    }
    impl UsbOtgFssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> UsbOtgFssel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for UsbOtgFssel {
        #[inline(always)]
        fn from(val: u8) -> UsbOtgFssel {
            UsbOtgFssel::from_bits(val)
        }
    }
    impl From<UsbOtgFssel> for u8 {
        #[inline(always)]
        fn from(val: UsbOtgFssel) -> u8 {
            UsbOtgFssel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usbpdctrl {
        #[doc = "In SUSPEND, PHY state machine, bias and USBPHYC PLL remain powered (default after reset)."]
        REMAIN_POWERED = 0x0,
        #[doc = "In SUSPEND, PHY state machine, bias and USBPHYC PLL are powered down."]
        POWER_DOWN = 0x01,
    }
    impl Usbpdctrl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbpdctrl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbpdctrl {
        #[inline(always)]
        fn from(val: u8) -> Usbpdctrl {
            Usbpdctrl::from_bits(val)
        }
    }
    impl From<Usbpdctrl> for u8 {
        #[inline(always)]
        fn from(val: Usbpdctrl) -> u8 {
            Usbpdctrl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usbphycsel {
        #[doc = "hse_ker_ck (default after reset)."]
        HSE = 0x0,
        #[doc = "hse_ker_ck / 2."]
        HSE_DIV_2 = 0x01,
        #[doc = "pll3_q_ck."]
        PLL3_Q = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Usbphycsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbphycsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbphycsel {
        #[inline(always)]
        fn from(val: u8) -> Usbphycsel {
            Usbphycsel::from_bits(val)
        }
    }
    impl From<Usbphycsel> for u8 {
        #[inline(always)]
        fn from(val: Usbphycsel) -> u8 {
            Usbphycsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usbrefcksel {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "The kernel clock frequency provided to the USBPHYC is 16 MHz."]
        MHZ16 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "The kernel clock frequency provided to the USBPHYC is 19.2 MHz."]
        MHZ19_2 = 0x08,
        #[doc = "The kernel clock frequency provided to the USBPHYC is 20MHz."]
        MHZ20 = 0x09,
        #[doc = "The kernel clock frequency provided to the USBPHYC is 24 MHz (default after reset)."]
        MHZ24 = 0x0a,
        #[doc = "The kernel clock frequency provided to the USBPHYC is 32 MHz."]
        MHZ32 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        #[doc = "The kernel clock frequency provided to the USBPHYC is 26 MHz."]
        MHZ26 = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Usbrefcksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbrefcksel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbrefcksel {
        #[inline(always)]
        fn from(val: u8) -> Usbrefcksel {
            Usbrefcksel::from_bits(val)
        }
    }
    impl From<Usbrefcksel> for u8 {
        #[inline(always)]
        fn from(val: Usbrefcksel) -> u8 {
            Usbrefcksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Xspisel {
        #[doc = "hclk5 selected as kernel peripheral clock (default after reset)."]
        HCLK5 = 0x0,
        #[doc = "pll2_s_ck selected as kernel peripheral clock."]
        PLL2_S = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Xspisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Xspisel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Xspisel {
        #[inline(always)]
        fn from(val: u8) -> Xspisel {
            Xspisel::from_bits(val)
        }
    }
    impl From<Xspisel> for u8 {
        #[inline(always)]
        fn from(val: Xspisel) -> u8 {
            Xspisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Xspiswp {
        #[doc = "The switch is in neutral mode and output clock is gated (default after reset)."]
        B_0X0 = 0x0,
        #[doc = "The switch is selecting hclk5."]
        B_0X1 = 0x01,
        #[doc = "The switch is selecting pll2_s_ck."]
        B_0X2 = 0x02,
        #[doc = "The switch is selecting pll2_t_ck."]
        B_0X3 = 0x03,
        #[doc = "The switch is in recovery position (hclk5/4)."]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Xspiswp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Xspiswp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Xspiswp {
        #[inline(always)]
        fn from(val: u8) -> Xspiswp {
            Xspiswp::from_bits(val)
        }
    }
    impl From<Xspiswp> for u8 {
        #[inline(always)]
        fn from(val: Xspiswp) -> u8 {
            Xspiswp::to_bits(val)
        }
    }
}
