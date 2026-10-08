#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "COMP1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp {
    ptr: *mut u8,
}
unsafe impl Send for Comp {}
unsafe impl Sync for Comp {}
impl Comp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Comparator status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Comparator interrupt clear flag register."]
    #[inline(always)]
    pub const fn icfr(self) -> crate::common::Reg<regs::Icfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Comparator option register."]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::Or, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Comparator configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Comparator configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Comparator configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "COMP channel 1 enable bit."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "COMP channel 1 enable bit."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Scaler bridge enable."]
        #[inline(always)]
        pub const fn brgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Scaler bridge enable."]
        #[inline(always)]
        pub fn set_brgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Voltage scaler enable bit."]
        #[inline(always)]
        pub const fn scalen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaler enable bit."]
        #[inline(always)]
        pub fn set_scalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "COMP channel 1 polarity selection bit."]
        #[inline(always)]
        pub const fn polarity(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "COMP channel 1 polarity selection bit."]
        #[inline(always)]
        pub fn set_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "COMP channel 1 interrupt enable."]
        #[inline(always)]
        pub const fn iten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "COMP channel 1 interrupt enable."]
        #[inline(always)]
        pub fn set_iten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "COMP channel 1 hysteresis selection bits."]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hyst {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Hyst::from_bits(val as u8)
        }
        #[doc = "COMP channel 1 hysteresis selection bits."]
        #[inline(always)]
        pub fn set_hyst(&mut self, val: super::vals::Hyst) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Power Mode of the COMP channel 1."]
        #[inline(always)]
        pub const fn pwrmode(&self) -> super::vals::Pwrmode {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Pwrmode::from_bits(val as u8)
        }
        #[doc = "Power Mode of the COMP channel 1."]
        #[inline(always)]
        pub fn set_pwrmode(&mut self, val: super::vals::Pwrmode) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "COMP channel 1 inverting input selection field."]
        #[inline(always)]
        pub const fn inmsel(&self) -> super::vals::Inmsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Inmsel::from_bits(val as u8)
        }
        #[doc = "COMP channel 1 inverting input selection field."]
        #[inline(always)]
        pub fn set_inmsel(&mut self, val: super::vals::Inmsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "COMP channel 1 non-inverting input selection bit."]
        #[inline(always)]
        pub const fn inpsel(&self) -> super::vals::Inpsel {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Inpsel::from_bits(val as u8)
        }
        #[doc = "COMP channel 1 non-inverting input selection bit."]
        #[inline(always)]
        pub fn set_inpsel(&mut self, val: super::vals::Inpsel) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "COMP channel 1 blanking source selection bits."]
        #[inline(always)]
        pub const fn blanking(&self) -> super::vals::Blanking {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Blanking::from_bits(val as u8)
        }
        #[doc = "COMP channel 1 blanking source selection bits."]
        #[inline(always)]
        pub fn set_blanking(&mut self, val: super::vals::Blanking) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Lock bit."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("en", &self.en())
                .field("brgen", &self.brgen())
                .field("scalen", &self.scalen())
                .field("polarity", &self.polarity())
                .field("iten", &self.iten())
                .field("hyst", &self.hyst())
                .field("pwrmode", &self.pwrmode())
                .field("inmsel", &self.inmsel())
                .field("inpsel", &self.inpsel())
                .field("blanking", &self.blanking())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ en: {=bool:?}, brgen: {=bool:?}, scalen: {=bool:?}, polarity: {=bool:?}, iten: {=bool:?}, hyst: {:?}, pwrmode: {:?}, inmsel: {:?}, inpsel: {:?}, blanking: {:?}, lock: {=bool:?} }}" , self . en () , self . brgen () , self . scalen () , self . polarity () , self . iten () , self . hyst () , self . pwrmode () , self . inmsel () , self . inpsel () , self . blanking () , self . lock ())
        }
    }
    #[doc = "Comparator configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "COMP channel 1 enable bit."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "COMP channel 1 enable bit."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Scaler bridge enable."]
        #[inline(always)]
        pub const fn brgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Scaler bridge enable."]
        #[inline(always)]
        pub fn set_brgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Voltage scaler enable bit."]
        #[inline(always)]
        pub const fn scalen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaler enable bit."]
        #[inline(always)]
        pub fn set_scalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "COMP channel 1 polarity selection bit."]
        #[inline(always)]
        pub const fn polarity(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "COMP channel 1 polarity selection bit."]
        #[inline(always)]
        pub fn set_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Window comparator mode selection bit."]
        #[inline(always)]
        pub const fn winmode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Window comparator mode selection bit."]
        #[inline(always)]
        pub fn set_winmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "COMP channel 1 interrupt enable."]
        #[inline(always)]
        pub const fn iten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "COMP channel 1 interrupt enable."]
        #[inline(always)]
        pub fn set_iten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "COMP channel 1 hysteresis selection bits."]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hyst {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Hyst::from_bits(val as u8)
        }
        #[doc = "COMP channel 1 hysteresis selection bits."]
        #[inline(always)]
        pub fn set_hyst(&mut self, val: super::vals::Hyst) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Power Mode of the COMP channel 1."]
        #[inline(always)]
        pub const fn pwrmode(&self) -> super::vals::Pwrmode {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Pwrmode::from_bits(val as u8)
        }
        #[doc = "Power Mode of the COMP channel 1."]
        #[inline(always)]
        pub fn set_pwrmode(&mut self, val: super::vals::Pwrmode) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "COMP channel 1 inverting input selection field."]
        #[inline(always)]
        pub const fn inmsel(&self) -> super::vals::Inmsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Inmsel::from_bits(val as u8)
        }
        #[doc = "COMP channel 1 inverting input selection field."]
        #[inline(always)]
        pub fn set_inmsel(&mut self, val: super::vals::Inmsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "COMP channel 1 non-inverting input selection bit."]
        #[inline(always)]
        pub const fn inpsel(&self) -> super::vals::Inpsel {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Inpsel::from_bits(val as u8)
        }
        #[doc = "COMP channel 1 non-inverting input selection bit."]
        #[inline(always)]
        pub fn set_inpsel(&mut self, val: super::vals::Inpsel) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "COMP channel 1 blanking source selection bits."]
        #[inline(always)]
        pub const fn blanking(&self) -> super::vals::Blanking {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Blanking::from_bits(val as u8)
        }
        #[doc = "COMP channel 1 blanking source selection bits."]
        #[inline(always)]
        pub fn set_blanking(&mut self, val: super::vals::Blanking) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Lock bit."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("en", &self.en())
                .field("brgen", &self.brgen())
                .field("scalen", &self.scalen())
                .field("polarity", &self.polarity())
                .field("winmode", &self.winmode())
                .field("iten", &self.iten())
                .field("hyst", &self.hyst())
                .field("pwrmode", &self.pwrmode())
                .field("inmsel", &self.inmsel())
                .field("inpsel", &self.inpsel())
                .field("blanking", &self.blanking())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr2 {{ en: {=bool:?}, brgen: {=bool:?}, scalen: {=bool:?}, polarity: {=bool:?}, winmode: {=bool:?}, iten: {=bool:?}, hyst: {:?}, pwrmode: {:?}, inmsel: {:?}, inpsel: {:?}, blanking: {:?}, lock: {=bool:?} }}" , self . en () , self . brgen () , self . scalen () , self . polarity () , self . winmode () , self . iten () , self . hyst () , self . pwrmode () , self . inmsel () , self . inpsel () , self . blanking () , self . lock ())
        }
    }
    #[doc = "Comparator interrupt clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icfr(pub u32);
    impl Icfr {
        #[doc = "Clear COMP channel 1 Interrupt Flag."]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear COMP channel 1 Interrupt Flag."]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Icfr {
        #[inline(always)]
        fn default() -> Icfr {
            Icfr(0)
        }
    }
    impl core::fmt::Debug for Icfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icfr")
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icfr {{ ccif[0]: {=bool:?}, ccif[1]: {=bool:?} }}",
                self.ccif(0usize),
                self.ccif(1usize)
            )
        }
    }
    #[doc = "Comparator option register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Or(pub u32);
    impl Or {
        #[doc = "Selection of source for alternate function of output ports."]
        #[inline(always)]
        pub const fn afop(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Selection of source for alternate function of output ports."]
        #[inline(always)]
        pub fn set_afop(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Or {
        #[inline(always)]
        fn default() -> Or {
            Or(0)
        }
    }
    impl core::fmt::Debug for Or {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Or").field("afop", &self.afop()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Or {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Or {{ afop: {=u16:?} }}", self.afop())
        }
    }
    #[doc = "Comparator status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "COMP channel 1 output status bit."]
        #[inline(always)]
        pub const fn cval(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "COMP channel 1 output status bit."]
        #[inline(always)]
        pub fn set_cval(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COMP channel 1 Interrupt Flag."]
        #[inline(always)]
        pub const fn cif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "COMP channel 1 Interrupt Flag."]
        #[inline(always)]
        pub fn set_cif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("cval[0]", &self.cval(0usize))
                .field("cval[1]", &self.cval(1usize))
                .field("cif[0]", &self.cif(0usize))
                .field("cif[1]", &self.cif(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ cval[0]: {=bool:?}, cval[1]: {=bool:?}, cif[0]: {=bool:?}, cif[1]: {=bool:?} }}",
                self.cval(0usize),
                self.cval(1usize),
                self.cif(0usize),
                self.cif(1usize)
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Blanking {
        NO_BLANKING = 0x0,
        TIM1OC5 = 0x01,
        TIM2OC3 = 0x02,
        TIM3OC3 = 0x03,
        TIM3OC4 = 0x04,
        TIM8OC5 = 0x05,
        TIM15OC1 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Blanking {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Blanking {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Blanking {
        #[inline(always)]
        fn from(val: u8) -> Blanking {
            Blanking::from_bits(val)
        }
    }
    impl From<Blanking> for u8 {
        #[inline(always)]
        fn from(val: Blanking) -> u8 {
            Blanking::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hyst {
        NONE = 0x0,
        LOW = 0x01,
        MEDIUM = 0x02,
        HIGH = 0x03,
    }
    impl Hyst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hyst {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hyst {
        #[inline(always)]
        fn from(val: u8) -> Hyst {
            Hyst::from_bits(val)
        }
    }
    impl From<Hyst> for u8 {
        #[inline(always)]
        fn from(val: Hyst) -> u8 {
            Hyst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Inmsel {
        VREF_1OVER4 = 0x0,
        VREF_1OVER2 = 0x01,
        VREF_3OVER4 = 0x02,
        VREF = 0x03,
        INM1 = 0x04,
        INM2 = 0x05,
        COMPX_INM1 = 0x06,
        COMPX_INM2 = 0x07,
    }
    impl Inmsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Inmsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Inmsel {
        #[inline(always)]
        fn from(val: u8) -> Inmsel {
            Inmsel::from_bits(val)
        }
    }
    impl From<Inmsel> for u8 {
        #[inline(always)]
        fn from(val: Inmsel) -> u8 {
            Inmsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Inpsel {
        INP1 = 0x0,
        INP2 = 0x01,
    }
    impl Inpsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Inpsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Inpsel {
        #[inline(always)]
        fn from(val: u8) -> Inpsel {
            Inpsel::from_bits(val)
        }
    }
    impl From<Inpsel> for u8 {
        #[inline(always)]
        fn from(val: Inpsel) -> u8 {
            Inpsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pwrmode {
        #[doc = "High speed / full power"]
        HIGH = 0x0,
        #[doc = "Medium speed / medium power"]
        MEDIUM = 0x01,
        #[doc = "Medium speed / medium power"]
        MEDIUM_EITHER = 0x02,
        #[doc = "Ultra low power / ultra-low-power"]
        LOW = 0x03,
    }
    impl Pwrmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pwrmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pwrmode {
        #[inline(always)]
        fn from(val: u8) -> Pwrmode {
            Pwrmode::from_bits(val)
        }
    }
    impl From<Pwrmode> for u8 {
        #[inline(always)]
        fn from(val: Pwrmode) -> u8 {
            Pwrmode::to_bits(val)
        }
    }
}
