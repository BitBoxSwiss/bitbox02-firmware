#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA channel configuration register (DMA_CCR)"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA channel 1 number of data register"]
    #[inline(always)]
    pub const fn ndtr(self) -> crate::common::Reg<regs::Ndtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DMA channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn par(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA channel 1 memory address register"]
    #[inline(always)]
    pub const fn mar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
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
    #[doc = "DMA interrupt status register (DMA_ISR)"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Isr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x08usize + n * 20usize) as _) }
    }
    #[doc = "channel selection register"]
    #[inline(always)]
    pub const fn cselr(self) -> crate::common::Reg<regs::Cselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
}
pub mod regs {
    #[doc = "DMA channel configuration register (DMA_CCR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Channel enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer complete interrupt enable"]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable"]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Half Transfer interrupt enable"]
        #[inline(always)]
        pub const fn htie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Half Transfer interrupt enable"]
        #[inline(always)]
        pub fn set_htie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transfer error interrupt enable"]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable"]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data transfer direction"]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Data transfer direction"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Circular mode enabled"]
        #[inline(always)]
        pub const fn circ(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Circular mode enabled"]
        #[inline(always)]
        pub fn set_circ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Peripheral increment mode enabled"]
        #[inline(always)]
        pub const fn pinc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral increment mode enabled"]
        #[inline(always)]
        pub fn set_pinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Memory increment mode enabled"]
        #[inline(always)]
        pub const fn minc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Memory increment mode enabled"]
        #[inline(always)]
        pub fn set_minc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Peripheral size"]
        #[inline(always)]
        pub const fn psize(&self) -> super::vals::Size {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Peripheral size"]
        #[inline(always)]
        pub fn set_psize(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Memory size"]
        #[inline(always)]
        pub const fn msize(&self) -> super::vals::Size {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Memory size"]
        #[inline(always)]
        pub fn set_msize(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Channel Priority level"]
        #[inline(always)]
        pub const fn pl(&self) -> super::vals::Pl {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Pl::from_bits(val as u8)
        }
        #[doc = "Channel Priority level"]
        #[inline(always)]
        pub fn set_pl(&mut self, val: super::vals::Pl) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Memory to memory mode enabled"]
        #[inline(always)]
        pub const fn mem2mem(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Memory to memory mode enabled"]
        #[inline(always)]
        pub fn set_mem2mem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("tcie", &self.tcie())
                .field("htie", &self.htie())
                .field("teie", &self.teie())
                .field("dir", &self.dir())
                .field("circ", &self.circ())
                .field("pinc", &self.pinc())
                .field("minc", &self.minc())
                .field("psize", &self.psize())
                .field("msize", &self.msize())
                .field("pl", &self.pl())
                .field("mem2mem", &self.mem2mem())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, tcie: {=bool:?}, htie: {=bool:?}, teie: {=bool:?}, dir: {:?}, circ: {=bool:?}, pinc: {=bool:?}, minc: {=bool:?}, psize: {:?}, msize: {:?}, pl: {:?}, mem2mem: {=bool:?} }}" , self . en () , self . tcie () , self . htie () , self . teie () , self . dir () , self . circ () , self . pinc () , self . minc () , self . psize () , self . msize () , self . pl () , self . mem2mem ())
        }
    }
    #[doc = "channel selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cselr(pub u32);
    impl Cselr {
        #[doc = "DMA channel selection"]
        #[inline(always)]
        pub const fn cs(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "DMA channel selection"]
        #[inline(always)]
        pub fn set_cs(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Cselr {
        #[inline(always)]
        fn default() -> Cselr {
            Cselr(0)
        }
    }
    impl core::fmt::Debug for Cselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cselr")
                .field("cs[0]", &self.cs(0usize))
                .field("cs[1]", &self.cs(1usize))
                .field("cs[2]", &self.cs(2usize))
                .field("cs[3]", &self.cs(3usize))
                .field("cs[4]", &self.cs(4usize))
                .field("cs[5]", &self.cs(5usize))
                .field("cs[6]", &self.cs(6usize))
                .field("cs[7]", &self.cs(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cselr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cselr {{ cs[0]: {=u8:?}, cs[1]: {=u8:?}, cs[2]: {=u8:?}, cs[3]: {=u8:?}, cs[4]: {=u8:?}, cs[5]: {=u8:?}, cs[6]: {=u8:?}, cs[7]: {=u8:?} }}" , self . cs (0usize) , self . cs (1usize) , self . cs (2usize) , self . cs (3usize) , self . cs (4usize) , self . cs (5usize) , self . cs (6usize) , self . cs (7usize))
        }
    }
    #[doc = "DMA interrupt status register (DMA_ISR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Channel 1 Global interrupt flag"]
        #[inline(always)]
        pub const fn gif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Global interrupt flag"]
        #[inline(always)]
        pub fn set_gif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel 1 Transfer Complete flag"]
        #[inline(always)]
        pub const fn tcif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Transfer Complete flag"]
        #[inline(always)]
        pub fn set_tcif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel 1 Half Transfer Complete flag"]
        #[inline(always)]
        pub const fn htif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Half Transfer Complete flag"]
        #[inline(always)]
        pub fn set_htif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel 1 Transfer Error flag"]
        #[inline(always)]
        pub const fn teif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Transfer Error flag"]
        #[inline(always)]
        pub fn set_teif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("gif[0]", &self.gif(0usize))
                .field("gif[1]", &self.gif(1usize))
                .field("gif[2]", &self.gif(2usize))
                .field("gif[3]", &self.gif(3usize))
                .field("gif[4]", &self.gif(4usize))
                .field("gif[5]", &self.gif(5usize))
                .field("gif[6]", &self.gif(6usize))
                .field("gif[7]", &self.gif(7usize))
                .field("tcif[0]", &self.tcif(0usize))
                .field("tcif[1]", &self.tcif(1usize))
                .field("tcif[2]", &self.tcif(2usize))
                .field("tcif[3]", &self.tcif(3usize))
                .field("tcif[4]", &self.tcif(4usize))
                .field("tcif[5]", &self.tcif(5usize))
                .field("tcif[6]", &self.tcif(6usize))
                .field("tcif[7]", &self.tcif(7usize))
                .field("htif[0]", &self.htif(0usize))
                .field("htif[1]", &self.htif(1usize))
                .field("htif[2]", &self.htif(2usize))
                .field("htif[3]", &self.htif(3usize))
                .field("htif[4]", &self.htif(4usize))
                .field("htif[5]", &self.htif(5usize))
                .field("htif[6]", &self.htif(6usize))
                .field("htif[7]", &self.htif(7usize))
                .field("teif[0]", &self.teif(0usize))
                .field("teif[1]", &self.teif(1usize))
                .field("teif[2]", &self.teif(2usize))
                .field("teif[3]", &self.teif(3usize))
                .field("teif[4]", &self.teif(4usize))
                .field("teif[5]", &self.teif(5usize))
                .field("teif[6]", &self.teif(6usize))
                .field("teif[7]", &self.teif(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ gif[0]: {=bool:?}, gif[1]: {=bool:?}, gif[2]: {=bool:?}, gif[3]: {=bool:?}, gif[4]: {=bool:?}, gif[5]: {=bool:?}, gif[6]: {=bool:?}, gif[7]: {=bool:?}, tcif[0]: {=bool:?}, tcif[1]: {=bool:?}, tcif[2]: {=bool:?}, tcif[3]: {=bool:?}, tcif[4]: {=bool:?}, tcif[5]: {=bool:?}, tcif[6]: {=bool:?}, tcif[7]: {=bool:?}, htif[0]: {=bool:?}, htif[1]: {=bool:?}, htif[2]: {=bool:?}, htif[3]: {=bool:?}, htif[4]: {=bool:?}, htif[5]: {=bool:?}, htif[6]: {=bool:?}, htif[7]: {=bool:?}, teif[0]: {=bool:?}, teif[1]: {=bool:?}, teif[2]: {=bool:?}, teif[3]: {=bool:?}, teif[4]: {=bool:?}, teif[5]: {=bool:?}, teif[6]: {=bool:?}, teif[7]: {=bool:?} }}" , self . gif (0usize) , self . gif (1usize) , self . gif (2usize) , self . gif (3usize) , self . gif (4usize) , self . gif (5usize) , self . gif (6usize) , self . gif (7usize) , self . tcif (0usize) , self . tcif (1usize) , self . tcif (2usize) , self . tcif (3usize) , self . tcif (4usize) , self . tcif (5usize) , self . tcif (6usize) , self . tcif (7usize) , self . htif (0usize) , self . htif (1usize) , self . htif (2usize) , self . htif (3usize) , self . htif (4usize) , self . htif (5usize) , self . htif (6usize) , self . htif (7usize) , self . teif (0usize) , self . teif (1usize) , self . teif (2usize) , self . teif (3usize) , self . teif (4usize) , self . teif (5usize) , self . teif (6usize) , self . teif (7usize))
        }
    }
    #[doc = "DMA channel 1 number of data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndtr(pub u32);
    impl Ndtr {
        #[doc = "Number of data to transfer"]
        #[inline(always)]
        pub const fn ndt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of data to transfer"]
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
    pub enum Dir {
        #[doc = "Read from peripheral"]
        FROM_PERIPHERAL = 0x0,
        #[doc = "Read from memory"]
        FROM_MEMORY = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
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
    pub enum Pl {
        #[doc = "Low priority"]
        LOW = 0x0,
        #[doc = "Medium priority"]
        MEDIUM = 0x01,
        #[doc = "High priority"]
        HIGH = 0x02,
        #[doc = "Very high priority"]
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
        #[doc = "8-bit size"]
        BITS8 = 0x0,
        #[doc = "16-bit size"]
        BITS16 = 0x01,
        #[doc = "32-bit size"]
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
