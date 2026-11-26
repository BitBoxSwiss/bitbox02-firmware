#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flash"]
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
    #[doc = "Access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Power down key register"]
    #[inline(always)]
    pub const fn pdkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Flash key register"]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Option byte key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Flash control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Flash ECC register"]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<regs::Eccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Flash option register"]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Flash Bank 1 PCROP Start address register"]
    #[inline(always)]
    pub const fn pcrop1sr(self) -> crate::common::Reg<regs::Pcrop1sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Flash Bank 1 PCROP End address register"]
    #[inline(always)]
    pub const fn pcrop1er(self) -> crate::common::Reg<regs::Pcrop1er, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Flash Bank 1 WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(self) -> crate::common::Reg<regs::Wrp1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Flash Bank 1 WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(self) -> crate::common::Reg<regs::Wrp1br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "securable area bank1 register"]
    #[inline(always)]
    pub const fn sec1r(self) -> crate::common::Reg<regs::Sec1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
}
pub mod regs {
    #[doc = "Access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Latency"]
        #[inline(always)]
        pub const fn latency(&self) -> super::vals::Latency {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Latency::from_bits(val as u8)
        }
        #[doc = "Latency"]
        #[inline(always)]
        pub fn set_latency(&mut self, val: super::vals::Latency) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Prefetch enable"]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable"]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Instruction cache enable"]
        #[inline(always)]
        pub const fn icen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction cache enable"]
        #[inline(always)]
        pub fn set_icen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data cache enable"]
        #[inline(always)]
        pub const fn dcen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data cache enable"]
        #[inline(always)]
        pub fn set_dcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Instruction cache reset"]
        #[inline(always)]
        pub const fn icrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction cache reset"]
        #[inline(always)]
        pub fn set_icrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data cache reset"]
        #[inline(always)]
        pub const fn dcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data cache reset"]
        #[inline(always)]
        pub fn set_dcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Flash Power-down mode during Low-power run mode"]
        #[inline(always)]
        pub const fn run_pd(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power-down mode during Low-power run mode"]
        #[inline(always)]
        pub fn set_run_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Flash Power-down mode during Low-power sleep mode"]
        #[inline(always)]
        pub const fn sleep_pd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power-down mode during Low-power sleep mode"]
        #[inline(always)]
        pub fn set_sleep_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Debug software enable"]
        #[inline(always)]
        pub const fn dbg_swen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Debug software enable"]
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
                .field("dcen", &self.dcen())
                .field("icrst", &self.icrst())
                .field("dcrst", &self.dcrst())
                .field("run_pd", &self.run_pd())
                .field("sleep_pd", &self.sleep_pd())
                .field("dbg_swen", &self.dbg_swen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Acr {{ latency: {:?}, prften: {=bool:?}, icen: {=bool:?}, dcen: {=bool:?}, icrst: {=bool:?}, dcrst: {=bool:?}, run_pd: {=bool:?}, sleep_pd: {=bool:?}, dbg_swen: {=bool:?} }}" , self . latency () , self . prften () , self . icen () , self . dcen () , self . icrst () , self . dcrst () , self . run_pd () , self . sleep_pd () , self . dbg_swen ())
        }
    }
    #[doc = "Flash control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bank 1 Mass erase"]
        #[inline(always)]
        pub const fn mer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 Mass erase"]
        #[inline(always)]
        pub fn set_mer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page number"]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0xff;
            val as u8
        }
        #[doc = "Page number"]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 3usize)) | (((val as u32) & 0xff) << 3usize);
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Options modification start"]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Options modification start"]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast programming"]
        #[inline(always)]
        pub const fn fstpg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming"]
        #[inline(always)]
        pub fn set_fstpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PCROP read error interrupt enable"]
        #[inline(always)]
        pub const fn rderrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP read error interrupt enable"]
        #[inline(always)]
        pub fn set_rderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Force the option byte loading"]
        #[inline(always)]
        pub const fn obl_launch(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Force the option byte loading"]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Securable memory area protection enable"]
        #[inline(always)]
        pub const fn sec_prot1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Securable memory area protection enable"]
        #[inline(always)]
        pub fn set_sec_prot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Options Lock"]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Options Lock"]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "FLASH_CR Lock"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_CR Lock"]
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
                .field("sec_prot1", &self.sec_prot1())
                .field("optlock", &self.optlock())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ pg: {=bool:?}, per: {=bool:?}, mer1: {=bool:?}, pnb: {=u8:?}, strt: {=bool:?}, optstrt: {=bool:?}, fstpg: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, rderrie: {=bool:?}, obl_launch: {=bool:?}, sec_prot1: {=bool:?}, optlock: {=bool:?}, lock: {=bool:?} }}" , self . pg () , self . per () , self . mer1 () , self . pnb () , self . strt () , self . optstrt () , self . fstpg () , self . eopie () , self . errie () , self . rderrie () , self . obl_launch () , self . sec_prot1 () , self . optlock () , self . lock ())
        }
    }
    #[doc = "Flash ECC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "ECC fail for Corrected ECC Error or Double ECC Error in info block"]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for Corrected ECC Error or Double ECC Error in info block"]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC fail for Corrected ECC Error or Double ECC Error in info block"]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for Corrected ECC Error or Double ECC Error in info block"]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC correction interrupt enable"]
        #[inline(always)]
        pub const fn eccie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction interrupt enable"]
        #[inline(always)]
        pub fn set_eccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC correction"]
        #[inline(always)]
        pub const fn eccc2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction"]
        #[inline(always)]
        pub fn set_eccc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ECC2 detection"]
        #[inline(always)]
        pub const fn eccd2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ECC2 detection"]
        #[inline(always)]
        pub fn set_eccd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ECC correction"]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction"]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ECC detection"]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection"]
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
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("eccie", &self.eccie())
                .field("eccc2", &self.eccc2())
                .field("eccd2", &self.eccd2())
                .field("eccc", &self.eccc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Eccr {{ addr_ecc: {=u32:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, eccie: {=bool:?}, eccc2: {=bool:?}, eccd2: {=bool:?}, eccc: {=bool:?}, eccd: {=bool:?} }}" , self . addr_ecc () , self . bk_ecc () , self . sysf_ecc () , self . eccie () , self . eccc2 () , self . eccd2 () , self . eccc () , self . eccd ())
        }
    }
    #[doc = "Flash option register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Read protection level"]
        #[inline(always)]
        pub const fn rdp(&self) -> super::vals::Rdp {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Rdp::from_bits(val as u8)
        }
        #[doc = "Read protection level"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: super::vals::Rdp) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "BOR reset Level"]
        #[inline(always)]
        pub const fn bor_lev(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "BOR reset Level"]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub const fn n_rst_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "nRST_SHDW"]
        #[inline(always)]
        pub const fn n_rst_shdw(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_SHDW"]
        #[inline(always)]
        pub fn set_n_rst_shdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub const fn idwg_sw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub fn set_idwg_sw(&mut self, val: bool) {
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
        #[doc = "PB4 pull-up enable"]
        #[inline(always)]
        pub const fn pb4_pupen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PB4 pull-up enable"]
        #[inline(always)]
        pub fn set_pb4_pupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Boot configuration"]
        #[inline(always)]
        pub const fn n_boot1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Boot configuration"]
        #[inline(always)]
        pub fn set_n_boot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SRAM2 parity check enable"]
        #[inline(always)]
        pub const fn sram2_pe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity check enable"]
        #[inline(always)]
        pub fn set_sram2_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 Erase when system reset"]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Erase when system reset"]
        #[inline(always)]
        pub fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "nSWBOOT0"]
        #[inline(always)]
        pub const fn n_swboot0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "nSWBOOT0"]
        #[inline(always)]
        pub fn set_n_swboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "nBOOT0 option bit"]
        #[inline(always)]
        pub const fn n_boot0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "nBOOT0 option bit"]
        #[inline(always)]
        pub fn set_n_boot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "NRST_MODE"]
        #[inline(always)]
        pub const fn nrst_mode(&self) -> super::vals::NrstMode {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::NrstMode::from_bits(val as u8)
        }
        #[doc = "NRST_MODE"]
        #[inline(always)]
        pub fn set_nrst_mode(&mut self, val: super::vals::NrstMode) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Internal reset holder enable bit"]
        #[inline(always)]
        pub const fn irhen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Internal reset holder enable bit"]
        #[inline(always)]
        pub fn set_irhen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("bor_lev", &self.bor_lev())
                .field("n_rst_stop", &self.n_rst_stop())
                .field("n_rst_stdby", &self.n_rst_stdby())
                .field("n_rst_shdw", &self.n_rst_shdw())
                .field("idwg_sw", &self.idwg_sw())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("pb4_pupen", &self.pb4_pupen())
                .field("n_boot1", &self.n_boot1())
                .field("sram2_pe", &self.sram2_pe())
                .field("sram2_rst", &self.sram2_rst())
                .field("n_swboot0", &self.n_swboot0())
                .field("n_boot0", &self.n_boot0())
                .field("nrst_mode", &self.nrst_mode())
                .field("irhen", &self.irhen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optr {{ rdp: {:?}, bor_lev: {=u8:?}, n_rst_stop: {=bool:?}, n_rst_stdby: {=bool:?}, n_rst_shdw: {=bool:?}, idwg_sw: {=bool:?}, iwdg_stop: {=bool:?}, iwdg_stdby: {=bool:?}, wwdg_sw: {=bool:?}, pb4_pupen: {=bool:?}, n_boot1: {=bool:?}, sram2_pe: {=bool:?}, sram2_rst: {=bool:?}, n_swboot0: {=bool:?}, n_boot0: {=bool:?}, nrst_mode: {:?}, irhen: {=bool:?} }}" , self . rdp () , self . bor_lev () , self . n_rst_stop () , self . n_rst_stdby () , self . n_rst_shdw () , self . idwg_sw () , self . iwdg_stop () , self . iwdg_stdby () , self . wwdg_sw () , self . pb4_pupen () , self . n_boot1 () , self . sram2_pe () , self . sram2_rst () , self . n_swboot0 () , self . n_boot0 () , self . nrst_mode () , self . irhen ())
        }
    }
    #[doc = "Flash Bank 1 PCROP End address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrop1er(pub u32);
    impl Pcrop1er {
        #[doc = "Bank 1 PCROP area end offset"]
        #[inline(always)]
        pub const fn pcrop1_end(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Bank 1 PCROP area end offset"]
        #[inline(always)]
        pub fn set_pcrop1_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "PCROP area preserved when RDP level decreased"]
        #[inline(always)]
        pub const fn pcrop_rdp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP area preserved when RDP level decreased"]
        #[inline(always)]
        pub fn set_pcrop_rdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pcrop1er {
        #[inline(always)]
        fn default() -> Pcrop1er {
            Pcrop1er(0)
        }
    }
    impl core::fmt::Debug for Pcrop1er {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrop1er")
                .field("pcrop1_end", &self.pcrop1_end())
                .field("pcrop_rdp", &self.pcrop_rdp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcrop1er {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcrop1er {{ pcrop1_end: {=u16:?}, pcrop_rdp: {=bool:?} }}",
                self.pcrop1_end(),
                self.pcrop_rdp()
            )
        }
    }
    #[doc = "Flash Bank 1 PCROP Start address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrop1sr(pub u32);
    impl Pcrop1sr {
        #[doc = "Bank 1 PCROP area start offset"]
        #[inline(always)]
        pub const fn pcrop1_strt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Bank 1 PCROP area start offset"]
        #[inline(always)]
        pub fn set_pcrop1_strt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Pcrop1sr {
        #[inline(always)]
        fn default() -> Pcrop1sr {
            Pcrop1sr(0)
        }
    }
    impl core::fmt::Debug for Pcrop1sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrop1sr")
                .field("pcrop1_strt", &self.pcrop1_strt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcrop1sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pcrop1sr {{ pcrop1_strt: {=u16:?} }}", self.pcrop1_strt())
        }
    }
    #[doc = "securable area bank1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1r(pub u32);
    impl Sec1r {
        #[doc = "SEC_SIZE1"]
        #[inline(always)]
        pub const fn sec_size1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SEC_SIZE1"]
        #[inline(always)]
        pub fn set_sec_size1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "used to force boot from user area"]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "used to force boot from user area"]
        #[inline(always)]
        pub fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Sec1r {
        #[inline(always)]
        fn default() -> Sec1r {
            Sec1r(0)
        }
    }
    impl core::fmt::Debug for Sec1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec1r")
                .field("sec_size1", &self.sec_size1())
                .field("boot_lock", &self.boot_lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sec1r {{ sec_size1: {=u8:?}, boot_lock: {=bool:?} }}",
                self.sec_size1(),
                self.boot_lock()
            )
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "End of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Write protected error"]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write protected error"]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Size error"]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Size error"]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Programming sequence error"]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error"]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fast programming data miss error"]
        #[inline(always)]
        pub const fn miserr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming data miss error"]
        #[inline(always)]
        pub fn set_miserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fast programming error"]
        #[inline(always)]
        pub const fn fasterr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming error"]
        #[inline(always)]
        pub fn set_fasterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PCROP read error"]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP read error"]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Option validity error"]
        #[inline(always)]
        pub const fn optverr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Option validity error"]
        #[inline(always)]
        pub fn set_optverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("miserr", &self.miserr())
                .field("fasterr", &self.fasterr())
                .field("rderr", &self.rderr())
                .field("optverr", &self.optverr())
                .field("bsy", &self.bsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ eop: {=bool:?}, operr: {=bool:?}, progerr: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, sizerr: {=bool:?}, pgserr: {=bool:?}, miserr: {=bool:?}, fasterr: {=bool:?}, rderr: {=bool:?}, optverr: {=bool:?}, bsy: {=bool:?} }}" , self . eop () , self . operr () , self . progerr () , self . wrperr () , self . pgaerr () , self . sizerr () , self . pgserr () , self . miserr () , self . fasterr () , self . rderr () , self . optverr () , self . bsy ())
        }
    }
    #[doc = "Flash Bank 1 WRP area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1ar(pub u32);
    impl Wrp1ar {
        #[doc = "Bank 1 WRP first area start offset"]
        #[inline(always)]
        pub const fn wrp1a_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP first area start offset"]
        #[inline(always)]
        pub fn set_wrp1a_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 1 WRP first area A end offset"]
        #[inline(always)]
        pub const fn wrp1a_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP first area A end offset"]
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
    #[doc = "Flash Bank 1 WRP area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1br(pub u32);
    impl Wrp1br {
        #[doc = "Bank 1 WRP second area B end offset"]
        #[inline(always)]
        pub const fn wrp1b_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B end offset"]
        #[inline(always)]
        pub fn set_wrp1b_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 1 WRP second area B start offset"]
        #[inline(always)]
        pub const fn wrp1b_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B start offset"]
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
    pub enum Latency {
        #[doc = "Zero wait states"]
        WS0 = 0x0,
        #[doc = "One wait state"]
        WS1 = 0x01,
        #[doc = "Two wait states"]
        WS2 = 0x02,
        #[doc = "Three wait states"]
        WS3 = 0x03,
        #[doc = "Four wait states"]
        WS4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Latency {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Latency {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Latency {
        #[inline(always)]
        fn from(val: u8) -> Latency {
            Latency::from_bits(val)
        }
    }
    impl From<Latency> for u8 {
        #[inline(always)]
        fn from(val: Latency) -> u8 {
            Latency::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum NrstMode {
        _RESERVED_0 = 0x0,
        #[doc = "Reset pin is in reset input mode only"]
        INPUT_ONLY = 0x01,
        #[doc = "Reset pin is in GPIO mode only"]
        GPIO = 0x02,
        #[doc = "Reset pin is in reset input and output mode"]
        INPUT_OUTPUT = 0x03,
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
        #[doc = "Read protection not active"]
        pub const LEVEL_0: Self = Self(0xaa);
        #[doc = "Memories read protection active"]
        pub const LEVEL_1: Self = Self(0xbb);
        #[doc = "Chip read protection active"]
        pub const LEVEL_2: Self = Self(0xcc);
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
                0xaa => f.write_str("LEVEL_0"),
                0xbb => f.write_str("LEVEL_1"),
                0xcc => f.write_str("LEVEL_2"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdp {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xaa => defmt::write!(f, "LEVEL_0"),
                0xbb => defmt::write!(f, "LEVEL_1"),
                0xcc => defmt::write!(f, "LEVEL_2"),
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
