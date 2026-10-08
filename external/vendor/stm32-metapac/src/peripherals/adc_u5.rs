#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ADC1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC interrupt and status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ADC interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ADC control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ADC configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ADC configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "sampling time register 1-2"]
    #[inline(always)]
    pub const fn smpr(self, n: usize) -> crate::common::Reg<regs::Smpr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "ADC channel preselection register."]
    #[inline(always)]
    pub const fn pcsel(self) -> crate::common::Reg<regs::Pcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "ADC regular sequence register 1."]
    #[inline(always)]
    pub const fn sqr1(self) -> crate::common::Reg<regs::Sqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "ADC regular sequence register 2."]
    #[inline(always)]
    pub const fn sqr2(self) -> crate::common::Reg<regs::Sqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "ADC regular sequence register 3."]
    #[inline(always)]
    pub const fn sqr3(self) -> crate::common::Reg<regs::Sqr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "ADC regular sequence register 4."]
    #[inline(always)]
    pub const fn sqr4(self) -> crate::common::Reg<regs::Sqr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "ADC regular Data Register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "ADC injected sequence register."]
    #[inline(always)]
    pub const fn jsqr(self) -> crate::common::Reg<regs::Jsqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "ADC offset register."]
    #[inline(always)]
    pub const fn ofr(self, n: usize) -> crate::common::Reg<regs::Ofr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "ADC gain compensation register."]
    #[inline(always)]
    pub const fn gcomp(self) -> crate::common::Reg<regs::Gcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "ADC injected data register."]
    #[inline(always)]
    pub const fn jdr(self, n: usize) -> crate::common::Reg<regs::Jdr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "ADC analog watchdog 2 configuration register."]
    #[inline(always)]
    pub const fn awd2cr(self) -> crate::common::Reg<regs::Awd2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "ADC analog watchdog 3 configuration register."]
    #[inline(always)]
    pub const fn awd3cr(self) -> crate::common::Reg<regs::Awd3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "ADC watchdog threshold register 1."]
    #[inline(always)]
    pub const fn ltr1(self) -> crate::common::Reg<regs::Ltr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "ADC watchdog threshold register 1."]
    #[inline(always)]
    pub const fn htr1(self) -> crate::common::Reg<regs::Htr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "ADC watchdog lower threshold register 2."]
    #[inline(always)]
    pub const fn ltr2(self) -> crate::common::Reg<regs::Ltr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "ADC watchdog higher threshold register 2."]
    #[inline(always)]
    pub const fn htr2(self) -> crate::common::Reg<regs::Htr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "ADC watchdog lower threshold register 3."]
    #[inline(always)]
    pub const fn ltr3(self) -> crate::common::Reg<regs::Ltr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "ADC watchdog higher threshold register 3."]
    #[inline(always)]
    pub const fn htr3(self) -> crate::common::Reg<regs::Htr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "ADC differential mode selection register."]
    #[inline(always)]
    pub const fn difsel(self) -> crate::common::Reg<regs::Difsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "ADC user control register."]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "ADC calibration factor register."]
    #[inline(always)]
    pub const fn calfact2(self) -> crate::common::Reg<regs::Calfact2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
}
#[doc = "ADC4."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc4 {
    ptr: *mut u8,
}
unsafe impl Send for Adc4 {}
unsafe impl Sync for Adc4 {}
impl Adc4 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC interrupt and status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Adc4Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ADC interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Adc4Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ADC control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Adc4Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ADC configuration register."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Adc4Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ADC configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Adc4Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "ADC sample time register."]
    #[inline(always)]
    pub const fn smpr(self) -> crate::common::Reg<regs::Adc4Smpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "ADC watchdog threshold register."]
    #[inline(always)]
    pub const fn awd1tr(self) -> crate::common::Reg<regs::Adc4Awdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "ADC watchdog threshold register."]
    #[inline(always)]
    pub const fn awd2tr(self) -> crate::common::Reg<regs::Adc4Awdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "ADC channel selection register \\[alternate\\]."]
    #[inline(always)]
    pub const fn chselrmod0(self) -> crate::common::Reg<regs::Adc4Chselrmod0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "ADC channel selection register \\[alternate\\]."]
    #[inline(always)]
    pub const fn chselrmod1(self) -> crate::common::Reg<regs::Adc4Chselrmod1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "ADC watchdog threshold register."]
    #[inline(always)]
    pub const fn awd3tr(self) -> crate::common::Reg<regs::Adc4Awdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "ADC data register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Adc4Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "ADC power register."]
    #[inline(always)]
    pub const fn pwrr(self) -> crate::common::Reg<regs::Adc4Pwrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "ADC Analog Watchdog 2 Configuration register."]
    #[inline(always)]
    pub const fn awd2cr(self) -> crate::common::Reg<regs::Adc4Awdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "ADC Analog Watchdog 3 Configuration register."]
    #[inline(always)]
    pub const fn awd3cr(self) -> crate::common::Reg<regs::Adc4Awdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "ADC Calibration factor."]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Adc4Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "ADC option register."]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::Adc4Or, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "ADC common configuration register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Adc4Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC Analog Watchdog Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Awdcr(pub u32);
    impl Adc4Awdcr {
        #[doc = "AWDCH0."]
        #[inline(always)]
        pub const fn awdch(&self, n: usize) -> bool {
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWDCH0."]
        #[inline(always)]
        pub fn set_awdch(&mut self, n: usize, val: bool) {
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Adc4Awdcr {
        #[inline(always)]
        fn default() -> Adc4Awdcr {
            Adc4Awdcr(0)
        }
    }
    impl core::fmt::Debug for Adc4Awdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Awdcr")
                .field("awdch[0]", &self.awdch(0usize))
                .field("awdch[1]", &self.awdch(1usize))
                .field("awdch[2]", &self.awdch(2usize))
                .field("awdch[3]", &self.awdch(3usize))
                .field("awdch[4]", &self.awdch(4usize))
                .field("awdch[5]", &self.awdch(5usize))
                .field("awdch[6]", &self.awdch(6usize))
                .field("awdch[7]", &self.awdch(7usize))
                .field("awdch[8]", &self.awdch(8usize))
                .field("awdch[9]", &self.awdch(9usize))
                .field("awdch[10]", &self.awdch(10usize))
                .field("awdch[11]", &self.awdch(11usize))
                .field("awdch[12]", &self.awdch(12usize))
                .field("awdch[13]", &self.awdch(13usize))
                .field("awdch[14]", &self.awdch(14usize))
                .field("awdch[15]", &self.awdch(15usize))
                .field("awdch[16]", &self.awdch(16usize))
                .field("awdch[17]", &self.awdch(17usize))
                .field("awdch[18]", &self.awdch(18usize))
                .field("awdch[19]", &self.awdch(19usize))
                .field("awdch[20]", &self.awdch(20usize))
                .field("awdch[21]", &self.awdch(21usize))
                .field("awdch[22]", &self.awdch(22usize))
                .field("awdch[23]", &self.awdch(23usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Awdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adc4Awdcr {{ awdch[0]: {=bool:?}, awdch[1]: {=bool:?}, awdch[2]: {=bool:?}, awdch[3]: {=bool:?}, awdch[4]: {=bool:?}, awdch[5]: {=bool:?}, awdch[6]: {=bool:?}, awdch[7]: {=bool:?}, awdch[8]: {=bool:?}, awdch[9]: {=bool:?}, awdch[10]: {=bool:?}, awdch[11]: {=bool:?}, awdch[12]: {=bool:?}, awdch[13]: {=bool:?}, awdch[14]: {=bool:?}, awdch[15]: {=bool:?}, awdch[16]: {=bool:?}, awdch[17]: {=bool:?}, awdch[18]: {=bool:?}, awdch[19]: {=bool:?}, awdch[20]: {=bool:?}, awdch[21]: {=bool:?}, awdch[22]: {=bool:?}, awdch[23]: {=bool:?} }}" , self . awdch (0usize) , self . awdch (1usize) , self . awdch (2usize) , self . awdch (3usize) , self . awdch (4usize) , self . awdch (5usize) , self . awdch (6usize) , self . awdch (7usize) , self . awdch (8usize) , self . awdch (9usize) , self . awdch (10usize) , self . awdch (11usize) , self . awdch (12usize) , self . awdch (13usize) , self . awdch (14usize) , self . awdch (15usize) , self . awdch (16usize) , self . awdch (17usize) , self . awdch (18usize) , self . awdch (19usize) , self . awdch (20usize) , self . awdch (21usize) , self . awdch (22usize) , self . awdch (23usize))
        }
    }
    #[doc = "ADC watchdog threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Awdtr(pub u32);
    impl Adc4Awdtr {
        #[doc = "LT3."]
        #[inline(always)]
        pub const fn lt3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "LT3."]
        #[inline(always)]
        pub fn set_lt3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HT3."]
        #[inline(always)]
        pub const fn ht3(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "HT3."]
        #[inline(always)]
        pub fn set_ht3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Adc4Awdtr {
        #[inline(always)]
        fn default() -> Adc4Awdtr {
            Adc4Awdtr(0)
        }
    }
    impl core::fmt::Debug for Adc4Awdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Awdtr")
                .field("lt3", &self.lt3())
                .field("ht3", &self.ht3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Awdtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Adc4Awdtr {{ lt3: {=u16:?}, ht3: {=u16:?} }}",
                self.lt3(),
                self.ht3()
            )
        }
    }
    #[doc = "ADC Calibration factor."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Calfact(pub u32);
    impl Adc4Calfact {
        #[doc = "CALFACT."]
        #[inline(always)]
        pub const fn calfact(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "CALFACT."]
        #[inline(always)]
        pub fn set_calfact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Adc4Calfact {
        #[inline(always)]
        fn default() -> Adc4Calfact {
            Adc4Calfact(0)
        }
    }
    impl core::fmt::Debug for Adc4Calfact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Calfact").field("calfact", &self.calfact()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Calfact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Adc4Calfact {{ calfact: {=u8:?} }}", self.calfact())
        }
    }
    #[doc = "ADC common configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Ccr(pub u32);
    impl Adc4Ccr {
        #[doc = "PRESC."]
        #[inline(always)]
        pub const fn presc(&self) -> super::vals::Adc4Presc {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Adc4Presc::from_bits(val as u8)
        }
        #[doc = "PRESC."]
        #[inline(always)]
        pub fn set_presc(&mut self, val: super::vals::Adc4Presc) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "VREFEN."]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFEN."]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "VSENSESEL."]
        #[inline(always)]
        pub const fn vsensesel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "VSENSESEL."]
        #[inline(always)]
        pub fn set_vsensesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "VBATEN."]
        #[inline(always)]
        pub const fn vbaten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VBATEN."]
        #[inline(always)]
        pub fn set_vbaten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Adc4Ccr {
        #[inline(always)]
        fn default() -> Adc4Ccr {
            Adc4Ccr(0)
        }
    }
    impl core::fmt::Debug for Adc4Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Ccr")
                .field("presc", &self.presc())
                .field("vrefen", &self.vrefen())
                .field("vsensesel", &self.vsensesel())
                .field("vbaten", &self.vbaten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Adc4Ccr {{ presc: {:?}, vrefen: {=bool:?}, vsensesel: {=bool:?}, vbaten: {=bool:?} }}",
                self.presc(),
                self.vrefen(),
                self.vsensesel(),
                self.vbaten()
            )
        }
    }
    #[doc = "ADC configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Cfgr1(pub u32);
    impl Adc4Cfgr1 {
        #[doc = "DMAEN."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMAEN."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMACFG."]
        #[inline(always)]
        pub const fn dmacfg(&self) -> super::vals::Adc4Dmacfg {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Adc4Dmacfg::from_bits(val as u8)
        }
        #[doc = "DMACFG."]
        #[inline(always)]
        pub fn set_dmacfg(&mut self, val: super::vals::Adc4Dmacfg) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "RES."]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Adc4Res {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Adc4Res::from_bits(val as u8)
        }
        #[doc = "RES."]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Adc4Res) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "SCANDIR."]
        #[inline(always)]
        pub const fn scandir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SCANDIR."]
        #[inline(always)]
        pub fn set_scandir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ALIGN."]
        #[inline(always)]
        pub const fn align(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ALIGN."]
        #[inline(always)]
        pub fn set_align(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EXTSEL."]
        #[inline(always)]
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "EXTSEL."]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "EXTEN."]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Adc4Exten {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Adc4Exten::from_bits(val as u8)
        }
        #[doc = "EXTEN."]
        #[inline(always)]
        pub fn set_exten(&mut self, val: super::vals::Adc4Exten) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "OVRMOD."]
        #[inline(always)]
        pub const fn ovrmod(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OVRMOD."]
        #[inline(always)]
        pub fn set_ovrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CONT."]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CONT."]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "WAIT."]
        #[inline(always)]
        pub const fn wait(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "WAIT."]
        #[inline(always)]
        pub fn set_wait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DISCEN."]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DISCEN."]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CHSELRMOD."]
        #[inline(always)]
        pub const fn chselrmod(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CHSELRMOD."]
        #[inline(always)]
        pub fn set_chselrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "AWD1SGL."]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "AWD1SGL."]
        #[inline(always)]
        pub fn set_awd1sgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "AWD1EN."]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "AWD1EN."]
        #[inline(always)]
        pub fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "AWD1CH."]
        #[inline(always)]
        pub const fn awd1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "AWD1CH."]
        #[inline(always)]
        pub fn set_awd1ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
    }
    impl Default for Adc4Cfgr1 {
        #[inline(always)]
        fn default() -> Adc4Cfgr1 {
            Adc4Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Adc4Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Cfgr1")
                .field("dmaen", &self.dmaen())
                .field("dmacfg", &self.dmacfg())
                .field("res", &self.res())
                .field("scandir", &self.scandir())
                .field("align", &self.align())
                .field("extsel", &self.extsel())
                .field("exten", &self.exten())
                .field("ovrmod", &self.ovrmod())
                .field("cont", &self.cont())
                .field("wait", &self.wait())
                .field("discen", &self.discen())
                .field("chselrmod", &self.chselrmod())
                .field("awd1sgl", &self.awd1sgl())
                .field("awd1en", &self.awd1en())
                .field("awd1ch", &self.awd1ch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adc4Cfgr1 {{ dmaen: {=bool:?}, dmacfg: {:?}, res: {:?}, scandir: {=bool:?}, align: {=bool:?}, extsel: {=u8:?}, exten: {:?}, ovrmod: {=bool:?}, cont: {=bool:?}, wait: {=bool:?}, discen: {=bool:?}, chselrmod: {=bool:?}, awd1sgl: {=bool:?}, awd1en: {=bool:?}, awd1ch: {=u8:?} }}" , self . dmaen () , self . dmacfg () , self . res () , self . scandir () , self . align () , self . extsel () , self . exten () , self . ovrmod () , self . cont () , self . wait () , self . discen () , self . chselrmod () , self . awd1sgl () , self . awd1en () , self . awd1ch ())
        }
    }
    #[doc = "ADC configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Cfgr2(pub u32);
    impl Adc4Cfgr2 {
        #[doc = "OVSE."]
        #[inline(always)]
        pub const fn ovse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OVSE."]
        #[inline(always)]
        pub fn set_ovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OVSR."]
        #[inline(always)]
        pub const fn ovsr(&self) -> super::vals::Adc4OversamplingRatio {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::Adc4OversamplingRatio::from_bits(val as u8)
        }
        #[doc = "OVSR."]
        #[inline(always)]
        pub fn set_ovsr(&mut self, val: super::vals::Adc4OversamplingRatio) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
        }
        #[doc = "OVSS."]
        #[inline(always)]
        pub const fn ovss(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "OVSS."]
        #[inline(always)]
        pub fn set_ovss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "TOVS."]
        #[inline(always)]
        pub const fn tovs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TOVS."]
        #[inline(always)]
        pub fn set_tovs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LFTRIG."]
        #[inline(always)]
        pub const fn lftrig(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "LFTRIG."]
        #[inline(always)]
        pub fn set_lftrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Adc4Cfgr2 {
        #[inline(always)]
        fn default() -> Adc4Cfgr2 {
            Adc4Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Adc4Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Cfgr2")
                .field("ovse", &self.ovse())
                .field("ovsr", &self.ovsr())
                .field("ovss", &self.ovss())
                .field("tovs", &self.tovs())
                .field("lftrig", &self.lftrig())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Adc4Cfgr2 {{ ovse: {=bool:?}, ovsr: {:?}, ovss: {=u8:?}, tovs: {=bool:?}, lftrig: {=bool:?} }}",
                self.ovse(),
                self.ovsr(),
                self.ovss(),
                self.tovs(),
                self.lftrig()
            )
        }
    }
    #[doc = "ADC channel selection register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Chselrmod0(pub u32);
    impl Adc4Chselrmod0 {
        #[doc = "CHSEL."]
        #[inline(always)]
        pub const fn chsel(&self, n: usize) -> bool {
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "CHSEL."]
        #[inline(always)]
        pub fn set_chsel(&mut self, n: usize, val: bool) {
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Adc4Chselrmod0 {
        #[inline(always)]
        fn default() -> Adc4Chselrmod0 {
            Adc4Chselrmod0(0)
        }
    }
    impl core::fmt::Debug for Adc4Chselrmod0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Chselrmod0")
                .field("chsel[0]", &self.chsel(0usize))
                .field("chsel[1]", &self.chsel(1usize))
                .field("chsel[2]", &self.chsel(2usize))
                .field("chsel[3]", &self.chsel(3usize))
                .field("chsel[4]", &self.chsel(4usize))
                .field("chsel[5]", &self.chsel(5usize))
                .field("chsel[6]", &self.chsel(6usize))
                .field("chsel[7]", &self.chsel(7usize))
                .field("chsel[8]", &self.chsel(8usize))
                .field("chsel[9]", &self.chsel(9usize))
                .field("chsel[10]", &self.chsel(10usize))
                .field("chsel[11]", &self.chsel(11usize))
                .field("chsel[12]", &self.chsel(12usize))
                .field("chsel[13]", &self.chsel(13usize))
                .field("chsel[14]", &self.chsel(14usize))
                .field("chsel[15]", &self.chsel(15usize))
                .field("chsel[16]", &self.chsel(16usize))
                .field("chsel[17]", &self.chsel(17usize))
                .field("chsel[18]", &self.chsel(18usize))
                .field("chsel[19]", &self.chsel(19usize))
                .field("chsel[20]", &self.chsel(20usize))
                .field("chsel[21]", &self.chsel(21usize))
                .field("chsel[22]", &self.chsel(22usize))
                .field("chsel[23]", &self.chsel(23usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Chselrmod0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adc4Chselrmod0 {{ chsel[0]: {=bool:?}, chsel[1]: {=bool:?}, chsel[2]: {=bool:?}, chsel[3]: {=bool:?}, chsel[4]: {=bool:?}, chsel[5]: {=bool:?}, chsel[6]: {=bool:?}, chsel[7]: {=bool:?}, chsel[8]: {=bool:?}, chsel[9]: {=bool:?}, chsel[10]: {=bool:?}, chsel[11]: {=bool:?}, chsel[12]: {=bool:?}, chsel[13]: {=bool:?}, chsel[14]: {=bool:?}, chsel[15]: {=bool:?}, chsel[16]: {=bool:?}, chsel[17]: {=bool:?}, chsel[18]: {=bool:?}, chsel[19]: {=bool:?}, chsel[20]: {=bool:?}, chsel[21]: {=bool:?}, chsel[22]: {=bool:?}, chsel[23]: {=bool:?} }}" , self . chsel (0usize) , self . chsel (1usize) , self . chsel (2usize) , self . chsel (3usize) , self . chsel (4usize) , self . chsel (5usize) , self . chsel (6usize) , self . chsel (7usize) , self . chsel (8usize) , self . chsel (9usize) , self . chsel (10usize) , self . chsel (11usize) , self . chsel (12usize) , self . chsel (13usize) , self . chsel (14usize) , self . chsel (15usize) , self . chsel (16usize) , self . chsel (17usize) , self . chsel (18usize) , self . chsel (19usize) , self . chsel (20usize) , self . chsel (21usize) , self . chsel (22usize) , self . chsel (23usize))
        }
    }
    #[doc = "ADC channel selection register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Chselrmod1(pub u32);
    impl Adc4Chselrmod1 {
        #[doc = "SQ"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "SQ"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Adc4Chselrmod1 {
        #[inline(always)]
        fn default() -> Adc4Chselrmod1 {
            Adc4Chselrmod1(0)
        }
    }
    impl core::fmt::Debug for Adc4Chselrmod1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Chselrmod1")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .field("sq[5]", &self.sq(5usize))
                .field("sq[6]", &self.sq(6usize))
                .field("sq[7]", &self.sq(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Chselrmod1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adc4Chselrmod1 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?}, sq[5]: {=u8:?}, sq[6]: {=u8:?}, sq[7]: {=u8:?} }}" , self . sq (0usize) , self . sq (1usize) , self . sq (2usize) , self . sq (3usize) , self . sq (4usize) , self . sq (5usize) , self . sq (6usize) , self . sq (7usize))
        }
    }
    #[doc = "ADC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Cr(pub u32);
    impl Adc4Cr {
        #[doc = "ADEN."]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADEN."]
        #[inline(always)]
        pub fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADDIS."]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADDIS."]
        #[inline(always)]
        pub fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADSTART."]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADSTART."]
        #[inline(always)]
        pub fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADSTP."]
        #[inline(always)]
        pub const fn adstp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADSTP."]
        #[inline(always)]
        pub fn set_adstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADVREGEN."]
        #[inline(always)]
        pub const fn advregen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADVREGEN."]
        #[inline(always)]
        pub fn set_advregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADCAL."]
        #[inline(always)]
        pub const fn adcal(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADCAL."]
        #[inline(always)]
        pub fn set_adcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Adc4Cr {
        #[inline(always)]
        fn default() -> Adc4Cr {
            Adc4Cr(0)
        }
    }
    impl core::fmt::Debug for Adc4Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Cr")
                .field("aden", &self.aden())
                .field("addis", &self.addis())
                .field("adstart", &self.adstart())
                .field("adstp", &self.adstp())
                .field("advregen", &self.advregen())
                .field("adcal", &self.adcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adc4Cr {{ aden: {=bool:?}, addis: {=bool:?}, adstart: {=bool:?}, adstp: {=bool:?}, advregen: {=bool:?}, adcal: {=bool:?} }}" , self . aden () , self . addis () , self . adstart () , self . adstp () , self . advregen () , self . adcal ())
        }
    }
    #[doc = "ADC data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Dr(pub u32);
    impl Adc4Dr {
        #[doc = "DATA."]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DATA."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Adc4Dr {
        #[inline(always)]
        fn default() -> Adc4Dr {
            Adc4Dr(0)
        }
    }
    impl core::fmt::Debug for Adc4Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Dr").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Adc4Dr {{ data: {=u16:?} }}", self.data())
        }
    }
    #[doc = "ADC interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Ier(pub u32);
    impl Adc4Ier {
        #[doc = "ADRDYIE."]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADRDYIE."]
        #[inline(always)]
        pub fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EOSMPIE."]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EOSMPIE."]
        #[inline(always)]
        pub fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EOCIE."]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EOCIE."]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOSIE."]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOSIE."]
        #[inline(always)]
        pub fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OVRIE."]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OVRIE."]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AWD1IE."]
        #[inline(always)]
        pub const fn awdie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWD1IE."]
        #[inline(always)]
        pub fn set_awdie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "EOCALIE."]
        #[inline(always)]
        pub const fn eocalie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "EOCALIE."]
        #[inline(always)]
        pub fn set_eocalie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LDORDYIE."]
        #[inline(always)]
        pub const fn ldordyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LDORDYIE."]
        #[inline(always)]
        pub fn set_ldordyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Adc4Ier {
        #[inline(always)]
        fn default() -> Adc4Ier {
            Adc4Ier(0)
        }
    }
    impl core::fmt::Debug for Adc4Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Ier")
                .field("adrdyie", &self.adrdyie())
                .field("eosmpie", &self.eosmpie())
                .field("eocie", &self.eocie())
                .field("eosie", &self.eosie())
                .field("ovrie", &self.ovrie())
                .field("awdie[0]", &self.awdie(0usize))
                .field("awdie[1]", &self.awdie(1usize))
                .field("awdie[2]", &self.awdie(2usize))
                .field("eocalie", &self.eocalie())
                .field("ldordyie", &self.ldordyie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adc4Ier {{ adrdyie: {=bool:?}, eosmpie: {=bool:?}, eocie: {=bool:?}, eosie: {=bool:?}, ovrie: {=bool:?}, awdie[0]: {=bool:?}, awdie[1]: {=bool:?}, awdie[2]: {=bool:?}, eocalie: {=bool:?}, ldordyie: {=bool:?} }}" , self . adrdyie () , self . eosmpie () , self . eocie () , self . eosie () , self . ovrie () , self . awdie (0usize) , self . awdie (1usize) , self . awdie (2usize) , self . eocalie () , self . ldordyie ())
        }
    }
    #[doc = "ADC interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Isr(pub u32);
    impl Adc4Isr {
        #[doc = "ADRDY."]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADRDY."]
        #[inline(always)]
        pub fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EOSMP."]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EOSMP."]
        #[inline(always)]
        pub fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EOC."]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EOC."]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOS."]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOS."]
        #[inline(always)]
        pub fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OVR."]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OVR."]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AWD1."]
        #[inline(always)]
        pub const fn awd(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWD1."]
        #[inline(always)]
        pub fn set_awd(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "EOCAL."]
        #[inline(always)]
        pub const fn eocal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "EOCAL."]
        #[inline(always)]
        pub fn set_eocal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LDORDY."]
        #[inline(always)]
        pub const fn ldordy(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LDORDY."]
        #[inline(always)]
        pub fn set_ldordy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Adc4Isr {
        #[inline(always)]
        fn default() -> Adc4Isr {
            Adc4Isr(0)
        }
    }
    impl core::fmt::Debug for Adc4Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Isr")
                .field("adrdy", &self.adrdy())
                .field("eosmp", &self.eosmp())
                .field("eoc", &self.eoc())
                .field("eos", &self.eos())
                .field("ovr", &self.ovr())
                .field("awd[0]", &self.awd(0usize))
                .field("awd[1]", &self.awd(1usize))
                .field("awd[2]", &self.awd(2usize))
                .field("eocal", &self.eocal())
                .field("ldordy", &self.ldordy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adc4Isr {{ adrdy: {=bool:?}, eosmp: {=bool:?}, eoc: {=bool:?}, eos: {=bool:?}, ovr: {=bool:?}, awd[0]: {=bool:?}, awd[1]: {=bool:?}, awd[2]: {=bool:?}, eocal: {=bool:?}, ldordy: {=bool:?} }}" , self . adrdy () , self . eosmp () , self . eoc () , self . eos () , self . ovr () , self . awd (0usize) , self . awd (1usize) , self . awd (2usize) , self . eocal () , self . ldordy ())
        }
    }
    #[doc = "ADC option register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Or(pub u32);
    impl Adc4Or {
        #[doc = "CHN21SEL."]
        #[inline(always)]
        pub const fn chn21sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CHN21SEL."]
        #[inline(always)]
        pub fn set_chn21sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Adc4Or {
        #[inline(always)]
        fn default() -> Adc4Or {
            Adc4Or(0)
        }
    }
    impl core::fmt::Debug for Adc4Or {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Or").field("chn21sel", &self.chn21sel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Or {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Adc4Or {{ chn21sel: {=bool:?} }}", self.chn21sel())
        }
    }
    #[doc = "ADC data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Pwrr(pub u32);
    impl Adc4Pwrr {
        #[doc = "AUTOFF."]
        #[inline(always)]
        pub const fn autoff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AUTOFF."]
        #[inline(always)]
        pub fn set_autoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DPD."]
        #[inline(always)]
        pub const fn dpd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DPD."]
        #[inline(always)]
        pub fn set_dpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VREFPROT."]
        #[inline(always)]
        pub const fn vrefprot(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VREFPROT."]
        #[inline(always)]
        pub fn set_vrefprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VREFSECSMP."]
        #[inline(always)]
        pub const fn vrefsecsmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VREFSECSMP."]
        #[inline(always)]
        pub fn set_vrefsecsmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Adc4Pwrr {
        #[inline(always)]
        fn default() -> Adc4Pwrr {
            Adc4Pwrr(0)
        }
    }
    impl core::fmt::Debug for Adc4Pwrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Pwrr")
                .field("autoff", &self.autoff())
                .field("dpd", &self.dpd())
                .field("vrefprot", &self.vrefprot())
                .field("vrefsecsmp", &self.vrefsecsmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Pwrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Adc4Pwrr {{ autoff: {=bool:?}, dpd: {=bool:?}, vrefprot: {=bool:?}, vrefsecsmp: {=bool:?} }}",
                self.autoff(),
                self.dpd(),
                self.vrefprot(),
                self.vrefsecsmp()
            )
        }
    }
    #[doc = "ADC sample time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc4Smpr(pub u32);
    impl Adc4Smpr {
        #[doc = "SMP1."]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::Adc4SampleTime {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Adc4SampleTime::from_bits(val as u8)
        }
        #[doc = "SMP1."]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::Adc4SampleTime) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Channel sampling time selection"]
        #[inline(always)]
        pub const fn smpsel(&self, n: usize) -> bool {
            assert!(n < 24usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel sampling time selection"]
        #[inline(always)]
        pub fn set_smpsel(&mut self, n: usize, val: bool) {
            assert!(n < 24usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Adc4Smpr {
        #[inline(always)]
        fn default() -> Adc4Smpr {
            Adc4Smpr(0)
        }
    }
    impl core::fmt::Debug for Adc4Smpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adc4Smpr")
                .field("smp[0]", &self.smp(0usize))
                .field("smp[1]", &self.smp(1usize))
                .field("smpsel[0]", &self.smpsel(0usize))
                .field("smpsel[1]", &self.smpsel(1usize))
                .field("smpsel[2]", &self.smpsel(2usize))
                .field("smpsel[3]", &self.smpsel(3usize))
                .field("smpsel[4]", &self.smpsel(4usize))
                .field("smpsel[5]", &self.smpsel(5usize))
                .field("smpsel[6]", &self.smpsel(6usize))
                .field("smpsel[7]", &self.smpsel(7usize))
                .field("smpsel[8]", &self.smpsel(8usize))
                .field("smpsel[9]", &self.smpsel(9usize))
                .field("smpsel[10]", &self.smpsel(10usize))
                .field("smpsel[11]", &self.smpsel(11usize))
                .field("smpsel[12]", &self.smpsel(12usize))
                .field("smpsel[13]", &self.smpsel(13usize))
                .field("smpsel[14]", &self.smpsel(14usize))
                .field("smpsel[15]", &self.smpsel(15usize))
                .field("smpsel[16]", &self.smpsel(16usize))
                .field("smpsel[17]", &self.smpsel(17usize))
                .field("smpsel[18]", &self.smpsel(18usize))
                .field("smpsel[19]", &self.smpsel(19usize))
                .field("smpsel[20]", &self.smpsel(20usize))
                .field("smpsel[21]", &self.smpsel(21usize))
                .field("smpsel[22]", &self.smpsel(22usize))
                .field("smpsel[23]", &self.smpsel(23usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adc4Smpr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adc4Smpr {{ smp[0]: {:?}, smp[1]: {:?}, smpsel[0]: {=bool:?}, smpsel[1]: {=bool:?}, smpsel[2]: {=bool:?}, smpsel[3]: {=bool:?}, smpsel[4]: {=bool:?}, smpsel[5]: {=bool:?}, smpsel[6]: {=bool:?}, smpsel[7]: {=bool:?}, smpsel[8]: {=bool:?}, smpsel[9]: {=bool:?}, smpsel[10]: {=bool:?}, smpsel[11]: {=bool:?}, smpsel[12]: {=bool:?}, smpsel[13]: {=bool:?}, smpsel[14]: {=bool:?}, smpsel[15]: {=bool:?}, smpsel[16]: {=bool:?}, smpsel[17]: {=bool:?}, smpsel[18]: {=bool:?}, smpsel[19]: {=bool:?}, smpsel[20]: {=bool:?}, smpsel[21]: {=bool:?}, smpsel[22]: {=bool:?}, smpsel[23]: {=bool:?} }}" , self . smp (0usize) , self . smp (1usize) , self . smpsel (0usize) , self . smpsel (1usize) , self . smpsel (2usize) , self . smpsel (3usize) , self . smpsel (4usize) , self . smpsel (5usize) , self . smpsel (6usize) , self . smpsel (7usize) , self . smpsel (8usize) , self . smpsel (9usize) , self . smpsel (10usize) , self . smpsel (11usize) , self . smpsel (12usize) , self . smpsel (13usize) , self . smpsel (14usize) , self . smpsel (15usize) , self . smpsel (16usize) , self . smpsel (17usize) , self . smpsel (18usize) , self . smpsel (19usize) , self . smpsel (20usize) , self . smpsel (21usize) , self . smpsel (22usize) , self . smpsel (23usize))
        }
    }
    #[doc = "ADC analog watchdog 2 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2cr(pub u32);
    impl Awd2cr {
        #[doc = "AWD2CH."]
        #[inline(always)]
        pub const fn awd2ch(&self, n: usize) -> bool {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWD2CH."]
        #[inline(always)]
        pub fn set_awd2ch(&mut self, n: usize, val: bool) {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Awd2cr {
        #[inline(always)]
        fn default() -> Awd2cr {
            Awd2cr(0)
        }
    }
    impl core::fmt::Debug for Awd2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd2cr")
                .field("awd2ch[0]", &self.awd2ch(0usize))
                .field("awd2ch[1]", &self.awd2ch(1usize))
                .field("awd2ch[2]", &self.awd2ch(2usize))
                .field("awd2ch[3]", &self.awd2ch(3usize))
                .field("awd2ch[4]", &self.awd2ch(4usize))
                .field("awd2ch[5]", &self.awd2ch(5usize))
                .field("awd2ch[6]", &self.awd2ch(6usize))
                .field("awd2ch[7]", &self.awd2ch(7usize))
                .field("awd2ch[8]", &self.awd2ch(8usize))
                .field("awd2ch[9]", &self.awd2ch(9usize))
                .field("awd2ch[10]", &self.awd2ch(10usize))
                .field("awd2ch[11]", &self.awd2ch(11usize))
                .field("awd2ch[12]", &self.awd2ch(12usize))
                .field("awd2ch[13]", &self.awd2ch(13usize))
                .field("awd2ch[14]", &self.awd2ch(14usize))
                .field("awd2ch[15]", &self.awd2ch(15usize))
                .field("awd2ch[16]", &self.awd2ch(16usize))
                .field("awd2ch[17]", &self.awd2ch(17usize))
                .field("awd2ch[18]", &self.awd2ch(18usize))
                .field("awd2ch[19]", &self.awd2ch(19usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Awd2cr {{ awd2ch[0]: {=bool:?}, awd2ch[1]: {=bool:?}, awd2ch[2]: {=bool:?}, awd2ch[3]: {=bool:?}, awd2ch[4]: {=bool:?}, awd2ch[5]: {=bool:?}, awd2ch[6]: {=bool:?}, awd2ch[7]: {=bool:?}, awd2ch[8]: {=bool:?}, awd2ch[9]: {=bool:?}, awd2ch[10]: {=bool:?}, awd2ch[11]: {=bool:?}, awd2ch[12]: {=bool:?}, awd2ch[13]: {=bool:?}, awd2ch[14]: {=bool:?}, awd2ch[15]: {=bool:?}, awd2ch[16]: {=bool:?}, awd2ch[17]: {=bool:?}, awd2ch[18]: {=bool:?}, awd2ch[19]: {=bool:?} }}" , self . awd2ch (0usize) , self . awd2ch (1usize) , self . awd2ch (2usize) , self . awd2ch (3usize) , self . awd2ch (4usize) , self . awd2ch (5usize) , self . awd2ch (6usize) , self . awd2ch (7usize) , self . awd2ch (8usize) , self . awd2ch (9usize) , self . awd2ch (10usize) , self . awd2ch (11usize) , self . awd2ch (12usize) , self . awd2ch (13usize) , self . awd2ch (14usize) , self . awd2ch (15usize) , self . awd2ch (16usize) , self . awd2ch (17usize) , self . awd2ch (18usize) , self . awd2ch (19usize))
        }
    }
    #[doc = "ADC analog watchdog 3 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3cr(pub u32);
    impl Awd3cr {
        #[doc = "AWD3CH."]
        #[inline(always)]
        pub const fn awd3ch(&self, n: usize) -> bool {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWD3CH."]
        #[inline(always)]
        pub fn set_awd3ch(&mut self, n: usize, val: bool) {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Awd3cr {
        #[inline(always)]
        fn default() -> Awd3cr {
            Awd3cr(0)
        }
    }
    impl core::fmt::Debug for Awd3cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd3cr")
                .field("awd3ch[0]", &self.awd3ch(0usize))
                .field("awd3ch[1]", &self.awd3ch(1usize))
                .field("awd3ch[2]", &self.awd3ch(2usize))
                .field("awd3ch[3]", &self.awd3ch(3usize))
                .field("awd3ch[4]", &self.awd3ch(4usize))
                .field("awd3ch[5]", &self.awd3ch(5usize))
                .field("awd3ch[6]", &self.awd3ch(6usize))
                .field("awd3ch[7]", &self.awd3ch(7usize))
                .field("awd3ch[8]", &self.awd3ch(8usize))
                .field("awd3ch[9]", &self.awd3ch(9usize))
                .field("awd3ch[10]", &self.awd3ch(10usize))
                .field("awd3ch[11]", &self.awd3ch(11usize))
                .field("awd3ch[12]", &self.awd3ch(12usize))
                .field("awd3ch[13]", &self.awd3ch(13usize))
                .field("awd3ch[14]", &self.awd3ch(14usize))
                .field("awd3ch[15]", &self.awd3ch(15usize))
                .field("awd3ch[16]", &self.awd3ch(16usize))
                .field("awd3ch[17]", &self.awd3ch(17usize))
                .field("awd3ch[18]", &self.awd3ch(18usize))
                .field("awd3ch[19]", &self.awd3ch(19usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Awd3cr {{ awd3ch[0]: {=bool:?}, awd3ch[1]: {=bool:?}, awd3ch[2]: {=bool:?}, awd3ch[3]: {=bool:?}, awd3ch[4]: {=bool:?}, awd3ch[5]: {=bool:?}, awd3ch[6]: {=bool:?}, awd3ch[7]: {=bool:?}, awd3ch[8]: {=bool:?}, awd3ch[9]: {=bool:?}, awd3ch[10]: {=bool:?}, awd3ch[11]: {=bool:?}, awd3ch[12]: {=bool:?}, awd3ch[13]: {=bool:?}, awd3ch[14]: {=bool:?}, awd3ch[15]: {=bool:?}, awd3ch[16]: {=bool:?}, awd3ch[17]: {=bool:?}, awd3ch[18]: {=bool:?}, awd3ch[19]: {=bool:?} }}" , self . awd3ch (0usize) , self . awd3ch (1usize) , self . awd3ch (2usize) , self . awd3ch (3usize) , self . awd3ch (4usize) , self . awd3ch (5usize) , self . awd3ch (6usize) , self . awd3ch (7usize) , self . awd3ch (8usize) , self . awd3ch (9usize) , self . awd3ch (10usize) , self . awd3ch (11usize) , self . awd3ch (12usize) , self . awd3ch (13usize) , self . awd3ch (14usize) , self . awd3ch (15usize) , self . awd3ch (16usize) , self . awd3ch (17usize) , self . awd3ch (18usize) , self . awd3ch (19usize))
        }
    }
    #[doc = "ADC user control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact(pub u32);
    impl Calfact {
        #[doc = "I_APB_ADDR."]
        #[inline(always)]
        pub const fn i_apb_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "I_APB_ADDR."]
        #[inline(always)]
        pub fn set_i_apb_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "I_APB_DATA."]
        #[inline(always)]
        pub const fn i_apb_data(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "I_APB_DATA."]
        #[inline(always)]
        pub fn set_i_apb_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "VALIDITY."]
        #[inline(always)]
        pub const fn validity(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VALIDITY."]
        #[inline(always)]
        pub fn set_validity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LATCH_COEF."]
        #[inline(always)]
        pub const fn latch_coef(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "LATCH_COEF."]
        #[inline(always)]
        pub fn set_latch_coef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAPTURE_COEF."]
        #[inline(always)]
        pub const fn capture_coef(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAPTURE_COEF."]
        #[inline(always)]
        pub fn set_capture_coef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Calfact {
        #[inline(always)]
        fn default() -> Calfact {
            Calfact(0)
        }
    }
    impl core::fmt::Debug for Calfact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calfact")
                .field("i_apb_addr", &self.i_apb_addr())
                .field("i_apb_data", &self.i_apb_data())
                .field("validity", &self.validity())
                .field("latch_coef", &self.latch_coef())
                .field("capture_coef", &self.capture_coef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calfact {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Calfact {{ i_apb_addr: {=u8:?}, i_apb_data: {=u8:?}, validity: {=bool:?}, latch_coef: {=bool:?}, capture_coef: {=bool:?} }}" , self . i_apb_addr () , self . i_apb_data () , self . validity () , self . latch_coef () , self . capture_coef ())
        }
    }
    #[doc = "ADC calibration factor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact2(pub u32);
    impl Calfact2 {
        #[doc = "CALFACT."]
        #[inline(always)]
        pub const fn calfact(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CALFACT."]
        #[inline(always)]
        pub fn set_calfact(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Calfact2 {
        #[inline(always)]
        fn default() -> Calfact2 {
            Calfact2(0)
        }
    }
    impl core::fmt::Debug for Calfact2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calfact2").field("calfact", &self.calfact()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calfact2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Calfact2 {{ calfact: {=u32:?} }}", self.calfact())
        }
    }
    #[doc = "ADC configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "DMNGT."]
        #[inline(always)]
        pub const fn dmngt(&self) -> super::vals::Dmngt {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Dmngt::from_bits(val as u8)
        }
        #[doc = "DMNGT."]
        #[inline(always)]
        pub fn set_dmngt(&mut self, val: super::vals::Dmngt) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "RES."]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "RES."]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "EXTSEL."]
        #[inline(always)]
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "EXTSEL."]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "EXTEN."]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Exten {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "EXTEN."]
        #[inline(always)]
        pub fn set_exten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "OVRMOD."]
        #[inline(always)]
        pub const fn ovrmod(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OVRMOD."]
        #[inline(always)]
        pub fn set_ovrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CONT."]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CONT."]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AUTDLY."]
        #[inline(always)]
        pub const fn autdly(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "AUTDLY."]
        #[inline(always)]
        pub fn set_autdly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DISCEN."]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DISCEN."]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DISCNUM."]
        #[inline(always)]
        pub const fn discnum(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "DISCNUM."]
        #[inline(always)]
        pub fn set_discnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "JDISCEN."]
        #[inline(always)]
        pub const fn jdiscen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "JDISCEN."]
        #[inline(always)]
        pub fn set_jdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "AWD1SGL."]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "AWD1SGL."]
        #[inline(always)]
        pub fn set_awd1sgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "AWD1EN."]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "AWD1EN."]
        #[inline(always)]
        pub fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "JAWD1EN."]
        #[inline(always)]
        pub const fn jawd1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "JAWD1EN."]
        #[inline(always)]
        pub fn set_jawd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "JAUTO."]
        #[inline(always)]
        pub const fn jauto(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "JAUTO."]
        #[inline(always)]
        pub fn set_jauto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "AWD1CH."]
        #[inline(always)]
        pub const fn awd1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "AWD1CH."]
        #[inline(always)]
        pub fn set_awd1ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
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
                .field("dmngt", &self.dmngt())
                .field("res", &self.res())
                .field("extsel", &self.extsel())
                .field("exten", &self.exten())
                .field("ovrmod", &self.ovrmod())
                .field("cont", &self.cont())
                .field("autdly", &self.autdly())
                .field("discen", &self.discen())
                .field("discnum", &self.discnum())
                .field("jdiscen", &self.jdiscen())
                .field("awd1sgl", &self.awd1sgl())
                .field("awd1en", &self.awd1en())
                .field("jawd1en", &self.jawd1en())
                .field("jauto", &self.jauto())
                .field("awd1ch", &self.awd1ch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ dmngt: {:?}, res: {:?}, extsel: {=u8:?}, exten: {:?}, ovrmod: {=bool:?}, cont: {=bool:?}, autdly: {=bool:?}, discen: {=bool:?}, discnum: {=u8:?}, jdiscen: {=bool:?}, awd1sgl: {=bool:?}, awd1en: {=bool:?}, jawd1en: {=bool:?}, jauto: {=bool:?}, awd1ch: {=u8:?} }}" , self . dmngt () , self . res () , self . extsel () , self . exten () , self . ovrmod () , self . cont () , self . autdly () , self . discen () , self . discnum () , self . jdiscen () , self . awd1sgl () , self . awd1en () , self . jawd1en () , self . jauto () , self . awd1ch ())
        }
    }
    #[doc = "ADC configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "ROVSE."]
        #[inline(always)]
        pub const fn rovse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ROVSE."]
        #[inline(always)]
        pub fn set_rovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "JOVSE."]
        #[inline(always)]
        pub const fn jovse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "JOVSE."]
        #[inline(always)]
        pub fn set_jovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "OVSS."]
        #[inline(always)]
        pub const fn ovss(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "OVSS."]
        #[inline(always)]
        pub fn set_ovss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "TROVS."]
        #[inline(always)]
        pub const fn trovs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TROVS."]
        #[inline(always)]
        pub fn set_trovs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ROVSM."]
        #[inline(always)]
        pub const fn rovsm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ROVSM."]
        #[inline(always)]
        pub fn set_rovsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "BULB."]
        #[inline(always)]
        pub const fn bulb(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "BULB."]
        #[inline(always)]
        pub fn set_bulb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SWTRIG."]
        #[inline(always)]
        pub const fn swtrig(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SWTRIG."]
        #[inline(always)]
        pub fn set_swtrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SMPTRIG."]
        #[inline(always)]
        pub const fn smptrig(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SMPTRIG."]
        #[inline(always)]
        pub fn set_smptrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "OVSR."]
        #[inline(always)]
        pub const fn ovsr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "OVSR."]
        #[inline(always)]
        pub fn set_ovsr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "LFTRIG."]
        #[inline(always)]
        pub const fn lftrig(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "LFTRIG."]
        #[inline(always)]
        pub fn set_lftrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "LSHIFT."]
        #[inline(always)]
        pub const fn lshift(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "LSHIFT."]
        #[inline(always)]
        pub fn set_lshift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
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
                .field("rovse", &self.rovse())
                .field("jovse", &self.jovse())
                .field("ovss", &self.ovss())
                .field("trovs", &self.trovs())
                .field("rovsm", &self.rovsm())
                .field("bulb", &self.bulb())
                .field("swtrig", &self.swtrig())
                .field("smptrig", &self.smptrig())
                .field("ovsr", &self.ovsr())
                .field("lftrig", &self.lftrig())
                .field("lshift", &self.lshift())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr2 {{ rovse: {=bool:?}, jovse: {=bool:?}, ovss: {=u8:?}, trovs: {=bool:?}, rovsm: {=bool:?}, bulb: {=bool:?}, swtrig: {=bool:?}, smptrig: {=bool:?}, ovsr: {=u16:?}, lftrig: {=bool:?}, lshift: {=u8:?} }}" , self . rovse () , self . jovse () , self . ovss () , self . trovs () , self . rovsm () , self . bulb () , self . swtrig () , self . smptrig () , self . ovsr () , self . lftrig () , self . lshift ())
        }
    }
    #[doc = "ADC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "ADEN."]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADEN."]
        #[inline(always)]
        pub fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADDIS."]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADDIS."]
        #[inline(always)]
        pub fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADSTART."]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADSTART."]
        #[inline(always)]
        pub fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "JADSTART."]
        #[inline(always)]
        pub const fn jadstart(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JADSTART."]
        #[inline(always)]
        pub fn set_jadstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADSTP."]
        #[inline(always)]
        pub const fn adstp(&self) -> super::vals::Adstp {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Adstp::from_bits(val as u8)
        }
        #[doc = "ADSTP."]
        #[inline(always)]
        pub fn set_adstp(&mut self, val: super::vals::Adstp) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "JADSTP."]
        #[inline(always)]
        pub const fn jadstp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JADSTP."]
        #[inline(always)]
        pub fn set_jadstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADCALLIN."]
        #[inline(always)]
        pub const fn adcallin(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADCALLIN."]
        #[inline(always)]
        pub fn set_adcallin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CALINDEX."]
        #[inline(always)]
        pub const fn calindex(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "CALINDEX."]
        #[inline(always)]
        pub fn set_calindex(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "ADVREGEN."]
        #[inline(always)]
        pub const fn advregen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADVREGEN."]
        #[inline(always)]
        pub fn set_advregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DEEPPWD."]
        #[inline(always)]
        pub const fn deeppwd(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DEEPPWD."]
        #[inline(always)]
        pub fn set_deeppwd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ADCAL."]
        #[inline(always)]
        pub const fn adcal(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADCAL."]
        #[inline(always)]
        pub fn set_adcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("aden", &self.aden())
                .field("addis", &self.addis())
                .field("adstart", &self.adstart())
                .field("jadstart", &self.jadstart())
                .field("adstp", &self.adstp())
                .field("jadstp", &self.jadstp())
                .field("adcallin", &self.adcallin())
                .field("calindex", &self.calindex())
                .field("advregen", &self.advregen())
                .field("deeppwd", &self.deeppwd())
                .field("adcal", &self.adcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ aden: {=bool:?}, addis: {=bool:?}, adstart: {=bool:?}, jadstart: {=bool:?}, adstp: {:?}, jadstp: {=bool:?}, adcallin: {=bool:?}, calindex: {=u8:?}, advregen: {=bool:?}, deeppwd: {=bool:?}, adcal: {=bool:?} }}" , self . aden () , self . addis () , self . adstart () , self . jadstart () , self . adstp () , self . jadstp () , self . adcallin () , self . calindex () , self . advregen () , self . deeppwd () , self . adcal ())
        }
    }
    #[doc = "ADC differential mode selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Difsel(pub u32);
    impl Difsel {
        #[doc = "channel differential or single-ended mode for channel"]
        #[inline(always)]
        pub const fn difsel(&self, n: usize) -> super::vals::Difsel {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Difsel::from_bits(val as u8)
        }
        #[doc = "channel differential or single-ended mode for channel"]
        #[inline(always)]
        pub fn set_difsel(&mut self, n: usize, val: super::vals::Difsel) {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Difsel {
        #[inline(always)]
        fn default() -> Difsel {
            Difsel(0)
        }
    }
    impl core::fmt::Debug for Difsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Difsel")
                .field("difsel[0]", &self.difsel(0usize))
                .field("difsel[1]", &self.difsel(1usize))
                .field("difsel[2]", &self.difsel(2usize))
                .field("difsel[3]", &self.difsel(3usize))
                .field("difsel[4]", &self.difsel(4usize))
                .field("difsel[5]", &self.difsel(5usize))
                .field("difsel[6]", &self.difsel(6usize))
                .field("difsel[7]", &self.difsel(7usize))
                .field("difsel[8]", &self.difsel(8usize))
                .field("difsel[9]", &self.difsel(9usize))
                .field("difsel[10]", &self.difsel(10usize))
                .field("difsel[11]", &self.difsel(11usize))
                .field("difsel[12]", &self.difsel(12usize))
                .field("difsel[13]", &self.difsel(13usize))
                .field("difsel[14]", &self.difsel(14usize))
                .field("difsel[15]", &self.difsel(15usize))
                .field("difsel[16]", &self.difsel(16usize))
                .field("difsel[17]", &self.difsel(17usize))
                .field("difsel[18]", &self.difsel(18usize))
                .field("difsel[19]", &self.difsel(19usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Difsel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Difsel {{ difsel[0]: {:?}, difsel[1]: {:?}, difsel[2]: {:?}, difsel[3]: {:?}, difsel[4]: {:?}, difsel[5]: {:?}, difsel[6]: {:?}, difsel[7]: {:?}, difsel[8]: {:?}, difsel[9]: {:?}, difsel[10]: {:?}, difsel[11]: {:?}, difsel[12]: {:?}, difsel[13]: {:?}, difsel[14]: {:?}, difsel[15]: {:?}, difsel[16]: {:?}, difsel[17]: {:?}, difsel[18]: {:?}, difsel[19]: {:?} }}" , self . difsel (0usize) , self . difsel (1usize) , self . difsel (2usize) , self . difsel (3usize) , self . difsel (4usize) , self . difsel (5usize) , self . difsel (6usize) , self . difsel (7usize) , self . difsel (8usize) , self . difsel (9usize) , self . difsel (10usize) , self . difsel (11usize) , self . difsel (12usize) , self . difsel (13usize) , self . difsel (14usize) , self . difsel (15usize) , self . difsel (16usize) , self . difsel (17usize) , self . difsel (18usize) , self . difsel (19usize))
        }
    }
    #[doc = "ADC regular Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "RDATA."]
        #[inline(always)]
        pub const fn rdata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RDATA."]
        #[inline(always)]
        pub fn set_rdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("rdata", &self.rdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ rdata: {=u32:?} }}", self.rdata())
        }
    }
    #[doc = "ADC gain compensation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcomp(pub u32);
    impl Gcomp {
        #[doc = "GCOMPCOEFF."]
        #[inline(always)]
        pub const fn gcompcoeff(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "GCOMPCOEFF."]
        #[inline(always)]
        pub fn set_gcompcoeff(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "GCOMP."]
        #[inline(always)]
        pub const fn gcomp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "GCOMP."]
        #[inline(always)]
        pub fn set_gcomp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gcomp {
        #[inline(always)]
        fn default() -> Gcomp {
            Gcomp(0)
        }
    }
    impl core::fmt::Debug for Gcomp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcomp")
                .field("gcompcoeff", &self.gcompcoeff())
                .field("gcomp", &self.gcomp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcomp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gcomp {{ gcompcoeff: {=u16:?}, gcomp: {=bool:?} }}",
                self.gcompcoeff(),
                self.gcomp()
            )
        }
    }
    #[doc = "ADC watchdog threshold register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr1(pub u32);
    impl Htr1 {
        #[doc = "HTR1."]
        #[inline(always)]
        pub const fn htr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "HTR1."]
        #[inline(always)]
        pub fn set_htr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
        #[doc = "AWDFILT1."]
        #[inline(always)]
        pub const fn awdfilt1(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "AWDFILT1."]
        #[inline(always)]
        pub fn set_awdfilt1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Htr1 {
        #[inline(always)]
        fn default() -> Htr1 {
            Htr1(0)
        }
    }
    impl core::fmt::Debug for Htr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Htr1")
                .field("htr1", &self.htr1())
                .field("awdfilt1", &self.awdfilt1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Htr1 {{ htr1: {=u32:?}, awdfilt1: {=u8:?} }}",
                self.htr1(),
                self.awdfilt1()
            )
        }
    }
    #[doc = "ADC watchdog higher threshold register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr2(pub u32);
    impl Htr2 {
        #[doc = "HTR2."]
        #[inline(always)]
        pub const fn htr2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "HTR2."]
        #[inline(always)]
        pub fn set_htr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Htr2 {
        #[inline(always)]
        fn default() -> Htr2 {
            Htr2(0)
        }
    }
    impl core::fmt::Debug for Htr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Htr2").field("htr2", &self.htr2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Htr2 {{ htr2: {=u32:?} }}", self.htr2())
        }
    }
    #[doc = "ADC watchdog higher threshold register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr3(pub u32);
    impl Htr3 {
        #[doc = "HTR3."]
        #[inline(always)]
        pub const fn htr3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "HTR3."]
        #[inline(always)]
        pub fn set_htr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Htr3 {
        #[inline(always)]
        fn default() -> Htr3 {
            Htr3(0)
        }
    }
    impl core::fmt::Debug for Htr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Htr3").field("htr3", &self.htr3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Htr3 {{ htr3: {=u32:?} }}", self.htr3())
        }
    }
    #[doc = "ADC interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "ADRDYIE."]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADRDYIE."]
        #[inline(always)]
        pub fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EOSMPIE."]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EOSMPIE."]
        #[inline(always)]
        pub fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EOCIE."]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EOCIE."]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOSIE."]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOSIE."]
        #[inline(always)]
        pub fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OVRIE."]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OVRIE."]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JEOCIE."]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JEOCIE."]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "JEOSIE."]
        #[inline(always)]
        pub const fn jeosie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "JEOSIE."]
        #[inline(always)]
        pub fn set_jeosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AWD1IE."]
        #[inline(always)]
        pub const fn awdie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWD1IE."]
        #[inline(always)]
        pub fn set_awdie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("adrdyie", &self.adrdyie())
                .field("eosmpie", &self.eosmpie())
                .field("eocie", &self.eocie())
                .field("eosie", &self.eosie())
                .field("ovrie", &self.ovrie())
                .field("jeocie", &self.jeocie())
                .field("jeosie", &self.jeosie())
                .field("awdie[0]", &self.awdie(0usize))
                .field("awdie[1]", &self.awdie(1usize))
                .field("awdie[2]", &self.awdie(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ adrdyie: {=bool:?}, eosmpie: {=bool:?}, eocie: {=bool:?}, eosie: {=bool:?}, ovrie: {=bool:?}, jeocie: {=bool:?}, jeosie: {=bool:?}, awdie[0]: {=bool:?}, awdie[1]: {=bool:?}, awdie[2]: {=bool:?} }}" , self . adrdyie () , self . eosmpie () , self . eocie () , self . eosie () , self . ovrie () , self . jeocie () , self . jeosie () , self . awdie (0usize) , self . awdie (1usize) , self . awdie (2usize))
        }
    }
    #[doc = "ADC interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "ADRDY."]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADRDY."]
        #[inline(always)]
        pub fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EOSMP."]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EOSMP."]
        #[inline(always)]
        pub fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EOC."]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EOC."]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOS."]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOS."]
        #[inline(always)]
        pub fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OVR."]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OVR."]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JEOC."]
        #[inline(always)]
        pub const fn jeoc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JEOC."]
        #[inline(always)]
        pub fn set_jeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "JEOS."]
        #[inline(always)]
        pub const fn jeos(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "JEOS."]
        #[inline(always)]
        pub fn set_jeos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AWD1."]
        #[inline(always)]
        pub const fn awd(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWD1."]
        #[inline(always)]
        pub fn set_awd(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LDORDY."]
        #[inline(always)]
        pub const fn ldordy(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LDORDY."]
        #[inline(always)]
        pub fn set_ldordy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("adrdy", &self.adrdy())
                .field("eosmp", &self.eosmp())
                .field("eoc", &self.eoc())
                .field("eos", &self.eos())
                .field("ovr", &self.ovr())
                .field("jeoc", &self.jeoc())
                .field("jeos", &self.jeos())
                .field("awd[0]", &self.awd(0usize))
                .field("awd[1]", &self.awd(1usize))
                .field("awd[2]", &self.awd(2usize))
                .field("ldordy", &self.ldordy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ adrdy: {=bool:?}, eosmp: {=bool:?}, eoc: {=bool:?}, eos: {=bool:?}, ovr: {=bool:?}, jeoc: {=bool:?}, jeos: {=bool:?}, awd[0]: {=bool:?}, awd[1]: {=bool:?}, awd[2]: {=bool:?}, ldordy: {=bool:?} }}" , self . adrdy () , self . eosmp () , self . eoc () , self . eos () , self . ovr () , self . jeoc () , self . jeos () , self . awd (0usize) , self . awd (1usize) , self . awd (2usize) , self . ldordy ())
        }
    }
    #[doc = "ADC injected data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr(pub u32);
    impl Jdr {
        #[doc = "JDATA."]
        #[inline(always)]
        pub const fn jdata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "JDATA."]
        #[inline(always)]
        pub fn set_jdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Jdr {
        #[inline(always)]
        fn default() -> Jdr {
            Jdr(0)
        }
    }
    impl core::fmt::Debug for Jdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jdr").field("jdata", &self.jdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jdr {{ jdata: {=u32:?} }}", self.jdata())
        }
    }
    #[doc = "ADC injected sequence register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jsqr(pub u32);
    impl Jsqr {
        #[doc = "JL."]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "JL."]
        #[inline(always)]
        pub fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "JEXTSEL."]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "JEXTSEL."]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "JEXTEN."]
        #[inline(always)]
        pub const fn jexten(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "JEXTEN."]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "JSQ1."]
        #[inline(always)]
        pub const fn jsq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 9usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "JSQ1."]
        #[inline(always)]
        pub fn set_jsq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 9usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Jsqr {
        #[inline(always)]
        fn default() -> Jsqr {
            Jsqr(0)
        }
    }
    impl core::fmt::Debug for Jsqr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jsqr")
                .field("jl", &self.jl())
                .field("jextsel", &self.jextsel())
                .field("jexten", &self.jexten())
                .field("jsq[0]", &self.jsq(0usize))
                .field("jsq[1]", &self.jsq(1usize))
                .field("jsq[2]", &self.jsq(2usize))
                .field("jsq[3]", &self.jsq(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jsqr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Jsqr {{ jl: {=u8:?}, jextsel: {=u8:?}, jexten: {=u8:?}, jsq[0]: {=u8:?}, jsq[1]: {=u8:?}, jsq[2]: {=u8:?}, jsq[3]: {=u8:?} }}" , self . jl () , self . jextsel () , self . jexten () , self . jsq (0usize) , self . jsq (1usize) , self . jsq (2usize) , self . jsq (3usize))
        }
    }
    #[doc = "ADC watchdog threshold register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr1(pub u32);
    impl Ltr1 {
        #[doc = "LTR1."]
        #[inline(always)]
        pub const fn ltr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "LTR1."]
        #[inline(always)]
        pub fn set_ltr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Ltr1 {
        #[inline(always)]
        fn default() -> Ltr1 {
            Ltr1(0)
        }
    }
    impl core::fmt::Debug for Ltr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ltr1").field("ltr1", &self.ltr1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ltr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ltr1 {{ ltr1: {=u32:?} }}", self.ltr1())
        }
    }
    #[doc = "ADC watchdog lower threshold register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr2(pub u32);
    impl Ltr2 {
        #[doc = "LTR2."]
        #[inline(always)]
        pub const fn ltr2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "LTR2."]
        #[inline(always)]
        pub fn set_ltr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Ltr2 {
        #[inline(always)]
        fn default() -> Ltr2 {
            Ltr2(0)
        }
    }
    impl core::fmt::Debug for Ltr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ltr2").field("ltr2", &self.ltr2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ltr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ltr2 {{ ltr2: {=u32:?} }}", self.ltr2())
        }
    }
    #[doc = "ADC watchdog lower threshold register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr3(pub u32);
    impl Ltr3 {
        #[doc = "LTR3."]
        #[inline(always)]
        pub const fn ltr3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "LTR3."]
        #[inline(always)]
        pub fn set_ltr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Ltr3 {
        #[inline(always)]
        fn default() -> Ltr3 {
            Ltr3(0)
        }
    }
    impl core::fmt::Debug for Ltr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ltr3").field("ltr3", &self.ltr3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ltr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ltr3 {{ ltr3: {=u32:?} }}", self.ltr3())
        }
    }
    #[doc = "ADC offset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ofr(pub u32);
    impl Ofr {
        #[doc = "OFFSET."]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "OFFSET."]
        #[inline(always)]
        pub fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "POSOFF."]
        #[inline(always)]
        pub const fn posoff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "POSOFF."]
        #[inline(always)]
        pub fn set_posoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USAT."]
        #[inline(always)]
        pub const fn usat(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USAT."]
        #[inline(always)]
        pub fn set_usat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SSAT."]
        #[inline(always)]
        pub const fn ssat(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SSAT."]
        #[inline(always)]
        pub fn set_ssat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "OFFSET_CH."]
        #[inline(always)]
        pub const fn offset_ch(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x1f;
            val as u8
        }
        #[doc = "OFFSET_CH."]
        #[inline(always)]
        pub fn set_offset_ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
        }
    }
    impl Default for Ofr {
        #[inline(always)]
        fn default() -> Ofr {
            Ofr(0)
        }
    }
    impl core::fmt::Debug for Ofr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ofr")
                .field("offset", &self.offset())
                .field("posoff", &self.posoff())
                .field("usat", &self.usat())
                .field("ssat", &self.ssat())
                .field("offset_ch", &self.offset_ch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ofr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ofr {{ offset: {=u32:?}, posoff: {=bool:?}, usat: {=bool:?}, ssat: {=bool:?}, offset_ch: {=u8:?} }}",
                self.offset(),
                self.posoff(),
                self.usat(),
                self.ssat(),
                self.offset_ch()
            )
        }
    }
    #[doc = "ADC channel preselection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcsel(pub u32);
    impl Pcsel {
        #[doc = "PCSEL."]
        #[inline(always)]
        pub const fn pcsel(&self, n: usize) -> super::vals::Pcsel {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pcsel::from_bits(val as u8)
        }
        #[doc = "PCSEL."]
        #[inline(always)]
        pub fn set_pcsel(&mut self, n: usize, val: super::vals::Pcsel) {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Pcsel {
        #[inline(always)]
        fn default() -> Pcsel {
            Pcsel(0)
        }
    }
    impl core::fmt::Debug for Pcsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcsel")
                .field("pcsel[0]", &self.pcsel(0usize))
                .field("pcsel[1]", &self.pcsel(1usize))
                .field("pcsel[2]", &self.pcsel(2usize))
                .field("pcsel[3]", &self.pcsel(3usize))
                .field("pcsel[4]", &self.pcsel(4usize))
                .field("pcsel[5]", &self.pcsel(5usize))
                .field("pcsel[6]", &self.pcsel(6usize))
                .field("pcsel[7]", &self.pcsel(7usize))
                .field("pcsel[8]", &self.pcsel(8usize))
                .field("pcsel[9]", &self.pcsel(9usize))
                .field("pcsel[10]", &self.pcsel(10usize))
                .field("pcsel[11]", &self.pcsel(11usize))
                .field("pcsel[12]", &self.pcsel(12usize))
                .field("pcsel[13]", &self.pcsel(13usize))
                .field("pcsel[14]", &self.pcsel(14usize))
                .field("pcsel[15]", &self.pcsel(15usize))
                .field("pcsel[16]", &self.pcsel(16usize))
                .field("pcsel[17]", &self.pcsel(17usize))
                .field("pcsel[18]", &self.pcsel(18usize))
                .field("pcsel[19]", &self.pcsel(19usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcsel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pcsel {{ pcsel[0]: {:?}, pcsel[1]: {:?}, pcsel[2]: {:?}, pcsel[3]: {:?}, pcsel[4]: {:?}, pcsel[5]: {:?}, pcsel[6]: {:?}, pcsel[7]: {:?}, pcsel[8]: {:?}, pcsel[9]: {:?}, pcsel[10]: {:?}, pcsel[11]: {:?}, pcsel[12]: {:?}, pcsel[13]: {:?}, pcsel[14]: {:?}, pcsel[15]: {:?}, pcsel[16]: {:?}, pcsel[17]: {:?}, pcsel[18]: {:?}, pcsel[19]: {:?} }}" , self . pcsel (0usize) , self . pcsel (1usize) , self . pcsel (2usize) , self . pcsel (3usize) , self . pcsel (4usize) , self . pcsel (5usize) , self . pcsel (6usize) , self . pcsel (7usize) , self . pcsel (8usize) , self . pcsel (9usize) , self . pcsel (10usize) , self . pcsel (11usize) , self . pcsel (12usize) , self . pcsel (13usize) , self . pcsel (14usize) , self . pcsel (15usize) , self . pcsel (16usize) , self . pcsel (17usize) , self . pcsel (18usize) , self . pcsel (19usize))
        }
    }
    #[doc = "ADC sample time register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr(pub u32);
    impl Smpr {
        #[doc = "SMP0."]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "SMP0."]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Smpr {
        #[inline(always)]
        fn default() -> Smpr {
            Smpr(0)
        }
    }
    impl core::fmt::Debug for Smpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smpr")
                .field("smp[0]", &self.smp(0usize))
                .field("smp[1]", &self.smp(1usize))
                .field("smp[2]", &self.smp(2usize))
                .field("smp[3]", &self.smp(3usize))
                .field("smp[4]", &self.smp(4usize))
                .field("smp[5]", &self.smp(5usize))
                .field("smp[6]", &self.smp(6usize))
                .field("smp[7]", &self.smp(7usize))
                .field("smp[8]", &self.smp(8usize))
                .field("smp[9]", &self.smp(9usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smpr {{ smp[0]: {:?}, smp[1]: {:?}, smp[2]: {:?}, smp[3]: {:?}, smp[4]: {:?}, smp[5]: {:?}, smp[6]: {:?}, smp[7]: {:?}, smp[8]: {:?}, smp[9]: {:?} }}" , self . smp (0usize) , self . smp (1usize) , self . smp (2usize) , self . smp (3usize) , self . smp (4usize) , self . smp (5usize) , self . smp (6usize) , self . smp (7usize) , self . smp (8usize) , self . smp (9usize))
        }
    }
    #[doc = "ADC regular sequence register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "L."]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "L."]
        #[inline(always)]
        pub fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SQ1."]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 6usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "SQ1."]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 6usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr1 {
        #[inline(always)]
        fn default() -> Sqr1 {
            Sqr1(0)
        }
    }
    impl core::fmt::Debug for Sqr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr1")
                .field("l", &self.l())
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr1 {{ l: {=u8:?}, sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?} }}",
                self.l(),
                self.sq(0usize),
                self.sq(1usize),
                self.sq(2usize),
                self.sq(3usize)
            )
        }
    }
    #[doc = "ADC regular sequence register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "SQ5."]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "SQ5."]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr2 {
        #[inline(always)]
        fn default() -> Sqr2 {
            Sqr2(0)
        }
    }
    impl core::fmt::Debug for Sqr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr2")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr2 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?} }}",
                self.sq(0usize),
                self.sq(1usize),
                self.sq(2usize),
                self.sq(3usize),
                self.sq(4usize)
            )
        }
    }
    #[doc = "ADC regular sequence register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr3(pub u32);
    impl Sqr3 {
        #[doc = "SQ10."]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "SQ10."]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr3 {
        #[inline(always)]
        fn default() -> Sqr3 {
            Sqr3(0)
        }
    }
    impl core::fmt::Debug for Sqr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr3")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr3 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?} }}",
                self.sq(0usize),
                self.sq(1usize),
                self.sq(2usize),
                self.sq(3usize),
                self.sq(4usize)
            )
        }
    }
    #[doc = "ADC regular sequence register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr4(pub u32);
    impl Sqr4 {
        #[doc = "SQ15."]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "SQ15."]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr4 {
        #[inline(always)]
        fn default() -> Sqr4 {
            Sqr4(0)
        }
    }
    impl core::fmt::Debug for Sqr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr4")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr4 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?} }}",
                self.sq(0usize),
                self.sq(1usize)
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc4Dmacfg {
        #[doc = "DMA One Shot mode selected"]
        ONE_SHOT = 0x0,
        #[doc = "DMA Circular mode selected"]
        CIRCULAR = 0x01,
    }
    impl Adc4Dmacfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc4Dmacfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc4Dmacfg {
        #[inline(always)]
        fn from(val: u8) -> Adc4Dmacfg {
            Adc4Dmacfg::from_bits(val)
        }
    }
    impl From<Adc4Dmacfg> for u8 {
        #[inline(always)]
        fn from(val: Adc4Dmacfg) -> u8 {
            Adc4Dmacfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc4Exten {
        #[doc = "Trigger detection disabled"]
        DISABLED = 0x0,
        #[doc = "Trigger detection on the rising edge"]
        RISING_EDGE = 0x01,
        #[doc = "Trigger detection on the falling edge"]
        FALLING_EDGE = 0x02,
        #[doc = "Trigger detection on both the rising and falling edges"]
        BOTH_EDGES = 0x03,
    }
    impl Adc4Exten {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc4Exten {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc4Exten {
        #[inline(always)]
        fn from(val: u8) -> Adc4Exten {
            Adc4Exten::from_bits(val)
        }
    }
    impl From<Adc4Exten> for u8 {
        #[inline(always)]
        fn from(val: Adc4Exten) -> u8 {
            Adc4Exten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc4OversamplingRatio {
        #[doc = "Oversample 2 times"]
        OVERSAMPLE2X = 0x0,
        #[doc = "Oversample 4 times"]
        OVERSAMPLE4X = 0x01,
        #[doc = "Oversample 8 times"]
        OVERSAMPLE8X = 0x02,
        #[doc = "Oversample 16 times"]
        OVERSAMPLE16X = 0x03,
        #[doc = "Oversample 32 times"]
        OVERSAMPLE32X = 0x04,
        #[doc = "Oversample 64 times"]
        OVERSAMPLE64X = 0x05,
        #[doc = "Oversample 128 times"]
        OVERSAMPLE128X = 0x06,
        #[doc = "Oversample 256 times"]
        OVERSAMPLE256X = 0x07,
    }
    impl Adc4OversamplingRatio {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc4OversamplingRatio {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc4OversamplingRatio {
        #[inline(always)]
        fn from(val: u8) -> Adc4OversamplingRatio {
            Adc4OversamplingRatio::from_bits(val)
        }
    }
    impl From<Adc4OversamplingRatio> for u8 {
        #[inline(always)]
        fn from(val: Adc4OversamplingRatio) -> u8 {
            Adc4OversamplingRatio::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc4Presc {
        #[doc = "adc_ker_ck_input not divided"]
        DIV1 = 0x0,
        #[doc = "adc_ker_ck_input divided by 2"]
        DIV2 = 0x01,
        #[doc = "adc_ker_ck_input divided by 4"]
        DIV4 = 0x02,
        #[doc = "adc_ker_ck_input divided by 6"]
        DIV6 = 0x03,
        #[doc = "adc_ker_ck_input divided by 8"]
        DIV8 = 0x04,
        #[doc = "adc_ker_ck_input divided by 10"]
        DIV10 = 0x05,
        #[doc = "adc_ker_ck_input divided by 12"]
        DIV12 = 0x06,
        #[doc = "adc_ker_ck_input divided by 16"]
        DIV16 = 0x07,
        #[doc = "adc_ker_ck_input divided by 32"]
        DIV32 = 0x08,
        #[doc = "adc_ker_ck_input divided by 64"]
        DIV64 = 0x09,
        #[doc = "adc_ker_ck_input divided by 128"]
        DIV128 = 0x0a,
        #[doc = "adc_ker_ck_input divided by 256"]
        DIV256 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Adc4Presc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc4Presc {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc4Presc {
        #[inline(always)]
        fn from(val: u8) -> Adc4Presc {
            Adc4Presc::from_bits(val)
        }
    }
    impl From<Adc4Presc> for u8 {
        #[inline(always)]
        fn from(val: Adc4Presc) -> u8 {
            Adc4Presc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc4Res {
        #[doc = "12-bit resolution"]
        BITS12 = 0x0,
        #[doc = "10-bit resolution"]
        BITS10 = 0x01,
        #[doc = "8-bit resolution"]
        BITS8 = 0x02,
        #[doc = "6-bit resolution"]
        BITS6 = 0x03,
    }
    impl Adc4Res {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc4Res {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc4Res {
        #[inline(always)]
        fn from(val: u8) -> Adc4Res {
            Adc4Res::from_bits(val)
        }
    }
    impl From<Adc4Res> for u8 {
        #[inline(always)]
        fn from(val: Adc4Res) -> u8 {
            Adc4Res::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adc4SampleTime {
        #[doc = "1.5 ADC cycles"]
        CYCLES1_5 = 0x0,
        #[doc = "3.5 ADC cycles"]
        CYCLES3_5 = 0x01,
        #[doc = "7.5 ADC cycles"]
        CYCLES7_5 = 0x02,
        #[doc = "12.5 ADC cycles"]
        CYCLES12_5 = 0x03,
        #[doc = "19.5 ADC cycles"]
        CYCLES19_5 = 0x04,
        #[doc = "39.5 ADC cycles"]
        CYCLES39_5 = 0x05,
        #[doc = "79.5 ADC cycles"]
        CYCLES79_5 = 0x06,
        #[doc = "160.5 ADC cycles"]
        CYCLES814_5 = 0x07,
    }
    impl Adc4SampleTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adc4SampleTime {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adc4SampleTime {
        #[inline(always)]
        fn from(val: u8) -> Adc4SampleTime {
            Adc4SampleTime::from_bits(val)
        }
    }
    impl From<Adc4SampleTime> for u8 {
        #[inline(always)]
        fn from(val: Adc4SampleTime) -> u8 {
            Adc4SampleTime::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adstp {
        _RESERVED_0 = 0x0,
        #[doc = "Stop conversion of channel"]
        STOP = 0x01,
    }
    impl Adstp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adstp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adstp {
        #[inline(always)]
        fn from(val: u8) -> Adstp {
            Adstp::from_bits(val)
        }
    }
    impl From<Adstp> for u8 {
        #[inline(always)]
        fn from(val: Adstp) -> u8 {
            Adstp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Difsel {
        #[doc = "Input channel is configured in single-ended mode"]
        SINGLE_ENDED = 0x0,
        #[doc = "Input channel is configured in differential mode"]
        DIFFERENTIAL = 0x01,
    }
    impl Difsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Difsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Difsel {
        #[inline(always)]
        fn from(val: u8) -> Difsel {
            Difsel::from_bits(val)
        }
    }
    impl From<Difsel> for u8 {
        #[inline(always)]
        fn from(val: Difsel) -> u8 {
            Difsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dmngt {
        #[doc = "Store output data in DR only"]
        DR = 0x0,
        #[doc = "DMA One Shot Mode selected"]
        DMA_ONE_SHOT = 0x01,
        #[doc = "MDF mode selected"]
        MDF = 0x02,
        #[doc = "DMA Circular Mode selected"]
        DMA_CIRCULAR = 0x03,
    }
    impl Dmngt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dmngt {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dmngt {
        #[inline(always)]
        fn from(val: u8) -> Dmngt {
            Dmngt::from_bits(val)
        }
    }
    impl From<Dmngt> for u8 {
        #[inline(always)]
        fn from(val: Dmngt) -> u8 {
            Dmngt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Exten {
        #[doc = "Trigger detection disabled"]
        DISABLED = 0x0,
        #[doc = "Trigger detection on the rising edge"]
        RISING_EDGE = 0x01,
        #[doc = "Trigger detection on the falling edge"]
        FALLING_EDGE = 0x02,
        #[doc = "Trigger detection on both the rising and falling edges"]
        BOTH_EDGES = 0x03,
    }
    impl Exten {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Exten {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Exten {
        #[inline(always)]
        fn from(val: u8) -> Exten {
            Exten::from_bits(val)
        }
    }
    impl From<Exten> for u8 {
        #[inline(always)]
        fn from(val: Exten) -> u8 {
            Exten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pcsel {
        #[doc = "Input channel x is not pre-selected"]
        NOT_PRESELECTED = 0x0,
        #[doc = "Pre-select input channel x"]
        PRESELECTED = 0x01,
    }
    impl Pcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pcsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pcsel {
        #[inline(always)]
        fn from(val: u8) -> Pcsel {
            Pcsel::from_bits(val)
        }
    }
    impl From<Pcsel> for u8 {
        #[inline(always)]
        fn from(val: Pcsel) -> u8 {
            Pcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Res {
        #[doc = "14-bit resolution"]
        BITS14 = 0x0,
        #[doc = "12-bit resolution"]
        BITS12 = 0x01,
        #[doc = "10-bit resolution"]
        BITS10 = 0x02,
        #[doc = "8-bit resolution"]
        BITS8 = 0x03,
    }
    impl Res {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Res {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Res {
        #[inline(always)]
        fn from(val: u8) -> Res {
            Res::from_bits(val)
        }
    }
    impl From<Res> for u8 {
        #[inline(always)]
        fn from(val: Res) -> u8 {
            Res::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SampleTime {
        #[doc = "1.5 ADC cycles"]
        CYCLES1_5 = 0x0,
        #[doc = "3.5 ADC cycles"]
        CYCLES3_5 = 0x01,
        #[doc = "7.5 ADC cycles"]
        CYCLES7_5 = 0x02,
        #[doc = "12.5 ADC cycles"]
        CYCLES12_5 = 0x03,
        #[doc = "19.5 ADC cycles"]
        CYCLES19_5 = 0x04,
        #[doc = "39.5 ADC cycles"]
        CYCLES39_5 = 0x05,
        #[doc = "79.5 ADC cycles"]
        CYCLES79_5 = 0x06,
        #[doc = "160.5 ADC cycles"]
        CYCLES160_5 = 0x07,
    }
    impl SampleTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SampleTime {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SampleTime {
        #[inline(always)]
        fn from(val: u8) -> SampleTime {
            SampleTime::from_bits(val)
        }
    }
    impl From<SampleTime> for u8 {
        #[inline(always)]
        fn from(val: SampleTime) -> u8 {
            SampleTime::to_bits(val)
        }
    }
}
