#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ADC register block."]
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
    #[doc = "ADC configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ADC configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "ADC sampling time register."]
    #[inline(always)]
    pub const fn smpr(self) -> crate::common::Reg<regs::Smpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "ADC watchdog threshold register."]
    #[inline(always)]
    pub const fn awd1tr(self) -> crate::common::Reg<regs::Awd1tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "ADC watchdog threshold register."]
    #[inline(always)]
    pub const fn awd2tr(self) -> crate::common::Reg<regs::Awd2tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "ADC channel selection register \\[alternate\\]."]
    #[inline(always)]
    pub const fn chselr(self) -> crate::common::Reg<regs::Chselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "ADC channel selection register \\[alternate\\]."]
    #[inline(always)]
    pub const fn chselr_alternate(self) -> crate::common::Reg<regs::ChselrAlternate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "ADC watchdog threshold register."]
    #[inline(always)]
    pub const fn awd3tr(self) -> crate::common::Reg<regs::Awd3tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "ADC data register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "ADC Power register."]
    #[inline(always)]
    pub const fn pwr(self) -> crate::common::Reg<regs::Pwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "ADC Analog Watchdog 2 Configuration register."]
    #[inline(always)]
    pub const fn awd2cr(self) -> crate::common::Reg<regs::Awd2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "ADC Analog Watchdog 3 Configuration register."]
    #[inline(always)]
    pub const fn awd3cr(self) -> crate::common::Reg<regs::Awd3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "ADC Calibration factor."]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "ADC common configuration register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC watchdog threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd1tr(pub u32);
    impl Awd1tr {
        #[doc = "Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub const fn lt1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub fn set_lt1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub const fn ht1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
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
    #[doc = "ADC Analog Watchdog 2 Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2cr(pub u32);
    impl Awd2cr {
        #[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn awd2ch(&self, n: usize) -> bool {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_awd2ch(&mut self, n: usize, val: bool) {
            assert!(n < 14usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Awd2cr {{ awd2ch[0]: {=bool:?}, awd2ch[1]: {=bool:?}, awd2ch[2]: {=bool:?}, awd2ch[3]: {=bool:?}, awd2ch[4]: {=bool:?}, awd2ch[5]: {=bool:?}, awd2ch[6]: {=bool:?}, awd2ch[7]: {=bool:?}, awd2ch[8]: {=bool:?}, awd2ch[9]: {=bool:?}, awd2ch[10]: {=bool:?}, awd2ch[11]: {=bool:?}, awd2ch[12]: {=bool:?}, awd2ch[13]: {=bool:?} }}" , self . awd2ch (0usize) , self . awd2ch (1usize) , self . awd2ch (2usize) , self . awd2ch (3usize) , self . awd2ch (4usize) , self . awd2ch (5usize) , self . awd2ch (6usize) , self . awd2ch (7usize) , self . awd2ch (8usize) , self . awd2ch (9usize) , self . awd2ch (10usize) , self . awd2ch (11usize) , self . awd2ch (12usize) , self . awd2ch (13usize))
        }
    }
    #[doc = "ADC watchdog threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2tr(pub u32);
    impl Awd2tr {
        #[doc = "Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub const fn lt2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub fn set_lt2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub const fn ht2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
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
    #[doc = "ADC Analog Watchdog 3 Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3cr(pub u32);
    impl Awd3cr {
        #[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn awd3ch(&self, n: usize) -> bool {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\\[3:0\\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_awd3ch(&mut self, n: usize, val: bool) {
            assert!(n < 14usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Awd3cr {{ awd3ch[0]: {=bool:?}, awd3ch[1]: {=bool:?}, awd3ch[2]: {=bool:?}, awd3ch[3]: {=bool:?}, awd3ch[4]: {=bool:?}, awd3ch[5]: {=bool:?}, awd3ch[6]: {=bool:?}, awd3ch[7]: {=bool:?}, awd3ch[8]: {=bool:?}, awd3ch[9]: {=bool:?}, awd3ch[10]: {=bool:?}, awd3ch[11]: {=bool:?}, awd3ch[12]: {=bool:?}, awd3ch[13]: {=bool:?} }}" , self . awd3ch (0usize) , self . awd3ch (1usize) , self . awd3ch (2usize) , self . awd3ch (3usize) , self . awd3ch (4usize) , self . awd3ch (5usize) , self . awd3ch (6usize) , self . awd3ch (7usize) , self . awd3ch (8usize) , self . awd3ch (9usize) , self . awd3ch (10usize) , self . awd3ch (11usize) , self . awd3ch (12usize) , self . awd3ch (13usize))
        }
    }
    #[doc = "ADC watchdog threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3tr(pub u32);
    impl Awd3tr {
        #[doc = "Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub const fn lt3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub fn set_lt3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
        #[inline(always)]
        pub const fn ht3(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638."]
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
    #[doc = "ADC Calibration factor."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact(pub u32);
    impl Calfact {
        #[doc = "Calibration factor These bits are written by hardware or by software. Once a calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new calibration is launched. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor. Note: Software can write these bits only when ADEN = 1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
        #[inline(always)]
        pub const fn calfact(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Calibration factor These bits are written by hardware or by software. Once a calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new calibration is launched. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor. Note: Software can write these bits only when ADEN = 1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
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
    #[doc = "ADC common configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn presc(&self) -> super::vals::Presc {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Presc::from_bits(val as u8)
        }
        #[doc = "ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_presc(&mut self, val: super::vals::Presc) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "VsubREFINT/sub enable This bit is set and cleared by software to enable/disable the VsubREFINT/sub buffer. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VsubREFINT/sub enable This bit is set and cleared by software to enable/disable the VsubREFINT/sub buffer. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature sensor selection This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn vsensesel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor selection This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_vsensesel(&mut self, val: bool) {
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
                .field("presc", &self.presc())
                .field("vrefen", &self.vrefen())
                .field("vsensesel", &self.vsensesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ presc: {:?}, vrefen: {=bool:?}, vsensesel: {=bool:?} }}",
                self.presc(),
                self.vrefen(),
                self.vsensesel()
            )
        }
    }
    #[doc = "ADC configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the automatic management of the converted data by the DMA controller. For more details, refer to Section : Managing converted data using the DMA on page 632. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the automatic management of the converted data by the DMA controller. For more details, refer to Section : Managing converted data using the DMA on page 632. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section : Managing converted data using the DMA on page 632 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn dmacfg(&self) -> super::vals::Dmacfg {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Dmacfg::from_bits(val as u8)
        }
        #[doc = "Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section : Managing converted data using the DMA on page 632 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_dmacfg(&mut self, val: super::vals::Dmacfg) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELRMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn scandir(&self) -> super::vals::Scandir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Scandir::from_bits(val as u8)
        }
        #[doc = "Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELRMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_scandir(&mut self, val: super::vals::Scandir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure 78: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 631 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn align(&self) -> super::vals::Align {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Align::from_bits(val as u8)
        }
        #[doc = "Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure 78: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 631 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_align(&mut self, val: super::vals::Align) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "External trigger selection These bits select the external event used to trigger the start of conversion (refer to table ADC interconnection in Section 20.4.2: ADC pins and internal signals for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn extsel(&self) -> super::vals::Extsel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Extsel::from_bits(val as u8)
        }
        #[doc = "External trigger selection These bits select the external event used to trigger the start of conversion (refer to table ADC interconnection in Section 20.4.2: ADC pins and internal signals for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: super::vals::Extsel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Exten {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_exten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn ovrmod(&self) -> super::vals::Ovrmod {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Ovrmod::from_bits(val as u8)
        }
        #[doc = "Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_ovrmod(&mut self, val: super::vals::Ovrmod) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn cont(&self) -> super::vals::Cont {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Cont::from_bits(val as u8)
        }
        #[doc = "Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_cont(&mut self, val: super::vals::Cont) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.sup./sup Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn wait(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.sup./sup Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_wait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Mode selection of the CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn chselrmod(&self) -> super::vals::Chselrmod {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Chselrmod::from_bits(val as u8)
        }
        #[doc = "Mode selection of the CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_chselrmod(&mut self, val: super::vals::Chselrmod) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> super::vals::Awd1sgl {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Awd1sgl::from_bits(val as u8)
        }
        #[doc = "Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_awd1sgl(&mut self, val: super::vals::Awd1sgl) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn awd1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
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
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ dmaen: {=bool:?}, dmacfg: {:?}, res: {:?}, scandir: {:?}, align: {:?}, extsel: {:?}, exten: {:?}, ovrmod: {:?}, cont: {:?}, wait: {=bool:?}, discen: {=bool:?}, chselrmod: {:?}, awd1sgl: {:?}, awd1en: {=bool:?}, awd1ch: {=u8:?} }}" , self . dmaen () , self . dmacfg () , self . res () , self . scandir () , self . align () , self . extsel () , self . exten () , self . ovrmod () , self . cont () , self . wait () , self . discen () , self . chselrmod () , self . awd1sgl () , self . awd1en () , self . awd1ch ())
        }
    }
    #[doc = "ADC configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn ovse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_ovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn ovsr(&self) -> super::vals::OversamplingRatio {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::OversamplingRatio::from_bits(val as u8)
        }
        #[doc = "Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_ovsr(&mut self, val: super::vals::OversamplingRatio) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
        }
        #[doc = "Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1(which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn ovss(&self) -> super::vals::Ovss {
            let val = (self.0 >> 5usize) & 0x0f;
            super::vals::Ovss::from_bits(val as u8)
        }
        #[doc = "Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1(which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_ovss(&mut self, val: super::vals::Ovss) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
        }
        #[doc = "Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn tovs(&self) -> super::vals::Tovs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Tovs::from_bits(val as u8)
        }
        #[doc = "Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_tovs(&mut self, val: super::vals::Tovs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Low frequency trigger mode enable This bit must be set by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn lftrig(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Low frequency trigger mode enable This bit must be set by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_lftrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ ovse: {=bool:?}, ovsr: {:?}, ovss: {:?}, tovs: {:?}, lftrig: {=bool:?} }}",
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
    pub struct Chselr(pub u32);
    impl Chselr {
        #[doc = "Channel x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn chsel0(&self, n: usize) -> bool {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_chsel0(&mut self, n: usize, val: bool) {
            assert!(n < 14usize);
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
                .field("chsel0[0]", &self.chsel0(0usize))
                .field("chsel0[1]", &self.chsel0(1usize))
                .field("chsel0[2]", &self.chsel0(2usize))
                .field("chsel0[3]", &self.chsel0(3usize))
                .field("chsel0[4]", &self.chsel0(4usize))
                .field("chsel0[5]", &self.chsel0(5usize))
                .field("chsel0[6]", &self.chsel0(6usize))
                .field("chsel0[7]", &self.chsel0(7usize))
                .field("chsel0[8]", &self.chsel0(8usize))
                .field("chsel0[9]", &self.chsel0(9usize))
                .field("chsel0[10]", &self.chsel0(10usize))
                .field("chsel0[11]", &self.chsel0(11usize))
                .field("chsel0[12]", &self.chsel0(12usize))
                .field("chsel0[13]", &self.chsel0(13usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chselr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Chselr {{ chsel0[0]: {=bool:?}, chsel0[1]: {=bool:?}, chsel0[2]: {=bool:?}, chsel0[3]: {=bool:?}, chsel0[4]: {=bool:?}, chsel0[5]: {=bool:?}, chsel0[6]: {=bool:?}, chsel0[7]: {=bool:?}, chsel0[8]: {=bool:?}, chsel0[9]: {=bool:?}, chsel0[10]: {=bool:?}, chsel0[11]: {=bool:?}, chsel0[12]: {=bool:?}, chsel0[13]: {=bool:?} }}" , self . chsel0 (0usize) , self . chsel0 (1usize) , self . chsel0 (2usize) , self . chsel0 (3usize) , self . chsel0 (4usize) , self . chsel0 (5usize) , self . chsel0 (6usize) , self . chsel0 (7usize) , self . chsel0 (8usize) , self . chsel0 (9usize) , self . chsel0 (10usize) , self . chsel0 (11usize) , self . chsel0 (12usize) , self . chsel0 (13usize))
        }
    }
    #[doc = "ADC channel selection register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChselrAlternate(pub u32);
    impl ChselrAlternate {
        #[doc = "1st conversion of the sequence These bits are programmed by software with the channel number assigned to the 1st conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "1st conversion of the sequence These bits are programmed by software with the channel number assigned to the 1st conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\\[3:0\\]
for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for ChselrAlternate {
        #[inline(always)]
        fn default() -> ChselrAlternate {
            ChselrAlternate(0)
        }
    }
    impl core::fmt::Debug for ChselrAlternate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChselrAlternate")
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
    impl defmt::Format for ChselrAlternate {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ChselrAlternate {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?}, sq[5]: {=u8:?}, sq[6]: {=u8:?}, sq[7]: {=u8:?} }}" , self . sq (0usize) , self . sq (1usize) , self . sq (2usize) , self . sq (3usize) , self . sq (4usize) , self . sq (5usize) , self . sq (6usize) , self . sq (7usize))
        }
    }
    #[doc = "ADC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of CR registers are 0 (ADCAL = 0, ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of CR registers are 0 (ADCAL = 0, ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: Setting ADDIS to 1 is only effective when ADEN = 1 and ADSTART = 0 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: Setting ADDIS to 1 is only effective when ADEN = 1 and ADSTART = 0 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \\[1:0\\]
configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONT=0, DISCEN = 1), when the software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC)."]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \\[1:0\\]
configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONT=0, DISCEN = 1), when the software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC)."]
        #[inline(always)]
        pub fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC stop conversion command This bit is set by software to stop and discard an ongoing conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to accept a new start conversion command. Note: To clear the A/D converter state, ADSTP must be set to 1 even if ADSTART is cleared to 0 after the software trigger A/D conversion. It is recommended to set ADSTP to 1 whenever the configuration needs to be modified."]
        #[inline(always)]
        pub const fn adstp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC stop conversion command This bit is set by software to stop and discard an ongoing conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to accept a new start conversion command. Note: To clear the A/D converter state, ADSTP must be set to 1 even if ADSTART is cleared to 0 after the software trigger A/D conversion. It is recommended to set ADSTP to 1 whenever the configuration needs to be modified."]
        #[inline(always)]
        pub fn set_adstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC voltage regulator enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after tsubADCVREG_SETUP/sub. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub const fn advregen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC voltage regulator enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after tsubADCVREG_SETUP/sub. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
        #[inline(always)]
        pub fn set_advregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0). Note: The software is allowed to update the calibration factor by writing CALFACT only when ADEN is set to 1 and ADSTART is cleared to 0 by writing ADSTP to 1 (ADC enabled and no conversion is ongoing)."]
        #[inline(always)]
        pub const fn adcal(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0). Note: The software is allowed to update the calibration factor by writing CALFACT only when ADEN is set to 1 and ADSTART is cleared to 0 by writing ADSTP to 1 (ADC enabled and no conversion is ongoing)."]
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
    #[doc = "ADC data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in Figure 78: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 631. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in Figure 78: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 631. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
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
    #[doc = "ADC interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn awdie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_awdie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn eocalie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_eocalie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LDO ready interrupt enable This bit is set and cleared by software. It is used to enable/disable the LDORDY interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensure that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn ldordyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LDO ready interrupt enable This bit is set and cleared by software. It is used to enable/disable the LDORDY interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensure that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_ldordyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("awdie[0]", &self.awdie(0usize))
                .field("awdie[1]", &self.awdie(1usize))
                .field("awdie[2]", &self.awdie(2usize))
                .field("eocalie", &self.eocalie())
                .field("ldordyie", &self.ldordyie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ adrdyie: {=bool:?}, eosmpie: {=bool:?}, eocie: {=bool:?}, eosie: {=bool:?}, ovrie: {=bool:?}, awdie[0]: {=bool:?}, awdie[1]: {=bool:?}, awdie[2]: {=bool:?}, eocalie: {=bool:?}, ldordyie: {=bool:?} }}" , self . adrdyie () , self . eosmpie () , self . eocie () , self . eosie () , self . ovrie () , self . awdie (0usize) , self . awdie (1usize) , self . awdie (2usize) , self . eocalie () , self . ldordyie ())
        }
    }
    #[doc = "ADC interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
        #[inline(always)]
        pub fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
        #[inline(always)]
        pub fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in TR1 and ADC_HR1 registers. It is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub const fn awd(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in TR1 and ADC_HR1 registers. It is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub fn set_awd(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "End of calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
        #[inline(always)]
        pub const fn eocal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "End of calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
        #[inline(always)]
        pub fn set_eocal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LDO ready This bit is set by hardware. It indicates that the ADC internal LDO output is ready. It is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub const fn ldordy(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LDO ready This bit is set by hardware. It indicates that the ADC internal LDO output is ready. It is cleared by software by writing 1 to it."]
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
                .field("awd[0]", &self.awd(0usize))
                .field("awd[1]", &self.awd(1usize))
                .field("awd[2]", &self.awd(2usize))
                .field("eocal", &self.eocal())
                .field("ldordy", &self.ldordy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ adrdy: {=bool:?}, eosmp: {=bool:?}, eoc: {=bool:?}, eos: {=bool:?}, ovr: {=bool:?}, awd[0]: {=bool:?}, awd[1]: {=bool:?}, awd[2]: {=bool:?}, eocal: {=bool:?}, ldordy: {=bool:?} }}" , self . adrdy () , self . eosmp () , self . eoc () , self . eos () , self . ovr () , self . awd (0usize) , self . awd (1usize) , self . awd (2usize) , self . eocal () , self . ldordy ())
        }
    }
    #[doc = "ADC Power register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwr(pub u32);
    impl Pwr {
        #[doc = "Auto-off mode bit This bit is set and cleared by software. it is used to enable/disable the Auto-off mode. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn autoff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-off mode bit This bit is set and cleared by software. it is used to enable/disable the Auto-off mode. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_autoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Deep-power-down mode bit This bit is set and cleared by software. It is used to enable/disable Deep-power-down mode in Autonomous mode when the ADC is not used. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing). Note: Setting DPD in Auto-off mode automatically disables the LDO."]
        #[inline(always)]
        pub const fn dpd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Deep-power-down mode bit This bit is set and cleared by software. It is used to enable/disable Deep-power-down mode in Autonomous mode when the ADC is not used. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing). Note: Setting DPD in Auto-off mode automatically disables the LDO."]
        #[inline(always)]
        pub fn set_dpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Pwr {
        #[inline(always)]
        fn default() -> Pwr {
            Pwr(0)
        }
    }
    impl core::fmt::Debug for Pwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwr")
                .field("autoff", &self.autoff())
                .field("dpd", &self.dpd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pwr {{ autoff: {=bool:?}, dpd: {=bool:?} }}",
                self.autoff(),
                self.dpd()
            )
        }
    }
    #[doc = "ADC sampling time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr(pub u32);
    impl Smpr {
        #[doc = "Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub const fn smpsel(&self, n: usize) -> super::vals::Smpsel {
            assert!(n < 14usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Smpsel::from_bits(val as u8)
        }
        #[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing)."]
        #[inline(always)]
        pub fn set_smpsel(&mut self, n: usize, val: super::vals::Smpsel) {
            assert!(n < 14usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smpr {{ smp[0]: {:?}, smp[1]: {:?}, smpsel[0]: {:?}, smpsel[1]: {:?}, smpsel[2]: {:?}, smpsel[3]: {:?}, smpsel[4]: {:?}, smpsel[5]: {:?}, smpsel[6]: {:?}, smpsel[7]: {:?}, smpsel[8]: {:?}, smpsel[9]: {:?}, smpsel[10]: {:?}, smpsel[11]: {:?}, smpsel[12]: {:?}, smpsel[13]: {:?} }}" , self . smp (0usize) , self . smp (1usize) , self . smpsel (0usize) , self . smpsel (1usize) , self . smpsel (2usize) , self . smpsel (3usize) , self . smpsel (4usize) , self . smpsel (5usize) , self . smpsel (6usize) , self . smpsel (7usize) , self . smpsel (8usize) , self . smpsel (9usize) , self . smpsel (10usize) , self . smpsel (11usize) , self . smpsel (12usize) , self . smpsel (13usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Align {
        #[doc = "Right alignment."]
        RIGHT = 0x0,
        #[doc = "Left alignment."]
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
        #[doc = "Analog watchdog 1 enabled on all channels."]
        ALL_CHANNELS = 0x0,
        #[doc = "Analog watchdog 1 enabled on a single channel."]
        SINGLE_CHANNEL = 0x01,
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
    pub enum Chselrmod {
        #[doc = "Each bit of the CHSELR register enables an input."]
        ENABLE_INPUT = 0x0,
        #[doc = "CHSELR register is able to sequence up to 8 channels."]
        SEQUENCE_EIGHT_CHANNELS = 0x01,
    }
    impl Chselrmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chselrmod {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chselrmod {
        #[inline(always)]
        fn from(val: u8) -> Chselrmod {
            Chselrmod::from_bits(val)
        }
    }
    impl From<Chselrmod> for u8 {
        #[inline(always)]
        fn from(val: Chselrmod) -> u8 {
            Chselrmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cont {
        #[doc = "Single conversion mode."]
        SINGLE = 0x0,
        #[doc = "Continuous conversion mode."]
        CONTINUOUS = 0x01,
    }
    impl Cont {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cont {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cont {
        #[inline(always)]
        fn from(val: u8) -> Cont {
            Cont::from_bits(val)
        }
    }
    impl From<Cont> for u8 {
        #[inline(always)]
        fn from(val: Cont) -> u8 {
            Cont::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dmacfg {
        #[doc = "DMA One Shot mode selected."]
        ONE_SHOT = 0x0,
        #[doc = "DMA Circular mode selected."]
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
    pub enum Extsel {
        #[doc = "adc_trg0."]
        TRG0 = 0x0,
        #[doc = "adc_trg1."]
        TRG1 = 0x01,
        #[doc = "adc_trg2."]
        TRG2 = 0x02,
        #[doc = "adc_trg3."]
        TRG3 = 0x03,
        #[doc = "adc_trg4."]
        TRG4 = 0x04,
        #[doc = "adc_trg5."]
        TRG5 = 0x05,
        #[doc = "adc_trg6."]
        TRG6 = 0x06,
        #[doc = "adc_trg7."]
        TRG7 = 0x07,
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
    pub enum OversamplingRatio {
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
    impl OversamplingRatio {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OversamplingRatio {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OversamplingRatio {
        #[inline(always)]
        fn from(val: u8) -> OversamplingRatio {
            OversamplingRatio::from_bits(val)
        }
    }
    impl From<OversamplingRatio> for u8 {
        #[inline(always)]
        fn from(val: OversamplingRatio) -> u8 {
            OversamplingRatio::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ovrmod {
        #[doc = "DR register is preserved with the old data when an overrun is detected."]
        PRESERVE = 0x0,
        #[doc = "DR register is overwritten with the last conversion result when an overrun is detected."]
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
    pub enum Ovss {
        #[doc = "No shift."]
        SHIFT0 = 0x0,
        #[doc = "Shift 1-bit."]
        SHIFT1 = 0x01,
        #[doc = "Shift 2-bits."]
        SHIFT2 = 0x02,
        #[doc = "Shift 3-bits."]
        SHIFT3 = 0x03,
        #[doc = "Shift 4-bits."]
        SHIFT4 = 0x04,
        #[doc = "Shift 5-bits."]
        SHIFT5 = 0x05,
        #[doc = "Shift 6-bits."]
        SHIFT6 = 0x06,
        #[doc = "Shift 7-bits."]
        SHIFT7 = 0x07,
        #[doc = "Shift 8-bits."]
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
        #[doc = "input ADC clock not divided."]
        DIV1 = 0x0,
        #[doc = "input ADC clock divided by 2."]
        DIV2 = 0x01,
        #[doc = "input ADC clock divided by 4."]
        DIV4 = 0x02,
        #[doc = "input ADC clock divided by 6."]
        DIV6 = 0x03,
        #[doc = "input ADC clock divided by 8."]
        DIV8 = 0x04,
        #[doc = "input ADC clock divided by 10."]
        DIV10 = 0x05,
        #[doc = "input ADC clock divided by 12."]
        DIV12 = 0x06,
        #[doc = "input ADC clock divided by 16."]
        DIV16 = 0x07,
        #[doc = "input ADC clock divided by 32."]
        DIV32 = 0x08,
        #[doc = "input ADC clock divided by 64."]
        DIV64 = 0x09,
        #[doc = "input ADC clock divided by 128."]
        DIV128 = 0x0a,
        #[doc = "input ADC clock divided by 256."]
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
        #[doc = "814.5 ADC cycles"]
        CYCLES814_5 = 0x07,
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
        #[doc = "Upward scan (from CHSEL0 to CHSEL11)."]
        UPWARDS = 0x0,
        #[doc = "Backward scan (from CHSEL11 to CHSEL0)."]
        BACKWARDS = 0x01,
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
        #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
        SMP1 = 0x0,
        #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
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
    pub enum Tovs {
        #[doc = "All oversampled conversions for a channel are done consecutively after a trigger."]
        ALL_AFTER_TRIGGER = 0x0,
        #[doc = "Each oversampled conversion for a channel needs a trigger."]
        EACH_AFTER_TRIGGER = 0x01,
    }
    impl Tovs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tovs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tovs {
        #[inline(always)]
        fn from(val: u8) -> Tovs {
            Tovs::from_bits(val)
        }
    }
    impl From<Tovs> for u8 {
        #[inline(always)]
        fn from(val: Tovs) -> u8 {
            Tovs::to_bits(val)
        }
    }
}
