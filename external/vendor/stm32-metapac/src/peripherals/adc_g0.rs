#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog to Digital Converter instance 1"]
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
    #[doc = "ADC interrupt and status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ADC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ADC control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ADC configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "ADC sampling time register"]
    #[inline(always)]
    pub const fn smpr(self) -> crate::common::Reg<regs::Smpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "watchdog threshold register"]
    #[inline(always)]
    pub const fn awd1tr(self) -> crate::common::Reg<regs::Awd1tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "watchdog threshold register"]
    #[inline(always)]
    pub const fn awd2tr(self) -> crate::common::Reg<regs::Awd2tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "channel selection register"]
    #[inline(always)]
    pub const fn chselr(self) -> crate::common::Reg<regs::Chselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
    #[inline(always)]
    pub const fn chselr_1(self) -> crate::common::Reg<regs::Chselr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "watchdog threshold register"]
    #[inline(always)]
    pub const fn awd3tr(self) -> crate::common::Reg<regs::Awd3tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "ADC group regular conversion data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "ADC analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn awd2cr(self) -> crate::common::Reg<regs::Awd2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "ADC analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn awd3cr(self) -> crate::common::Reg<regs::Awd3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "ADC calibration factors register"]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "ADC common control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
}
pub mod regs {
    #[doc = "watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd1tr(pub u32);
    impl Awd1tr {
        #[doc = "ADC analog watchdog 1 threshold low"]
        #[inline(always)]
        pub const fn lt1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 1 threshold low"]
        #[inline(always)]
        pub fn set_lt1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "ADC analog watchdog 1 threshold high"]
        #[inline(always)]
        pub const fn ht1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 1 threshold high"]
        #[inline(always)]
        pub fn set_ht1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awd1tr {
        #[inline(always)]
        fn default() -> Awd1tr {
            Awd1tr(0)
        }
    }
    impl core::fmt::Debug for Awd1tr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd1tr")
                .field("lt1", &self.lt1())
                .field("ht1", &self.ht1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd1tr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Awd1tr {{ lt1: {=u16:?}, ht1: {=u16:?} }}", self.lt1(), self.ht1())
        }
    }
    #[doc = "ADC analog watchdog 2 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2cr(pub u32);
    impl Awd2cr {
        #[doc = "ADC analog watchdog 2 monitored channel selection"]
        #[inline(always)]
        pub const fn awd2ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "ADC analog watchdog 2 monitored channel selection"]
        #[inline(always)]
        pub fn set_awd2ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
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
            f.debug_struct("Awd2cr").field("awd2ch", &self.awd2ch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Awd2cr {{ awd2ch: {=u32:?} }}", self.awd2ch())
        }
    }
    #[doc = "watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2tr(pub u32);
    impl Awd2tr {
        #[doc = "ADC analog watchdog 2 threshold low"]
        #[inline(always)]
        pub const fn lt2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 2 threshold low"]
        #[inline(always)]
        pub fn set_lt2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "ADC analog watchdog 2 threshold high"]
        #[inline(always)]
        pub const fn ht2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 2 threshold high"]
        #[inline(always)]
        pub fn set_ht2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awd2tr {
        #[inline(always)]
        fn default() -> Awd2tr {
            Awd2tr(0)
        }
    }
    impl core::fmt::Debug for Awd2tr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd2tr")
                .field("lt2", &self.lt2())
                .field("ht2", &self.ht2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd2tr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Awd2tr {{ lt2: {=u16:?}, ht2: {=u16:?} }}", self.lt2(), self.ht2())
        }
    }
    #[doc = "ADC analog watchdog 3 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3cr(pub u32);
    impl Awd3cr {
        #[doc = "ADC analog watchdog 3 monitored channel selection"]
        #[inline(always)]
        pub const fn awd3ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "ADC analog watchdog 3 monitored channel selection"]
        #[inline(always)]
        pub fn set_awd3ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
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
            f.debug_struct("Awd3cr").field("awd3ch", &self.awd3ch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Awd3cr {{ awd3ch: {=u32:?} }}", self.awd3ch())
        }
    }
    #[doc = "watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3tr(pub u32);
    impl Awd3tr {
        #[doc = "ADC analog watchdog 3 threshold high"]
        #[inline(always)]
        pub const fn lt3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 3 threshold high"]
        #[inline(always)]
        pub fn set_lt3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "ADC analog watchdog 3 threshold high"]
        #[inline(always)]
        pub const fn ht3(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 3 threshold high"]
        #[inline(always)]
        pub fn set_ht3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awd3tr {
        #[inline(always)]
        fn default() -> Awd3tr {
            Awd3tr(0)
        }
    }
    impl core::fmt::Debug for Awd3tr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd3tr")
                .field("lt3", &self.lt3())
                .field("ht3", &self.ht3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd3tr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Awd3tr {{ lt3: {=u16:?}, ht3: {=u16:?} }}", self.lt3(), self.ht3())
        }
    }
    #[doc = "ADC calibration factors register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact(pub u32);
    impl Calfact {
        #[doc = "ADC calibration factor in single-ended mode"]
        #[inline(always)]
        pub const fn calfact(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "ADC calibration factor in single-ended mode"]
        #[inline(always)]
        pub fn set_calfact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
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
            f.debug_struct("Calfact").field("calfact", &self.calfact()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calfact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Calfact {{ calfact: {=u8:?} }}", self.calfact())
        }
    }
    #[doc = "ADC common control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub const fn presc(&self) -> super::vals::Presc {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Presc::from_bits(val as u8)
        }
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub fn set_presc(&mut self, val: super::vals::Presc) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature sensor enable"]
        #[inline(always)]
        pub const fn tsen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor enable"]
        #[inline(always)]
        pub fn set_tsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "VBAT enable"]
        #[inline(always)]
        pub const fn vbaten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT enable"]
        #[inline(always)]
        pub fn set_vbaten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("presc", &self.presc())
                .field("vrefen", &self.vrefen())
                .field("tsen", &self.tsen())
                .field("vbaten", &self.vbaten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ presc: {:?}, vrefen: {=bool:?}, tsen: {=bool:?}, vbaten: {=bool:?} }}",
                self.presc(),
                self.vrefen(),
                self.tsen(),
                self.vbaten()
            )
        }
    }
    #[doc = "ADC configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "ADC DMA transfer enable"]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC DMA transfer enable"]
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
        #[doc = "Scan sequence direction"]
        #[inline(always)]
        pub const fn scandir(&self) -> super::vals::Scandir {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Scandir::from_bits(val as u8)
        }
        #[doc = "Scan sequence direction"]
        #[inline(always)]
        pub fn set_scandir(&mut self, val: super::vals::Scandir) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC data resolution"]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "ADC data resolution"]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "ADC data alignement"]
        #[inline(always)]
        pub const fn align(&self) -> super::vals::Align {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Align::from_bits(val as u8)
        }
        #[doc = "ADC data alignement"]
        #[inline(always)]
        pub fn set_align(&mut self, val: super::vals::Align) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC group regular external trigger source"]
        #[inline(always)]
        pub const fn extsel(&self) -> super::vals::Extsel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Extsel::from_bits(val as u8)
        }
        #[doc = "ADC group regular external trigger source"]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: super::vals::Extsel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "ADC group regular external trigger polarity"]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Exten {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "ADC group regular external trigger polarity"]
        #[inline(always)]
        pub fn set_exten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "ADC group regular overrun configuration"]
        #[inline(always)]
        pub const fn ovrmod(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular overrun configuration"]
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
        #[doc = "Wait conversion mode"]
        #[inline(always)]
        pub const fn wait(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wait conversion mode"]
        #[inline(always)]
        pub fn set_wait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Auto-off mode"]
        #[inline(always)]
        pub const fn autoff(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-off mode"]
        #[inline(always)]
        pub fn set_autoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC group regular sequencer discontinuous mode"]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular sequencer discontinuous mode"]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Mode selection of the ADC_CHSELR register"]
        #[inline(always)]
        pub const fn chselrmod(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Mode selection of the ADC_CHSELR register"]
        #[inline(always)]
        pub fn set_chselrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC analog watchdog 1 monitoring a single channel or all channels"]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 1 monitoring a single channel or all channels"]
        #[inline(always)]
        pub fn set_awd1sgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ADC analog watchdog 1 enable on scope ADC group regular"]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 1 enable on scope ADC group regular"]
        #[inline(always)]
        pub fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ADC analog watchdog 1 monitored channel selection"]
        #[inline(always)]
        pub const fn awd1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC analog watchdog 1 monitored channel selection"]
        #[inline(always)]
        pub fn set_awd1ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
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
                .field("dmaen", &self.dmaen())
                .field("dmacfg", &self.dmacfg())
                .field("scandir", &self.scandir())
                .field("res", &self.res())
                .field("align", &self.align())
                .field("extsel", &self.extsel())
                .field("exten", &self.exten())
                .field("ovrmod", &self.ovrmod())
                .field("cont", &self.cont())
                .field("wait", &self.wait())
                .field("autoff", &self.autoff())
                .field("discen", &self.discen())
                .field("chselrmod", &self.chselrmod())
                .field("awd1sgl", &self.awd1sgl())
                .field("awd1en", &self.awd1en())
                .field("awd1ch", &self.awd1ch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ dmaen: {=bool:?}, dmacfg: {:?}, scandir: {:?}, res: {:?}, align: {:?}, extsel: {:?}, exten: {:?}, ovrmod: {=bool:?}, cont: {=bool:?}, wait: {=bool:?}, autoff: {=bool:?}, discen: {=bool:?}, chselrmod: {=bool:?}, awd1sgl: {=bool:?}, awd1en: {=bool:?}, awd1ch: {=u8:?} }}" , self . dmaen () , self . dmacfg () , self . scandir () , self . res () , self . align () , self . extsel () , self . exten () , self . ovrmod () , self . cont () , self . wait () , self . autoff () , self . discen () , self . chselrmod () , self . awd1sgl () , self . awd1en () , self . awd1ch ())
        }
    }
    #[doc = "ADC configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "ADC oversampler enable on scope ADC group regular"]
        #[inline(always)]
        pub const fn ovse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC oversampler enable on scope ADC group regular"]
        #[inline(always)]
        pub fn set_ovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC oversampling ratio"]
        #[inline(always)]
        pub const fn ovsr(&self) -> super::vals::Ovsr {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::Ovsr::from_bits(val as u8)
        }
        #[doc = "ADC oversampling ratio"]
        #[inline(always)]
        pub fn set_ovsr(&mut self, val: super::vals::Ovsr) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
        }
        #[doc = "ADC oversampling shift"]
        #[inline(always)]
        pub const fn ovss(&self) -> super::vals::Ovss {
            let val = (self.0 >> 5usize) & 0x0f;
            super::vals::Ovss::from_bits(val as u8)
        }
        #[doc = "ADC oversampling shift"]
        #[inline(always)]
        pub fn set_ovss(&mut self, val: super::vals::Ovss) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
        }
        #[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
        #[inline(always)]
        pub const fn tovs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
        #[inline(always)]
        pub fn set_tovs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Low frequency trigger mode enable"]
        #[inline(always)]
        pub const fn lftrig(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Low frequency trigger mode enable"]
        #[inline(always)]
        pub fn set_lftrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ADC clock mode"]
        #[inline(always)]
        pub const fn ckmode(&self) -> super::vals::Ckmode {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Ckmode::from_bits(val as u8)
        }
        #[doc = "ADC clock mode"]
        #[inline(always)]
        pub fn set_ckmode(&mut self, val: super::vals::Ckmode) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
                .field("ovse", &self.ovse())
                .field("ovsr", &self.ovsr())
                .field("ovss", &self.ovss())
                .field("tovs", &self.tovs())
                .field("lftrig", &self.lftrig())
                .field("ckmode", &self.ckmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ ovse: {=bool:?}, ovsr: {:?}, ovss: {:?}, tovs: {=bool:?}, lftrig: {=bool:?}, ckmode: {:?} }}",
                self.ovse(),
                self.ovsr(),
                self.ovss(),
                self.tovs(),
                self.lftrig(),
                self.ckmode()
            )
        }
    }
    #[doc = "channel selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chselr(pub u32);
    impl Chselr {
        #[doc = "Channel-x selection"]
        #[inline(always)]
        pub const fn chsel(&self, n: usize) -> bool {
            assert!(n < 19usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel-x selection"]
        #[inline(always)]
        pub fn set_chsel(&mut self, n: usize, val: bool) {
            assert!(n < 19usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Chselr {
        #[inline(always)]
        fn default() -> Chselr {
            Chselr(0)
        }
    }
    impl core::fmt::Debug for Chselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chselr")
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chselr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Chselr {{ chsel[0]: {=bool:?}, chsel[1]: {=bool:?}, chsel[2]: {=bool:?}, chsel[3]: {=bool:?}, chsel[4]: {=bool:?}, chsel[5]: {=bool:?}, chsel[6]: {=bool:?}, chsel[7]: {=bool:?}, chsel[8]: {=bool:?}, chsel[9]: {=bool:?}, chsel[10]: {=bool:?}, chsel[11]: {=bool:?}, chsel[12]: {=bool:?}, chsel[13]: {=bool:?}, chsel[14]: {=bool:?}, chsel[15]: {=bool:?}, chsel[16]: {=bool:?}, chsel[17]: {=bool:?}, chsel[18]: {=bool:?} }}" , self . chsel (0usize) , self . chsel (1usize) , self . chsel (2usize) , self . chsel (3usize) , self . chsel (4usize) , self . chsel (5usize) , self . chsel (6usize) , self . chsel (7usize) , self . chsel (8usize) , self . chsel (9usize) , self . chsel (10usize) , self . chsel (11usize) , self . chsel (12usize) , self . chsel (13usize) , self . chsel (14usize) , self . chsel (15usize) , self . chsel (16usize) , self . chsel (17usize) , self . chsel (18usize))
        }
    }
    #[doc = "channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chselr1(pub u32);
    impl Chselr1 {
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> super::vals::Sq {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::Sq::from_bits(val as u8)
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: super::vals::Sq) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for Chselr1 {
        #[inline(always)]
        fn default() -> Chselr1 {
            Chselr1(0)
        }
    }
    impl core::fmt::Debug for Chselr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chselr1")
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
    impl defmt::Format for Chselr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Chselr1 {{ sq[0]: {:?}, sq[1]: {:?}, sq[2]: {:?}, sq[3]: {:?}, sq[4]: {:?}, sq[5]: {:?}, sq[6]: {:?}, sq[7]: {:?} }}" , self . sq (0usize) , self . sq (1usize) , self . sq (2usize) , self . sq (3usize) , self . sq (4usize) , self . sq (5usize) , self . sq (6usize) , self . sq (7usize))
        }
    }
    #[doc = "ADC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "ADC enable"]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC enable"]
        #[inline(always)]
        pub fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC disable"]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC disable"]
        #[inline(always)]
        pub fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC group regular conversion start"]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular conversion start"]
        #[inline(always)]
        pub fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC group regular conversion stop"]
        #[inline(always)]
        pub const fn adstp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular conversion stop"]
        #[inline(always)]
        pub fn set_adstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC voltage regulator enable"]
        #[inline(always)]
        pub const fn advregen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC voltage regulator enable"]
        #[inline(always)]
        pub fn set_advregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("adstp", &self.adstp())
                .field("advregen", &self.advregen())
                .field("adcal", &self.adcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ aden: {=bool:?}, addis: {=bool:?}, adstart: {=bool:?}, adstp: {=bool:?}, advregen: {=bool:?}, adcal: {=bool:?} }}" , self . aden () , self . addis () , self . adstart () , self . adstp () , self . advregen () , self . adcal ())
        }
    }
    #[doc = "ADC group regular conversion data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "ADC group regular conversion data"]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ADC group regular conversion data"]
        #[inline(always)]
        pub fn set_data(&mut self, val: u16) {
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
            f.debug_struct("Dr").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ data: {=u16:?} }}", self.data())
        }
    }
    #[doc = "ADC interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "ADC ready interrupt"]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready interrupt"]
        #[inline(always)]
        pub fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC group regular end of sampling interrupt"]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of sampling interrupt"]
        #[inline(always)]
        pub fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC group regular end of unitary conversion interrupt"]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of unitary conversion interrupt"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC group regular end of sequence conversions interrupt"]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of sequence conversions interrupt"]
        #[inline(always)]
        pub fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC group regular overrun interrupt"]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular overrun interrupt"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC analog watchdog 1 interrupt"]
        #[inline(always)]
        pub const fn awd1ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 1 interrupt"]
        #[inline(always)]
        pub fn set_awd1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC analog watchdog 2 interrupt"]
        #[inline(always)]
        pub const fn awd2ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 2 interrupt"]
        #[inline(always)]
        pub fn set_awd2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC analog watchdog 3 interrupt"]
        #[inline(always)]
        pub const fn awd3ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 3 interrupt"]
        #[inline(always)]
        pub fn set_awd3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "End of calibration interrupt enable"]
        #[inline(always)]
        pub const fn eocalie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "End of calibration interrupt enable"]
        #[inline(always)]
        pub fn set_eocalie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Channel Configuration Ready Interrupt enable"]
        #[inline(always)]
        pub const fn ccrdyie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Configuration Ready Interrupt enable"]
        #[inline(always)]
        pub fn set_ccrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("awd1ie", &self.awd1ie())
                .field("awd2ie", &self.awd2ie())
                .field("awd3ie", &self.awd3ie())
                .field("eocalie", &self.eocalie())
                .field("ccrdyie", &self.ccrdyie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ adrdyie: {=bool:?}, eosmpie: {=bool:?}, eocie: {=bool:?}, eosie: {=bool:?}, ovrie: {=bool:?}, awd1ie: {=bool:?}, awd2ie: {=bool:?}, awd3ie: {=bool:?}, eocalie: {=bool:?}, ccrdyie: {=bool:?} }}" , self . adrdyie () , self . eosmpie () , self . eocie () , self . eosie () , self . ovrie () , self . awd1ie () , self . awd2ie () , self . awd3ie () , self . eocalie () , self . ccrdyie ())
        }
    }
    #[doc = "ADC interrupt and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "ADC ready flag"]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready flag"]
        #[inline(always)]
        pub fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC group regular end of sampling flag"]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of sampling flag"]
        #[inline(always)]
        pub fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC group regular end of unitary conversion flag"]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of unitary conversion flag"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC group regular end of sequence conversions flag"]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of sequence conversions flag"]
        #[inline(always)]
        pub fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC group regular overrun flag"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular overrun flag"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC analog watchdog 1 flag"]
        #[inline(always)]
        pub const fn awd1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 1 flag"]
        #[inline(always)]
        pub fn set_awd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC analog watchdog 2 flag"]
        #[inline(always)]
        pub const fn awd2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 2 flag"]
        #[inline(always)]
        pub fn set_awd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC analog watchdog 3 flag"]
        #[inline(always)]
        pub const fn awd3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 3 flag"]
        #[inline(always)]
        pub fn set_awd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "End Of Calibration flag"]
        #[inline(always)]
        pub const fn eocal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "End Of Calibration flag"]
        #[inline(always)]
        pub fn set_eocal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Channel Configuration Ready flag"]
        #[inline(always)]
        pub const fn ccrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Configuration Ready flag"]
        #[inline(always)]
        pub fn set_ccrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("awd1", &self.awd1())
                .field("awd2", &self.awd2())
                .field("awd3", &self.awd3())
                .field("eocal", &self.eocal())
                .field("ccrdy", &self.ccrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ adrdy: {=bool:?}, eosmp: {=bool:?}, eoc: {=bool:?}, eos: {=bool:?}, ovr: {=bool:?}, awd1: {=bool:?}, awd2: {=bool:?}, awd3: {=bool:?}, eocal: {=bool:?}, ccrdy: {=bool:?} }}" , self . adrdy () , self . eosmp () , self . eoc () , self . eos () , self . ovr () , self . awd1 () , self . awd2 () , self . awd3 () , self . eocal () , self . ccrdy ())
        }
    }
    #[doc = "ADC sampling time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr(pub u32);
    impl Smpr {
        #[doc = "Sampling time selection"]
        #[inline(always)]
        pub const fn sample_time(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Sampling time selection"]
        #[inline(always)]
        pub fn set_sample_time(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Channel sampling time selection"]
        #[inline(always)]
        pub const fn smpsel(&self, n: usize) -> super::vals::Smpsel {
            assert!(n < 19usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Smpsel::from_bits(val as u8)
        }
        #[doc = "Channel sampling time selection"]
        #[inline(always)]
        pub fn set_smpsel(&mut self, n: usize, val: super::vals::Smpsel) {
            assert!(n < 19usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
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
                .field("sample_time[0]", &self.sample_time(0usize))
                .field("sample_time[1]", &self.sample_time(1usize))
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smpr {{ sample_time[0]: {:?}, sample_time[1]: {:?}, smpsel[0]: {:?}, smpsel[1]: {:?}, smpsel[2]: {:?}, smpsel[3]: {:?}, smpsel[4]: {:?}, smpsel[5]: {:?}, smpsel[6]: {:?}, smpsel[7]: {:?}, smpsel[8]: {:?}, smpsel[9]: {:?}, smpsel[10]: {:?}, smpsel[11]: {:?}, smpsel[12]: {:?}, smpsel[13]: {:?}, smpsel[14]: {:?}, smpsel[15]: {:?}, smpsel[16]: {:?}, smpsel[17]: {:?}, smpsel[18]: {:?} }}" , self . sample_time (0usize) , self . sample_time (1usize) , self . smpsel (0usize) , self . smpsel (1usize) , self . smpsel (2usize) , self . smpsel (3usize) , self . smpsel (4usize) , self . smpsel (5usize) , self . smpsel (6usize) , self . smpsel (7usize) , self . smpsel (8usize) , self . smpsel (9usize) , self . smpsel (10usize) , self . smpsel (11usize) , self . smpsel (12usize) , self . smpsel (13usize) , self . smpsel (14usize) , self . smpsel (15usize) , self . smpsel (16usize) , self . smpsel (17usize) , self . smpsel (18usize))
        }
    }
}
pub mod vals {
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
    pub enum Ckmode {
        #[doc = "ADCCLK (Asynchronous clock mode)"]
        ADCLK = 0x0,
        #[doc = "PCLK/2 (Synchronous clock mode)"]
        PCLK_DIV2 = 0x01,
        #[doc = "PCLK/4 (Synchronous clock mode)"]
        PCLK_DIV4 = 0x02,
        #[doc = "PCLK (Synchronous clock mode)"]
        PCLK = 0x03,
    }
    impl Ckmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckmode {
        #[inline(always)]
        fn from(val: u8) -> Ckmode {
            Ckmode::from_bits(val)
        }
    }
    impl From<Ckmode> for u8 {
        #[inline(always)]
        fn from(val: Ckmode) -> u8 {
            Ckmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dmacfg {
        #[doc = "DMA one shot mode selected"]
        ONE_SHOT = 0x0,
        #[doc = "DMA circular mode selected"]
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
        #[doc = "Hardware trigger detection disabled"]
        DISABLED = 0x0,
        #[doc = "Hardware trigger detection on the rising edge"]
        RISING_EDGE = 0x01,
        #[doc = "Hardware trigger detection on the falling edge"]
        FALLING_EDGE = 0x02,
        #[doc = "Hardware trigger detection on both the rising and falling edges"]
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
    pub enum Extsel {
        #[doc = "Timer 1 TRGO2 event"]
        TIM1_TRGO2 = 0x0,
        #[doc = "Timer 1 CC4 event"]
        TIM1_CC4 = 0x01,
        #[doc = "Timer 2 TRGO event"]
        TIM2_TRGO = 0x02,
        #[doc = "Timer 3 TRGO event"]
        TIM3_TRGO = 0x03,
        #[doc = "Timer 15 TRGO event"]
        TIM15_TRGO = 0x04,
        #[doc = "Timer 6 TRGO event"]
        TIM6_TRGO = 0x05,
        #[doc = "Timer 4 TRGO event"]
        TIM4_TRGO = 0x06,
        #[doc = "EXTI line 11 event"]
        EXTI_LINE11 = 0x07,
    }
    impl Extsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extsel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Ovsr {
        #[doc = "2x"]
        MUL2 = 0x0,
        #[doc = "4x"]
        MUL4 = 0x01,
        #[doc = "8x"]
        MUL8 = 0x02,
        #[doc = "16x"]
        MUL16 = 0x03,
        #[doc = "32x"]
        MUL32 = 0x04,
        #[doc = "64x"]
        MUL64 = 0x05,
        #[doc = "128x"]
        MUL128 = 0x06,
        #[doc = "256x"]
        MUL256 = 0x07,
    }
    impl Ovsr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ovsr {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ovsr {
        #[inline(always)]
        fn from(val: u8) -> Ovsr {
            Ovsr::from_bits(val)
        }
    }
    impl From<Ovsr> for u8 {
        #[inline(always)]
        fn from(val: Ovsr) -> u8 {
            Ovsr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ovss {
        #[doc = "No shift"]
        NO_SHIFT = 0x0,
        #[doc = "Shift 1-bit"]
        SHIFT1 = 0x01,
        #[doc = "Shift 2-bits"]
        SHIFT2 = 0x02,
        #[doc = "Shift 3-bits"]
        SHIFT3 = 0x03,
        #[doc = "Shift 4-bits"]
        SHIFT4 = 0x04,
        #[doc = "Shift 5-bits"]
        SHIFT5 = 0x05,
        #[doc = "Shift 6-bits"]
        SHIFT6 = 0x06,
        #[doc = "Shift 7-bits"]
        SHIFT7 = 0x07,
        #[doc = "Shift 8-bits"]
        SHIFT8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Ovss {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ovss {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ovss {
        #[inline(always)]
        fn from(val: u8) -> Ovss {
            Ovss::from_bits(val)
        }
    }
    impl From<Ovss> for u8 {
        #[inline(always)]
        fn from(val: Ovss) -> u8 {
            Ovss::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Presc {
        #[doc = "Input ADC clock not divided"]
        DIV1 = 0x0,
        #[doc = "Input ADC clock divided by 2"]
        DIV2 = 0x01,
        #[doc = "Input ADC clock divided by 4"]
        DIV4 = 0x02,
        #[doc = "Input ADC clock divided by 6"]
        DIV6 = 0x03,
        #[doc = "Input ADC clock divided by 8"]
        DIV8 = 0x04,
        #[doc = "Input ADC clock divided by 10"]
        DIV10 = 0x05,
        #[doc = "Input ADC clock divided by 12"]
        DIV12 = 0x06,
        #[doc = "Input ADC clock divided by 16"]
        DIV16 = 0x07,
        #[doc = "Input ADC clock divided by 32"]
        DIV32 = 0x08,
        #[doc = "Input ADC clock divided by 64"]
        DIV64 = 0x09,
        #[doc = "Input ADC clock divided by 128"]
        DIV128 = 0x0a,
        #[doc = "Input ADC clock divided by 256"]
        DIV256 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Presc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Presc {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Presc {
        #[inline(always)]
        fn from(val: u8) -> Presc {
            Presc::from_bits(val)
        }
    }
    impl From<Presc> for u8 {
        #[inline(always)]
        fn from(val: Presc) -> u8 {
            Presc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Res {
        #[doc = "12 bits"]
        BITS12 = 0x0,
        #[doc = "10 bits"]
        BITS10 = 0x01,
        #[doc = "8 bits"]
        BITS8 = 0x02,
        #[doc = "6 bits"]
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
        #[doc = "3.5 ADC clock cycles"]
        CYCLES3_5 = 0x01,
        #[doc = "7.5 ADC clock cycles"]
        CYCLES7_5 = 0x02,
        #[doc = "12.5 ADC clock cycles"]
        CYCLES12_5 = 0x03,
        #[doc = "19.5 ADC clock cycles"]
        CYCLES19_5 = 0x04,
        #[doc = "39.5 ADC clock cycles"]
        CYCLES39_5 = 0x05,
        #[doc = "79.5 ADC clock cycles"]
        CYCLES79_5 = 0x06,
        #[doc = "160.5 ADC clock cycles"]
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scandir {
        #[doc = "Upward scan (from CHSEL0 to CHSEL17)"]
        UPWARD = 0x0,
        #[doc = "Backward scan (from CHSEL17 to CHSEL0)"]
        BACKWARD = 0x01,
    }
    impl Scandir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scandir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scandir {
        #[inline(always)]
        fn from(val: u8) -> Scandir {
            Scandir::from_bits(val)
        }
    }
    impl From<Scandir> for u8 {
        #[inline(always)]
        fn from(val: Scandir) -> u8 {
            Scandir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpsel {
        #[doc = "Sampling time of CHANNELx use the setting of SMP1 register"]
        SMP1 = 0x0,
        #[doc = "Sampling time of CHANNELx use the setting of SMP2 register"]
        SMP2 = 0x01,
    }
    impl Smpsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsel {
        #[inline(always)]
        fn from(val: u8) -> Smpsel {
            Smpsel::from_bits(val)
        }
    }
    impl From<Smpsel> for u8 {
        #[inline(always)]
        fn from(val: Smpsel) -> u8 {
            Smpsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sq {
        #[doc = "Channel 0 selected for the Nth conversion"]
        CH0 = 0x0,
        #[doc = "Channel 1 selected for the Nth conversion"]
        CH1 = 0x01,
        #[doc = "Channel 2 selected for the Nth conversion"]
        CH2 = 0x02,
        #[doc = "Channel 3 selected for the Nth conversion"]
        CH3 = 0x03,
        #[doc = "Channel 4 selected for the Nth conversion"]
        CH4 = 0x04,
        #[doc = "Channel 5 selected for the Nth conversion"]
        CH5 = 0x05,
        #[doc = "Channel 6 selected for the Nth conversion"]
        CH6 = 0x06,
        #[doc = "Channel 7 selected for the Nth conversion"]
        CH7 = 0x07,
        #[doc = "Channel 8 selected for the Nth conversion"]
        CH8 = 0x08,
        #[doc = "Channel 9 selected for the Nth conversion"]
        CH9 = 0x09,
        #[doc = "Channel 10 selected for the Nth conversion"]
        CH10 = 0x0a,
        #[doc = "Channel 11 selected for the Nth conversion"]
        CH11 = 0x0b,
        #[doc = "Channel 12 selected for the Nth conversion"]
        CH12 = 0x0c,
        #[doc = "Channel 13 selected for the Nth conversion"]
        CH13 = 0x0d,
        #[doc = "Channel 14 selected for the Nth conversion"]
        CH14 = 0x0e,
        #[doc = "End of sequence"]
        EOS = 0x0f,
    }
    impl Sq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sq {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sq {
        #[inline(always)]
        fn from(val: u8) -> Sq {
            Sq::from_bits(val)
        }
    }
    impl From<Sq> for u8 {
        #[inline(always)]
        fn from(val: Sq) -> u8 {
            Sq::to_bits(val)
        }
    }
}
