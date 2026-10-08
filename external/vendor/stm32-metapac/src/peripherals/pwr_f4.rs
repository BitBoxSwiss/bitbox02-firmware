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
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "power control/status register"]
    #[inline(always)]
    pub const fn csr1(self) -> crate::common::Reg<regs::Csr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "power control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power deep sleep"]
        #[inline(always)]
        pub const fn lpds(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power deep sleep"]
        #[inline(always)]
        pub fn set_lpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        pub const fn pls(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "PVD level selection"]
        #[inline(always)]
        pub fn set_pls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
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
        #[doc = "Flash power down in Stop mode"]
        #[inline(always)]
        pub const fn fpds(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Flash power down in Stop mode"]
        #[inline(always)]
        pub fn set_fpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Low-Power Regulator Low Voltage in deepsleep"]
        #[inline(always)]
        pub const fn lplvds(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Regulator Low Voltage in deepsleep"]
        #[inline(always)]
        pub fn set_lplvds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Main regulator low voltage in deepsleep mode"]
        #[inline(always)]
        pub const fn mrlvds(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Main regulator low voltage in deepsleep mode"]
        #[inline(always)]
        pub fn set_mrlvds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ADCDC1"]
        #[inline(always)]
        pub const fn adcdc1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADCDC1"]
        #[inline(always)]
        pub fn set_adcdc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Regulator voltage scaling output selection"]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Regulator voltage scaling output selection"]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Over-drive enable (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub const fn oden(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Over-drive enable (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub fn set_oden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Over-drive switching enabled (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub const fn odswen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Over-drive switching enabled (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub fn set_odswen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Under-drive enable in stop mode (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub const fn uden(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Under-drive enable in stop mode (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub fn set_uden(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Flash Memory Stop while System Run"]
        #[inline(always)]
        pub const fn fmssr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Memory Stop while System Run"]
        #[inline(always)]
        pub fn set_fmssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Flash Interface Stop while System Run"]
        #[inline(always)]
        pub const fn fissr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Interface Stop while System Run"]
        #[inline(always)]
        pub fn set_fissr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("lpds", &self.lpds())
                .field("pdds", &self.pdds())
                .field("cwuf", &self.cwuf())
                .field("csbf", &self.csbf())
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("dbp", &self.dbp())
                .field("fpds", &self.fpds())
                .field("lplvds", &self.lplvds())
                .field("mrlvds", &self.mrlvds())
                .field("adcdc1", &self.adcdc1())
                .field("vos", &self.vos())
                .field("oden", &self.oden())
                .field("odswen", &self.odswen())
                .field("uden", &self.uden())
                .field("fmssr", &self.fmssr())
                .field("fissr", &self.fissr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ lpds: {=bool:?}, pdds: {:?}, cwuf: {=bool:?}, csbf: {=bool:?}, pvde: {=bool:?}, pls: {=u8:?}, dbp: {=bool:?}, fpds: {=bool:?}, lplvds: {=bool:?}, mrlvds: {=bool:?}, adcdc1: {=bool:?}, vos: {:?}, oden: {=bool:?}, odswen: {=bool:?}, uden: {=u8:?}, fmssr: {=bool:?}, fissr: {=bool:?} }}" , self . lpds () , self . pdds () , self . cwuf () , self . csbf () , self . pvde () , self . pls () , self . dbp () , self . fpds () , self . lplvds () , self . mrlvds () , self . adcdc1 () , self . vos () , self . oden () , self . odswen () , self . uden () , self . fmssr () , self . fissr ())
        }
    }
    #[doc = "power control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr1(pub u32);
    impl Csr1 {
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
        #[doc = "Backup regulator ready"]
        #[inline(always)]
        pub const fn brr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Backup regulator ready"]
        #[inline(always)]
        pub fn set_brr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable WKUP2 pin"]
        #[inline(always)]
        pub const fn ewup2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP2 pin"]
        #[inline(always)]
        pub fn set_ewup2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable WKUP pin"]
        #[inline(always)]
        pub const fn ewup(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP pin"]
        #[inline(always)]
        pub fn set_ewup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Backup regulator enable"]
        #[inline(always)]
        pub const fn bre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Backup regulator enable"]
        #[inline(always)]
        pub fn set_bre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Regulator voltage scaling output selection ready bit (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Regulator voltage scaling output selection ready bit (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Over-drive mode ready (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub const fn odrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Over-drive mode ready (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub fn set_odrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Over-drive mode switching ready (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub const fn odswrdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Over-drive mode switching ready (STM32F4\\[23\\]
ONLY)"]
        #[inline(always)]
        pub fn set_odswrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Under-drive ready flag"]
        #[inline(always)]
        pub const fn udrdy(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Under-drive ready flag"]
        #[inline(always)]
        pub fn set_udrdy(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for Csr1 {
        #[inline(always)]
        fn default() -> Csr1 {
            Csr1(0)
        }
    }
    impl core::fmt::Debug for Csr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr1")
                .field("wuf", &self.wuf())
                .field("sbf", &self.sbf())
                .field("pvdo", &self.pvdo())
                .field("brr", &self.brr())
                .field("ewup2", &self.ewup2())
                .field("ewup", &self.ewup())
                .field("bre", &self.bre())
                .field("vosrdy", &self.vosrdy())
                .field("odrdy", &self.odrdy())
                .field("odswrdy", &self.odswrdy())
                .field("udrdy", &self.udrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr1 {{ wuf: {=bool:?}, sbf: {=bool:?}, pvdo: {=bool:?}, brr: {=bool:?}, ewup2: {=bool:?}, ewup: {=bool:?}, bre: {=bool:?}, vosrdy: {=bool:?}, odrdy: {=bool:?}, odswrdy: {=bool:?}, udrdy: {=u8:?} }}" , self . wuf () , self . sbf () , self . pvdo () , self . brr () , self . ewup2 () , self . ewup () , self . bre () , self . vosrdy () , self . odrdy () , self . odswrdy () , self . udrdy ())
        }
    }
}
pub mod vals {
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
    pub enum Vos {
        _RESERVED_0 = 0x0,
        #[doc = "Scale 3 mode (STM32F4\\[23\\]
ONLY)"]
        SCALE3 = 0x01,
        #[doc = "Scale 2 mode"]
        SCALE2 = 0x02,
        #[doc = "Scale 1 mode (reset value)"]
        SCALE1 = 0x03,
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
