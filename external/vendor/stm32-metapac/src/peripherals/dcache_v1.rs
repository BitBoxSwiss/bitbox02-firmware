#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Data cache."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcache {
    ptr: *mut u8,
}
unsafe impl Send for Dcache {}
unsafe impl Sync for Dcache {}
impl Dcache {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DCACHE control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DCACHE status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DCACHE interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DCACHE flag clear register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DCACHE read-hit monitor register."]
    #[inline(always)]
    pub const fn rhmonr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DCACHE read-miss monitor register."]
    #[inline(always)]
    pub const fn rmmonr(self) -> crate::common::Reg<regs::Rmmonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DCACHE write-hit monitor register."]
    #[inline(always)]
    pub const fn whmonr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DCACHE write-miss monitor register."]
    #[inline(always)]
    pub const fn wmmonr(self) -> crate::common::Reg<regs::Wmmonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "DCACHE command range start address register."]
    #[inline(always)]
    pub const fn cmdrsaddrr(self) -> crate::common::Reg<regs::Cmdrsaddrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "DCACHE command range end address register."]
    #[inline(always)]
    pub const fn cmdreaddrr(self) -> crate::common::Reg<regs::Cmdreaddrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "DCACHE command range end address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdreaddrr(pub u32);
    impl Cmdreaddrr {
        #[doc = "end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
        #[inline(always)]
        pub const fn cmdendaddr(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
        #[inline(always)]
        pub fn set_cmdendaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for Cmdreaddrr {
        #[inline(always)]
        fn default() -> Cmdreaddrr {
            Cmdreaddrr(0)
        }
    }
    impl core::fmt::Debug for Cmdreaddrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdreaddrr")
                .field("cmdendaddr", &self.cmdendaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmdreaddrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmdreaddrr {{ cmdendaddr: {=u32:?} }}", self.cmdendaddr())
        }
    }
    #[doc = "DCACHE command range start address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdrsaddrr(pub u32);
    impl Cmdrsaddrr {
        #[doc = "start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written.."]
        #[inline(always)]
        pub const fn cmdstartaddr(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written.."]
        #[inline(always)]
        pub fn set_cmdstartaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for Cmdrsaddrr {
        #[inline(always)]
        fn default() -> Cmdrsaddrr {
            Cmdrsaddrr(0)
        }
    }
    impl core::fmt::Debug for Cmdrsaddrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdrsaddrr")
                .field("cmdstartaddr", &self.cmdstartaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmdrsaddrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmdrsaddrr {{ cmdstartaddr: {=u32:?} }}", self.cmdstartaddr())
        }
    }
    #[doc = "DCACHE control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect."]
        #[inline(always)]
        pub const fn cacheinv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_cacheinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved."]
        #[inline(always)]
        pub const fn cachecmd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved."]
        #[inline(always)]
        pub fn set_cachecmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect."]
        #[inline(always)]
        pub const fn startcmd(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_startcmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "read-hit monitor enable."]
        #[inline(always)]
        pub const fn rhitmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "read-hit monitor enable."]
        #[inline(always)]
        pub fn set_rhitmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "read-miss monitor enable."]
        #[inline(always)]
        pub const fn rmissmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "read-miss monitor enable."]
        #[inline(always)]
        pub fn set_rmissmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "read-hit monitor reset."]
        #[inline(always)]
        pub const fn rhitmrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "read-hit monitor reset."]
        #[inline(always)]
        pub fn set_rhitmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "read-miss monitor reset."]
        #[inline(always)]
        pub const fn rmissmrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "read-miss monitor reset."]
        #[inline(always)]
        pub fn set_rmissmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "write-hit monitor enable."]
        #[inline(always)]
        pub const fn whitmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "write-hit monitor enable."]
        #[inline(always)]
        pub fn set_whitmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "write-miss monitor enable."]
        #[inline(always)]
        pub const fn wmissmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "write-miss monitor enable."]
        #[inline(always)]
        pub fn set_wmissmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "write-hit monitor reset."]
        #[inline(always)]
        pub const fn whitmrst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "write-hit monitor reset."]
        #[inline(always)]
        pub fn set_whitmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "write-miss monitor reset."]
        #[inline(always)]
        pub const fn wmissmrst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "write-miss monitor reset."]
        #[inline(always)]
        pub fn set_wmissmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "output burst type for cache master port read accesses Write access is always done in INCR burst type."]
        #[inline(always)]
        pub const fn hburst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "output burst type for cache master port read accesses Write access is always done in INCR burst type."]
        #[inline(always)]
        pub fn set_hburst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("en", &self.en())
                .field("cacheinv", &self.cacheinv())
                .field("cachecmd", &self.cachecmd())
                .field("startcmd", &self.startcmd())
                .field("rhitmen", &self.rhitmen())
                .field("rmissmen", &self.rmissmen())
                .field("rhitmrst", &self.rhitmrst())
                .field("rmissmrst", &self.rmissmrst())
                .field("whitmen", &self.whitmen())
                .field("wmissmen", &self.wmissmen())
                .field("whitmrst", &self.whitmrst())
                .field("wmissmrst", &self.wmissmrst())
                .field("hburst", &self.hburst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, cacheinv: {=bool:?}, cachecmd: {=u8:?}, startcmd: {=bool:?}, rhitmen: {=bool:?}, rmissmen: {=bool:?}, rhitmrst: {=bool:?}, rmissmrst: {=bool:?}, whitmen: {=bool:?}, wmissmen: {=bool:?}, whitmrst: {=bool:?}, wmissmrst: {=bool:?}, hburst: {=bool:?} }}" , self . en () , self . cacheinv () , self . cachecmd () , self . startcmd () , self . rhitmen () , self . rmissmen () , self . rhitmrst () , self . rmissmrst () , self . whitmen () , self . wmissmen () , self . whitmrst () , self . wmissmrst () , self . hburst ())
        }
    }
    #[doc = "DCACHE flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "clear full invalidate busy end flag Set by software."]
        #[inline(always)]
        pub const fn cbsyendf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clear full invalidate busy end flag Set by software."]
        #[inline(always)]
        pub fn set_cbsyendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "clear cache error flag Set by software."]
        #[inline(always)]
        pub const fn cerrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear cache error flag Set by software."]
        #[inline(always)]
        pub fn set_cerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "clear command end flag Set by software."]
        #[inline(always)]
        pub const fn ccmdendf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "clear command end flag Set by software."]
        #[inline(always)]
        pub fn set_ccmdendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("cbsyendf", &self.cbsyendf())
                .field("cerrf", &self.cerrf())
                .field("ccmdendf", &self.ccmdendf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fcr {{ cbsyendf: {=bool:?}, cerrf: {=bool:?}, ccmdendf: {=bool:?} }}",
                self.cbsyendf(),
                self.cerrf(),
                self.ccmdendf()
            )
        }
    }
    #[doc = "DCACHE interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "interrupt enable on busy end Set by SW to enable an interrupt generation at the end of a cache full invalidate operation."]
        #[inline(always)]
        pub const fn bsyendie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable on busy end Set by SW to enable an interrupt generation at the end of a cache full invalidate operation."]
        #[inline(always)]
        pub fn set_bsyendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (eviction or clean operation write-back error)."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (eviction or clean operation write-back error)."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "interrupt enable on command end Set by software to enable an interrupt generation at the end of a cache command (clean and/or invalidate an address range)."]
        #[inline(always)]
        pub const fn cmdendie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable on command end Set by software to enable an interrupt generation at the end of a cache command (clean and/or invalidate an address range)."]
        #[inline(always)]
        pub fn set_cmdendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("bsyendie", &self.bsyendie())
                .field("errie", &self.errie())
                .field("cmdendie", &self.cmdendie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ bsyendie: {=bool:?}, errie: {=bool:?}, cmdendie: {=bool:?} }}",
                self.bsyendie(),
                self.errie(),
                self.cmdendie()
            )
        }
    }
    #[doc = "DCACHE read-miss monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rmmonr(pub u32);
    impl Rmmonr {
        #[doc = "cache read-miss monitor counter."]
        #[inline(always)]
        pub const fn rmissmon(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "cache read-miss monitor counter."]
        #[inline(always)]
        pub fn set_rmissmon(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rmmonr {
        #[inline(always)]
        fn default() -> Rmmonr {
            Rmmonr(0)
        }
    }
    impl core::fmt::Debug for Rmmonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rmmonr").field("rmissmon", &self.rmissmon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rmmonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rmmonr {{ rmissmon: {=u16:?} }}", self.rmissmon())
        }
    }
    #[doc = "DCACHE status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "full invalidate busy flag."]
        #[inline(always)]
        pub const fn busyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "full invalidate busy flag."]
        #[inline(always)]
        pub fn set_busyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "full invalidate busy end flag Cleared by writing DCACHE_FCR.CBSYENDF = 1."]
        #[inline(always)]
        pub const fn bsyendf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "full invalidate busy end flag Cleared by writing DCACHE_FCR.CBSYENDF = 1."]
        #[inline(always)]
        pub fn set_bsyendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "cache error flag Cleared by writing DCACHE_FCR.CERRF = 1."]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "cache error flag Cleared by writing DCACHE_FCR.CERRF = 1."]
        #[inline(always)]
        pub fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "command busy flag."]
        #[inline(always)]
        pub const fn busycmdf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "command busy flag."]
        #[inline(always)]
        pub fn set_busycmdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "command end flag Cleared by writing DCACHE_FCR.CCMDENDF = 1."]
        #[inline(always)]
        pub const fn cmdendf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "command end flag Cleared by writing DCACHE_FCR.CCMDENDF = 1."]
        #[inline(always)]
        pub fn set_cmdendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("busyf", &self.busyf())
                .field("bsyendf", &self.bsyendf())
                .field("errf", &self.errf())
                .field("busycmdf", &self.busycmdf())
                .field("cmdendf", &self.cmdendf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ busyf: {=bool:?}, bsyendf: {=bool:?}, errf: {=bool:?}, busycmdf: {=bool:?}, cmdendf: {=bool:?} }}" , self . busyf () , self . bsyendf () , self . errf () , self . busycmdf () , self . cmdendf ())
        }
    }
    #[doc = "DCACHE write-miss monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wmmonr(pub u32);
    impl Wmmonr {
        #[doc = "cache write-miss monitor counter."]
        #[inline(always)]
        pub const fn wmissmon(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "cache write-miss monitor counter."]
        #[inline(always)]
        pub fn set_wmissmon(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wmmonr {
        #[inline(always)]
        fn default() -> Wmmonr {
            Wmmonr(0)
        }
    }
    impl core::fmt::Debug for Wmmonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wmmonr").field("wmissmon", &self.wmissmon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wmmonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wmmonr {{ wmissmon: {=u16:?} }}", self.wmissmon())
        }
    }
}
