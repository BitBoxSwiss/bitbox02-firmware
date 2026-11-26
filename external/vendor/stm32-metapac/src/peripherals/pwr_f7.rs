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
    #[doc = "power control register"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "power control/status register"]
    #[inline(always)]
    pub const fn csr2(self) -> crate::common::Reg<regs::Csr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
        #[doc = "Low-power regulator in deepsleep under-drive mode"]
        #[inline(always)]
        pub const fn lpuds(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power regulator in deepsleep under-drive mode"]
        #[inline(always)]
        pub fn set_lpuds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Main regulator in deepsleep under-drive mode"]
        #[inline(always)]
        pub const fn mruds(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Main regulator in deepsleep under-drive mode"]
        #[inline(always)]
        pub fn set_mruds(&mut self, val: bool) {
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
        #[doc = "Over-drive enable"]
        #[inline(always)]
        pub const fn oden(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Over-drive enable"]
        #[inline(always)]
        pub fn set_oden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Over-drive switching enabled"]
        #[inline(always)]
        pub const fn odswen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Over-drive switching enabled"]
        #[inline(always)]
        pub fn set_odswen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Under-drive enable in stop mode"]
        #[inline(always)]
        pub const fn uden(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Under-drive enable in stop mode"]
        #[inline(always)]
        pub fn set_uden(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
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
                .field("csbf", &self.csbf())
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("dbp", &self.dbp())
                .field("fpds", &self.fpds())
                .field("lpuds", &self.lpuds())
                .field("mruds", &self.mruds())
                .field("adcdc1", &self.adcdc1())
                .field("vos", &self.vos())
                .field("oden", &self.oden())
                .field("odswen", &self.odswen())
                .field("uden", &self.uden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ lpds: {=bool:?}, pdds: {:?}, csbf: {=bool:?}, pvde: {=bool:?}, pls: {=u8:?}, dbp: {=bool:?}, fpds: {=bool:?}, lpuds: {=bool:?}, mruds: {=bool:?}, adcdc1: {=bool:?}, vos: {:?}, oden: {=bool:?}, odswen: {=bool:?}, uden: {=u8:?} }}" , self . lpds () , self . pdds () , self . csbf () , self . pvde () , self . pls () , self . dbp () , self . fpds () , self . lpuds () , self . mruds () , self . adcdc1 () , self . vos () , self . oden () , self . odswen () , self . uden ())
        }
    }
    #[doc = "power control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Clear Wakeup Pin flag for PA0"]
        #[inline(always)]
        pub const fn cwupf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear Wakeup Pin flag for PA0"]
        #[inline(always)]
        pub fn set_cwupf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup pin polarity bit for PA0"]
        #[inline(always)]
        pub const fn wupp(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for PA0"]
        #[inline(always)]
        pub fn set_wupp(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("cwupf[0]", &self.cwupf(0usize))
                .field("cwupf[1]", &self.cwupf(1usize))
                .field("cwupf[2]", &self.cwupf(2usize))
                .field("cwupf[3]", &self.cwupf(3usize))
                .field("cwupf[4]", &self.cwupf(4usize))
                .field("cwupf[5]", &self.cwupf(5usize))
                .field("wupp[0]", &self.wupp(0usize))
                .field("wupp[1]", &self.wupp(1usize))
                .field("wupp[2]", &self.wupp(2usize))
                .field("wupp[3]", &self.wupp(3usize))
                .field("wupp[4]", &self.wupp(4usize))
                .field("wupp[5]", &self.wupp(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ cwupf[0]: {=bool:?}, cwupf[1]: {=bool:?}, cwupf[2]: {=bool:?}, cwupf[3]: {=bool:?}, cwupf[4]: {=bool:?}, cwupf[5]: {=bool:?}, wupp[0]: {=bool:?}, wupp[1]: {=bool:?}, wupp[2]: {=bool:?}, wupp[3]: {=bool:?}, wupp[4]: {=bool:?}, wupp[5]: {=bool:?} }}" , self . cwupf (0usize) , self . cwupf (1usize) , self . cwupf (2usize) , self . cwupf (3usize) , self . cwupf (4usize) , self . cwupf (5usize) , self . wupp (0usize) , self . wupp (1usize) , self . wupp (2usize) , self . wupp (3usize) , self . wupp (4usize) , self . wupp (5usize))
        }
    }
    #[doc = "power control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr1(pub u32);
    impl Csr1 {
        #[doc = "Wakeup internal flag"]
        #[inline(always)]
        pub const fn wuif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup internal flag"]
        #[inline(always)]
        pub fn set_wuif(&mut self, val: bool) {
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
        #[doc = "Enable internal wakeup"]
        #[inline(always)]
        pub const fn eiwup(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wakeup"]
        #[inline(always)]
        pub fn set_eiwup(&mut self, val: bool) {
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
        #[doc = "Regulator voltage scaling output selection ready bit"]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Regulator voltage scaling output selection ready bit"]
        #[inline(always)]
        pub fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Over-drive mode ready"]
        #[inline(always)]
        pub const fn odrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Over-drive mode ready"]
        #[inline(always)]
        pub fn set_odrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Over-drive mode switching ready"]
        #[inline(always)]
        pub const fn odswrdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Over-drive mode switching ready"]
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
                .field("wuif", &self.wuif())
                .field("sbf", &self.sbf())
                .field("pvdo", &self.pvdo())
                .field("brr", &self.brr())
                .field("eiwup", &self.eiwup())
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
            defmt :: write ! (f , "Csr1 {{ wuif: {=bool:?}, sbf: {=bool:?}, pvdo: {=bool:?}, brr: {=bool:?}, eiwup: {=bool:?}, bre: {=bool:?}, vosrdy: {=bool:?}, odrdy: {=bool:?}, odswrdy: {=bool:?}, udrdy: {=u8:?} }}" , self . wuif () , self . sbf () , self . pvdo () , self . brr () , self . eiwup () , self . bre () , self . vosrdy () , self . odrdy () , self . odswrdy () , self . udrdy ())
        }
    }
    #[doc = "power control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr2(pub u32);
    impl Csr2 {
        #[doc = "Wakeup Pin flag for PA0"]
        #[inline(always)]
        pub const fn wupf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup Pin flag for PA0"]
        #[inline(always)]
        pub fn set_wupf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Enable Wakeup pin for PA0"]
        #[inline(always)]
        pub const fn ewup(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin for PA0"]
        #[inline(always)]
        pub fn set_ewup(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Csr2 {
        #[inline(always)]
        fn default() -> Csr2 {
            Csr2(0)
        }
    }
    impl core::fmt::Debug for Csr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr2")
                .field("wupf[0]", &self.wupf(0usize))
                .field("wupf[1]", &self.wupf(1usize))
                .field("wupf[2]", &self.wupf(2usize))
                .field("wupf[3]", &self.wupf(3usize))
                .field("wupf[4]", &self.wupf(4usize))
                .field("wupf[5]", &self.wupf(5usize))
                .field("ewup[0]", &self.ewup(0usize))
                .field("ewup[1]", &self.ewup(1usize))
                .field("ewup[2]", &self.ewup(2usize))
                .field("ewup[3]", &self.ewup(3usize))
                .field("ewup[4]", &self.ewup(4usize))
                .field("ewup[5]", &self.ewup(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr2 {{ wupf[0]: {=bool:?}, wupf[1]: {=bool:?}, wupf[2]: {=bool:?}, wupf[3]: {=bool:?}, wupf[4]: {=bool:?}, wupf[5]: {=bool:?}, ewup[0]: {=bool:?}, ewup[1]: {=bool:?}, ewup[2]: {=bool:?}, ewup[3]: {=bool:?}, ewup[4]: {=bool:?}, ewup[5]: {=bool:?} }}" , self . wupf (0usize) , self . wupf (1usize) , self . wupf (2usize) , self . wupf (3usize) , self . wupf (4usize) , self . wupf (5usize) , self . ewup (0usize) , self . ewup (1usize) , self . ewup (2usize) , self . ewup (3usize) , self . ewup (4usize) , self . ewup (5usize))
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
        #[doc = "Scale 3 mode"]
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
