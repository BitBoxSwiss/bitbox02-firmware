#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Secure advanced encryption standard hardware accelerator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saes {
    ptr: *mut u8,
}
unsafe impl Send for Saes {}
unsafe impl Sync for Saes {}
impl Saes {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SAES control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SAES status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SAES data input register."]
    #[inline(always)]
    pub const fn dinr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SAES data output register."]
    #[inline(always)]
    pub const fn doutr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SAES key register 0."]
    #[inline(always)]
    pub const fn keyr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(
                self.ptr.add(
                    0x10usize + ([0usize, 4usize, 8usize, 12usize, 32usize, 36usize, 40usize, 44usize][n] as usize),
                ) as _,
            )
        }
    }
    #[doc = "SAES initialization vector register 0."]
    #[inline(always)]
    pub const fn ivr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "SAES suspend registers."]
    #[inline(always)]
    pub const fn suspr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "SAES interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "SAES interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "SAES interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
}
pub mod regs {
    #[doc = "SAES control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\\[1:0\\]
other than 00, use the IPRST bit rather than the bit EN."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\\[1:0\\]
other than 00, use the IPRST bit rather than the bit EN."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub const fn datatype(&self) -> super::vals::Datatype {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Datatype::from_bits(val as u8)
        }
        #[doc = "Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub fn set_datatype(&mut self, val: super::vals::Datatype) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub const fn chmod(&self) -> super::vals::Chmod {
            let mut val = 0;
            val += (((self.0 >> 5usize) & 0x03) << 0usize);
            val += (((self.0 >> 16usize) & 0x01) << 2usize);
            super::vals::Chmod::from_bits(val as u8)
        }
        #[doc = "Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub fn set_chmod(&mut self, val: super::vals::Chmod) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32 >> 0usize) & 0x03) << 5usize);
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32 >> 2usize) & 0x01) << 16usize);
        }
        #[doc = "DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation)."]
        #[inline(always)]
        pub const fn dmainen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation)."]
        #[inline(always)]
        pub fn set_dmainen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation)."]
        #[inline(always)]
        pub const fn dmaouten(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation)."]
        #[inline(always)]
        pub fn set_dmaouten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
        #[inline(always)]
        pub const fn gcmph(&self) -> super::vals::Gcmph {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Gcmph::from_bits(val as u8)
        }
        #[doc = "GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
        #[inline(always)]
        pub fn set_gcmph(&mut self, val: super::vals::Gcmph) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
        #[doc = "Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\\[1:0\\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub const fn keysize(&self) -> super::vals::Keysize {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Keysize::from_bits(val as u8)
        }
        #[doc = "Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\\[1:0\\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub fn set_keysize(&mut self, val: super::vals::Keysize) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub const fn keyprot(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub fn set_keyprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
        #[inline(always)]
        pub const fn npblb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
        #[inline(always)]
        pub fn set_npblb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\\]
