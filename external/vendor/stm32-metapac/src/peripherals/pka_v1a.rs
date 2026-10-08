#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Public key accelerator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pka {
    ptr: *mut u8,
}
unsafe impl Send for Pka {}
unsafe impl Sync for Pka {}
impl Pka {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PKA control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PKA status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PKA clear flag register."]
    #[inline(always)]
    pub const fn clrfr(self) -> crate::common::Reg<regs::Clrfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PKA internal memeory."]
    #[inline(always)]
    pub const fn ram(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1334usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "PKA clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clrfr(pub u32);
    impl Clrfr {
        #[doc = "Clear PKA End of Operation flag."]
        #[inline(always)]
        pub const fn procendfc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PKA End of Operation flag."]
        #[inline(always)]
        pub fn set_procendfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Clear PKA RAM error flag."]
        #[inline(always)]
        pub const fn ramerrfc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PKA RAM error flag."]
        #[inline(always)]
        pub fn set_ramerrfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clear address error flag."]
        #[inline(always)]
        pub const fn addrerrfc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Clear address error flag."]
        #[inline(always)]
        pub fn set_addrerrfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clear operation error flag."]
        #[inline(always)]
        pub const fn operrfc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clear operation error flag."]
        #[inline(always)]
        pub fn set_operrfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Clrfr {
        #[inline(always)]
        fn default() -> Clrfr {
            Clrfr(0)
        }
    }
    impl core::fmt::Debug for Clrfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clrfr")
                .field("procendfc", &self.procendfc())
                .field("ramerrfc", &self.ramerrfc())
                .field("addrerrfc", &self.addrerrfc())
                .field("operrfc", &self.operrfc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clrfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clrfr {{ procendfc: {=bool:?}, ramerrfc: {=bool:?}, addrerrfc: {=bool:?}, operrfc: {=bool:?} }}",
                self.procendfc(),
                self.ramerrfc(),
                self.addrerrfc(),
                self.operrfc()
            )
        }
    }
    #[doc = "PKA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "End of operation interrupt enable."]
        #[inline(always)]
        pub const fn procendie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable."]
        #[inline(always)]
        pub fn set_procendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RAM error interrupt enable."]
        #[inline(always)]
        pub const fn ramerrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "RAM error interrupt enable."]
        #[inline(always)]
        pub fn set_ramerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Address error interrupt enable."]
        #[inline(always)]
        pub const fn addrerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Address error interrupt enable."]
        #[inline(always)]
        pub fn set_addrerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Operation error interrupt enable."]
        #[inline(always)]
        pub const fn operrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error interrupt enable."]
        #[inline(always)]
        pub fn set_operrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("start", &self.start())
                .field("mode", &self.mode())
                .field("procendie", &self.procendie())
                .field("ramerrie", &self.ramerrie())
                .field("addrerrie", &self.addrerrie())
                .field("operrie", &self.operrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, start: {=bool:?}, mode: {=u8:?}, procendie: {=bool:?}, ramerrie: {=bool:?}, addrerrie: {=bool:?}, operrie: {=bool:?} }}" , self . en () , self . start () , self . mode () , self . procendie () , self . ramerrie () , self . addrerrie () , self . operrie ())
        }
    }
    #[doc = "PKA status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "PKA initialization OK This bit is asserted when PKA initialization is complete. When RNG is not able to output proper random numbers INITOK stays at 0."]
        #[inline(always)]
        pub const fn initok(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PKA initialization OK This bit is asserted when PKA initialization is complete. When RNG is not able to output proper random numbers INITOK stays at 0."]
        #[inline(always)]
        pub fn set_initok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Limited mode flag This bit is updated when EN bit in PKA_CR is set."]
        #[inline(always)]
        pub const fn lmf(&self) -> super::vals::Lmf {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Lmf::from_bits(val as u8)
        }
        #[doc = "Limited mode flag This bit is updated when EN bit in PKA_CR is set."]
        #[inline(always)]
        pub fn set_lmf(&mut self, val: super::vals::Lmf) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "PKA operation is in progress This bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started. If PKA is started with a wrong opcode, it is busy for a couple of cycles, then it aborts automatically the operation and go back to ready (BUSY bit is set to 0)."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PKA operation is in progress This bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started. If PKA is started with a wrong opcode, it is busy for a couple of cycles, then it aborts automatically the operation and go back to ready (BUSY bit is set to 0)."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PKA End of Operation flag."]
        #[inline(always)]
        pub const fn procendf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PKA End of Operation flag."]
        #[inline(always)]
        pub fn set_procendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PKA RAM error flag This bit is cleared using RAMERRFC bit in PKA_CLRFR."]
        #[inline(always)]
        pub const fn ramerrf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA RAM error flag This bit is cleared using RAMERRFC bit in PKA_CLRFR."]
        #[inline(always)]
        pub fn set_ramerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Address error flag This bit is cleared using ADDRERRFC bit in PKA_CLRFR."]
        #[inline(always)]
        pub const fn addrerrf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Address error flag This bit is cleared using ADDRERRFC bit in PKA_CLRFR."]
        #[inline(always)]
        pub fn set_addrerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Operation error flag This bit is cleared using OPERRFC bit in PKA_CLRFR."]
        #[inline(always)]
        pub const fn operrf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error flag This bit is cleared using OPERRFC bit in PKA_CLRFR."]
        #[inline(always)]
        pub fn set_operrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("initok", &self.initok())
                .field("lmf", &self.lmf())
                .field("busy", &self.busy())
                .field("procendf", &self.procendf())
                .field("ramerrf", &self.ramerrf())
                .field("addrerrf", &self.addrerrf())
                .field("operrf", &self.operrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ initok: {=bool:?}, lmf: {:?}, busy: {=bool:?}, procendf: {=bool:?}, ramerrf: {=bool:?}, addrerrf: {=bool:?}, operrf: {=bool:?} }}" , self . initok () , self . lmf () , self . busy () , self . procendf () , self . ramerrf () , self . addrerrf () , self . operrf ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lmf {
        #[doc = "All values documented in MODE bitfield can be used."]
        ALL = 0x0,
        #[doc = "Only ECDSA verification (MODE = 0x26) is supported by the PKA."]
        LIMITED = 0x01,
    }
    impl Lmf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lmf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lmf {
        #[inline(always)]
        fn from(val: u8) -> Lmf {
            Lmf::from_bits(val)
        }
    }
    impl From<Lmf> for u8 {
        #[inline(always)]
        fn from(val: Lmf) -> u8 {
            Lmf::to_bits(val)
        }
    }
}
