#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparator v1. (RM0444 18)"]
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
    #[doc = "Comparator control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "Comparator control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "COMP enable bit."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "COMP enable bit."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparator signal selector for inverting input INM."]
        #[inline(always)]
        pub const fn inmsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Comparator signal selector for inverting input INM."]
        #[inline(always)]
        pub fn set_inmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Comparator signal selector for non-inverting input INP."]
        #[inline(always)]
        pub const fn inpsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Comparator signal selector for non-inverting input INP."]
        #[inline(always)]
        pub fn set_inpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Comparator non-inverting input selector for window mode."]
        #[inline(always)]
        pub const fn winmode(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator non-inverting input selector for window mode."]
        #[inline(always)]
        pub fn set_winmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Comparator output selector."]
        #[inline(always)]
        pub const fn winout(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output selector."]
        #[inline(always)]
        pub fn set_winout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Comparator polarity selector."]
        #[inline(always)]
        pub const fn polarity(&self) -> super::vals::Polarity {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Polarity::from_bits(val as u8)
        }
        #[doc = "Comparator polarity selector."]
        #[inline(always)]
        pub fn set_polarity(&mut self, val: super::vals::Polarity) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Comparator hysteresis selector."]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hyst {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Hyst::from_bits(val as u8)
        }
        #[doc = "Comparator hysteresis selector."]
        #[inline(always)]
        pub fn set_hyst(&mut self, val: super::vals::Hyst) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Comparator power mode selector."]
        #[inline(always)]
        pub const fn pwrmode(&self) -> super::vals::Pwrmode {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Pwrmode::from_bits(val as u8)
        }
        #[doc = "Comparator power mode selector."]
        #[inline(always)]
        pub fn set_pwrmode(&mut self, val: super::vals::Pwrmode) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Comparator blanking source selector."]
        #[inline(always)]
        pub const fn blanksel(&self) -> super::vals::Blanksel {
            let val = (self.0 >> 20usize) & 0x1f;
            super::vals::Blanksel::from_bits(val as u8)
        }
        #[doc = "Comparator blanking source selector."]
        #[inline(always)]
        pub fn set_blanksel(&mut self, val: super::vals::Blanksel) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val.to_bits() as u32) & 0x1f) << 20usize);
        }
        #[doc = "Comparator output status. (READ ONLY)"]
        #[inline(always)]
        pub const fn value_do_not_set(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output status. (READ ONLY)"]
        #[inline(always)]
        pub fn set_value_do_not_set(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "CSR register lock."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CSR register lock."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
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
                .field("en", &self.en())
                .field("inmsel", &self.inmsel())
                .field("inpsel", &self.inpsel())
                .field("winmode", &self.winmode())
                .field("winout", &self.winout())
                .field("polarity", &self.polarity())
                .field("hyst", &self.hyst())
                .field("pwrmode", &self.pwrmode())
                .field("blanksel", &self.blanksel())
                .field("value_do_not_set", &self.value_do_not_set())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ en: {=bool:?}, inmsel: {=u8:?}, inpsel: {=u8:?}, winmode: {=bool:?}, winout: {=bool:?}, polarity: {:?}, hyst: {:?}, pwrmode: {:?}, blanksel: {:?}, value_do_not_set: {=bool:?}, lock: {=bool:?} }}" , self . en () , self . inmsel () , self . inpsel () , self . winmode () , self . winout () , self . polarity () , self . hyst () , self . pwrmode () , self . blanksel () , self . value_do_not_set () , self . lock ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Blanksel {
        NONE = 0x0,
        #[doc = "TIM1 OC4"]
        TIM1OC4 = 0x01,
        #[doc = "TIM1 OC5"]
        TIM1OC5 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "TIM2 OC3"]
        TIM2OC3 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "TIM3 OC3"]
        TIM3OC3 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        #[doc = "TIM15 OC2"]
        TIM15OC2 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Blanksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Blanksel {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Blanksel {
        #[inline(always)]
        fn from(val: u8) -> Blanksel {
            Blanksel::from_bits(val)
        }
    }
    impl From<Blanksel> for u8 {
        #[inline(always)]
        fn from(val: Blanksel) -> u8 {
            Blanksel::to_bits(val)
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
    pub enum Polarity {
        NON_INVERTED = 0x0,
        INVERTED = 0x01,
    }
    impl Polarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Polarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Polarity {
        #[inline(always)]
        fn from(val: u8) -> Polarity {
            Polarity::from_bits(val)
        }
    }
    impl From<Polarity> for u8 {
        #[inline(always)]
        fn from(val: Polarity) -> u8 {
            Polarity::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pwrmode {
        HIGH_SPEED = 0x0,
        MEDIUM_SPEED = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
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
