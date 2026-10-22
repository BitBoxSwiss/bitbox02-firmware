#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Power control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "power control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "power control/status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "power control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Low-power deepsleep/Sleep/Low-power run"]
        #[inline(always)]
        pub const fn lpsdsr(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Low-power deepsleep/Sleep/Low-power run"]
        #[inline(always)]
        pub fn set_lpsdsr(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Power down deepsleep"]
        #[inline(always)]
        pub const fn pdds(&self) -> super::vals::Pdds {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Pdds::from_bits(val as u8)
        }
        #[doc = "Power down deepsleep"]
        #[inline(always)]
        pub fn set_pdds(&mut self, val: super::vals::Pdds) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear wakeup flag"]
        #[inline(always)]
        pub const fn cwuf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag"]
        #[inline(always)]
        pub fn set_cwuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear standby flag"]
        #[inline(always)]
        pub const fn csbf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear standby flag"]
        #[inline(always)]
        pub fn set_csbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PVD level selection"]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "PVD level selection"]
        #[inline(always)]
        pub fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Ultra-low-power mode"]
        #[inline(always)]
        pub const fn ulp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Ultra-low-power mode"]
        #[inline(always)]
        pub fn set_ulp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fast wakeup"]
        #[inline(always)]
        pub const fn fwu(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Fast wakeup"]
        #[inline(always)]
        pub fn set_fwu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 11usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
        }
        #[doc = "Deep sleep mode with Flash memory kept off"]
        #[inline(always)]
        pub const fn ds_ee_koff(&self) -> super::vals::DsEeKoff {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::DsEeKoff::from_bits(val as u8)
        }
        #[doc = "Deep sleep mode with Flash memory kept off"]
        #[inline(always)]
        pub fn set_ds_ee_koff(&mut self, val: super::vals::DsEeKoff) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Low power run mode"]
        #[inline(always)]
        pub const fn lprun(&self) -> super::vals::Mode {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Low power run mode"]
        #[inline(always)]
        pub fn set_lprun(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Regulator in Low-power deepsleep mode"]
        #[inline(always)]
        pub const fn lpds(&self) -> super::vals::Mode {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Regulator in Low-power deepsleep mode"]
        #[inline(always)]
        pub fn set_lpds(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
                .field("lpsdsr", &self.lpsdsr())
                .field("pdds", &self.pdds())
                .field("cwuf", &self.cwuf())
                .field("csbf", &self.csbf())
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("dbp", &self.dbp())
                .field("ulp", &self.ulp())
                .field("fwu", &self.fwu())
                .field("vos", &self.vos())
                .field("ds_ee_koff", &self.ds_ee_koff())
                .field("lprun", &self.lprun())
                .field("lpds", &self.lpds())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ lpsdsr: {:?}, pdds: {:?}, cwuf: {=bool:?}, csbf: {=bool:?}, pvde: {=bool:?}, pls: {:?}, dbp: {=bool:?}, ulp: {=bool:?}, fwu: {=bool:?}, vos: {:?}, ds_ee_koff: {:?}, lprun: {:?}, lpds: {:?} }}" , self . lpsdsr () , self . pdds () , self . cwuf () , self . csbf () , self . pvde () , self . pls () , self . dbp () , self . ulp () , self . fwu () , self . vos () , self . ds_ee_koff () , self . lprun () , self . lpds ())
        }
    }
    #[doc = "power control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Wakeup flag"]
        #[inline(always)]
        pub const fn wuf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag"]
        #[inline(always)]
        pub fn set_wuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Standby flag"]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Standby flag"]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD output"]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD output"]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Internal voltage reference ready flag"]
        #[inline(always)]
        pub const fn vrefintrdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Internal voltage reference ready flag"]
        #[inline(always)]
        pub fn set_vrefintrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Voltage Scaling select flag"]
        #[inline(always)]
        pub const fn vosf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage Scaling select flag"]
        #[inline(always)]
        pub fn set_vosf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Regulator LP flag"]
        #[inline(always)]
        pub const fn reglpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Regulator LP flag"]
        #[inline(always)]
        pub fn set_reglpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable WKUP pin 1"]
        #[inline(always)]
        pub const fn ewup1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP pin 1"]
        #[inline(always)]
        pub fn set_ewup1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable WKUP pin 2"]
        #[inline(always)]
        pub const fn ewup2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP pin 2"]
        #[inline(always)]
        pub fn set_ewup2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable WKUP pin 3"]
        #[inline(always)]
        pub const fn ewup3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP pin 3"]
        #[inline(always)]
        pub fn set_ewup3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("wuf", &self.wuf())
                .field("sbf", &self.sbf())
                .field("pvdo", &self.pvdo())
                .field("vrefintrdyf", &self.vrefintrdyf())
                .field("vosf", &self.vosf())
                .field("reglpf", &self.reglpf())
                .field("ewup1", &self.ewup1())
                .field("ewup2", &self.ewup2())
                .field("ewup3", &self.ewup3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ wuf: {=bool:?}, sbf: {=bool:?}, pvdo: {=bool:?}, vrefintrdyf: {=bool:?}, vosf: {=bool:?}, reglpf: {=bool:?}, ewup1: {=bool:?}, ewup2: {=bool:?}, ewup3: {=bool:?} }}" , self . wuf () , self . sbf () , self . pvdo () , self . vrefintrdyf () , self . vosf () , self . reglpf () , self . ewup1 () , self . ewup2 () , self . ewup3 ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DsEeKoff {
        #[doc = "NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set"]
        NVMWAKE_UP = 0x0,
        #[doc = "NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)"]
        NVMSLEEP = 0x01,
    }
    impl DsEeKoff {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DsEeKoff {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DsEeKoff {
        #[inline(always)]
        fn from(val: u8) -> DsEeKoff {
            DsEeKoff::from_bits(val)
        }
    }
    impl From<DsEeKoff> for u8 {
        #[inline(always)]
        fn from(val: DsEeKoff) -> u8 {
            DsEeKoff::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Voltage regulator in Main mode"]
        MAIN_MODE = 0x0,
        #[doc = "Voltage regulator switches to low-power mode"]
        LOW_POWER_MODE = 0x01,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x01) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pdds {
        #[doc = "Enter Stop mode when the CPU enters deepsleep"]
        STOP_MODE = 0x0,
        #[doc = "Enter Standby mode when the CPU enters deepsleep"]
        STANDBY_MODE = 0x01,
    }
    impl Pdds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pdds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pdds {
        #[inline(always)]
        fn from(val: u8) -> Pdds {
            Pdds::from_bits(val)
        }
    }
    impl From<Pdds> for u8 {
        #[inline(always)]
        fn from(val: Pdds) -> u8 {
            Pdds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pls {
        #[doc = "1.9 V"]
        V1_9 = 0x0,
        #[doc = "2.1 V"]
        V2_1 = 0x01,
        #[doc = "2.3 V"]
        V2_3 = 0x02,
        #[doc = "2.5 V"]
        V2_5 = 0x03,
        #[doc = "2.7 V"]
        V2_7 = 0x04,
        #[doc = "2.9 V"]
        V2_9 = 0x05,
        #[doc = "3.1 V"]
        V3_1 = 0x06,
        #[doc = "External input analog voltage (Compare internally to VREFINT)"]
        EXTERNAL = 0x07,
    }
    impl Pls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pls {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pls {
        #[inline(always)]
        fn from(val: u8) -> Pls {
            Pls::from_bits(val)
        }
    }
    impl From<Pls> for u8 {
        #[inline(always)]
        fn from(val: Pls) -> u8 {
            Pls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vos {
        _RESERVED_0 = 0x0,
        #[doc = "1.8 V (range 1)"]
        RANGE1 = 0x01,
        #[doc = "1.5 V (range 2)"]
        RANGE2 = 0x02,
        #[doc = "1.2 V (range 3)"]
        RANGE3 = 0x03,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vos {
        #[inline(always)]
        fn from(val: u8) -> Vos {
            Vos::from_bits(val)
        }
    }
    impl From<Vos> for u8 {
        #[inline(always)]
        fn from(val: Vos) -> u8 {
            Vos::to_bits(val)
        }
    }
}
