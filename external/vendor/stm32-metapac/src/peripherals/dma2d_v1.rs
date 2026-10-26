#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DMA2D controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2d {
    ptr: *mut u8,
}
unsafe impl Send for Dma2d {}
unsafe impl Sync for Dma2d {}
impl Dma2d {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "foreground memory address register"]
    #[inline(always)]
    pub const fn fgmar(self) -> crate::common::Reg<regs::Fgmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "foreground offset register"]
    #[inline(always)]
    pub const fn fgor(self) -> crate::common::Reg<regs::Fgor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "background memory address register"]
    #[inline(always)]
    pub const fn bgmar(self) -> crate::common::Reg<regs::Bgmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "background offset register"]
    #[inline(always)]
    pub const fn bgor(self) -> crate::common::Reg<regs::Bgor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "foreground PFC control register"]
    #[inline(always)]
    pub const fn fgpfccr(self) -> crate::common::Reg<regs::Fgpfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "foreground color register"]
    #[inline(always)]
    pub const fn fgcolr(self) -> crate::common::Reg<regs::Fgcolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "background PFC control register"]
    #[inline(always)]
    pub const fn bgpfccr(self) -> crate::common::Reg<regs::Bgpfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "background color register"]
    #[inline(always)]
    pub const fn bgcolr(self) -> crate::common::Reg<regs::Bgcolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "foreground CLUT memory address register"]
    #[inline(always)]
    pub const fn fgcmar(self) -> crate::common::Reg<regs::Fgcmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "background CLUT memory address register"]
    #[inline(always)]
    pub const fn bgcmar(self) -> crate::common::Reg<regs::Bgcmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "output PFC control register"]
    #[inline(always)]
    pub const fn opfccr(self) -> crate::common::Reg<regs::Opfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "output color register"]
    #[inline(always)]
    pub const fn ocolr(self) -> crate::common::Reg<regs::Ocolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "output memory address register"]
    #[inline(always)]
    pub const fn omar(self) -> crate::common::Reg<regs::Omar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "output offset register"]
    #[inline(always)]
    pub const fn oor(self) -> crate::common::Reg<regs::Oor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "number of line register"]
    #[inline(always)]
    pub const fn nlr(self) -> crate::common::Reg<regs::Nlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "line watermark register"]
    #[inline(always)]
    pub const fn lwr(self) -> crate::common::Reg<regs::Lwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "AHB master timer configuration register"]
    #[inline(always)]
    pub const fn amtcr(self) -> crate::common::Reg<regs::Amtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "FGCLUT"]
    #[inline(always)]
    pub const fn fgclut(self) -> crate::common::Reg<regs::Fgclut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "BGCLUT"]
    #[inline(always)]
    pub const fn bgclut(self) -> crate::common::Reg<regs::Bgclut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB master timer configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Amtcr(pub u32);
    impl Amtcr {
        #[doc = "Enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Dead Time"]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Dead Time"]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Amtcr {
        #[inline(always)]
        fn default() -> Amtcr {
            Amtcr(0)
        }
    }
    impl core::fmt::Debug for Amtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Amtcr")
                .field("en", &self.en())
                .field("dt", &self.dt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Amtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Amtcr {{ en: {=bool:?}, dt: {=u8:?} }}", self.en(), self.dt())
        }
    }
    #[doc = "BGCLUT"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgclut(pub u32);
    impl Bgclut {
        #[doc = "BLUE"]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "BLUE"]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "GREEN"]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "GREEN"]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RED"]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "RED"]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "APLHA"]
        #[inline(always)]
        pub const fn aplha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "APLHA"]
        #[inline(always)]
        pub fn set_aplha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Bgclut {
        #[inline(always)]
        fn default() -> Bgclut {
            Bgclut(0)
        }
    }
    impl core::fmt::Debug for Bgclut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgclut")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .field("aplha", &self.aplha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgclut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bgclut {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?}, aplha: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red(),
                self.aplha()
            )
        }
    }
    #[doc = "background CLUT memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgcmar(pub u32);
    impl Bgcmar {
        #[doc = "Memory address"]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory address"]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bgcmar {
        #[inline(always)]
        fn default() -> Bgcmar {
            Bgcmar(0)
        }
    }
    impl core::fmt::Debug for Bgcmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgcmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgcmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bgcmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "background color register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgcolr(pub u32);
    impl Bgcolr {
        #[doc = "Blue Value"]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue Value"]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green Value"]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green Value"]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red Value"]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red Value"]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Bgcolr {
        #[inline(always)]
        fn default() -> Bgcolr {
            Bgcolr(0)
        }
    }
    impl core::fmt::Debug for Bgcolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgcolr")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgcolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bgcolr {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red()
            )
        }
    }
    #[doc = "background memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgmar(pub u32);
    impl Bgmar {
        #[doc = "Memory address"]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory address"]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bgmar {
        #[inline(always)]
        fn default() -> Bgmar {
            Bgmar(0)
        }
    }
    impl core::fmt::Debug for Bgmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bgmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "background offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgor(pub u32);
    impl Bgor {
        #[doc = "Line offset"]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Line offset"]
        #[inline(always)]
        pub fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Bgor {
        #[inline(always)]
        fn default() -> Bgor {
            Bgor(0)
        }
    }
    impl core::fmt::Debug for Bgor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bgor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[doc = "background PFC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgpfccr(pub u32);
    impl Bgpfccr {
        #[doc = "Color mode"]
        #[inline(always)]
        pub const fn cm(&self) -> super::vals::BgpfccrCm {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::BgpfccrCm::from_bits(val as u8)
        }
        #[doc = "Color mode"]
        #[inline(always)]
        pub fn set_cm(&mut self, val: super::vals::BgpfccrCm) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "CLUT Color mode"]
        #[inline(always)]
        pub const fn ccm(&self) -> super::vals::BgpfccrCcm {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::BgpfccrCcm::from_bits(val as u8)
        }
        #[doc = "CLUT Color mode"]
        #[inline(always)]
        pub fn set_ccm(&mut self, val: super::vals::BgpfccrCcm) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn start(&self) -> super::vals::BgpfccrStart {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::BgpfccrStart::from_bits(val as u8)
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_start(&mut self, val: super::vals::BgpfccrStart) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CLUT size"]
        #[inline(always)]
        pub const fn cs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "CLUT size"]
        #[inline(always)]
        pub fn set_cs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Alpha mode"]
        #[inline(always)]
        pub const fn am(&self) -> super::vals::BgpfccrAm {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::BgpfccrAm::from_bits(val as u8)
        }
        #[doc = "Alpha mode"]
        #[inline(always)]
        pub fn set_am(&mut self, val: super::vals::BgpfccrAm) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Alpha value"]
        #[inline(always)]
        pub const fn alpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Alpha value"]
        #[inline(always)]
        pub fn set_alpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Bgpfccr {
        #[inline(always)]
        fn default() -> Bgpfccr {
            Bgpfccr(0)
        }
    }
    impl core::fmt::Debug for Bgpfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgpfccr")
                .field("cm", &self.cm())
                .field("ccm", &self.ccm())
                .field("start", &self.start())
                .field("cs", &self.cs())
                .field("am", &self.am())
                .field("alpha", &self.alpha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgpfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bgpfccr {{ cm: {:?}, ccm: {:?}, start: {:?}, cs: {=u8:?}, am: {:?}, alpha: {=u8:?} }}",
                self.cm(),
                self.ccm(),
                self.start(),
                self.cs(),
                self.am(),
                self.alpha()
            )
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Start"]
        #[inline(always)]
        pub const fn start(&self) -> super::vals::CrStart {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::CrStart::from_bits(val as u8)
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_start(&mut self, val: super::vals::CrStart) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Suspend"]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend"]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Abort"]
        #[inline(always)]
        pub const fn abort(&self) -> super::vals::Abort {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Abort::from_bits(val as u8)
        }
        #[doc = "Abort"]
        #[inline(always)]
        pub fn set_abort(&mut self, val: super::vals::Abort) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Transfer error interrupt enable"]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable"]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transfer complete interrupt enable"]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable"]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transfer watermark interrupt enable"]
        #[inline(always)]
        pub const fn twie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer watermark interrupt enable"]
        #[inline(always)]
        pub fn set_twie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CLUT access error interrupt enable"]
        #[inline(always)]
        pub const fn caeie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT access error interrupt enable"]
        #[inline(always)]
        pub fn set_caeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CLUT transfer complete interrupt enable"]
        #[inline(always)]
        pub const fn ctcie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT transfer complete interrupt enable"]
        #[inline(always)]
        pub fn set_ctcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Configuration Error Interrupt Enable"]
        #[inline(always)]
        pub const fn ceie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Error Interrupt Enable"]
        #[inline(always)]
        pub fn set_ceie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DMA2D mode"]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "DMA2D mode"]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
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
                .field("start", &self.start())
                .field("susp", &self.susp())
                .field("abort", &self.abort())
                .field("teie", &self.teie())
                .field("tcie", &self.tcie())
                .field("twie", &self.twie())
                .field("caeie", &self.caeie())
                .field("ctcie", &self.ctcie())
                .field("ceie", &self.ceie())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ start: {:?}, susp: {=bool:?}, abort: {:?}, teie: {=bool:?}, tcie: {=bool:?}, twie: {=bool:?}, caeie: {=bool:?}, ctcie: {=bool:?}, ceie: {=bool:?}, mode: {:?} }}" , self . start () , self . susp () , self . abort () , self . teie () , self . tcie () , self . twie () , self . caeie () , self . ctcie () , self . ceie () , self . mode ())
        }
    }
    #[doc = "FGCLUT"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgclut(pub u32);
    impl Fgclut {
        #[doc = "BLUE"]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "BLUE"]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "GREEN"]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "GREEN"]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RED"]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "RED"]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "APLHA"]
        #[inline(always)]
        pub const fn aplha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "APLHA"]
        #[inline(always)]
        pub fn set_aplha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fgclut {
        #[inline(always)]
        fn default() -> Fgclut {
            Fgclut(0)
        }
    }
    impl core::fmt::Debug for Fgclut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgclut")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .field("aplha", &self.aplha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgclut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fgclut {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?}, aplha: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red(),
                self.aplha()
            )
        }
    }
    #[doc = "foreground CLUT memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgcmar(pub u32);
    impl Fgcmar {
        #[doc = "Memory Address"]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory Address"]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fgcmar {
        #[inline(always)]
        fn default() -> Fgcmar {
            Fgcmar(0)
        }
    }
    impl core::fmt::Debug for Fgcmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgcmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgcmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fgcmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "foreground color register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgcolr(pub u32);
    impl Fgcolr {
        #[doc = "Blue Value"]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue Value"]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green Value"]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green Value"]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red Value"]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red Value"]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Fgcolr {
        #[inline(always)]
        fn default() -> Fgcolr {
            Fgcolr(0)
        }
    }
    impl core::fmt::Debug for Fgcolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgcolr")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgcolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fgcolr {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red()
            )
        }
    }
    #[doc = "foreground memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgmar(pub u32);
    impl Fgmar {
        #[doc = "Memory address"]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory address"]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fgmar {
        #[inline(always)]
        fn default() -> Fgmar {
            Fgmar(0)
        }
    }
    impl core::fmt::Debug for Fgmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fgmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "foreground offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgor(pub u32);
    impl Fgor {
        #[doc = "Line offset"]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Line offset"]
        #[inline(always)]
        pub fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Fgor {
        #[inline(always)]
        fn default() -> Fgor {
            Fgor(0)
        }
    }
    impl core::fmt::Debug for Fgor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fgor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[doc = "foreground PFC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgpfccr(pub u32);
    impl Fgpfccr {
        #[doc = "Color mode"]
        #[inline(always)]
        pub const fn cm(&self) -> super::vals::FgpfccrCm {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::FgpfccrCm::from_bits(val as u8)
        }
        #[doc = "Color mode"]
        #[inline(always)]
        pub fn set_cm(&mut self, val: super::vals::FgpfccrCm) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "CLUT color mode"]
        #[inline(always)]
        pub const fn ccm(&self) -> super::vals::FgpfccrCcm {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::FgpfccrCcm::from_bits(val as u8)
        }
        #[doc = "CLUT color mode"]
        #[inline(always)]
        pub fn set_ccm(&mut self, val: super::vals::FgpfccrCcm) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn start(&self) -> super::vals::FgpfccrStart {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::FgpfccrStart::from_bits(val as u8)
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_start(&mut self, val: super::vals::FgpfccrStart) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CLUT size"]
        #[inline(always)]
        pub const fn cs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "CLUT size"]
        #[inline(always)]
        pub fn set_cs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Alpha mode"]
        #[inline(always)]
        pub const fn am(&self) -> super::vals::FgpfccrAm {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::FgpfccrAm::from_bits(val as u8)
        }
        #[doc = "Alpha mode"]
        #[inline(always)]
        pub fn set_am(&mut self, val: super::vals::FgpfccrAm) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Alpha value"]
        #[inline(always)]
        pub const fn alpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Alpha value"]
        #[inline(always)]
        pub fn set_alpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fgpfccr {
        #[inline(always)]
        fn default() -> Fgpfccr {
            Fgpfccr(0)
        }
    }
    impl core::fmt::Debug for Fgpfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgpfccr")
                .field("cm", &self.cm())
                .field("ccm", &self.ccm())
                .field("start", &self.start())
                .field("cs", &self.cs())
                .field("am", &self.am())
                .field("alpha", &self.alpha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgpfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fgpfccr {{ cm: {:?}, ccm: {:?}, start: {:?}, cs: {=u8:?}, am: {:?}, alpha: {=u8:?} }}",
                self.cm(),
                self.ccm(),
                self.start(),
                self.cs(),
                self.am(),
                self.alpha()
            )
        }
    }
    #[doc = "interrupt flag clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifcr(pub u32);
    impl Ifcr {
        #[doc = "Clear Transfer error interrupt flag"]
        #[inline(always)]
        pub const fn cteif(&self) -> super::vals::Cteif {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Cteif::from_bits(val as u8)
        }
        #[doc = "Clear Transfer error interrupt flag"]
        #[inline(always)]
        pub fn set_cteif(&mut self, val: super::vals::Cteif) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear transfer complete interrupt flag"]
        #[inline(always)]
        pub const fn ctcif(&self) -> super::vals::Ctcif {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Ctcif::from_bits(val as u8)
        }
        #[doc = "Clear transfer complete interrupt flag"]
        #[inline(always)]
        pub fn set_ctcif(&mut self, val: super::vals::Ctcif) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear transfer watermark interrupt flag"]
        #[inline(always)]
        pub const fn ctwif(&self) -> super::vals::Ctwif {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Ctwif::from_bits(val as u8)
        }
        #[doc = "Clear transfer watermark interrupt flag"]
        #[inline(always)]
        pub fn set_ctwif(&mut self, val: super::vals::Ctwif) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear CLUT access error interrupt flag"]
        #[inline(always)]
        pub const fn caecif(&self) -> super::vals::Caecif {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Caecif::from_bits(val as u8)
        }
        #[doc = "Clear CLUT access error interrupt flag"]
        #[inline(always)]
        pub fn set_caecif(&mut self, val: super::vals::Caecif) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear CLUT transfer complete interrupt flag"]
        #[inline(always)]
        pub const fn cctcif(&self) -> super::vals::Cctcif {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Cctcif::from_bits(val as u8)
        }
        #[doc = "Clear CLUT transfer complete interrupt flag"]
        #[inline(always)]
        pub fn set_cctcif(&mut self, val: super::vals::Cctcif) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear configuration error interrupt flag"]
        #[inline(always)]
        pub const fn cceif(&self) -> super::vals::Cceif {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Cceif::from_bits(val as u8)
        }
        #[doc = "Clear configuration error interrupt flag"]
        #[inline(always)]
        pub fn set_cceif(&mut self, val: super::vals::Cceif) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ifcr {
        #[inline(always)]
        fn default() -> Ifcr {
            Ifcr(0)
        }
    }
    impl core::fmt::Debug for Ifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifcr")
                .field("cteif", &self.cteif())
                .field("ctcif", &self.ctcif())
                .field("ctwif", &self.ctwif())
                .field("caecif", &self.caecif())
                .field("cctcif", &self.cctcif())
                .field("cceif", &self.cceif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ifcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ifcr {{ cteif: {:?}, ctcif: {:?}, ctwif: {:?}, caecif: {:?}, cctcif: {:?}, cceif: {:?} }}",
                self.cteif(),
                self.ctcif(),
                self.ctwif(),
                self.caecif(),
                self.cctcif(),
                self.cceif()
            )
        }
    }
    #[doc = "Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Transfer error interrupt flag"]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt flag"]
        #[inline(always)]
        pub fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer complete interrupt flag"]
        #[inline(always)]
        pub const fn tcif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt flag"]
        #[inline(always)]
        pub fn set_tcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer watermark interrupt flag"]
        #[inline(always)]
        pub const fn twif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer watermark interrupt flag"]
        #[inline(always)]
        pub fn set_twif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CLUT access error interrupt flag"]
        #[inline(always)]
        pub const fn caeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT access error interrupt flag"]
        #[inline(always)]
        pub fn set_caeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CLUT transfer complete interrupt flag"]
        #[inline(always)]
        pub const fn ctcif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT transfer complete interrupt flag"]
        #[inline(always)]
        pub fn set_ctcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Configuration error interrupt flag"]
        #[inline(always)]
        pub const fn ceif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration error interrupt flag"]
        #[inline(always)]
        pub fn set_ceif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("teif", &self.teif())
                .field("tcif", &self.tcif())
                .field("twif", &self.twif())
                .field("caeif", &self.caeif())
                .field("ctcif", &self.ctcif())
                .field("ceif", &self.ceif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ teif: {=bool:?}, tcif: {=bool:?}, twif: {=bool:?}, caeif: {=bool:?}, ctcif: {=bool:?}, ceif: {=bool:?} }}" , self . teif () , self . tcif () , self . twif () , self . caeif () , self . ctcif () , self . ceif ())
        }
    }
    #[doc = "line watermark register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lwr(pub u32);
    impl Lwr {
        #[doc = "Line watermark"]
        #[inline(always)]
        pub const fn lw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Line watermark"]
        #[inline(always)]
        pub fn set_lw(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lwr {
        #[inline(always)]
        fn default() -> Lwr {
            Lwr(0)
        }
    }
    impl core::fmt::Debug for Lwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lwr").field("lw", &self.lw()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lwr {{ lw: {=u16:?} }}", self.lw())
        }
    }
    #[doc = "number of line register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nlr(pub u32);
    impl Nlr {
        #[doc = "Number of lines"]
        #[inline(always)]
        pub const fn nl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of lines"]
        #[inline(always)]
        pub fn set_nl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Pixel per lines"]
        #[inline(always)]
        pub const fn pl(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "Pixel per lines"]
        #[inline(always)]
        pub fn set_pl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Nlr {
        #[inline(always)]
        fn default() -> Nlr {
            Nlr(0)
        }
    }
    impl core::fmt::Debug for Nlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nlr")
                .field("nl", &self.nl())
                .field("pl", &self.pl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nlr {{ nl: {=u16:?}, pl: {=u16:?} }}", self.nl(), self.pl())
        }
    }
    #[doc = "output color register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ocolr(pub u32);
    impl Ocolr {
        #[doc = "Blue Value"]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue Value"]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green Value"]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green Value"]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red Value"]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red Value"]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Alpha Channel Value"]
        #[inline(always)]
        pub const fn aplha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Alpha Channel Value"]
        #[inline(always)]
        pub fn set_aplha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Ocolr {
        #[inline(always)]
        fn default() -> Ocolr {
            Ocolr(0)
        }
    }
    impl core::fmt::Debug for Ocolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ocolr")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .field("aplha", &self.aplha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ocolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ocolr {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?}, aplha: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red(),
                self.aplha()
            )
        }
    }
    #[doc = "output memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Omar(pub u32);
    impl Omar {
        #[doc = "Memory Address"]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory Address"]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Omar {
        #[inline(always)]
        fn default() -> Omar {
            Omar(0)
        }
    }
    impl core::fmt::Debug for Omar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Omar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Omar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Omar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "output offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oor(pub u32);
    impl Oor {
        #[doc = "Line Offset"]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Line Offset"]
        #[inline(always)]
        pub fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Oor {
        #[inline(always)]
        fn default() -> Oor {
            Oor(0)
        }
    }
    impl core::fmt::Debug for Oor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[doc = "output PFC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opfccr(pub u32);
    impl Opfccr {
        #[doc = "Color mode"]
        #[inline(always)]
        pub const fn cm(&self) -> super::vals::OpfccrCm {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::OpfccrCm::from_bits(val as u8)
        }
        #[doc = "Color mode"]
        #[inline(always)]
        pub fn set_cm(&mut self, val: super::vals::OpfccrCm) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Opfccr {
        #[inline(always)]
        fn default() -> Opfccr {
            Opfccr(0)
        }
    }
    impl core::fmt::Debug for Opfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opfccr").field("cm", &self.cm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Opfccr {{ cm: {:?} }}", self.cm())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Abort {
        _RESERVED_0 = 0x0,
        #[doc = "Transfer abort requested"]
        ABORT_REQUEST = 0x01,
    }
    impl Abort {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Abort {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Abort {
        #[inline(always)]
        fn from(val: u8) -> Abort {
            Abort::from_bits(val)
        }
    }
    impl From<Abort> for u8 {
        #[inline(always)]
        fn from(val: Abort) -> u8 {
            Abort::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrAm {
        #[doc = "No modification of alpha channel"]
        NO_MODIFY = 0x0,
        #[doc = "Replace with value in ALPHA\\[7:0\\]"]
        REPLACE = 0x01,
        #[doc = "Multiply with value in ALPHA\\[7:0\\]"]
        MULTIPLY = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl BgpfccrAm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrAm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrAm {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrAm {
            BgpfccrAm::from_bits(val)
        }
    }
    impl From<BgpfccrAm> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrAm) -> u8 {
            BgpfccrAm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrCcm {
        #[doc = "CLUT color format ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "CLUT color format RGB888"]
        RGB888 = 0x01,
    }
    impl BgpfccrCcm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrCcm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrCcm {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrCcm {
            BgpfccrCcm::from_bits(val)
        }
    }
    impl From<BgpfccrCcm> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrCcm) -> u8 {
            BgpfccrCcm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrCm {
        #[doc = "Color mode ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "Color mode RGB888"]
        RGB888 = 0x01,
        #[doc = "Color mode RGB565"]
        RGB565 = 0x02,
        #[doc = "Color mode ARGB1555"]
        ARGB1555 = 0x03,
        #[doc = "Color mode ARGB4444"]
        ARGB4444 = 0x04,
        #[doc = "Color mode L8"]
        L8 = 0x05,
        #[doc = "Color mode AL44"]
        AL44 = 0x06,
        #[doc = "Color mode AL88"]
        AL88 = 0x07,
        #[doc = "Color mode L4"]
        L4 = 0x08,
        #[doc = "Color mode A8"]
        A8 = 0x09,
        #[doc = "Color mode A4"]
        A4 = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl BgpfccrCm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrCm {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrCm {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrCm {
            BgpfccrCm::from_bits(val)
        }
    }
    impl From<BgpfccrCm> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrCm) -> u8 {
            BgpfccrCm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrStart {
        _RESERVED_0 = 0x0,
        #[doc = "Start the automatic loading of the CLUT"]
        START = 0x01,
    }
    impl BgpfccrStart {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrStart {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrStart {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrStart {
            BgpfccrStart::from_bits(val)
        }
    }
    impl From<BgpfccrStart> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrStart) -> u8 {
            BgpfccrStart::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Caecif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the CAEIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Caecif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Caecif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Caecif {
        #[inline(always)]
        fn from(val: u8) -> Caecif {
            Caecif::from_bits(val)
        }
    }
    impl From<Caecif> for u8 {
        #[inline(always)]
        fn from(val: Caecif) -> u8 {
            Caecif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cceif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the CEIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Cceif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cceif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cceif {
        #[inline(always)]
        fn from(val: u8) -> Cceif {
            Cceif::from_bits(val)
        }
    }
    impl From<Cceif> for u8 {
        #[inline(always)]
        fn from(val: Cceif) -> u8 {
            Cceif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cctcif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the CTCIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Cctcif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cctcif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cctcif {
        #[inline(always)]
        fn from(val: u8) -> Cctcif {
            Cctcif::from_bits(val)
        }
    }
    impl From<Cctcif> for u8 {
        #[inline(always)]
        fn from(val: Cctcif) -> u8 {
            Cctcif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CrStart {
        _RESERVED_0 = 0x0,
        #[doc = "Launch the DMA2D"]
        START = 0x01,
    }
    impl CrStart {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CrStart {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CrStart {
        #[inline(always)]
        fn from(val: u8) -> CrStart {
            CrStart::from_bits(val)
        }
    }
    impl From<CrStart> for u8 {
        #[inline(always)]
        fn from(val: CrStart) -> u8 {
            CrStart::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ctcif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the TCIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Ctcif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ctcif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ctcif {
        #[inline(always)]
        fn from(val: u8) -> Ctcif {
            Ctcif::from_bits(val)
        }
    }
    impl From<Ctcif> for u8 {
        #[inline(always)]
        fn from(val: Ctcif) -> u8 {
            Ctcif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cteif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the TEIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Cteif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cteif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cteif {
        #[inline(always)]
        fn from(val: u8) -> Cteif {
            Cteif::from_bits(val)
        }
    }
    impl From<Cteif> for u8 {
        #[inline(always)]
        fn from(val: Cteif) -> u8 {
            Cteif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ctwif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the TWIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Ctwif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ctwif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ctwif {
        #[inline(always)]
        fn from(val: u8) -> Ctwif {
            Ctwif::from_bits(val)
        }
    }
    impl From<Ctwif> for u8 {
        #[inline(always)]
        fn from(val: Ctwif) -> u8 {
            Ctwif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrAm {
        #[doc = "No modification of alpha channel"]
        NO_MODIFY = 0x0,
        #[doc = "Replace with value in ALPHA\\[7:0\\]"]
        REPLACE = 0x01,
        #[doc = "Multiply with value in ALPHA\\[7:0\\]"]
        MULTIPLY = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl FgpfccrAm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrAm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrAm {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrAm {
            FgpfccrAm::from_bits(val)
        }
    }
    impl From<FgpfccrAm> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrAm) -> u8 {
            FgpfccrAm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrCcm {
        #[doc = "CLUT color format ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "CLUT color format RGB888"]
        RGB888 = 0x01,
    }
    impl FgpfccrCcm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrCcm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrCcm {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrCcm {
            FgpfccrCcm::from_bits(val)
        }
    }
    impl From<FgpfccrCcm> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrCcm) -> u8 {
            FgpfccrCcm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrCm {
        #[doc = "Color mode ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "Color mode RGB888"]
        RGB888 = 0x01,
        #[doc = "Color mode RGB565"]
        RGB565 = 0x02,
        #[doc = "Color mode ARGB1555"]
        ARGB1555 = 0x03,
        #[doc = "Color mode ARGB4444"]
        ARGB4444 = 0x04,
        #[doc = "Color mode L8"]
        L8 = 0x05,
        #[doc = "Color mode AL44"]
        AL44 = 0x06,
        #[doc = "Color mode AL88"]
        AL88 = 0x07,
        #[doc = "Color mode L4"]
        L4 = 0x08,
        #[doc = "Color mode A8"]
        A8 = 0x09,
        #[doc = "Color mode A4"]
        A4 = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl FgpfccrCm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrCm {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrCm {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrCm {
            FgpfccrCm::from_bits(val)
        }
    }
    impl From<FgpfccrCm> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrCm) -> u8 {
            FgpfccrCm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrStart {
        _RESERVED_0 = 0x0,
        #[doc = "Start the automatic loading of the CLUT"]
        START = 0x01,
    }
    impl FgpfccrStart {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrStart {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrStart {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrStart {
            FgpfccrStart::from_bits(val)
        }
    }
    impl From<FgpfccrStart> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrStart) -> u8 {
            FgpfccrStart::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Memory-to-memory (FG fetch only)"]
        MEMORY_TO_MEMORY = 0x0,
        #[doc = "Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
        MEMORY_TO_MEMORY_PFC = 0x01,
        #[doc = "Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
        MEMORY_TO_MEMORY_PFCBLENDING = 0x02,
        #[doc = "Register-to-memory"]
        REGISTER_TO_MEMORY = 0x03,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OpfccrCm {
        #[doc = "ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "RGB888"]
        RGB888 = 0x01,
        #[doc = "RGB565"]
        RGB565 = 0x02,
        #[doc = "ARGB1555"]
        ARGB1555 = 0x03,
        #[doc = "ARGB4444"]
        ARGB4444 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl OpfccrCm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpfccrCm {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpfccrCm {
        #[inline(always)]
        fn from(val: u8) -> OpfccrCm {
            OpfccrCm::from_bits(val)
        }
    }
    impl From<OpfccrCm> for u8 {
        #[inline(always)]
        fn from(val: OpfccrCm) -> u8 {
            OpfccrCm::to_bits(val)
        }
    }
}
