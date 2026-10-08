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
    #[doc = "VREFBUF control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "VREFBUF calibration control register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "VREFBUF calibration control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
        #[inline(always)]
        pub const fn trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
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
    #[doc = "VREFBUF control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
        #[inline(always)]
        pub const fn envr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
        #[inline(always)]
        pub fn set_envr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration."]
        #[inline(always)]
        pub const fn hiz(&self) -> super::vals::Hiz {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Hiz::from_bits(val as u8)
        }
        #[doc = "High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration."]
        #[inline(always)]
        pub fn set_hiz(&mut self, val: super::vals::Hiz) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
        #[doc = "Voltage reference scale These bits select the value generated by the voltage reference buffer. VRS = 000: VREFBUF0 voltage selected. VRS = 001: VREFBUF1 voltage selected. VRS = 010: VREFBUF2 voltage selected. VRS = 011: VREFBUF3 voltage selected. Others: Reserved Note: Refer to the product datasheet for each VREFBUFx voltage setting value. The software can program this bitfield only when the VREFBUF is disabled (ENVR=0)."]
        #[inline(always)]
        pub const fn vrs(&self) -> super::vals::Vrs {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Vrs::from_bits(val as u8)
        }
        #[doc = "Voltage reference scale These bits select the value generated by the voltage reference buffer. VRS = 000: VREFBUF0 voltage selected. VRS = 001: VREFBUF1 voltage selected. VRS = 010: VREFBUF2 voltage selected. VRS = 011: VREFBUF3 voltage selected. Others: Reserved Note: Refer to the product datasheet for each VREFBUFx voltage setting value. The software can program this bitfield only when the VREFBUF is disabled (ENVR=0)."]
        #[inline(always)]
        pub fn set_vrs(&mut self, val: super::vals::Vrs) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
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
                .field("vrr", &self.vrr())
                .field("vrs", &self.vrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ envr: {=bool:?}, hiz: {:?}, vrr: {=bool:?}, vrs: {:?} }}",
                self.envr(),
                self.hiz(),
                self.vrr(),
                self.vrs()
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
        #[doc = "Voltage reference set to around 1.5 V."]
        VREF0 = 0x0,
        #[doc = "Voltage reference set to around 1.8 V."]
        VREF1 = 0x01,
        #[doc = "Voltage reference set to around 2.048 V."]
        VREF2 = 0x02,
        #[doc = "Voltage reference set to around 2.5 V (ADC, DAC are not compatible with this setting)."]
        VREF3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Vrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vrs {
            unsafe { core::mem::transmute(val & 0x07) }
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
