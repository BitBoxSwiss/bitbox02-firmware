#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DMA controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "low interrupt status register"]
    #[inline(always)]
    pub const fn isr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "low interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::W> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    #[inline(always)]
    pub const fn st(self, n: usize) -> St {
        assert!(n < 8usize);
        unsafe { St::from_ptr(self.ptr.add(0x10usize + n * 24usize) as _) }
    }
}
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct St {
    ptr: *mut u8,
}
unsafe impl Send for St {}
unsafe impl Sync for St {}
impl St {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "stream x configuration register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "stream x number of data register"]
    #[inline(always)]
    pub const fn ndtr(self) -> crate::common::Reg<regs::Ndtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "stream x peripheral address register"]
    #[inline(always)]
    pub const fn par(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0ar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1ar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "stream x FIFO control register"]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "stream x configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Stream enable / flag stream ready when read low"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Stream enable / flag stream ready when read low"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Direct mode error interrupt enable"]
        #[inline(always)]
        pub const fn dmeie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Direct mode error interrupt enable"]
        #[inline(always)]
        pub fn set_dmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer error interrupt enable"]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable"]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Half transfer interrupt enable"]
        #[inline(always)]
        pub const fn htie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Half transfer interrupt enable"]
        #[inline(always)]
        pub fn set_htie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transfer complete interrupt enable"]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable"]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Peripheral flow controller"]
        #[inline(always)]
        pub const fn pfctrl(&self) -> super::vals::Pfctrl {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Pfctrl::from_bits(val as u8)
        }
        #[doc = "Peripheral flow controller"]
        #[inline(always)]
        pub fn set_pfctrl(&mut self, val: super::vals::Pfctrl) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Data transfer direction"]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Data transfer direction"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Circular mode enabled"]
        #[inline(always)]
        pub const fn circ(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Circular mode enabled"]
        #[inline(always)]
        pub fn set_circ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Peripheral increment mode enabled"]
        #[inline(always)]
        pub const fn pinc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral increment mode enabled"]
        #[inline(always)]
        pub fn set_pinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Memory increment mode enabled"]
        #[inline(always)]
        pub const fn minc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Memory increment mode enabled"]
        #[inline(always)]
        pub fn set_minc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Peripheral data size"]
        #[inline(always)]
        pub const fn psize(&self) -> super::vals::Size {
            let val = (self.0 >> 11usize) & 0x03;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Peripheral data size"]
        #[inline(always)]
        pub fn set_psize(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
        }
        #[doc = "Memory data size"]
        #[inline(always)]
        pub const fn msize(&self) -> super::vals::Size {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Memory data size"]
        #[inline(always)]
        pub fn set_msize(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
        #[doc = "Peripheral increment offset size"]
        #[inline(always)]
        pub const fn pincos(&self) -> super::vals::Pincos {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Pincos::from_bits(val as u8)
        }
        #[doc = "Peripheral increment offset size"]
        #[inline(always)]
        pub fn set_pincos(&mut self, val: super::vals::Pincos) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Priority level"]
        #[inline(always)]
        pub const fn pl(&self) -> super::vals::Pl {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Pl::from_bits(val as u8)
        }
        #[doc = "Priority level"]
        #[inline(always)]
        pub fn set_pl(&mut self, val: super::vals::Pl) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Double buffer mode enabled"]
        #[inline(always)]
        pub const fn dbm(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode enabled"]
        #[inline(always)]
        pub fn set_dbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Current target (only in double buffer mode)"]
        #[inline(always)]
        pub const fn ct(&self) -> super::vals::Ct {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ct::from_bits(val as u8)
        }
        #[doc = "Current target (only in double buffer mode)"]
        #[inline(always)]
        pub fn set_ct(&mut self, val: super::vals::Ct) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Peripheral burst transfer configuration"]
        #[inline(always)]
        pub const fn pburst(&self) -> super::vals::Burst {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Burst::from_bits(val as u8)
        }
        #[doc = "Peripheral burst transfer configuration"]
        #[inline(always)]
        pub fn set_pburst(&mut self, val: super::vals::Burst) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "Memory burst transfer configuration"]
        #[inline(always)]
        pub const fn mburst(&self) -> super::vals::Burst {
            let val = (self.0 >> 23usize) & 0x03;
            super::vals::Burst::from_bits(val as u8)
        }
        #[doc = "Memory burst transfer configuration"]
        #[inline(always)]
        pub fn set_mburst(&mut self, val: super::vals::Burst) {
            self.0 = (self.0 & !(0x03 << 23usize)) | (((val.to_bits() as u32) & 0x03) << 23usize);
        }
        #[doc = "Channel selection"]
        #[inline(always)]
        pub const fn chsel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x0f;
            val as u8
        }
        #[doc = "Channel selection"]
        #[inline(always)]
        pub fn set_chsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
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
                .field("dmeie", &self.dmeie())
                .field("teie", &self.teie())
                .field("htie", &self.htie())
                .field("tcie", &self.tcie())
                .field("pfctrl", &self.pfctrl())
                .field("dir", &self.dir())
                .field("circ", &self.circ())
                .field("pinc", &self.pinc())
                .field("minc", &self.minc())
                .field("psize", &self.psize())
                .field("msize", &self.msize())
                .field("pincos", &self.pincos())
                .field("pl", &self.pl())
                .field("dbm", &self.dbm())
                .field("ct", &self.ct())
                .field("pburst", &self.pburst())
                .field("mburst", &self.mburst())
                .field("chsel", &self.chsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, dmeie: {=bool:?}, teie: {=bool:?}, htie: {=bool:?}, tcie: {=bool:?}, pfctrl: {:?}, dir: {:?}, circ: {=bool:?}, pinc: {=bool:?}, minc: {=bool:?}, psize: {:?}, msize: {:?}, pincos: {:?}, pl: {:?}, dbm: {=bool:?}, ct: {:?}, pburst: {:?}, mburst: {:?}, chsel: {=u8:?} }}" , self . en () , self . dmeie () , self . teie () , self . htie () , self . tcie () , self . pfctrl () , self . dir () , self . circ () , self . pinc () , self . minc () , self . psize () , self . msize () , self . pincos () , self . pl () , self . dbm () , self . ct () , self . pburst () , self . mburst () , self . chsel ())
        }
    }
    #[doc = "stream x FIFO control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "FIFO threshold selection"]
        #[inline(always)]
        pub const fn fth(&self) -> super::vals::Fth {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Fth::from_bits(val as u8)
        }
        #[doc = "FIFO threshold selection"]
        #[inline(always)]
        pub fn set_fth(&mut self, val: super::vals::Fth) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Direct mode disable"]
        #[inline(always)]
        pub const fn dmdis(&self) -> super::vals::Dmdis {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Dmdis::from_bits(val as u8)
        }
        #[doc = "Direct mode disable"]
        #[inline(always)]
        pub fn set_dmdis(&mut self, val: super::vals::Dmdis) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO status"]
        #[inline(always)]
        pub const fn fs(&self) -> super::vals::Fs {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Fs::from_bits(val as u8)
        }
        #[doc = "FIFO status"]
        #[inline(always)]
        pub fn set_fs(&mut self, val: super::vals::Fs) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "FIFO error interrupt enable"]
        #[inline(always)]
        pub const fn feie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO error interrupt enable"]
        #[inline(always)]
        pub fn set_feie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("fth", &self.fth())
                .field("dmdis", &self.dmdis())
                .field("fs", &self.fs())
                .field("feie", &self.feie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fcr {{ fth: {:?}, dmdis: {:?}, fs: {:?}, feie: {=bool:?} }}",
                self.fth(),
                self.dmdis(),
                self.fs(),
                self.feie()
            )
        }
    }
    #[doc = "interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ixr(pub u32);
    impl Ixr {
        #[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
        #[inline(always)]
        pub const fn feif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
        #[inline(always)]
        pub fn set_feif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
        #[inline(always)]
        pub const fn dmeif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
        #[inline(always)]
        pub fn set_dmeif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Stream x transfer error interrupt flag (x=3..0)"]
        #[inline(always)]
        pub const fn teif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Stream x transfer error interrupt flag (x=3..0)"]
        #[inline(always)]
        pub fn set_teif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Stream x half transfer interrupt flag (x=3..0)"]
        #[inline(always)]
        pub const fn htif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 4usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Stream x half transfer interrupt flag (x=3..0)"]
        #[inline(always)]
        pub fn set_htif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 4usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
        #[inline(always)]
        pub const fn tcif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 5usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
        #[inline(always)]
        pub fn set_tcif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 5usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ixr {
        #[inline(always)]
        fn default() -> Ixr {
            Ixr(0)
        }
    }
    impl core::fmt::Debug for Ixr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ixr")
                .field("feif[0]", &self.feif(0usize))
                .field("feif[1]", &self.feif(1usize))
                .field("feif[2]", &self.feif(2usize))
                .field("feif[3]", &self.feif(3usize))
                .field("dmeif[0]", &self.dmeif(0usize))
                .field("dmeif[1]", &self.dmeif(1usize))
                .field("dmeif[2]", &self.dmeif(2usize))
                .field("dmeif[3]", &self.dmeif(3usize))
                .field("teif[0]", &self.teif(0usize))
                .field("teif[1]", &self.teif(1usize))
                .field("teif[2]", &self.teif(2usize))
                .field("teif[3]", &self.teif(3usize))
                .field("htif[0]", &self.htif(0usize))
                .field("htif[1]", &self.htif(1usize))
                .field("htif[2]", &self.htif(2usize))
                .field("htif[3]", &self.htif(3usize))
                .field("tcif[0]", &self.tcif(0usize))
                .field("tcif[1]", &self.tcif(1usize))
                .field("tcif[2]", &self.tcif(2usize))
                .field("tcif[3]", &self.tcif(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ixr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ixr {{ feif[0]: {=bool:?}, feif[1]: {=bool:?}, feif[2]: {=bool:?}, feif[3]: {=bool:?}, dmeif[0]: {=bool:?}, dmeif[1]: {=bool:?}, dmeif[2]: {=bool:?}, dmeif[3]: {=bool:?}, teif[0]: {=bool:?}, teif[1]: {=bool:?}, teif[2]: {=bool:?}, teif[3]: {=bool:?}, htif[0]: {=bool:?}, htif[1]: {=bool:?}, htif[2]: {=bool:?}, htif[3]: {=bool:?}, tcif[0]: {=bool:?}, tcif[1]: {=bool:?}, tcif[2]: {=bool:?}, tcif[3]: {=bool:?} }}" , self . feif (0usize) , self . feif (1usize) , self . feif (2usize) , self . feif (3usize) , self . dmeif (0usize) , self . dmeif (1usize) , self . dmeif (2usize) , self . dmeif (3usize) , self . teif (0usize) , self . teif (1usize) , self . teif (2usize) , self . teif (3usize) , self . htif (0usize) , self . htif (1usize) , self . htif (2usize) , self . htif (3usize) , self . tcif (0usize) , self . tcif (1usize) , self . tcif (2usize) , self . tcif (3usize))
        }
    }
    #[doc = "stream x number of data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndtr(pub u32);
    impl Ndtr {
        #[doc = "Number of data items to transfer"]
        #[inline(always)]
        pub const fn ndt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of data items to transfer"]
        #[inline(always)]
        pub fn set_ndt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ndtr {
        #[inline(always)]
        fn default() -> Ndtr {
            Ndtr(0)
        }
    }
    impl core::fmt::Debug for Ndtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ndtr").field("ndt", &self.ndt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ndtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ndtr {{ ndt: {=u16:?} }}", self.ndt())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Burst {
        #[doc = "Single transfer"]
        SINGLE = 0x0,
        #[doc = "Incremental burst of 4 beats"]
        INCR4 = 0x01,
        #[doc = "Incremental burst of 8 beats"]
        INCR8 = 0x02,
        #[doc = "Incremental burst of 16 beats"]
        INCR16 = 0x03,
    }
    impl Burst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Burst {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Burst {
        #[inline(always)]
        fn from(val: u8) -> Burst {
            Burst::from_bits(val)
        }
    }
    impl From<Burst> for u8 {
        #[inline(always)]
        fn from(val: Burst) -> u8 {
            Burst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ct {
        #[doc = "The current target memory is Memory 0"]
        MEMORY0 = 0x0,
        #[doc = "The current target memory is Memory 1"]
        MEMORY1 = 0x01,
    }
    impl Ct {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ct {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ct {
        #[inline(always)]
        fn from(val: u8) -> Ct {
            Ct::from_bits(val)
        }
    }
    impl From<Ct> for u8 {
        #[inline(always)]
        fn from(val: Ct) -> u8 {
            Ct::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dir {
        #[doc = "Peripheral-to-memory"]
        PERIPHERAL_TO_MEMORY = 0x0,
        #[doc = "Memory-to-peripheral"]
        MEMORY_TO_PERIPHERAL = 0x01,
        #[doc = "Memory-to-memory"]
        MEMORY_TO_MEMORY = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dmdis {
        #[doc = "Direct mode is enabled"]
        ENABLED = 0x0,
        #[doc = "Direct mode is disabled"]
        DISABLED = 0x01,
    }
    impl Dmdis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dmdis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dmdis {
        #[inline(always)]
        fn from(val: u8) -> Dmdis {
            Dmdis::from_bits(val)
        }
    }
    impl From<Dmdis> for u8 {
        #[inline(always)]
        fn from(val: Dmdis) -> u8 {
            Dmdis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fs {
        #[doc = "0 < fifo_level < 1/4"]
        QUARTER1 = 0x0,
        #[doc = "1/4 <= fifo_level < 1/2"]
        QUARTER2 = 0x01,
        #[doc = "1/2 <= fifo_level < 3/4"]
        QUARTER3 = 0x02,
        #[doc = "3/4 <= fifo_level < full"]
        QUARTER4 = 0x03,
        #[doc = "FIFO is empty"]
        EMPTY = 0x04,
        #[doc = "FIFO is full"]
        FULL = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fs {
        #[inline(always)]
        fn from(val: u8) -> Fs {
            Fs::from_bits(val)
        }
    }
    impl From<Fs> for u8 {
        #[inline(always)]
        fn from(val: Fs) -> u8 {
            Fs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fth {
        #[doc = "1/4 full FIFO"]
        QUARTER = 0x0,
        #[doc = "1/2 full FIFO"]
        HALF = 0x01,
        #[doc = "3/4 full FIFO"]
        THREE_QUARTERS = 0x02,
        #[doc = "Full FIFO"]
        FULL = 0x03,
    }
    impl Fth {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fth {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fth {
        #[inline(always)]
        fn from(val: u8) -> Fth {
            Fth::from_bits(val)
        }
    }
    impl From<Fth> for u8 {
        #[inline(always)]
        fn from(val: Fth) -> u8 {
            Fth::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pfctrl {
        #[doc = "The DMA is the flow controller"]
        DMA = 0x0,
        #[doc = "The peripheral is the flow controller"]
        PERIPHERAL = 0x01,
    }
    impl Pfctrl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pfctrl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pfctrl {
        #[inline(always)]
        fn from(val: u8) -> Pfctrl {
            Pfctrl::from_bits(val)
        }
    }
    impl From<Pfctrl> for u8 {
        #[inline(always)]
        fn from(val: Pfctrl) -> u8 {
            Pfctrl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pincos {
        #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
        PSIZE = 0x0,
        #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
        FIXED4 = 0x01,
    }
    impl Pincos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pincos {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pincos {
        #[inline(always)]
        fn from(val: u8) -> Pincos {
            Pincos::from_bits(val)
        }
    }
    impl From<Pincos> for u8 {
        #[inline(always)]
        fn from(val: Pincos) -> u8 {
            Pincos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pl {
        #[doc = "Low"]
        LOW = 0x0,
        #[doc = "Medium"]
        MEDIUM = 0x01,
        #[doc = "High"]
        HIGH = 0x02,
        #[doc = "Very high"]
        VERY_HIGH = 0x03,
    }
    impl Pl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pl {
        #[inline(always)]
        fn from(val: u8) -> Pl {
            Pl::from_bits(val)
        }
    }
    impl From<Pl> for u8 {
        #[inline(always)]
        fn from(val: Pl) -> u8 {
            Pl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Size {
        #[doc = "Byte (8-bit)"]
        BITS8 = 0x0,
        #[doc = "Half-word (16-bit)"]
        BITS16 = 0x01,
        #[doc = "Word (32-bit)"]
        BITS32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Size {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Size {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Size {
        #[inline(always)]
        fn from(val: u8) -> Size {
            Size::from_bits(val)
        }
    }
    impl From<Size> for u8 {
        #[inline(always)]
        fn from(val: Size) -> u8 {
            Size::to_bits(val)
        }
    }
}
