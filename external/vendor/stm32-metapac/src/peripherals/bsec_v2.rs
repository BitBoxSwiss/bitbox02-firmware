#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Boot and security and OTP control."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsec {
    ptr: *mut u8,
}
unsafe impl Send for Bsec {}
unsafe impl Sync for Bsec {}
impl Bsec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BSEC fuse value register."]
    #[inline(always)]
    pub const fn fvr(self, n: usize) -> crate::common::Reg<regs::Fvr, crate::common::RW> {
        assert!(n < 376usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "BSEC sticky program lock register."]
    #[inline(always)]
    pub const fn splock(self, n: usize) -> crate::common::Reg<regs::Lockr, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
    }
    #[doc = "BSEC sticky write lock register."]
    #[inline(always)]
    pub const fn swlock(self, n: usize) -> crate::common::Reg<regs::Lockr, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0840usize + n * 4usize) as _) }
    }
    #[doc = "BSEC sticky read lock register."]
    #[inline(always)]
    pub const fn srlock(self, n: usize) -> crate::common::Reg<regs::Lockr, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0880usize + n * 4usize) as _) }
    }
    #[doc = "BSEC OTP valid status register."]
    #[inline(always)]
    pub const fn otpvldr(self, n: usize) -> crate::common::Reg<regs::Vldr, crate::common::R> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08c0usize + n * 4usize) as _) }
    }
    #[doc = "BSEC shadow fuse status register."]
    #[inline(always)]
    pub const fn sfsr(self, n: usize) -> crate::common::Reg<regs::Sfsr, crate::common::R> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0940usize + n * 4usize) as _) }
    }
    #[doc = "BSEC OTP control register."]
    #[inline(always)]
    pub const fn otpcr(self) -> crate::common::Reg<regs::Otpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c04usize) as _) }
    }
    #[doc = "BSEC OTP write data register."]
    #[inline(always)]
    pub const fn wdr(self) -> crate::common::Reg<regs::Wdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c08usize) as _) }
    }
    #[doc = "BSEC scratch register."]
    #[inline(always)]
    pub const fn scratchr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e00usize + n * 4usize) as _) }
    }
    #[doc = "BSEC global lock register."]
    #[inline(always)]
    pub const fn lockr(self) -> crate::common::Reg<regs::Glockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e10usize) as _) }
    }
    #[doc = "BSEC JTAG input register."]
    #[inline(always)]
    pub const fn jtaginr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e14usize) as _) }
    }
    #[doc = "BSEC JTAG output register."]
    #[inline(always)]
    pub const fn jtagoutr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e18usize) as _) }
    }
    #[doc = "BSEC unmap register."]
    #[inline(always)]
    pub const fn unmapr(self) -> crate::common::Reg<regs::Unmapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e24usize) as _) }
    }
    #[doc = "BSEC status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e40usize) as _) }
    }
    #[doc = "BSEC OTP status register."]
    #[inline(always)]
    pub const fn otpsr(self) -> crate::common::Reg<regs::Otpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e44usize) as _) }
    }
    #[doc = "BSEC epoch register."]
    #[inline(always)]
    pub const fn epochr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e80usize + n * 4usize) as _) }
    }
    #[doc = "BSEC epoch select register."]
    #[inline(always)]
    pub const fn epoch_selr(self) -> crate::common::Reg<regs::EpochSelr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e88usize) as _) }
    }
    #[doc = "BSEC debug control register."]
    #[inline(always)]
    pub const fn dbgcr(self) -> crate::common::Reg<regs::Dbgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e8cusize) as _) }
    }
    #[doc = "BSEC AP unlock register."]
    #[inline(always)]
    pub const fn ap_unlock(self) -> crate::common::Reg<regs::ApUnlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e90usize) as _) }
    }
    #[doc = "BSEC HDPL status register."]
    #[inline(always)]
    pub const fn hdplsr(self) -> crate::common::Reg<regs::Hdplsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e94usize) as _) }
    }
    #[doc = "BSEC HDPL control register."]
    #[inline(always)]
    pub const fn hdplcr(self) -> crate::common::Reg<regs::Hdplcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e98usize) as _) }
    }
    #[doc = "BSEC next HDPL register."]
    #[inline(always)]
    pub const fn nextlr(self) -> crate::common::Reg<regs::Nextlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e9cusize) as _) }
    }
    #[doc = "BSEC write-once scratch register."]
    #[inline(always)]
    pub const fn woscr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f40usize + n * 4usize) as _) }
    }
    #[doc = "BSEC hot reset count register."]
    #[inline(always)]
    pub const fn hrcr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[doc = "BSEC warm reset count register."]
    #[inline(always)]
    pub const fn wrcr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
}
pub mod regs {
    #[doc = "BSEC AP unlock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ApUnlock(pub u32);
    impl ApUnlock {
        #[doc = "Access port unlock key."]
        #[inline(always)]
        pub const fn ap_unlock(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Access port unlock key."]
        #[inline(always)]
        pub fn set_ap_unlock(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ApUnlock {
        #[inline(always)]
        fn default() -> ApUnlock {
            ApUnlock(0)
        }
    }
    impl core::fmt::Debug for ApUnlock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ApUnlock")
                .field("ap_unlock", &self.ap_unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ApUnlock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ApUnlock {{ ap_unlock: {=u32:?} }}", self.ap_unlock())
        }
    }
    #[doc = "BSEC debug control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgcr(pub u32);
    impl Dbgcr {
        #[doc = "Debug enable."]
        #[inline(always)]
        pub const fn dbgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Debug enable."]
        #[inline(always)]
        pub fn set_dbgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-invasive debug enable."]
        #[inline(always)]
        pub const fn niden(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-invasive debug enable."]
        #[inline(always)]
        pub fn set_niden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure privileged invasive debug enable."]
        #[inline(always)]
        pub const fn spiden(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Secure privileged invasive debug enable."]
        #[inline(always)]
        pub fn set_spiden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secure privileged non-invasive debug enable."]
        #[inline(always)]
        pub const fn spniden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secure privileged non-invasive debug enable."]
        #[inline(always)]
        pub fn set_spniden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Debug software enable."]
        #[inline(always)]
        pub const fn dbgswen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Debug software enable."]
        #[inline(always)]
        pub fn set_dbgswen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Dbgcr {
        #[inline(always)]
        fn default() -> Dbgcr {
            Dbgcr(0)
        }
    }
    impl core::fmt::Debug for Dbgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgcr")
                .field("dbgen", &self.dbgen())
                .field("niden", &self.niden())
                .field("spiden", &self.spiden())
                .field("spniden", &self.spniden())
                .field("dbgswen", &self.dbgswen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dbgcr {{ dbgen: {=bool:?}, niden: {=bool:?}, spiden: {=bool:?}, spniden: {=bool:?}, dbgswen: {=bool:?} }}" , self . dbgen () , self . niden () , self . spiden () , self . spniden () , self . dbgswen ())
        }
    }
    #[doc = "BSEC epoch select register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpochSelr(pub u32);
    impl EpochSelr {
        #[doc = "Epoch select."]
        #[inline(always)]
        pub const fn epoch_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Epoch select."]
        #[inline(always)]
        pub fn set_epoch_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EpochSelr {
        #[inline(always)]
        fn default() -> EpochSelr {
            EpochSelr(0)
        }
    }
    impl core::fmt::Debug for EpochSelr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpochSelr")
                .field("epoch_sel", &self.epoch_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EpochSelr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EpochSelr {{ epoch_sel: {=bool:?} }}", self.epoch_sel())
        }
    }
    #[doc = "BSEC fuse value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fvr(pub u32);
    impl Fvr {
        #[doc = "Fuse value."]
        #[inline(always)]
        pub const fn fv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Fuse value."]
        #[inline(always)]
        pub fn set_fv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fvr {
        #[inline(always)]
        fn default() -> Fvr {
            Fvr(0)
        }
    }
    impl core::fmt::Debug for Fvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fvr").field("fv", &self.fv()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fvr {{ fv: {=u32:?} }}", self.fv())
        }
    }
    #[doc = "BSEC global lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Glockr(pub u32);
    impl Glockr {
        #[doc = "Global lock. When set, all writes to BSEC are ignored until next reset."]
        #[inline(always)]
        pub const fn glock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global lock. When set, all writes to BSEC are ignored until next reset."]
        #[inline(always)]
        pub fn set_glock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Glockr {
        #[inline(always)]
        fn default() -> Glockr {
            Glockr(0)
        }
    }
    impl core::fmt::Debug for Glockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Glockr").field("glock", &self.glock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Glockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Glockr {{ glock: {=bool:?} }}", self.glock())
        }
    }
    #[doc = "BSEC HDPL control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdplcr(pub u32);
    impl Hdplcr {
        #[doc = "Increment HDPL. Write HDPL_INCR_CODE to increment HDPL."]
        #[inline(always)]
        pub const fn incr_hdpl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Increment HDPL. Write HDPL_INCR_CODE to increment HDPL."]
        #[inline(always)]
        pub fn set_incr_hdpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Hdplcr {
        #[inline(always)]
        fn default() -> Hdplcr {
            Hdplcr(0)
        }
    }
    impl core::fmt::Debug for Hdplcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdplcr").field("incr_hdpl", &self.incr_hdpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdplcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hdplcr {{ incr_hdpl: {=u8:?} }}", self.incr_hdpl())
        }
    }
    #[doc = "BSEC HDPL status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdplsr(pub u32);
    impl Hdplsr {
        #[doc = "Current hardware device protection level."]
        #[inline(always)]
        pub const fn hdpl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Current hardware device protection level."]
        #[inline(always)]
        pub fn set_hdpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Hdplsr {
        #[inline(always)]
        fn default() -> Hdplsr {
            Hdplsr(0)
        }
    }
    impl core::fmt::Debug for Hdplsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdplsr").field("hdpl", &self.hdpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdplsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hdplsr {{ hdpl: {=u8:?} }}", self.hdpl())
        }
    }
    #[doc = "BSEC lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lockr(pub u32);
    impl Lockr {
        #[doc = "Lock bits for fuse words 0 to 31 of the corresponding group."]
        #[inline(always)]
        pub const fn lock(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Lock bits for fuse words 0 to 31 of the corresponding group."]
        #[inline(always)]
        pub fn set_lock(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Lockr {
        #[inline(always)]
        fn default() -> Lockr {
            Lockr(0)
        }
    }
    impl core::fmt::Debug for Lockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lockr")
                .field("lock[0]", &self.lock(0usize))
                .field("lock[1]", &self.lock(1usize))
                .field("lock[2]", &self.lock(2usize))
                .field("lock[3]", &self.lock(3usize))
                .field("lock[4]", &self.lock(4usize))
                .field("lock[5]", &self.lock(5usize))
                .field("lock[6]", &self.lock(6usize))
                .field("lock[7]", &self.lock(7usize))
                .field("lock[8]", &self.lock(8usize))
                .field("lock[9]", &self.lock(9usize))
                .field("lock[10]", &self.lock(10usize))
                .field("lock[11]", &self.lock(11usize))
                .field("lock[12]", &self.lock(12usize))
                .field("lock[13]", &self.lock(13usize))
                .field("lock[14]", &self.lock(14usize))
                .field("lock[15]", &self.lock(15usize))
                .field("lock[16]", &self.lock(16usize))
                .field("lock[17]", &self.lock(17usize))
                .field("lock[18]", &self.lock(18usize))
                .field("lock[19]", &self.lock(19usize))
                .field("lock[20]", &self.lock(20usize))
                .field("lock[21]", &self.lock(21usize))
                .field("lock[22]", &self.lock(22usize))
                .field("lock[23]", &self.lock(23usize))
                .field("lock[24]", &self.lock(24usize))
                .field("lock[25]", &self.lock(25usize))
                .field("lock[26]", &self.lock(26usize))
                .field("lock[27]", &self.lock(27usize))
                .field("lock[28]", &self.lock(28usize))
                .field("lock[29]", &self.lock(29usize))
                .field("lock[30]", &self.lock(30usize))
                .field("lock[31]", &self.lock(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Lockr {{ lock[0]: {=bool:?}, lock[1]: {=bool:?}, lock[2]: {=bool:?}, lock[3]: {=bool:?}, lock[4]: {=bool:?}, lock[5]: {=bool:?}, lock[6]: {=bool:?}, lock[7]: {=bool:?}, lock[8]: {=bool:?}, lock[9]: {=bool:?}, lock[10]: {=bool:?}, lock[11]: {=bool:?}, lock[12]: {=bool:?}, lock[13]: {=bool:?}, lock[14]: {=bool:?}, lock[15]: {=bool:?}, lock[16]: {=bool:?}, lock[17]: {=bool:?}, lock[18]: {=bool:?}, lock[19]: {=bool:?}, lock[20]: {=bool:?}, lock[21]: {=bool:?}, lock[22]: {=bool:?}, lock[23]: {=bool:?}, lock[24]: {=bool:?}, lock[25]: {=bool:?}, lock[26]: {=bool:?}, lock[27]: {=bool:?}, lock[28]: {=bool:?}, lock[29]: {=bool:?}, lock[30]: {=bool:?}, lock[31]: {=bool:?} }}" , self . lock (0usize) , self . lock (1usize) , self . lock (2usize) , self . lock (3usize) , self . lock (4usize) , self . lock (5usize) , self . lock (6usize) , self . lock (7usize) , self . lock (8usize) , self . lock (9usize) , self . lock (10usize) , self . lock (11usize) , self . lock (12usize) , self . lock (13usize) , self . lock (14usize) , self . lock (15usize) , self . lock (16usize) , self . lock (17usize) , self . lock (18usize) , self . lock (19usize) , self . lock (20usize) , self . lock (21usize) , self . lock (22usize) , self . lock (23usize) , self . lock (24usize) , self . lock (25usize) , self . lock (26usize) , self . lock (27usize) , self . lock (28usize) , self . lock (29usize) , self . lock (30usize) , self . lock (31usize))
        }
    }
    #[doc = "BSEC next HDPL register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nextlr(pub u32);
    impl Nextlr {
        #[doc = "Next HDPL level."]
        #[inline(always)]
        pub const fn nextl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Next HDPL level."]
        #[inline(always)]
        pub fn set_nextl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Nextlr {
        #[inline(always)]
        fn default() -> Nextlr {
            Nextlr(0)
        }
    }
    impl core::fmt::Debug for Nextlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nextlr").field("nextl", &self.nextl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nextlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nextlr {{ nextl: {=u8:?} }}", self.nextl())
        }
    }
    #[doc = "BSEC OTP control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otpcr(pub u32);
    impl Otpcr {
        #[doc = "Fuse word address."]
        #[inline(always)]
        pub const fn addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Fuse word address."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Fuse word programming."]
        #[inline(always)]
        pub const fn prog(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Fuse word programming."]
        #[inline(always)]
        pub fn set_prog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Permanent programming lock."]
        #[inline(always)]
        pub const fn pplock(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Permanent programming lock."]
        #[inline(always)]
        pub fn set_pplock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Backup registers security."]
        #[inline(always)]
        pub const fn bkpsec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers security."]
        #[inline(always)]
        pub fn set_bkpsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Otpcr {
        #[inline(always)]
        fn default() -> Otpcr {
            Otpcr(0)
        }
    }
    impl core::fmt::Debug for Otpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otpcr")
                .field("addr", &self.addr())
                .field("prog", &self.prog())
                .field("pplock", &self.pplock())
                .field("bkpsec", &self.bkpsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Otpcr {{ addr: {=u16:?}, prog: {=bool:?}, pplock: {=bool:?}, bkpsec: {=bool:?} }}",
                self.addr(),
                self.prog(),
                self.pplock(),
                self.bkpsec()
            )
        }
    }
    #[doc = "BSEC OTP status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otpsr(pub u32);
    impl Otpsr {
        #[doc = "OTP busy flag."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OTP busy flag."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hide-up status."]
        #[inline(always)]
        pub const fn hideup(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hide-up status."]
        #[inline(always)]
        pub fn set_hideup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Programming permanent lock status."]
        #[inline(always)]
        pub const fn pplock(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Programming permanent lock status."]
        #[inline(always)]
        pub fn set_pplock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OTP error."]
        #[inline(always)]
        pub const fn otperr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OTP error."]
        #[inline(always)]
        pub fn set_otperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OTP security."]
        #[inline(always)]
        pub const fn otpsec(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OTP security."]
        #[inline(always)]
        pub fn set_otpsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Otpsr {
        #[inline(always)]
        fn default() -> Otpsr {
            Otpsr(0)
        }
    }
    impl core::fmt::Debug for Otpsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otpsr")
                .field("busy", &self.busy())
                .field("hideup", &self.hideup())
                .field("pplock", &self.pplock())
                .field("otperr", &self.otperr())
                .field("otpsec", &self.otpsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otpsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Otpsr {{ busy: {=bool:?}, hideup: {=bool:?}, pplock: {=bool:?}, otperr: {=bool:?}, otpsec: {=bool:?} }}" , self . busy () , self . hideup () , self . pplock () , self . otperr () , self . otpsec ())
        }
    }
    #[doc = "BSEC shadow fuse status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sfsr(pub u32);
    impl Sfsr {
        #[doc = "Shadow fuse status bits."]
        #[inline(always)]
        pub const fn sfsr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Shadow fuse status bits."]
        #[inline(always)]
        pub fn set_sfsr(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sfsr {
        #[inline(always)]
        fn default() -> Sfsr {
            Sfsr(0)
        }
    }
    impl core::fmt::Debug for Sfsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sfsr")
                .field("sfsr[0]", &self.sfsr(0usize))
                .field("sfsr[1]", &self.sfsr(1usize))
                .field("sfsr[2]", &self.sfsr(2usize))
                .field("sfsr[3]", &self.sfsr(3usize))
                .field("sfsr[4]", &self.sfsr(4usize))
                .field("sfsr[5]", &self.sfsr(5usize))
                .field("sfsr[6]", &self.sfsr(6usize))
                .field("sfsr[7]", &self.sfsr(7usize))
                .field("sfsr[8]", &self.sfsr(8usize))
                .field("sfsr[9]", &self.sfsr(9usize))
                .field("sfsr[10]", &self.sfsr(10usize))
                .field("sfsr[11]", &self.sfsr(11usize))
                .field("sfsr[12]", &self.sfsr(12usize))
                .field("sfsr[13]", &self.sfsr(13usize))
                .field("sfsr[14]", &self.sfsr(14usize))
                .field("sfsr[15]", &self.sfsr(15usize))
                .field("sfsr[16]", &self.sfsr(16usize))
                .field("sfsr[17]", &self.sfsr(17usize))
                .field("sfsr[18]", &self.sfsr(18usize))
                .field("sfsr[19]", &self.sfsr(19usize))
                .field("sfsr[20]", &self.sfsr(20usize))
                .field("sfsr[21]", &self.sfsr(21usize))
                .field("sfsr[22]", &self.sfsr(22usize))
                .field("sfsr[23]", &self.sfsr(23usize))
                .field("sfsr[24]", &self.sfsr(24usize))
                .field("sfsr[25]", &self.sfsr(25usize))
                .field("sfsr[26]", &self.sfsr(26usize))
                .field("sfsr[27]", &self.sfsr(27usize))
                .field("sfsr[28]", &self.sfsr(28usize))
                .field("sfsr[29]", &self.sfsr(29usize))
                .field("sfsr[30]", &self.sfsr(30usize))
                .field("sfsr[31]", &self.sfsr(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sfsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sfsr {{ sfsr[0]: {=bool:?}, sfsr[1]: {=bool:?}, sfsr[2]: {=bool:?}, sfsr[3]: {=bool:?}, sfsr[4]: {=bool:?}, sfsr[5]: {=bool:?}, sfsr[6]: {=bool:?}, sfsr[7]: {=bool:?}, sfsr[8]: {=bool:?}, sfsr[9]: {=bool:?}, sfsr[10]: {=bool:?}, sfsr[11]: {=bool:?}, sfsr[12]: {=bool:?}, sfsr[13]: {=bool:?}, sfsr[14]: {=bool:?}, sfsr[15]: {=bool:?}, sfsr[16]: {=bool:?}, sfsr[17]: {=bool:?}, sfsr[18]: {=bool:?}, sfsr[19]: {=bool:?}, sfsr[20]: {=bool:?}, sfsr[21]: {=bool:?}, sfsr[22]: {=bool:?}, sfsr[23]: {=bool:?}, sfsr[24]: {=bool:?}, sfsr[25]: {=bool:?}, sfsr[26]: {=bool:?}, sfsr[27]: {=bool:?}, sfsr[28]: {=bool:?}, sfsr[29]: {=bool:?}, sfsr[30]: {=bool:?}, sfsr[31]: {=bool:?} }}" , self . sfsr (0usize) , self . sfsr (1usize) , self . sfsr (2usize) , self . sfsr (3usize) , self . sfsr (4usize) , self . sfsr (5usize) , self . sfsr (6usize) , self . sfsr (7usize) , self . sfsr (8usize) , self . sfsr (9usize) , self . sfsr (10usize) , self . sfsr (11usize) , self . sfsr (12usize) , self . sfsr (13usize) , self . sfsr (14usize) , self . sfsr (15usize) , self . sfsr (16usize) , self . sfsr (17usize) , self . sfsr (18usize) , self . sfsr (19usize) , self . sfsr (20usize) , self . sfsr (21usize) , self . sfsr (22usize) , self . sfsr (23usize) , self . sfsr (24usize) , self . sfsr (25usize) , self . sfsr (26usize) , self . sfsr (27usize) , self . sfsr (28usize) , self . sfsr (29usize) , self . sfsr (30usize) , self . sfsr (31usize))
        }
    }
    #[doc = "BSEC status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Hardware key valid. When set, DHUK can be used in SAES."]
        #[inline(always)]
        pub const fn hvalid(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware key valid. When set, DHUK can be used in SAES."]
        #[inline(always)]
        pub fn set_hvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Debug request. Host debugger is requesting debug."]
        #[inline(always)]
        pub const fn dbgreq(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Debug request. Host debugger is requesting debug."]
        #[inline(always)]
        pub fn set_dbgreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Non-volatile state."]
        #[inline(always)]
        pub const fn nvstate(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "Non-volatile state."]
        #[inline(always)]
        pub fn set_nvstate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
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
                .field("hvalid", &self.hvalid())
                .field("dbgreq", &self.dbgreq())
                .field("nvstate", &self.nvstate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ hvalid: {=bool:?}, dbgreq: {=bool:?}, nvstate: {=u8:?} }}",
                self.hvalid(),
                self.dbgreq(),
                self.nvstate()
            )
        }
    }
    #[doc = "BSEC unmap register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Unmapr(pub u32);
    impl Unmapr {
        #[doc = "Unmap OTP area."]
        #[inline(always)]
        pub const fn unmap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Unmap OTP area."]
        #[inline(always)]
        pub fn set_unmap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Unmapr {
        #[inline(always)]
        fn default() -> Unmapr {
            Unmapr(0)
        }
    }
    impl core::fmt::Debug for Unmapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Unmapr").field("unmap", &self.unmap()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Unmapr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Unmapr {{ unmap: {=bool:?} }}", self.unmap())
        }
    }
    #[doc = "BSEC OTP valid status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vldr(pub u32);
    impl Vldr {
        #[doc = "OTP valid status bits."]
        #[inline(always)]
        pub const fn otpvld(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "OTP valid status bits."]
        #[inline(always)]
        pub fn set_otpvld(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Vldr {
        #[inline(always)]
        fn default() -> Vldr {
            Vldr(0)
        }
    }
    impl core::fmt::Debug for Vldr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vldr")
                .field("otpvld[0]", &self.otpvld(0usize))
                .field("otpvld[1]", &self.otpvld(1usize))
                .field("otpvld[2]", &self.otpvld(2usize))
                .field("otpvld[3]", &self.otpvld(3usize))
                .field("otpvld[4]", &self.otpvld(4usize))
                .field("otpvld[5]", &self.otpvld(5usize))
                .field("otpvld[6]", &self.otpvld(6usize))
                .field("otpvld[7]", &self.otpvld(7usize))
                .field("otpvld[8]", &self.otpvld(8usize))
                .field("otpvld[9]", &self.otpvld(9usize))
                .field("otpvld[10]", &self.otpvld(10usize))
                .field("otpvld[11]", &self.otpvld(11usize))
                .field("otpvld[12]", &self.otpvld(12usize))
                .field("otpvld[13]", &self.otpvld(13usize))
                .field("otpvld[14]", &self.otpvld(14usize))
                .field("otpvld[15]", &self.otpvld(15usize))
                .field("otpvld[16]", &self.otpvld(16usize))
                .field("otpvld[17]", &self.otpvld(17usize))
                .field("otpvld[18]", &self.otpvld(18usize))
                .field("otpvld[19]", &self.otpvld(19usize))
                .field("otpvld[20]", &self.otpvld(20usize))
                .field("otpvld[21]", &self.otpvld(21usize))
                .field("otpvld[22]", &self.otpvld(22usize))
                .field("otpvld[23]", &self.otpvld(23usize))
                .field("otpvld[24]", &self.otpvld(24usize))
                .field("otpvld[25]", &self.otpvld(25usize))
                .field("otpvld[26]", &self.otpvld(26usize))
                .field("otpvld[27]", &self.otpvld(27usize))
                .field("otpvld[28]", &self.otpvld(28usize))
                .field("otpvld[29]", &self.otpvld(29usize))
                .field("otpvld[30]", &self.otpvld(30usize))
                .field("otpvld[31]", &self.otpvld(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vldr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vldr {{ otpvld[0]: {=bool:?}, otpvld[1]: {=bool:?}, otpvld[2]: {=bool:?}, otpvld[3]: {=bool:?}, otpvld[4]: {=bool:?}, otpvld[5]: {=bool:?}, otpvld[6]: {=bool:?}, otpvld[7]: {=bool:?}, otpvld[8]: {=bool:?}, otpvld[9]: {=bool:?}, otpvld[10]: {=bool:?}, otpvld[11]: {=bool:?}, otpvld[12]: {=bool:?}, otpvld[13]: {=bool:?}, otpvld[14]: {=bool:?}, otpvld[15]: {=bool:?}, otpvld[16]: {=bool:?}, otpvld[17]: {=bool:?}, otpvld[18]: {=bool:?}, otpvld[19]: {=bool:?}, otpvld[20]: {=bool:?}, otpvld[21]: {=bool:?}, otpvld[22]: {=bool:?}, otpvld[23]: {=bool:?}, otpvld[24]: {=bool:?}, otpvld[25]: {=bool:?}, otpvld[26]: {=bool:?}, otpvld[27]: {=bool:?}, otpvld[28]: {=bool:?}, otpvld[29]: {=bool:?}, otpvld[30]: {=bool:?}, otpvld[31]: {=bool:?} }}" , self . otpvld (0usize) , self . otpvld (1usize) , self . otpvld (2usize) , self . otpvld (3usize) , self . otpvld (4usize) , self . otpvld (5usize) , self . otpvld (6usize) , self . otpvld (7usize) , self . otpvld (8usize) , self . otpvld (9usize) , self . otpvld (10usize) , self . otpvld (11usize) , self . otpvld (12usize) , self . otpvld (13usize) , self . otpvld (14usize) , self . otpvld (15usize) , self . otpvld (16usize) , self . otpvld (17usize) , self . otpvld (18usize) , self . otpvld (19usize) , self . otpvld (20usize) , self . otpvld (21usize) , self . otpvld (22usize) , self . otpvld (23usize) , self . otpvld (24usize) , self . otpvld (25usize) , self . otpvld (26usize) , self . otpvld (27usize) , self . otpvld (28usize) , self . otpvld (29usize) , self . otpvld (30usize) , self . otpvld (31usize))
        }
    }
    #[doc = "BSEC write data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdr(pub u32);
    impl Wdr {
        #[doc = "Write data value to program in OTP."]
        #[inline(always)]
        pub const fn wdata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write data value to program in OTP."]
        #[inline(always)]
        pub fn set_wdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wdr {
        #[inline(always)]
        fn default() -> Wdr {
            Wdr(0)
        }
    }
    impl core::fmt::Debug for Wdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdr").field("wdata", &self.wdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdr {{ wdata: {=u32:?} }}", self.wdata())
        }
    }
}
