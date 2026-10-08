#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flexible static memory controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsmc {
    ptr: *mut u8,
}
unsafe impl Send for Fsmc {}
unsafe impl Sync for Fsmc {}
impl Fsmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 1-4"]
    #[inline(always)]
    pub const fn bcr(self, n: usize) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register 1-4"]
    #[inline(always)]
    pub const fn btr(self, n: usize) -> crate::common::Reg<regs::Btr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 8usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash write timing registers 1-4"]
    #[inline(always)]
    pub const fn bwtr(self, n: usize) -> crate::common::Reg<regs::Bwtr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "SRAM/NOR-Flash chip-select control register"]
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
        #[doc = "WRAPMOD"]
        #[inline(always)]
        pub const fn wrapmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "WRAPMOD"]
        #[inline(always)]
        pub fn set_wrapmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("wrapmod", &self.wrapmod())
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
            defmt :: write ! (f , "Bcr {{ mbken: {=bool:?}, muxen: {=bool:?}, mtyp: {:?}, mwid: {:?}, faccen: {=bool:?}, bursten: {=bool:?}, waitpol: {:?}, wrapmod: {=bool:?}, waitcfg: {:?}, wren: {=bool:?}, waiten: {=bool:?}, extmod: {=bool:?}, asyncwait: {=bool:?}, cpsize: {:?}, cburstrw: {=bool:?} }}" , self . mbken () , self . muxen () , self . mtyp () , self . mwid () , self . faccen () , self . bursten () , self . waitpol () , self . wrapmod () , self . waitcfg () , self . wren () , self . waiten () , self . extmod () , self . asyncwait () , self . cpsize () , self . cburstrw ())
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
