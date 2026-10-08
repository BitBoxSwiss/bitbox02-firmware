#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Voltage reference buffer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrefbuf {
    ptr: *mut u8,
}
unsafe impl Send for Vrefbuf {}
unsafe impl Sync for Vrefbuf {}
impl Vrefbuf {
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
    #[doc = "calibration control register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "calibration control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Trimming code."]
        #[inline(always)]
        pub const fn trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Trimming code."]
        #[inline(always)]
        pub fn set_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
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
            f.debug_struct("Ccr").field("trim", &self.trim()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr {{ trim: {=u8:?} }}", self.trim())
        }
    }
    #[doc = "control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Voltage reference buffer mode enable."]
        #[inline(always)]
        pub const fn envr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage reference buffer mode enable."]
        #[inline(always)]
        pub fn set_envr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "High impedance mode."]
        #[inline(always)]
        pub const fn hiz(&self) -> super::vals::Hiz {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Hiz::from_bits(val as u8)
        }
        #[doc = "High impedance mode."]
        #[inline(always)]
        pub fn set_hiz(&mut self, val: super::vals::Hiz) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Voltage reference scale."]
        #[inline(always)]
        pub const fn vrs(&self) -> super::vals::Vrs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Vrs::from_bits(val as u8)
        }
        #[doc = "Voltage reference scale."]
        #[inline(always)]
        pub fn set_vrs(&mut self, val: super::vals::Vrs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Voltage reference buffer ready."]
        #[inline(always)]
        pub const fn vrr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage reference buffer ready."]
        #[inline(always)]
        pub fn set_vrr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("envr", &self.envr())
                .field("hiz", &self.hiz())
                .field("vrs", &self.vrs())
                .field("vrr", &self.vrr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ envr: {=bool:?}, hiz: {:?}, vrs: {:?}, vrr: {=bool:?} }}",
                self.envr(),
                self.hiz(),
                self.vrs(),
                self.vrr()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hiz {
        #[doc = "VREF+ pin is internally connected to the voltage reference buffer output."]
        CONNECTED = 0x0,
        #[doc = "VREF+ pin is high impedance."]
        HIGH_Z = 0x01,
    }
    impl Hiz {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hiz {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hiz {
        #[inline(always)]
        fn from(val: u8) -> Hiz {
            Hiz::from_bits(val)
        }
    }
    impl From<Hiz> for u8 {
        #[inline(always)]
        fn from(val: Hiz) -> u8 {
            Hiz::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vrs {
        #[doc = "Voltage reference set to around 2.048 V."]
        VREF0 = 0x0,
        #[doc = "Voltage reference set to around 2.5 V."]
        VREF1 = 0x01,
    }
    impl Vrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vrs {
        #[inline(always)]
        fn from(val: u8) -> Vrs {
            Vrs::from_bits(val)
        }
    }
    impl From<Vrs> for u8 {
        #[inline(always)]
        fn from(val: Vrs) -> u8 {
            Vrs::to_bits(val)
        }
    }
}
