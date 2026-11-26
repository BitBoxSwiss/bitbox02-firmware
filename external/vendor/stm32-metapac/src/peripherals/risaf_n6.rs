#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Resource isolation slave unit for address space protection."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Risaf {
    ptr: *mut u8,
}
unsafe impl Send for Risaf {}
unsafe impl Sync for Risaf {}
impl Risaf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RISAF configuration register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RISAF illegal access status register."]
    #[inline(always)]
    pub const fn iasr(self) -> crate::common::Reg<regs::Iasr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RISAF illegal access clear register."]
    #[inline(always)]
    pub const fn iacr(self) -> crate::common::Reg<regs::Iacr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RISAF illegal access error status register."]
    #[inline(always)]
    pub const fn iaesr(self) -> crate::common::Reg<regs::Iaesr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RISAF illegal address register."]
    #[inline(always)]
    pub const fn iaddr(self) -> crate::common::Reg<regs::Iaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RISAF region configuration register."]
    #[inline(always)]
    pub const fn reg_cfgr(self, n: usize) -> crate::common::Reg<regs::RegCfgr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 64usize) as _) }
    }
    #[doc = "RISAF region start address register."]
    #[inline(always)]
    pub const fn reg_startr(self, n: usize) -> crate::common::Reg<regs::RegStartr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize + n * 64usize) as _) }
    }
    #[doc = "RISAF region end address register."]
    #[inline(always)]
    pub const fn reg_endr(self, n: usize) -> crate::common::Reg<regs::RegEndr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize + n * 64usize) as _) }
    }
    #[doc = "RISAF region CID configuration register."]
    #[inline(always)]
    pub const fn reg_cidcfgr(self, n: usize) -> crate::common::Reg<regs::RegCidcfgr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize + n * 64usize) as _) }
    }
}
pub mod regs {
    #[doc = "RISAF configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Global lock. When set, writes to RISAF registers are ignored except for IACR and region configuration registers."]
        #[inline(always)]
        pub const fn glock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global lock. When set, writes to RISAF registers are ignored except for IACR and region configuration registers."]
        #[inline(always)]
        pub fn set_glock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Cr").field("glock", &self.glock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr {{ glock: {=bool:?} }}", self.glock())
        }
    }
    #[doc = "RISAF illegal access clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iacr(pub u32);
    impl Iacr {
        #[doc = "Clear configuration access error flag."]
        #[inline(always)]
        pub const fn caef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear configuration access error flag."]
        #[inline(always)]
        pub fn set_caef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear illegal access error flag."]
        #[inline(always)]
        pub const fn iaef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear illegal access error flag."]
        #[inline(always)]
        pub fn set_iaef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Iacr {
        #[inline(always)]
        fn default() -> Iacr {
            Iacr(0)
        }
    }
    impl core::fmt::Debug for Iacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iacr")
                .field("caef", &self.caef())
                .field("iaef", &self.iaef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iacr {{ caef: {=bool:?}, iaef: {=bool:?} }}",
                self.caef(),
                self.iaef()
            )
        }
    }
    #[doc = "RISAF illegal address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iaddr(pub u32);
    impl Iaddr {
        #[doc = "Illegal access address."]
        #[inline(always)]
        pub const fn iadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Illegal access address."]
        #[inline(always)]
        pub fn set_iadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iaddr {
        #[inline(always)]
        fn default() -> Iaddr {
            Iaddr(0)
        }
    }
    impl core::fmt::Debug for Iaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iaddr").field("iadd", &self.iadd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iaddr {{ iadd: {=u32:?} }}", self.iadd())
        }
    }
    #[doc = "RISAF illegal access error status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iaesr(pub u32);
    impl Iaesr {
        #[doc = "Illegal access compartment ID."]
        #[inline(always)]
        pub const fn iacid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Illegal access compartment ID."]
        #[inline(always)]
        pub fn set_iacid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Illegal access was privileged."]
        #[inline(always)]
        pub const fn iapriv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal access was privileged."]
        #[inline(always)]
        pub fn set_iapriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Illegal access was secure."]
        #[inline(always)]
        pub const fn iasec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal access was secure."]
        #[inline(always)]
        pub fn set_iasec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Illegal access was a write (0 = read/fetch, 1 = write)."]
        #[inline(always)]
        pub const fn ianrw(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal access was a write (0 = read/fetch, 1 = write)."]
        #[inline(always)]
        pub fn set_ianrw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Iaesr {
        #[inline(always)]
        fn default() -> Iaesr {
            Iaesr(0)
        }
    }
    impl core::fmt::Debug for Iaesr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iaesr")
                .field("iacid", &self.iacid())
                .field("iapriv", &self.iapriv())
                .field("iasec", &self.iasec())
                .field("ianrw", &self.ianrw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iaesr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iaesr {{ iacid: {=u8:?}, iapriv: {=bool:?}, iasec: {=bool:?}, ianrw: {=bool:?} }}",
                self.iacid(),
                self.iapriv(),
                self.iasec(),
                self.ianrw()
            )
        }
    }
    #[doc = "RISAF illegal access status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iasr(pub u32);
    impl Iasr {
        #[doc = "Configuration access error flag."]
        #[inline(always)]
        pub const fn caef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration access error flag."]
        #[inline(always)]
        pub fn set_caef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Illegal access error flag."]
        #[inline(always)]
        pub const fn iaef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal access error flag."]
        #[inline(always)]
        pub fn set_iaef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Iasr {
        #[inline(always)]
        fn default() -> Iasr {
            Iasr(0)
        }
    }
    impl core::fmt::Debug for Iasr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iasr")
                .field("caef", &self.caef())
                .field("iaef", &self.iaef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iasr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iasr {{ caef: {=bool:?}, iaef: {=bool:?} }}",
                self.caef(),
                self.iaef()
            )
        }
    }
    #[doc = "RISAF region configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegCfgr(pub u32);
    impl RegCfgr {
        #[doc = "Base region enable."]
        #[inline(always)]
        pub const fn bren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Base region enable."]
        #[inline(always)]
        pub fn set_bren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure region. When set, only secure requests can access this region."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure region. When set, only secure requests can access this region."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged access for compartment y. When set, compartment y can only access in privileged mode."]
        #[inline(always)]
        pub const fn privc(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Privileged access for compartment y. When set, compartment y can only access in privileged mode."]
        #[inline(always)]
        pub fn set_privc(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for RegCfgr {
        #[inline(always)]
        fn default() -> RegCfgr {
            RegCfgr(0)
        }
    }
    impl core::fmt::Debug for RegCfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegCfgr")
                .field("bren", &self.bren())
                .field("sec", &self.sec())
                .field("privc[0]", &self.privc(0usize))
                .field("privc[1]", &self.privc(1usize))
                .field("privc[2]", &self.privc(2usize))
                .field("privc[3]", &self.privc(3usize))
                .field("privc[4]", &self.privc(4usize))
                .field("privc[5]", &self.privc(5usize))
                .field("privc[6]", &self.privc(6usize))
                .field("privc[7]", &self.privc(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RegCfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RegCfgr {{ bren: {=bool:?}, sec: {=bool:?}, privc[0]: {=bool:?}, privc[1]: {=bool:?}, privc[2]: {=bool:?}, privc[3]: {=bool:?}, privc[4]: {=bool:?}, privc[5]: {=bool:?}, privc[6]: {=bool:?}, privc[7]: {=bool:?} }}" , self . bren () , self . sec () , self . privc (0usize) , self . privc (1usize) , self . privc (2usize) , self . privc (3usize) , self . privc (4usize) , self . privc (5usize) , self . privc (6usize) , self . privc (7usize))
        }
    }
    #[doc = "RISAF region CID configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegCidcfgr(pub u32);
    impl RegCidcfgr {
        #[doc = "Read enable for compartment y."]
        #[inline(always)]
        pub const fn rdenc(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Read enable for compartment y."]
        #[inline(always)]
        pub fn set_rdenc(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Write enable for compartment y."]
        #[inline(always)]
        pub const fn wrenc(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Write enable for compartment y."]
        #[inline(always)]
        pub fn set_wrenc(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for RegCidcfgr {
        #[inline(always)]
        fn default() -> RegCidcfgr {
            RegCidcfgr(0)
        }
    }
    impl core::fmt::Debug for RegCidcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegCidcfgr")
                .field("rdenc[0]", &self.rdenc(0usize))
                .field("rdenc[1]", &self.rdenc(1usize))
                .field("rdenc[2]", &self.rdenc(2usize))
                .field("rdenc[3]", &self.rdenc(3usize))
                .field("rdenc[4]", &self.rdenc(4usize))
                .field("rdenc[5]", &self.rdenc(5usize))
                .field("rdenc[6]", &self.rdenc(6usize))
                .field("rdenc[7]", &self.rdenc(7usize))
                .field("wrenc[0]", &self.wrenc(0usize))
                .field("wrenc[1]", &self.wrenc(1usize))
                .field("wrenc[2]", &self.wrenc(2usize))
                .field("wrenc[3]", &self.wrenc(3usize))
                .field("wrenc[4]", &self.wrenc(4usize))
                .field("wrenc[5]", &self.wrenc(5usize))
                .field("wrenc[6]", &self.wrenc(6usize))
                .field("wrenc[7]", &self.wrenc(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RegCidcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RegCidcfgr {{ rdenc[0]: {=bool:?}, rdenc[1]: {=bool:?}, rdenc[2]: {=bool:?}, rdenc[3]: {=bool:?}, rdenc[4]: {=bool:?}, rdenc[5]: {=bool:?}, rdenc[6]: {=bool:?}, rdenc[7]: {=bool:?}, wrenc[0]: {=bool:?}, wrenc[1]: {=bool:?}, wrenc[2]: {=bool:?}, wrenc[3]: {=bool:?}, wrenc[4]: {=bool:?}, wrenc[5]: {=bool:?}, wrenc[6]: {=bool:?}, wrenc[7]: {=bool:?} }}" , self . rdenc (0usize) , self . rdenc (1usize) , self . rdenc (2usize) , self . rdenc (3usize) , self . rdenc (4usize) , self . rdenc (5usize) , self . rdenc (6usize) , self . rdenc (7usize) , self . wrenc (0usize) , self . wrenc (1usize) , self . wrenc (2usize) , self . wrenc (3usize) , self . wrenc (4usize) , self . wrenc (5usize) , self . wrenc (6usize) , self . wrenc (7usize))
        }
    }
    #[doc = "RISAF region end address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegEndr(pub u32);
    impl RegEndr {
        #[doc = "Base region end address."]
        #[inline(always)]
        pub const fn baddend(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Base region end address."]
        #[inline(always)]
        pub fn set_baddend(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RegEndr {
        #[inline(always)]
        fn default() -> RegEndr {
            RegEndr(0)
        }
    }
    impl core::fmt::Debug for RegEndr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegEndr").field("baddend", &self.baddend()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RegEndr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RegEndr {{ baddend: {=u32:?} }}", self.baddend())
        }
    }
    #[doc = "RISAF region start address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegStartr(pub u32);
    impl RegStartr {
        #[doc = "Base region start address."]
        #[inline(always)]
        pub const fn baddstart(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Base region start address."]
        #[inline(always)]
        pub fn set_baddstart(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RegStartr {
        #[inline(always)]
        fn default() -> RegStartr {
            RegStartr(0)
        }
    }
    impl core::fmt::Debug for RegStartr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegStartr")
                .field("baddstart", &self.baddstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RegStartr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RegStartr {{ baddstart: {=u32:?} }}", self.baddstart())
        }
    }
}
