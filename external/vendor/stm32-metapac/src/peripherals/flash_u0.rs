#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Mamba FLASH register block"]
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
    #[doc = "FLASH key register"]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<regs::Optkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FLASH status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FLASH control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "FLASH ECC register"]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<regs::Eccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FLASH option register"]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FLASH WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(self) -> crate::common::Reg<regs::Wrp1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FLASH WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(self) -> crate::common::Reg<regs::Wrp1br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FLASH security register"]
    #[inline(always)]
    pub const fn secr(self) -> crate::common::Reg<regs::Secr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
}
pub mod regs {
    #[doc = "FLASH access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "CPU Prefetch enable"]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CPU Prefetch enable"]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CPU Instruction cache enable"]
        #[inline(always)]
        pub const fn icen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CPU Instruction cache enable"]
        #[inline(always)]
        pub fn set_icen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CPU Instruction cache reset This bit can be written only when the instruction cache is disabled."]
        #[inline(always)]
        pub const fn icrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CPU Instruction cache reset This bit can be written only when the instruction cache is disabled."]
        #[inline(always)]
        pub fn set_icrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Main flash memory area empty This bit indicates whether the first location of the main flash memory area is erased or has a programmed value. The bit can be set and reset by software."]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Main flash memory area empty This bit indicates whether the first location of the main flash memory area is erased or has a programmed value. The bit can be set and reset by software."]
        #[inline(always)]
        pub fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Debug access software enable Software may use this bit to enable/disable the debugger read access."]
        #[inline(always)]
        pub const fn dbg_swen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Debug access software enable Software may use this bit to enable/disable the debugger read access."]
        #[inline(always)]
        pub fn set_dbg_swen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("prften", &self.prften())
                .field("icen", &self.icen())
                .field("icrst", &self.icrst())
                .field("empty", &self.empty())
                .field("dbg_swen", &self.dbg_swen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Acr {{ latency: {=u8:?}, prften: {=bool:?}, icen: {=bool:?}, icrst: {=bool:?}, empty: {=bool:?}, dbg_swen: {=bool:?} }}" , self . latency () , self . prften () , self . icen () , self . icrst () , self . empty () , self . dbg_swen ())
        }
    }
    #[doc = "FLASH control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Flash memory programming enable"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory programming enable"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page erase enable"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page erase enable"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mass erase When set, this bit triggers the mass erase, that is, all user pages."]
        #[inline(always)]
        pub const fn mer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase When set, this bit triggers the mass erase, that is, all user pages."]
        #[inline(always)]
        pub fn set_mer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the main memory are not allowed."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the main memory are not allowed."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
        }
        #[doc = "Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR."]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR."]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast programming enable"]
        #[inline(always)]
        pub const fn fstpg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming enable"]
        #[inline(always)]
        pub fn set_fstpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register."]
        #[inline(always)]
        pub const fn rderrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register."]
        #[inline(always)]
        pub fn set_rderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set."]
        #[inline(always)]
        pub const fn obl_launch(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set."]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\\[4:0\\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset."]
        #[inline(always)]
        pub const fn sec_prot(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\\[4:0\\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset."]
        #[inline(always)]
        pub fn set_sec_prot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
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
                .field("pg", &self.pg())
                .field("per", &self.per())
                .field("mer1", &self.mer1())
                .field("pnb", &self.pnb())
                .field("strt", &self.strt())
                .field("optstrt", &self.optstrt())
                .field("fstpg", &self.fstpg())
                .field("eopie", &self.eopie())
                .field("errie", &self.errie())
                .field("rderrie", &self.rderrie())
                .field("obl_launch", &self.obl_launch())
                .field("sec_prot", &self.sec_prot())
                .field("optlock", &self.optlock())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ pg: {=bool:?}, per: {=bool:?}, mer1: {=bool:?}, pnb: {=u8:?}, strt: {=bool:?}, optstrt: {=bool:?}, fstpg: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, rderrie: {=bool:?}, obl_launch: {=bool:?}, sec_prot: {=bool:?}, optlock: {=bool:?}, lock: {=bool:?} }}" , self . pg () , self . per () , self . mer1 () , self . pnb () , self . strt () , self . optstrt () , self . fstpg () , self . eopie () , self . errie () , self . rderrie () , self . obl_launch () , self . sec_prot () , self . optlock () , self . lock ())
        }
    }
    #[doc = "FLASH ECC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC fail double-word address offset In case of ECC error or ECC correction detected, this bitfield contains double-word offset (multiple of 64 bits) to main Flash memory."]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "ECC fail double-word address offset In case of ECC error or ECC correction detected, this bitfield contains double-word offset (multiple of 64 bits) to main Flash memory."]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ECC correction interrupt enable"]
        #[inline(always)]
        pub const fn ecccie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction interrupt enable"]
        #[inline(always)]
        pub fn set_ecccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC correction Set by hardware when one ECC error has been detected and corrected. An interrupt is generated if ECCIE is set. Cleared by writing 1."]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction Set by hardware when one ECC error has been detected and corrected. An interrupt is generated if ECCIE is set. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ECC detection Set by hardware when two ECC errors have been detected. When this bit is set, a NMI is generated. Cleared by writing 1."]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection Set by hardware when two ECC errors have been detected. When this bit is set, a NMI is generated. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccr {
        #[inline(always)]
        fn default() -> Eccr {
            Eccr(0)
        }
    }
    impl core::fmt::Debug for Eccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr")
                .field("addr_ecc", &self.addr_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("ecccie", &self.ecccie())
                .field("eccc", &self.eccc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Eccr {{ addr_ecc: {=u16:?}, sysf_ecc: {=bool:?}, ecccie: {=bool:?}, eccc: {=bool:?}, eccd: {=bool:?} }}" , self . addr_ecc () , self . sysf_ecc () , self . ecccie () , self . eccc () , self . eccd ())
        }
    }
    #[doc = "FLASH key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "FLASH key The following values must be written consecutively to unlock the FLASH control register (FLASH_CR), thus enabling programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB"]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FLASH key The following values must be written consecutively to unlock the FLASH control register (FLASH_CR), thus enabling programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB"]
        #[inline(always)]
        pub fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    impl core::fmt::Debug for Keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Keyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Keyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Keyr {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "FLASH option key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optkeyr(pub u32);
    impl Optkeyr {
        #[doc = "Option byte key The following values must be written consecutively to unlock the flash memory option registers, enabling option byte programming/erasing operations: KEY1: 0x0819 2A3B KEY2: 0x4C5D 6E7F"]
        #[inline(always)]
        pub const fn optkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Option byte key The following values must be written consecutively to unlock the flash memory option registers, enabling option byte programming/erasing operations: KEY1: 0x0819 2A3B KEY2: 0x4C5D 6E7F"]
        #[inline(always)]
        pub fn set_optkey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Optkeyr {
        #[inline(always)]
        fn default() -> Optkeyr {
            Optkeyr(0)
        }
    }
    impl core::fmt::Debug for Optkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optkeyr").field("optkey", &self.optkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Optkeyr {{ optkey: {=u32:?} }}", self.optkey())
        }
    }
    #[doc = "FLASH option register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Read protection level Other: Level 1, memories read protection active"]
        #[inline(always)]
        pub const fn rdp(&self) -> super::vals::Rdp {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Rdp::from_bits(val as u8)
        }
        #[doc = "Read protection level Other: Level 1, memories read protection active"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: super::vals::Rdp) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "BOR reset level"]
        #[inline(always)]
        pub const fn borr_lev(&self) -> super::vals::BorrLev {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::BorrLev::from_bits(val as u8)
        }
        #[doc = "BOR reset level"]
        #[inline(always)]
        pub fn set_borr_lev(&mut self, val: super::vals::BorrLev) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Reset generated when entering Stop mode"]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generated when entering Stop mode"]
        #[inline(always)]
        pub fn set_nrst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset generated when entering Standby mode"]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generated when entering Standby mode"]
        #[inline(always)]
        pub fn set_nrst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset generated when entering Shutdown mode"]
        #[inline(always)]
        pub const fn nrst_shdw(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generated when entering Shutdown mode"]
        #[inline(always)]
        pub fn set_nrst_shdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Backup domain reset"]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain reset"]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SRAM parity check control enable/disable"]
        #[inline(always)]
        pub const fn ram_parity_check(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM parity check control enable/disable"]
        #[inline(always)]
        pub fn set_ram_parity_check(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Backup SRAM erase prevention"]
        #[inline(always)]
        pub const fn bkpsram_hw_erase_disable(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Backup SRAM erase prevention"]
        #[inline(always)]
        pub fn set_bkpsram_hw_erase_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "BOOT0 signal source selection This option bit defines the source of the BOOT0 signal."]
        #[inline(always)]
        pub const fn nboot_sel(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT0 signal source selection This option bit defines the source of the BOOT0 signal."]
        #[inline(always)]
        pub fn set_nboot_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration."]
        #[inline(always)]
        pub const fn nboot1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration."]
        #[inline(always)]
        pub fn set_nboot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "NBOOT0 option bit"]
        #[inline(always)]
        pub const fn nboot0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "NBOOT0 option bit"]
        #[inline(always)]
        pub fn set_nboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "NRST pin configuration"]
        #[inline(always)]
        pub const fn nrst_mode(&self) -> super::vals::NrstMode {
            let val = (self.0 >> 27usize) & 0x03;
            super::vals::NrstMode::from_bits(val as u8)
        }
        #[doc = "NRST pin configuration"]
        #[inline(always)]
        pub fn set_nrst_mode(&mut self, val: super::vals::NrstMode) {
            self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
        }
        #[doc = "Internal reset holder enable bit"]
        #[inline(always)]
        pub const fn irhen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Internal reset holder enable bit"]
        #[inline(always)]
        pub fn set_irhen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Optr {
        #[inline(always)]
        fn default() -> Optr {
            Optr(0)
        }
    }
    impl core::fmt::Debug for Optr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optr")
                .field("rdp", &self.rdp())
                .field("borr_lev", &self.borr_lev())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stdby", &self.nrst_stdby())
                .field("nrst_shdw", &self.nrst_shdw())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("bdrst", &self.bdrst())
                .field("ram_parity_check", &self.ram_parity_check())
                .field("bkpsram_hw_erase_disable", &self.bkpsram_hw_erase_disable())
                .field("nboot_sel", &self.nboot_sel())
                .field("nboot1", &self.nboot1())
                .field("nboot0", &self.nboot0())
                .field("nrst_mode", &self.nrst_mode())
                .field("irhen", &self.irhen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optr {{ rdp: {:?}, borr_lev: {:?}, nrst_stop: {=bool:?}, nrst_stdby: {=bool:?}, nrst_shdw: {=bool:?}, iwdg_sw: {=bool:?}, iwdg_stop: {=bool:?}, iwdg_stdby: {=bool:?}, wwdg_sw: {=bool:?}, bdrst: {=bool:?}, ram_parity_check: {=bool:?}, bkpsram_hw_erase_disable: {=bool:?}, nboot_sel: {=bool:?}, nboot1: {=bool:?}, nboot0: {=bool:?}, nrst_mode: {:?}, irhen: {=bool:?} }}" , self . rdp () , self . borr_lev () , self . nrst_stop () , self . nrst_stdby () , self . nrst_shdw () , self . iwdg_sw () , self . iwdg_stop () , self . iwdg_stdby () , self . wwdg_sw () , self . bdrst () , self . ram_parity_check () , self . bkpsram_hw_erase_disable () , self . nboot_sel () , self . nboot1 () , self . nboot0 () , self . nrst_mode () , self . irhen ())
        }
    }
    #[doc = "FLASH security register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secr(pub u32);
    impl Secr {
        #[doc = "Last page of the first hide protection area"]
        #[inline(always)]
        pub const fn hdp1_pend(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Last page of the first hide protection area"]
        #[inline(always)]
        pub fn set_hdp1_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch)."]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch)."]
        #[inline(always)]
        pub fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hide protection area enable"]
        #[inline(always)]
        pub const fn hdp1en(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Hide protection area enable"]
        #[inline(always)]
        pub fn set_hdp1en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Secr {
        #[inline(always)]
        fn default() -> Secr {
            Secr(0)
        }
    }
    impl core::fmt::Debug for Secr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secr")
                .field("hdp1_pend", &self.hdp1_pend())
                .field("boot_lock", &self.boot_lock())
                .field("hdp1en", &self.hdp1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secr {{ hdp1_pend: {=u8:?}, boot_lock: {=bool:?}, hdp1en: {=u8:?} }}",
                self.hdp1_pend(),
                self.boot_lock(),
                self.hdp1en()
            )
        }
    }
    #[doc = "FLASH status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "End of operation Set by hardware when one or more flash memory operation (programming / erase) has been completed successfully. This bit is set only if the end of operation interrupts are enabled (EOPIE=1). Cleared by writing 1."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation Set by hardware when one or more flash memory operation (programming / erase) has been completed successfully. This bit is set only if the end of operation interrupts are enabled (EOPIE=1). Cleared by writing 1."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Operation error Set by hardware when a flash memory operation (program / erase) completes unsuccessfully. This bit is set only if error interrupts are enabled (ERRIE=1). Cleared by writing 1."]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error Set by hardware when a flash memory operation (program / erase) completes unsuccessfully. This bit is set only if error interrupts are enabled (ERRIE=1). Cleared by writing 1."]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Programming error Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'. Cleared by writing 1."]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Programming error Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Write protection error Set by hardware when an address to be erased/programmed belongs to a write-protected part (by WRP, PCROP or RDP Level 1) of the flash memory. Cleared by writing 1."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error Set by hardware when an address to be erased/programmed belongs to a write-protected part (by WRP, PCROP or RDP Level 1) of the flash memory. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programming alignment error Set by hardware when the data to program cannot be contained in the same double word (64-bit) flash memory in case of standard programming, or if there is a change of page during fast programming. Cleared by writing 1."]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Programming alignment error Set by hardware when the data to program cannot be contained in the same double word (64-bit) flash memory in case of standard programming, or if there is a change of page during fast programming. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Size error Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access). Cleared by writing 1."]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Size error Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access). Cleared by writing 1."]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Programming sequence error Set by hardware when a write access to the flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Cleared by writing 1."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error Set by hardware when a write access to the flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fast programming data miss error In Fast programming mode, 16 double words (128 bytes) must be sent to flash memory successively, and the new data must be sent to the logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time. Cleared by writing 1."]
        #[inline(always)]
        pub const fn misserr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming data miss error In Fast programming mode, 16 double words (128 bytes) must be sent to flash memory successively, and the new data must be sent to the logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_misserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fast programming error Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time. Cleared by writing 1."]
        #[inline(always)]
        pub const fn fasterr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming error Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_fasterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PCROP read error Set by hardware when an address to be read belongs to a read protected area of the flash memory (PCROP protection). An interrupt is generated if RDERRIE is set in FLASH_CR. Cleared by writing 1."]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP read error Set by hardware when an address to be read belongs to a read protected area of the flash memory (PCROP protection). An interrupt is generated if RDERRIE is set in FLASH_CR. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Option and Engineering bits loading validity error"]
        #[inline(always)]
        pub const fn optverr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Option and Engineering bits loading validity error"]
        #[inline(always)]
        pub fn set_optverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Busy This flag indicates that a flash memory operation requested by FLASH control register (FLASH_CR) is in progress. This bit is set at the beginning of the flash memory operation, and cleared when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub const fn bsy1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy This flag indicates that a flash memory operation requested by FLASH control register (FLASH_CR) is in progress. This bit is set at the beginning of the flash memory operation, and cleared when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub fn set_bsy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Programming or erase configuration busy. This flag is set and cleared by hardware. It is set when the first word is sent for program or when setting the STRT bit of FLASH control register (FLASH_CR) for erase. It is cleared when the flash memory program or erase operation completes or ends with an error. When set, launching any other operation through the FLASH control register (FLASH_CR) is impossible, and must be postponed (a programming or erase operation is ongoing). When cleared, the program and erase settings in the FLASH control register (FLASH_CR) can be modified."]
        #[inline(always)]
        pub const fn cfgbsy(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming or erase configuration busy. This flag is set and cleared by hardware. It is set when the first word is sent for program or when setting the STRT bit of FLASH control register (FLASH_CR) for erase. It is cleared when the flash memory program or erase operation completes or ends with an error. When set, launching any other operation through the FLASH control register (FLASH_CR) is impossible, and must be postponed (a programming or erase operation is ongoing). When cleared, the program and erase settings in the FLASH control register (FLASH_CR) can be modified."]
        #[inline(always)]
        pub fn set_cfgbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("eop", &self.eop())
                .field("operr", &self.operr())
                .field("progerr", &self.progerr())
                .field("wrperr", &self.wrperr())
                .field("pgaerr", &self.pgaerr())
                .field("sizerr", &self.sizerr())
                .field("pgserr", &self.pgserr())
                .field("misserr", &self.misserr())
                .field("fasterr", &self.fasterr())
                .field("rderr", &self.rderr())
                .field("optverr", &self.optverr())
                .field("bsy1", &self.bsy1())
                .field("cfgbsy", &self.cfgbsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ eop: {=bool:?}, operr: {=bool:?}, progerr: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, sizerr: {=bool:?}, pgserr: {=bool:?}, misserr: {=bool:?}, fasterr: {=bool:?}, rderr: {=bool:?}, optverr: {=bool:?}, bsy1: {=bool:?}, cfgbsy: {=bool:?} }}" , self . eop () , self . operr () , self . progerr () , self . wrperr () , self . pgaerr () , self . sizerr () , self . pgserr () , self . misserr () , self . fasterr () , self . rderr () , self . optverr () , self . bsy1 () , self . cfgbsy ())
        }
    }
    #[doc = "FLASH WRP area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1ar(pub u32);
    impl Wrp1ar {
        #[doc = "WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
        #[inline(always)]
        pub const fn wrp1a_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
        #[inline(always)]
        pub fn set_wrp1a_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
        #[inline(always)]
        pub const fn wrp1a_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device."]
        #[inline(always)]
        pub fn set_wrp1a_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Wrp1ar {
        #[inline(always)]
        fn default() -> Wrp1ar {
            Wrp1ar(0)
        }
    }
    impl core::fmt::Debug for Wrp1ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1ar")
                .field("wrp1a_strt", &self.wrp1a_strt())
                .field("wrp1a_end", &self.wrp1a_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp1ar {{ wrp1a_strt: {=u8:?}, wrp1a_end: {=u8:?} }}",
                self.wrp1a_strt(),
                self.wrp1a_end()
            )
        }
    }
    #[doc = "FLASH WRP area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1br(pub u32);
    impl Wrp1br {
        #[doc = "WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
        #[inline(always)]
        pub const fn wrp1b_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
        #[inline(always)]
        pub fn set_wrp1b_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
        #[inline(always)]
        pub const fn wrp1b_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device."]
        #[inline(always)]
        pub fn set_wrp1b_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Wrp1br {
        #[inline(always)]
        fn default() -> Wrp1br {
            Wrp1br(0)
        }
    }
    impl core::fmt::Debug for Wrp1br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1br")
                .field("wrp1b_strt", &self.wrp1b_strt())
                .field("wrp1b_end", &self.wrp1b_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp1br {{ wrp1b_strt: {=u8:?}, wrp1b_end: {=u8:?} }}",
                self.wrp1b_strt(),
                self.wrp1b_end()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BorrLev {
        #[doc = "BOR rising level 1 with threshold around 2.1 V"]
        LEVEL1 = 0x0,
        #[doc = "BOR rising level 2 with threshold around 2.3 V"]
        LEVEL2 = 0x01,
        #[doc = "BOR rising level 3 with threshold around 2.6 V"]
        LEVEL3 = 0x02,
        #[doc = "BOR rising level 4 with threshold around 2.9 V"]
        LEVEL4 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl BorrLev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BorrLev {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BorrLev {
        #[inline(always)]
        fn from(val: u8) -> BorrLev {
            BorrLev::from_bits(val)
        }
    }
    impl From<BorrLev> for u8 {
        #[inline(always)]
        fn from(val: BorrLev) -> u8 {
            BorrLev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum NrstMode {
        _RESERVED_0 = 0x0,
        #[doc = "Reset input only: a low level on the NRST pin generates system reset; internal RESET is not propagated to the NRST pin."]
        ONLY_INPUT = 0x01,
        #[doc = "Standard GPIO: only internal RESET is possible"]
        ONLY_INTERNAL = 0x02,
        #[doc = "Bidirectional reset: the NRST pin is configured in reset input/output (legacy) mode"]
        BIDIRECTIONAL = 0x03,
    }
    impl NrstMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NrstMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NrstMode {
        #[inline(always)]
        fn from(val: u8) -> NrstMode {
            NrstMode::from_bits(val)
        }
    }
    impl From<NrstMode> for u8 {
        #[inline(always)]
        fn from(val: NrstMode) -> u8 {
            NrstMode::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdp(u8);
    impl Rdp {
        #[doc = "Level 0, read protection not active"]
        pub const LEVEL0: Self = Self(0xaa);
        #[doc = "Level 2, chip read protection active"]
        pub const LEVEL2: Self = Self(0xcc);
    }
    impl Rdp {
        pub const fn from_bits(val: u8) -> Rdp {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Rdp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xaa => f.write_str("LEVEL0"),
                0xcc => f.write_str("LEVEL2"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdp {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xaa => defmt::write!(f, "LEVEL0"),
                0xcc => defmt::write!(f, "LEVEL2"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Rdp {
        #[inline(always)]
        fn from(val: u8) -> Rdp {
            Rdp::from_bits(val)
        }
    }
    impl From<Rdp> for u8 {
        #[inline(always)]
        fn from(val: Rdp) -> u8 {
            Rdp::to_bits(val)
        }
    }
}
