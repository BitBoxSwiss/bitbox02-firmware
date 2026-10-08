#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Operational amplifier"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp {
    ptr: *mut u8,
}
unsafe impl Send for Opamp {}
unsafe impl Sync for Opamp {}
impl Opamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control/status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Enable"]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable"]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force internal reference on VP (reserved for test)"]
        #[inline(always)]
        pub const fn force_vp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force internal reference on VP (reserved for test)"]
        #[inline(always)]
        pub fn set_force_vp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Non-inverting input selection"]
        #[inline(always)]
        pub const fn vp_sel(&self) -> super::vals::VpSel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::VpSel::from_bits(val as u8)
        }
        #[doc = "Non-inverting input selection"]
        #[inline(always)]
        pub fn set_vp_sel(&mut self, val: super::vals::VpSel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Inverting input selection"]
        #[inline(always)]
        pub const fn vm_sel(&self) -> super::vals::VmSel {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::VmSel::from_bits(val as u8)
        }
        #[doc = "Inverting input selection"]
        #[inline(always)]
        pub fn set_vm_sel(&mut self, val: super::vals::VmSel) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Timer controlled Mux mode enable"]
        #[inline(always)]
        pub const fn tcm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer controlled Mux mode enable"]
        #[inline(always)]
        pub fn set_tcm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Inverting input secondary selection"]
        #[inline(always)]
        pub const fn vms_sel(&self) -> super::vals::VmsSel {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::VmsSel::from_bits(val as u8)
        }
        #[doc = "Inverting input secondary selection"]
        #[inline(always)]
        pub fn set_vms_sel(&mut self, val: super::vals::VmsSel) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Non-inverting input secondary selection"]
        #[inline(always)]
        pub const fn vps_sel(&self) -> super::vals::VpsSel {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::VpsSel::from_bits(val as u8)
        }
        #[doc = "Non-inverting input secondary selection"]
        #[inline(always)]
        pub fn set_vps_sel(&mut self, val: super::vals::VpsSel) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
        #[doc = "Calibration mode enable"]
        #[inline(always)]
        pub const fn calon(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration mode enable"]
        #[inline(always)]
        pub fn set_calon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Calibration selection"]
        #[inline(always)]
        pub const fn calsel(&self) -> super::vals::Calsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Calsel::from_bits(val as u8)
        }
        #[doc = "Calibration selection"]
        #[inline(always)]
        pub fn set_calsel(&mut self, val: super::vals::Calsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Gain in PGA mode"]
        #[inline(always)]
        pub const fn pga_gain(&self) -> super::vals::PgaGain {
            let val = (self.0 >> 14usize) & 0x0f;
            super::vals::PgaGain::from_bits(val as u8)
        }
        #[doc = "Gain in PGA mode"]
        #[inline(always)]
        pub fn set_pga_gain(&mut self, val: super::vals::PgaGain) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
        }
        #[doc = "User trimming enable"]
        #[inline(always)]
        pub const fn usertrim(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "User trimming enable"]
        #[inline(always)]
        pub fn set_usertrim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Offset trimming value (PMOS)"]
        #[inline(always)]
        pub const fn trimoffsetp(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "Offset trimming value (PMOS)"]
        #[inline(always)]
        pub fn set_trimoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "Offset trimming value (NMOS)"]
        #[inline(always)]
        pub const fn trimoffsetn(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Offset trimming value (NMOS)"]
        #[inline(always)]
        pub fn set_trimoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "Output the internal reference voltage"]
        #[inline(always)]
        pub const fn tstref(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Output the internal reference voltage"]
        #[inline(always)]
        pub fn set_tstref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Calibration output"]
        #[inline(always)]
        pub const fn calout(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration output"]
        #[inline(always)]
        pub fn set_calout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "OPAMP lock"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP lock"]
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
                .field("opampen", &self.opampen())
                .field("force_vp", &self.force_vp())
                .field("vp_sel", &self.vp_sel())
                .field("vm_sel", &self.vm_sel())
                .field("tcm_en", &self.tcm_en())
                .field("vms_sel", &self.vms_sel())
                .field("vps_sel", &self.vps_sel())
                .field("calon", &self.calon())
                .field("calsel", &self.calsel())
                .field("pga_gain", &self.pga_gain())
                .field("usertrim", &self.usertrim())
                .field("trimoffsetp", &self.trimoffsetp())
                .field("trimoffsetn", &self.trimoffsetn())
                .field("tstref", &self.tstref())
                .field("calout", &self.calout())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ opampen: {=bool:?}, force_vp: {=bool:?}, vp_sel: {:?}, vm_sel: {:?}, tcm_en: {=bool:?}, vms_sel: {:?}, vps_sel: {:?}, calon: {=bool:?}, calsel: {:?}, pga_gain: {:?}, usertrim: {=bool:?}, trimoffsetp: {=u8:?}, trimoffsetn: {=u8:?}, tstref: {=bool:?}, calout: {=bool:?}, lock: {=bool:?} }}" , self . opampen () , self . force_vp () , self . vp_sel () , self . vm_sel () , self . tcm_en () , self . vms_sel () , self . vps_sel () , self . calon () , self . calsel () , self . pga_gain () , self . usertrim () , self . trimoffsetp () , self . trimoffsetn () , self . tstref () , self . calout () , self . lock ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calsel {
        #[doc = "VREFOPAMP = 3.3% VDDA"]
        PERCENT3_3 = 0x0,
        #[doc = "VREFOPAMP = 10% VDDA"]
        PERCENT10 = 0x01,
        #[doc = "VREFOPAMP = 50% VDDA"]
        PERCENT50 = 0x02,
        #[doc = "VREFOPAMP = 90% VDDA"]
        PERCENT90 = 0x03,
    }
    impl Calsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calsel {
        #[inline(always)]
        fn from(val: u8) -> Calsel {
            Calsel::from_bits(val)
        }
    }
    impl From<Calsel> for u8 {
        #[inline(always)]
        fn from(val: Calsel) -> u8 {
            Calsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PgaGain {
        #[doc = "Gain 2"]
        GAIN2 = 0x0,
        #[doc = "Gain 4"]
        GAIN4 = 0x01,
        #[doc = "Gain 8"]
        GAIN8 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Gain 16"]
        GAIN16 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Gain 2, feedback connected to VM0"]
        GAIN2_VM0 = 0x08,
        #[doc = "Gain 4, feedback connected to VM0"]
        GAIN4_VM0 = 0x09,
        #[doc = "Gain 8, feedback connected to VM0"]
        GAIN8_VM0 = 0x0a,
        #[doc = "Gain 16, feedback connected to VM0"]
        GAIN16_VM0 = 0x0b,
        #[doc = "Gain 2, feedback connected to VM1"]
        GAIN2_VM1 = 0x0c,
        #[doc = "Gain 4, feedback connected to VM1"]
        GAIN4_VM1 = 0x0d,
        #[doc = "Gain 8, feedback connected to VM1"]
        GAIN8_VM1 = 0x0e,
        #[doc = "Gain 16, feedback connected to VM1"]
        GAIN16_VM1 = 0x0f,
    }
    impl PgaGain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PgaGain {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PgaGain {
        #[inline(always)]
        fn from(val: u8) -> PgaGain {
            PgaGain::from_bits(val)
        }
    }
    impl From<PgaGain> for u8 {
        #[inline(always)]
        fn from(val: PgaGain) -> u8 {
            PgaGain::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VmSel {
        #[doc = "PC5 (VM0) used as OPAMP2 inverting input"]
        PC5 = 0x0,
        #[doc = "PA5 (VM1) used as OPAMP2 inverting input"]
        PA5 = 0x01,
        #[doc = "Resistor feedback output (PGA mode)"]
        PGA = 0x02,
        #[doc = "Follower mode"]
        FOLLOWER = 0x03,
    }
    impl VmSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VmSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VmSel {
        #[inline(always)]
        fn from(val: u8) -> VmSel {
            VmSel::from_bits(val)
        }
    }
    impl From<VmSel> for u8 {
        #[inline(always)]
        fn from(val: VmSel) -> u8 {
            VmSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VmsSel {
        #[doc = "PC5 (VM0) used as OPAMP2 inverting input when TCM_EN = 1"]
        PC5 = 0x0,
        #[doc = "PA5 (VM1) used as OPAMP2 inverting input when TCM_EN = 1"]
        PA5 = 0x01,
    }
    impl VmsSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VmsSel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VmsSel {
        #[inline(always)]
        fn from(val: u8) -> VmsSel {
            VmsSel::from_bits(val)
        }
    }
    impl From<VmsSel> for u8 {
        #[inline(always)]
        fn from(val: VmsSel) -> u8 {
            VmsSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VpSel {
        _RESERVED_0 = 0x0,
        #[doc = "PB14 used as OPAMP2 non-inverting input"]
        PB14 = 0x01,
        #[doc = "PB0 used as OPAMP2 non-inverting input"]
        PB0 = 0x02,
        #[doc = "PA7 used as OPAMP2 non-inverting input"]
        PA7 = 0x03,
    }
    impl VpSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VpSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VpSel {
        #[inline(always)]
        fn from(val: u8) -> VpSel {
            VpSel::from_bits(val)
        }
    }
    impl From<VpSel> for u8 {
        #[inline(always)]
        fn from(val: VpSel) -> u8 {
            VpSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VpsSel {
        _RESERVED_0 = 0x0,
        #[doc = "PB14 used as OPAMP2 non-inverting input when TCM_EN=1"]
        PB14 = 0x01,
        #[doc = "PB0 used as OPAMP2 non-inverting input when TCM_EN=1"]
        PB0 = 0x02,
        #[doc = "PA7 used as OPAMP2 non-inverting input when TCM_EN=1"]
        PA7 = 0x03,
    }
    impl VpsSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VpsSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VpsSel {
        #[inline(always)]
        fn from(val: u8) -> VpsSel {
            VpsSel::from_bits(val)
        }
    }
    impl From<VpsSel> for u8 {
        #[inline(always)]
        fn from(val: VpsSel) -> u8 {
            VpsSel::to_bits(val)
        }
    }
}
