#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FLASH address block description"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLASH access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FLASH non-secure key register"]
    #[inline(always)]
    pub const fn nskeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FLASH secure key register"]
    #[inline(always)]
    pub const fn seckeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FLASH non-secure OBK key register"]
    #[inline(always)]
    pub const fn nsobkkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FLASH secure OBK key register"]
    #[inline(always)]
    pub const fn secobkkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "FLASH operation status register"]
    #[inline(always)]
    pub const fn opsr(self) -> crate::common::Reg<regs::Opsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FLASH option control register"]
    #[inline(always)]
    pub const fn optcr(self) -> crate::common::Reg<regs::Optcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FLASH non-secure status register"]
    #[inline(always)]
    pub const fn nssr(self) -> crate::common::Reg<regs::Nssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FLASH secure status register"]
    #[inline(always)]
    pub const fn secsr(self) -> crate::common::Reg<regs::Secsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FLASH non-secure control register"]
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::Nscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FLASH secure control register"]
    #[inline(always)]
    pub const fn seccr(self) -> crate::common::Reg<regs::Seccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FLASH non-secure clear control register"]
    #[inline(always)]
    pub const fn nsccr(self) -> crate::common::Reg<regs::Nsccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FLASH secure clear control register"]
    #[inline(always)]
    pub const fn secccr(self) -> crate::common::Reg<regs::Secccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "FLASH privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "FLASH non-secure OBK configuration register"]
    #[inline(always)]
    pub const fn nsobkcfgr(self) -> crate::common::Reg<regs::Nsobkcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FLASH secure OBK configuration register"]
    #[inline(always)]
    pub const fn secobkcfgr(self) -> crate::common::Reg<regs::Secobkcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FLASH HDP extension register"]
    #[inline(always)]
    pub const fn hdpextr(self) -> crate::common::Reg<regs::Hdpextr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_cur(self) -> crate::common::Reg<regs::Optsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_prg(self) -> crate::common::Reg<regs::Optsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FLASH non-secure EPOCH register"]
    #[inline(always)]
    pub const fn nsepochr_cur(self) -> crate::common::Reg<regs::Nsepochr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "FLASH secure EPOCH register"]
    #[inline(always)]
    pub const fn secepochr_cur(self) -> crate::common::Reg<regs::Secepochr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "FLASH option status register 2"]
    #[inline(always)]
    pub const fn optsr2_cur(self) -> crate::common::Reg<regs::Optsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "FLASH option status register 2"]
    #[inline(always)]
    pub const fn optsr2_prg(self) -> crate::common::Reg<regs::Optsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "FLASH non-secure boot register"]
    #[inline(always)]
    pub const fn nsbootr_cur(self) -> crate::common::Reg<regs::Nsbootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FLASH non-secure boot register"]
    #[inline(always)]
    pub const fn nsbootr_prg(self) -> crate::common::Reg<regs::Nsbootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FLASH secure boot register"]
    #[inline(always)]
    pub const fn secbootr_cur(self) -> crate::common::Reg<regs::Secbootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "FLASH secure boot register"]
    #[inline(always)]
    pub const fn bootr_prg(self) -> crate::common::Reg<regs::Bootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "FLASH non-secure OTP block lock"]
    #[inline(always)]
    pub const fn otpblr_cur(self) -> crate::common::Reg<regs::Otpblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "FLASH non-secure OTP block lock"]
    #[inline(always)]
    pub const fn otpblr_prg(self) -> crate::common::Reg<regs::Otpblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "FLASH secure block based register for Bank 1"]
    #[inline(always)]
    pub const fn secbb1r1(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "FLASH secure block based register for Bank 1"]
    #[inline(always)]
    pub const fn secbb1r2(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "FLASH secure block based register for Bank 1"]
    #[inline(always)]
    pub const fn secbb1r3(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "FLASH secure block based register for Bank 1"]
    #[inline(always)]
    pub const fn secbb1r4(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "FLASH privilege block based register for Bank 1"]
    #[inline(always)]
    pub const fn privbb1r1(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "FLASH privilege block based register for Bank 1"]
    #[inline(always)]
    pub const fn privbb1r2(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "FLASH privilege block based register for Bank 1"]
    #[inline(always)]
    pub const fn privbb1r3(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "FLASH privilege block based register for Bank 1"]
    #[inline(always)]
    pub const fn privbb1r4(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "FLASH security watermark for Bank 1"]
    #[inline(always)]
    pub const fn secwm1r_cur(self) -> crate::common::Reg<regs::Secwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "FLASH security watermark for Bank 1"]
    #[inline(always)]
    pub const fn secwm1r_prg(self) -> crate::common::Reg<regs::Secwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "FLASH write sector group protection for Bank 1"]
    #[inline(always)]
    pub const fn wrp1r_cur(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "FLASH write sector group protection for Bank 1"]
    #[inline(always)]
    pub const fn wrp1r_prg(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "FLASH data sector configuration Bank 1"]
    #[inline(always)]
    pub const fn edata1r_cur(self) -> crate::common::Reg<regs::Edata1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "FLASH data sector configuration Bank 1"]
    #[inline(always)]
    pub const fn edata1r_prg(self) -> crate::common::Reg<regs::Edata1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "FLASH HDP Bank 1 configuration"]
    #[inline(always)]
    pub const fn hdp1r_cur(self) -> crate::common::Reg<regs::Hdp1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "FLASH HDP Bank 1 configuration"]
    #[inline(always)]
    pub const fn hdp1r_prg(self) -> crate::common::Reg<regs::Hdp1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "FLASH ECC correction register"]
    #[inline(always)]
    pub const fn ecccorr(self) -> crate::common::Reg<regs::Ecccorr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "FLASH ECC detection register"]
    #[inline(always)]
    pub const fn eccdetr(self) -> crate::common::Reg<regs::Eccdetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "FLASH ECC data"]
    #[inline(always)]
    pub const fn eccdr(self) -> crate::common::Reg<regs::Eccdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "FLASH secure block-based register for Bank 2"]
    #[inline(always)]
    pub const fn secbb2r1(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "FLASH secure block-based register for Bank 2"]
    #[inline(always)]
    pub const fn secbb2r2(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[doc = "FLASH secure block-based register for Bank 2"]
    #[inline(always)]
    pub const fn secbb2r3(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "FLASH secure block-based register for Bank 2"]
    #[inline(always)]
    pub const fn secbb2r4(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[doc = "FLASH privilege block-based register for Bank 2"]
    #[inline(always)]
    pub const fn privbb2r1(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "FLASH privilege block-based register for Bank 2"]
    #[inline(always)]
    pub const fn privbb2r2(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[doc = "FLASH privilege block-based register for Bank 2"]
    #[inline(always)]
    pub const fn privbb2r3(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "FLASH privilege block-based register for Bank 2"]
    #[inline(always)]
    pub const fn privbb2r4(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[doc = "FLASH security watermark for Bank 2"]
    #[inline(always)]
    pub const fn secwm2r_cur(self) -> crate::common::Reg<regs::Secwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "FLASH security watermark for Bank 2"]
    #[inline(always)]
    pub const fn secwm2r_prg(self) -> crate::common::Reg<regs::Secwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[doc = "FLASH write sector group protection for Bank 2"]
    #[inline(always)]
    pub const fn wrp2r_cur(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "FLASH write sector group protection for Bank 2"]
    #[inline(always)]
    pub const fn wrp2r_prg(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "FLASH data sectors configuration Bank 2"]
    #[inline(always)]
    pub const fn edata2r_cur(self) -> crate::common::Reg<regs::Edata2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[doc = "FLASH data sector configuration Bank 2"]
    #[inline(always)]
    pub const fn edata2r_prg(self) -> crate::common::Reg<regs::Edata2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f4usize) as _) }
    }
    #[doc = "FLASH HDP Bank 2 configuration"]
    #[inline(always)]
    pub const fn hdp2r_cur(self) -> crate::common::Reg<regs::Hdp2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[doc = "FLASH HDP Bank 2 configuration"]
    #[inline(always)]
    pub const fn hdp2r_prg(self) -> crate::common::Reg<regs::Hdp2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "FLASH access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Read latency These bits are used to control the number of wait states used during read operations on both nonvolatile memory banks. The application software has to program them to the correct value depending on the embedded flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Read latency These bits are used to control the number of wait states used during read operations on both nonvolatile memory banks. The application software has to program them to the correct value depending on the embedded flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Flash signal delay These bits are used to control the delay between nonvolatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded flash memory interface frequency. Please refer to Table�44 for details. Note: No check is performed to verify that the configuration is correct. Note: Two WRHIGHFREQ values can be selected for some frequencies."]
        #[inline(always)]
        pub const fn wrhighfreq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Flash signal delay These bits are used to control the delay between nonvolatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded flash memory interface frequency. Please refer to Table�44 for details. Note: No check is performed to verify that the configuration is correct. Note: Two WRHIGHFREQ values can be selected for some frequencies."]
        #[inline(always)]
        pub fn set_wrhighfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    impl core::fmt::Debug for Acr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acr")
                .field("latency", &self.latency())
                .field("wrhighfreq", &self.wrhighfreq())
                .field("prften", &self.prften())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acr {{ latency: {=u8:?}, wrhighfreq: {=u8:?}, prften: {=bool:?} }}",
                self.latency(),
                self.wrhighfreq(),
                self.prften()
            )
        }
    }
    #[doc = "FLASH secure boot register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bootr(pub u32);
    impl Bootr {
        #[doc = "A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
        #[inline(always)]
        pub const fn secboot_lock(&self) -> super::vals::BootrSecbootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::BootrSecbootLock::from_bits(val as u8)
        }
        #[doc = "A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
        #[inline(always)]
        pub fn set_secboot_lock(&mut self, val: super::vals::BootrSecbootLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Secure unique boot entry address. These bits allow configuring the secure UBE address."]
        #[inline(always)]
        pub const fn secbootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Secure unique boot entry address. These bits allow configuring the secure UBE address."]
        #[inline(always)]
        pub fn set_secbootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Bootr {
        #[inline(always)]
        fn default() -> Bootr {
            Bootr(0)
        }
    }
    impl core::fmt::Debug for Bootr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bootr")
                .field("secboot_lock", &self.secboot_lock())
                .field("secbootadd", &self.secbootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bootr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bootr {{ secboot_lock: {:?}, secbootadd: {=u32:?} }}",
                self.secboot_lock(),
                self.secbootadd()
            )
        }
    }
    #[doc = "FLASH ECC correction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecccorr(pub u32);
    impl Ecccorr {
        #[doc = "ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area)."]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area)."]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Single ECC error corrected in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error."]
        #[inline(always)]
        pub const fn obk_ecc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Single ECC error corrected in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error."]
        #[inline(always)]
        pub fn set_obk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ECC fail for corrected ECC error in flash high-cycle data area It indicates if flash high-cycle data area is concerned by ECC error."]
        #[inline(always)]
        pub const fn edata_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for corrected ECC error in flash high-cycle data area It indicates if flash high-cycle data area is concerned by ECC error."]
        #[inline(always)]
        pub fn set_edata_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC fail bank for corrected ECC error It indicates which bank is concerned by ECC error"]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank for corrected ECC error It indicates which bank is concerned by ECC error"]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC fail for corrected ECC error in system flash memory It indicates if system flash memory is concerned by ECC error."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for corrected ECC error in system flash memory It indicates if system flash memory is concerned by ECC error."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
        #[inline(always)]
        pub const fn otp_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
        #[inline(always)]
        pub fn set_otp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
        #[inline(always)]
        pub const fn ecccie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
        #[inline(always)]
        pub fn set_ecccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ecccorr {
        #[inline(always)]
        fn default() -> Ecccorr {
            Ecccorr(0)
        }
    }
    impl core::fmt::Debug for Ecccorr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecccorr")
                .field("addr_ecc", &self.addr_ecc())
                .field("obk_ecc", &self.obk_ecc())
                .field("edata_ecc", &self.edata_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("otp_ecc", &self.otp_ecc())
                .field("ecccie", &self.ecccie())
                .field("eccc", &self.eccc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecccorr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ecccorr {{ addr_ecc: {=u16:?}, obk_ecc: {=bool:?}, edata_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, ecccie: {=bool:?}, eccc: {=bool:?} }}" , self . addr_ecc () , self . obk_ecc () , self . edata_ecc () , self . bk_ecc () , self . sysf_ecc () , self . otp_ecc () , self . ecccie () , self . eccc ())
        }
    }
    #[doc = "FLASH ECC detection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdetr(pub u32);
    impl Eccdetr {
        #[doc = "ECC error address When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved. The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area)."]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error address When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved. The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area)."]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "ECC fail double ECC error in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error."]
        #[inline(always)]
        pub const fn obk_ecc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail double ECC error in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error."]
        #[inline(always)]
        pub fn set_obk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ECC fail double ECC error in flash high-cycle data area It indicates if flash high-cycle data area is concerned by ECC error."]
        #[inline(always)]
        pub const fn edata_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail double ECC error in flash high-cycle data area It indicates if flash high-cycle data area is concerned by ECC error."]
        #[inline(always)]
        pub fn set_edata_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC fail bank for double ECC error It indicates which bank is concerned by ECC error"]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank for double ECC error It indicates which bank is concerned by ECC error"]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC fail for double ECC error in system flash memory It indicates if system flash memory is concerned by ECC error."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for double ECC error in system flash memory It indicates if system flash memory is concerned by ECC error."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP ECC error bit This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
        #[inline(always)]
        pub const fn otp_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP ECC error bit This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
        #[inline(always)]
        pub fn set_otp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC detection Set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection Set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
        #[inline(always)]
        pub fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccdetr {
        #[inline(always)]
        fn default() -> Eccdetr {
            Eccdetr(0)
        }
    }
    impl core::fmt::Debug for Eccdetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdetr")
                .field("addr_ecc", &self.addr_ecc())
                .field("obk_ecc", &self.obk_ecc())
                .field("edata_ecc", &self.edata_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("otp_ecc", &self.otp_ecc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdetr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Eccdetr {{ addr_ecc: {=u16:?}, obk_ecc: {=bool:?}, edata_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, eccd: {=bool:?} }}" , self . addr_ecc () , self . obk_ecc () , self . edata_ecc () , self . bk_ecc () , self . sysf_ecc () , self . otp_ecc () , self . eccd ())
        }
    }
    #[doc = "FLASH ECC data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdr(pub u32);
    impl Eccdr {
        #[doc = "ECC error data When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit data (data area, read-only/OTP area), the failing data is read to this register. By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory."]
        #[inline(always)]
        pub const fn data_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error data When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit data (data area, read-only/OTP area), the failing data is read to this register. By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory."]
        #[inline(always)]
        pub fn set_data_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Eccdr {
        #[inline(always)]
        fn default() -> Eccdr {
            Eccdr(0)
        }
    }
    impl core::fmt::Debug for Eccdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdr").field("data_ecc", &self.data_ecc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccdr {{ data_ecc: {=u16:?} }}", self.data_ecc())
        }
    }
    #[doc = "FLASH data sector configuration Bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Edata1r(pub u32);
    impl Edata1r {
        #[doc = "EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
        #[inline(always)]
        pub const fn edata1_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
        #[inline(always)]
        pub fn set_edata1_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Bank 1 flash high-cycle data enable"]
        #[inline(always)]
        pub const fn edata1_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 flash high-cycle data enable"]
        #[inline(always)]
        pub fn set_edata1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Edata1r {
        #[inline(always)]
        fn default() -> Edata1r {
            Edata1r(0)
        }
    }
    impl core::fmt::Debug for Edata1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Edata1r")
                .field("edata1_strt", &self.edata1_strt())
                .field("edata1_en", &self.edata1_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Edata1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Edata1r {{ edata1_strt: {=u8:?}, edata1_en: {=bool:?} }}",
                self.edata1_strt(),
                self.edata1_en()
            )
        }
    }
    #[doc = "FLASH data sector configuration Bank 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Edata2r(pub u32);
    impl Edata2r {
        #[doc = "EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data."]
        #[inline(always)]
        pub const fn edata2_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data."]
        #[inline(always)]
        pub fn set_edata2_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Bank 2 flash high-cycle data enable"]
        #[inline(always)]
        pub const fn edata2_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 flash high-cycle data enable"]
        #[inline(always)]
        pub fn set_edata2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Edata2r {
        #[inline(always)]
        fn default() -> Edata2r {
            Edata2r(0)
        }
    }
    impl core::fmt::Debug for Edata2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Edata2r")
                .field("edata2_strt", &self.edata2_strt())
                .field("edata2_en", &self.edata2_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Edata2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Edata2r {{ edata2_strt: {=u8:?}, edata2_en: {=bool:?} }}",
                self.edata2_strt(),
                self.edata2_en()
            )
        }
    }
    #[doc = "FLASH HDP Bank 1 configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp1r(pub u32);
    impl Hdp1r {
        #[doc = "HDPL barrier start set in number of 8-Kbyte sectors"]
        #[inline(always)]
        pub const fn hdp1_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "HDPL barrier start set in number of 8-Kbyte sectors"]
        #[inline(always)]
        pub fn set_hdp1_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "HDPL barrier end set in number of 8-Kbyte sectors"]
        #[inline(always)]
        pub const fn hdp1_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "HDPL barrier end set in number of 8-Kbyte sectors"]
        #[inline(always)]
        pub fn set_hdp1_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Hdp1r {
        #[inline(always)]
        fn default() -> Hdp1r {
            Hdp1r(0)
        }
    }
    impl core::fmt::Debug for Hdp1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp1r")
                .field("hdp1_strt", &self.hdp1_strt())
                .field("hdp1_end", &self.hdp1_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp1r {{ hdp1_strt: {=u8:?}, hdp1_end: {=u8:?} }}",
                self.hdp1_strt(),
                self.hdp1_end()
            )
        }
    }
    #[doc = "FLASH HDP Bank 2 configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp2r(pub u32);
    impl Hdp2r {
        #[doc = "HDPL barrier start set in number of 8-Kbyte sectors"]
        #[inline(always)]
        pub const fn hdp2_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "HDPL barrier start set in number of 8-Kbyte sectors"]
        #[inline(always)]
        pub fn set_hdp2_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "HDPL barrier end set in number of 8-Kbyte sectors"]
        #[inline(always)]
        pub const fn hdp2_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "HDPL barrier end set in number of 8-Kbyte sectors"]
        #[inline(always)]
        pub fn set_hdp2_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Hdp2r {
        #[inline(always)]
        fn default() -> Hdp2r {
            Hdp2r(0)
        }
    }
    impl core::fmt::Debug for Hdp2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp2r")
                .field("hdp2_strt", &self.hdp2_strt())
                .field("hdp2_end", &self.hdp2_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp2r {{ hdp2_strt: {=u8:?}, hdp2_end: {=u8:?} }}",
                self.hdp2_strt(),
                self.hdp2_end()
            )
        }
    }
    #[doc = "FLASH HDP extension register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdpextr(pub u32);
    impl Hdpextr {
        #[doc = "HDP area extension in 8�Kbytes sectors in Bank1. Extension is added after the HDP1_END sector (included)."]
        #[inline(always)]
        pub const fn hdp1_ext(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "HDP area extension in 8�Kbytes sectors in Bank1. Extension is added after the HDP1_END sector (included)."]
        #[inline(always)]
        pub fn set_hdp1_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "HDP area extension in 8�Kbytes sectors in bank 2. Extension is added after the HDP2_END sector (included)."]
        #[inline(always)]
        pub const fn hdp2_ext(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "HDP area extension in 8�Kbytes sectors in bank 2. Extension is added after the HDP2_END sector (included)."]
        #[inline(always)]
        pub fn set_hdp2_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Hdpextr {
        #[inline(always)]
        fn default() -> Hdpextr {
            Hdpextr(0)
        }
    }
    impl core::fmt::Debug for Hdpextr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdpextr")
                .field("hdp1_ext", &self.hdp1_ext())
                .field("hdp2_ext", &self.hdp2_ext())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpextr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdpextr {{ hdp1_ext: {=u8:?}, hdp2_ext: {=u8:?} }}",
                self.hdp1_ext(),
                self.hdp2_ext()
            )
        }
    }
    #[doc = "FLASH non-secure boot register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootr(pub u32);
    impl Nsbootr {
        #[doc = "A field locking the values of SWAP_ BANK, and NSBOOTADD settings."]
        #[inline(always)]
        pub const fn nsboot_lock(&self) -> super::vals::NsbootrNsbootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::NsbootrNsbootLock::from_bits(val as u8)
        }
        #[doc = "A field locking the values of SWAP_ BANK, and NSBOOTADD settings."]
        #[inline(always)]
        pub fn set_nsboot_lock(&mut self, val: super::vals::NsbootrNsbootLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Non secure unique boot entry address These bits allow configuring the Non secure BOOT address"]
        #[inline(always)]
        pub const fn nsbootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Non secure unique boot entry address These bits allow configuring the Non secure BOOT address"]
        #[inline(always)]
        pub fn set_nsbootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Nsbootr {
        #[inline(always)]
        fn default() -> Nsbootr {
            Nsbootr(0)
        }
    }
    impl core::fmt::Debug for Nsbootr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsbootr")
                .field("nsboot_lock", &self.nsboot_lock())
                .field("nsbootadd", &self.nsbootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsbootr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nsbootr {{ nsboot_lock: {:?}, nsbootadd: {=u32:?} }}",
                self.nsboot_lock(),
                self.nsbootadd()
            )
        }
    }
    #[doc = "FLASH non-secure clear control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsccr(pub u32);
    impl Nsccr {
        #[doc = "EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBKERR flag clear bit. Setting this bit to 1 resets to 0 OBKERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBKERR flag clear bit. Setting this bit to 1 resets to 0 OBKERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBKWERR flag clear bit. Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBKWERR flag clear bit. Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Clear the flag corresponding flag in FLASH_NSSR by writing this bit."]
        #[inline(always)]
        pub const fn clr_optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the flag corresponding flag in FLASH_NSSR by writing this bit."]
        #[inline(always)]
        pub fn set_clr_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Nsccr {
        #[inline(always)]
        fn default() -> Nsccr {
            Nsccr(0)
        }
    }
    impl core::fmt::Debug for Nsccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsccr")
                .field("clr_eop", &self.clr_eop())
                .field("clr_wrperr", &self.clr_wrperr())
                .field("clr_pgserr", &self.clr_pgserr())
                .field("clr_strberr", &self.clr_strberr())
                .field("clr_incerr", &self.clr_incerr())
                .field("clr_obkerr", &self.clr_obkerr())
                .field("clr_obkwerr", &self.clr_obkwerr())
                .field("clr_optchangeerr", &self.clr_optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nsccr {{ clr_eop: {=bool:?}, clr_wrperr: {=bool:?}, clr_pgserr: {=bool:?}, clr_strberr: {=bool:?}, clr_incerr: {=bool:?}, clr_obkerr: {=bool:?}, clr_obkwerr: {=bool:?}, clr_optchangeerr: {=bool:?} }}" , self . clr_eop () , self . clr_wrperr () , self . clr_pgserr () , self . clr_strberr () , self . clr_incerr () , self . clr_obkerr () , self . clr_obkwerr () , self . clr_optchangeerr ())
        }
    }
    #[doc = "FLASH non-secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr(pub u32);
    impl Nscr {
        #[doc = "configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
        #[inline(always)]
        pub const fn ser(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
        #[inline(always)]
        pub fn set_ser(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "erase request Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "erase request Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
        #[inline(always)]
        pub fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty and was filled by non-secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty and was filled by non-secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
        #[inline(always)]
        pub fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. .."]
        #[inline(always)]
        pub const fn snb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x7f;
            val as u8
        }
        #[doc = "sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. .."]
        #[inline(always)]
        pub fn set_snb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 6usize)) | (((val as u32) & 0x7f) << 6usize);
        }
        #[doc = "Mass erase request Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase request Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error interrupt enable bit When this bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error interrupt enable bit When this bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error interrupt enable bit When this bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error interrupt enable bit When this bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBK general error interrupt enable bit OBKERRIE enables generating an interrupt in case of OBK specific access error. This bit can be programmed only when LOCK bit is cleared to 0."]
        #[inline(always)]
        pub const fn obkerrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBK general error interrupt enable bit OBKERRIE enables generating an interrupt in case of OBK specific access error. This bit can be programmed only when LOCK bit is cleared to 0."]
        #[inline(always)]
        pub fn set_obkerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBK write error interrupt enable bit OBKWERRIE enables generation of interrupt in case of OBK specific write error. This bit can be programmed only when LOCK bit is cleared to 0."]
        #[inline(always)]
        pub const fn obkwerrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBK write error interrupt enable bit OBKWERRIE enables generation of interrupt in case of OBK specific write error. This bit can be programmed only when LOCK bit is cleared to 0."]
        #[inline(always)]
        pub fn set_obkwerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Option byte change error interrupt enable bit This bit controls if an interrupt must be generated when an error occurs during an option byte change. It can be programmed only when LOCK bit is cleared to 0."]
        #[inline(always)]
        pub const fn optchangeerrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error interrupt enable bit This bit controls if an interrupt must be generated when an error occurs during an option byte change. It can be programmed only when LOCK bit is cleared to 0."]
        #[inline(always)]
        pub fn set_optchangeerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
        #[inline(always)]
        pub const fn bksel(&self) -> super::vals::NscrBksel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::NscrBksel::from_bits(val as u8)
        }
        #[doc = "Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
        #[inline(always)]
        pub fn set_bksel(&mut self, val: super::vals::NscrBksel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Nscr {
        #[inline(always)]
        fn default() -> Nscr {
            Nscr(0)
        }
    }
    impl core::fmt::Debug for Nscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nscr")
                .field("lock", &self.lock())
                .field("pg", &self.pg())
                .field("ser", &self.ser())
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("strt", &self.strt())
                .field("snb", &self.snb())
                .field("mer", &self.mer())
                .field("eopie", &self.eopie())
                .field("wrperrie", &self.wrperrie())
                .field("pgserrie", &self.pgserrie())
                .field("strberrie", &self.strberrie())
                .field("incerrie", &self.incerrie())
                .field("obkerrie", &self.obkerrie())
                .field("obkwerrie", &self.obkwerrie())
                .field("optchangeerrie", &self.optchangeerrie())
                .field("bksel", &self.bksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nscr {{ lock: {=bool:?}, pg: {=bool:?}, ser: {=bool:?}, ber: {=bool:?}, fw: {=bool:?}, strt: {=bool:?}, snb: {=u8:?}, mer: {=bool:?}, eopie: {=bool:?}, wrperrie: {=bool:?}, pgserrie: {=bool:?}, strberrie: {=bool:?}, incerrie: {=bool:?}, obkerrie: {=bool:?}, obkwerrie: {=bool:?}, optchangeerrie: {=bool:?}, bksel: {:?} }}" , self . lock () , self . pg () , self . ser () , self . ber () , self . fw () , self . strt () , self . snb () , self . mer () , self . eopie () , self . wrperrie () , self . pgserrie () , self . strberrie () , self . incerrie () , self . obkerrie () , self . obkwerrie () , self . optchangeerrie () , self . bksel ())
        }
    }
    #[doc = "FLASH non-secure EPOCH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsepochr(pub u32);
    impl Nsepochr {
        #[doc = "Non-volatile non-secure EPOCH counter"]
        #[inline(always)]
        pub const fn ns_epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Non-volatile non-secure EPOCH counter"]
        #[inline(always)]
        pub fn set_ns_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Nsepochr {
        #[inline(always)]
        fn default() -> Nsepochr {
            Nsepochr(0)
        }
    }
    impl core::fmt::Debug for Nsepochr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsepochr").field("ns_epoch", &self.ns_epoch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsepochr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nsepochr {{ ns_epoch: {=u32:?} }}", self.ns_epoch())
        }
    }
    #[doc = "FLASH non-secure OBK configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsobkcfgr(pub u32);
    impl Nsobkcfgr {
        #[doc = "OBKCFGR lock option configuration bit This bit locks the FLASH_NSOBKCFGR register. The correct write sequence to FLASH_NSOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OBKCFGR lock option configuration bit This bit locks the FLASH_NSOBKCFGR register. The correct write sequence to FLASH_NSOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL ≠ 0. Minimum value is 16 for OBK-HDPL = 1, 144 for OBK-HDPL = 2 and 192 for OBK-HDPL = 3."]
        #[inline(always)]
        pub const fn swap_sect_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL ≠ 0. Minimum value is 16 for OBK-HDPL = 1, 144 for OBK-HDPL = 2 and 192 for OBK-HDPL = 3."]
        #[inline(always)]
        pub fn set_swap_sect_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "alternate sector bit This bit must not change while filling the write buffer, otherwise an error (OBKERR) is generated"]
        #[inline(always)]
        pub const fn alt_sect(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "alternate sector bit This bit must not change while filling the write buffer, otherwise an error (OBKERR) is generated"]
        #[inline(always)]
        pub fn set_alt_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as BUSY bit."]
        #[inline(always)]
        pub const fn alt_sect_erase(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as BUSY bit."]
        #[inline(always)]
        pub fn set_alt_sect_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Key index (offset /16 bits) pointing for next swap. 0x01 means that only the first OBK data (128 bits) is copied from current to alternate OBK sector 0x02 means that the two first OBK data is copied … …"]
        #[inline(always)]
        pub const fn swap_offset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Key index (offset /16 bits) pointing for next swap. 0x01 means that only the first OBK data (128 bits) is copied from current to alternate OBK sector 0x02 means that the two first OBK data is copied … …"]
        #[inline(always)]
        pub fn set_swap_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Nsobkcfgr {
        #[inline(always)]
        fn default() -> Nsobkcfgr {
            Nsobkcfgr(0)
        }
    }
    impl core::fmt::Debug for Nsobkcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsobkcfgr")
                .field("lock", &self.lock())
                .field("swap_sect_req", &self.swap_sect_req())
                .field("alt_sect", &self.alt_sect())
                .field("alt_sect_erase", &self.alt_sect_erase())
                .field("swap_offset", &self.swap_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsobkcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nsobkcfgr {{ lock: {=bool:?}, swap_sect_req: {=bool:?}, alt_sect: {=bool:?}, alt_sect_erase: {=bool:?}, swap_offset: {=u16:?} }}" , self . lock () , self . swap_sect_req () , self . alt_sect () , self . alt_sect_erase () , self . swap_offset ())
        }
    }
    #[doc = "FLASH non-secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nssr(pub u32);
    impl Nssr {
        #[doc = "busy flag BSY flag indicates that a flash memory is busy by an operation (write, erase, option byte change, OBK operation). It is set at the beginning of a flash memory operation and cleared when the operation finishes, or an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "busy flag BSY flag indicates that a flash memory is busy by an operation (write, erase, option byte change, OBK operation). It is set at the beginning of a flash memory operation and cleared when the operation finishes, or an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "write buffer not empty flag WBNE flag is set when the flash interface is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "write buffer not empty flag WBNE flag is set when the flash interface is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "data buffer not empty flag DBNE flag is set when the flash interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
        #[inline(always)]
        pub const fn dbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "data buffer not empty flag DBNE flag is set when the flash interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
        #[inline(always)]
        pub fn set_dbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR."]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR."]
        #[inline(always)]
        pub fn set_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error flag NSINCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears NSINCERR."]
        #[inline(always)]
        pub const fn incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "inconsistency error flag NSINCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears NSINCERR."]
        #[inline(always)]
        pub fn set_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBK general error flag OBKERR flag is raised when the OBK-HDPL signal from the SBS does not match the HDPL value associated with the key slot during access to the key location. Alternatively also when the ALT_SECT is unexpectedly changed while the write buffer is being filled."]
        #[inline(always)]
        pub const fn obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBK general error flag OBKERR flag is raised when the OBK-HDPL signal from the SBS does not match the HDPL value associated with the key slot during access to the key location. Alternatively also when the ALT_SECT is unexpectedly changed while the write buffer is being filled."]
        #[inline(always)]
        pub fn set_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBK write error flag OBKWERR flag is raised when the address is not virgin on a write access to the OBK storage. Alternatively also when the OBK selector in the alternate sector is not virgin during a swap operation."]
        #[inline(always)]
        pub const fn obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBK write error flag OBKWERR flag is raised when the address is not virgin on a write access to the OBK storage. Alternatively also when the OBK selector in the alternate sector is not virgin during a swap operation."]
        #[inline(always)]
        pub fn set_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_NSCCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set."]
        #[inline(always)]
        pub const fn optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_NSCCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set."]
        #[inline(always)]
        pub fn set_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Nssr {
        #[inline(always)]
        fn default() -> Nssr {
            Nssr(0)
        }
    }
    impl core::fmt::Debug for Nssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nssr")
                .field("bsy", &self.bsy())
                .field("wbne", &self.wbne())
                .field("dbne", &self.dbne())
                .field("eop", &self.eop())
                .field("wrperr", &self.wrperr())
                .field("pgserr", &self.pgserr())
                .field("strberr", &self.strberr())
                .field("incerr", &self.incerr())
                .field("obkerr", &self.obkerr())
                .field("obkwerr", &self.obkwerr())
                .field("optchangeerr", &self.optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nssr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nssr {{ bsy: {=bool:?}, wbne: {=bool:?}, dbne: {=bool:?}, eop: {=bool:?}, wrperr: {=bool:?}, pgserr: {=bool:?}, strberr: {=bool:?}, incerr: {=bool:?}, obkerr: {=bool:?}, obkwerr: {=bool:?}, optchangeerr: {=bool:?} }}" , self . bsy () , self . wbne () , self . dbne () , self . eop () , self . wrperr () , self . pgserr () , self . strberr () , self . incerr () , self . obkerr () , self . obkwerr () , self . optchangeerr ())
        }
    }
    #[doc = "FLASH operation status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opsr(pub u32);
    impl Opsr {
        #[doc = "Interrupted operation address"]
        #[inline(always)]
        pub const fn addr_op(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupted operation address"]
        #[inline(always)]
        pub fn set_addr_op(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Flash high-cycle data area operation interrupted It indicates if flash high-cycle data area is concerned by operation."]
        #[inline(always)]
        pub const fn data_op(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Flash high-cycle data area operation interrupted It indicates if flash high-cycle data area is concerned by operation."]
        #[inline(always)]
        pub fn set_data_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Interrupted operation bank It indicates which bank was concerned by operation."]
        #[inline(always)]
        pub const fn bk_op(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupted operation bank It indicates which bank was concerned by operation."]
        #[inline(always)]
        pub fn set_bk_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Operation in system flash memory interrupted Indicates that reset interrupted an ongoing operation in system flash."]
        #[inline(always)]
        pub const fn sysf_op(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Operation in system flash memory interrupted Indicates that reset interrupted an ongoing operation in system flash."]
        #[inline(always)]
        pub fn set_sysf_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area (or OBKeys area)."]
        #[inline(always)]
        pub const fn otp_op(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area (or OBKeys area)."]
        #[inline(always)]
        pub fn set_otp_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Flash memory operation code"]
        #[inline(always)]
        pub const fn code_op(&self) -> super::vals::CodeOp {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::CodeOp::from_bits(val as u8)
        }
        #[doc = "Flash memory operation code"]
        #[inline(always)]
        pub fn set_code_op(&mut self, val: super::vals::CodeOp) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Opsr {
        #[inline(always)]
        fn default() -> Opsr {
            Opsr(0)
        }
    }
    impl core::fmt::Debug for Opsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opsr")
                .field("addr_op", &self.addr_op())
                .field("data_op", &self.data_op())
                .field("bk_op", &self.bk_op())
                .field("sysf_op", &self.sysf_op())
                .field("otp_op", &self.otp_op())
                .field("code_op", &self.code_op())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Opsr {{ addr_op: {=u32:?}, data_op: {=bool:?}, bk_op: {=bool:?}, sysf_op: {=bool:?}, otp_op: {=bool:?}, code_op: {:?} }}" , self . addr_op () , self . data_op () , self . bk_op () , self . sysf_op () , self . otp_op () , self . code_op ())
        }
    }
    #[doc = "FLASH option control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr(pub u32);
    impl Optcr {
        #[doc = "FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It is set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It is reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG flash interface register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in nonvolatile memory."]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It is set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It is reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG flash interface register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in nonvolatile memory."]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR."]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::OptcrSwapBank {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OptcrSwapBank::from_bits(val as u8)
        }
        #[doc = "Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR."]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: super::vals::OptcrSwapBank) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optcr {
        #[inline(always)]
        fn default() -> Optcr {
            Optcr(0)
        }
    }
    impl core::fmt::Debug for Optcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optcr")
                .field("optlock", &self.optlock())
                .field("optstrt", &self.optstrt())
                .field("swap_bank", &self.swap_bank())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optcr {{ optlock: {=bool:?}, optstrt: {=bool:?}, swap_bank: {:?} }}",
                self.optlock(),
                self.optstrt(),
                self.swap_bank()
            )
        }
    }
    #[doc = "FLASH option status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr(pub u32);
    impl Optsr {
        #[doc = "Brownout level option status bit These bits reflects the power level that generates a system reset. 00 or 11: BOR Level 1, the threshold level is low (around 2.1�V)"]
        #[inline(always)]
        pub const fn bor_lev(&self) -> super::vals::OptsrBorLev {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::OptsrBorLev::from_bits(val as u8)
        }
        #[doc = "Brownout level option status bit These bits reflects the power level that generates a system reset. 00 or 11: BOR Level 1, the threshold level is low (around 2.1�V)"]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: super::vals::OptsrBorLev) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Brownout high enable"]
        #[inline(always)]
        pub const fn borh_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Brownout high enable"]
        #[inline(always)]
        pub fn set_borh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IWDG control mode option status bit"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::OptsrIwdgSw {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::OptsrIwdgSw::from_bits(val as u8)
        }
        #[doc = "IWDG control mode option status bit"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: super::vals::OptsrIwdgSw) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WWDG control mode option status bit"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> super::vals::OptsrWwdgSw {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrWwdgSw::from_bits(val as u8)
        }
        #[doc = "WWDG control mode option status bit"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: super::vals::OptsrWwdgSw) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Core domain Stop entry reset option status bit"]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> super::vals::OptsrNrstStop {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrNrstStop::from_bits(val as u8)
        }
        #[doc = "Core domain Stop entry reset option status bit"]
        #[inline(always)]
        pub fn set_nrst_stop(&mut self, val: super::vals::OptsrNrstStop) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Core domain Standby entry reset option status bit"]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> super::vals::OptsrNrstStdby {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::OptsrNrstStdby::from_bits(val as u8)
        }
        #[doc = "Core domain Standby entry reset option status bit"]
        #[inline(always)]
        pub fn set_nrst_stdby(&mut self, val: super::vals::OptsrNrstStdby) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Life state code (based on Hamming 8,4). More information in Section�7.6.11: Product state transitions."]
        #[inline(always)]
        pub const fn product_state(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Life state code (based on Hamming 8,4). More information in Section�7.6.11: Product state transitions."]
        #[inline(always)]
        pub fn set_product_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "High-speed IO at low V<sub>DD</sub> voltage configuration bit. This bit can be set only with V<sub>DD</sub> below 2.7�V."]
        #[inline(always)]
        pub const fn io_vdd_hslv(&self) -> super::vals::OptsrIoVddHslv {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::OptsrIoVddHslv::from_bits(val as u8)
        }
        #[doc = "High-speed IO at low V<sub>DD</sub> voltage configuration bit. This bit can be set only with V<sub>DD</sub> below 2.7�V."]
        #[inline(always)]
        pub fn set_io_vdd_hslv(&mut self, val: super::vals::OptsrIoVddHslv) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "High-speed IO at low V<sub>DDIO2</sub> voltage configuration bit. This bit can be set only with V<sub>DDIO2</sub> below 2.7�V."]
        #[inline(always)]
        pub const fn io_vddio2_hslv(&self) -> super::vals::OptsrIoVddioHslv {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::OptsrIoVddioHslv::from_bits(val as u8)
        }
        #[doc = "High-speed IO at low V<sub>DDIO2</sub> voltage configuration bit. This bit can be set only with V<sub>DDIO2</sub> below 2.7�V."]
        #[inline(always)]
        pub fn set_io_vddio2_hslv(&mut self, val: super::vals::OptsrIoVddioHslv) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode."]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> super::vals::OptsrIwdgStop {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::OptsrIwdgStop::from_bits(val as u8)
        }
        #[doc = "IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode."]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: super::vals::OptsrIwdgStop) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode."]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> super::vals::OptsrIwdgStdby {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::OptsrIwdgStdby::from_bits(val as u8)
        }
        #[doc = "IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode."]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: super::vals::OptsrIwdgStdby) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Available only on cryptography enabled devices. Unique boot entry control, selects either ST or OEM iRoT for secure boot."]
        #[inline(always)]
        pub const fn boot_ube(&self) -> super::vals::OptsrBootUbe {
            let val = (self.0 >> 22usize) & 0xff;
            super::vals::OptsrBootUbe::from_bits(val as u8)
        }
        #[doc = "Available only on cryptography enabled devices. Unique boot entry control, selects either ST or OEM iRoT for secure boot."]
        #[inline(always)]
        pub fn set_boot_ube(&mut self, val: super::vals::OptsrBootUbe) {
            self.0 = (self.0 & !(0xff << 22usize)) | (((val.to_bits() as u32) & 0xff) << 22usize);
        }
        #[doc = "Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset."]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::OptsrSwapBank {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OptsrSwapBank::from_bits(val as u8)
        }
        #[doc = "Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset."]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: super::vals::OptsrSwapBank) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optsr {
        #[inline(always)]
        fn default() -> Optsr {
            Optsr(0)
        }
    }
    impl core::fmt::Debug for Optsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr")
                .field("bor_lev", &self.bor_lev())
                .field("borh_en", &self.borh_en())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stdby", &self.nrst_stdby())
                .field("product_state", &self.product_state())
                .field("io_vdd_hslv", &self.io_vdd_hslv())
                .field("io_vddio2_hslv", &self.io_vddio2_hslv())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("boot_ube", &self.boot_ube())
                .field("swap_bank", &self.swap_bank())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optsr {{ bor_lev: {:?}, borh_en: {=bool:?}, iwdg_sw: {:?}, wwdg_sw: {:?}, nrst_stop: {:?}, nrst_stdby: {:?}, product_state: {=u8:?}, io_vdd_hslv: {:?}, io_vddio2_hslv: {:?}, iwdg_stop: {:?}, iwdg_stdby: {:?}, boot_ube: {:?}, swap_bank: {:?} }}" , self . bor_lev () , self . borh_en () , self . iwdg_sw () , self . wwdg_sw () , self . nrst_stop () , self . nrst_stdby () , self . product_state () , self . io_vdd_hslv () , self . io_vddio2_hslv () , self . iwdg_stop () , self . iwdg_stdby () , self . boot_ube () , self . swap_bank ())
        }
    }
    #[doc = "FLASH option status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr2(pub u32);
    impl Optsr2 {
        #[doc = "SRAM1 and SRAM3 erase upon system reset"]
        #[inline(always)]
        pub const fn sram13_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 and SRAM3 erase upon system reset"]
        #[inline(always)]
        pub fn set_sram13_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SRAM2 erase when system reset"]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase when system reset"]
        #[inline(always)]
        pub fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Backup RAM ECC detection and correction disable"]
        #[inline(always)]
        pub const fn bkpram_ecc(&self) -> super::vals::OptsrBkpramEcc {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrBkpramEcc::from_bits(val as u8)
        }
        #[doc = "Backup RAM ECC detection and correction disable"]
        #[inline(always)]
        pub fn set_bkpram_ecc(&mut self, val: super::vals::OptsrBkpramEcc) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM3 ECC detection and correction disable"]
        #[inline(always)]
        pub const fn sram3_ecc(&self) -> super::vals::OptsrSramEcc {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::OptsrSramEcc::from_bits(val as u8)
        }
        #[doc = "SRAM3 ECC detection and correction disable"]
        #[inline(always)]
        pub fn set_sram3_ecc(&mut self, val: super::vals::OptsrSramEcc) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "SRAM2 ECC detection and correction disable"]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> super::vals::OptsrSramEcc {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrSramEcc::from_bits(val as u8)
        }
        #[doc = "SRAM2 ECC detection and correction disable"]
        #[inline(always)]
        pub fn set_sram2_ecc(&mut self, val: super::vals::OptsrSramEcc) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "USB power delivery configuration option bit"]
        #[inline(always)]
        pub const fn usbpd_dis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "USB power delivery configuration option bit"]
        #[inline(always)]
        pub fn set_usbpd_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TrustZone enable configuration bits This bit enables the device is in TrustZone mode during an option byte change."]
        #[inline(always)]
        pub const fn tzen(&self) -> super::vals::OptsrTzen {
            let val = (self.0 >> 24usize) & 0xff;
            super::vals::OptsrTzen::from_bits(val as u8)
        }
        #[doc = "TrustZone enable configuration bits This bit enables the device is in TrustZone mode during an option byte change."]
        #[inline(always)]
        pub fn set_tzen(&mut self, val: super::vals::OptsrTzen) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Optsr2 {
        #[inline(always)]
        fn default() -> Optsr2 {
            Optsr2(0)
        }
    }
    impl core::fmt::Debug for Optsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr2")
                .field("sram13_rst", &self.sram13_rst())
                .field("sram2_rst", &self.sram2_rst())
                .field("bkpram_ecc", &self.bkpram_ecc())
                .field("sram3_ecc", &self.sram3_ecc())
                .field("sram2_ecc", &self.sram2_ecc())
                .field("usbpd_dis", &self.usbpd_dis())
                .field("tzen", &self.tzen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optsr2 {{ sram13_rst: {=bool:?}, sram2_rst: {=bool:?}, bkpram_ecc: {:?}, sram3_ecc: {:?}, sram2_ecc: {:?}, usbpd_dis: {=bool:?}, tzen: {:?} }}" , self . sram13_rst () , self . sram2_rst () , self . bkpram_ecc () , self . sram3_ecc () , self . sram2_ecc () , self . usbpd_dis () , self . tzen ())
        }
    }
    #[doc = "FLASH non-secure OTP block lock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otpblr(pub u32);
    impl Otpblr {
        #[doc = "OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it’s not possible to remove the write protection. Also if not locked, it is not possible to erase OTP words."]
        #[inline(always)]
        pub const fn lockbl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it’s not possible to remove the write protection. Also if not locked, it is not possible to erase OTP words."]
        #[inline(always)]
        pub fn set_lockbl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Otpblr {
        #[inline(always)]
        fn default() -> Otpblr {
            Otpblr(0)
        }
    }
    impl core::fmt::Debug for Otpblr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otpblr").field("lockbl", &self.lockbl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otpblr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Otpblr {{ lockbl: {=u32:?} }}", self.lockbl())
        }
    }
    #[doc = "FLASH privilege block-based register for Bank 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb(pub u32);
    impl Privbb {
        #[doc = "Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute"]
        #[inline(always)]
        pub const fn privbb(&self) -> super::vals::PrivbbrPrivbb {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::PrivbbrPrivbb::from_bits(val as u32)
        }
        #[doc = "Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute"]
        #[inline(always)]
        pub fn set_privbb(&mut self, val: super::vals::PrivbbrPrivbb) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Privbb {
        #[inline(always)]
        fn default() -> Privbb {
            Privbb(0)
        }
    }
    impl core::fmt::Debug for Privbb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb").field("privbb", &self.privbb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Privbb {{ privbb: {:?} }}", self.privbb())
        }
    }
    #[doc = "FLASH privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "privilege attribute for secure registers"]
        #[inline(always)]
        pub const fn spriv(&self) -> super::vals::Spriv {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Spriv::from_bits(val as u8)
        }
        #[doc = "privilege attribute for secure registers"]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: super::vals::Spriv) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "privilege attribute for non secure registers"]
        #[inline(always)]
        pub const fn nspriv(&self) -> super::vals::Nspriv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Nspriv::from_bits(val as u8)
        }
        #[doc = "privilege attribute for non secure registers"]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: super::vals::Nspriv) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
                .field("spriv", &self.spriv())
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {:?}, nspriv: {:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[doc = "FLASH secure block-based register for Bank 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb(pub u32);
    impl Secbb {
        #[doc = "Secure/non-secure flash Bank 2 sector attribute"]
        #[inline(always)]
        pub const fn secbb(&self) -> super::vals::SecbbrSecbb {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::SecbbrSecbb::from_bits(val as u32)
        }
        #[doc = "Secure/non-secure flash Bank 2 sector attribute"]
        #[inline(always)]
        pub fn set_secbb(&mut self, val: super::vals::SecbbrSecbb) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb {
        #[inline(always)]
        fn default() -> Secbb {
            Secbb(0)
        }
    }
    impl core::fmt::Debug for Secbb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb").field("secbb", &self.secbb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb {{ secbb: {:?} }}", self.secbb())
        }
    }
    #[doc = "FLASH secure boot register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbootr(pub u32);
    impl Secbootr {
        #[doc = "A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings."]
        #[inline(always)]
        pub const fn secboot_lock(&self) -> super::vals::SecbootrSecbootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::SecbootrSecbootLock::from_bits(val as u8)
        }
        #[doc = "A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings."]
        #[inline(always)]
        pub fn set_secboot_lock(&mut self, val: super::vals::SecbootrSecbootLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Unique boot entry secure address These bits reflect the Secure UBE address"]
        #[inline(always)]
        pub const fn secbootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Unique boot entry secure address These bits reflect the Secure UBE address"]
        #[inline(always)]
        pub fn set_secbootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Secbootr {
        #[inline(always)]
        fn default() -> Secbootr {
            Secbootr(0)
        }
    }
    impl core::fmt::Debug for Secbootr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbootr")
                .field("secboot_lock", &self.secboot_lock())
                .field("secbootadd", &self.secbootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbootr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secbootr {{ secboot_lock: {:?}, secbootadd: {=u32:?} }}",
                self.secboot_lock(),
                self.secbootadd()
            )
        }
    }
    #[doc = "FLASH secure clear control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secccr(pub u32);
    impl Secccr {
        #[doc = "EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_SECSR register."]
        #[inline(always)]
        pub const fn clr_eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_SECSR register."]
        #[inline(always)]
        pub fn set_clr_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub const fn clr_wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub fn set_clr_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub const fn clr_pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub fn set_clr_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub const fn clr_strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub fn set_clr_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub const fn clr_incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub fn set_clr_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBKWERR flag clear bit Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub const fn clr_obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBKWERR flag clear bit Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub fn set_clr_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBKWERR flag clear bit Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub const fn clr_obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBKWERR flag clear bit Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register."]
        #[inline(always)]
        pub fn set_clr_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Secccr {
        #[inline(always)]
        fn default() -> Secccr {
            Secccr(0)
        }
    }
    impl core::fmt::Debug for Secccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secccr")
                .field("clr_eop", &self.clr_eop())
                .field("clr_wrperr", &self.clr_wrperr())
                .field("clr_pgserr", &self.clr_pgserr())
                .field("clr_strberr", &self.clr_strberr())
                .field("clr_incerr", &self.clr_incerr())
                .field("clr_obkerr", &self.clr_obkerr())
                .field("clr_obkwerr", &self.clr_obkwerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secccr {{ clr_eop: {=bool:?}, clr_wrperr: {=bool:?}, clr_pgserr: {=bool:?}, clr_strberr: {=bool:?}, clr_incerr: {=bool:?}, clr_obkerr: {=bool:?}, clr_obkwerr: {=bool:?} }}" , self . clr_eop () , self . clr_wrperr () , self . clr_pgserr () , self . clr_strberr () , self . clr_incerr () , self . clr_obkerr () , self . clr_obkwerr ())
        }
    }
    #[doc = "FLASH secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccr(pub u32);
    impl Seccr {
        #[doc = "configuration lock bit This bit locks the FLASH_SECCR register. The correct write sequence to FLASH_SECKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_SECCR register do not change."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "configuration lock bit This bit locks the FLASH_SECCR register. The correct write sequence to FLASH_SECKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_SECCR register do not change."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If BER and MER are also set, a PGSERR is raised."]
        #[inline(always)]
        pub const fn ser(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If BER and MER are also set, a PGSERR is raised."]
        #[inline(always)]
        pub fn set_ser(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "erase request Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "erase request Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
        #[inline(always)]
        pub fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty and was filled by secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty and was filled by secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
        #[inline(always)]
        pub fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reseted at the end of the operation or when an error occurs. It cannot be reset by software."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reseted at the end of the operation or when an error occurs. It cannot be reset by software."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. .."]
        #[inline(always)]
        pub const fn snb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x7f;
            val as u8
        }
        #[doc = "sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. .."]
        #[inline(always)]
        pub fn set_snb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 6usize)) | (((val as u32) & 0x7f) << 6usize);
        }
        #[doc = "mass erase request Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are also set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "mass erase request Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are also set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program/erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program/erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBK general error interrupt enable bit OBKERRIE enables generating an interrupt in case of OBK specific access error. OBKERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn obkerrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBK general error interrupt enable bit OBKERRIE enables generating an interrupt in case of OBK specific access error. OBKERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_obkerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBK write error interrupt enable bit OBKWERRIE enables generation of interrupt in case of OBK specific write error. OBKWERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn obkwerrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBK write error interrupt enable bit OBKWERRIE enables generation of interrupt in case of OBK specific write error. OBKWERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_obkwerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Flash memory security state invert. This bit inverts the flash memory security state."]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory security state invert. This bit inverts the flash memory security state."]
        #[inline(always)]
        pub fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
        #[inline(always)]
        pub const fn bksel(&self) -> super::vals::SeccrBksel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::SeccrBksel::from_bits(val as u8)
        }
        #[doc = "bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
        #[inline(always)]
        pub fn set_bksel(&mut self, val: super::vals::SeccrBksel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccr {
        #[inline(always)]
        fn default() -> Seccr {
            Seccr(0)
        }
    }
    impl core::fmt::Debug for Seccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccr")
                .field("lock", &self.lock())
                .field("pg", &self.pg())
                .field("ser", &self.ser())
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("strt", &self.strt())
                .field("snb", &self.snb())
                .field("mer", &self.mer())
                .field("eopie", &self.eopie())
                .field("wrperrie", &self.wrperrie())
                .field("pgserrie", &self.pgserrie())
                .field("strberrie", &self.strberrie())
                .field("incerrie", &self.incerrie())
                .field("obkerrie", &self.obkerrie())
                .field("obkwerrie", &self.obkwerrie())
                .field("inv", &self.inv())
                .field("bksel", &self.bksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccr {{ lock: {=bool:?}, pg: {=bool:?}, ser: {=bool:?}, ber: {=bool:?}, fw: {=bool:?}, strt: {=bool:?}, snb: {=u8:?}, mer: {=bool:?}, eopie: {=bool:?}, wrperrie: {=bool:?}, pgserrie: {=bool:?}, strberrie: {=bool:?}, incerrie: {=bool:?}, obkerrie: {=bool:?}, obkwerrie: {=bool:?}, inv: {=bool:?}, bksel: {:?} }}" , self . lock () , self . pg () , self . ser () , self . ber () , self . fw () , self . strt () , self . snb () , self . mer () , self . eopie () , self . wrperrie () , self . pgserrie () , self . strberrie () , self . incerrie () , self . obkerrie () , self . obkwerrie () , self . inv () , self . bksel ())
        }
    }
    #[doc = "FLASH secure EPOCH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secepochr(pub u32);
    impl Secepochr {
        #[doc = "Non-volatile secure EPOCH counter"]
        #[inline(always)]
        pub const fn sec_epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Non-volatile secure EPOCH counter"]
        #[inline(always)]
        pub fn set_sec_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Secepochr {
        #[inline(always)]
        fn default() -> Secepochr {
            Secepochr(0)
        }
    }
    impl core::fmt::Debug for Secepochr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secepochr")
                .field("sec_epoch", &self.sec_epoch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secepochr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secepochr {{ sec_epoch: {=u32:?} }}", self.sec_epoch())
        }
    }
    #[doc = "FLASH secure OBK configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secobkcfgr(pub u32);
    impl Secobkcfgr {
        #[doc = "OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL ≠ 0. Minimum value is 16 for OBK-HDPL = 1, 144 for OBK-HDPL = 2 and 192 for OBK-HDPL = 3."]
        #[inline(always)]
        pub const fn swap_sect_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL ≠ 0. Minimum value is 16 for OBK-HDPL = 1, 144 for OBK-HDPL = 2 and 192 for OBK-HDPL = 3."]
        #[inline(always)]
        pub fn set_swap_sect_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated"]
        #[inline(always)]
        pub const fn alt_sect(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated"]
        #[inline(always)]
        pub fn set_alt_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit."]
        #[inline(always)]
        pub const fn alt_sect_erase(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit."]
        #[inline(always)]
        pub fn set_alt_sect_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "key index (offset /16 bits) pointing for next swap. …"]
        #[inline(always)]
        pub const fn swap_offset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "key index (offset /16 bits) pointing for next swap. …"]
        #[inline(always)]
        pub fn set_swap_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Secobkcfgr {
        #[inline(always)]
        fn default() -> Secobkcfgr {
            Secobkcfgr(0)
        }
    }
    impl core::fmt::Debug for Secobkcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secobkcfgr")
                .field("lock", &self.lock())
                .field("swap_sect_req", &self.swap_sect_req())
                .field("alt_sect", &self.alt_sect())
                .field("alt_sect_erase", &self.alt_sect_erase())
                .field("swap_offset", &self.swap_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secobkcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secobkcfgr {{ lock: {=bool:?}, swap_sect_req: {=bool:?}, alt_sect: {=bool:?}, alt_sect_erase: {=bool:?}, swap_offset: {=u16:?} }}" , self . lock () , self . swap_sect_req () , self . alt_sect () , self . alt_sect_erase () , self . swap_offset ())
        }
    }
    #[doc = "FLASH secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secsr(pub u32);
    impl Secsr {
        #[doc = "busy flag BSY flag indicates that a FLASH memory is busy (write, erase, option byte change, OBK operations). It is set at the beginning of a flash memory operation and cleared when the operation finishes or an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "busy flag BSY flag indicates that a FLASH memory is busy (write, erase, option byte change, OBK operations). It is set at the beginning of a flash memory operation and cleared when the operation finishes or an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "write buffer not empty flag WBNE flag is set when the flash interface is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the flash interface detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "write buffer not empty flag WBNE flag is set when the flash interface is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the flash interface detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "data buffer not empty flag DBNE flag is set when the embedded flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
        #[inline(always)]
        pub const fn dbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "data buffer not empty flag DBNE flag is set when the embedded flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
        #[inline(always)]
        pub fn set_dbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
        #[inline(always)]
        pub fn set_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
        #[inline(always)]
        pub const fn incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
        #[inline(always)]
        pub fn set_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBK general error flag OBKERR flag is raised when the OBK-HDPL signal from the SBS does not match the HDPL value associated with the key slot during access to the key location. Alternatively also when the ALT_SECT is unexpectedly changed while the write buffer is being filled."]
        #[inline(always)]
        pub const fn obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBK general error flag OBKERR flag is raised when the OBK-HDPL signal from the SBS does not match the HDPL value associated with the key slot during access to the key location. Alternatively also when the ALT_SECT is unexpectedly changed while the write buffer is being filled."]
        #[inline(always)]
        pub fn set_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBK write error flag OBKWERR flag is raised when the address is not virgin on a write access to the OBK storage. Alternatively also when the OBK selector in the alternate sector is not virgin during a swap operation."]
        #[inline(always)]
        pub const fn obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBK write error flag OBKWERR flag is raised when the address is not virgin on a write access to the OBK storage. Alternatively also when the OBK selector in the alternate sector is not virgin during a swap operation."]
        #[inline(always)]
        pub fn set_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Secsr {
        #[inline(always)]
        fn default() -> Secsr {
            Secsr(0)
        }
    }
    impl core::fmt::Debug for Secsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secsr")
                .field("bsy", &self.bsy())
                .field("wbne", &self.wbne())
                .field("dbne", &self.dbne())
                .field("eop", &self.eop())
                .field("wrperr", &self.wrperr())
                .field("pgserr", &self.pgserr())
                .field("strberr", &self.strberr())
                .field("incerr", &self.incerr())
                .field("obkerr", &self.obkerr())
                .field("obkwerr", &self.obkwerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secsr {{ bsy: {=bool:?}, wbne: {=bool:?}, dbne: {=bool:?}, eop: {=bool:?}, wrperr: {=bool:?}, pgserr: {=bool:?}, strberr: {=bool:?}, incerr: {=bool:?}, obkerr: {=bool:?}, obkwerr: {=bool:?} }}" , self . bsy () , self . wbne () , self . dbne () , self . eop () , self . wrperr () , self . pgserr () , self . strberr () , self . incerr () , self . obkerr () , self . obkwerr ())
        }
    }
    #[doc = "FLASH security watermark for Bank 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm(pub u32);
    impl Secwm {
        #[doc = "Bank2 security WM area start sector"]
        #[inline(always)]
        pub const fn secwmstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank2 security WM area start sector"]
        #[inline(always)]
        pub fn set_secwmstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank2 security WM end sector"]
        #[inline(always)]
        pub const fn secwmend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank2 security WM end sector"]
        #[inline(always)]
        pub fn set_secwmend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Secwm {
        #[inline(always)]
        fn default() -> Secwm {
            Secwm(0)
        }
    }
    impl core::fmt::Debug for Secwm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm")
                .field("secwmstrt", &self.secwmstrt())
                .field("secwmend", &self.secwmend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm {{ secwmstrt: {=u8:?}, secwmend: {=u8:?} }}",
                self.secwmstrt(),
                self.secwmend()
            )
        }
    }
    #[doc = "FLASH write sector group protection for Bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp(pub u32);
    impl Wrp {
        #[doc = "Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
        #[inline(always)]
        pub const fn wrpsg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
        #[inline(always)]
        pub fn set_wrpsg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrp {
        #[inline(always)]
        fn default() -> Wrp {
            Wrp(0)
        }
    }
    impl core::fmt::Debug for Wrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp").field("wrpsg", &self.wrpsg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp {{ wrpsg: {=u32:?} }}", self.wrpsg())
        }
    }
}
pub mod vals {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct BootrSecbootLock(u8);
    impl BootrSecbootLock {
        #[doc = "The BOOT_UBE and SECBOOTADD are frozen. SWAP_BANK can only be modified with TZEN set to 0xC3 (disabled)."]
        pub const B_0X_B4: Self = Self(0xb4);
        #[doc = "The BOOT_UBE, SWAP_ BANK and SECBOOTADD can still be modified following their individual rules."]
        pub const B_0X_C3: Self = Self(0xc3);
    }
    impl BootrSecbootLock {
        pub const fn from_bits(val: u8) -> BootrSecbootLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for BootrSecbootLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B_0X_B4"),
                0xc3 => f.write_str("B_0X_C3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BootrSecbootLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B_0X_B4"),
                0xc3 => defmt::write!(f, "B_0X_C3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for BootrSecbootLock {
        #[inline(always)]
        fn from(val: u8) -> BootrSecbootLock {
            BootrSecbootLock::from_bits(val)
        }
    }
    impl From<BootrSecbootLock> for u8 {
        #[inline(always)]
        fn from(val: BootrSecbootLock) -> u8 {
            BootrSecbootLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CodeOp {
        #[doc = "No flash operation on going during previous reset"]
        B_0X0 = 0x0,
        #[doc = "Single write operation interrupted"]
        B_0X1 = 0x01,
        #[doc = "OBK alternate sector erase"]
        B_0X2 = 0x02,
        #[doc = "Sector erase operation interrupted"]
        B_0X3 = 0x03,
        #[doc = "Bank erase operation interrupted"]
        B_0X4 = 0x04,
        #[doc = "Mass erase operation interrupted"]
        B_0X5 = 0x05,
        #[doc = "Option change operation interrupted"]
        B_0X6 = 0x06,
        #[doc = "OBK swap sector request"]
        B_0X7 = 0x07,
    }
    impl CodeOp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CodeOp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CodeOp {
        #[inline(always)]
        fn from(val: u8) -> CodeOp {
            CodeOp::from_bits(val)
        }
    }
    impl From<CodeOp> for u8 {
        #[inline(always)]
        fn from(val: CodeOp) -> u8 {
            CodeOp::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct NsbootrNsbootLock(u8);
    impl NsbootrNsbootLock {
        #[doc = "The NSBOOTADD is frozen. SWAP_ BANK can only be modified with TZEN set to 0xB4 (enabled)."]
        pub const B_0X_B4: Self = Self(0xb4);
        #[doc = "The SWAP_ BANK and NSBOOTADD can still be modified following their individual rules."]
        pub const B_0X_C3: Self = Self(0xc3);
    }
    impl NsbootrNsbootLock {
        pub const fn from_bits(val: u8) -> NsbootrNsbootLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for NsbootrNsbootLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B_0X_B4"),
                0xc3 => f.write_str("B_0X_C3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NsbootrNsbootLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B_0X_B4"),
                0xc3 => defmt::write!(f, "B_0X_C3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for NsbootrNsbootLock {
        #[inline(always)]
        fn from(val: u8) -> NsbootrNsbootLock {
            NsbootrNsbootLock::from_bits(val)
        }
    }
    impl From<NsbootrNsbootLock> for u8 {
        #[inline(always)]
        fn from(val: NsbootrNsbootLock) -> u8 {
            NsbootrNsbootLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum NscrBksel {
        #[doc = "Bank1 is selected for Bank erase / sector erase / interrupt enable"]
        B_0X0 = 0x0,
        #[doc = "Bank2 is selected for BER / SER"]
        B_0X1 = 0x01,
    }
    impl NscrBksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NscrBksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NscrBksel {
        #[inline(always)]
        fn from(val: u8) -> NscrBksel {
            NscrBksel::from_bits(val)
        }
    }
    impl From<NscrBksel> for u8 {
        #[inline(always)]
        fn from(val: NscrBksel) -> u8 {
            NscrBksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nspriv {
        #[doc = "access to non secure registers is always granted"]
        B_0X0 = 0x0,
        #[doc = "access to non secure registers is denied in case of unprivileged access."]
        B_0X1 = 0x01,
    }
    impl Nspriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nspriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nspriv {
        #[inline(always)]
        fn from(val: u8) -> Nspriv {
            Nspriv::from_bits(val)
        }
    }
    impl From<Nspriv> for u8 {
        #[inline(always)]
        fn from(val: Nspriv) -> u8 {
            Nspriv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptcrSwapBank {
        #[doc = "Bank1 and Bank2 not swapped"]
        B_0X0 = 0x0,
        #[doc = "Bank1 and Bank2 swapped"]
        B_0X1 = 0x01,
    }
    impl OptcrSwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptcrSwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptcrSwapBank {
        #[inline(always)]
        fn from(val: u8) -> OptcrSwapBank {
            OptcrSwapBank::from_bits(val)
        }
    }
    impl From<OptcrSwapBank> for u8 {
        #[inline(always)]
        fn from(val: OptcrSwapBank) -> u8 {
            OptcrSwapBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrBkpramEcc {
        #[doc = "BKPRAM ECC check enabled"]
        B_0X0 = 0x0,
        #[doc = "BKPRAM ECC check disabled"]
        B_0X1 = 0x01,
    }
    impl OptsrBkpramEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrBkpramEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrBkpramEcc {
        #[inline(always)]
        fn from(val: u8) -> OptsrBkpramEcc {
            OptsrBkpramEcc::from_bits(val)
        }
    }
    impl From<OptsrBkpramEcc> for u8 {
        #[inline(always)]
        fn from(val: OptsrBkpramEcc) -> u8 {
            OptsrBkpramEcc::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OptsrBootUbe(u8);
    impl OptsrBootUbe {
        #[doc = "OEM-iRoT (user flash) selected. In Open PRODUCT_STATE this value selects bootloader. Defaut value."]
        pub const B_0X_B4: Self = Self(0xb4);
        #[doc = "ST-iRoT (system flash) selected"]
        pub const B_0X_C3: Self = Self(0xc3);
    }
    impl OptsrBootUbe {
        pub const fn from_bits(val: u8) -> OptsrBootUbe {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for OptsrBootUbe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B_0X_B4"),
                0xc3 => f.write_str("B_0X_C3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OptsrBootUbe {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B_0X_B4"),
                0xc3 => defmt::write!(f, "B_0X_C3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for OptsrBootUbe {
        #[inline(always)]
        fn from(val: u8) -> OptsrBootUbe {
            OptsrBootUbe::from_bits(val)
        }
    }
    impl From<OptsrBootUbe> for u8 {
        #[inline(always)]
        fn from(val: OptsrBootUbe) -> u8 {
            OptsrBootUbe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrBorLev {
        _RESERVED_0 = 0x0,
        #[doc = "BOR Level 2, the threshold level is medium (around 2.4 V)"]
        B_0X1 = 0x01,
        #[doc = "BOR Level 3, the threshold level is high (around 2.7 V)"]
        B_0X2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl OptsrBorLev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrBorLev {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrBorLev {
        #[inline(always)]
        fn from(val: u8) -> OptsrBorLev {
            OptsrBorLev::from_bits(val)
        }
    }
    impl From<OptsrBorLev> for u8 {
        #[inline(always)]
        fn from(val: OptsrBorLev) -> u8 {
            OptsrBorLev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIoVddHslv {
        #[doc = "High-speed IO at low V<sub>DD</sub> voltage feature disabled (V<sub>DD</sub> can exceed 2.7�V)"]
        B_0X0 = 0x0,
        #[doc = "High-speed IO at low V<sub>DD</sub> voltage feature enabled (V<sub>DD</sub> remains below 2.7�V)"]
        B_0X1 = 0x01,
    }
    impl OptsrIoVddHslv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIoVddHslv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIoVddHslv {
        #[inline(always)]
        fn from(val: u8) -> OptsrIoVddHslv {
            OptsrIoVddHslv::from_bits(val)
        }
    }
    impl From<OptsrIoVddHslv> for u8 {
        #[inline(always)]
        fn from(val: OptsrIoVddHslv) -> u8 {
            OptsrIoVddHslv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIoVddioHslv {
        #[doc = "High-speed IO at low V<sub>DDIO2</sub> voltage feature disabled (V<sub>DDIO2</sub> can exceed 2.7�V)"]
        B_0X0 = 0x0,
        #[doc = "High-speed IO at low V<sub>DDIO2</sub> voltage feature enabled (V<sub>DDIO2</sub> remains below 2.7�V)"]
        B_0X1 = 0x01,
    }
    impl OptsrIoVddioHslv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIoVddioHslv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIoVddioHslv {
        #[inline(always)]
        fn from(val: u8) -> OptsrIoVddioHslv {
            OptsrIoVddioHslv::from_bits(val)
        }
    }
    impl From<OptsrIoVddioHslv> for u8 {
        #[inline(always)]
        fn from(val: OptsrIoVddioHslv) -> u8 {
            OptsrIoVddioHslv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgStdby {
        #[doc = "Independent watchdog frozen in Standby mode"]
        B_0X0 = 0x0,
        #[doc = "Independent watchdog keep running in Standby mode."]
        B_0X1 = 0x01,
    }
    impl OptsrIwdgStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgStdby {
            OptsrIwdgStdby::from_bits(val)
        }
    }
    impl From<OptsrIwdgStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgStdby) -> u8 {
            OptsrIwdgStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgStop {
        #[doc = "Independent watchdog frozen in system Stop mode"]
        B_0X0 = 0x0,
        #[doc = "Independent watchdog keep running in system Stop mode."]
        B_0X1 = 0x01,
    }
    impl OptsrIwdgStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgStop {
            OptsrIwdgStop::from_bits(val)
        }
    }
    impl From<OptsrIwdgStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgStop) -> u8 {
            OptsrIwdgStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgSw {
        #[doc = "IWDG watchdog is controlled by hardware"]
        B_0X0 = 0x0,
        #[doc = "IWDG watchdog is controlled by software"]
        B_0X1 = 0x01,
    }
    impl OptsrIwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgSw {
            OptsrIwdgSw::from_bits(val)
        }
    }
    impl From<OptsrIwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgSw) -> u8 {
            OptsrIwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrNrstStdby {
        #[doc = "a reset is generated when entering Standby mode on core domain"]
        B_0X0 = 0x0,
        #[doc = "no reset generated when entering Standby mode on core domain."]
        B_0X1 = 0x01,
    }
    impl OptsrNrstStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstStdby {
            OptsrNrstStdby::from_bits(val)
        }
    }
    impl From<OptsrNrstStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstStdby) -> u8 {
            OptsrNrstStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrNrstStop {
        #[doc = "a reset is generated when entering Stop mode on core domain"]
        B_0X0 = 0x0,
        #[doc = "no reset generated when entering Stop mode on core domain."]
        B_0X1 = 0x01,
    }
    impl OptsrNrstStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstStop {
            OptsrNrstStop::from_bits(val)
        }
    }
    impl From<OptsrNrstStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstStop) -> u8 {
            OptsrNrstStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrSramEcc {
        #[doc = "SRAM2 ECC check enabled"]
        B_0X0 = 0x0,
        #[doc = "SRAM2 ECC check disabled"]
        B_0X1 = 0x01,
    }
    impl OptsrSramEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrSramEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrSramEcc {
        #[inline(always)]
        fn from(val: u8) -> OptsrSramEcc {
            OptsrSramEcc::from_bits(val)
        }
    }
    impl From<OptsrSramEcc> for u8 {
        #[inline(always)]
        fn from(val: OptsrSramEcc) -> u8 {
            OptsrSramEcc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrSwapBank {
        #[doc = "Bank1 and Bank2 not swapped"]
        B_0X0 = 0x0,
        #[doc = "Bank1 and Bank2 swapped"]
        B_0X1 = 0x01,
    }
    impl OptsrSwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrSwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrSwapBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrSwapBank {
            OptsrSwapBank::from_bits(val)
        }
    }
    impl From<OptsrSwapBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrSwapBank) -> u8 {
            OptsrSwapBank::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OptsrTzen(u8);
    impl OptsrTzen {
        #[doc = "TrustZone enabled."]
        pub const B_0X_B4: Self = Self(0xb4);
        #[doc = "TrustZone disabled"]
        pub const B_0X_C3: Self = Self(0xc3);
    }
    impl OptsrTzen {
        pub const fn from_bits(val: u8) -> OptsrTzen {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for OptsrTzen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B_0X_B4"),
                0xc3 => f.write_str("B_0X_C3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OptsrTzen {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B_0X_B4"),
                0xc3 => defmt::write!(f, "B_0X_C3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for OptsrTzen {
        #[inline(always)]
        fn from(val: u8) -> OptsrTzen {
            OptsrTzen::from_bits(val)
        }
    }
    impl From<OptsrTzen> for u8 {
        #[inline(always)]
        fn from(val: OptsrTzen) -> u8 {
            OptsrTzen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrWwdgSw {
        #[doc = "WWDG watchdog is controlled by hardware"]
        B_0X0 = 0x0,
        #[doc = "WWDG watchdog is controlled by software"]
        B_0X1 = 0x01,
    }
    impl OptsrWwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrWwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrWwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrWwdgSw {
            OptsrWwdgSw::from_bits(val)
        }
    }
    impl From<OptsrWwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrWwdgSw) -> u8 {
            OptsrWwdgSw::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct PrivbbrPrivbb(u32);
    impl PrivbbrPrivbb {
        #[doc = "sectors (32 * (x-1) + y) in bank 2 is unprivileged"]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "sector (32 * (x-1) + y) in bank 2 is privileged"]
        pub const B_0X1: Self = Self(0x01);
    }
    impl PrivbbrPrivbb {
        pub const fn from_bits(val: u32) -> PrivbbrPrivbb {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for PrivbbrPrivbb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                0x01 => f.write_str("B_0X1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PrivbbrPrivbb {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                0x01 => defmt::write!(f, "B_0X1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for PrivbbrPrivbb {
        #[inline(always)]
        fn from(val: u32) -> PrivbbrPrivbb {
            PrivbbrPrivbb::from_bits(val)
        }
    }
    impl From<PrivbbrPrivbb> for u32 {
        #[inline(always)]
        fn from(val: PrivbbrPrivbb) -> u32 {
            PrivbbrPrivbb::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct SecbbrSecbb(u32);
    impl SecbbrSecbb {
        #[doc = "sectors (32 * (x-1) + y) in bank 2 is non secure"]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "sector (32 * (x-1) + y) in bank 2 is secure"]
        pub const B_0X1: Self = Self(0x01);
    }
    impl SecbbrSecbb {
        pub const fn from_bits(val: u32) -> SecbbrSecbb {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for SecbbrSecbb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                0x01 => f.write_str("B_0X1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SecbbrSecbb {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                0x01 => defmt::write!(f, "B_0X1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for SecbbrSecbb {
        #[inline(always)]
        fn from(val: u32) -> SecbbrSecbb {
            SecbbrSecbb::from_bits(val)
        }
    }
    impl From<SecbbrSecbb> for u32 {
        #[inline(always)]
        fn from(val: SecbbrSecbb) -> u32 {
            SecbbrSecbb::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct SecbootrSecbootLock(u8);
    impl SecbootrSecbootLock {
        #[doc = "The BOOT_UBE and SECBOOTADD are frozen. SWAP_ BANK can only be modified with TZEN set to 0xC3 (disabled)."]
        pub const B_0X_B4: Self = Self(0xb4);
        #[doc = "The BOOT_UBE, SWAP_BANK and SECBOOTADD can still be modified following their individual rules."]
        pub const B_0X_C3: Self = Self(0xc3);
    }
    impl SecbootrSecbootLock {
        pub const fn from_bits(val: u8) -> SecbootrSecbootLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for SecbootrSecbootLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B_0X_B4"),
                0xc3 => f.write_str("B_0X_C3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SecbootrSecbootLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B_0X_B4"),
                0xc3 => defmt::write!(f, "B_0X_C3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for SecbootrSecbootLock {
        #[inline(always)]
        fn from(val: u8) -> SecbootrSecbootLock {
            SecbootrSecbootLock::from_bits(val)
        }
    }
    impl From<SecbootrSecbootLock> for u8 {
        #[inline(always)]
        fn from(val: SecbootrSecbootLock) -> u8 {
            SecbootrSecbootLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SeccrBksel {
        #[doc = "Bank1 is selected for Bank erase / sector erase / interrupt enable"]
        B_0X0 = 0x0,
        #[doc = "Bank2 is selected for BER / SER"]
        B_0X1 = 0x01,
    }
    impl SeccrBksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SeccrBksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SeccrBksel {
        #[inline(always)]
        fn from(val: u8) -> SeccrBksel {
            SeccrBksel::from_bits(val)
        }
    }
    impl From<SeccrBksel> for u8 {
        #[inline(always)]
        fn from(val: SeccrBksel) -> u8 {
            SeccrBksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spriv {
        #[doc = "access to secure registers is always granted"]
        B_0X0 = 0x0,
        #[doc = "access to secure registers is denied in case of unprivileged access."]
        B_0X1 = 0x01,
    }
    impl Spriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spriv {
        #[inline(always)]
        fn from(val: u8) -> Spriv {
            Spriv::from_bits(val)
        }
    }
    impl From<Spriv> for u8 {
        #[inline(always)]
        fn from(val: Spriv) -> u8 {
            Spriv::to_bits(val)
        }
    }
}
