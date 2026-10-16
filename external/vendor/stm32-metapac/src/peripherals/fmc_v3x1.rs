#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flexible memory controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmc {
    ptr: *mut u8,
}
unsafe impl Send for Fmc {}
unsafe impl Sync for Fmc {}
impl Fmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 1"]
    #[inline(always)]
    pub const fn bcr1(self) -> crate::common::Reg<regs::Bcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register 1-4"]
    #[inline(always)]
    pub const fn btr(self, n: usize) -> crate::common::Reg<regs::Btr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 8usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 2-4"]
    #[inline(always)]
    pub const fn bcr(self, n: usize) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 8usize) as _) }
    }
    #[doc = "PC Card/NAND Flash control register"]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FIFO status and interrupt register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Common memory space timing register"]
    #[inline(always)]
    pub const fn pmem(self) -> crate::common::Reg<regs::Pmem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Attribute memory space timing register"]
    #[inline(always)]
    pub const fn patt(self) -> crate::common::Reg<regs::Patt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "ECC result register"]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<regs::Eccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash write timing registers 1-4"]
    #[inline(always)]
    pub const fn bwtr(self, n: usize) -> crate::common::Reg<regs::Bwtr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 8usize) as _) }
    }
    #[doc = "SDRAM Control Register 1-2"]
    #[inline(always)]
    pub const fn sdcr(self, n: usize) -> crate::common::Reg<regs::Sdcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
    }
    #[doc = "SDRAM Timing register 1-2"]
    #[inline(always)]
    pub const fn sdtr(self, n: usize) -> crate::common::Reg<regs::Sdtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize + n * 4usize) as _) }
    }
    #[doc = "SDRAM Command Mode register"]
    #[inline(always)]
    pub const fn sdcmr(self) -> crate::common::Reg<regs::Sdcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "SDRAM Refresh Timer register"]
    #[inline(always)]
    pub const fn sdrtr(self) -> crate::common::Reg<regs::Sdrtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "SDRAM Status register"]
    #[inline(always)]
    pub const fn sdsr(self) -> crate::common::Reg<regs::Sdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
}
pub mod regs {
    #[doc = "SRAM/NOR-Flash chip-select control register 2-4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "Memory bank enable bit"]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit"]
        #[inline(always)]
        pub fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit"]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit"]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable"]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable"]
        #[inline(always)]
        pub fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit"]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit"]
        #[inline(always)]
        pub fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit"]
        #[inline(always)]
        pub const fn waitpol(&self) -> super::vals::Waitpol {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Waitpol::from_bits(val as u8)
        }
        #[doc = "Wait signal polarity bit"]
        #[inline(always)]
        pub fn set_waitpol(&mut self, val: super::vals::Waitpol) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wait timing configuration"]
        #[inline(always)]
        pub const fn waitcfg(&self) -> super::vals::Waitcfg {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Waitcfg::from_bits(val as u8)
        }
        #[doc = "Wait timing configuration"]
        #[inline(always)]
        pub fn set_waitcfg(&mut self, val: super::vals::Waitcfg) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit"]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit"]
        #[inline(always)]
        pub fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit"]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit"]
        #[inline(always)]
        pub fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable"]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable"]
        #[inline(always)]
        pub fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers"]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers"]
        #[inline(always)]
        pub fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRAM page size"]
        #[inline(always)]
        pub const fn cpsize(&self) -> super::vals::Cpsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cpsize::from_bits(val as u8)
        }
        #[doc = "CRAM page size"]
        #[inline(always)]
        pub fn set_cpsize(&mut self, val: super::vals::Cpsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Write burst enable"]
        #[inline(always)]
        pub const fn cburstrw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Write burst enable"]
        #[inline(always)]
        pub fn set_cburstrw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    impl core::fmt::Debug for Bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr")
                .field("mbken", &self.mbken())
                .field("muxen", &self.muxen())
                .field("mtyp", &self.mtyp())
                .field("mwid", &self.mwid())
                .field("faccen", &self.faccen())
                .field("bursten", &self.bursten())
                .field("waitpol", &self.waitpol())
                .field("waitcfg", &self.waitcfg())
                .field("wren", &self.wren())
                .field("waiten", &self.waiten())
                .field("extmod", &self.extmod())
                .field("asyncwait", &self.asyncwait())
                .field("cpsize", &self.cpsize())
                .field("cburstrw", &self.cburstrw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bcr {{ mbken: {=bool:?}, muxen: {=bool:?}, mtyp: {:?}, mwid: {:?}, faccen: {=bool:?}, bursten: {=bool:?}, waitpol: {:?}, waitcfg: {:?}, wren: {=bool:?}, waiten: {=bool:?}, extmod: {=bool:?}, asyncwait: {=bool:?}, cpsize: {:?}, cburstrw: {=bool:?} }}" , self . mbken () , self . muxen () , self . mtyp () , self . mwid () , self . faccen () , self . bursten () , self . waitpol () , self . waitcfg () , self . wren () , self . waiten () , self . extmod () , self . asyncwait () , self . cpsize () , self . cburstrw ())
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr1(pub u32);
    impl Bcr1 {
        #[doc = "Memory bank enable bit"]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit"]
        #[inline(always)]
        pub fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit"]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit"]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable"]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable"]
        #[inline(always)]
        pub fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit"]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit"]
        #[inline(always)]
        pub fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit"]
        #[inline(always)]
        pub const fn waitpol(&self) -> super::vals::Waitpol {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Waitpol::from_bits(val as u8)
        }
        #[doc = "Wait signal polarity bit"]
        #[inline(always)]
        pub fn set_waitpol(&mut self, val: super::vals::Waitpol) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wait timing configuration"]
        #[inline(always)]
        pub const fn waitcfg(&self) -> super::vals::Waitcfg {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Waitcfg::from_bits(val as u8)
        }
        #[doc = "Wait timing configuration"]
        #[inline(always)]
        pub fn set_waitcfg(&mut self, val: super::vals::Waitcfg) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit"]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit"]
        #[inline(always)]
        pub fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit"]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit"]
        #[inline(always)]
        pub fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable"]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable"]
        #[inline(always)]
        pub fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers"]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers"]
        #[inline(always)]
        pub fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRAM page size"]
        #[inline(always)]
        pub const fn cpsize(&self) -> super::vals::Cpsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cpsize::from_bits(val as u8)
        }
        #[doc = "CRAM page size"]
        #[inline(always)]
        pub fn set_cpsize(&mut self, val: super::vals::Cpsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Write burst enable"]
        #[inline(always)]
        pub const fn cburstrw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Write burst enable"]
        #[inline(always)]
        pub fn set_cburstrw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Continuous clock enable"]
        #[inline(always)]
        pub const fn cclken(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous clock enable"]
        #[inline(always)]
        pub fn set_cclken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Write FIFO disable"]
        #[inline(always)]
        pub const fn wfdis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Write FIFO disable"]
        #[inline(always)]
        pub fn set_wfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub const fn bmap(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub fn set_bmap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "FMC controller enable"]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FMC controller enable"]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bcr1 {
        #[inline(always)]
        fn default() -> Bcr1 {
            Bcr1(0)
        }
    }
    impl core::fmt::Debug for Bcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr1")
                .field("mbken", &self.mbken())
                .field("muxen", &self.muxen())
                .field("mtyp", &self.mtyp())
                .field("mwid", &self.mwid())
                .field("faccen", &self.faccen())
                .field("bursten", &self.bursten())
                .field("waitpol", &self.waitpol())
                .field("waitcfg", &self.waitcfg())
                .field("wren", &self.wren())
                .field("waiten", &self.waiten())
                .field("extmod", &self.extmod())
                .field("asyncwait", &self.asyncwait())
                .field("cpsize", &self.cpsize())
                .field("cburstrw", &self.cburstrw())
                .field("cclken", &self.cclken())
                .field("wfdis", &self.wfdis())
                .field("bmap", &self.bmap())
                .field("fmcen", &self.fmcen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bcr1 {{ mbken: {=bool:?}, muxen: {=bool:?}, mtyp: {:?}, mwid: {:?}, faccen: {=bool:?}, bursten: {=bool:?}, waitpol: {:?}, waitcfg: {:?}, wren: {=bool:?}, waiten: {=bool:?}, extmod: {=bool:?}, asyncwait: {=bool:?}, cpsize: {:?}, cburstrw: {=bool:?}, cclken: {=bool:?}, wfdis: {=bool:?}, bmap: {=u8:?}, fmcen: {=bool:?} }}" , self . mbken () , self . muxen () , self . mtyp () , self . mwid () , self . faccen () , self . bursten () , self . waitpol () , self . waitcfg () , self . wren () , self . waiten () , self . extmod () , self . asyncwait () , self . cpsize () , self . cburstrw () , self . cclken () , self . wfdis () , self . bmap () , self . fmcen ())
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btr(pub u32);
    impl Btr {
        #[doc = "Address setup phase duration"]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration"]
        #[inline(always)]
        pub fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration"]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration"]
        #[inline(always)]
        pub fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration"]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration"]
        #[inline(always)]
        pub fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration"]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration"]
        #[inline(always)]
        pub fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Clock divide ratio (for FMC_CLK signal)"]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Clock divide ratio (for FMC_CLK signal)"]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Data latency for synchronous memory"]
        #[inline(always)]
        pub const fn datlat(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Data latency for synchronous memory"]
        #[inline(always)]
        pub fn set_datlat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Access mode"]
        #[inline(always)]
        pub const fn accmod(&self) -> super::vals::Accmod {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Accmod::from_bits(val as u8)
        }
        #[doc = "Access mode"]
        #[inline(always)]
        pub fn set_accmod(&mut self, val: super::vals::Accmod) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Btr {
        #[inline(always)]
        fn default() -> Btr {
            Btr(0)
        }
    }
    impl core::fmt::Debug for Btr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Btr")
                .field("addset", &self.addset())
                .field("addhld", &self.addhld())
                .field("datast", &self.datast())
                .field("busturn", &self.busturn())
                .field("clkdiv", &self.clkdiv())
                .field("datlat", &self.datlat())
                .field("accmod", &self.accmod())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Btr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Btr {{ addset: {=u8:?}, addhld: {=u8:?}, datast: {=u8:?}, busturn: {=u8:?}, clkdiv: {=u8:?}, datlat: {=u8:?}, accmod: {:?} }}" , self . addset () , self . addhld () , self . datast () , self . busturn () , self . clkdiv () , self . datlat () , self . accmod ())
        }
    }
    #[doc = "SRAM/NOR-Flash write timing registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bwtr(pub u32);
    impl Bwtr {
        #[doc = "Address setup phase duration"]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration"]
        #[inline(always)]
        pub fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration"]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration"]
        #[inline(always)]
        pub fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration"]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration"]
        #[inline(always)]
        pub fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration"]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration"]
        #[inline(always)]
        pub fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Access mode"]
        #[inline(always)]
        pub const fn accmod(&self) -> super::vals::Accmod {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Accmod::from_bits(val as u8)
        }
        #[doc = "Access mode"]
        #[inline(always)]
        pub fn set_accmod(&mut self, val: super::vals::Accmod) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Bwtr {
        #[inline(always)]
        fn default() -> Bwtr {
            Bwtr(0)
        }
    }
    impl core::fmt::Debug for Bwtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bwtr")
                .field("addset", &self.addset())
                .field("addhld", &self.addhld())
                .field("datast", &self.datast())
                .field("busturn", &self.busturn())
                .field("accmod", &self.accmod())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bwtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bwtr {{ addset: {=u8:?}, addhld: {=u8:?}, datast: {=u8:?}, busturn: {=u8:?}, accmod: {:?} }}",
                self.addset(),
                self.addhld(),
                self.datast(),
                self.busturn(),
                self.accmod()
            )
        }
    }
    #[doc = "ECC result register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC computation result value"]
        #[inline(always)]
        pub const fn ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC computation result value"]
        #[inline(always)]
        pub fn set_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            f.debug_struct("Eccr").field("ecc", &self.ecc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccr {{ ecc: {=u32:?} }}", self.ecc())
        }
    }
    #[doc = "Attribute memory space timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Patt(pub u32);
    impl Patt {
        #[doc = "Attribute memory setup time"]
        #[inline(always)]
        pub const fn attset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory setup time"]
        #[inline(always)]
        pub fn set_attset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Attribute memory wait time"]
        #[inline(always)]
        pub const fn attwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory wait time"]
        #[inline(always)]
        pub fn set_attwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Attribute memory hold time"]
        #[inline(always)]
        pub const fn atthold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory hold time"]
        #[inline(always)]
        pub fn set_atthold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Attribute memory data bus Hi-Z time"]
        #[inline(always)]
        pub const fn atthiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory data bus Hi-Z time"]
        #[inline(always)]
        pub fn set_atthiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Patt {
        #[inline(always)]
        fn default() -> Patt {
            Patt(0)
        }
    }
    impl core::fmt::Debug for Patt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Patt")
                .field("attset", &self.attset())
                .field("attwait", &self.attwait())
                .field("atthold", &self.atthold())
                .field("atthiz", &self.atthiz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Patt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Patt {{ attset: {=u8:?}, attwait: {=u8:?}, atthold: {=u8:?}, atthiz: {=u8:?} }}",
                self.attset(),
                self.attwait(),
                self.atthold(),
                self.atthiz()
            )
        }
    }
    #[doc = "PC Card/NAND Flash control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Wait feature enable bit"]
        #[inline(always)]
        pub const fn pwaiten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wait feature enable bit"]
        #[inline(always)]
        pub fn set_pwaiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "NAND Flash memory bank enable bit"]
        #[inline(always)]
        pub const fn pbken(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "NAND Flash memory bank enable bit"]
        #[inline(always)]
        pub fn set_pbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data bus width"]
        #[inline(always)]
        pub const fn pwid(&self) -> super::vals::Pwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Pwid::from_bits(val as u8)
        }
        #[doc = "Data bus width"]
        #[inline(always)]
        pub fn set_pwid(&mut self, val: super::vals::Pwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "ECC computation logic enable bit"]
        #[inline(always)]
        pub const fn eccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ECC computation logic enable bit"]
        #[inline(always)]
        pub fn set_eccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CLE to RE delay"]
        #[inline(always)]
        pub const fn tclr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "CLE to RE delay"]
        #[inline(always)]
        pub fn set_tclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "ALE to RE delay"]
        #[inline(always)]
        pub const fn tar(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "ALE to RE delay"]
        #[inline(always)]
        pub fn set_tar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "ECC page size"]
        #[inline(always)]
        pub const fn eccps(&self) -> super::vals::Eccps {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::Eccps::from_bits(val as u8)
        }
        #[doc = "ECC page size"]
        #[inline(always)]
        pub fn set_eccps(&mut self, val: super::vals::Eccps) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    impl core::fmt::Debug for Pcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcr")
                .field("pwaiten", &self.pwaiten())
                .field("pbken", &self.pbken())
                .field("pwid", &self.pwid())
                .field("eccen", &self.eccen())
                .field("tclr", &self.tclr())
                .field("tar", &self.tar())
                .field("eccps", &self.eccps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pcr {{ pwaiten: {=bool:?}, pbken: {=bool:?}, pwid: {:?}, eccen: {=bool:?}, tclr: {=u8:?}, tar: {=u8:?}, eccps: {:?} }}" , self . pwaiten () , self . pbken () , self . pwid () , self . eccen () , self . tclr () , self . tar () , self . eccps ())
        }
    }
    #[doc = "Common memory space timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmem(pub u32);
    impl Pmem {
        #[doc = "Common memory x setup time"]
        #[inline(always)]
        pub const fn memset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x setup time"]
        #[inline(always)]
        pub fn set_memset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Common memory wait time"]
        #[inline(always)]
        pub const fn memwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory wait time"]
        #[inline(always)]
        pub fn set_memwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Common memory hold time"]
        #[inline(always)]
        pub const fn memhold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory hold time"]
        #[inline(always)]
        pub fn set_memhold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Common memory x data bus Hi-Z time"]
        #[inline(always)]
        pub const fn memhiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x data bus Hi-Z time"]
        #[inline(always)]
        pub fn set_memhiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pmem {
        #[inline(always)]
        fn default() -> Pmem {
            Pmem(0)
        }
    }
    impl core::fmt::Debug for Pmem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmem")
                .field("memset", &self.memset())
                .field("memwait", &self.memwait())
                .field("memhold", &self.memhold())
                .field("memhiz", &self.memhiz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmem {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmem {{ memset: {=u8:?}, memwait: {=u8:?}, memhold: {=u8:?}, memhiz: {=u8:?} }}",
                self.memset(),
                self.memwait(),
                self.memhold(),
                self.memhiz()
            )
        }
    }
    #[doc = "SDRAM Command Mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcmr(pub u32);
    impl Sdcmr {
        #[doc = "Command mode"]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Command mode"]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Command target bank 2"]
        #[inline(always)]
        pub const fn ctb2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Command target bank 2"]
        #[inline(always)]
        pub fn set_ctb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Command target bank 1"]
        #[inline(always)]
        pub const fn ctb1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Command target bank 1"]
        #[inline(always)]
        pub fn set_ctb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Number of Auto-refresh"]
        #[inline(always)]
        pub const fn nrfs(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Auto-refresh"]
        #[inline(always)]
        pub fn set_nrfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Mode Register definition"]
        #[inline(always)]
        pub const fn mrd(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x1fff;
            val as u16
        }
        #[doc = "Mode Register definition"]
        #[inline(always)]
        pub fn set_mrd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 9usize)) | (((val as u32) & 0x1fff) << 9usize);
        }
    }
    impl Default for Sdcmr {
        #[inline(always)]
        fn default() -> Sdcmr {
            Sdcmr(0)
        }
    }
    impl core::fmt::Debug for Sdcmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdcmr")
                .field("mode", &self.mode())
                .field("ctb2", &self.ctb2())
                .field("ctb1", &self.ctb1())
                .field("nrfs", &self.nrfs())
                .field("mrd", &self.mrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdcmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdcmr {{ mode: {:?}, ctb2: {=bool:?}, ctb1: {=bool:?}, nrfs: {=u8:?}, mrd: {=u16:?} }}",
                self.mode(),
                self.ctb2(),
                self.ctb1(),
                self.nrfs(),
                self.mrd()
            )
        }
    }
    #[doc = "SDRAM Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcr(pub u32);
    impl Sdcr {
        #[doc = "Number of column address bits"]
        #[inline(always)]
        pub const fn nc(&self) -> super::vals::Nc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Nc::from_bits(val as u8)
        }
        #[doc = "Number of column address bits"]
        #[inline(always)]
        pub fn set_nc(&mut self, val: super::vals::Nc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Number of row address bits"]
        #[inline(always)]
        pub const fn nr(&self) -> super::vals::Nr {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Nr::from_bits(val as u8)
        }
        #[doc = "Number of row address bits"]
        #[inline(always)]
        pub fn set_nr(&mut self, val: super::vals::Nr) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Number of internal banks"]
        #[inline(always)]
        pub const fn nb(&self) -> super::vals::Nb {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Nb::from_bits(val as u8)
        }
        #[doc = "Number of internal banks"]
        #[inline(always)]
        pub fn set_nb(&mut self, val: super::vals::Nb) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "CAS latency"]
        #[inline(always)]
        pub const fn cas(&self) -> super::vals::Cas {
            let val = (self.0 >> 7usize) & 0x03;
            super::vals::Cas::from_bits(val as u8)
        }
        #[doc = "CAS latency"]
        #[inline(always)]
        pub fn set_cas(&mut self, val: super::vals::Cas) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
        }
        #[doc = "Write protection"]
        #[inline(always)]
        pub const fn wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection"]
        #[inline(always)]
        pub fn set_wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SDRAM clock configuration"]
        #[inline(always)]
        pub const fn sdclk(&self) -> super::vals::Sdclk {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Sdclk::from_bits(val as u8)
        }
        #[doc = "SDRAM clock configuration"]
        #[inline(always)]
        pub fn set_sdclk(&mut self, val: super::vals::Sdclk) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Burst read"]
        #[inline(always)]
        pub const fn rburst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Burst read"]
        #[inline(always)]
        pub fn set_rburst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Read pipe"]
        #[inline(always)]
        pub const fn rpipe(&self) -> super::vals::Rpipe {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Rpipe::from_bits(val as u8)
        }
        #[doc = "Read pipe"]
        #[inline(always)]
        pub fn set_rpipe(&mut self, val: super::vals::Rpipe) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
    }
    impl Default for Sdcr {
        #[inline(always)]
        fn default() -> Sdcr {
            Sdcr(0)
        }
    }
    impl core::fmt::Debug for Sdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdcr")
                .field("nc", &self.nc())
                .field("nr", &self.nr())
                .field("mwid", &self.mwid())
                .field("nb", &self.nb())
                .field("cas", &self.cas())
                .field("wp", &self.wp())
                .field("sdclk", &self.sdclk())
                .field("rburst", &self.rburst())
                .field("rpipe", &self.rpipe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sdcr {{ nc: {:?}, nr: {:?}, mwid: {:?}, nb: {:?}, cas: {:?}, wp: {=bool:?}, sdclk: {:?}, rburst: {=bool:?}, rpipe: {:?} }}" , self . nc () , self . nr () , self . mwid () , self . nb () , self . cas () , self . wp () , self . sdclk () , self . rburst () , self . rpipe ())
        }
    }
    #[doc = "SDRAM Refresh Timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdrtr(pub u32);
    impl Sdrtr {
        #[doc = "Clear Refresh error flag"]
        #[inline(always)]
        pub const fn cre(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Refresh error flag"]
        #[inline(always)]
        pub fn set_cre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Refresh Timer Count"]
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x1fff;
            val as u16
        }
        #[doc = "Refresh Timer Count"]
        #[inline(always)]
        pub fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 1usize)) | (((val as u32) & 0x1fff) << 1usize);
        }
        #[doc = "RES Interrupt Enable"]
        #[inline(always)]
        pub const fn reie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RES Interrupt Enable"]
        #[inline(always)]
        pub fn set_reie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sdrtr {
        #[inline(always)]
        fn default() -> Sdrtr {
            Sdrtr(0)
        }
    }
    impl core::fmt::Debug for Sdrtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdrtr")
                .field("cre", &self.cre())
                .field("count", &self.count())
                .field("reie", &self.reie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdrtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdrtr {{ cre: {=bool:?}, count: {=u16:?}, reie: {=bool:?} }}",
                self.cre(),
                self.count(),
                self.reie()
            )
        }
    }
    #[doc = "SDRAM Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdsr(pub u32);
    impl Sdsr {
        #[doc = "Refresh error flag"]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh error flag"]
        #[inline(always)]
        pub fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Status Mode for Bank 1"]
        #[inline(always)]
        pub const fn modes1(&self) -> super::vals::Modes {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Modes::from_bits(val as u8)
        }
        #[doc = "Status Mode for Bank 1"]
        #[inline(always)]
        pub fn set_modes1(&mut self, val: super::vals::Modes) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Status Mode for Bank 2"]
        #[inline(always)]
        pub const fn modes2(&self) -> super::vals::Modes {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Modes::from_bits(val as u8)
        }
        #[doc = "Status Mode for Bank 2"]
        #[inline(always)]
        pub fn set_modes2(&mut self, val: super::vals::Modes) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
    }
    impl Default for Sdsr {
        #[inline(always)]
        fn default() -> Sdsr {
            Sdsr(0)
        }
    }
    impl core::fmt::Debug for Sdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdsr")
                .field("re", &self.re())
                .field("modes1", &self.modes1())
                .field("modes2", &self.modes2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdsr {{ re: {=bool:?}, modes1: {:?}, modes2: {:?} }}",
                self.re(),
                self.modes1(),
                self.modes2()
            )
        }
    }
    #[doc = "SDRAM Timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdtr(pub u32);
    impl Sdtr {
        #[doc = "Load Mode Register to Active"]
        #[inline(always)]
        pub const fn tmrd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Load Mode Register to Active"]
        #[inline(always)]
        pub fn set_tmrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Exit self-refresh delay"]
        #[inline(always)]
        pub const fn txsr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Exit self-refresh delay"]
        #[inline(always)]
        pub fn set_txsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Self refresh time"]
        #[inline(always)]
        pub const fn tras(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Self refresh time"]
        #[inline(always)]
        pub fn set_tras(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Row cycle delay"]
        #[inline(always)]
        pub const fn trc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Row cycle delay"]
        #[inline(always)]
        pub fn set_trc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Recovery delay"]
        #[inline(always)]
        pub const fn twr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Recovery delay"]
        #[inline(always)]
        pub fn set_twr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Row precharge delay"]
        #[inline(always)]
        pub const fn trp(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Row precharge delay"]
        #[inline(always)]
        pub fn set_trp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Row to column delay"]
        #[inline(always)]
        pub const fn trcd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Row to column delay"]
        #[inline(always)]
        pub fn set_trcd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Sdtr {
        #[inline(always)]
        fn default() -> Sdtr {
            Sdtr(0)
        }
    }
    impl core::fmt::Debug for Sdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdtr")
                .field("tmrd", &self.tmrd())
                .field("txsr", &self.txsr())
                .field("tras", &self.tras())
                .field("trc", &self.trc())
                .field("twr", &self.twr())
                .field("trp", &self.trp())
                .field("trcd", &self.trcd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdtr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sdtr {{ tmrd: {=u8:?}, txsr: {=u8:?}, tras: {=u8:?}, trc: {=u8:?}, twr: {=u8:?}, trp: {=u8:?}, trcd: {=u8:?} }}" , self . tmrd () , self . txsr () , self . tras () , self . trc () , self . twr () , self . trp () , self . trcd ())
        }
    }
    #[doc = "FIFO status and interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Interrupt rising edge status"]
        #[inline(always)]
        pub const fn irs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt rising edge status"]
        #[inline(always)]
        pub fn set_irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt high-level status"]
        #[inline(always)]
        pub const fn ils(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt high-level status"]
        #[inline(always)]
        pub fn set_ils(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt falling edge status"]
        #[inline(always)]
        pub const fn ifs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt falling edge status"]
        #[inline(always)]
        pub fn set_ifs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt rising edge detection enable bit"]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt rising edge detection enable bit"]
        #[inline(always)]
        pub fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt high-level detection enable bit"]
        #[inline(always)]
        pub const fn ilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt high-level detection enable bit"]
        #[inline(always)]
        pub fn set_ilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interrupt falling edge detection enable bit"]
        #[inline(always)]
        pub const fn ifen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt falling edge detection enable bit"]
        #[inline(always)]
        pub fn set_ifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO empty status"]
        #[inline(always)]
        pub const fn fempt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO empty status"]
        #[inline(always)]
        pub fn set_fempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("irs", &self.irs())
                .field("ils", &self.ils())
                .field("ifs", &self.ifs())
                .field("iren", &self.iren())
                .field("ilen", &self.ilen())
                .field("ifen", &self.ifen())
                .field("fempt", &self.fempt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ irs: {=bool:?}, ils: {=bool:?}, ifs: {=bool:?}, iren: {=bool:?}, ilen: {=bool:?}, ifen: {=bool:?}, fempt: {=bool:?} }}" , self . irs () , self . ils () , self . ifs () , self . iren () , self . ilen () , self . ifen () , self . fempt ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Accmod {
        #[doc = "Access mode A"]
        A = 0x0,
        #[doc = "Access mode B"]
        B = 0x01,
        #[doc = "Access mode C"]
        C = 0x02,
        #[doc = "Access mode D"]
        D = 0x03,
    }
    impl Accmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Accmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Accmod {
        #[inline(always)]
        fn from(val: u8) -> Accmod {
            Accmod::from_bits(val)
        }
    }
    impl From<Accmod> for u8 {
        #[inline(always)]
        fn from(val: Accmod) -> u8 {
            Accmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cas {
        _RESERVED_0 = 0x0,
        #[doc = "1 cycle"]
        CLOCKS1 = 0x01,
        #[doc = "2 cycles"]
        CLOCKS2 = 0x02,
        #[doc = "3 cycles"]
        CLOCKS3 = 0x03,
    }
    impl Cas {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cas {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cas {
        #[inline(always)]
        fn from(val: u8) -> Cas {
            Cas::from_bits(val)
        }
    }
    impl From<Cas> for u8 {
        #[inline(always)]
        fn from(val: Cas) -> u8 {
            Cas::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpsize {
        #[doc = "No burst split when crossing page boundary"]
        NO_BURST_SPLIT = 0x0,
        #[doc = "128 bytes CRAM page size"]
        BYTES128 = 0x01,
        #[doc = "256 bytes CRAM page size"]
        BYTES256 = 0x02,
        #[doc = "512 bytes CRAM page size"]
        BYTES512 = 0x03,
        #[doc = "1024 bytes CRAM page size"]
        BYTES1024 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cpsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpsize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpsize {
        #[inline(always)]
        fn from(val: u8) -> Cpsize {
            Cpsize::from_bits(val)
        }
    }
    impl From<Cpsize> for u8 {
        #[inline(always)]
        fn from(val: Cpsize) -> u8 {
            Cpsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eccps {
        #[doc = "ECC page size 256 bytes"]
        BYTES256 = 0x0,
        #[doc = "ECC page size 512 bytes"]
        BYTES512 = 0x01,
        #[doc = "ECC page size 1024 bytes"]
        BYTES1024 = 0x02,
        #[doc = "ECC page size 2048 bytes"]
        BYTES2048 = 0x03,
        #[doc = "ECC page size 4096 bytes"]
        BYTES4096 = 0x04,
        #[doc = "ECC page size 8192 bytes"]
        BYTES8192 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Eccps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eccps {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eccps {
        #[inline(always)]
        fn from(val: u8) -> Eccps {
            Eccps::from_bits(val)
        }
    }
    impl From<Eccps> for u8 {
        #[inline(always)]
        fn from(val: Eccps) -> u8 {
            Eccps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Normal Mode"]
        NORMAL = 0x0,
        #[doc = "Clock Configuration Enable"]
        CLOCK_CONFIGURATION_ENABLE = 0x01,
        #[doc = "PALL (All Bank Precharge) command"]
        PALL = 0x02,
        #[doc = "Auto-refresh command"]
        AUTO_REFRESH_COMMAND = 0x03,
        #[doc = "Load Mode Resgier"]
        LOAD_MODE_REGISTER = 0x04,
        #[doc = "Self-refresh command"]
        SELF_REFRESH_COMMAND = 0x05,
        #[doc = "Power-down command"]
        POWER_DOWN_COMMAND = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Modes {
        #[doc = "Normal Mode"]
        NORMAL = 0x0,
        #[doc = "Self-refresh mode"]
        SELF_REFRESH = 0x01,
        #[doc = "Power-down mode"]
        POWER_DOWN = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Modes {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Modes {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Modes {
        #[inline(always)]
        fn from(val: u8) -> Modes {
            Modes::from_bits(val)
        }
    }
    impl From<Modes> for u8 {
        #[inline(always)]
        fn from(val: Modes) -> u8 {
            Modes::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mtyp {
        #[doc = "SRAM memory type"]
        SRAM = 0x0,
        #[doc = "PSRAM (CRAM) memory type"]
        PSRAM = 0x01,
        #[doc = "NOR Flash/OneNAND Flash"]
        FLASH = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mtyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mtyp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mtyp {
        #[inline(always)]
        fn from(val: u8) -> Mtyp {
            Mtyp::from_bits(val)
        }
    }
    impl From<Mtyp> for u8 {
        #[inline(always)]
        fn from(val: Mtyp) -> u8 {
            Mtyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mwid {
        #[doc = "Memory data bus width 8 bits"]
        BITS8 = 0x0,
        #[doc = "Memory data bus width 16 bits"]
        BITS16 = 0x01,
        #[doc = "Memory data bus width 32 bits"]
        BITS32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mwid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mwid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mwid {
        #[inline(always)]
        fn from(val: u8) -> Mwid {
            Mwid::from_bits(val)
        }
    }
    impl From<Mwid> for u8 {
        #[inline(always)]
        fn from(val: Mwid) -> u8 {
            Mwid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nb {
        #[doc = "Two internal Banks"]
        NB2 = 0x0,
        #[doc = "Four internal Banks"]
        NB4 = 0x01,
    }
    impl Nb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nb {
        #[inline(always)]
        fn from(val: u8) -> Nb {
            Nb::from_bits(val)
        }
    }
    impl From<Nb> for u8 {
        #[inline(always)]
        fn from(val: Nb) -> u8 {
            Nb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nc {
        #[doc = "8 bits"]
        BITS8 = 0x0,
        #[doc = "9 bits"]
        BITS9 = 0x01,
        #[doc = "10 bits"]
        BITS10 = 0x02,
        #[doc = "11 bits"]
        BITS11 = 0x03,
    }
    impl Nc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nc {
        #[inline(always)]
        fn from(val: u8) -> Nc {
            Nc::from_bits(val)
        }
    }
    impl From<Nc> for u8 {
        #[inline(always)]
        fn from(val: Nc) -> u8 {
            Nc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nr {
        #[doc = "11 bits"]
        BITS11 = 0x0,
        #[doc = "12 bits"]
        BITS12 = 0x01,
        #[doc = "13 bits"]
        BITS13 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Nr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nr {
        #[inline(always)]
        fn from(val: u8) -> Nr {
            Nr::from_bits(val)
        }
    }
    impl From<Nr> for u8 {
        #[inline(always)]
        fn from(val: Nr) -> u8 {
            Nr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pwid {
        #[doc = "External memory device width 8 bits"]
        BITS8 = 0x0,
        #[doc = "External memory device width 16 bits"]
        BITS16 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pwid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pwid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pwid {
        #[inline(always)]
        fn from(val: u8) -> Pwid {
            Pwid::from_bits(val)
        }
    }
    impl From<Pwid> for u8 {
        #[inline(always)]
        fn from(val: Pwid) -> u8 {
            Pwid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rpipe {
        #[doc = "No clock cycle delay"]
        NO_DELAY = 0x0,
        #[doc = "One clock cycle delay"]
        CLOCKS1 = 0x01,
        #[doc = "Two clock cycles delay"]
        CLOCKS2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Rpipe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rpipe {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rpipe {
        #[inline(always)]
        fn from(val: u8) -> Rpipe {
            Rpipe::from_bits(val)
        }
    }
    impl From<Rpipe> for u8 {
        #[inline(always)]
        fn from(val: Rpipe) -> u8 {
            Rpipe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdclk {
        #[doc = "SDCLK clock disabled"]
        DISABLED = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "SDCLK period = 2 x HCLK period"]
        DIV2 = 0x02,
        #[doc = "SDCLK period = 3 x HCLK period"]
        DIV3 = 0x03,
    }
    impl Sdclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdclk {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdclk {
        #[inline(always)]
        fn from(val: u8) -> Sdclk {
            Sdclk::from_bits(val)
        }
    }
    impl From<Sdclk> for u8 {
        #[inline(always)]
        fn from(val: Sdclk) -> u8 {
            Sdclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Waitcfg {
        #[doc = "NWAIT signal is active one data cycle before wait state"]
        BEFORE_WAIT_STATE = 0x0,
        #[doc = "NWAIT signal is active during wait state"]
        DURING_WAIT_STATE = 0x01,
    }
    impl Waitcfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waitcfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waitcfg {
        #[inline(always)]
        fn from(val: u8) -> Waitcfg {
            Waitcfg::from_bits(val)
        }
    }
    impl From<Waitcfg> for u8 {
        #[inline(always)]
        fn from(val: Waitcfg) -> u8 {
            Waitcfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Waitpol {
        #[doc = "NWAIT active low"]
        ACTIVE_LOW = 0x0,
        #[doc = "NWAIT active high"]
        ACTIVE_HIGH = 0x01,
    }
    impl Waitpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waitpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waitpol {
        #[inline(always)]
        fn from(val: u8) -> Waitpol {
            Waitpol::from_bits(val)
        }
    }
    impl From<Waitpol> for u8 {
        #[inline(always)]
        fn from(val: Waitpol) -> u8 {
            Waitpol::to_bits(val)
        }
    }
}
