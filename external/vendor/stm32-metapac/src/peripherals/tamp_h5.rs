#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Tamper and backup."]
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
    #[doc = "TAMP control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "TAMP control register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "TAMP control register 3."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "TAMP filter control register."]
    #[inline(always)]
    pub const fn fltcr(self) -> crate::common::Reg<regs::Fltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "TAMP active tamper control register 1."]
    #[inline(always)]
    pub const fn atcr1(self) -> crate::common::Reg<regs::Atcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "TAMP active tamper seed register."]
    #[inline(always)]
    pub const fn atseedr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "TAMP active tamper output register."]
    #[inline(always)]
    pub const fn ator(self) -> crate::common::Reg<regs::Ator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "TAMP active tamper control register 2."]
    #[inline(always)]
    pub const fn atcr2(self) -> crate::common::Reg<regs::Atcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "TAMP secure mode register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "TAMP privilege mode control register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "TAMP interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "TAMP status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "TAMP non-secure masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "TAMP secure masked interrupt status register."]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Smisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "TAMP status clear register."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "TAMP monotonic counter 1 register."]
    #[inline(always)]
    pub const fn count1r(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "TAMP option register."]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::Or, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "TAMP resources protection configuration register."]
    #[inline(always)]
    pub const fn rpcfgr(self) -> crate::common::Reg<regs::Rpcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "TAMP backup x register. (x=0-31)"]
    #[inline(always)]
    pub const fn bkpr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "TAMP active tamper control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr1(pub u32);
    impl Atcr1 {
        #[doc = "Tamper x active mode. (x=1-8)"]
        #[inline(always)]
        pub const fn tampam(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper x active mode. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampam(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Active tamper shared output x selection The selected output must be available in the package pinout. (x=1-4)"]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Active tamper shared output x selection The selected output must be available in the package pinout. (x=1-4)"]
        #[inline(always)]
        pub fn set_atosel(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
        #[inline(always)]
        pub const fn atcksel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
        #[inline(always)]
        pub fn set_atcksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to."]
        #[inline(always)]
        pub const fn atper(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to."]
        #[inline(always)]
        pub fn set_atper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8."]
        #[inline(always)]
        pub const fn atoshare(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8."]
        #[inline(always)]
        pub fn set_atoshare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Active tamper filter enable."]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper filter enable."]
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
            defmt :: write ! (f , "Atcr1 {{ tampam[0]: {=bool:?}, tampam[1]: {=bool:?}, tampam[2]: {=bool:?}, tampam[3]: {=bool:?}, tampam[4]: {=bool:?}, tampam[5]: {=bool:?}, tampam[6]: {=bool:?}, tampam[7]: {=bool:?}, atosel[0]: {=u8:?}, atosel[1]: {=u8:?}, atosel[2]: {=u8:?}, atosel[3]: {=u8:?}, atcksel: {=u8:?}, atper: {=u8:?}, atoshare: {=bool:?}, flten: {=bool:?} }}" , self . tampam (0usize) , self . tampam (1usize) , self . tampam (2usize) , self . tampam (3usize) , self . tampam (4usize) , self . tampam (5usize) , self . tampam (6usize) , self . tampam (7usize) , self . atosel (0usize) , self . atosel (1usize) , self . atosel (2usize) , self . atosel (3usize) , self . atcksel () , self . atper () , self . atoshare () , self . flten ())
        }
    }
    #[doc = "TAMP active tamper control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr2(pub u32);
    impl Atcr2 {
        #[doc = "Active tamper shared output x selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSELx\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1. (x=1-8)"]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 8usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Active tamper shared output x selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSELx\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1. (x=1-8)"]
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
    #[doc = "TAMP active tamper output register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ator(pub u32);
    impl Ator {
        #[doc = "Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode."]
        #[inline(always)]
        pub const fn prng(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode."]
        #[inline(always)]
        pub fn set_prng(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
        #[inline(always)]
        pub const fn seedf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
        #[inline(always)]
        pub fn set_seedf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
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
    #[doc = "TAMP control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Tamper detection on TAMP_INx enable. (x=1-8)"]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection on TAMP_INx enable. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 enable."]
        #[inline(always)]
        pub const fn itamp1e(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 enable."]
        #[inline(always)]
        pub fn set_itamp1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 enable."]
        #[inline(always)]
        pub const fn itamp2e(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 enable."]
        #[inline(always)]
        pub fn set_itamp2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 enable."]
        #[inline(always)]
        pub const fn itamp3e(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 enable."]
        #[inline(always)]
        pub fn set_itamp3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 enable."]
        #[inline(always)]
        pub const fn itamp4e(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 enable."]
        #[inline(always)]
        pub fn set_itamp4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 enable."]
        #[inline(always)]
        pub const fn itamp5e(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 enable."]
        #[inline(always)]
        pub fn set_itamp5e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 enable."]
        #[inline(always)]
        pub const fn itamp6e(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 enable."]
        #[inline(always)]
        pub fn set_itamp6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 enable."]
        #[inline(always)]
        pub const fn itamp7e(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 enable."]
        #[inline(always)]
        pub fn set_itamp7e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 enable."]
        #[inline(always)]
        pub const fn itamp8e(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 enable."]
        #[inline(always)]
        pub fn set_itamp8e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Internal tamper 9 enable."]
        #[inline(always)]
        pub const fn itamp9e(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 enable."]
        #[inline(always)]
        pub fn set_itamp9e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 enable."]
        #[inline(always)]
        pub const fn itamp11e(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 enable."]
        #[inline(always)]
        pub fn set_itamp11e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Internal tamper 12 enable."]
        #[inline(always)]
        pub const fn itamp12e(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 12 enable."]
        #[inline(always)]
        pub fn set_itamp12e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Internal tamper 13 enable."]
        #[inline(always)]
        pub const fn itamp13e(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 13 enable."]
        #[inline(always)]
        pub fn set_itamp13e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Internal tamper 15 enable."]
        #[inline(always)]
        pub const fn itamp15e(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 15 enable."]
        #[inline(always)]
        pub fn set_itamp15e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("itamp1e", &self.itamp1e())
                .field("itamp2e", &self.itamp2e())
                .field("itamp3e", &self.itamp3e())
                .field("itamp4e", &self.itamp4e())
                .field("itamp5e", &self.itamp5e())
                .field("itamp6e", &self.itamp6e())
                .field("itamp7e", &self.itamp7e())
                .field("itamp8e", &self.itamp8e())
                .field("itamp9e", &self.itamp9e())
                .field("itamp11e", &self.itamp11e())
                .field("itamp12e", &self.itamp12e())
                .field("itamp13e", &self.itamp13e())
                .field("itamp15e", &self.itamp15e())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ tampe[0]: {=bool:?}, tampe[1]: {=bool:?}, tampe[2]: {=bool:?}, tampe[3]: {=bool:?}, tampe[4]: {=bool:?}, tampe[5]: {=bool:?}, tampe[6]: {=bool:?}, tampe[7]: {=bool:?}, itamp1e: {=bool:?}, itamp2e: {=bool:?}, itamp3e: {=bool:?}, itamp4e: {=bool:?}, itamp5e: {=bool:?}, itamp6e: {=bool:?}, itamp7e: {=bool:?}, itamp8e: {=bool:?}, itamp9e: {=bool:?}, itamp11e: {=bool:?}, itamp12e: {=bool:?}, itamp13e: {=bool:?}, itamp15e: {=bool:?} }}" , self . tampe (0usize) , self . tampe (1usize) , self . tampe (2usize) , self . tampe (3usize) , self . tampe (4usize) , self . tampe (5usize) , self . tampe (6usize) , self . tampe (7usize) , self . itamp1e () , self . itamp2e () , self . itamp3e () , self . itamp4e () , self . itamp5e () , self . itamp6e () , self . itamp7e () , self . itamp8e () , self . itamp9e () , self . itamp11e () , self . itamp12e () , self . itamp13e () , self . itamp15e ())
        }
    }
    #[doc = "TAMP control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Tamper x potential mode. (x=1-8)"]
        #[inline(always)]
        pub const fn tamppom(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper x potential mode. (x=1-8)"]
        #[inline(always)]
        pub fn set_tamppom(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Tamper x mask. The tamper x interrupt must not be enabled when TAMPxMSK is set. (x=1-3)"]
        #[inline(always)]
        pub const fn tampmsk(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper x mask. The tamper x interrupt must not be enabled when TAMPxMSK is set. (x=1-3)"]
        #[inline(always)]
        pub fn set_tampmsk(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Backup registers and device secrets access blocked."]
        #[inline(always)]
        pub const fn bkblock(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secrets access blocked."]
        #[inline(always)]
        pub fn set_bkblock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Backup registers and device secrets erase Writing ‘1’ to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0."]
        #[inline(always)]
        pub const fn bkerase(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secrets erase Writing ‘1’ to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0."]
        #[inline(always)]
        pub fn set_bkerase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Active level for tamper x input If TAMPFLT = 00 Tamper x input rising edge triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge triggers a tamper detection event. (x=1-8)"]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Active level for tamper x input If TAMPFLT = 00 Tamper x input rising edge triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge triggers a tamper detection event. (x=1-8)"]
        #[inline(always)]
        pub fn set_tamptrg(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 24usize + n * 1usize;
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
                .field("tamppom[0]", &self.tamppom(0usize))
                .field("tamppom[1]", &self.tamppom(1usize))
                .field("tamppom[2]", &self.tamppom(2usize))
                .field("tamppom[3]", &self.tamppom(3usize))
                .field("tamppom[4]", &self.tamppom(4usize))
                .field("tamppom[5]", &self.tamppom(5usize))
                .field("tamppom[6]", &self.tamppom(6usize))
                .field("tamppom[7]", &self.tamppom(7usize))
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
            defmt :: write ! (f , "Cr2 {{ tamppom[0]: {=bool:?}, tamppom[1]: {=bool:?}, tamppom[2]: {=bool:?}, tamppom[3]: {=bool:?}, tamppom[4]: {=bool:?}, tamppom[5]: {=bool:?}, tamppom[6]: {=bool:?}, tamppom[7]: {=bool:?}, tampmsk[0]: {=bool:?}, tampmsk[1]: {=bool:?}, tampmsk[2]: {=bool:?}, bkblock: {=bool:?}, bkerase: {=bool:?}, tamptrg[0]: {=bool:?}, tamptrg[1]: {=bool:?}, tamptrg[2]: {=bool:?}, tamptrg[3]: {=bool:?}, tamptrg[4]: {=bool:?}, tamptrg[5]: {=bool:?}, tamptrg[6]: {=bool:?}, tamptrg[7]: {=bool:?} }}" , self . tamppom (0usize) , self . tamppom (1usize) , self . tamppom (2usize) , self . tamppom (3usize) , self . tamppom (4usize) , self . tamppom (5usize) , self . tamppom (6usize) , self . tamppom (7usize) , self . tampmsk (0usize) , self . tampmsk (1usize) , self . tampmsk (2usize) , self . bkblock () , self . bkerase () , self . tamptrg (0usize) , self . tamptrg (1usize) , self . tamptrg (2usize) , self . tamptrg (3usize) , self . tamptrg (4usize) , self . tamptrg (5usize) , self . tamptrg (6usize) , self . tamptrg (7usize))
        }
    }
    #[doc = "TAMP control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Internal tamper 1 potential mode."]
        #[inline(always)]
        pub const fn itamp1pom(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 potential mode."]
        #[inline(always)]
        pub fn set_itamp1pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal tamper 2 potential mode."]
        #[inline(always)]
        pub const fn itamp2pom(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 potential mode."]
        #[inline(always)]
        pub fn set_itamp2pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal tamper 3 potential mode."]
        #[inline(always)]
        pub const fn itamp3pom(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 potential mode."]
        #[inline(always)]
        pub fn set_itamp3pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Internal tamper 4 potential mode."]
        #[inline(always)]
        pub const fn itamp4pom(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 potential mode."]
        #[inline(always)]
        pub fn set_itamp4pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Internal tamper 5 potential mode."]
        #[inline(always)]
        pub const fn itamp5pom(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 potential mode."]
        #[inline(always)]
        pub fn set_itamp5pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Internal tamper 6 potential mode."]
        #[inline(always)]
        pub const fn itamp6pom(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 potential mode."]
        #[inline(always)]
        pub fn set_itamp6pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Internal tamper 7 potential mode."]
        #[inline(always)]
        pub const fn itamp7pom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 potential mode."]
        #[inline(always)]
        pub fn set_itamp7pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Internal tamper 8 potential mode."]
        #[inline(always)]
        pub const fn itamp8pom(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 potential mode."]
        #[inline(always)]
        pub fn set_itamp8pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Internal tamper 9 potential mode."]
        #[inline(always)]
        pub const fn itamp9pom(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 potential mode."]
        #[inline(always)]
        pub fn set_itamp9pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Internal tamper 11 potential mode."]
        #[inline(always)]
        pub const fn itamp11pom(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 potential mode."]
        #[inline(always)]
        pub fn set_itamp11pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Internal tamper 12 potential mode."]
        #[inline(always)]
        pub const fn itamp12pom(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 12 potential mode."]
        #[inline(always)]
        pub fn set_itamp12pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Internal tamper 13 potential mode."]
        #[inline(always)]
        pub const fn itamp13pom(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 13 potential mode."]
        #[inline(always)]
        pub fn set_itamp13pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Internal tamper 15 potential mode."]
        #[inline(always)]
        pub const fn itamp15pom(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 15 potential mode."]
        #[inline(always)]
        pub fn set_itamp15pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("itamp1pom", &self.itamp1pom())
                .field("itamp2pom", &self.itamp2pom())
                .field("itamp3pom", &self.itamp3pom())
                .field("itamp4pom", &self.itamp4pom())
                .field("itamp5pom", &self.itamp5pom())
                .field("itamp6pom", &self.itamp6pom())
                .field("itamp7pom", &self.itamp7pom())
                .field("itamp8pom", &self.itamp8pom())
                .field("itamp9pom", &self.itamp9pom())
                .field("itamp11pom", &self.itamp11pom())
                .field("itamp12pom", &self.itamp12pom())
                .field("itamp13pom", &self.itamp13pom())
                .field("itamp15pom", &self.itamp15pom())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ itamp1pom: {=bool:?}, itamp2pom: {=bool:?}, itamp3pom: {=bool:?}, itamp4pom: {=bool:?}, itamp5pom: {=bool:?}, itamp6pom: {=bool:?}, itamp7pom: {=bool:?}, itamp8pom: {=bool:?}, itamp9pom: {=bool:?}, itamp11pom: {=bool:?}, itamp12pom: {=bool:?}, itamp13pom: {=bool:?}, itamp15pom: {=bool:?} }}" , self . itamp1pom () , self . itamp2pom () , self . itamp3pom () , self . itamp4pom () , self . itamp5pom () , self . itamp6pom () , self . itamp7pom () , self . itamp8pom () , self . itamp9pom () , self . itamp11pom () , self . itamp12pom () , self . itamp13pom () , self . itamp15pom ())
        }
    }
    #[doc = "TAMP filter control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltcr(pub u32);
    impl Fltcr {
        #[doc = "Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
        #[inline(always)]
        pub const fn tampfreq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
        #[inline(always)]
        pub fn set_tampfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
        #[inline(always)]
        pub const fn tampflt(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
        #[inline(always)]
        pub fn set_tampflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
        #[inline(always)]
        pub const fn tampprch(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
        #[inline(always)]
        pub fn set_tampprch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
        #[inline(always)]
        pub const fn tamppudis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
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
                "Fltcr {{ tampfreq: {=u8:?}, tampflt: {=u8:?}, tampprch: {=u8:?}, tamppudis: {=bool:?} }}",
                self.tampfreq(),
                self.tampflt(),
                self.tampprch(),
                self.tamppudis()
            )
        }
    }
    #[doc = "TAMP interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Tamper x interrupt enable. (x=1-8)"]
        #[inline(always)]
        pub const fn tampie(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper x interrupt enable. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampie(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 interrupt enable."]
        #[inline(always)]
        pub const fn itamp1ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 interrupt enable."]
        #[inline(always)]
        pub const fn itamp2ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 interrupt enable."]
        #[inline(always)]
        pub const fn itamp3ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 interrupt enable."]
        #[inline(always)]
        pub const fn itamp4ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 interrupt enable."]
        #[inline(always)]
        pub const fn itamp5ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp5ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 interrupt enable."]
        #[inline(always)]
        pub const fn itamp6ie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp6ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 interrupt enable."]
        #[inline(always)]
        pub const fn itamp7ie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp7ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 interrupt enable."]
        #[inline(always)]
        pub const fn itamp8ie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp8ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Internal tamper 9 interrupt enable."]
        #[inline(always)]
        pub const fn itamp9ie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp9ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 interrupt enable."]
        #[inline(always)]
        pub const fn itamp11ie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp11ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Internal tamper 12 interrupt enable."]
        #[inline(always)]
        pub const fn itamp12ie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 12 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp12ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Internal tamper 13 interrupt enable."]
        #[inline(always)]
        pub const fn itamp13ie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 13 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp13ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Internal tamper 15 interrupt enable."]
        #[inline(always)]
        pub const fn itamp15ie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 15 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp15ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("itamp1ie", &self.itamp1ie())
                .field("itamp2ie", &self.itamp2ie())
                .field("itamp3ie", &self.itamp3ie())
                .field("itamp4ie", &self.itamp4ie())
                .field("itamp5ie", &self.itamp5ie())
                .field("itamp6ie", &self.itamp6ie())
                .field("itamp7ie", &self.itamp7ie())
                .field("itamp8ie", &self.itamp8ie())
                .field("itamp9ie", &self.itamp9ie())
                .field("itamp11ie", &self.itamp11ie())
                .field("itamp12ie", &self.itamp12ie())
                .field("itamp13ie", &self.itamp13ie())
                .field("itamp15ie", &self.itamp15ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ tampie[0]: {=bool:?}, tampie[1]: {=bool:?}, tampie[2]: {=bool:?}, tampie[3]: {=bool:?}, tampie[4]: {=bool:?}, tampie[5]: {=bool:?}, tampie[6]: {=bool:?}, tampie[7]: {=bool:?}, itamp1ie: {=bool:?}, itamp2ie: {=bool:?}, itamp3ie: {=bool:?}, itamp4ie: {=bool:?}, itamp5ie: {=bool:?}, itamp6ie: {=bool:?}, itamp7ie: {=bool:?}, itamp8ie: {=bool:?}, itamp9ie: {=bool:?}, itamp11ie: {=bool:?}, itamp12ie: {=bool:?}, itamp13ie: {=bool:?}, itamp15ie: {=bool:?} }}" , self . tampie (0usize) , self . tampie (1usize) , self . tampie (2usize) , self . tampie (3usize) , self . tampie (4usize) , self . tampie (5usize) , self . tampie (6usize) , self . tampie (7usize) , self . itamp1ie () , self . itamp2ie () , self . itamp3ie () , self . itamp4ie () , self . itamp5ie () , self . itamp6ie () , self . itamp7ie () , self . itamp8ie () , self . itamp9ie () , self . itamp11ie () , self . itamp12ie () , self . itamp13ie () , self . itamp15ie ())
        }
    }
    #[doc = "TAMP non-secure masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "TAMP1 non-secure interrupt masked flag This flag is set by hardware when the tamper 1 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMP1 non-secure interrupt masked flag This flag is set by hardware when the tamper 1 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 1 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp1mf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 1 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp1mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 2 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp2mf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 2 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp2mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 3 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp3mf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 3 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp3mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 4 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp4mf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 4 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp4mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 5 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp5mf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 5 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp5mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 6 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp6mf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 6 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp6mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 tamper non-secure interrupt masked flag This flag is set by hardware when the internal tamper 7 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp7mf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 tamper non-secure interrupt masked flag This flag is set by hardware when the internal tamper 7 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp7mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 8 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp8mf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 8 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp8mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "internal tamper 9 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 9 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp9mf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 9 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 9 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp9mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "internal tamper 11 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 11 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp11mf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 11 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 11 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp11mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "internal tamper 12 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 12 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp12mf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 12 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 12 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp12mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "internal tamper 13 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 13 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp13mf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 13 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 13 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp13mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "internal tamper 15 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 15 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp15mf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 15 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 15 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp15mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("itamp1mf", &self.itamp1mf())
                .field("itamp2mf", &self.itamp2mf())
                .field("itamp3mf", &self.itamp3mf())
                .field("itamp4mf", &self.itamp4mf())
                .field("itamp5mf", &self.itamp5mf())
                .field("itamp6mf", &self.itamp6mf())
                .field("itamp7mf", &self.itamp7mf())
                .field("itamp8mf", &self.itamp8mf())
                .field("itamp9mf", &self.itamp9mf())
                .field("itamp11mf", &self.itamp11mf())
                .field("itamp12mf", &self.itamp12mf())
                .field("itamp13mf", &self.itamp13mf())
                .field("itamp15mf", &self.itamp15mf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Misr {{ tampmf[0]: {=bool:?}, tampmf[1]: {=bool:?}, tampmf[2]: {=bool:?}, tampmf[3]: {=bool:?}, tampmf[4]: {=bool:?}, tampmf[5]: {=bool:?}, tampmf[6]: {=bool:?}, tampmf[7]: {=bool:?}, itamp1mf: {=bool:?}, itamp2mf: {=bool:?}, itamp3mf: {=bool:?}, itamp4mf: {=bool:?}, itamp5mf: {=bool:?}, itamp6mf: {=bool:?}, itamp7mf: {=bool:?}, itamp8mf: {=bool:?}, itamp9mf: {=bool:?}, itamp11mf: {=bool:?}, itamp12mf: {=bool:?}, itamp13mf: {=bool:?}, itamp15mf: {=bool:?} }}" , self . tampmf (0usize) , self . tampmf (1usize) , self . tampmf (2usize) , self . tampmf (3usize) , self . tampmf (4usize) , self . tampmf (5usize) , self . tampmf (6usize) , self . tampmf (7usize) , self . itamp1mf () , self . itamp2mf () , self . itamp3mf () , self . itamp4mf () , self . itamp5mf () , self . itamp6mf () , self . itamp7mf () , self . itamp8mf () , self . itamp9mf () , self . itamp11mf () , self . itamp12mf () , self . itamp13mf () , self . itamp15mf ())
        }
    }
    #[doc = "TAMP option register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Or(pub u32);
    impl Or {
        #[doc = "TAMP_OUT3 mapping."]
        #[inline(always)]
        pub const fn out3_rmp(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "TAMP_OUT3 mapping."]
        #[inline(always)]
        pub fn set_out3_rmp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "TAMP_OUT5 mapping."]
        #[inline(always)]
        pub const fn out5_rmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_OUT5 mapping."]
        #[inline(always)]
        pub fn set_out5_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TAMP_IN2 mapping."]
        #[inline(always)]
        pub const fn in2_rmp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_IN2 mapping."]
        #[inline(always)]
        pub fn set_in2_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TAMP_IN3 mapping."]
        #[inline(always)]
        pub const fn in3_rmp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_IN3 mapping."]
        #[inline(always)]
        pub fn set_in3_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TAMP_IN4 mapping."]
        #[inline(always)]
        pub const fn in4_rmp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_IN4 mapping."]
        #[inline(always)]
        pub fn set_in4_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Or {
        #[inline(always)]
        fn default() -> Or {
            Or(0)
        }
    }
    impl core::fmt::Debug for Or {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Or")
                .field("out3_rmp", &self.out3_rmp())
                .field("out5_rmp", &self.out5_rmp())
                .field("in2_rmp", &self.in2_rmp())
                .field("in3_rmp", &self.in3_rmp())
                .field("in4_rmp", &self.in4_rmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Or {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Or {{ out3_rmp: {=u8:?}, out5_rmp: {=bool:?}, in2_rmp: {=bool:?}, in3_rmp: {=bool:?}, in4_rmp: {=bool:?} }}" , self . out3_rmp () , self . out5_rmp () , self . in2_rmp () , self . in3_rmp () , self . in4_rmp ())
        }
    }
    #[doc = "TAMP privilege mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Monotonic counter 1 privilege protection."]
        #[inline(always)]
        pub const fn cnt1priv(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Monotonic counter 1 privilege protection."]
        #[inline(always)]
        pub fn set_cnt1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup registers zone 1 privilege protection."]
        #[inline(always)]
        pub const fn bkprwpriv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 1 privilege protection."]
        #[inline(always)]
        pub fn set_bkprwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Backup registers zone 2 privilege protection."]
        #[inline(always)]
        pub const fn bkpwpriv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 2 privilege protection."]
        #[inline(always)]
        pub fn set_bkpwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper privilege protection (excluding backup registers) Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub const fn tamppriv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper privilege protection (excluding backup registers) Note: Refer to for details on the read protection."]
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
    #[doc = "TAMP erase configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpcfgr(pub u32);
    impl Rpcfgr {
        #[doc = "Configurable resource 0 protection."]
        #[inline(always)]
        pub const fn rpcfg0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configurable resource 0 protection."]
        #[inline(always)]
        pub fn set_rpcfg0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Rpcfgr {
        #[inline(always)]
        fn default() -> Rpcfgr {
            Rpcfgr(0)
        }
    }
    impl core::fmt::Debug for Rpcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpcfgr").field("rpcfg0", &self.rpcfg0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rpcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rpcfgr {{ rpcfg0: {=bool:?} }}", self.rpcfg0())
        }
    }
    #[doc = "TAMP status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the TAMP_SR register. (x=1-8)"]
        #[inline(always)]
        pub const fn ctampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the TAMP_SR register. (x=1-8)"]
        #[inline(always)]
        pub fn set_ctampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp1f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp2f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp5f(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp5f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp6f(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp6f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp7f(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp7f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp8f(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp8f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp9f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp9f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp11f(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp11f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp12f(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp12f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp13f(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp13f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Clear ITAMP15 detection flag Writing 1 in this bit clears the ITAMP15F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp15f(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP15 detection flag Writing 1 in this bit clears the ITAMP15F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp15f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("citamp1f", &self.citamp1f())
                .field("citamp2f", &self.citamp2f())
                .field("citamp3f", &self.citamp3f())
                .field("citamp4f", &self.citamp4f())
                .field("citamp5f", &self.citamp5f())
                .field("citamp6f", &self.citamp6f())
                .field("citamp7f", &self.citamp7f())
                .field("citamp8f", &self.citamp8f())
                .field("citamp9f", &self.citamp9f())
                .field("citamp11f", &self.citamp11f())
                .field("citamp12f", &self.citamp12f())
                .field("citamp13f", &self.citamp13f())
                .field("citamp15f", &self.citamp15f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Scr {{ ctampf[0]: {=bool:?}, ctampf[1]: {=bool:?}, ctampf[2]: {=bool:?}, ctampf[3]: {=bool:?}, ctampf[4]: {=bool:?}, ctampf[5]: {=bool:?}, ctampf[6]: {=bool:?}, ctampf[7]: {=bool:?}, citamp1f: {=bool:?}, citamp2f: {=bool:?}, citamp3f: {=bool:?}, citamp4f: {=bool:?}, citamp5f: {=bool:?}, citamp6f: {=bool:?}, citamp7f: {=bool:?}, citamp8f: {=bool:?}, citamp9f: {=bool:?}, citamp11f: {=bool:?}, citamp12f: {=bool:?}, citamp13f: {=bool:?}, citamp15f: {=bool:?} }}" , self . ctampf (0usize) , self . ctampf (1usize) , self . ctampf (2usize) , self . ctampf (3usize) , self . ctampf (4usize) , self . ctampf (5usize) , self . ctampf (6usize) , self . ctampf (7usize) , self . citamp1f () , self . citamp2f () , self . citamp3f () , self . citamp4f () , self . citamp5f () , self . citamp6f () , self . citamp7f () , self . citamp8f () , self . citamp9f () , self . citamp11f () , self . citamp12f () , self . citamp13f () , self . citamp15f ())
        }
    }
    #[doc = "TAMP secure mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub const fn bkprwsec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub fn set_bkprwsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Monotonic counter 1 secure protection."]
        #[inline(always)]
        pub const fn cnt1sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Monotonic counter 1 secure protection."]
        #[inline(always)]
        pub fn set_cnt1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub const fn bkpwsec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub fn set_bkpwsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub const fn bhklock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub fn set_bhklock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub const fn tampsec(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
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
    #[doc = "TAMP secure masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smisr(pub u32);
    impl Smisr {
        #[doc = "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper x secure interrupt is raised. (x=1-8)"]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper x secure interrupt is raised. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 secure interrupt masked flag This flag is set by hardware when the internal tamper 1 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp1mf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 secure interrupt masked flag This flag is set by hardware when the internal tamper 1 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp1mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 secure interrupt masked flag This flag is set by hardware when the internal tamper 2 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp2mf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 secure interrupt masked flag This flag is set by hardware when the internal tamper 2 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp2mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 secure interrupt masked flag This flag is set by hardware when the internal tamper 3 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp3mf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 secure interrupt masked flag This flag is set by hardware when the internal tamper 3 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp3mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 secure interrupt masked flag This flag is set by hardware when the internal tamper 4 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp4mf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 secure interrupt masked flag This flag is set by hardware when the internal tamper 4 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp4mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 secure interrupt masked flag This flag is set by hardware when the internal tamper 5 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp5mf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 secure interrupt masked flag This flag is set by hardware when the internal tamper 5 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp5mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 secure interrupt masked flag This flag is set by hardware when the internal tamper 6 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp6mf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 secure interrupt masked flag This flag is set by hardware when the internal tamper 6 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp6mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 secure interrupt masked flag This flag is set by hardware when the internal tamper 7 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp7mf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 secure interrupt masked flag This flag is set by hardware when the internal tamper 7 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp7mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 secure interrupt masked flag This flag is set by hardware when the internal tamper 8 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp8mf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 secure interrupt masked flag This flag is set by hardware when the internal tamper 8 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp8mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "internal tamper 9 secure interrupt masked flag This flag is set by hardware when the internal tamper 9 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp9mf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 9 secure interrupt masked flag This flag is set by hardware when the internal tamper 9 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp9mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "internal tamper 11 secure interrupt masked flag This flag is set by hardware when the internal tamper 11 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp11mf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 11 secure interrupt masked flag This flag is set by hardware when the internal tamper 11 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp11mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "internal tamper 12 secure interrupt masked flag This flag is set by hardware when the internal tamper 12 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp12mf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 12 secure interrupt masked flag This flag is set by hardware when the internal tamper 12 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp12mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "internal tamper 13 secure interrupt masked flag This flag is set by hardware when the internal tamper 13 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp13mf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 13 secure interrupt masked flag This flag is set by hardware when the internal tamper 13 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp13mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "internal tamper 15 secure interrupt masked flag This flag is set by hardware when the internal tamper 15 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp15mf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 15 secure interrupt masked flag This flag is set by hardware when the internal tamper 15 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp15mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("itamp1mf", &self.itamp1mf())
                .field("itamp2mf", &self.itamp2mf())
                .field("itamp3mf", &self.itamp3mf())
                .field("itamp4mf", &self.itamp4mf())
                .field("itamp5mf", &self.itamp5mf())
                .field("itamp6mf", &self.itamp6mf())
                .field("itamp7mf", &self.itamp7mf())
                .field("itamp8mf", &self.itamp8mf())
                .field("itamp9mf", &self.itamp9mf())
                .field("itamp11mf", &self.itamp11mf())
                .field("itamp12mf", &self.itamp12mf())
                .field("itamp13mf", &self.itamp13mf())
                .field("itamp15mf", &self.itamp15mf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smisr {{ tampmf[0]: {=bool:?}, tampmf[1]: {=bool:?}, tampmf[2]: {=bool:?}, tampmf[3]: {=bool:?}, tampmf[4]: {=bool:?}, tampmf[5]: {=bool:?}, tampmf[6]: {=bool:?}, tampmf[7]: {=bool:?}, itamp1mf: {=bool:?}, itamp2mf: {=bool:?}, itamp3mf: {=bool:?}, itamp4mf: {=bool:?}, itamp5mf: {=bool:?}, itamp6mf: {=bool:?}, itamp7mf: {=bool:?}, itamp8mf: {=bool:?}, itamp9mf: {=bool:?}, itamp11mf: {=bool:?}, itamp12mf: {=bool:?}, itamp13mf: {=bool:?}, itamp15mf: {=bool:?} }}" , self . tampmf (0usize) , self . tampmf (1usize) , self . tampmf (2usize) , self . tampmf (3usize) , self . tampmf (4usize) , self . tampmf (5usize) , self . tampmf (6usize) , self . tampmf (7usize) , self . itamp1mf () , self . itamp2mf () , self . itamp3mf () , self . itamp4mf () , self . itamp5mf () , self . itamp6mf () , self . itamp7mf () , self . itamp8mf () , self . itamp9mf () , self . itamp11mf () , self . itamp12mf () , self . itamp13mf () , self . itamp15mf ())
        }
    }
    #[doc = "TAMP status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input. (x=1-8)"]
        #[inline(always)]
        pub const fn tampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 1."]
        #[inline(always)]
        pub const fn itamp1f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 1."]
        #[inline(always)]
        pub fn set_itamp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 2."]
        #[inline(always)]
        pub const fn itamp2f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 2."]
        #[inline(always)]
        pub fn set_itamp2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
        #[inline(always)]
        pub const fn itamp3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
        #[inline(always)]
        pub fn set_itamp3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
        #[inline(always)]
        pub const fn itamp4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
        #[inline(always)]
        pub fn set_itamp4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
        #[inline(always)]
        pub const fn itamp5f(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
        #[inline(always)]
        pub fn set_itamp5f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
        #[inline(always)]
        pub const fn itamp6f(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
        #[inline(always)]
        pub fn set_itamp6f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 7."]
        #[inline(always)]
        pub const fn itamp7f(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 7."]
        #[inline(always)]
        pub fn set_itamp7f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 8."]
        #[inline(always)]
        pub const fn itamp8f(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 8."]
        #[inline(always)]
        pub fn set_itamp8f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Internal tamper 9 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 9."]
        #[inline(always)]
        pub const fn itamp9f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 9."]
        #[inline(always)]
        pub fn set_itamp9f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 11."]
        #[inline(always)]
        pub const fn itamp11f(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 11."]
        #[inline(always)]
        pub fn set_itamp11f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Internal tamper 12 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 12."]
        #[inline(always)]
        pub const fn itamp12f(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 12 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 12."]
        #[inline(always)]
        pub fn set_itamp12f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Internal tamper 13 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 13."]
        #[inline(always)]
        pub const fn itamp13f(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 13 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 13."]
        #[inline(always)]
        pub fn set_itamp13f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Internal tamper 15 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 15."]
        #[inline(always)]
        pub const fn itamp15f(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 15 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 15."]
        #[inline(always)]
        pub fn set_itamp15f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("itamp1f", &self.itamp1f())
                .field("itamp2f", &self.itamp2f())
                .field("itamp3f", &self.itamp3f())
                .field("itamp4f", &self.itamp4f())
                .field("itamp5f", &self.itamp5f())
                .field("itamp6f", &self.itamp6f())
                .field("itamp7f", &self.itamp7f())
                .field("itamp8f", &self.itamp8f())
                .field("itamp9f", &self.itamp9f())
                .field("itamp11f", &self.itamp11f())
                .field("itamp12f", &self.itamp12f())
                .field("itamp13f", &self.itamp13f())
                .field("itamp15f", &self.itamp15f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ tampf[0]: {=bool:?}, tampf[1]: {=bool:?}, tampf[2]: {=bool:?}, tampf[3]: {=bool:?}, tampf[4]: {=bool:?}, tampf[5]: {=bool:?}, tampf[6]: {=bool:?}, tampf[7]: {=bool:?}, itamp1f: {=bool:?}, itamp2f: {=bool:?}, itamp3f: {=bool:?}, itamp4f: {=bool:?}, itamp5f: {=bool:?}, itamp6f: {=bool:?}, itamp7f: {=bool:?}, itamp8f: {=bool:?}, itamp9f: {=bool:?}, itamp11f: {=bool:?}, itamp12f: {=bool:?}, itamp13f: {=bool:?}, itamp15f: {=bool:?} }}" , self . tampf (0usize) , self . tampf (1usize) , self . tampf (2usize) , self . tampf (3usize) , self . tampf (4usize) , self . tampf (5usize) , self . tampf (6usize) , self . tampf (7usize) , self . itamp1f () , self . itamp2f () , self . itamp3f () , self . itamp4f () , self . itamp5f () , self . itamp6f () , self . itamp7f () , self . itamp8f () , self . itamp9f () , self . itamp11f () , self . itamp12f () , self . itamp13f () , self . itamp15f ())
        }
    }
}
