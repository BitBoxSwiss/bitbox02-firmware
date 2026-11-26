#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Resource isolation framework security controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rifsc {
    ptr: *mut u8,
}
unsafe impl Send for Rifsc {}
unsafe impl Sync for Rifsc {}
impl Rifsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RIFSC RISC slave control register."]
    #[inline(always)]
    pub const fn risc_cr(self) -> crate::common::Reg<regs::RiscCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RIFSC RISC slave security configuration register."]
    #[inline(always)]
    pub const fn risc_seccfgr(self, n: usize) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "RIFSC RISC slave privilege configuration register."]
    #[inline(always)]
    pub const fn risc_privcfgr(self, n: usize) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "RIFSC RISC slave resource configuration lock register."]
    #[inline(always)]
    pub const fn risc_rcfglockr(self, n: usize) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
    }
    #[doc = "RIFSC RIMC master control register."]
    #[inline(always)]
    pub const fn rimc_cr(self) -> crate::common::Reg<regs::RimcCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[doc = "RIFSC RIMC master attribute register."]
    #[inline(always)]
    pub const fn rimc_attr(self, n: usize) -> crate::common::Reg<regs::RimcAttr, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c10usize + n * 4usize) as _) }
    }
    #[doc = "RIFSC peripheral protection status register."]
    #[inline(always)]
    pub const fn ppsr(self, n: usize) -> crate::common::Reg<regs::Cfgr, crate::common::R> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fb0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "RIFSC configuration register for 32 peripherals."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Configuration bit for peripheral N."]
        #[inline(always)]
        pub const fn cfg(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Configuration bit for peripheral N."]
        #[inline(always)]
        pub fn set_cfg(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("cfg[0]", &self.cfg(0usize))
                .field("cfg[1]", &self.cfg(1usize))
                .field("cfg[2]", &self.cfg(2usize))
                .field("cfg[3]", &self.cfg(3usize))
                .field("cfg[4]", &self.cfg(4usize))
                .field("cfg[5]", &self.cfg(5usize))
                .field("cfg[6]", &self.cfg(6usize))
                .field("cfg[7]", &self.cfg(7usize))
                .field("cfg[8]", &self.cfg(8usize))
                .field("cfg[9]", &self.cfg(9usize))
                .field("cfg[10]", &self.cfg(10usize))
                .field("cfg[11]", &self.cfg(11usize))
                .field("cfg[12]", &self.cfg(12usize))
                .field("cfg[13]", &self.cfg(13usize))
                .field("cfg[14]", &self.cfg(14usize))
                .field("cfg[15]", &self.cfg(15usize))
                .field("cfg[16]", &self.cfg(16usize))
                .field("cfg[17]", &self.cfg(17usize))
                .field("cfg[18]", &self.cfg(18usize))
                .field("cfg[19]", &self.cfg(19usize))
                .field("cfg[20]", &self.cfg(20usize))
                .field("cfg[21]", &self.cfg(21usize))
                .field("cfg[22]", &self.cfg(22usize))
                .field("cfg[23]", &self.cfg(23usize))
                .field("cfg[24]", &self.cfg(24usize))
                .field("cfg[25]", &self.cfg(25usize))
                .field("cfg[26]", &self.cfg(26usize))
                .field("cfg[27]", &self.cfg(27usize))
                .field("cfg[28]", &self.cfg(28usize))
                .field("cfg[29]", &self.cfg(29usize))
                .field("cfg[30]", &self.cfg(30usize))
                .field("cfg[31]", &self.cfg(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ cfg[0]: {=bool:?}, cfg[1]: {=bool:?}, cfg[2]: {=bool:?}, cfg[3]: {=bool:?}, cfg[4]: {=bool:?}, cfg[5]: {=bool:?}, cfg[6]: {=bool:?}, cfg[7]: {=bool:?}, cfg[8]: {=bool:?}, cfg[9]: {=bool:?}, cfg[10]: {=bool:?}, cfg[11]: {=bool:?}, cfg[12]: {=bool:?}, cfg[13]: {=bool:?}, cfg[14]: {=bool:?}, cfg[15]: {=bool:?}, cfg[16]: {=bool:?}, cfg[17]: {=bool:?}, cfg[18]: {=bool:?}, cfg[19]: {=bool:?}, cfg[20]: {=bool:?}, cfg[21]: {=bool:?}, cfg[22]: {=bool:?}, cfg[23]: {=bool:?}, cfg[24]: {=bool:?}, cfg[25]: {=bool:?}, cfg[26]: {=bool:?}, cfg[27]: {=bool:?}, cfg[28]: {=bool:?}, cfg[29]: {=bool:?}, cfg[30]: {=bool:?}, cfg[31]: {=bool:?} }}" , self . cfg (0usize) , self . cfg (1usize) , self . cfg (2usize) , self . cfg (3usize) , self . cfg (4usize) , self . cfg (5usize) , self . cfg (6usize) , self . cfg (7usize) , self . cfg (8usize) , self . cfg (9usize) , self . cfg (10usize) , self . cfg (11usize) , self . cfg (12usize) , self . cfg (13usize) , self . cfg (14usize) , self . cfg (15usize) , self . cfg (16usize) , self . cfg (17usize) , self . cfg (18usize) , self . cfg (19usize) , self . cfg (20usize) , self . cfg (21usize) , self . cfg (22usize) , self . cfg (23usize) , self . cfg (24usize) , self . cfg (25usize) , self . cfg (26usize) , self . cfg (27usize) , self . cfg (28usize) , self . cfg (29usize) , self . cfg (30usize) , self . cfg (31usize))
        }
    }
    #[doc = "RIFSC RIMC master attribute register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RimcAttr(pub u32);
    impl RimcAttr {
        #[doc = "Master compartment ID."]
        #[inline(always)]
        pub const fn mcid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Master compartment ID."]
        #[inline(always)]
        pub fn set_mcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Master secure attribute."]
        #[inline(always)]
        pub const fn msec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Master secure attribute."]
        #[inline(always)]
        pub fn set_msec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Master privilege attribute."]
        #[inline(always)]
        pub const fn mpriv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Master privilege attribute."]
        #[inline(always)]
        pub fn set_mpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Master lock. When set, this master attribute register cannot be modified."]
        #[inline(always)]
        pub const fn mlock(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Master lock. When set, this master attribute register cannot be modified."]
        #[inline(always)]
        pub fn set_mlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for RimcAttr {
        #[inline(always)]
        fn default() -> RimcAttr {
            RimcAttr(0)
        }
    }
    impl core::fmt::Debug for RimcAttr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RimcAttr")
                .field("mcid", &self.mcid())
                .field("msec", &self.msec())
                .field("mpriv", &self.mpriv())
                .field("mlock", &self.mlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RimcAttr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RimcAttr {{ mcid: {=u8:?}, msec: {=bool:?}, mpriv: {=bool:?}, mlock: {=bool:?} }}",
                self.mcid(),
                self.msec(),
                self.mpriv(),
                self.mlock()
            )
        }
    }
    #[doc = "RIFSC RIMC master control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RimcCr(pub u32);
    impl RimcCr {
        #[doc = "Global lock. When set, all writes to RIFSC RIMC registers are ignored."]
        #[inline(always)]
        pub const fn glock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global lock. When set, all writes to RIFSC RIMC registers are ignored."]
        #[inline(always)]
        pub fn set_glock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Debug access port compartment ID."]
        #[inline(always)]
        pub const fn dapcid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Debug access port compartment ID."]
        #[inline(always)]
        pub fn set_dapcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for RimcCr {
        #[inline(always)]
        fn default() -> RimcCr {
            RimcCr(0)
        }
    }
    impl core::fmt::Debug for RimcCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RimcCr")
                .field("glock", &self.glock())
                .field("dapcid", &self.dapcid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RimcCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RimcCr {{ glock: {=bool:?}, dapcid: {=u8:?} }}",
                self.glock(),
                self.dapcid()
            )
        }
    }
    #[doc = "RIFSC RISC slave control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RiscCr(pub u32);
    impl RiscCr {
        #[doc = "Global lock. When set, all writes to RIFSC RISC registers are ignored."]
        #[inline(always)]
        pub const fn glock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global lock. When set, all writes to RIFSC RISC registers are ignored."]
        #[inline(always)]
        pub fn set_glock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for RiscCr {
        #[inline(always)]
        fn default() -> RiscCr {
            RiscCr(0)
        }
    }
    impl core::fmt::Debug for RiscCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RiscCr").field("glock", &self.glock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RiscCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RiscCr {{ glock: {=bool:?} }}", self.glock())
        }
    }
}
