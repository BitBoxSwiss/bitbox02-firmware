#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Tamper and backup registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamp {
    ptr: *mut u8,
}
unsafe impl Send for Tamp {}
unsafe impl Sync for Tamp {}
impl Tamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "TAMP control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "TAMP filter control register"]
    #[inline(always)]
    pub const fn fltcr(self) -> crate::common::Reg<regs::Fltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "TAMP interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "TAMP status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "TAMP masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "TAMP status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "monotonic counter register"]
    #[inline(always)]
    pub const fn countr(self) -> crate::common::Reg<regs::Countr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "TAMP backup register"]
    #[inline(always)]
    pub const fn bkpr(self, n: usize) -> crate::common::Reg<regs::Bkpr, crate::common::RW> {
        assert!(n < 20usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "TAMP backup register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bkpr(pub u32);
    impl Bkpr {
        #[doc = "BKP"]
        #[inline(always)]
        pub const fn bkp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "BKP"]
        #[inline(always)]
        pub fn set_bkp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bkpr {
        #[inline(always)]
        fn default() -> Bkpr {
            Bkpr(0)
        }
    }
    impl core::fmt::Debug for Bkpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bkpr").field("bkp", &self.bkp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bkpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bkpr {{ bkp: {=u32:?} }}", self.bkp())
        }
    }
    #[doc = "monotonic counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Countr(pub u32);
    impl Countr {
        #[doc = "COUNT"]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "COUNT"]
        #[inline(always)]
        pub fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Countr {
        #[inline(always)]
        fn default() -> Countr {
            Countr(0)
        }
    }
    impl core::fmt::Debug for Countr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Countr").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Countr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Countr {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Tamper detection on IN X enable"]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection on IN X enable"]
        #[inline(always)]
        pub fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X enable"]
        #[inline(always)]
        pub const fn itampe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X enable"]
        #[inline(always)]
        pub fn set_itampe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("tampe[0]", &self.tampe(0usize))
                .field("tampe[1]", &self.tampe(1usize))
                .field("tampe[2]", &self.tampe(2usize))
                .field("itampe[0]", &self.itampe(0usize))
                .field("itampe[1]", &self.itampe(1usize))
                .field("itampe[2]", &self.itampe(2usize))
                .field("itampe[3]", &self.itampe(3usize))
                .field("itampe[4]", &self.itampe(4usize))
                .field("itampe[5]", &self.itampe(5usize))
                .field("itampe[6]", &self.itampe(6usize))
                .field("itampe[7]", &self.itampe(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ tampe[0]: {=bool:?}, tampe[1]: {=bool:?}, tampe[2]: {=bool:?}, itampe[0]: {=bool:?}, itampe[1]: {=bool:?}, itampe[2]: {=bool:?}, itampe[3]: {=bool:?}, itampe[4]: {=bool:?}, itampe[5]: {=bool:?}, itampe[6]: {=bool:?}, itampe[7]: {=bool:?} }}" , self . tampe (0usize) , self . tampe (1usize) , self . tampe (2usize) , self . itampe (0usize) , self . itampe (1usize) , self . itampe (2usize) , self . itampe (3usize) , self . itampe (4usize) , self . itampe (5usize) , self . itampe (6usize) , self . itampe (7usize))
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Tamper X no erase"]
        #[inline(always)]
        pub const fn tampnoer(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X no erase"]
        #[inline(always)]
        pub fn set_tampnoer(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Tamper X mask. The tamper X interrupt must not be enabled when TAMPMSK is set."]
        #[inline(always)]
        pub const fn tampmsk(&self, n: usize) -> super::vals::Tampmsk {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Tampmsk::from_bits(val as u8)
        }
        #[doc = "Tamper X mask. The tamper X interrupt must not be enabled when TAMPMSK is set."]
        #[inline(always)]
        pub fn set_tampmsk(&mut self, n: usize, val: super::vals::Tampmsk) {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Backup registers erase"]
        #[inline(always)]
        pub const fn bkerase(&self) -> super::vals::Bkerase {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Bkerase::from_bits(val as u8)
        }
        #[doc = "Backup registers erase"]
        #[inline(always)]
        pub fn set_bkerase(&mut self, val: super::vals::Bkerase) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Active level for tamper X input"]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> super::vals::Tamptrg {
            assert!(n < 3usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Tamptrg::from_bits(val as u8)
        }
        #[doc = "Active level for tamper X input"]
        #[inline(always)]
        pub fn set_tamptrg(&mut self, n: usize, val: super::vals::Tamptrg) {
            assert!(n < 3usize);
            let offs = 24usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
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
                .field("tampnoer[0]", &self.tampnoer(0usize))
                .field("tampnoer[1]", &self.tampnoer(1usize))
                .field("tampnoer[2]", &self.tampnoer(2usize))
                .field("tampmsk[0]", &self.tampmsk(0usize))
                .field("tampmsk[1]", &self.tampmsk(1usize))
                .field("tampmsk[2]", &self.tampmsk(2usize))
                .field("bkerase", &self.bkerase())
                .field("tamptrg[0]", &self.tamptrg(0usize))
                .field("tamptrg[1]", &self.tamptrg(1usize))
                .field("tamptrg[2]", &self.tamptrg(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ tampnoer[0]: {=bool:?}, tampnoer[1]: {=bool:?}, tampnoer[2]: {=bool:?}, tampmsk[0]: {:?}, tampmsk[1]: {:?}, tampmsk[2]: {:?}, bkerase: {:?}, tamptrg[0]: {:?}, tamptrg[1]: {:?}, tamptrg[2]: {:?} }}" , self . tampnoer (0usize) , self . tampnoer (1usize) , self . tampnoer (2usize) , self . tampmsk (0usize) , self . tampmsk (1usize) , self . tampmsk (2usize) , self . bkerase () , self . tamptrg (0usize) , self . tamptrg (1usize) , self . tamptrg (2usize))
        }
    }
    #[doc = "TAMP control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Internal Tamper X no erase"]
        #[inline(always)]
        pub const fn itampnoer(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal Tamper X no erase"]
        #[inline(always)]
        pub fn set_itampnoer(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("itampnoer[0]", &self.itampnoer(0usize))
                .field("itampnoer[1]", &self.itampnoer(1usize))
                .field("itampnoer[2]", &self.itampnoer(2usize))
                .field("itampnoer[3]", &self.itampnoer(3usize))
                .field("itampnoer[4]", &self.itampnoer(4usize))
                .field("itampnoer[5]", &self.itampnoer(5usize))
                .field("itampnoer[6]", &self.itampnoer(6usize))
                .field("itampnoer[7]", &self.itampnoer(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ itampnoer[0]: {=bool:?}, itampnoer[1]: {=bool:?}, itampnoer[2]: {=bool:?}, itampnoer[3]: {=bool:?}, itampnoer[4]: {=bool:?}, itampnoer[5]: {=bool:?}, itampnoer[6]: {=bool:?}, itampnoer[7]: {=bool:?} }}" , self . itampnoer (0usize) , self . itampnoer (1usize) , self . itampnoer (2usize) , self . itampnoer (3usize) , self . itampnoer (4usize) , self . itampnoer (5usize) , self . itampnoer (6usize) , self . itampnoer (7usize))
        }
    }
    #[doc = "TAMP filter control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltcr(pub u32);
    impl Fltcr {
        #[doc = "Tamper sampling frequency. Determines the frequency at which each of the INx inputs are sampled."]
        #[inline(always)]
        pub const fn tampfreq(&self) -> super::vals::Tampfreq {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Tampfreq::from_bits(val as u8)
        }
        #[doc = "Tamper sampling frequency. Determines the frequency at which each of the INx inputs are sampled."]
        #[inline(always)]
        pub fn set_tampfreq(&mut self, val: super::vals::Tampfreq) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "INx filter count. These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the INx inputs."]
        #[inline(always)]
        pub const fn tampflt(&self) -> super::vals::Tampflt {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Tampflt::from_bits(val as u8)
        }
        #[doc = "INx filter count. These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the INx inputs."]
        #[inline(always)]
        pub fn set_tampflt(&mut self, val: super::vals::Tampflt) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "INx precharge duration. These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the INx inputs."]
        #[inline(always)]
        pub const fn tampprch(&self) -> super::vals::Tampprch {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Tampprch::from_bits(val as u8)
        }
        #[doc = "INx precharge duration. These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the INx inputs."]
        #[inline(always)]
        pub fn set_tampprch(&mut self, val: super::vals::Tampprch) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "INx pull-up disable. This bit determines if each of the TAMPx pins are precharged before each sample."]
        #[inline(always)]
        pub const fn tamppudis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "INx pull-up disable. This bit determines if each of the TAMPx pins are precharged before each sample."]
        #[inline(always)]
        pub fn set_tamppudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Fltcr {
        #[inline(always)]
        fn default() -> Fltcr {
            Fltcr(0)
        }
    }
    impl core::fmt::Debug for Fltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fltcr")
                .field("tampfreq", &self.tampfreq())
                .field("tampflt", &self.tampflt())
                .field("tampprch", &self.tampprch())
                .field("tamppudis", &self.tamppudis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fltcr {{ tampfreq: {:?}, tampflt: {:?}, tampprch: {:?}, tamppudis: {=bool:?} }}",
                self.tampfreq(),
                self.tampflt(),
                self.tampprch(),
                self.tamppudis()
            )
        }
    }
    #[doc = "TAMP interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Tamper X interrupt enable"]
        #[inline(always)]
        pub const fn tampie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_tampie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub const fn itampie(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_itampie(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("tampie[0]", &self.tampie(0usize))
                .field("tampie[1]", &self.tampie(1usize))
                .field("tampie[2]", &self.tampie(2usize))
                .field("itampie[0]", &self.itampie(0usize))
                .field("itampie[1]", &self.itampie(1usize))
                .field("itampie[2]", &self.itampie(2usize))
                .field("itampie[3]", &self.itampie(3usize))
                .field("itampie[4]", &self.itampie(4usize))
                .field("itampie[5]", &self.itampie(5usize))
                .field("itampie[6]", &self.itampie(6usize))
                .field("itampie[7]", &self.itampie(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ tampie[0]: {=bool:?}, tampie[1]: {=bool:?}, tampie[2]: {=bool:?}, itampie[0]: {=bool:?}, itampie[1]: {=bool:?}, itampie[2]: {=bool:?}, itampie[3]: {=bool:?}, itampie[4]: {=bool:?}, itampie[5]: {=bool:?}, itampie[6]: {=bool:?}, itampie[7]: {=bool:?} }}" , self . tampie (0usize) , self . tampie (1usize) , self . tampie (2usize) , self . itampie (0usize) , self . itampie (1usize) , self . itampie (2usize) , self . itampie (3usize) , self . itampie (4usize) , self . itampie (5usize) , self . itampie (6usize) , self . itampie (7usize))
        }
    }
    #[doc = "TAMP masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Tamper X interrupt masked flag"]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X interrupt masked flag"]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X interrupt masked flag"]
        #[inline(always)]
        pub const fn itampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt masked flag"]
        #[inline(always)]
        pub fn set_itampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("tampmf[0]", &self.tampmf(0usize))
                .field("tampmf[1]", &self.tampmf(1usize))
                .field("tampmf[2]", &self.tampmf(2usize))
                .field("itampmf[0]", &self.itampmf(0usize))
                .field("itampmf[1]", &self.itampmf(1usize))
                .field("itampmf[2]", &self.itampmf(2usize))
                .field("itampmf[3]", &self.itampmf(3usize))
                .field("itampmf[4]", &self.itampmf(4usize))
                .field("itampmf[5]", &self.itampmf(5usize))
                .field("itampmf[6]", &self.itampmf(6usize))
                .field("itampmf[7]", &self.itampmf(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Misr {{ tampmf[0]: {=bool:?}, tampmf[1]: {=bool:?}, tampmf[2]: {=bool:?}, itampmf[0]: {=bool:?}, itampmf[1]: {=bool:?}, itampmf[2]: {=bool:?}, itampmf[3]: {=bool:?}, itampmf[4]: {=bool:?}, itampmf[5]: {=bool:?}, itampmf[6]: {=bool:?}, itampmf[7]: {=bool:?} }}" , self . tampmf (0usize) , self . tampmf (1usize) , self . tampmf (2usize) , self . itampmf (0usize) , self . itampmf (1usize) , self . itampmf (2usize) , self . itampmf (3usize) , self . itampmf (4usize) , self . itampmf (5usize) , self . itampmf (6usize) , self . itampmf (7usize))
        }
    }
    #[doc = "TAMP status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear tamper X detection flag"]
        #[inline(always)]
        pub const fn ctampf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear tamper X detection flag"]
        #[inline(always)]
        pub fn set_ctampf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear internal tamper X detection flag"]
        #[inline(always)]
        pub const fn citampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear internal tamper X detection flag"]
        #[inline(always)]
        pub fn set_citampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field("ctampf[0]", &self.ctampf(0usize))
                .field("ctampf[1]", &self.ctampf(1usize))
                .field("ctampf[2]", &self.ctampf(2usize))
                .field("citampf[0]", &self.citampf(0usize))
                .field("citampf[1]", &self.citampf(1usize))
                .field("citampf[2]", &self.citampf(2usize))
                .field("citampf[3]", &self.citampf(3usize))
                .field("citampf[4]", &self.citampf(4usize))
                .field("citampf[5]", &self.citampf(5usize))
                .field("citampf[6]", &self.citampf(6usize))
                .field("citampf[7]", &self.citampf(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Scr {{ ctampf[0]: {=bool:?}, ctampf[1]: {=bool:?}, ctampf[2]: {=bool:?}, citampf[0]: {=bool:?}, citampf[1]: {=bool:?}, citampf[2]: {=bool:?}, citampf[3]: {=bool:?}, citampf[4]: {=bool:?}, citampf[5]: {=bool:?}, citampf[6]: {=bool:?}, citampf[7]: {=bool:?} }}" , self . ctampf (0usize) , self . ctampf (1usize) , self . ctampf (2usize) , self . citampf (0usize) , self . citampf (1usize) , self . citampf (2usize) , self . citampf (3usize) , self . citampf (4usize) , self . citampf (5usize) , self . citampf (6usize) , self . citampf (7usize))
        }
    }
    #[doc = "TAMP status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Tamper X detection flag"]
        #[inline(always)]
        pub const fn tampf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X detection flag"]
        #[inline(always)]
        pub fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X detection flag"]
        #[inline(always)]
        pub const fn itampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X detection flag"]
        #[inline(always)]
        pub fn set_itampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("tampf[0]", &self.tampf(0usize))
                .field("tampf[1]", &self.tampf(1usize))
                .field("tampf[2]", &self.tampf(2usize))
                .field("itampf[0]", &self.itampf(0usize))
                .field("itampf[1]", &self.itampf(1usize))
                .field("itampf[2]", &self.itampf(2usize))
                .field("itampf[3]", &self.itampf(3usize))
                .field("itampf[4]", &self.itampf(4usize))
                .field("itampf[5]", &self.itampf(5usize))
                .field("itampf[6]", &self.itampf(6usize))
                .field("itampf[7]", &self.itampf(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ tampf[0]: {=bool:?}, tampf[1]: {=bool:?}, tampf[2]: {=bool:?}, itampf[0]: {=bool:?}, itampf[1]: {=bool:?}, itampf[2]: {=bool:?}, itampf[3]: {=bool:?}, itampf[4]: {=bool:?}, itampf[5]: {=bool:?}, itampf[6]: {=bool:?}, itampf[7]: {=bool:?} }}" , self . tampf (0usize) , self . tampf (1usize) , self . tampf (2usize) , self . itampf (0usize) , self . itampf (1usize) , self . itampf (2usize) , self . itampf (3usize) , self . itampf (4usize) , self . itampf (5usize) , self . itampf (6usize) , self . itampf (7usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bkerase {
        _RESERVED_0 = 0x0,
        #[doc = "Reset backup registers"]
        RESET = 0x01,
    }
    impl Bkerase {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bkerase {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bkerase {
        #[inline(always)]
        fn from(val: u8) -> Bkerase {
            Bkerase::from_bits(val)
        }
    }
    impl From<Bkerase> for u8 {
        #[inline(always)]
        fn from(val: Bkerase) -> u8 {
            Bkerase::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tampflt {
        #[doc = "Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)\""]
        NO_FILTER = 0x0,
        #[doc = "Tamper event is activated after 2 consecutive samples at the active level\""]
        FILTER2 = 0x01,
        #[doc = "Tamper event is activated after 4 consecutive samples at the active level\""]
        FILTER4 = 0x02,
        #[doc = "Tamper event is activated after 8 consecutive samples at the active level\""]
        FILTER8 = 0x03,
    }
    impl Tampflt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampflt {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampflt {
        #[inline(always)]
        fn from(val: u8) -> Tampflt {
            Tampflt::from_bits(val)
        }
    }
    impl From<Tampflt> for u8 {
        #[inline(always)]
        fn from(val: Tampflt) -> u8 {
            Tampflt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tampfreq {
        #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
        HZ_1 = 0x0,
        #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
        HZ_2 = 0x01,
        #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
        HZ_4 = 0x02,
        #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
        HZ_8 = 0x03,
        #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
        HZ_16 = 0x04,
        #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
        HZ_32 = 0x05,
        #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
        HZ_64 = 0x06,
        #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
        HZ_128 = 0x07,
    }
    impl Tampfreq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampfreq {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampfreq {
        #[inline(always)]
        fn from(val: u8) -> Tampfreq {
            Tampfreq::from_bits(val)
        }
    }
    impl From<Tampfreq> for u8 {
        #[inline(always)]
        fn from(val: Tampfreq) -> u8 {
            Tampfreq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tampmsk {
        #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
        RESET_BY_SOFTWARE = 0x0,
        #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set"]
        RESET_BY_HARDWARE = 0x01,
    }
    impl Tampmsk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampmsk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampmsk {
        #[inline(always)]
        fn from(val: u8) -> Tampmsk {
            Tampmsk::from_bits(val)
        }
    }
    impl From<Tampmsk> for u8 {
        #[inline(always)]
        fn from(val: Tampmsk) -> u8 {
            Tampmsk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tampprch {
        #[doc = "1 RTCCLK cycle"]
        CYCLES1 = 0x0,
        #[doc = "2 RTCCLK cycles"]
        CYCLES2 = 0x01,
        #[doc = "4 RTCCLK cycles"]
        CYCLES4 = 0x02,
        #[doc = "8 RTCCLK cycles"]
        CYCLES8 = 0x03,
    }
    impl Tampprch {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampprch {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampprch {
        #[inline(always)]
        fn from(val: u8) -> Tampprch {
            Tampprch::from_bits(val)
        }
    }
    impl From<Tampprch> for u8 {
        #[inline(always)]
        fn from(val: Tampprch) -> u8 {
            Tampprch::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tamptrg {
        #[doc = "If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event"]
        FILTERED_LOW_OR_UNFILTERED_HIGH = 0x0,
        #[doc = "If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event"]
        FILTERED_HIGH_OR_UNFILTERED_LOW = 0x01,
    }
    impl Tamptrg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tamptrg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tamptrg {
        #[inline(always)]
        fn from(val: u8) -> Tamptrg {
            Tamptrg::from_bits(val)
        }
    }
    impl From<Tamptrg> for u8 {
        #[inline(always)]
        fn from(val: Tamptrg) -> u8 {
            Tamptrg::to_bits(val)
        }
    }
}
