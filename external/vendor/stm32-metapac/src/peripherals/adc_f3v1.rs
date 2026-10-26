#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog-to-Digital Converter"]
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
    #[doc = "configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "sample time register 1"]
    #[inline(always)]
    pub const fn smpr1(self) -> crate::common::Reg<regs::Smpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "sample time register 2"]
    #[inline(always)]
    pub const fn smpr2(self) -> crate::common::Reg<regs::Smpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "watchdog threshold register 1"]
    #[inline(always)]
    pub const fn tr1(self) -> crate::common::Reg<regs::Tr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "watchdog threshold register"]
    #[inline(always)]
    pub const fn tr2(self) -> crate::common::Reg<regs::Tr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "watchdog threshold register 3"]
    #[inline(always)]
    pub const fn tr3(self) -> crate::common::Reg<regs::Tr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "regular sequence register 1"]
    #[inline(always)]
    pub const fn sqr1(self) -> crate::common::Reg<regs::Sqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "regular sequence register 2"]
    #[inline(always)]
    pub const fn sqr2(self) -> crate::common::Reg<regs::Sqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "regular sequence register 3"]
    #[inline(always)]
    pub const fn sqr3(self) -> crate::common::Reg<regs::Sqr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "regular sequence register 4"]
    #[inline(always)]
    pub const fn sqr4(self) -> crate::common::Reg<regs::Sqr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "regular Data Register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "injected sequence register"]
    #[inline(always)]
    pub const fn jsqr(self) -> crate::common::Reg<regs::Jsqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "offset register X"]
    #[inline(always)]
    pub const fn ofr(self, n: usize) -> crate::common::Reg<regs::Ofr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "injected data register X"]
    #[inline(always)]
    pub const fn jdr(self, n: usize) -> crate::common::Reg<regs::Jdr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Analog Watchdog X Configuration Register"]
    #[inline(always)]
    pub const fn awdcr(self, n: usize) -> crate::common::Reg<regs::Awdcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "Differential Mode Selection Register 2"]
    #[inline(always)]
    pub const fn difsel(self) -> crate::common::Reg<regs::Difsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Calibration Factors"]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Analog Watchdog 2 Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awdcr(pub u32);
    impl Awdcr {
        #[doc = "AWD2CH"]
        #[inline(always)]
        pub const fn awd2ch0(&self, n: usize) -> bool {
            assert!(n < 17usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWD2CH"]
        #[inline(always)]
        pub fn set_awd2ch0(&mut self, n: usize, val: bool) {
            assert!(n < 17usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Awdcr {
        #[inline(always)]
        fn default() -> Awdcr {
            Awdcr(0)
        }
    }
    impl core::fmt::Debug for Awdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awdcr")
                .field("awd2ch0[0]", &self.awd2ch0(0usize))
                .field("awd2ch0[1]", &self.awd2ch0(1usize))
                .field("awd2ch0[2]", &self.awd2ch0(2usize))
                .field("awd2ch0[3]", &self.awd2ch0(3usize))
                .field("awd2ch0[4]", &self.awd2ch0(4usize))
                .field("awd2ch0[5]", &self.awd2ch0(5usize))
                .field("awd2ch0[6]", &self.awd2ch0(6usize))
                .field("awd2ch0[7]", &self.awd2ch0(7usize))
                .field("awd2ch0[8]", &self.awd2ch0(8usize))
                .field("awd2ch0[9]", &self.awd2ch0(9usize))
                .field("awd2ch0[10]", &self.awd2ch0(10usize))
                .field("awd2ch0[11]", &self.awd2ch0(11usize))
                .field("awd2ch0[12]", &self.awd2ch0(12usize))
                .field("awd2ch0[13]", &self.awd2ch0(13usize))
                .field("awd2ch0[14]", &self.awd2ch0(14usize))
                .field("awd2ch0[15]", &self.awd2ch0(15usize))
                .field("awd2ch0[16]", &self.awd2ch0(16usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Awdcr {{ awd2ch0[0]: {=bool:?}, awd2ch0[1]: {=bool:?}, awd2ch0[2]: {=bool:?}, awd2ch0[3]: {=bool:?}, awd2ch0[4]: {=bool:?}, awd2ch0[5]: {=bool:?}, awd2ch0[6]: {=bool:?}, awd2ch0[7]: {=bool:?}, awd2ch0[8]: {=bool:?}, awd2ch0[9]: {=bool:?}, awd2ch0[10]: {=bool:?}, awd2ch0[11]: {=bool:?}, awd2ch0[12]: {=bool:?}, awd2ch0[13]: {=bool:?}, awd2ch0[14]: {=bool:?}, awd2ch0[15]: {=bool:?}, awd2ch0[16]: {=bool:?} }}" , self . awd2ch0 (0usize) , self . awd2ch0 (1usize) , self . awd2ch0 (2usize) , self . awd2ch0 (3usize) , self . awd2ch0 (4usize) , self . awd2ch0 (5usize) , self . awd2ch0 (6usize) , self . awd2ch0 (7usize) , self . awd2ch0 (8usize) , self . awd2ch0 (9usize) , self . awd2ch0 (10usize) , self . awd2ch0 (11usize) , self . awd2ch0 (12usize) , self . awd2ch0 (13usize) , self . awd2ch0 (14usize) , self . awd2ch0 (15usize) , self . awd2ch0 (16usize))
        }
    }
    #[doc = "Calibration Factors"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact(pub u32);
    impl Calfact {
        #[doc = "CALFACT_S"]
        #[inline(always)]
        pub const fn calfact_s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "CALFACT_S"]
        #[inline(always)]
        pub fn set_calfact_s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "CALFACT_D"]
        #[inline(always)]
        pub const fn calfact_d(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "CALFACT_D"]
        #[inline(always)]
        pub fn set_calfact_d(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
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
                "Calfact {{ calfact_s: {=u8:?}, calfact_d: {=u8:?} }}",
                self.calfact_s(),
                self.calfact_d()
            )
        }
    }
    #[doc = "configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Direct memory access enable"]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Direct memory access enable"]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Direct memory access configuration"]
        #[inline(always)]
        pub const fn dmacfg(&self) -> super::vals::Dmacfg {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Dmacfg::from_bits(val as u8)
        }
        #[doc = "Direct memory access configuration"]
        #[inline(always)]
        pub fn set_dmacfg(&mut self, val: super::vals::Dmacfg) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Data resolution"]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "Data resolution"]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Data alignment"]
        #[inline(always)]
        pub const fn align(&self) -> super::vals::Align {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Align::from_bits(val as u8)
        }
        #[doc = "Data alignment"]
        #[inline(always)]
        pub fn set_align(&mut self, val: super::vals::Align) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "External trigger selection for regular group"]
        #[inline(always)]
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "External trigger selection for regular group"]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "External trigger enable and polarity selection for regular channels"]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Exten {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "External trigger enable and polarity selection for regular channels"]
        #[inline(always)]
        pub fn set_exten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Overrun Mode"]
        #[inline(always)]
        pub const fn ovrmod(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun Mode"]
        #[inline(always)]
        pub fn set_ovrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
        #[doc = "Delayed conversion mode"]
        #[inline(always)]
        pub const fn autdly(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed conversion mode"]
        #[inline(always)]
        pub fn set_autdly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Discontinuous mode for regular channels"]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode for regular channels"]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Discontinuous mode channel count"]
        #[inline(always)]
        pub const fn discnum(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Discontinuous mode channel count"]
        #[inline(always)]
        pub fn set_discnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Discontinuous mode on injected channels"]
        #[inline(always)]
        pub const fn jdiscen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode on injected channels"]
        #[inline(always)]
        pub fn set_jdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "JSQR queue mode"]
        #[inline(always)]
        pub const fn jqm(&self) -> super::vals::Jqm {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Jqm::from_bits(val as u8)
        }
        #[doc = "JSQR queue mode"]
        #[inline(always)]
        pub fn set_jqm(&mut self, val: super::vals::Jqm) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Enable the watchdog 1 on a single channel or on all channels"]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> super::vals::Awd1sgl {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Awd1sgl::from_bits(val as u8)
        }
        #[doc = "Enable the watchdog 1 on a single channel or on all channels"]
        #[inline(always)]
        pub fn set_awd1sgl(&mut self, val: super::vals::Awd1sgl) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog 1 enable on regular channels"]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 enable on regular channels"]
        #[inline(always)]
        pub fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Analog watchdog 1 enable on injected channels"]
        #[inline(always)]
        pub const fn jawd1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 enable on injected channels"]
        #[inline(always)]
        pub fn set_jawd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Automatic injected group conversion"]
        #[inline(always)]
        pub const fn jauto(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic injected group conversion"]
        #[inline(always)]
        pub fn set_jauto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Analog watchdog 1 channel selection"]
        #[inline(always)]
        pub const fn awd1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "Analog watchdog 1 channel selection"]
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
                .field("dmaen", &self.dmaen())
                .field("dmacfg", &self.dmacfg())
                .field("res", &self.res())
                .field("align", &self.align())
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ dmaen: {=bool:?}, dmacfg: {:?}, res: {:?}, align: {:?}, extsel: {=u8:?}, exten: {:?}, ovrmod: {=bool:?}, cont: {=bool:?}, autdly: {=bool:?}, discen: {=bool:?}, discnum: {=u8:?}, jdiscen: {=bool:?}, jqm: {:?}, awd1sgl: {:?}, awd1en: {=bool:?}, jawd1en: {=bool:?}, jauto: {=bool:?}, awd1ch: {=u8:?} }}" , self . dmaen () , self . dmacfg () , self . res () , self . align () , self . extsel () , self . exten () , self . ovrmod () , self . cont () , self . autdly () , self . discen () , self . discnum () , self . jdiscen () , self . jqm () , self . awd1sgl () , self . awd1en () , self . jawd1en () , self . jauto () , self . awd1ch ())
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "ADC enable control"]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC enable control"]
        #[inline(always)]
        pub fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC disable command"]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC disable command"]
        #[inline(always)]
        pub fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC start of regular conversion"]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC start of regular conversion"]
        #[inline(always)]
        pub fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC start of injected conversion"]
        #[inline(always)]
        pub const fn jadstart(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADC start of injected conversion"]
        #[inline(always)]
        pub fn set_jadstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC stop of regular conversion command"]
        #[inline(always)]
        pub const fn adstp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC stop of regular conversion command"]
        #[inline(always)]
        pub fn set_adstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC stop of injected conversion command"]
        #[inline(always)]
        pub const fn jadstp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC stop of injected conversion command"]
        #[inline(always)]
        pub fn set_jadstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC voltage regulator enable"]
        #[inline(always)]
        pub const fn advregen(&self) -> super::vals::Advregen {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Advregen::from_bits(val as u8)
        }
        #[doc = "ADC voltage regulator enable"]
        #[inline(always)]
        pub fn set_advregen(&mut self, val: super::vals::Advregen) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Differential mode for calibration"]
        #[inline(always)]
        pub const fn adcaldif(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Differential mode for calibration"]
        #[inline(always)]
        pub fn set_adcaldif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ADC calibration"]
        #[inline(always)]
        pub const fn adcal(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADC calibration"]
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
                .field("advregen", &self.advregen())
                .field("adcaldif", &self.adcaldif())
                .field("adcal", &self.adcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ aden: {=bool:?}, addis: {=bool:?}, adstart: {=bool:?}, jadstart: {=bool:?}, adstp: {=bool:?}, jadstp: {=bool:?}, advregen: {:?}, adcaldif: {=bool:?}, adcal: {=bool:?} }}" , self . aden () , self . addis () , self . adstart () , self . jadstart () , self . adstp () , self . jadstp () , self . advregen () , self . adcaldif () , self . adcal ())
        }
    }
    #[doc = "Differential Mode Selection Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Difsel(pub u32);
    impl Difsel {
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_10(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_10(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_11(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_11(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_12(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_12(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_13(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_13(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_14(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_14(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_15(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_15(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_16(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_16(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_17(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_17(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_18(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_18(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_19(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_19(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_110(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_110(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_111(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_111(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_112(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_112(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_113(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_113(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_114(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_114(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_115(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_115(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_116(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_116(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_117(&self) -> super::vals::Difsel10 {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Difsel10::from_bits(val as u8)
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_117(&mut self, val: super::vals::Difsel10) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
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
                .field("difsel_10", &self.difsel_10())
                .field("difsel_11", &self.difsel_11())
                .field("difsel_12", &self.difsel_12())
                .field("difsel_13", &self.difsel_13())
                .field("difsel_14", &self.difsel_14())
                .field("difsel_15", &self.difsel_15())
                .field("difsel_16", &self.difsel_16())
                .field("difsel_17", &self.difsel_17())
                .field("difsel_18", &self.difsel_18())
                .field("difsel_19", &self.difsel_19())
                .field("difsel_110", &self.difsel_110())
                .field("difsel_111", &self.difsel_111())
                .field("difsel_112", &self.difsel_112())
                .field("difsel_113", &self.difsel_113())
                .field("difsel_114", &self.difsel_114())
                .field("difsel_115", &self.difsel_115())
                .field("difsel_116", &self.difsel_116())
                .field("difsel_117", &self.difsel_117())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Difsel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Difsel {{ difsel_10: {:?}, difsel_11: {:?}, difsel_12: {:?}, difsel_13: {:?}, difsel_14: {:?}, difsel_15: {:?}, difsel_16: {:?}, difsel_17: {:?}, difsel_18: {:?}, difsel_19: {:?}, difsel_110: {:?}, difsel_111: {:?}, difsel_112: {:?}, difsel_113: {:?}, difsel_114: {:?}, difsel_115: {:?}, difsel_116: {:?}, difsel_117: {:?} }}" , self . difsel_10 () , self . difsel_11 () , self . difsel_12 () , self . difsel_13 () , self . difsel_14 () , self . difsel_15 () , self . difsel_16 () , self . difsel_17 () , self . difsel_18 () , self . difsel_19 () , self . difsel_110 () , self . difsel_111 () , self . difsel_112 () , self . difsel_113 () , self . difsel_114 () , self . difsel_115 () , self . difsel_116 () , self . difsel_117 ())
        }
    }
    #[doc = "regular Data Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Regular data"]
        #[inline(always)]
        pub const fn rdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data"]
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
    #[doc = "interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "ADC ready interrupt enable"]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready interrupt enable"]
        #[inline(always)]
        pub fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of sampling flag interrupt enable for regular conversions"]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of sampling flag interrupt enable for regular conversions"]
        #[inline(always)]
        pub fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of regular conversion interrupt enable"]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion interrupt enable"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End of regular sequence of conversions interrupt enable"]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence of conversions interrupt enable"]
        #[inline(always)]
        pub fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of injected conversion interrupt enable"]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion interrupt enable"]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "End of injected sequence of conversions interrupt enable"]
        #[inline(always)]
        pub const fn jeosie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected sequence of conversions interrupt enable"]
        #[inline(always)]
        pub fn set_jeosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog watchdog X interrupt enable"]
        #[inline(always)]
        pub const fn awdie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog X interrupt enable"]
        #[inline(always)]
        pub fn set_awdie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected context queue overflow interrupt enable"]
        #[inline(always)]
        pub const fn jqovfie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Injected context queue overflow interrupt enable"]
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
                .field("awdie[0]", &self.awdie(0usize))
                .field("awdie[1]", &self.awdie(1usize))
                .field("awdie[2]", &self.awdie(2usize))
                .field("jqovfie", &self.jqovfie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ adrdyie: {=bool:?}, eosmpie: {=bool:?}, eocie: {=bool:?}, eosie: {=bool:?}, ovrie: {=bool:?}, jeocie: {=bool:?}, jeosie: {=bool:?}, awdie[0]: {=bool:?}, awdie[1]: {=bool:?}, awdie[2]: {=bool:?}, jqovfie: {=bool:?} }}" , self . adrdyie () , self . eosmpie () , self . eocie () , self . eosie () , self . ovrie () , self . jeocie () , self . jeosie () , self . awdie (0usize) , self . awdie (1usize) , self . awdie (2usize) , self . jqovfie ())
        }
    }
    #[doc = "interrupt and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "ADC Ready"]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC Ready"]
        #[inline(always)]
        pub fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of sampling flag"]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of sampling flag"]
        #[inline(always)]
        pub fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of conversion flag"]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of conversion flag"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End of regular sequence flag"]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence flag"]
        #[inline(always)]
        pub fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC overrun"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC overrun"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Injected channel end of conversion flag"]
        #[inline(always)]
        pub const fn jeoc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel end of conversion flag"]
        #[inline(always)]
        pub fn set_jeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Injected channel end of sequence flag"]
        #[inline(always)]
        pub const fn jeos(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel end of sequence flag"]
        #[inline(always)]
        pub fn set_jeos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog watchdog flag"]
        #[inline(always)]
        pub const fn awd(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog flag"]
        #[inline(always)]
        pub fn set_awd(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected context queue overflow"]
        #[inline(always)]
        pub const fn jqovf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Injected context queue overflow"]
        #[inline(always)]
        pub fn set_jqovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("jqovf", &self.jqovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ adrdy: {=bool:?}, eosmp: {=bool:?}, eoc: {=bool:?}, eos: {=bool:?}, ovr: {=bool:?}, jeoc: {=bool:?}, jeos: {=bool:?}, awd[0]: {=bool:?}, awd[1]: {=bool:?}, awd[2]: {=bool:?}, jqovf: {=bool:?} }}" , self . adrdy () , self . eosmp () , self . eoc () , self . eos () , self . ovr () , self . jeoc () , self . jeos () , self . awd (0usize) , self . awd (1usize) , self . awd (2usize) , self . jqovf ())
        }
    }
    #[doc = "injected data register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr(pub u32);
    impl Jdr {
        #[doc = "Injected data"]
        #[inline(always)]
        pub const fn jdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected data"]
        #[inline(always)]
        pub fn set_jdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            defmt::write!(f, "Jdr {{ jdata: {=u16:?} }}", self.jdata())
        }
    }
    #[doc = "injected sequence register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jsqr(pub u32);
    impl Jsqr {
        #[doc = "Injected channel sequence length"]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Injected channel sequence length"]
        #[inline(always)]
        pub fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "External Trigger Selection for injected group"]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "External Trigger Selection for injected group"]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "External Trigger Enable and Polarity Selection for injected channels"]
        #[inline(always)]
        pub const fn jexten(&self) -> super::vals::Exten {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "External Trigger Enable and Polarity Selection for injected channels"]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "X conversion in the injected sequence"]
        #[inline(always)]
        pub const fn jsq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 8usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "X conversion in the injected sequence"]
        #[inline(always)]
        pub fn set_jsq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 8usize + n * 6usize;
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
    #[doc = "offset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ofr(pub u32);
    impl Ofr {
        #[doc = "Data offset y for the channel programmed into bits OFFSETy_CH"]
        #[inline(always)]
        pub const fn offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data offset y for the channel programmed into bits OFFSETy_CH"]
        #[inline(always)]
        pub fn set_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Data offset y for the channel programmed into bits OFFSETy_CH"]
        #[inline(always)]
        pub const fn ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "Data offset y for the channel programmed into bits OFFSETy_CH"]
        #[inline(always)]
        pub fn set_ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
        #[doc = "Offset y Enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Offset y Enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
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
                .field("offset", &self.offset())
                .field("ch", &self.ch())
                .field("en", &self.en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ofr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ofr {{ offset: {=u16:?}, ch: {=u8:?}, en: {=bool:?} }}",
                self.offset(),
                self.ch(),
                self.en()
            )
        }
    }
    #[doc = "sample time register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr1(pub u32);
    impl Smpr1 {
        #[doc = "Channel x sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 9usize);
            let offs = 3usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Channel x sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 9usize);
            let offs = 3usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Smpr1 {
        #[inline(always)]
        fn default() -> Smpr1 {
            Smpr1(0)
        }
    }
    impl core::fmt::Debug for Smpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smpr1")
                .field("smp[0]", &self.smp(0usize))
                .field("smp[1]", &self.smp(1usize))
                .field("smp[2]", &self.smp(2usize))
                .field("smp[3]", &self.smp(3usize))
                .field("smp[4]", &self.smp(4usize))
                .field("smp[5]", &self.smp(5usize))
                .field("smp[6]", &self.smp(6usize))
                .field("smp[7]", &self.smp(7usize))
                .field("smp[8]", &self.smp(8usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smpr1 {{ smp[0]: {:?}, smp[1]: {:?}, smp[2]: {:?}, smp[3]: {:?}, smp[4]: {:?}, smp[5]: {:?}, smp[6]: {:?}, smp[7]: {:?}, smp[8]: {:?} }}" , self . smp (0usize) , self . smp (1usize) , self . smp (2usize) , self . smp (3usize) , self . smp (4usize) , self . smp (5usize) , self . smp (6usize) , self . smp (7usize) , self . smp (8usize))
        }
    }
    #[doc = "sample time register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr2(pub u32);
    impl Smpr2 {
        #[doc = "Channel x sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 9usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Channel x sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 9usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Smpr2 {
        #[inline(always)]
        fn default() -> Smpr2 {
            Smpr2(0)
        }
    }
    impl core::fmt::Debug for Smpr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smpr2")
                .field("smp[0]", &self.smp(0usize))
                .field("smp[1]", &self.smp(1usize))
                .field("smp[2]", &self.smp(2usize))
                .field("smp[3]", &self.smp(3usize))
                .field("smp[4]", &self.smp(4usize))
                .field("smp[5]", &self.smp(5usize))
                .field("smp[6]", &self.smp(6usize))
                .field("smp[7]", &self.smp(7usize))
                .field("smp[8]", &self.smp(8usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smpr2 {{ smp[0]: {:?}, smp[1]: {:?}, smp[2]: {:?}, smp[3]: {:?}, smp[4]: {:?}, smp[5]: {:?}, smp[6]: {:?}, smp[7]: {:?}, smp[8]: {:?} }}" , self . smp (0usize) , self . smp (1usize) , self . smp (2usize) , self . smp (3usize) , self . smp (4usize) , self . smp (5usize) , self . smp (6usize) , self . smp (7usize) , self . smp (8usize))
        }
    }
    #[doc = "regular sequence register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "X conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 6usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "X conversion in regular sequence"]
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
    #[doc = "regular sequence register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "X conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "X conversion in regular sequence"]
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
    #[doc = "regular sequence register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr3(pub u32);
    impl Sqr3 {
        #[doc = "X conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "X conversion in regular sequence"]
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
    #[doc = "regular sequence register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr4(pub u32);
    impl Sqr4 {
        #[doc = "X conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "X conversion in regular sequence"]
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
    #[doc = "watchdog threshold register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tr1(pub u32);
    impl Tr1 {
        #[doc = "LT1"]
        #[inline(always)]
        pub const fn lt1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "LT1"]
        #[inline(always)]
        pub fn set_lt1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HT1"]
        #[inline(always)]
        pub const fn ht1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "HT1"]
        #[inline(always)]
        pub fn set_ht1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Tr1 {
        #[inline(always)]
        fn default() -> Tr1 {
            Tr1(0)
        }
    }
    impl core::fmt::Debug for Tr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tr1")
                .field("lt1", &self.lt1())
                .field("ht1", &self.ht1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tr1 {{ lt1: {=u16:?}, ht1: {=u16:?} }}", self.lt1(), self.ht1())
        }
    }
    #[doc = "watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tr2(pub u32);
    impl Tr2 {
        #[doc = "LT2"]
        #[inline(always)]
        pub const fn lt2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "LT2"]
        #[inline(always)]
        pub fn set_lt2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "HT2"]
        #[inline(always)]
        pub const fn ht2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "HT2"]
        #[inline(always)]
        pub fn set_ht2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Tr2 {
        #[inline(always)]
        fn default() -> Tr2 {
            Tr2(0)
        }
    }
    impl core::fmt::Debug for Tr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tr2")
                .field("lt2", &self.lt2())
                .field("ht2", &self.ht2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tr2 {{ lt2: {=u8:?}, ht2: {=u8:?} }}", self.lt2(), self.ht2())
        }
    }
    #[doc = "watchdog threshold register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tr3(pub u32);
    impl Tr3 {
        #[doc = "LT3"]
        #[inline(always)]
        pub const fn lt3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "LT3"]
        #[inline(always)]
        pub fn set_lt3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "HT3"]
        #[inline(always)]
        pub const fn ht3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "HT3"]
        #[inline(always)]
        pub fn set_ht3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Tr3 {
        #[inline(always)]
        fn default() -> Tr3 {
            Tr3(0)
        }
    }
    impl core::fmt::Debug for Tr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tr3")
                .field("lt3", &self.lt3())
                .field("ht3", &self.ht3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tr3 {{ lt3: {=u8:?}, ht3: {=u8:?} }}", self.lt3(), self.ht3())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Advregen {
        #[doc = "Intermediate state required when moving the ADC voltage regulator between states"]
        INTERMEDIATE = 0x0,
        #[doc = "ADC voltage regulator enabled"]
        ENABLED = 0x01,
        #[doc = "ADC voltage regulator disabled"]
        DISABLED = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Advregen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Advregen {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Advregen {
        #[inline(always)]
        fn from(val: u8) -> Advregen {
            Advregen::from_bits(val)
        }
    }
    impl From<Advregen> for u8 {
        #[inline(always)]
        fn from(val: Advregen) -> u8 {
            Advregen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Align {
        #[doc = "Right alignment"]
        RIGHT = 0x0,
        #[doc = "Left alignment"]
        LEFT = 0x01,
    }
    impl Align {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Align {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Align {
        #[inline(always)]
        fn from(val: u8) -> Align {
            Align::from_bits(val)
        }
    }
    impl From<Align> for u8 {
        #[inline(always)]
        fn from(val: Align) -> u8 {
            Align::to_bits(val)
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
    pub enum Difsel10 {
        #[doc = "Input channel is configured in single-ended mode"]
        SINGLE_ENDED = 0x0,
        #[doc = "Input channel is configured in differential mode"]
        DIFFERENTIAL = 0x01,
    }
    impl Difsel10 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Difsel10 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Difsel10 {
        #[inline(always)]
        fn from(val: u8) -> Difsel10 {
            Difsel10::from_bits(val)
        }
    }
    impl From<Difsel10> for u8 {
        #[inline(always)]
        fn from(val: Difsel10) -> u8 {
            Difsel10::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dmacfg {
        #[doc = "DMA One Shot mode selected"]
        ONE_SHOT = 0x0,
        #[doc = "DMA Circular mode selected"]
        CIRCULAR = 0x01,
    }
    impl Dmacfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dmacfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dmacfg {
        #[inline(always)]
        fn from(val: u8) -> Dmacfg {
            Dmacfg::from_bits(val)
        }
    }
    impl From<Dmacfg> for u8 {
        #[inline(always)]
        fn from(val: Dmacfg) -> u8 {
            Dmacfg::to_bits(val)
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
    pub enum Res {
        #[doc = "12-bit resolution"]
        BITS12 = 0x0,
        #[doc = "10-bit resolution"]
        BITS10 = 0x01,
        #[doc = "8-bit resolution"]
        BITS8 = 0x02,
        #[doc = "6-bit resolution"]
        BITS6 = 0x03,
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
        #[doc = "1.5 ADC clock cycles"]
        CYCLES1_5 = 0x0,
        #[doc = "2.5 ADC clock cycles"]
        CYCLES2_5 = 0x01,
        #[doc = "4.5 ADC clock cycles"]
        CYCLES4_5 = 0x02,
        #[doc = "7.5 ADC clock cycles"]
        CYCLES7_5 = 0x03,
        #[doc = "19.5 ADC clock cycles"]
        CYCLES19_5 = 0x04,
        #[doc = "61.5 ADC clock cycles"]
        CYCLES61_5 = 0x05,
        #[doc = "181.5 ADC clock cycles"]
        CYCLES181_5 = 0x06,
        #[doc = "601.5 ADC clock cycles"]
        CYCLES601_5 = 0x07,
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
