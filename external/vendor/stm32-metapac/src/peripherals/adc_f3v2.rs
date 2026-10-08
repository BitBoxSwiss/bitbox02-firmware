#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog-to-digital converter"]
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
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "sample time register 1"]
    #[inline(always)]
    pub const fn smpr1(self) -> crate::common::Reg<regs::Smpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "sample time register 2"]
    #[inline(always)]
    pub const fn smpr2(self) -> crate::common::Reg<regs::Smpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "sample time register 3"]
    #[inline(always)]
    pub const fn smpr3(self) -> crate::common::Reg<regs::Smpr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "injected channel data offset register 1"]
    #[inline(always)]
    pub const fn jofr1(self) -> crate::common::Reg<regs::Jofr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "injected channel data offset register 2"]
    #[inline(always)]
    pub const fn jofr2(self) -> crate::common::Reg<regs::Jofr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "injected channel data offset register 3"]
    #[inline(always)]
    pub const fn jofr3(self) -> crate::common::Reg<regs::Jofr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "injected channel data offset register 4"]
    #[inline(always)]
    pub const fn jofr4(self) -> crate::common::Reg<regs::Jofr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "watchdog higher threshold register"]
    #[inline(always)]
    pub const fn htr(self) -> crate::common::Reg<regs::Htr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "watchdog lower threshold register"]
    #[inline(always)]
    pub const fn ltr(self) -> crate::common::Reg<regs::Ltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
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
    #[doc = "regular sequence register 5"]
    #[inline(always)]
    pub const fn sqr5(self) -> crate::common::Reg<regs::Sqr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "injected sequence register"]
    #[inline(always)]
    pub const fn jsqr(self) -> crate::common::Reg<regs::Jsqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "injected data register x1"]
    #[inline(always)]
    pub const fn jdr1(self) -> crate::common::Reg<regs::Jdr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "injected data register 2"]
    #[inline(always)]
    pub const fn jdr2(self) -> crate::common::Reg<regs::Jdr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "injected data register 3"]
    #[inline(always)]
    pub const fn jdr3(self) -> crate::common::Reg<regs::Jdr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "injected data register 4"]
    #[inline(always)]
    pub const fn jdr4(self) -> crate::common::Reg<regs::Jdr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "regular data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "sample time register 0"]
    #[inline(always)]
    pub const fn smpr0(self) -> crate::common::Reg<regs::Smpr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "ADC common status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "ADC common control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC common control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub const fn adcpre(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub fn set_adcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Temperature sensor and VREFINT enable"]
        #[inline(always)]
        pub const fn tsvrefe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor and VREFINT enable"]
        #[inline(always)]
        pub fn set_tsvrefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr")
                .field("adcpre", &self.adcpre())
                .field("tsvrefe", &self.tsvrefe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ adcpre: {=u8:?}, tsvrefe: {=bool:?} }}",
                self.adcpre(),
                self.tsvrefe()
            )
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Analog watchdog channel select bits"]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Analog watchdog channel select bits"]
        #[inline(always)]
        pub fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Interrupt enable for EOC"]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable for EOC"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Analog watchdog interrupt enable"]
        #[inline(always)]
        pub const fn awdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog interrupt enable"]
        #[inline(always)]
        pub fn set_awdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Interrupt enable for injected channels"]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable for injected channels"]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Scan mode"]
        #[inline(always)]
        pub const fn scan(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Scan mode"]
        #[inline(always)]
        pub fn set_scan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable the watchdog on a single channel in scan mode"]
        #[inline(always)]
        pub const fn awdsgl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the watchdog on a single channel in scan mode"]
        #[inline(always)]
        pub fn set_awdsgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Automatic injected group conversion"]
        #[inline(always)]
        pub const fn jauto(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic injected group conversion"]
        #[inline(always)]
        pub fn set_jauto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Discontinuous mode on regular channels"]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode on regular channels"]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Discontinuous mode on injected channels"]
        #[inline(always)]
        pub const fn jdiscen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode on injected channels"]
        #[inline(always)]
        pub fn set_jdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Discontinuous mode channel count"]
        #[inline(always)]
        pub const fn discnum(&self) -> super::vals::Discnum {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Discnum::from_bits(val as u8)
        }
        #[doc = "Discontinuous mode channel count"]
        #[inline(always)]
        pub fn set_discnum(&mut self, val: super::vals::Discnum) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "Power down during the delay phase"]
        #[inline(always)]
        pub const fn pdd(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Power down during the delay phase"]
        #[inline(always)]
        pub fn set_pdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Power down during the idle phase"]
        #[inline(always)]
        pub const fn pdi(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Power down during the idle phase"]
        #[inline(always)]
        pub fn set_pdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Analog watchdog enable on injected channels"]
        #[inline(always)]
        pub const fn jawden(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog enable on injected channels"]
        #[inline(always)]
        pub fn set_jawden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog enable on regular channels"]
        #[inline(always)]
        pub const fn awden(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog enable on regular channels"]
        #[inline(always)]
        pub fn set_awden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Resolution"]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "Resolution"]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("awdch", &self.awdch())
                .field("eocie", &self.eocie())
                .field("awdie", &self.awdie())
                .field("jeocie", &self.jeocie())
                .field("scan", &self.scan())
                .field("awdsgl", &self.awdsgl())
                .field("jauto", &self.jauto())
                .field("discen", &self.discen())
                .field("jdiscen", &self.jdiscen())
                .field("discnum", &self.discnum())
                .field("pdd", &self.pdd())
                .field("pdi", &self.pdi())
                .field("jawden", &self.jawden())
                .field("awden", &self.awden())
                .field("res", &self.res())
                .field("ovrie", &self.ovrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ awdch: {=u8:?}, eocie: {=bool:?}, awdie: {=bool:?}, jeocie: {=bool:?}, scan: {=bool:?}, awdsgl: {=bool:?}, jauto: {=bool:?}, discen: {=bool:?}, jdiscen: {=bool:?}, discnum: {:?}, pdd: {=bool:?}, pdi: {=bool:?}, jawden: {=bool:?}, awden: {=bool:?}, res: {:?}, ovrie: {=bool:?} }}" , self . awdch () , self . eocie () , self . awdie () , self . jeocie () , self . scan () , self . awdsgl () , self . jauto () , self . discen () , self . jdiscen () , self . discnum () , self . pdd () , self . pdi () , self . jawden () , self . awden () , self . res () , self . ovrie ())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "A/D Converter ON / OFF"]
        #[inline(always)]
        pub const fn adon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A/D Converter ON / OFF"]
        #[inline(always)]
        pub fn set_adon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Continuous conversion"]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous conversion"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC configuration"]
        #[inline(always)]
        pub const fn adc_cfg(&self) -> super::vals::AdcCfg {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::AdcCfg::from_bits(val as u8)
        }
        #[doc = "ADC configuration"]
        #[inline(always)]
        pub fn set_adc_cfg(&mut self, val: super::vals::AdcCfg) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Delay selection"]
        #[inline(always)]
        pub const fn dels(&self) -> super::vals::Dels {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Dels::from_bits(val as u8)
        }
        #[doc = "Delay selection"]
        #[inline(always)]
        pub fn set_dels(&mut self, val: super::vals::Dels) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Direct memory access mode"]
        #[inline(always)]
        pub const fn dma(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Direct memory access mode"]
        #[inline(always)]
        pub fn set_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DMA disable selection"]
        #[inline(always)]
        pub const fn dds(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DMA disable selection"]
        #[inline(always)]
        pub fn set_dds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "End of conversion selection"]
        #[inline(always)]
        pub const fn eocs(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "End of conversion selection"]
        #[inline(always)]
        pub fn set_eocs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Data alignment"]
        #[inline(always)]
        pub const fn align(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data alignment"]
        #[inline(always)]
        pub fn set_align(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "External event select for injected group"]
        #[inline(always)]
        pub const fn jextsel(&self) -> super::vals::Jextsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Jextsel::from_bits(val as u8)
        }
        #[doc = "External event select for injected group"]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: super::vals::Jextsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "External trigger enable for injected channels"]
        #[inline(always)]
        pub const fn jexten(&self) -> super::vals::Exten {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "External trigger enable for injected channels"]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Start conversion of injected channels"]
        #[inline(always)]
        pub const fn jswstart(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Start conversion of injected channels"]
        #[inline(always)]
        pub fn set_jswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "External event select for regular group"]
        #[inline(always)]
        pub const fn extsel(&self) -> super::vals::Extsel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Extsel::from_bits(val as u8)
        }
        #[doc = "External event select for regular group"]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: super::vals::Extsel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "External trigger enable for regular channels"]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Exten {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "External trigger enable for regular channels"]
        #[inline(always)]
        pub fn set_exten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Start conversion of regular channels"]
        #[inline(always)]
        pub const fn swstart(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Start conversion of regular channels"]
        #[inline(always)]
        pub fn set_swstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("adon", &self.adon())
                .field("cont", &self.cont())
                .field("adc_cfg", &self.adc_cfg())
                .field("dels", &self.dels())
                .field("dma", &self.dma())
                .field("dds", &self.dds())
                .field("eocs", &self.eocs())
                .field("align", &self.align())
                .field("jextsel", &self.jextsel())
                .field("jexten", &self.jexten())
                .field("jswstart", &self.jswstart())
                .field("extsel", &self.extsel())
                .field("exten", &self.exten())
                .field("swstart", &self.swstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ adon: {=bool:?}, cont: {=bool:?}, adc_cfg: {:?}, dels: {:?}, dma: {=bool:?}, dds: {=bool:?}, eocs: {=bool:?}, align: {=bool:?}, jextsel: {:?}, jexten: {:?}, jswstart: {=bool:?}, extsel: {:?}, exten: {:?}, swstart: {=bool:?} }}" , self . adon () , self . cont () , self . adc_cfg () , self . dels () , self . dma () , self . dds () , self . eocs () , self . align () , self . jextsel () , self . jexten () , self . jswstart () , self . extsel () , self . exten () , self . swstart ())
        }
    }
    #[doc = "ADC common status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Analog watchdog flag of the ADC"]
        #[inline(always)]
        pub const fn awd1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog flag of the ADC"]
        #[inline(always)]
        pub fn set_awd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of conversion of the ADC"]
        #[inline(always)]
        pub const fn eoc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of conversion of the ADC"]
        #[inline(always)]
        pub fn set_eoc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected channel end of conversion of the ADC"]
        #[inline(always)]
        pub const fn jeoc1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel end of conversion of the ADC"]
        #[inline(always)]
        pub fn set_jeoc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Injected channel Start flag of the ADC"]
        #[inline(always)]
        pub const fn jstrt1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel Start flag of the ADC"]
        #[inline(always)]
        pub fn set_jstrt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Regular channel Start flag of the ADC"]
        #[inline(always)]
        pub const fn strt1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel Start flag of the ADC"]
        #[inline(always)]
        pub fn set_strt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Overrun flag of the ADC"]
        #[inline(always)]
        pub const fn ovr1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag of the ADC"]
        #[inline(always)]
        pub fn set_ovr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADON Status of ADC1"]
        #[inline(always)]
        pub const fn adons1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ADON Status of ADC1"]
        #[inline(always)]
        pub fn set_adons1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("awd1", &self.awd1())
                .field("eoc1", &self.eoc1())
                .field("jeoc1", &self.jeoc1())
                .field("jstrt1", &self.jstrt1())
                .field("strt1", &self.strt1())
                .field("ovr1", &self.ovr1())
                .field("adons1", &self.adons1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ awd1: {=bool:?}, eoc1: {=bool:?}, jeoc1: {=bool:?}, jstrt1: {=bool:?}, strt1: {=bool:?}, ovr1: {=bool:?}, adons1: {=bool:?} }}" , self . awd1 () , self . eoc1 () , self . jeoc1 () , self . jstrt1 () , self . strt1 () , self . ovr1 () , self . adons1 ())
        }
    }
    #[doc = "regular data register"]
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
    #[doc = "watchdog higher threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr(pub u32);
    impl Htr {
        #[doc = "Analog watchdog higher threshold"]
        #[inline(always)]
        pub const fn ht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog higher threshold"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Htr {
        #[inline(always)]
        fn default() -> Htr {
            Htr(0)
        }
    }
    impl core::fmt::Debug for Htr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Htr").field("ht", &self.ht()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Htr {{ ht: {=u16:?} }}", self.ht())
        }
    }
    #[doc = "injected data register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr1(pub u32);
    impl Jdr1 {
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
    impl Default for Jdr1 {
        #[inline(always)]
        fn default() -> Jdr1 {
            Jdr1(0)
        }
    }
    impl core::fmt::Debug for Jdr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jdr1").field("jdata", &self.jdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jdr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jdr1 {{ jdata: {=u16:?} }}", self.jdata())
        }
    }
    #[doc = "injected data register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr2(pub u32);
    impl Jdr2 {
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
    impl Default for Jdr2 {
        #[inline(always)]
        fn default() -> Jdr2 {
            Jdr2(0)
        }
    }
    impl core::fmt::Debug for Jdr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jdr2").field("jdata", &self.jdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jdr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jdr2 {{ jdata: {=u16:?} }}", self.jdata())
        }
    }
    #[doc = "injected data register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr3(pub u32);
    impl Jdr3 {
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
    impl Default for Jdr3 {
        #[inline(always)]
        fn default() -> Jdr3 {
            Jdr3(0)
        }
    }
    impl core::fmt::Debug for Jdr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jdr3").field("jdata", &self.jdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jdr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jdr3 {{ jdata: {=u16:?} }}", self.jdata())
        }
    }
    #[doc = "injected data register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr4(pub u32);
    impl Jdr4 {
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
    impl Default for Jdr4 {
        #[inline(always)]
        fn default() -> Jdr4 {
            Jdr4(0)
        }
    }
    impl core::fmt::Debug for Jdr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jdr4").field("jdata", &self.jdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jdr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jdr4 {{ jdata: {=u16:?} }}", self.jdata())
        }
    }
    #[doc = "injected channel data offset register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr1(pub u32);
    impl Jofr1 {
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub const fn joffset1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub fn set_joffset1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr1 {
        #[inline(always)]
        fn default() -> Jofr1 {
            Jofr1(0)
        }
    }
    impl core::fmt::Debug for Jofr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jofr1").field("joffset1", &self.joffset1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jofr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jofr1 {{ joffset1: {=u16:?} }}", self.joffset1())
        }
    }
    #[doc = "injected channel data offset register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr2(pub u32);
    impl Jofr2 {
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub const fn joffset2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub fn set_joffset2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr2 {
        #[inline(always)]
        fn default() -> Jofr2 {
            Jofr2(0)
        }
    }
    impl core::fmt::Debug for Jofr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jofr2").field("joffset2", &self.joffset2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jofr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jofr2 {{ joffset2: {=u16:?} }}", self.joffset2())
        }
    }
    #[doc = "injected channel data offset register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr3(pub u32);
    impl Jofr3 {
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub const fn joffset3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub fn set_joffset3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr3 {
        #[inline(always)]
        fn default() -> Jofr3 {
            Jofr3(0)
        }
    }
    impl core::fmt::Debug for Jofr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jofr3").field("joffset3", &self.joffset3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jofr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jofr3 {{ joffset3: {=u16:?} }}", self.joffset3())
        }
    }
    #[doc = "injected channel data offset register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr4(pub u32);
    impl Jofr4 {
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub const fn joffset4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub fn set_joffset4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr4 {
        #[inline(always)]
        fn default() -> Jofr4 {
            Jofr4(0)
        }
    }
    impl core::fmt::Debug for Jofr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jofr4").field("joffset4", &self.joffset4()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jofr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jofr4 {{ joffset4: {=u16:?} }}", self.joffset4())
        }
    }
    #[doc = "injected sequence register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jsqr(pub u32);
    impl Jsqr {
        #[doc = "conversion in injected sequence"]
        #[inline(always)]
        pub const fn jsq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "conversion in injected sequence"]
        #[inline(always)]
        pub fn set_jsq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
        #[doc = "Injected sequence length"]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Injected sequence length"]
        #[inline(always)]
        pub fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
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
                .field("jsq[0]", &self.jsq(0usize))
                .field("jsq[1]", &self.jsq(1usize))
                .field("jsq[2]", &self.jsq(2usize))
                .field("jsq[3]", &self.jsq(3usize))
                .field("jl", &self.jl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jsqr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Jsqr {{ jsq[0]: {=u8:?}, jsq[1]: {=u8:?}, jsq[2]: {=u8:?}, jsq[3]: {=u8:?}, jl: {=u8:?} }}",
                self.jsq(0usize),
                self.jsq(1usize),
                self.jsq(2usize),
                self.jsq(3usize),
                self.jl()
            )
        }
    }
    #[doc = "watchdog lower threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr(pub u32);
    impl Ltr {
        #[doc = "Analog watchdog lower threshold"]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog lower threshold"]
        #[inline(always)]
        pub fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Ltr {
        #[inline(always)]
        fn default() -> Ltr {
            Ltr(0)
        }
    }
    impl core::fmt::Debug for Ltr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ltr").field("lt", &self.lt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ltr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ltr {{ lt: {=u16:?} }}", self.lt())
        }
    }
    #[doc = "sample time register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr0(pub u32);
    impl Smpr0 {
        #[doc = "channel 30-31 sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 2usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 30-31 sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 2usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Smpr0 {
        #[inline(always)]
        fn default() -> Smpr0 {
            Smpr0(0)
        }
    }
    impl core::fmt::Debug for Smpr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smpr0")
                .field("smp[0]", &self.smp(0usize))
                .field("smp[1]", &self.smp(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Smpr0 {{ smp[0]: {:?}, smp[1]: {:?} }}",
                self.smp(0usize),
                self.smp(1usize)
            )
        }
    }
    #[doc = "sample time register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr1(pub u32);
    impl Smpr1 {
        #[doc = "channel 20-29 sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 20-29 sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
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
                .field("smp[9]", &self.smp(9usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smpr1 {{ smp[0]: {:?}, smp[1]: {:?}, smp[2]: {:?}, smp[3]: {:?}, smp[4]: {:?}, smp[5]: {:?}, smp[6]: {:?}, smp[7]: {:?}, smp[8]: {:?}, smp[9]: {:?} }}" , self . smp (0usize) , self . smp (1usize) , self . smp (2usize) , self . smp (3usize) , self . smp (4usize) , self . smp (5usize) , self . smp (6usize) , self . smp (7usize) , self . smp (8usize) , self . smp (9usize))
        }
    }
    #[doc = "sample time register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr2(pub u32);
    impl Smpr2 {
        #[doc = "channel 10-19 sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 10-19 sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 10usize);
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
                .field("smp[9]", &self.smp(9usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smpr2 {{ smp[0]: {:?}, smp[1]: {:?}, smp[2]: {:?}, smp[3]: {:?}, smp[4]: {:?}, smp[5]: {:?}, smp[6]: {:?}, smp[7]: {:?}, smp[8]: {:?}, smp[9]: {:?} }}" , self . smp (0usize) , self . smp (1usize) , self . smp (2usize) , self . smp (3usize) , self . smp (4usize) , self . smp (5usize) , self . smp (6usize) , self . smp (7usize) , self . smp (8usize) , self . smp (9usize))
        }
    }
    #[doc = "sample time register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr3(pub u32);
    impl Smpr3 {
        #[doc = "channel 0-9 sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 0-9 sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Smpr3 {
        #[inline(always)]
        fn default() -> Smpr3 {
            Smpr3(0)
        }
    }
    impl core::fmt::Debug for Smpr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smpr3")
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
    impl defmt::Format for Smpr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smpr3 {{ smp[0]: {:?}, smp[1]: {:?}, smp[2]: {:?}, smp[3]: {:?}, smp[4]: {:?}, smp[5]: {:?}, smp[6]: {:?}, smp[7]: {:?}, smp[8]: {:?}, smp[9]: {:?} }}" , self . smp (0usize) , self . smp (1usize) , self . smp (2usize) , self . smp (3usize) , self . smp (4usize) , self . smp (5usize) , self . smp (6usize) , self . smp (7usize) , self . smp (8usize) , self . smp (9usize))
        }
    }
    #[doc = "regular sequence register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "25th-29th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "25th-29th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
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
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("l", &self.l())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr1 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, l: {=u8:?} }}",
                self.sq(0usize),
                self.sq(1usize),
                self.sq(2usize),
                self.sq(3usize),
                self.l()
            )
        }
    }
    #[doc = "regular sequence register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "19th-24th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "19th-24th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
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
                .field("sq[5]", &self.sq(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sqr2 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?}, sq[5]: {=u8:?} }}" , self . sq (0usize) , self . sq (1usize) , self . sq (2usize) , self . sq (3usize) , self . sq (4usize) , self . sq (5usize))
        }
    }
    #[doc = "regular sequence register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr3(pub u32);
    impl Sqr3 {
        #[doc = "13th-18th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "13th-18th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
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
                .field("sq[5]", &self.sq(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sqr3 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?}, sq[5]: {=u8:?} }}" , self . sq (0usize) , self . sq (1usize) , self . sq (2usize) , self . sq (3usize) , self . sq (4usize) , self . sq (5usize))
        }
    }
    #[doc = "regular sequence register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr4(pub u32);
    impl Sqr4 {
        #[doc = "7th-12th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "7th-12th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
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
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .field("sq[5]", &self.sq(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sqr4 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?}, sq[5]: {=u8:?} }}" , self . sq (0usize) , self . sq (1usize) , self . sq (2usize) , self . sq (3usize) , self . sq (4usize) , self . sq (5usize))
        }
    }
    #[doc = "regular sequence register 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr5(pub u32);
    impl Sqr5 {
        #[doc = "1st-6th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "1st-6th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr5 {
        #[inline(always)]
        fn default() -> Sqr5 {
            Sqr5(0)
        }
    }
    impl core::fmt::Debug for Sqr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr5")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .field("sq[5]", &self.sq(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sqr5 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?}, sq[5]: {=u8:?} }}" , self . sq (0usize) , self . sq (1usize) , self . sq (2usize) , self . sq (3usize) , self . sq (4usize) , self . sq (5usize))
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Analog watchdog flag"]
        #[inline(always)]
        pub const fn awd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog flag"]
        #[inline(always)]
        pub fn set_awd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Regular channel end of conversion"]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel end of conversion"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected channel end of conversion"]
        #[inline(always)]
        pub const fn jeoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel end of conversion"]
        #[inline(always)]
        pub fn set_jeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Injected channel start flag"]
        #[inline(always)]
        pub const fn jstrt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel start flag"]
        #[inline(always)]
        pub fn set_jstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Regular channel start flag"]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel start flag"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Overrun"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC ON status"]
        #[inline(always)]
        pub const fn adons(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ON status"]
        #[inline(always)]
        pub fn set_adons(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Regular channel not ready"]
        #[inline(always)]
        pub const fn rcnr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel not ready"]
        #[inline(always)]
        pub fn set_rcnr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Injected channel not ready"]
        #[inline(always)]
        pub const fn jcnr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel not ready"]
        #[inline(always)]
        pub fn set_jcnr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("awd", &self.awd())
                .field("eoc", &self.eoc())
                .field("jeoc", &self.jeoc())
                .field("jstrt", &self.jstrt())
                .field("strt", &self.strt())
                .field("ovr", &self.ovr())
                .field("adons", &self.adons())
                .field("rcnr", &self.rcnr())
                .field("jcnr", &self.jcnr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ awd: {=bool:?}, eoc: {=bool:?}, jeoc: {=bool:?}, jstrt: {=bool:?}, strt: {=bool:?}, ovr: {=bool:?}, adons: {=bool:?}, rcnr: {=bool:?}, jcnr: {=bool:?} }}" , self . awd () , self . eoc () , self . jeoc () , self . jstrt () , self . strt () , self . ovr () , self . adons () , self . rcnr () , self . jcnr ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AdcCfg {
        #[doc = "Bank A selected for channels ADC_IN0..31"]
        BANK_A = 0x0,
        #[doc = "Bank B selected for channels ADC_IN0..31b"]
        BANK_B = 0x01,
    }
    impl AdcCfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AdcCfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AdcCfg {
        #[inline(always)]
        fn from(val: u8) -> AdcCfg {
            AdcCfg::from_bits(val)
        }
    }
    impl From<AdcCfg> for u8 {
        #[inline(always)]
        fn from(val: AdcCfg) -> u8 {
            AdcCfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dels {
        #[doc = "No Delay"]
        NO_DELAY = 0x0,
        #[doc = "Until the converted data have been read"]
        AFTER_READ = 0x01,
        #[doc = "Delay 7 APB clock cycles after the conversion"]
        DELAY_7_CLK = 0x02,
        #[doc = "Delay 16 APB clock cycles after the conversion"]
        DELAY_15_CLK = 0x03,
        #[doc = "Delay 31 APB clock cycles after the conversion"]
        DELAY_31_CLK = 0x04,
        #[doc = "Delay 63 APB clock cycles after the conversion"]
        DELAY_63_CLK = 0x05,
        #[doc = "Delay 127 APB clock cycles after the conversion"]
        DELAY_127_CLK = 0x06,
        #[doc = "Delay 255 APB clock cycles after the conversion"]
        DELAY_255_CLK = 0x07,
    }
    impl Dels {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dels {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dels {
        #[inline(always)]
        fn from(val: u8) -> Dels {
            Dels::from_bits(val)
        }
    }
    impl From<Dels> for u8 {
        #[inline(always)]
        fn from(val: Dels) -> u8 {
            Dels::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Discnum {
        #[doc = "1 conversions are discontinued and the conversion is carried out on one channel"]
        DISCNUM_1 = 0x0,
        #[doc = "2 conversion is discontinued and the conversions are carried out on 2 channels"]
        DISCNUM_2 = 0x01,
        #[doc = "3 conversions are discontinued and the conversions are carried out on 3 channels"]
        DISCNUM_3 = 0x02,
        #[doc = "4 conversions are discontinued and the conversions are carried out on 4 channels"]
        DISCNUM_4 = 0x03,
        #[doc = "5 conversions are discontinued and the conversions are carried out on 5 channels"]
        DISCNUM_5 = 0x04,
        #[doc = "6 conversions are discontinued and the conversions are carried out on 6 channels"]
        DISCNUM_6 = 0x05,
        #[doc = "7 conversions are discontinued and the conversions are carried out on 7 channels"]
        DISCNUM_7 = 0x06,
        #[doc = "8 conversions are discontinued and the conversions are carried out on 8 channels"]
        DISCNUM_8 = 0x07,
    }
    impl Discnum {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Discnum {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Discnum {
        #[inline(always)]
        fn from(val: u8) -> Discnum {
            Discnum::from_bits(val)
        }
    }
    impl From<Discnum> for u8 {
        #[inline(always)]
        fn from(val: Discnum) -> u8 {
            Discnum::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Exten {
        #[doc = "Trigger detection disabled"]
        DISABLED = 0x0,
        #[doc = "Trigger detection on the rising edge"]
        RISING = 0x01,
        #[doc = "Trigger detection on the falling edge"]
        FALLING = 0x02,
        #[doc = "Trigger detection on both edges"]
        BOTH = 0x03,
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
    pub enum Extsel {
        #[doc = "Timer 9 CC2 event"]
        TIM9_CC2 = 0x0,
        #[doc = "Timer 9 TRGO event"]
        TIM9_TRGO = 0x01,
        #[doc = "Timer 2 CC3 event"]
        TIM2_CC3 = 0x02,
        #[doc = "Timer 2 CC2 event"]
        TIM2_CC2 = 0x03,
        #[doc = "Timer 3 TRGO event"]
        TIM3_TRGO = 0x04,
        #[doc = "Timer 4 CC4 event"]
        TIM4_CC4 = 0x05,
        #[doc = "Timer 2 TRGO event"]
        TIM2_TRGO = 0x06,
        #[doc = "Timer 3 CC1 event"]
        TIM3_CC1 = 0x07,
        #[doc = "Timer 3 CC3 event"]
        TIM3_CC3 = 0x08,
        #[doc = "Timer 4 TRGO event"]
        TIM4_TRGO = 0x09,
        #[doc = "Timer 6 TRGO event"]
        TIM6_TRGO = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "External interrupt line 11"]
        EXTI_LINE11 = 0x0f,
    }
    impl Extsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extsel {
        #[inline(always)]
        fn from(val: u8) -> Extsel {
            Extsel::from_bits(val)
        }
    }
    impl From<Extsel> for u8 {
        #[inline(always)]
        fn from(val: Extsel) -> u8 {
            Extsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Jextsel {
        #[doc = "Timer 9 CC1 event"]
        TIM9_CC1 = 0x0,
        #[doc = "Timer 9 TRGO event"]
        TIM9_TRGO = 0x01,
        #[doc = "Timer 2 TRGO event"]
        TIM2_TRGO = 0x02,
        #[doc = "Timer 2 CC1 event"]
        TIM2_CC1 = 0x03,
        #[doc = "Timer 3 CC4 event"]
        TIM3_CC4 = 0x04,
        #[doc = "Timer 4 TRGO event"]
        TIM4_TRGO = 0x05,
        #[doc = "Timer 4 CC1 event"]
        TIM4_CC1 = 0x06,
        #[doc = "Timer 4 CC2 event"]
        TIM4_CC2 = 0x07,
        #[doc = "Timer 4 CC3 event"]
        TIM4_CC3 = 0x08,
        #[doc = "Timer 4 CC3 event"]
        TIM10_CC1 = 0x09,
        #[doc = "Timer 7 TRGO event"]
        TIM7_TRGO = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "External interrupt line 15"]
        EXTI_LINE15 = 0x0f,
    }
    impl Jextsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jextsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jextsel {
        #[inline(always)]
        fn from(val: u8) -> Jextsel {
            Jextsel::from_bits(val)
        }
    }
    impl From<Jextsel> for u8 {
        #[inline(always)]
        fn from(val: Jextsel) -> u8 {
            Jextsel::to_bits(val)
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
        #[doc = "4 ADC clock cycles"]
        CYCLES4 = 0x0,
        #[doc = "9 ADC clock cycles"]
        CYCLES9 = 0x01,
        #[doc = "16 ADC clock cycles"]
        CYCLES16 = 0x02,
        #[doc = "24 ADC clock cycles"]
        CYCLES24 = 0x03,
        #[doc = "48 ADC clock cycles"]
        CYCLES48 = 0x04,
        #[doc = "96 ADC clock cycles"]
        CYCLES96 = 0x05,
        #[doc = "192 ADC clock cycles"]
        CYCLES192 = 0x06,
        #[doc = "384 ADC clock cycles"]
        CYCLES384 = 0x07,
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
