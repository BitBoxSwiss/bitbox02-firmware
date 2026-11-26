#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparator v2. (RM0440 24)"]
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
        #[doc = "Comparator signal selector for inverting input INM. (RM0440 24.3.2 Table 197)"]
        #[inline(always)]
        pub const fn inmsel(&self) -> super::vals::Inm {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Inm::from_bits(val as u8)
        }
        #[doc = "Comparator signal selector for inverting input INM. (RM0440 24.3.2 Table 197)"]
        #[inline(always)]
        pub fn set_inmsel(&mut self, val: super::vals::Inm) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Comparator signal selector for non-inverting input INP. (RM0440 24.3.2 Table 196)"]
        #[inline(always)]
        pub const fn inpsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator signal selector for non-inverting input INP. (RM0440 24.3.2 Table 196)"]
        #[inline(always)]
        pub fn set_inpsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
        pub const fn hyst(&self) -> super::vals::Hysteresis {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Hysteresis::from_bits(val as u8)
        }
        #[doc = "Comparator hysteresis selector."]
        #[inline(always)]
        pub fn set_hyst(&mut self, val: super::vals::Hysteresis) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Comparator blanking source selector. (RM0440 24.3.6 Table 198)"]
        #[inline(always)]
        pub const fn blanksel(&self) -> super::vals::Blanking {
            let val = (self.0 >> 19usize) & 0x07;
            super::vals::Blanking::from_bits(val as u8)
        }
        #[doc = "Comparator blanking source selector. (RM0440 24.3.6 Table 198)"]
        #[inline(always)]
        pub fn set_blanksel(&mut self, val: super::vals::Blanking) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
        }
        #[doc = "Vrefint resistor bridge enable. (RM0440 24.6)"]
        #[inline(always)]
        pub const fn brgen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Vrefint resistor bridge enable. (RM0440 24.6)"]
        #[inline(always)]
        pub fn set_brgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Vrefint scaled input enable. (RM0440 24.6)"]
        #[inline(always)]
        pub const fn scalen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Vrefint scaled input enable. (RM0440 24.6)"]
        #[inline(always)]
        pub fn set_scalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Comparator output status. (READ ONLY)"]
        #[inline(always)]
        pub const fn value(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output status. (READ ONLY)"]
        #[inline(always)]
        pub fn set_value(&mut self, val: bool) {
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
                .field("polarity", &self.polarity())
                .field("hyst", &self.hyst())
                .field("blanksel", &self.blanksel())
                .field("brgen", &self.brgen())
                .field("scalen", &self.scalen())
                .field("value", &self.value())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ en: {=bool:?}, inmsel: {:?}, inpsel: {=bool:?}, polarity: {:?}, hyst: {:?}, blanksel: {:?}, brgen: {=bool:?}, scalen: {=bool:?}, value: {=bool:?}, lock: {=bool:?} }}" , self . en () , self . inmsel () , self . inpsel () , self . polarity () , self . hyst () , self . blanksel () , self . brgen () , self . scalen () , self . value () , self . lock ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Blanking {
        #[doc = "No blanking."]
        NO_BLANKING = 0x0,
        #[doc = "Check data sheet for blanking options"]
        BLANK1 = 0x01,
        #[doc = "Check data sheet for blanking options"]
        BLANK2 = 0x02,
        #[doc = "Check data sheet for blanking options"]
        BLANK3 = 0x03,
        #[doc = "Check data sheet for blanking options"]
        BLANK4 = 0x04,
        #[doc = "Check data sheet for blanking options"]
        BLANK5 = 0x05,
        #[doc = "Check data sheet for blanking options"]
        BLANK6 = 0x06,
        #[doc = "Check data sheet for blanking options"]
        BLANK7 = 0x07,
    }
    impl Blanking {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Blanking {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Hysteresis {
        NONE = 0x0,
        #[doc = "10mV hysteresis"]
        HYST10M = 0x01,
        #[doc = "20mV hysteresis"]
        HYST20M = 0x02,
        #[doc = "30mV hysteresis"]
        HYST30M = 0x03,
        #[doc = "40mV hysteresis"]
        HYST40M = 0x04,
        #[doc = "50mV hysteresis"]
        HYST50M = 0x05,
        #[doc = "60mV hysteresis"]
        HYST60M = 0x06,
        #[doc = "70mV hysteresis"]
        HYST70M = 0x07,
    }
    impl Hysteresis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hysteresis {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hysteresis {
        #[inline(always)]
        fn from(val: u8) -> Hysteresis {
            Hysteresis::from_bits(val)
        }
    }
    impl From<Hysteresis> for u8 {
        #[inline(always)]
        fn from(val: Hysteresis) -> u8 {
            Hysteresis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Inm {
        #[doc = "Inverting input set to 1/4 VRef"]
        QUARTER_VREF = 0x0,
        #[doc = "Inverting input set to 1/2 VRef"]
        HALF_VREF = 0x01,
        #[doc = "Inverting input set to 3/4 VRef"]
        THREE_QUARTER_VREF = 0x02,
        #[doc = "Inverting input set to VRef"]
        VREF = 0x03,
        #[doc = "Inverting input set to DAC output (RM0440 24.3.2 Table)"]
        DACA = 0x04,
        #[doc = "Inverting input set to DAC output (RM0440 24.3.2 Table)"]
        DACB = 0x05,
        #[doc = "Inverting input set to IO (RM0440 24.3.2 Table)"]
        INM1 = 0x06,
        #[doc = "Inverting input set to IO (RM0440 24.3.2 Table)"]
        INM2 = 0x07,
    }
    impl Inm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Inm {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Inm {
        #[inline(always)]
        fn from(val: u8) -> Inm {
            Inm::from_bits(val)
        }
    }
    impl From<Inm> for u8 {
        #[inline(always)]
        fn from(val: Inm) -> u8 {
            Inm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Polarity {
        #[doc = "Output is not inverted."]
        NOT_INVERTED = 0x0,
        #[doc = "Output is inverted."]
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
}
