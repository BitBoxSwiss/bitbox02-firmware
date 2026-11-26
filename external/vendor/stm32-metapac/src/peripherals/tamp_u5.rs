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
    #[doc = "TAMP control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "TAMP control register 2"]
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
    #[doc = "TAMP active tamper control register 1"]
    #[inline(always)]
    pub const fn atcr1(self) -> crate::common::Reg<regs::Atcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "TAMP active tamper seed register"]
    #[inline(always)]
    pub const fn atseedr(self) -> crate::common::Reg<regs::Atseedr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "TAMP active tamper output register"]
    #[inline(always)]
    pub const fn ator(self) -> crate::common::Reg<regs::Ator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "TAMP active tamper control register 2"]
    #[inline(always)]
    pub const fn atcr2(self) -> crate::common::Reg<regs::Atcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "TAMP secure mode register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "TAMP privilege mode control register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "TAMP interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "TAMP status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "TAMP non-secure masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "TAMP secure masked interrupt status register"]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Smisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "TAMP status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "TAMP monotonic counter 1 register"]
    #[inline(always)]
    pub const fn countr(self) -> crate::common::Reg<regs::Countr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "TAMP erase configuration register"]
    #[inline(always)]
    pub const fn ercfgr(self) -> crate::common::Reg<regs::Ercfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "TAMP backup X register"]
    #[inline(always)]
    pub const fn bkpr(self, n: usize) -> crate::common::Reg<regs::Bkpr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "TAMP active tamper control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr1(pub u32);
    impl Atcr1 {
        #[doc = "Tamper X active mode"]
        #[inline(always)]
        pub const fn tampam(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X active mode"]
        #[inline(always)]
        pub fn set_tampam(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Active tamper shared output X selection. The selected output must be available in the package pinout"]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Active tamper shared output X selection. The selected output must be available in the package pinout"]
        #[inline(always)]
        pub fn set_atosel(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Active tamper RTC asynchronous prescaler clock selection. These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE.. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128.. .... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
        #[inline(always)]
        pub const fn atcksel(&self) -> super::vals::Atcksel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Atcksel::from_bits(val as u8)
        }
        #[doc = "Active tamper RTC asynchronous prescaler clock selection. These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE.. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128.. .... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
        #[inline(always)]
        pub fn set_atcksel(&mut self, val: super::vals::Atcksel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Active tamper output change period. The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
        #[inline(always)]
        pub const fn atper(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Active tamper output change period. The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
        #[inline(always)]
        pub fn set_atper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Active tamper output sharing. IN1 is compared with TAMPOUTSEL1. IN2 is compared with TAMPOUTSEL2. IN3 is compared with TAMPOUTSEL3. IN4 is compared with TAMPOUTSEL4. IN5 is compared with TAMPOUTSEL5. IN6 is compared with TAMPOUTSEL6. IN7 is compared with TAMPOUTSEL7. IN8 is compared with TAMPOUTSEL8"]
        #[inline(always)]
        pub const fn atoshare(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper output sharing. IN1 is compared with TAMPOUTSEL1. IN2 is compared with TAMPOUTSEL2. IN3 is compared with TAMPOUTSEL3. IN4 is compared with TAMPOUTSEL4. IN5 is compared with TAMPOUTSEL5. IN6 is compared with TAMPOUTSEL6. IN7 is compared with TAMPOUTSEL7. IN8 is compared with TAMPOUTSEL8"]
        #[inline(always)]
        pub fn set_atoshare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Active tamper filter enable"]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper filter enable"]
        #[inline(always)]
        pub fn set_flten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Atcr1 {
        #[inline(always)]
        fn default() -> Atcr1 {
            Atcr1(0)
        }
    }
    impl core::fmt::Debug for Atcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atcr1")
                .field("tampam[0]", &self.tampam(0usize))
                .field("tampam[1]", &self.tampam(1usize))
                .field("tampam[2]", &self.tampam(2usize))
                .field("tampam[3]", &self.tampam(3usize))
                .field("tampam[4]", &self.tampam(4usize))
                .field("tampam[5]", &self.tampam(5usize))
                .field("tampam[6]", &self.tampam(6usize))
                .field("tampam[7]", &self.tampam(7usize))
                .field("atosel[0]", &self.atosel(0usize))
                .field("atosel[1]", &self.atosel(1usize))
                .field("atosel[2]", &self.atosel(2usize))
                .field("atosel[3]", &self.atosel(3usize))
                .field("atcksel", &self.atcksel())
                .field("atper", &self.atper())
                .field("atoshare", &self.atoshare())
                .field("flten", &self.flten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Atcr1 {{ tampam[0]: {=bool:?}, tampam[1]: {=bool:?}, tampam[2]: {=bool:?}, tampam[3]: {=bool:?}, tampam[4]: {=bool:?}, tampam[5]: {=bool:?}, tampam[6]: {=bool:?}, tampam[7]: {=bool:?}, atosel[0]: {=u8:?}, atosel[1]: {=u8:?}, atosel[2]: {=u8:?}, atosel[3]: {=u8:?}, atcksel: {:?}, atper: {=u8:?}, atoshare: {=bool:?}, flten: {=bool:?} }}" , self . tampam (0usize) , self . tampam (1usize) , self . tampam (2usize) , self . tampam (3usize) , self . tampam (4usize) , self . tampam (5usize) , self . tampam (6usize) , self . tampam (7usize) , self . atosel (0usize) , self . atosel (1usize) , self . atosel (2usize) , self . atosel (3usize) , self . atcksel () , self . atper () , self . atoshare () , self . flten ())
        }
    }
    #[doc = "TAMP active tamper control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr2(pub u32);
    impl Atcr2 {
        #[doc = "Active tamper shared output X selection. The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\]
in the ATCR1, and so can also be read or. written through ATCR1."]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 8usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Active tamper shared output X selection. The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\]
in the ATCR1, and so can also be read or. written through ATCR1."]
        #[inline(always)]
        pub fn set_atosel(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 8usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
    }
    impl Default for Atcr2 {
        #[inline(always)]
        fn default() -> Atcr2 {
            Atcr2(0)
        }
    }
    impl core::fmt::Debug for Atcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atcr2")
                .field("atosel[0]", &self.atosel(0usize))
                .field("atosel[1]", &self.atosel(1usize))
                .field("atosel[2]", &self.atosel(2usize))
                .field("atosel[3]", &self.atosel(3usize))
                .field("atosel[4]", &self.atosel(4usize))
                .field("atosel[5]", &self.atosel(5usize))
                .field("atosel[6]", &self.atosel(6usize))
                .field("atosel[7]", &self.atosel(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Atcr2 {{ atosel[0]: {=u8:?}, atosel[1]: {=u8:?}, atosel[2]: {=u8:?}, atosel[3]: {=u8:?}, atosel[4]: {=u8:?}, atosel[5]: {=u8:?}, atosel[6]: {=u8:?}, atosel[7]: {=u8:?} }}" , self . atosel (0usize) , self . atosel (1usize) , self . atosel (2usize) , self . atosel (3usize) , self . atosel (4usize) , self . atosel (5usize) , self . atosel (6usize) , self . atosel (7usize))
        }
    }
    #[doc = "TAMP active tamper output register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ator(pub u32);
    impl Ator {
        #[doc = "Pseudo-random generator value. This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode."]
        #[inline(always)]
        pub const fn prng(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Pseudo-random generator value. This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode."]
        #[inline(always)]
        pub fn set_prng(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Seed running flag. This flag is set by hardware when a new seed is written in the ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
        #[inline(always)]
        pub const fn seedf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Seed running flag. This flag is set by hardware when a new seed is written in the ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
        #[inline(always)]
        pub fn set_seedf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Active tamper initialization status. This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper initialization status. This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
        #[inline(always)]
        pub fn set_inits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ator {
        #[inline(always)]
        fn default() -> Ator {
            Ator(0)
        }
    }
    impl core::fmt::Debug for Ator {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ator")
                .field("prng", &self.prng())
                .field("seedf", &self.seedf())
                .field("inits", &self.inits())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ator {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ator {{ prng: {=u8:?}, seedf: {=bool:?}, inits: {=bool:?} }}",
                self.prng(),
                self.seedf(),
                self.inits()
            )
        }
    }
    #[doc = "TAMP active tamper seed register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atseedr(pub u32);
    impl Atseedr {
        #[doc = "Pseudo-random generator seed value. This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
        #[inline(always)]
        pub const fn seed(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Pseudo-random generator seed value. This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
        #[inline(always)]
        pub fn set_seed(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Atseedr {
        #[inline(always)]
        fn default() -> Atseedr {
            Atseedr(0)
        }
    }
    impl core::fmt::Debug for Atseedr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atseedr").field("seed", &self.seed()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atseedr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Atseedr {{ seed: {=u32:?} }}", self.seed())
        }
    }
    #[doc = "TAMP backup register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bkpr(pub u32);
    impl Bkpr {
        #[doc = "The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub const fn bkp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
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
    #[doc = "TAMP monotonic counter 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Countr(pub u32);
    impl Countr {
        #[doc = "This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value."]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value."]
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
    #[doc = "TAMP control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Tamper detection on INx enable"]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection on INx enable"]
        #[inline(always)]
        pub fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X enable"]
        #[inline(always)]
        pub const fn itampe(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X enable"]
        #[inline(always)]
        pub fn set_itampe(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
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
                .field("tampe[3]", &self.tampe(3usize))
                .field("tampe[4]", &self.tampe(4usize))
                .field("tampe[5]", &self.tampe(5usize))
                .field("tampe[6]", &self.tampe(6usize))
                .field("tampe[7]", &self.tampe(7usize))
                .field("itampe[0]", &self.itampe(0usize))
                .field("itampe[1]", &self.itampe(1usize))
                .field("itampe[2]", &self.itampe(2usize))
                .field("itampe[3]", &self.itampe(3usize))
                .field("itampe[4]", &self.itampe(4usize))
                .field("itampe[5]", &self.itampe(5usize))
                .field("itampe[6]", &self.itampe(6usize))
                .field("itampe[7]", &self.itampe(7usize))
                .field("itampe[8]", &self.itampe(8usize))
                .field("itampe[9]", &self.itampe(9usize))
                .field("itampe[10]", &self.itampe(10usize))
                .field("itampe[11]", &self.itampe(11usize))
                .field("itampe[12]", &self.itampe(12usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ tampe[0]: {=bool:?}, tampe[1]: {=bool:?}, tampe[2]: {=bool:?}, tampe[3]: {=bool:?}, tampe[4]: {=bool:?}, tampe[5]: {=bool:?}, tampe[6]: {=bool:?}, tampe[7]: {=bool:?}, itampe[0]: {=bool:?}, itampe[1]: {=bool:?}, itampe[2]: {=bool:?}, itampe[3]: {=bool:?}, itampe[4]: {=bool:?}, itampe[5]: {=bool:?}, itampe[6]: {=bool:?}, itampe[7]: {=bool:?}, itampe[8]: {=bool:?}, itampe[9]: {=bool:?}, itampe[10]: {=bool:?}, itampe[11]: {=bool:?}, itampe[12]: {=bool:?} }}" , self . tampe (0usize) , self . tampe (1usize) , self . tampe (2usize) , self . tampe (3usize) , self . tampe (4usize) , self . tampe (5usize) , self . tampe (6usize) , self . tampe (7usize) , self . itampe (0usize) , self . itampe (1usize) , self . itampe (2usize) , self . itampe (3usize) , self . itampe (4usize) , self . itampe (5usize) , self . itampe (6usize) , self . itampe (7usize) , self . itampe (8usize) , self . itampe (9usize) , self . itampe (10usize) , self . itampe (11usize) , self . itampe (12usize))
        }
    }
    #[doc = "TAMP control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Tamper X no erase"]
        #[inline(always)]
        pub const fn tampnoer(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X no erase"]
        #[inline(always)]
        pub fn set_tampnoer(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Tamper X mask. The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
        #[inline(always)]
        pub const fn tampmsk(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X mask. The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
        #[inline(always)]
        pub fn set_tampmsk(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Backup registers and device secrets access blocked"]
        #[inline(always)]
        pub const fn bkblock(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secrets access blocked"]
        #[inline(always)]
        pub fn set_bkblock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Backup registers and device secrets erase. Writing '1 to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0."]
        #[inline(always)]
        pub const fn bkerase(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secrets erase. Writing '1 to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0."]
        #[inline(always)]
        pub fn set_bkerase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Active level for tamper 1 input."]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> super::vals::Tamptrg {
            assert!(n < 8usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Tamptrg::from_bits(val as u8)
        }
        #[doc = "Active level for tamper 1 input."]
        #[inline(always)]
        pub fn set_tamptrg(&mut self, n: usize, val: super::vals::Tamptrg) {
            assert!(n < 8usize);
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
                .field("tampnoer[3]", &self.tampnoer(3usize))
                .field("tampnoer[4]", &self.tampnoer(4usize))
                .field("tampnoer[5]", &self.tampnoer(5usize))
                .field("tampnoer[6]", &self.tampnoer(6usize))
                .field("tampnoer[7]", &self.tampnoer(7usize))
                .field("tampmsk[0]", &self.tampmsk(0usize))
                .field("tampmsk[1]", &self.tampmsk(1usize))
                .field("tampmsk[2]", &self.tampmsk(2usize))
                .field("bkblock", &self.bkblock())
                .field("bkerase", &self.bkerase())
                .field("tamptrg[0]", &self.tamptrg(0usize))
                .field("tamptrg[1]", &self.tamptrg(1usize))
                .field("tamptrg[2]", &self.tamptrg(2usize))
                .field("tamptrg[3]", &self.tamptrg(3usize))
                .field("tamptrg[4]", &self.tamptrg(4usize))
                .field("tamptrg[5]", &self.tamptrg(5usize))
                .field("tamptrg[6]", &self.tamptrg(6usize))
                .field("tamptrg[7]", &self.tamptrg(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ tampnoer[0]: {=bool:?}, tampnoer[1]: {=bool:?}, tampnoer[2]: {=bool:?}, tampnoer[3]: {=bool:?}, tampnoer[4]: {=bool:?}, tampnoer[5]: {=bool:?}, tampnoer[6]: {=bool:?}, tampnoer[7]: {=bool:?}, tampmsk[0]: {=bool:?}, tampmsk[1]: {=bool:?}, tampmsk[2]: {=bool:?}, bkblock: {=bool:?}, bkerase: {=bool:?}, tamptrg[0]: {:?}, tamptrg[1]: {:?}, tamptrg[2]: {:?}, tamptrg[3]: {:?}, tamptrg[4]: {:?}, tamptrg[5]: {:?}, tamptrg[6]: {:?}, tamptrg[7]: {:?} }}" , self . tampnoer (0usize) , self . tampnoer (1usize) , self . tampnoer (2usize) , self . tampnoer (3usize) , self . tampnoer (4usize) , self . tampnoer (5usize) , self . tampnoer (6usize) , self . tampnoer (7usize) , self . tampmsk (0usize) , self . tampmsk (1usize) , self . tampmsk (2usize) , self . bkblock () , self . bkerase () , self . tamptrg (0usize) , self . tamptrg (1usize) , self . tamptrg (2usize) , self . tamptrg (3usize) , self . tamptrg (4usize) , self . tamptrg (5usize) , self . tamptrg (6usize) , self . tamptrg (7usize))
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
            assert!(n < 13usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal Tamper X no erase"]
        #[inline(always)]
        pub fn set_itampnoer(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
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
                .field("itampnoer[8]", &self.itampnoer(8usize))
                .field("itampnoer[9]", &self.itampnoer(9usize))
                .field("itampnoer[10]", &self.itampnoer(10usize))
                .field("itampnoer[11]", &self.itampnoer(11usize))
                .field("itampnoer[12]", &self.itampnoer(12usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ itampnoer[0]: {=bool:?}, itampnoer[1]: {=bool:?}, itampnoer[2]: {=bool:?}, itampnoer[3]: {=bool:?}, itampnoer[4]: {=bool:?}, itampnoer[5]: {=bool:?}, itampnoer[6]: {=bool:?}, itampnoer[7]: {=bool:?}, itampnoer[8]: {=bool:?}, itampnoer[9]: {=bool:?}, itampnoer[10]: {=bool:?}, itampnoer[11]: {=bool:?}, itampnoer[12]: {=bool:?} }}" , self . itampnoer (0usize) , self . itampnoer (1usize) , self . itampnoer (2usize) , self . itampnoer (3usize) , self . itampnoer (4usize) , self . itampnoer (5usize) , self . itampnoer (6usize) , self . itampnoer (7usize) , self . itampnoer (8usize) , self . itampnoer (9usize) , self . itampnoer (10usize) , self . itampnoer (11usize) , self . itampnoer (12usize))
        }
    }
    #[doc = "TAMP erase configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ercfgr(pub u32);
    impl Ercfgr {
        #[doc = "Configurable device secrets configuration"]
        #[inline(always)]
        pub const fn ercfg0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configurable device secrets configuration"]
        #[inline(always)]
        pub fn set_ercfg0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ercfgr {
        #[inline(always)]
        fn default() -> Ercfgr {
            Ercfgr(0)
        }
    }
    impl core::fmt::Debug for Ercfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ercfgr").field("ercfg0", &self.ercfg0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ercfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ercfgr {{ ercfg0: {=bool:?} }}", self.ercfg0())
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
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_tampie(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub const fn itampie(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_itampie(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
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
                .field("tampie[3]", &self.tampie(3usize))
                .field("tampie[4]", &self.tampie(4usize))
                .field("tampie[5]", &self.tampie(5usize))
                .field("tampie[6]", &self.tampie(6usize))
                .field("tampie[7]", &self.tampie(7usize))
                .field("itampie[0]", &self.itampie(0usize))
                .field("itampie[1]", &self.itampie(1usize))
                .field("itampie[2]", &self.itampie(2usize))
                .field("itampie[3]", &self.itampie(3usize))
                .field("itampie[4]", &self.itampie(4usize))
                .field("itampie[5]", &self.itampie(5usize))
                .field("itampie[6]", &self.itampie(6usize))
                .field("itampie[7]", &self.itampie(7usize))
                .field("itampie[8]", &self.itampie(8usize))
                .field("itampie[9]", &self.itampie(9usize))
                .field("itampie[10]", &self.itampie(10usize))
                .field("itampie[11]", &self.itampie(11usize))
                .field("itampie[12]", &self.itampie(12usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ tampie[0]: {=bool:?}, tampie[1]: {=bool:?}, tampie[2]: {=bool:?}, tampie[3]: {=bool:?}, tampie[4]: {=bool:?}, tampie[5]: {=bool:?}, tampie[6]: {=bool:?}, tampie[7]: {=bool:?}, itampie[0]: {=bool:?}, itampie[1]: {=bool:?}, itampie[2]: {=bool:?}, itampie[3]: {=bool:?}, itampie[4]: {=bool:?}, itampie[5]: {=bool:?}, itampie[6]: {=bool:?}, itampie[7]: {=bool:?}, itampie[8]: {=bool:?}, itampie[9]: {=bool:?}, itampie[10]: {=bool:?}, itampie[11]: {=bool:?}, itampie[12]: {=bool:?} }}" , self . tampie (0usize) , self . tampie (1usize) , self . tampie (2usize) , self . tampie (3usize) , self . tampie (4usize) , self . tampie (5usize) , self . tampie (6usize) , self . tampie (7usize) , self . itampie (0usize) , self . itampie (1usize) , self . itampie (2usize) , self . itampie (3usize) , self . itampie (4usize) , self . itampie (5usize) , self . itampie (6usize) , self . itampie (7usize) , self . itampie (8usize) , self . itampie (9usize) , self . itampie (10usize) , self . itampie (11usize) , self . itampie (12usize))
        }
    }
    #[doc = "TAMP non-secure masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "TAMPx non-secure interrupt masked flag. This flag is set by hardware when the tamper X non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx non-secure interrupt masked flag. This flag is set by hardware when the tamper X non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X non-secure interrupt masked flag. This flag is set by hardware when the internal tamper X non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itampmf(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X non-secure interrupt masked flag. This flag is set by hardware when the internal tamper X non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itampmf(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
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
                .field("tampmf[3]", &self.tampmf(3usize))
                .field("tampmf[4]", &self.tampmf(4usize))
                .field("tampmf[5]", &self.tampmf(5usize))
                .field("tampmf[6]", &self.tampmf(6usize))
                .field("tampmf[7]", &self.tampmf(7usize))
                .field("itampmf[0]", &self.itampmf(0usize))
                .field("itampmf[1]", &self.itampmf(1usize))
                .field("itampmf[2]", &self.itampmf(2usize))
                .field("itampmf[3]", &self.itampmf(3usize))
                .field("itampmf[4]", &self.itampmf(4usize))
                .field("itampmf[5]", &self.itampmf(5usize))
                .field("itampmf[6]", &self.itampmf(6usize))
                .field("itampmf[7]", &self.itampmf(7usize))
                .field("itampmf[8]", &self.itampmf(8usize))
                .field("itampmf[9]", &self.itampmf(9usize))
                .field("itampmf[10]", &self.itampmf(10usize))
                .field("itampmf[11]", &self.itampmf(11usize))
                .field("itampmf[12]", &self.itampmf(12usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Misr {{ tampmf[0]: {=bool:?}, tampmf[1]: {=bool:?}, tampmf[2]: {=bool:?}, tampmf[3]: {=bool:?}, tampmf[4]: {=bool:?}, tampmf[5]: {=bool:?}, tampmf[6]: {=bool:?}, tampmf[7]: {=bool:?}, itampmf[0]: {=bool:?}, itampmf[1]: {=bool:?}, itampmf[2]: {=bool:?}, itampmf[3]: {=bool:?}, itampmf[4]: {=bool:?}, itampmf[5]: {=bool:?}, itampmf[6]: {=bool:?}, itampmf[7]: {=bool:?}, itampmf[8]: {=bool:?}, itampmf[9]: {=bool:?}, itampmf[10]: {=bool:?}, itampmf[11]: {=bool:?}, itampmf[12]: {=bool:?} }}" , self . tampmf (0usize) , self . tampmf (1usize) , self . tampmf (2usize) , self . tampmf (3usize) , self . tampmf (4usize) , self . tampmf (5usize) , self . tampmf (6usize) , self . tampmf (7usize) , self . itampmf (0usize) , self . itampmf (1usize) , self . itampmf (2usize) , self . itampmf (3usize) , self . itampmf (4usize) , self . itampmf (5usize) , self . itampmf (6usize) , self . itampmf (7usize) , self . itampmf (8usize) , self . itampmf (9usize) , self . itampmf (10usize) , self . itampmf (11usize) , self . itampmf (12usize))
        }
    }
    #[doc = "TAMP privilege mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Monotonic counter 1 privilege protection"]
        #[inline(always)]
        pub const fn cnt1priv(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Monotonic counter 1 privilege protection"]
        #[inline(always)]
        pub fn set_cnt1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup registers zone 1 privilege protection"]
        #[inline(always)]
        pub const fn bkprwpriv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 1 privilege protection"]
        #[inline(always)]
        pub fn set_bkprwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Backup registers zone 2 privilege protection"]
        #[inline(always)]
        pub const fn bkpwpriv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 2 privilege protection"]
        #[inline(always)]
        pub fn set_bkpwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper privilege protection (excluding backup registers). Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub const fn tamppriv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper privilege protection (excluding backup registers). Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub fn set_tamppriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("cnt1priv", &self.cnt1priv())
                .field("bkprwpriv", &self.bkprwpriv())
                .field("bkpwpriv", &self.bkpwpriv())
                .field("tamppriv", &self.tamppriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ cnt1priv: {=bool:?}, bkprwpriv: {=bool:?}, bkpwpriv: {=bool:?}, tamppriv: {=bool:?} }}",
                self.cnt1priv(),
                self.bkprwpriv(),
                self.bkpwpriv(),
                self.tamppriv()
            )
        }
    }
    #[doc = "TAMP status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the SR register."]
        #[inline(always)]
        pub const fn ctampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the SR register."]
        #[inline(always)]
        pub fn set_ctampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear ITAMPx detection flag. Writing 1 in this bit clears the ITAMPxF bit in the SR register."]
        #[inline(always)]
        pub const fn citampf(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMPx detection flag. Writing 1 in this bit clears the ITAMPxF bit in the SR register."]
        #[inline(always)]
        pub fn set_citampf(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
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
                .field("ctampf[3]", &self.ctampf(3usize))
                .field("ctampf[4]", &self.ctampf(4usize))
                .field("ctampf[5]", &self.ctampf(5usize))
                .field("ctampf[6]", &self.ctampf(6usize))
                .field("ctampf[7]", &self.ctampf(7usize))
                .field("citampf[0]", &self.citampf(0usize))
                .field("citampf[1]", &self.citampf(1usize))
                .field("citampf[2]", &self.citampf(2usize))
                .field("citampf[3]", &self.citampf(3usize))
                .field("citampf[4]", &self.citampf(4usize))
                .field("citampf[5]", &self.citampf(5usize))
                .field("citampf[6]", &self.citampf(6usize))
                .field("citampf[7]", &self.citampf(7usize))
                .field("citampf[8]", &self.citampf(8usize))
                .field("citampf[9]", &self.citampf(9usize))
                .field("citampf[10]", &self.citampf(10usize))
                .field("citampf[11]", &self.citampf(11usize))
                .field("citampf[12]", &self.citampf(12usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Scr {{ ctampf[0]: {=bool:?}, ctampf[1]: {=bool:?}, ctampf[2]: {=bool:?}, ctampf[3]: {=bool:?}, ctampf[4]: {=bool:?}, ctampf[5]: {=bool:?}, ctampf[6]: {=bool:?}, ctampf[7]: {=bool:?}, citampf[0]: {=bool:?}, citampf[1]: {=bool:?}, citampf[2]: {=bool:?}, citampf[3]: {=bool:?}, citampf[4]: {=bool:?}, citampf[5]: {=bool:?}, citampf[6]: {=bool:?}, citampf[7]: {=bool:?}, citampf[8]: {=bool:?}, citampf[9]: {=bool:?}, citampf[10]: {=bool:?}, citampf[11]: {=bool:?}, citampf[12]: {=bool:?} }}" , self . ctampf (0usize) , self . ctampf (1usize) , self . ctampf (2usize) , self . ctampf (3usize) , self . ctampf (4usize) , self . ctampf (5usize) , self . ctampf (6usize) , self . ctampf (7usize) , self . citampf (0usize) , self . citampf (1usize) , self . citampf (2usize) , self . citampf (3usize) , self . citampf (4usize) , self . citampf (5usize) , self . citampf (6usize) , self . citampf (7usize) , self . citampf (8usize) , self . citampf (9usize) , self . citampf (10usize) , self . citampf (11usize) , self . citampf (12usize))
        }
    }
    #[doc = "TAMP secure mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Backup registers read/write protection offset. Protection zone 1 is defined for backup registers from BKP0R to BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub const fn bkprwsec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers read/write protection offset. Protection zone 1 is defined for backup registers from BKP0R to BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub fn set_bkprwsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Monotonic counter 1 secure protection"]
        #[inline(always)]
        pub const fn cnt1sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Monotonic counter 1 secure protection"]
        #[inline(always)]
        pub fn set_cnt1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup registers write protection offset. Protection zone 2 is defined for backup registers from BKPyR (y = BKPRWSEC, from 0 to 128) to BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSECBKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub const fn bkpwsec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers write protection offset. Protection zone 2 is defined for backup registers from BKPyR (y = BKPRWSEC, from 0 to 128) to BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSECBKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub fn set_bkpwsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Boot hardware key lock. This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub const fn bhklock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Boot hardware key lock. This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub fn set_bhklock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper protection (excluding monotonic counters and backup registers). Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub const fn tampsec(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper protection (excluding monotonic counters and backup registers). Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub fn set_tampsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("bkprwsec", &self.bkprwsec())
                .field("cnt1sec", &self.cnt1sec())
                .field("bkpwsec", &self.bkpwsec())
                .field("bhklock", &self.bhklock())
                .field("tampsec", &self.tampsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr {{ bkprwsec: {=u8:?}, cnt1sec: {=bool:?}, bkpwsec: {=u8:?}, bhklock: {=bool:?}, tampsec: {=bool:?} }}" , self . bkprwsec () , self . cnt1sec () , self . bkpwsec () , self . bhklock () , self . tampsec ())
        }
    }
    #[doc = "TAMP secure masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smisr(pub u32);
    impl Smisr {
        #[doc = "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper X secure interrupt is raised."]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper X secure interrupt is raised."]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X secure interrupt masked flag. This flag is set by hardware when the internal tamper X secure interrupt is raised."]
        #[inline(always)]
        pub const fn itampmf(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X secure interrupt masked flag. This flag is set by hardware when the internal tamper X secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itampmf(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Smisr {
        #[inline(always)]
        fn default() -> Smisr {
            Smisr(0)
        }
    }
    impl core::fmt::Debug for Smisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smisr")
                .field("tampmf[0]", &self.tampmf(0usize))
                .field("tampmf[1]", &self.tampmf(1usize))
                .field("tampmf[2]", &self.tampmf(2usize))
                .field("tampmf[3]", &self.tampmf(3usize))
                .field("tampmf[4]", &self.tampmf(4usize))
                .field("tampmf[5]", &self.tampmf(5usize))
                .field("tampmf[6]", &self.tampmf(6usize))
                .field("tampmf[7]", &self.tampmf(7usize))
                .field("itampmf[0]", &self.itampmf(0usize))
                .field("itampmf[1]", &self.itampmf(1usize))
                .field("itampmf[2]", &self.itampmf(2usize))
                .field("itampmf[3]", &self.itampmf(3usize))
                .field("itampmf[4]", &self.itampmf(4usize))
                .field("itampmf[5]", &self.itampmf(5usize))
                .field("itampmf[6]", &self.itampmf(6usize))
                .field("itampmf[7]", &self.itampmf(7usize))
                .field("itampmf[8]", &self.itampmf(8usize))
                .field("itampmf[9]", &self.itampmf(9usize))
                .field("itampmf[10]", &self.itampmf(10usize))
                .field("itampmf[11]", &self.itampmf(11usize))
                .field("itampmf[12]", &self.itampmf(12usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smisr {{ tampmf[0]: {=bool:?}, tampmf[1]: {=bool:?}, tampmf[2]: {=bool:?}, tampmf[3]: {=bool:?}, tampmf[4]: {=bool:?}, tampmf[5]: {=bool:?}, tampmf[6]: {=bool:?}, tampmf[7]: {=bool:?}, itampmf[0]: {=bool:?}, itampmf[1]: {=bool:?}, itampmf[2]: {=bool:?}, itampmf[3]: {=bool:?}, itampmf[4]: {=bool:?}, itampmf[5]: {=bool:?}, itampmf[6]: {=bool:?}, itampmf[7]: {=bool:?}, itampmf[8]: {=bool:?}, itampmf[9]: {=bool:?}, itampmf[10]: {=bool:?}, itampmf[11]: {=bool:?}, itampmf[12]: {=bool:?} }}" , self . tampmf (0usize) , self . tampmf (1usize) , self . tampmf (2usize) , self . tampmf (3usize) , self . tampmf (4usize) , self . tampmf (5usize) , self . tampmf (6usize) , self . tampmf (7usize) , self . itampmf (0usize) , self . itampmf (1usize) , self . itampmf (2usize) , self . itampmf (3usize) , self . itampmf (4usize) , self . itampmf (5usize) , self . itampmf (6usize) , self . itampmf (7usize) , self . itampmf (8usize) , self . itampmf (9usize) , self . itampmf (10usize) , self . itampmf (11usize) , self . itampmf (12usize))
        }
    }
    #[doc = "TAMP status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input."]
        #[inline(always)]
        pub const fn tampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input."]
        #[inline(always)]
        pub fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X flag. This flag is set by hardware when a tamper detection event is detected on the internal tamper X."]
        #[inline(always)]
        pub const fn itampf(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X flag. This flag is set by hardware when a tamper detection event is detected on the internal tamper X."]
        #[inline(always)]
        pub fn set_itampf(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
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
                .field("tampf[3]", &self.tampf(3usize))
                .field("tampf[4]", &self.tampf(4usize))
                .field("tampf[5]", &self.tampf(5usize))
                .field("tampf[6]", &self.tampf(6usize))
                .field("tampf[7]", &self.tampf(7usize))
                .field("itampf[0]", &self.itampf(0usize))
                .field("itampf[1]", &self.itampf(1usize))
                .field("itampf[2]", &self.itampf(2usize))
                .field("itampf[3]", &self.itampf(3usize))
                .field("itampf[4]", &self.itampf(4usize))
                .field("itampf[5]", &self.itampf(5usize))
                .field("itampf[6]", &self.itampf(6usize))
                .field("itampf[7]", &self.itampf(7usize))
                .field("itampf[8]", &self.itampf(8usize))
                .field("itampf[9]", &self.itampf(9usize))
                .field("itampf[10]", &self.itampf(10usize))
                .field("itampf[11]", &self.itampf(11usize))
                .field("itampf[12]", &self.itampf(12usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ tampf[0]: {=bool:?}, tampf[1]: {=bool:?}, tampf[2]: {=bool:?}, tampf[3]: {=bool:?}, tampf[4]: {=bool:?}, tampf[5]: {=bool:?}, tampf[6]: {=bool:?}, tampf[7]: {=bool:?}, itampf[0]: {=bool:?}, itampf[1]: {=bool:?}, itampf[2]: {=bool:?}, itampf[3]: {=bool:?}, itampf[4]: {=bool:?}, itampf[5]: {=bool:?}, itampf[6]: {=bool:?}, itampf[7]: {=bool:?}, itampf[8]: {=bool:?}, itampf[9]: {=bool:?}, itampf[10]: {=bool:?}, itampf[11]: {=bool:?}, itampf[12]: {=bool:?} }}" , self . tampf (0usize) , self . tampf (1usize) , self . tampf (2usize) , self . tampf (3usize) , self . tampf (4usize) , self . tampf (5usize) , self . tampf (6usize) , self . tampf (7usize) , self . itampf (0usize) , self . itampf (1usize) , self . itampf (2usize) , self . itampf (3usize) , self . itampf (4usize) , self . itampf (5usize) , self . itampf (6usize) , self . itampf (7usize) , self . itampf (8usize) , self . itampf (9usize) , self . itampf (10usize) , self . itampf (11usize) , self . itampf (12usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Atcksel {
        #[doc = "RTCCLK is selected"]
        DIV1 = 0x0,
        #[doc = "RTCCLK/2 is selected when (PREDIV_A+1) = 128 (actually selects 1st flip flop output)"]
        DIV2 = 0x01,
        #[doc = "RTCCLK/4 is selected when (PREDIV_A+1) = 128 (actually selects 2nd flip flop output)"]
        DIV4 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "RTCCLK/128 is selected when (PREDIV_A+1) = 128 (actually selects 7th flip flop output)"]
        DIV128 = 0x07,
    }
    impl Atcksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Atcksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Atcksel {
        #[inline(always)]
        fn from(val: u8) -> Atcksel {
            Atcksel::from_bits(val)
        }
    }
    impl From<Atcksel> for u8 {
        #[inline(always)]
        fn from(val: Atcksel) -> u8 {
            Atcksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tampflt {
        #[doc = "Tamper event is activated on edge of INx input transitions to the active level (no internal pull-up on INx input)."]
        NO_FILTER = 0x0,
        #[doc = "Tamper event is activated after 2 consecutive samples at the active level."]
        FILTER2 = 0x01,
        #[doc = "Tamper event is activated after 4 consecutive samples at the active level."]
        FILTER4 = 0x02,
        #[doc = "Tamper event is activated after 8 consecutive samples at the active level."]
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
        #[doc = "If TAMPFLT 00 Tamper 2 input staying low triggers a tamper detection event."]
        FILTERED_LOW_OR_UNFILTERED_HIGH = 0x0,
        #[doc = "If TAMPFLT 00 Tamper 2 input staying high triggers a tamper detection event."]
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
