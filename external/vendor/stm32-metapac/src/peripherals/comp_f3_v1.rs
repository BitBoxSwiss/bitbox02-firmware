#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General purpose comparators."]
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
    #[doc = "control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Comparator enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparator 1 non inverting input connection to DAC output. Only available on COMP1"]
        #[inline(always)]
        pub const fn inp_dac(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator 1 non inverting input connection to DAC output. Only available on COMP1"]
        #[inline(always)]
        pub fn set_inp_dac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Comparator mode."]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Comparator mode."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Comparator inverting input selection."]
        #[inline(always)]
        pub const fn insel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Comparator inverting input selection."]
        #[inline(always)]
        pub fn set_insel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Window mode enable. Only available on COMP2"]
        #[inline(always)]
        pub const fn wndwen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Window mode enable. Only available on COMP2"]
        #[inline(always)]
        pub fn set_wndwen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Comparator output selection."]
        #[inline(always)]
        pub const fn outsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Comparator output selection."]
        #[inline(always)]
        pub fn set_outsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Comparator output polarity."]
        #[inline(always)]
        pub const fn pol(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output polarity."]
        #[inline(always)]
        pub fn set_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Comparator hysteresis."]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hyst {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Hyst::from_bits(val as u8)
        }
        #[doc = "Comparator hysteresis."]
        #[inline(always)]
        pub fn set_hyst(&mut self, val: super::vals::Hyst) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Comparator output."]
        #[inline(always)]
        pub const fn out(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output."]
        #[inline(always)]
        pub fn set_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Comparator lock."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator lock."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("inp_dac", &self.inp_dac())
                .field("mode", &self.mode())
                .field("insel", &self.insel())
                .field("wndwen", &self.wndwen())
                .field("outsel", &self.outsel())
                .field("pol", &self.pol())
                .field("hyst", &self.hyst())
                .field("out", &self.out())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ en: {=bool:?}, inp_dac: {=bool:?}, mode: {:?}, insel: {=u8:?}, wndwen: {=bool:?}, outsel: {=u8:?}, pol: {=bool:?}, hyst: {:?}, out: {=bool:?}, lock: {=bool:?} }}" , self . en () , self . inp_dac () , self . mode () , self . insel () , self . wndwen () , self . outsel () , self . pol () , self . hyst () , self . out () , self . lock ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hyst {
        NONE = 0x0,
        #[doc = "Low hysteresis"]
        LOW = 0x01,
        #[doc = "Medium hysteresis"]
        MEDIUM = 0x02,
        #[doc = "High hysteresis"]
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
    pub enum Mode {
        #[doc = "High Speed mode"]
        HIGH_SPEED = 0x0,
        #[doc = "Medium Speed mode"]
        MEDIUM_SPEED = 0x01,
        #[doc = "Low Speed mode"]
        LOW_SPEED = 0x02,
        #[doc = "Very Low Speed mode"]
        VERY_LOW_SPEED = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
}
