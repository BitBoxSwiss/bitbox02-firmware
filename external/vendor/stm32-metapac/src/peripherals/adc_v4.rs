#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog to Digital Converter"]
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
    #[doc = "interrupt and status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "configuration register 2"]
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
    #[doc = "pre channel selection register"]
    #[inline(always)]
    pub const fn pcsel(self) -> crate::common::Reg<regs::Pcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "analog watchdog 1 threshold register"]
    #[inline(always)]
    pub const fn ltr1(self) -> crate::common::Reg<regs::Ltr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "analog watchdog 2 threshold register"]
    #[inline(always)]
    pub const fn htr1(self) -> crate::common::Reg<regs::Htr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "group regular sequencer ranks register 1"]
    #[inline(always)]
    pub const fn sqr1(self) -> crate::common::Reg<regs::Sqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "group regular sequencer ranks register 2"]
    #[inline(always)]
    pub const fn sqr2(self) -> crate::common::Reg<regs::Sqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "group regular sequencer ranks register 3"]
    #[inline(always)]
    pub const fn sqr3(self) -> crate::common::Reg<regs::Sqr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "group regular sequencer ranks register 4"]
    #[inline(always)]
    pub const fn sqr4(self) -> crate::common::Reg<regs::Sqr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "group regular conversion data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "group injected sequencer register"]
    #[inline(always)]
    pub const fn jsqr(self) -> crate::common::Reg<regs::Jsqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "offset number 1-4 register"]
    #[inline(always)]
    pub const fn ofr(self, n: usize) -> crate::common::Reg<regs::Ofr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "group injected sequencer rank 1-4 register"]
    #[inline(always)]
    pub const fn jdr(self, n: usize) -> crate::common::Reg<regs::Jdr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn awd2cr(self) -> crate::common::Reg<regs::Awd2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn awd3cr(self) -> crate::common::Reg<regs::Awd3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "watchdog lower threshold register 2"]
    #[inline(always)]
    pub const fn ltr2(self) -> crate::common::Reg<regs::Ltr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "watchdog higher threshold register 2"]
    #[inline(always)]
    pub const fn htr2(self) -> crate::common::Reg<regs::Htr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "watchdog lower threshold register 3"]
    #[inline(always)]
    pub const fn ltr3(self) -> crate::common::Reg<regs::Ltr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "watchdog higher threshold register 3"]
    #[inline(always)]
    pub const fn htr3(self) -> crate::common::Reg<regs::Htr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "channel differential or single-ended mode selection register"]
    #[inline(always)]
    pub const fn difsel(self) -> crate::common::Reg<regs::Difsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "calibration factors register"]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Calibration Factor register 2"]
    #[inline(always)]
    pub const fn calfact2(self) -> crate::common::Reg<regs::Calfact2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
}
pub mod regs {
    #[doc = "analog watchdog 2 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2cr(pub u32);
    impl Awd2cr {
        #[doc = "analog watchdog 2 monitored channel selection"]
        #[inline(always)]
        pub const fn awd2ch(&self, n: usize) -> bool {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 2 monitored channel selection"]
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
    #[doc = "analog watchdog 3 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3cr(pub u32);
    impl Awd3cr {
        #[doc = "analog watchdog 3 monitored channel selection"]
        #[inline(always)]
        pub const fn awd3ch(&self, n: usize) -> bool {
            assert!(n < 20usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 3 monitored channel selection"]
        #[inline(always)]
        pub fn set_awd3ch(&mut self, n: usize, val: bool) {
            assert!(n < 20usize);
            let offs = 1usize + n * 1usize;
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
    #[doc = "calibration factors register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact(pub u32);
    impl Calfact {
        #[doc = "calibration factor in single-ended mode"]
        #[inline(always)]
        pub const fn calfact_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "calibration factor in single-ended mode"]
        #[inline(always)]
        pub fn set_calfact_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "calibration factor in differential mode"]
        #[inline(always)]
        pub const fn calfact_d(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "calibration factor in differential mode"]
        #[inline(always)]
        pub fn set_calfact_d(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
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
                .field("calfact_s", &self.calfact_s())
                .field("calfact_d", &self.calfact_d())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calfact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calfact {{ calfact_s: {=u16:?}, calfact_d: {=u16:?} }}",
                self.calfact_s(),
                self.calfact_d()
            )
        }
    }
    #[doc = "Calibration Factor register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact2(pub u32);
    impl Calfact2 {
        #[doc = "Linearity Calibration Factor"]
        #[inline(always)]
        pub const fn lincalfact(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Linearity Calibration Factor"]
        #[inline(always)]
        pub fn set_lincalfact(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
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
            f.debug_struct("Calfact2")
                .field("lincalfact", &self.lincalfact())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calfact2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Calfact2 {{ lincalfact: {=u32:?} }}", self.lincalfact())
        }
    }
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "DMA transfer enable"]
        #[inline(always)]
        pub const fn dmngt(&self) -> super::vals::Dmngt {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Dmngt::from_bits(val as u8)
        }
        #[doc = "DMA transfer enable"]
        #[inline(always)]
        pub fn set_dmngt(&mut self, val: super::vals::Dmngt) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "data resolution"]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "data resolution"]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
        }
        #[doc = "group regular external trigger source"]
        #[inline(always)]
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "group regular external trigger source"]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "group regular external trigger polarity"]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Exten {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "group regular external trigger polarity"]
        #[inline(always)]
        pub fn set_exten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "group regular overrun configuration"]
        #[inline(always)]
        pub const fn ovrmod(&self) -> super::vals::Ovrmod {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Ovrmod::from_bits(val as u8)
        }
        #[doc = "group regular overrun configuration"]
        #[inline(always)]
        pub fn set_ovrmod(&mut self, val: super::vals::Ovrmod) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Continuous conversion"]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous conversion"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "low power auto wait"]
        #[inline(always)]
        pub const fn autdly(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "low power auto wait"]
        #[inline(always)]
        pub fn set_autdly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "group regular sequencer discontinuous mode"]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "group regular sequencer discontinuous mode"]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "group regular sequencer discontinuous number of ranks"]
        #[inline(always)]
        pub const fn discnum(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "group regular sequencer discontinuous number of ranks"]
        #[inline(always)]
        pub fn set_discnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "group injected sequencer discontinuous mode"]
        #[inline(always)]
        pub const fn jdiscen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "group injected sequencer discontinuous mode"]
        #[inline(always)]
        pub fn set_jdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "group injected contexts queue mode"]
        #[inline(always)]
        pub const fn jqm(&self) -> super::vals::Jqm {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Jqm::from_bits(val as u8)
        }
        #[doc = "group injected contexts queue mode"]
        #[inline(always)]
        pub fn set_jqm(&mut self, val: super::vals::Jqm) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "analog watchdog 1 monitoring a single channel or all channels"]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> super::vals::Awd1sgl {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Awd1sgl::from_bits(val as u8)
        }
        #[doc = "analog watchdog 1 monitoring a single channel or all channels"]
        #[inline(always)]
        pub fn set_awd1sgl(&mut self, val: super::vals::Awd1sgl) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "analog watchdog 1 enable on scope group regular"]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 1 enable on scope group regular"]
        #[inline(always)]
        pub fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "analog watchdog 1 enable on scope group injected"]
        #[inline(always)]
        pub const fn jawd1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 1 enable on scope group injected"]
        #[inline(always)]
        pub fn set_jawd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "group injected automatic trigger mode"]
        #[inline(always)]
        pub const fn jauto(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "group injected automatic trigger mode"]
        #[inline(always)]
        pub fn set_jauto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "analog watchdog 1 monitored channel selection"]
        #[inline(always)]
        pub const fn awd1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "analog watchdog 1 monitored channel selection"]
        #[inline(always)]
        pub fn set_awd1ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
        #[doc = "group injected contexts queue disable"]
        #[inline(always)]
        pub const fn jqdis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "group injected contexts queue disable"]
        #[inline(always)]
        pub fn set_jqdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("jqm", &self.jqm())
                .field("awd1sgl", &self.awd1sgl())
                .field("awd1en", &self.awd1en())
                .field("jawd1en", &self.jawd1en())
                .field("jauto", &self.jauto())
                .field("awd1ch", &self.awd1ch())
                .field("jqdis", &self.jqdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ dmngt: {:?}, res: {:?}, extsel: {=u8:?}, exten: {:?}, ovrmod: {:?}, cont: {=bool:?}, autdly: {=bool:?}, discen: {=bool:?}, discnum: {=u8:?}, jdiscen: {=bool:?}, jqm: {:?}, awd1sgl: {:?}, awd1en: {=bool:?}, jawd1en: {=bool:?}, jauto: {=bool:?}, awd1ch: {=u8:?}, jqdis: {=bool:?} }}" , self . dmngt () , self . res () , self . extsel () , self . exten () , self . ovrmod () , self . cont () , self . autdly () , self . discen () , self . discnum () , self . jdiscen () , self . jqm () , self . awd1sgl () , self . awd1en () , self . jawd1en () , self . jauto () , self . awd1ch () , self . jqdis ())
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "oversampler enable on scope group regular"]
        #[inline(always)]
        pub const fn rovse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "oversampler enable on scope group regular"]
        #[inline(always)]
        pub fn set_rovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "oversampler enable on scope group injected"]
        #[inline(always)]
        pub const fn jovse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "oversampler enable on scope group injected"]
        #[inline(always)]
        pub fn set_jovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "oversampling shift"]
        #[inline(always)]
        pub const fn ovss(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "oversampling shift"]
        #[inline(always)]
        pub fn set_ovss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "oversampling discontinuous mode (triggered mode) for group regular"]
        #[inline(always)]
        pub const fn trovs(&self) -> super::vals::Trovs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Trovs::from_bits(val as u8)
        }
        #[doc = "oversampling discontinuous mode (triggered mode) for group regular"]
        #[inline(always)]
        pub fn set_trovs(&mut self, val: super::vals::Trovs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Regular Oversampling mode"]
        #[inline(always)]
        pub const fn rovsm(&self) -> super::vals::Rovsm {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Rovsm::from_bits(val as u8)
        }
        #[doc = "Regular Oversampling mode"]
        #[inline(always)]
        pub fn set_rovsm(&mut self, val: super::vals::Rovsm) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Right-shift data after Offset 1 correction"]
        #[inline(always)]
        pub const fn rshift1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Right-shift data after Offset 1 correction"]
        #[inline(always)]
        pub fn set_rshift1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Right-shift data after Offset 2 correction"]
        #[inline(always)]
        pub const fn rshift2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Right-shift data after Offset 2 correction"]
        #[inline(always)]
        pub fn set_rshift2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Right-shift data after Offset 3 correction"]
        #[inline(always)]
        pub const fn rshift3(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Right-shift data after Offset 3 correction"]
        #[inline(always)]
        pub fn set_rshift3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Right-shift data after Offset 4 correction"]
        #[inline(always)]
        pub const fn rshift4(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Right-shift data after Offset 4 correction"]
        #[inline(always)]
        pub fn set_rshift4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Oversampling ratio"]
        #[inline(always)]
        pub const fn ovsr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Oversampling ratio"]
        #[inline(always)]
        pub fn set_ovsr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Left shift factor"]
        #[inline(always)]
        pub const fn lshift(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Left shift factor"]
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
                .field("rshift1", &self.rshift1())
                .field("rshift2", &self.rshift2())
                .field("rshift3", &self.rshift3())
                .field("rshift4", &self.rshift4())
                .field("ovsr", &self.ovsr())
                .field("lshift", &self.lshift())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr2 {{ rovse: {=bool:?}, jovse: {=bool:?}, ovss: {=u8:?}, trovs: {:?}, rovsm: {:?}, rshift1: {=bool:?}, rshift2: {=bool:?}, rshift3: {=bool:?}, rshift4: {=bool:?}, ovsr: {=u16:?}, lshift: {=u8:?} }}" , self . rovse () , self . jovse () , self . ovss () , self . trovs () , self . rovsm () , self . rshift1 () , self . rshift2 () , self . rshift3 () , self . rshift4 () , self . ovsr () , self . lshift ())
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "enable"]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable"]
        #[inline(always)]
        pub fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "disable"]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "disable"]
        #[inline(always)]
        pub fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "group regular conversion start"]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "group regular conversion start"]
        #[inline(always)]
        pub fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "group injected conversion start"]
        #[inline(always)]
        pub const fn jadstart(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "group injected conversion start"]
        #[inline(always)]
        pub fn set_jadstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "group regular conversion stop"]
        #[inline(always)]
        pub const fn adstp(&self) -> super::vals::Adstp {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Adstp::from_bits(val as u8)
        }
        #[doc = "group regular conversion stop"]
        #[inline(always)]
        pub fn set_adstp(&mut self, val: super::vals::Adstp) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "group injected conversion stop"]
        #[inline(always)]
        pub const fn jadstp(&self) -> super::vals::Adstp {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Adstp::from_bits(val as u8)
        }
        #[doc = "group injected conversion stop"]
        #[inline(always)]
        pub fn set_jadstp(&mut self, val: super::vals::Adstp) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Boost mode control"]
        #[inline(always)]
        pub const fn boost(&self) -> super::vals::Boost {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Boost::from_bits(val as u8)
        }
        #[doc = "Boost mode control"]
        #[inline(always)]
        pub fn set_boost(&mut self, val: super::vals::Boost) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Linearity calibration"]
        #[inline(always)]
        pub const fn adcallin(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Linearity calibration"]
        #[inline(always)]
        pub fn set_adcallin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Linearity calibration ready Word 1"]
        #[inline(always)]
        pub const fn lincalrdyw1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Linearity calibration ready Word 1"]
        #[inline(always)]
        pub fn set_lincalrdyw1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Linearity calibration ready Word 2"]
        #[inline(always)]
        pub const fn lincalrdyw2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Linearity calibration ready Word 2"]
        #[inline(always)]
        pub fn set_lincalrdyw2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Linearity calibration ready Word 3"]
        #[inline(always)]
        pub const fn lincalrdyw3(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Linearity calibration ready Word 3"]
        #[inline(always)]
        pub fn set_lincalrdyw3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Linearity calibration ready Word 4"]
        #[inline(always)]
        pub const fn lincalrdyw4(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Linearity calibration ready Word 4"]
        #[inline(always)]
        pub fn set_lincalrdyw4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Linearity calibration ready Word 5"]
        #[inline(always)]
        pub const fn lincalrdyw5(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Linearity calibration ready Word 5"]
        #[inline(always)]
        pub fn set_lincalrdyw5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Linearity calibration ready Word 6"]
        #[inline(always)]
        pub const fn lincalrdyw6(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Linearity calibration ready Word 6"]
        #[inline(always)]
        pub fn set_lincalrdyw6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "voltage regulator enable"]
        #[inline(always)]
        pub const fn advregen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "voltage regulator enable"]
        #[inline(always)]
        pub fn set_advregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "deep power down enable"]
        #[inline(always)]
        pub const fn deeppwd(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "deep power down enable"]
        #[inline(always)]
        pub fn set_deeppwd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "differential mode for calibration"]
        #[inline(always)]
        pub const fn adcaldif(&self) -> super::vals::Adcaldif {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Adcaldif::from_bits(val as u8)
        }
        #[doc = "differential mode for calibration"]
        #[inline(always)]
        pub fn set_adcaldif(&mut self, val: super::vals::Adcaldif) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "calibration"]
        #[inline(always)]
        pub const fn adcal(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "calibration"]
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
                .field("boost", &self.boost())
                .field("adcallin", &self.adcallin())
                .field("lincalrdyw1", &self.lincalrdyw1())
                .field("lincalrdyw2", &self.lincalrdyw2())
                .field("lincalrdyw3", &self.lincalrdyw3())
                .field("lincalrdyw4", &self.lincalrdyw4())
                .field("lincalrdyw5", &self.lincalrdyw5())
                .field("lincalrdyw6", &self.lincalrdyw6())
                .field("advregen", &self.advregen())
                .field("deeppwd", &self.deeppwd())
                .field("adcaldif", &self.adcaldif())
                .field("adcal", &self.adcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ aden: {=bool:?}, addis: {=bool:?}, adstart: {=bool:?}, jadstart: {=bool:?}, adstp: {:?}, jadstp: {:?}, boost: {:?}, adcallin: {=bool:?}, lincalrdyw1: {=bool:?}, lincalrdyw2: {=bool:?}, lincalrdyw3: {=bool:?}, lincalrdyw4: {=bool:?}, lincalrdyw5: {=bool:?}, lincalrdyw6: {=bool:?}, advregen: {=bool:?}, deeppwd: {=bool:?}, adcaldif: {:?}, adcal: {=bool:?} }}" , self . aden () , self . addis () , self . adstart () , self . jadstart () , self . adstp () , self . jadstp () , self . boost () , self . adcallin () , self . lincalrdyw1 () , self . lincalrdyw2 () , self . lincalrdyw3 () , self . lincalrdyw4 () , self . lincalrdyw5 () , self . lincalrdyw6 () , self . advregen () , self . deeppwd () , self . adcaldif () , self . adcal ())
        }
    }
    #[doc = "channel differential or single-ended mode selection register"]
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
    #[doc = "group regular conversion data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "group regular conversion data"]
        #[inline(always)]
        pub const fn rdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "group regular conversion data"]
        #[inline(always)]
        pub fn set_rdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            defmt::write!(f, "Dr {{ rdata: {=u16:?} }}", self.rdata())
        }
    }
    #[doc = "analog watchdog 2 threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr1(pub u32);
    impl Htr1 {
        #[doc = "analog watchdog 2 threshold low"]
        #[inline(always)]
        pub const fn htr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "analog watchdog 2 threshold low"]
        #[inline(always)]
        pub fn set_htr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
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
            f.debug_struct("Htr1").field("htr1", &self.htr1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Htr1 {{ htr1: {=u32:?} }}", self.htr1())
        }
    }
    #[doc = "watchdog higher threshold register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr2(pub u32);
    impl Htr2 {
        #[doc = "Analog watchdog 2 higher threshold"]
        #[inline(always)]
        pub const fn htr2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog 2 higher threshold"]
        #[inline(always)]
        pub fn set_htr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
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
    #[doc = "watchdog higher threshold register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr3(pub u32);
    impl Htr3 {
        #[doc = "Analog watchdog 3 higher threshold"]
        #[inline(always)]
        pub const fn htr3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog 3 higher threshold"]
        #[inline(always)]
        pub fn set_htr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
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
    #[doc = "interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "ready interrupt"]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ready interrupt"]
        #[inline(always)]
        pub fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "group regular end of sampling interrupt"]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "group regular end of sampling interrupt"]
        #[inline(always)]
        pub fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "group regular end of unitary conversion interrupt"]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "group regular end of unitary conversion interrupt"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "group regular end of sequence conversions interrupt"]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "group regular end of sequence conversions interrupt"]
        #[inline(always)]
        pub fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "group regular overrun interrupt"]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "group regular overrun interrupt"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "group injected end of unitary conversion interrupt"]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "group injected end of unitary conversion interrupt"]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "group injected end of sequence conversions interrupt"]
        #[inline(always)]
        pub const fn jeosie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "group injected end of sequence conversions interrupt"]
        #[inline(always)]
        pub fn set_jeosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "analog watchdog 1 interrupt"]
        #[inline(always)]
        pub const fn awd1ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 1 interrupt"]
        #[inline(always)]
        pub fn set_awd1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "analog watchdog 2 interrupt"]
        #[inline(always)]
        pub const fn awd2ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 2 interrupt"]
        #[inline(always)]
        pub fn set_awd2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "analog watchdog 3 interrupt"]
        #[inline(always)]
        pub const fn awd3ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 3 interrupt"]
        #[inline(always)]
        pub fn set_awd3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "group injected contexts queue overflow interrupt"]
        #[inline(always)]
        pub const fn jqovfie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "group injected contexts queue overflow interrupt"]
        #[inline(always)]
        pub fn set_jqovfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("awd1ie", &self.awd1ie())
                .field("awd2ie", &self.awd2ie())
                .field("awd3ie", &self.awd3ie())
                .field("jqovfie", &self.jqovfie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ adrdyie: {=bool:?}, eosmpie: {=bool:?}, eocie: {=bool:?}, eosie: {=bool:?}, ovrie: {=bool:?}, jeocie: {=bool:?}, jeosie: {=bool:?}, awd1ie: {=bool:?}, awd2ie: {=bool:?}, awd3ie: {=bool:?}, jqovfie: {=bool:?} }}" , self . adrdyie () , self . eosmpie () , self . eocie () , self . eosie () , self . ovrie () , self . jeocie () , self . jeosie () , self . awd1ie () , self . awd2ie () , self . awd3ie () , self . jqovfie ())
        }
    }
    #[doc = "interrupt and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "ready flag"]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ready flag"]
        #[inline(always)]
        pub fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "group regular end of sampling flag"]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "group regular end of sampling flag"]
        #[inline(always)]
        pub fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "group regular end of unitary conversion flag"]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "group regular end of unitary conversion flag"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "group regular end of sequence conversions flag"]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "group regular end of sequence conversions flag"]
        #[inline(always)]
        pub fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "group regular overrun flag"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "group regular overrun flag"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "group injected end of unitary conversion flag"]
        #[inline(always)]
        pub const fn jeoc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "group injected end of unitary conversion flag"]
        #[inline(always)]
        pub fn set_jeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "group injected end of sequence conversions flag"]
        #[inline(always)]
        pub const fn jeos(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "group injected end of sequence conversions flag"]
        #[inline(always)]
        pub fn set_jeos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "analog watchdog 1 flag"]
        #[inline(always)]
        pub const fn awd1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 1 flag"]
        #[inline(always)]
        pub fn set_awd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "analog watchdog 2 flag"]
        #[inline(always)]
        pub const fn awd2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 2 flag"]
        #[inline(always)]
        pub fn set_awd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "analog watchdog 3 flag"]
        #[inline(always)]
        pub const fn awd3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog 3 flag"]
        #[inline(always)]
        pub fn set_awd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "group injected contexts queue overflow flag"]
        #[inline(always)]
        pub const fn jqovf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "group injected contexts queue overflow flag"]
        #[inline(always)]
        pub fn set_jqovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ADC LDO output voltage ready (not always available)"]
        #[inline(always)]
        pub const fn ldordy(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ADC LDO output voltage ready (not always available)"]
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
                .field("awd1", &self.awd1())
                .field("awd2", &self.awd2())
                .field("awd3", &self.awd3())
                .field("jqovf", &self.jqovf())
                .field("ldordy", &self.ldordy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ adrdy: {=bool:?}, eosmp: {=bool:?}, eoc: {=bool:?}, eos: {=bool:?}, ovr: {=bool:?}, jeoc: {=bool:?}, jeos: {=bool:?}, awd1: {=bool:?}, awd2: {=bool:?}, awd3: {=bool:?}, jqovf: {=bool:?}, ldordy: {=bool:?} }}" , self . adrdy () , self . eosmp () , self . eoc () , self . eos () , self . ovr () , self . jeoc () , self . jeos () , self . awd1 () , self . awd2 () , self . awd3 () , self . jqovf () , self . ldordy ())
        }
    }
    #[doc = "group injected sequencer rank 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr(pub u32);
    impl Jdr {
        #[doc = "group injected sequencer rank 1 conversion data"]
        #[inline(always)]
        pub const fn jdata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "group injected sequencer rank 1 conversion data"]
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
    #[doc = "group injected sequencer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jsqr(pub u32);
    impl Jsqr {
        #[doc = "group injected sequencer scan length"]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "group injected sequencer scan length"]
        #[inline(always)]
        pub fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "group injected external trigger source"]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "group injected external trigger source"]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "group injected external trigger polarity"]
        #[inline(always)]
        pub const fn jexten(&self) -> super::vals::Exten {
            let val = (self.0 >> 7usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "group injected external trigger polarity"]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
        }
        #[doc = "group injected sequencer rank 1-4"]
        #[inline(always)]
        pub const fn jsq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 9usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "group injected sequencer rank 1-4"]
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
            defmt :: write ! (f , "Jsqr {{ jl: {=u8:?}, jextsel: {=u8:?}, jexten: {:?}, jsq[0]: {=u8:?}, jsq[1]: {=u8:?}, jsq[2]: {=u8:?}, jsq[3]: {=u8:?} }}" , self . jl () , self . jextsel () , self . jexten () , self . jsq (0usize) , self . jsq (1usize) , self . jsq (2usize) , self . jsq (3usize))
        }
    }
    #[doc = "analog watchdog 1 threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr1(pub u32);
    impl Ltr1 {
        #[doc = "analog watchdog 1 threshold low"]
        #[inline(always)]
        pub const fn ltr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "analog watchdog 1 threshold low"]
        #[inline(always)]
        pub fn set_ltr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
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
    #[doc = "watchdog lower threshold register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr2(pub u32);
    impl Ltr2 {
        #[doc = "Analog watchdog 2 lower threshold"]
        #[inline(always)]
        pub const fn ltr2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog 2 lower threshold"]
        #[inline(always)]
        pub fn set_ltr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
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
    #[doc = "watchdog lower threshold register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr3(pub u32);
    impl Ltr3 {
        #[doc = "Analog watchdog 3 lower threshold"]
        #[inline(always)]
        pub const fn ltr3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog 3 lower threshold"]
        #[inline(always)]
        pub fn set_ltr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
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
    #[doc = "offset number x register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ofr(pub u32);
    impl Ofr {
        #[doc = "offset number x offset level"]
        #[inline(always)]
        pub const fn offset1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "offset number x offset level"]
        #[inline(always)]
        pub fn set_offset1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
        #[doc = "offset number x channel selection"]
        #[inline(always)]
        pub const fn offset1_ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "offset number x channel selection"]
        #[inline(always)]
        pub fn set_offset1_ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
        #[doc = "Signed saturation enable"]
        #[inline(always)]
        pub const fn ssate(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Signed saturation enable"]
        #[inline(always)]
        pub fn set_ssate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("offset1", &self.offset1())
                .field("offset1_ch", &self.offset1_ch())
                .field("ssate", &self.ssate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ofr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ofr {{ offset1: {=u32:?}, offset1_ch: {=u8:?}, ssate: {=bool:?} }}",
                self.offset1(),
                self.offset1_ch(),
                self.ssate()
            )
        }
    }
    #[doc = "channel preselection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcsel(pub u32);
    impl Pcsel {
        #[doc = "Channel x (VINP\\[i\\]) pre selection"]
        #[inline(always)]
        pub const fn pcsel(&self, n: usize) -> super::vals::Pcsel {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pcsel::from_bits(val as u8)
        }
        #[doc = "Channel x (VINP\\[i\\]) pre selection"]
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
    #[doc = "sampling time register n"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr(pub u32);
    impl Smpr {
        #[doc = "channel n * 10 + x sampling time"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel n * 10 + x sampling time"]
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
    #[doc = "group regular sequencer ranks register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "L3"]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "L3"]
        #[inline(always)]
        pub fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "group regular sequencer rank 1-4"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 6usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "group regular sequencer rank 1-4"]
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
    #[doc = "group regular sequencer ranks register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "group regular sequencer rank 5-9"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "group regular sequencer rank 5-9"]
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
    #[doc = "group regular sequencer ranks register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr3(pub u32);
    impl Sqr3 {
        #[doc = "group regular sequencer rank 10-14"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "group regular sequencer rank 10-14"]
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
    #[doc = "group regular sequencer ranks register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr4(pub u32);
    impl Sqr4 {
        #[doc = "group regular sequencer rank 15-16"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "group regular sequencer rank 15-16"]
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
    pub enum Adcaldif {
        #[doc = "Calibration for single-ended mode"]
        SINGLE_ENDED = 0x0,
        #[doc = "Calibration for differential mode"]
        DIFFERENTIAL = 0x01,
    }
    impl Adcaldif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcaldif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcaldif {
        #[inline(always)]
        fn from(val: u8) -> Adcaldif {
            Adcaldif::from_bits(val)
        }
    }
    impl From<Adcaldif> for u8 {
        #[inline(always)]
        fn from(val: Adcaldif) -> u8 {
            Adcaldif::to_bits(val)
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
    pub enum Awd1sgl {
        #[doc = "Analog watchdog 1 enabled on all channels"]
        ALL = 0x0,
        #[doc = "Analog watchdog 1 enabled on single channel selected in AWD1CH"]
        SINGLE = 0x01,
    }
    impl Awd1sgl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Awd1sgl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Awd1sgl {
        #[inline(always)]
        fn from(val: u8) -> Awd1sgl {
            Awd1sgl::from_bits(val)
        }
    }
    impl From<Awd1sgl> for u8 {
        #[inline(always)]
        fn from(val: Awd1sgl) -> u8 {
            Awd1sgl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Boost {
        #[doc = "Boost mode used when clock ≤ 6.25 MHz"]
        LT6_25 = 0x0,
        #[doc = "Boost mode used when 6.25 MHz < clock ≤ 12.5 MHz"]
        LT12_5 = 0x01,
        #[doc = "Boost mode used when 12.5 MHz < clock ≤ 25.0 MHz"]
        LT25 = 0x02,
        #[doc = "Boost mode used when 25.0 MHz < clock ≤ 50.0 MHz"]
        LT50 = 0x03,
    }
    impl Boost {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Boost {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Boost {
        #[inline(always)]
        fn from(val: u8) -> Boost {
            Boost::from_bits(val)
        }
    }
    impl From<Boost> for u8 {
        #[inline(always)]
        fn from(val: Boost) -> u8 {
            Boost::to_bits(val)
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
        #[doc = "DFSDM mode selected"]
        DFSDM = 0x02,
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
    pub enum Jqm {
        #[doc = "JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
        MODE0 = 0x0,
        #[doc = "JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
        MODE1 = 0x01,
    }
    impl Jqm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jqm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jqm {
        #[inline(always)]
        fn from(val: u8) -> Jqm {
            Jqm::from_bits(val)
        }
    }
    impl From<Jqm> for u8 {
        #[inline(always)]
        fn from(val: Jqm) -> u8 {
            Jqm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ovrmod {
        #[doc = "Preserve DR register when an overrun is detected"]
        PRESERVE = 0x0,
        #[doc = "Overwrite DR register when an overrun is detected"]
        OVERWRITE = 0x01,
    }
    impl Ovrmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ovrmod {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ovrmod {
        #[inline(always)]
        fn from(val: u8) -> Ovrmod {
            Ovrmod::from_bits(val)
        }
    }
    impl From<Ovrmod> for u8 {
        #[inline(always)]
        fn from(val: Ovrmod) -> u8 {
            Ovrmod::to_bits(val)
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
        #[doc = "16-bit resolution"]
        BITS16 = 0x0,
        #[doc = "14-bit resolution in legacy mode (not optimized power consumption)"]
        BITS14 = 0x01,
        #[doc = "12-bit resolution in legacy mode (not optimized power consumption)"]
        BITS12 = 0x02,
        #[doc = "10-bit resolution"]
        BITS10 = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "14-bit resolution"]
        BITS14V = 0x05,
        #[doc = "12-bit resolution"]
        BITS12V = 0x06,
        #[doc = "8-bit resolution"]
        BITS8 = 0x07,
    }
    impl Res {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Res {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Rovsm {
        #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
        CONTINUED = 0x0,
        #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
        RESUMED = 0x01,
    }
    impl Rovsm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rovsm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rovsm {
        #[inline(always)]
        fn from(val: u8) -> Rovsm {
            Rovsm::from_bits(val)
        }
    }
    impl From<Rovsm> for u8 {
        #[inline(always)]
        fn from(val: Rovsm) -> u8 {
            Rovsm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SampleTime {
        #[doc = "1.5 clock cycles"]
        CYCLES1_5 = 0x0,
        #[doc = "2.5 clock cycles"]
        CYCLES2_5 = 0x01,
        #[doc = "8.5 clock cycles"]
        CYCLES8_5 = 0x02,
        #[doc = "16.5 clock cycles"]
        CYCLES16_5 = 0x03,
        #[doc = "32.5 clock cycles"]
        CYCLES32_5 = 0x04,
        #[doc = "64.5 clock cycles"]
        CYCLES64_5 = 0x05,
        #[doc = "387.5 clock cycles"]
        CYCLES387_5 = 0x06,
        #[doc = "810.5 clock cycles"]
        CYCLES810_5 = 0x07,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trovs {
        #[doc = "All oversampled conversions for a channel are run following a trigger"]
        AUTOMATIC = 0x0,
        #[doc = "Each oversampled conversion for a channel needs a new trigger"]
        TRIGGERED = 0x01,
    }
    impl Trovs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trovs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trovs {
        #[inline(always)]
        fn from(val: u8) -> Trovs {
            Trovs::from_bits(val)
        }
    }
    impl From<Trovs> for u8 {
        #[inline(always)]
        fn from(val: Trovs) -> u8 {
            Trovs::to_bits(val)
        }
    }
}
