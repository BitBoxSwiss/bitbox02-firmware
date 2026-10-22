#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Filter math accelerator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmac {
    ptr: *mut u8,
}
unsafe impl Send for Fmac {}
unsafe impl Sync for Fmac {}
impl Fmac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "X1 buffer configuration register"]
    #[inline(always)]
    pub const fn x1bufcfg(self) -> crate::common::Reg<regs::X1bufcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "X2 buffer configuration register"]
    #[inline(always)]
    pub const fn x2bufcfg(self) -> crate::common::Reg<regs::X2bufcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Y buffer configuration register"]
    #[inline(always)]
    pub const fn ybufcfg(self) -> crate::common::Reg<regs::Ybufcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Parameter register"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Write data register"]
    #[inline(always)]
    pub const fn wdata(self) -> crate::common::Reg<regs::Wdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Read data register"]
    #[inline(always)]
    pub const fn rdata(self) -> crate::common::Reg<regs::Rdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Enable read interrupt"]
        #[inline(always)]
        pub const fn rien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable read interrupt"]
        #[inline(always)]
        pub fn set_rien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable write interrupt"]
        #[inline(always)]
        pub const fn wien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable write interrupt"]
        #[inline(always)]
        pub fn set_wien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable overflow error interrupts"]
        #[inline(always)]
        pub const fn ovflien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable overflow error interrupts"]
        #[inline(always)]
        pub fn set_ovflien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable underflow error interrupts"]
        #[inline(always)]
        pub const fn unflien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable underflow error interrupts"]
        #[inline(always)]
        pub fn set_unflien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable saturation error interrupts"]
        #[inline(always)]
        pub const fn satien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable saturation error interrupts"]
        #[inline(always)]
        pub fn set_satien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable DMA read channel requests"]
        #[inline(always)]
        pub const fn dmaren(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA read channel requests"]
        #[inline(always)]
        pub fn set_dmaren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable DMA write channel requests"]
        #[inline(always)]
        pub const fn dmawen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA write channel requests"]
        #[inline(always)]
        pub fn set_dmawen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable clipping"]
        #[inline(always)]
        pub const fn clipen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable clipping"]
        #[inline(always)]
        pub fn set_clipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Reset FMAC unit"]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset FMAC unit"]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("rien", &self.rien())
                .field("wien", &self.wien())
                .field("ovflien", &self.ovflien())
                .field("unflien", &self.unflien())
                .field("satien", &self.satien())
                .field("dmaren", &self.dmaren())
                .field("dmawen", &self.dmawen())
                .field("clipen", &self.clipen())
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ rien: {=bool:?}, wien: {=bool:?}, ovflien: {=bool:?}, unflien: {=bool:?}, satien: {=bool:?}, dmaren: {=bool:?}, dmawen: {=bool:?}, clipen: {=bool:?}, reset: {=bool:?} }}" , self . rien () , self . wien () , self . ovflien () , self . unflien () , self . satien () , self . dmaren () , self . dmawen () , self . clipen () , self . reset ())
        }
    }
    #[doc = "Parameter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Param(pub u32);
    impl Param {
        #[doc = "Input parameter P"]
        #[inline(always)]
        pub const fn p(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Input parameter P"]
        #[inline(always)]
        pub fn set_p(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Input parameter Q"]
        #[inline(always)]
        pub const fn q(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Input parameter Q"]
        #[inline(always)]
        pub fn set_q(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Input parameter R"]
        #[inline(always)]
        pub const fn r(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Input parameter R"]
        #[inline(always)]
        pub fn set_r(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Function"]
        #[inline(always)]
        pub const fn func(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Function"]
        #[inline(always)]
        pub fn set_func(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[doc = "Enable execution"]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable execution"]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Param {
        #[inline(always)]
        fn default() -> Param {
            Param(0)
        }
    }
    impl core::fmt::Debug for Param {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Param")
                .field("p", &self.p())
                .field("q", &self.q())
                .field("r", &self.r())
                .field("func", &self.func())
                .field("start", &self.start())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Param {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Param {{ p: {=u8:?}, q: {=u8:?}, r: {=u8:?}, func: {=u8:?}, start: {=bool:?} }}",
                self.p(),
                self.q(),
                self.r(),
                self.func(),
                self.start()
            )
        }
    }
    #[doc = "Read data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdata(pub u32);
    impl Rdata {
        #[doc = "Read data (contents of the Y output buffer at the address indicated by the READ pointer)"]
        #[inline(always)]
        pub const fn res(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Read data (contents of the Y output buffer at the address indicated by the READ pointer)"]
        #[inline(always)]
        pub fn set_res(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rdata {
        #[inline(always)]
        fn default() -> Rdata {
            Rdata(0)
        }
    }
    impl core::fmt::Debug for Rdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdata").field("res", &self.res()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rdata {{ res: {=u16:?} }}", self.res())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Y buffer empty flag"]
        #[inline(always)]
        pub const fn yempty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Y buffer empty flag"]
        #[inline(always)]
        pub fn set_yempty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "X1 buffer full flag"]
        #[inline(always)]
        pub const fn x1full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "X1 buffer full flag"]
        #[inline(always)]
        pub fn set_x1full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Overflow error flag"]
        #[inline(always)]
        pub const fn ovfl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow error flag"]
        #[inline(always)]
        pub fn set_ovfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Underflow error flag"]
        #[inline(always)]
        pub const fn unfl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Underflow error flag"]
        #[inline(always)]
        pub fn set_unfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Saturation error flag"]
        #[inline(always)]
        pub const fn sat(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Saturation error flag"]
        #[inline(always)]
        pub fn set_sat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("yempty", &self.yempty())
                .field("x1full", &self.x1full())
                .field("ovfl", &self.ovfl())
                .field("unfl", &self.unfl())
                .field("sat", &self.sat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ yempty: {=bool:?}, x1full: {=bool:?}, ovfl: {=bool:?}, unfl: {=bool:?}, sat: {=bool:?} }}",
                self.yempty(),
                self.x1full(),
                self.ovfl(),
                self.unfl(),
                self.sat()
            )
        }
    }
    #[doc = "Write data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdata(pub u32);
    impl Wdata {
        #[doc = "Write data (write data are transferred to the address indicated by the write pointer)"]
        #[inline(always)]
        pub const fn wdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Write data (write data are transferred to the address indicated by the write pointer)"]
        #[inline(always)]
        pub fn set_wdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wdata {
        #[inline(always)]
        fn default() -> Wdata {
            Wdata(0)
        }
    }
    impl core::fmt::Debug for Wdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdata").field("wdata", &self.wdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdata {{ wdata: {=u16:?} }}", self.wdata())
        }
    }
    #[doc = "X1 buffer configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct X1bufcfg(pub u32);
    impl X1bufcfg {
        #[doc = "Base address of X1 buffer"]
        #[inline(always)]
        pub const fn x1_base(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Base address of X1 buffer"]
        #[inline(always)]
        pub fn set_x1_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Allocated size of X1 buffer in 16-bit words"]
        #[inline(always)]
        pub const fn x1_buf_size(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Allocated size of X1 buffer in 16-bit words"]
        #[inline(always)]
        pub fn set_x1_buf_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Watermark for buffer full flag"]
        #[inline(always)]
        pub const fn full_wm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Watermark for buffer full flag"]
        #[inline(always)]
        pub fn set_full_wm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for X1bufcfg {
        #[inline(always)]
        fn default() -> X1bufcfg {
            X1bufcfg(0)
        }
    }
    impl core::fmt::Debug for X1bufcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("X1bufcfg")
                .field("x1_base", &self.x1_base())
                .field("x1_buf_size", &self.x1_buf_size())
                .field("full_wm", &self.full_wm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for X1bufcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "X1bufcfg {{ x1_base: {=u8:?}, x1_buf_size: {=u8:?}, full_wm: {=u8:?} }}",
                self.x1_base(),
                self.x1_buf_size(),
                self.full_wm()
            )
        }
    }
    #[doc = "X2 buffer configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct X2bufcfg(pub u32);
    impl X2bufcfg {
        #[doc = "Base address of X2 buffer"]
        #[inline(always)]
        pub const fn x2_base(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Base address of X2 buffer"]
        #[inline(always)]
        pub fn set_x2_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Size of X2 buffer in 16-bit words"]
        #[inline(always)]
        pub const fn x2_buf_size(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Size of X2 buffer in 16-bit words"]
        #[inline(always)]
        pub fn set_x2_buf_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for X2bufcfg {
        #[inline(always)]
        fn default() -> X2bufcfg {
            X2bufcfg(0)
        }
    }
    impl core::fmt::Debug for X2bufcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("X2bufcfg")
                .field("x2_base", &self.x2_base())
                .field("x2_buf_size", &self.x2_buf_size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for X2bufcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "X2bufcfg {{ x2_base: {=u8:?}, x2_buf_size: {=u8:?} }}",
                self.x2_base(),
                self.x2_buf_size()
            )
        }
    }
    #[doc = "Y buffer configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ybufcfg(pub u32);
    impl Ybufcfg {
        #[doc = "Base address of Y buffer"]
        #[inline(always)]
        pub const fn y_base(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Base address of Y buffer"]
        #[inline(always)]
        pub fn set_y_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Size of Y buffer in 16-bit words"]
        #[inline(always)]
        pub const fn y_buf_size(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Size of Y buffer in 16-bit words"]
        #[inline(always)]
        pub fn set_y_buf_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Watermark for buffer empty flag"]
        #[inline(always)]
        pub const fn empty_wm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Watermark for buffer empty flag"]
        #[inline(always)]
        pub fn set_empty_wm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for Ybufcfg {
        #[inline(always)]
        fn default() -> Ybufcfg {
            Ybufcfg(0)
        }
    }
    impl core::fmt::Debug for Ybufcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ybufcfg")
                .field("y_base", &self.y_base())
                .field("y_buf_size", &self.y_buf_size())
                .field("empty_wm", &self.empty_wm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ybufcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ybufcfg {{ y_base: {=u8:?}, y_buf_size: {=u8:?}, empty_wm: {=u8:?} }}",
                self.y_base(),
                self.y_buf_size(),
                self.empty_wm()
            )
        }
    }
}