bitfield. This sharing is valid only while KMOD\\[1:0\\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\\[1:0\\]
other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub const fn kmod(&self) -> super::vals::Kmod {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Kmod::from_bits(val as u8)
        }
        #[doc = "Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\\]
bitfield. This sharing is valid only while KMOD\\[1:0\\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\\[1:0\\]
other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub fn set_kmod(&mut self, val: super::vals::Kmod) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Key share identification This bitfield defines, at the end of a decryption process with KMOD\\[1:0\\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub const fn kshareid(&self) -> super::vals::Kshareid {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Kshareid::from_bits(val as u8)
        }
        #[doc = "Key share identification This bitfield defines, at the end of a decryption process with KMOD\\[1:0\\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub fn set_kshareid(&mut self, val: super::vals::Kshareid) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\\[2:0\\]
with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\\[2:0\\]
bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\\[1:0\\]
is other than zero, KEYSEL\\[2:0\\]
is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub const fn keysel(&self) -> super::vals::Keysel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Keysel::from_bits(val as u8)
        }
        #[doc = "Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\\[2:0\\]
with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\\[2:0\\]
bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\\[1:0\\]
is other than zero, KEYSEL\\[2:0\\]
is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access."]
        #[inline(always)]
        pub fn set_keysel(&mut self, val: super::vals::Keysel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
        #[doc = "SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers."]
        #[inline(always)]
        pub const fn iprst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers."]
        #[inline(always)]
        pub fn set_iprst(&mut self, val: bool) {
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
                .field("datatype", &self.datatype())
                .field("mode", &self.mode())
                .field("chmod", &self.chmod())
                .field("dmainen", &self.dmainen())
                .field("dmaouten", &self.dmaouten())
                .field("gcmph", &self.gcmph())
                .field("keysize", &self.keysize())
                .field("keyprot", &self.keyprot())
                .field("npblb", &self.npblb())
                .field("kmod", &self.kmod())
                .field("kshareid", &self.kshareid())
                .field("keysel", &self.keysel())
                .field("iprst", &self.iprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, datatype: {:?}, mode: {:?}, chmod: {:?}, dmainen: {=bool:?}, dmaouten: {=bool:?}, gcmph: {:?}, keysize: {:?}, keyprot: {=bool:?}, npblb: {=u8:?}, kmod: {:?}, kshareid: {:?}, keysel: {:?}, iprst: {=bool:?} }}" , self . en () , self . datatype () , self . mode () , self . chmod () , self . dmainen () , self . dmaouten () , self . gcmph () , self . keysize () , self . keyprot () , self . npblb () , self . kmod () , self . kshareid () , self . keysel () , self . iprst ())
        }
    }
    #[doc = "SAES interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Computation complete flag clear Setting this bit clears the CCF status bit of the SAES_SR and SAES_ISR registers."]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag clear Setting this bit clears the CCF status bit of the SAES_SR and SAES_ISR registers."]
        #[inline(always)]
        pub fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the SAES_ISR register, and both RDERR and WRERR flags in the SAES_SR register."]
        #[inline(always)]
        pub const fn rweif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the SAES_ISR register, and both RDERR and WRERR flags in the SAES_SR register."]
        #[inline(always)]
        pub fn set_rweif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt flag clear Setting this bit clears the KEIF status bit of the SAES_ISR register."]
        #[inline(always)]
        pub const fn keif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt flag clear Setting this bit clears the KEIF status bit of the SAES_ISR register."]
        #[inline(always)]
        pub fn set_keif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RNG error interrupt flag clear Application must set this bit to clear the RNGEIF status bit in SAES_ISR register."]
        #[inline(always)]
        pub const fn rngeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RNG error interrupt flag clear Application must set this bit to clear the RNGEIF status bit in SAES_ISR register."]
        #[inline(always)]
        pub fn set_rngeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("ccf", &self.ccf())
                .field("rweif", &self.rweif())
                .field("keif", &self.keif())
                .field("rngeif", &self.rngeif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ ccf: {=bool:?}, rweif: {=bool:?}, keif: {=bool:?}, rngeif: {=bool:?} }}",
                self.ccf(),
                self.rweif(),
                self.keif(),
                self.rngeif()
            )
        }
    }
    #[doc = "SAES interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set."]
        #[inline(always)]
        pub const fn ccfie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set."]
        #[inline(always)]
        pub fn set_ccfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set."]
        #[inline(always)]
        pub const fn rweie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set."]
        #[inline(always)]
        pub fn set_rweie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set."]
        #[inline(always)]
        pub const fn keie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set."]
        #[inline(always)]
        pub fn set_keie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set."]
        #[inline(always)]
        pub const fn rngeie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set."]
        #[inline(always)]
        pub fn set_rngeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("ccfie", &self.ccfie())
                .field("rweie", &self.rweie())
                .field("keie", &self.keie())
                .field("rngeie", &self.rngeie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ ccfie: {=bool:?}, rweie: {=bool:?}, keie: {=bool:?}, rngeie: {=bool:?} }}",
                self.ccfie(),
                self.rweie(),
                self.keie(),
                self.rngeie()
            )
        }
    }
    #[doc = "SAES interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the SAES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1."]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the SAES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1."]
        #[inline(always)]
        pub fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the SAES_SR register. RWEIF bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the SAES_IER register. This flags has no meaning when key derivation mode is selected."]
        #[inline(always)]
        pub const fn rweif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the SAES_SR register. RWEIF bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the SAES_IER register. This flags has no meaning when key derivation mode is selected."]
        #[inline(always)]
        pub fn set_rweif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers or key register usage is forbidden. Setting the corresponding bit of the SAES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the SAES_IER register is set. KEIF is triggered upon any of the following errors: SAES fails to load the DHUK (KEYSEL = 001 or 100). SAES fails to load the BHK (KEYSEL = 010 or 100) respecting the correct order. AES fails to load the key shared by SAES peripheral (KMOD=10). When KEYVALID = 1 and (KEYPROT = 1 or KEYSEL is not 0x0), the security context of the application that loads the key (secure or non-secure) does not match the security attribute of the access to SAES_CR or SAES_DOUT. In this case, KEYVALID and EN bits are cleared. SAES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 register, or reverse. For KEYSIZE = 1, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 then SAES_KEYR4 then SAES_KEYR5 then SAES_KEYR6 then SAES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set."]
        #[inline(always)]
        pub const fn keif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers or key register usage is forbidden. Setting the corresponding bit of the SAES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the SAES_IER register is set. KEIF is triggered upon any of the following errors: SAES fails to load the DHUK (KEYSEL = 001 or 100). SAES fails to load the BHK (KEYSEL = 010 or 100) respecting the correct order. AES fails to load the key shared by SAES peripheral (KMOD=10). When KEYVALID = 1 and (KEYPROT = 1 or KEYSEL is not 0x0), the security context of the application that loads the key (secure or non-secure) does not match the security attribute of the access to SAES_CR or SAES_DOUT. In this case, KEYVALID and EN bits are cleared. SAES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 register, or reverse. For KEYSIZE = 1, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 then SAES_KEYR4 then SAES_KEYR5 then SAES_KEYR6 then SAES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set."]
        #[inline(always)]
        pub fn set_keif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RNG error interrupt flag This read-only bit is set by hardware when an error is detected on RNG bus interface (e.g. bad entropy). RNGEIE bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RNGEIE bit has been previously set in the SAES_IER register. Clearing this bit triggers the reload of a new random number from RNG peripheral."]
        #[inline(always)]
        pub const fn rngeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RNG error interrupt flag This read-only bit is set by hardware when an error is detected on RNG bus interface (e.g. bad entropy). RNGEIE bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RNGEIE bit has been previously set in the SAES_IER register. Clearing this bit triggers the reload of a new random number from RNG peripheral."]
        #[inline(always)]
        pub fn set_rngeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("ccf", &self.ccf())
                .field("rweif", &self.rweif())
                .field("keif", &self.keif())
                .field("rngeif", &self.rngeif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ ccf: {=bool:?}, rweif: {=bool:?}, keif: {=bool:?}, rngeif: {=bool:?} }}",
                self.ccf(),
                self.rweif(),
                self.keif(),
                self.rngeif()
            )
        }
    }
    #[doc = "SAES status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Read error flag This flag indicates the detection of an unexpected read operation from the SAES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected read returns zero."]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read error flag This flag indicates the detection of an unexpected read operation from the SAES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected read returns zero."]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write error This flag indicates the detection of an unexpected write operation to the SAES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected write is ignored."]
        #[inline(always)]
        pub const fn wrerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write error This flag indicates the detection of an unexpected write operation to the SAES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected write is ignored."]
        #[inline(always)]
        pub fn set_wrerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Busy This flag indicates whether SAES is idle or busy during GCM payload encryption phase: The flag is set upon SAES initialization, upon fetching random number from the RNG, or upon transferring a shared key to a target peripheral. When GCM encryption is selected, the flag must be at zero before selecting the GCM final phase."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Busy This flag indicates whether SAES is idle or busy during GCM payload encryption phase: The flag is set upon SAES initialization, upon fetching random number from the RNG, or upon transferring a shared key to a target peripheral. When GCM encryption is selected, the flag must be at zero before selecting the GCM final phase."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Key Valid flag This bit is set by hardware when the amount of key information defined by KEYSIZE in SAES_CR has been loaded in SAES_KEYx key registers. In normal mode when KEYSEL equals to zero, the application must write the key registers in the correct sequence, otherwise the KEIF flag of the SAES_ISR register is set and KEYVALID stays at zero. When KEYSEL is different from zero the BUSY flag is automatically set by SAES. When key is loaded successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the SAES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero. When the KEIF flag is set, the application must clear it through the SAES_ICR register, otherwise KEYVALID cannot be set. See the KEIF bit description for more details. For more information on key loading please refer to."]
        #[inline(always)]
        pub const fn keyvalid(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Key Valid flag This bit is set by hardware when the amount of key information defined by KEYSIZE in SAES_CR has been loaded in SAES_KEYx key registers. In normal mode when KEYSEL equals to zero, the application must write the key registers in the correct sequence, otherwise the KEIF flag of the SAES_ISR register is set and KEYVALID stays at zero. When KEYSEL is different from zero the BUSY flag is automatically set by SAES. When key is loaded successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the SAES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero. When the KEIF flag is set, the application must clear it through the SAES_ICR register, otherwise KEYVALID cannot be set. See the KEIF bit description for more details. For more information on key loading please refer to."]
        #[inline(always)]
        pub fn set_keyvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("rderr", &self.rderr())
                .field("wrerr", &self.wrerr())
                .field("busy", &self.busy())
                .field("keyvalid", &self.keyvalid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ rderr: {=bool:?}, wrerr: {=bool:?}, busy: {=bool:?}, keyvalid: {=bool:?} }}",
                self.rderr(),
                self.wrerr(),
                self.busy(),
                self.keyvalid()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chmod {
        #[doc = "Electronic codebook"]
        ECB = 0x0,
        #[doc = "Cipher-block chaining"]
        CBC = 0x01,
        #[doc = "Counter mode"]
        CTR = 0x02,
        #[doc = "Galois counter mode and Galois message authentication code"]
        GCM_GMAC = 0x03,
        #[doc = "Counter with CBC-MAC"]
        CCM = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Chmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chmod {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chmod {
        #[inline(always)]
        fn from(val: u8) -> Chmod {
            Chmod::from_bits(val)
        }
    }
    impl From<Chmod> for u8 {
        #[inline(always)]
        fn from(val: Chmod) -> u8 {
            Chmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Datatype {
        #[doc = "No swapping (32-bit data)."]
        NONE = 0x0,
        #[doc = "Half-word swapping (16-bit data)"]
        HALF_WORD = 0x01,
        #[doc = "Byte swapping (8-bit data)"]
        BYTE = 0x02,
        #[doc = "Bit-level swapping"]
        BIT = 0x03,
    }
    impl Datatype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Datatype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Datatype {
        #[inline(always)]
        fn from(val: u8) -> Datatype {
            Datatype::from_bits(val)
        }
    }
    impl From<Datatype> for u8 {
        #[inline(always)]
        fn from(val: Datatype) -> u8 {
            Datatype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Gcmph {
        #[doc = "Initialization phase"]
        INIT_PHASE = 0x0,
        #[doc = "Header phase"]
        HEADER_PHASE = 0x01,
        #[doc = "Payload phase"]
        PAYLOAD_PHASE = 0x02,
        #[doc = "Final phase"]
        FINAL_PHASE = 0x03,
    }
    impl Gcmph {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Gcmph {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Gcmph {
        #[inline(always)]
        fn from(val: u8) -> Gcmph {
            Gcmph::from_bits(val)
        }
    }
    impl From<Gcmph> for u8 {
        #[inline(always)]
        fn from(val: Gcmph) -> u8 {
            Gcmph::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Keysel {
        #[doc = "Software key, loaded in key registers SAES_KEYx"]
        SOFTWARE_KEY = 0x0,
        #[doc = "Derived hardware unique key"]
        DHUK = 0x01,
        #[doc = "Boot hardware key"]
        BHK = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "XOR of DHUK and BHK"]
        XOR_DHUK_BHK = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Keysel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Keysel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Keysel {
        #[inline(always)]
        fn from(val: u8) -> Keysel {
            Keysel::from_bits(val)
        }
    }
    impl From<Keysel> for u8 {
        #[inline(always)]
        fn from(val: Keysel) -> u8 {
            Keysel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Keysize {
        #[doc = "128-bit"]
        BITS128 = 0x0,
        #[doc = "256-bit"]
        BITS256 = 0x01,
    }
    impl Keysize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Keysize {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Keysize {
        #[inline(always)]
        fn from(val: u8) -> Keysize {
            Keysize::from_bits(val)
        }
    }
    impl From<Keysize> for u8 {
        #[inline(always)]
        fn from(val: Keysize) -> u8 {
            Keysize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Kmod {
        #[doc = "AES peripheral"]
        NORMAL = 0x0,
        #[doc = "Wrapped key for SAES mode. Key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected, read-as-zero SAES_DOUTR register is automatically loaded into SAES key registers after a successful decryption process."]
        WRAPPED_KEY = 0x01,
        #[doc = "Shared key mode. After a successful decryption process (unwrapping), SAES key registers are shared with the peripheral described in KSHAREID\\[1:0\\]
bitfield. This sharing is valid only while KMOD\\[1:0\\]
at 0x2 and KEYVALID=1. When a decryption is selected, read-as-zero SAES_DOUTR register is automatically loaded into SAES key registers after a successful decryption process."]
        SHARED_KEY = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Kmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Kmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Kmod {
        #[inline(always)]
        fn from(val: u8) -> Kmod {
            Kmod::from_bits(val)
        }
    }
    impl From<Kmod> for u8 {
        #[inline(always)]
        fn from(val: Kmod) -> u8 {
            Kmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Kshareid {
        #[doc = "AES peripheral"]
        AES = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Kshareid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Kshareid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Kshareid {
        #[inline(always)]
        fn from(val: u8) -> Kshareid {
            Kshareid::from_bits(val)
        }
    }
    impl From<Kshareid> for u8 {
        #[inline(always)]
        fn from(val: Kshareid) -> u8 {
            Kshareid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        ENCRYPTION = 0x0,
        #[doc = "Key derivation (or key preparation), for ECB/CBC decryption only"]
        KEY_DERIVATION = 0x01,
        DECRYPTION = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
}
