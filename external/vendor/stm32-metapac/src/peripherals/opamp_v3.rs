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
    #[doc = "Offset trimming register in normal mode"]
    #[inline(always)]
    pub const fn otr(self) -> crate::common::Reg<regs::Otr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Offset trimming register in low-power mode"]
    #[inline(always)]
    pub const fn lpotr(self) -> crate::common::Reg<regs::Lpotr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
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
        #[doc = "Low-power mode enable. The operational amplifier must be disabled to change this configuration."]
        #[inline(always)]
        pub const fn opalpm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power mode enable. The operational amplifier must be disabled to change this configuration."]
        #[inline(always)]
        pub fn set_opalpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PGA mode"]
        #[inline(always)]
        pub const fn opamode(&self) -> super::vals::Opamode {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Opamode::from_bits(val as u8)
        }
        #[doc = "PGA mode"]
        #[inline(always)]
        pub fn set_opamode(&mut self, val: super::vals::Opamode) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Gain in PGA mode"]
        #[inline(always)]
        pub const fn pga_gain(&self) -> super::vals::PgaGain {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::PgaGain::from_bits(val as u8)
        }
        #[doc = "Gain in PGA mode"]
        #[inline(always)]
        pub fn set_pga_gain(&mut self, val: super::vals::PgaGain) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Inverting input selection"]
        #[inline(always)]
        pub const fn vm_sel(&self) -> super::vals::VmSel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::VmSel::from_bits(val as u8)
        }
        #[doc = "Inverting input selection"]
        #[inline(always)]
        pub fn set_vm_sel(&mut self, val: super::vals::VmSel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Non inverted input selection"]
        #[inline(always)]
        pub const fn vp_sel(&self) -> super::vals::VpSel {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::VpSel::from_bits(val as u8)
        }
        #[doc = "Non inverted input selection"]
        #[inline(always)]
        pub fn set_vp_sel(&mut self, val: super::vals::VpSel) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Calibration mode enable"]
        #[inline(always)]
        pub const fn calon(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration mode enable"]
        #[inline(always)]
        pub fn set_calon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Calibration selection"]
        #[inline(always)]
        pub const fn calsel(&self) -> super::vals::Calsel {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Calsel::from_bits(val as u8)
        }
        #[doc = "Calibration selection"]
        #[inline(always)]
        pub fn set_calsel(&mut self, val: super::vals::Calsel) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "User trimming enable"]
        #[inline(always)]
        pub const fn usertrim(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "User trimming enable"]
        #[inline(always)]
        pub fn set_usertrim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Calibration output"]
        #[inline(always)]
        pub const fn calout(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration output"]
        #[inline(always)]
        pub fn set_calout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Power supply range for stability"]
        #[inline(always)]
        pub const fn opa_range(&self) -> super::vals::OpaRange {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OpaRange::from_bits(val as u8)
        }
        #[doc = "Power supply range for stability"]
        #[inline(always)]
        pub fn set_opa_range(&mut self, val: super::vals::OpaRange) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                .field("opalpm", &self.opalpm())
                .field("opamode", &self.opamode())
                .field("pga_gain", &self.pga_gain())
                .field("vm_sel", &self.vm_sel())
                .field("vp_sel", &self.vp_sel())
                .field("calon", &self.calon())
                .field("calsel", &self.calsel())
                .field("usertrim", &self.usertrim())
                .field("calout", &self.calout())
                .field("opa_range", &self.opa_range())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ opampen: {=bool:?}, opalpm: {=bool:?}, opamode: {:?}, pga_gain: {:?}, vm_sel: {:?}, vp_sel: {:?}, calon: {=bool:?}, calsel: {:?}, usertrim: {=bool:?}, calout: {=bool:?}, opa_range: {:?} }}" , self . opampen () , self . opalpm () , self . opamode () , self . pga_gain () , self . vm_sel () , self . vp_sel () , self . calon () , self . calsel () , self . usertrim () , self . calout () , self . opa_range ())
        }
    }
    #[doc = "Offset trimming register in low-power mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpotr(pub u32);
    impl Lpotr {
        #[doc = "Offset trimming value (NMOS)"]
        #[inline(always)]
        pub const fn trimlpoffsetn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Offset trimming value (NMOS)"]
        #[inline(always)]
        pub fn set_trimlpoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Offset trimming value (PMOS)"]
        #[inline(always)]
        pub const fn trimlpoffsetp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Offset trimming value (PMOS)"]
        #[inline(always)]
        pub fn set_trimlpoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Lpotr {
        #[inline(always)]
        fn default() -> Lpotr {
            Lpotr(0)
        }
    }
    impl core::fmt::Debug for Lpotr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lpotr")
                .field("trimlpoffsetn", &self.trimlpoffsetn())
                .field("trimlpoffsetp", &self.trimlpoffsetp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lpotr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lpotr {{ trimlpoffsetn: {=u8:?}, trimlpoffsetp: {=u8:?} }}",
                self.trimlpoffsetn(),
                self.trimlpoffsetp()
            )
        }
    }
    #[doc = "Offset trimming register in normal mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otr(pub u32);
    impl Otr {
        #[doc = "Offset trimming value (NMOS)"]
        #[inline(always)]
        pub const fn trimoffsetn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Offset trimming value (NMOS)"]
        #[inline(always)]
        pub fn set_trimoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Offset trimming value (PMOS)"]
        #[inline(always)]
        pub const fn trimoffsetp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Offset trimming value (PMOS)"]
        #[inline(always)]
        pub fn set_trimoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Otr {
        #[inline(always)]
        fn default() -> Otr {
            Otr(0)
        }
    }
    impl core::fmt::Debug for Otr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otr")
                .field("trimoffsetn", &self.trimoffsetn())
                .field("trimoffsetp", &self.trimoffsetp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Otr {{ trimoffsetn: {=u8:?}, trimoffsetp: {=u8:?} }}",
                self.trimoffsetn(),
                self.trimoffsetp()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calsel {
        #[doc = "NMOS calibration, 0.2 V applied to OPAMP inputs during calibration"]
        NMOS = 0x0,
        #[doc = "PMOS calibration, VDDA - 0.2 V applied to OPAMP inputs during calibration"]
        PMOS = 0x01,
    }
    impl Calsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calsel {
            unsafe { core::mem::transmute(val & 0x01) }
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
    pub enum OpaRange {
        #[doc = "Low range (VDDA < 2.4 V)"]
        LOW = 0x0,
        #[doc = "High range (VDDA > 2.4 V)"]
        HIGH = 0x01,
    }
    impl OpaRange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpaRange {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpaRange {
        #[inline(always)]
        fn from(val: u8) -> OpaRange {
            OpaRange::from_bits(val)
        }
    }
    impl From<OpaRange> for u8 {
        #[inline(always)]
        fn from(val: OpaRange) -> u8 {
            OpaRange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Opamode {
        #[doc = "Internal PGA disable"]
        DISABLE = 0x0,
        #[doc = "Internal PGA disable (duplicate)"]
        DISABLE2 = 0x01,
        #[doc = "Internal PGA enable, gain programmed in PGA_GAIN"]
        ENABLE = 0x02,
        #[doc = "Internal follower"]
        FOLLOWER = 0x03,
    }
    impl Opamode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Opamode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Opamode {
        #[inline(always)]
        fn from(val: u8) -> Opamode {
            Opamode::from_bits(val)
        }
    }
    impl From<Opamode> for u8 {
        #[inline(always)]
        fn from(val: Opamode) -> u8 {
            Opamode::to_bits(val)
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
        #[doc = "Gain 16"]
        GAIN16 = 0x03,
    }
    impl PgaGain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PgaGain {
            unsafe { core::mem::transmute(val & 0x03) }
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
        #[doc = "GPIO connected to VINM (valid also in PGA mode for filtering)"]
        GPIO = 0x0,
        #[doc = "Low leakage inputs connected (only available in certain packages)"]
        LOW_LEAKAGE = 0x01,
        #[doc = "VINM not externally connected, valid only in PGA mode"]
        NOT_CONNECTED = 0x02,
        _RESERVED_3 = 0x03,
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
    pub enum VpSel {
        #[doc = "GPIO connected to VINP"]
        GPIO = 0x0,
        #[doc = "DAC connected to VINP"]
        DAC = 0x01,
    }
    impl VpSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VpSel {
            unsafe { core::mem::transmute(val & 0x01) }
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
}
