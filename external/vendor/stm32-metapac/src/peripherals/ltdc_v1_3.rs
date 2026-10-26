#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cluster Layer%S"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer {
    ptr: *mut u8,
}
unsafe impl Send for Layer {}
unsafe impl Sync for Layer {}
impl Layer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Layerx Configuration 0 Register."]
    #[inline(always)]
    pub const fn c0r(self) -> crate::common::Reg<regs::C0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Layerx Configuration 1 Register."]
    #[inline(always)]
    pub const fn c1r(self) -> crate::common::Reg<regs::C1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Layerx Reload Control Register."]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Layerx Control Register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Layerx Window Horizontal Position Configuration Register."]
    #[inline(always)]
    pub const fn whpcr(self) -> crate::common::Reg<regs::Whpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Layerx Window Vertical Position Configuration Register."]
    #[inline(always)]
    pub const fn wvpcr(self) -> crate::common::Reg<regs::Wvpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Layerx Color Keying Configuration Register."]
    #[inline(always)]
    pub const fn ckcr(self) -> crate::common::Reg<regs::Ckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Layerx Pixel Format Configuration Register."]
    #[inline(always)]
    pub const fn pfcr(self) -> crate::common::Reg<regs::Pfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Layerx Constant Alpha Configuration Register."]
    #[inline(always)]
    pub const fn cacr(self) -> crate::common::Reg<regs::Cacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Layerx Default Color Configuration Register."]
    #[inline(always)]
    pub const fn dccr(self) -> crate::common::Reg<regs::Dccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Layerx Blending Factors Configuration Register."]
    #[inline(always)]
    pub const fn bfcr(self) -> crate::common::Reg<regs::Bfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Layerx Burst Length Configuration Register."]
    #[inline(always)]
    pub const fn blcr(self) -> crate::common::Reg<regs::Blcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Layerx Planar Configuration Register."]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Address Register."]
    #[inline(always)]
    pub const fn cfbar(self) -> crate::common::Reg<regs::Cfbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Length Register."]
    #[inline(always)]
    pub const fn cfblr(self) -> crate::common::Reg<regs::Cfblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Line Number Register."]
    #[inline(always)]
    pub const fn cfblnr(self) -> crate::common::Reg<regs::Cfblnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Layer1 Auxiliary Frame Buffer Address 0 Register."]
    #[inline(always)]
    pub const fn afba0r(self) -> crate::common::Reg<regs::Afba0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Layer1 Auxiliary Frame Buffer Address 1 Register."]
    #[inline(always)]
    pub const fn afba1r(self) -> crate::common::Reg<regs::Afba1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Layer1 Auxiliary Frame Buffer Length Register."]
    #[inline(always)]
    pub const fn afblr(self) -> crate::common::Reg<regs::Afblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Layer1 Auxiliary Frame Buffer Line Number Register."]
    #[inline(always)]
    pub const fn afblnr(self) -> crate::common::Reg<regs::Afblnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Layerx Clut Write Register."]
    #[inline(always)]
    pub const fn clutwr(self) -> crate::common::Reg<regs::Clutwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Layerx Conversion Ycbcr Rgb 0 Register."]
    #[inline(always)]
    pub const fn cyr0r(self) -> crate::common::Reg<regs::Cyr0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Layerx Conversion Ycbcr Rgb 1 Register."]
    #[inline(always)]
    pub const fn cyr1r(self) -> crate::common::Reg<regs::Cyr1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Layerx Flexible Pixel Format 0 Register."]
    #[inline(always)]
    pub const fn fpf0r(self) -> crate::common::Reg<regs::Fpf0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Layerx Flexible Pixel Format 1 Register."]
    #[inline(always)]
    pub const fn fpf1r(self) -> crate::common::Reg<regs::Fpf1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
}
#[doc = "LCD-TFT Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ltdc {
    ptr: *mut u8,
}
unsafe impl Send for Ltdc {}
unsafe impl Sync for Ltdc {}
impl Ltdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ltdc Synchronization Size Configuration Register."]
    #[inline(always)]
    pub const fn sscr(self) -> crate::common::Reg<regs::Sscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Ltdc Back Porch Configuration Register."]
    #[inline(always)]
    pub const fn bpcr(self) -> crate::common::Reg<regs::Bpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Ltdc Active Width Configuration Register."]
    #[inline(always)]
    pub const fn awcr(self) -> crate::common::Reg<regs::Awcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Ltdc Total Width Configuration Register."]
    #[inline(always)]
    pub const fn twcr(self) -> crate::common::Reg<regs::Twcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Ltdc Global Control Register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Ltdc Global Configuration 1 Register."]
    #[inline(always)]
    pub const fn gc1r(self) -> crate::common::Reg<regs::Gc1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Ltdc Global Configuration 2 Register."]
    #[inline(always)]
    pub const fn gc2r(self) -> crate::common::Reg<regs::Gc2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Ltdc Shadow Reload Configuration Register."]
    #[inline(always)]
    pub const fn srcr(self) -> crate::common::Reg<regs::Srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Ltdc Gamma Correction Configuration Register."]
    #[inline(always)]
    pub const fn gccr(self) -> crate::common::Reg<regs::Gccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Ltdc Background Color Configuration Register."]
    #[inline(always)]
    pub const fn bccr(self) -> crate::common::Reg<regs::Bccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Ltdc Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Ltdc Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Ltdc Interrupt Clear Register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Line Interrupt Position Configuration Register."]
    #[inline(always)]
    pub const fn lipcr(self) -> crate::common::Reg<regs::Lipcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Ltdc Current Position Status Register."]
    #[inline(always)]
    pub const fn cpsr(self) -> crate::common::Reg<regs::Cpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Ltdc Current Display Status Register."]
    #[inline(always)]
    pub const fn cdsr(self) -> crate::common::Reg<regs::Cdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Ltdc External Display Control Register."]
    #[inline(always)]
    pub const fn edcr(self) -> crate::common::Reg<regs::Edcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Ltdc Interrupt Enable Register 2."]
    #[inline(always)]
    pub const fn ier2(self) -> crate::common::Reg<regs::Ier2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Ltdc Interrupt Status Register 2."]
    #[inline(always)]
    pub const fn isr2(self) -> crate::common::Reg<regs::Isr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Ltdc Interrupt Clear Register 2."]
    #[inline(always)]
    pub const fn icr2(self) -> crate::common::Reg<regs::Icr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Line Interrupt Position Configuration Register 2."]
    #[inline(always)]
    pub const fn lipcr2(self) -> crate::common::Reg<regs::Lipcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Ltdc Expected Crc Register."]
    #[inline(always)]
    pub const fn ecrcr(self) -> crate::common::Reg<regs::Ecrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Ltdc Computed Crc Register."]
    #[inline(always)]
    pub const fn ccrcr(self) -> crate::common::Reg<regs::Ccrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Ltdc Fifo Underrun Threshold Register."]
    #[inline(always)]
    pub const fn futr(self) -> crate::common::Reg<regs::Futr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Cluster Layer%S"]
    #[inline(always)]
    pub const fn layer(self, n: usize) -> Layer {
        assert!(n < 2usize);
        unsafe { Layer::from_ptr(self.ptr.add(0x0100usize + n * 256usize) as _) }
    }
}
pub mod regs {
    #[doc = "Layer1 Auxiliary Frame Buffer Address 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afba0r(pub u32);
    impl Afba0r {
        #[doc = "Frame Buffer Start Address."]
        #[inline(always)]
        pub const fn afbadd0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame Buffer Start Address."]
        #[inline(always)]
        pub fn set_afbadd0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Afba0r {
        #[inline(always)]
        fn default() -> Afba0r {
            Afba0r(0)
        }
    }
    impl core::fmt::Debug for Afba0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afba0r").field("afbadd0", &self.afbadd0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afba0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Afba0r {{ afbadd0: {=u32:?} }}", self.afbadd0())
        }
    }
    #[doc = "Layer1 Auxiliary Frame Buffer Address 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afba1r(pub u32);
    impl Afba1r {
        #[doc = "Auxiliary Frame Buffer Start Address."]
        #[inline(always)]
        pub const fn afbadd1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Auxiliary Frame Buffer Start Address."]
        #[inline(always)]
        pub fn set_afbadd1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Afba1r {
        #[inline(always)]
        fn default() -> Afba1r {
            Afba1r(0)
        }
    }
    impl core::fmt::Debug for Afba1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afba1r").field("afbadd1", &self.afbadd1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afba1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Afba1r {{ afbadd1: {=u32:?} }}", self.afbadd1())
        }
    }
    #[doc = "Layer1 Auxiliary Frame Buffer Line Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afblnr(pub u32);
    impl Afblnr {
        #[doc = "Auxiliary Frame Buffer Line Number."]
        #[inline(always)]
        pub const fn afblnbr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auxiliary Frame Buffer Line Number."]
        #[inline(always)]
        pub fn set_afblnbr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Afblnr {
        #[inline(always)]
        fn default() -> Afblnr {
            Afblnr(0)
        }
    }
    impl core::fmt::Debug for Afblnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afblnr").field("afblnbr", &self.afblnbr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afblnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Afblnr {{ afblnbr: {=u16:?} }}", self.afblnbr())
        }
    }
    #[doc = "Layer1 Auxiliary Frame Buffer Length Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afblr(pub u32);
    impl Afblr {
        #[doc = "Auxiliary Frame Buffer Line Length."]
        #[inline(always)]
        pub const fn afbll(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auxiliary Frame Buffer Line Length."]
        #[inline(always)]
        pub fn set_afbll(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Auxiliary Frame Buffer Pitch In Bytes."]
        #[inline(always)]
        pub const fn afbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Auxiliary Frame Buffer Pitch In Bytes."]
        #[inline(always)]
        pub fn set_afbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Afblr {
        #[inline(always)]
        fn default() -> Afblr {
            Afblr(0)
        }
    }
    impl core::fmt::Debug for Afblr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afblr")
                .field("afbll", &self.afbll())
                .field("afbp", &self.afbp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afblr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afblr {{ afbll: {=u16:?}, afbp: {=u16:?} }}",
                self.afbll(),
                self.afbp()
            )
        }
    }
    #[doc = "Ltdc Active Width Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awcr(pub u32);
    impl Awcr {
        #[doc = "Accumulated Active Height (In Units Of Horizontal Scan Line)."]
        #[inline(always)]
        pub const fn aah(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Accumulated Active Height (In Units Of Horizontal Scan Line)."]
        #[inline(always)]
        pub fn set_aah(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Accumulated Active Width (In Units Of Pixel Clock Period)."]
        #[inline(always)]
        pub const fn aaw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Accumulated Active Width (In Units Of Pixel Clock Period)."]
        #[inline(always)]
        pub fn set_aaw(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Awcr {
        #[inline(always)]
        fn default() -> Awcr {
            Awcr(0)
        }
    }
    impl core::fmt::Debug for Awcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awcr")
                .field("aah", &self.aah())
                .field("aaw", &self.aaw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Awcr {{ aah: {=u16:?}, aaw: {=u16:?} }}", self.aah(), self.aaw())
        }
    }
    #[doc = "Ltdc Background Color Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bccr(pub u32);
    impl Bccr {
        #[doc = "Background Color Blue Value."]
        #[inline(always)]
        pub const fn bcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Background Color Blue Value."]
        #[inline(always)]
        pub fn set_bcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Background Color Green Value."]
        #[inline(always)]
        pub const fn bcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Background Color Green Value."]
        #[inline(always)]
        pub fn set_bcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Background Color Red Value."]
        #[inline(always)]
        pub const fn bcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Background Color Red Value."]
        #[inline(always)]
        pub fn set_bcred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Bccr {
        #[inline(always)]
        fn default() -> Bccr {
            Bccr(0)
        }
    }
    impl core::fmt::Debug for Bccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bccr")
                .field("bcblue", &self.bcblue())
                .field("bcgreen", &self.bcgreen())
                .field("bcred", &self.bcred())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bccr {{ bcblue: {=u8:?}, bcgreen: {=u8:?}, bcred: {=u8:?} }}",
                self.bcblue(),
                self.bcgreen(),
                self.bcred()
            )
        }
    }
    #[doc = "Layerx Blending Factors Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bfcr(pub u32);
    impl Bfcr {
        #[doc = "Blending Factor 2."]
        #[inline(always)]
        pub const fn bf2(&self) -> super::vals::Bf2 {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Bf2::from_bits(val as u8)
        }
        #[doc = "Blending Factor 2."]
        #[inline(always)]
        pub fn set_bf2(&mut self, val: super::vals::Bf2) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Blending Factor 1."]
        #[inline(always)]
        pub const fn bf1(&self) -> super::vals::Bf1 {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Bf1::from_bits(val as u8)
        }
        #[doc = "Blending Factor 1."]
        #[inline(always)]
        pub fn set_bf1(&mut self, val: super::vals::Bf1) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Blending Order."]
        #[inline(always)]
        pub const fn bor(&self) -> super::vals::Bor {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Bor::from_bits(val as u8)
        }
        #[doc = "Blending Order."]
        #[inline(always)]
        pub fn set_bor(&mut self, val: super::vals::Bor) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Bfcr {
        #[inline(always)]
        fn default() -> Bfcr {
            Bfcr(0)
        }
    }
    impl core::fmt::Debug for Bfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bfcr")
                .field("bf2", &self.bf2())
                .field("bf1", &self.bf1())
                .field("bor", &self.bor())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bfcr {{ bf2: {:?}, bf1: {:?}, bor: {:?} }}",
                self.bf2(),
                self.bf1(),
                self.bor()
            )
        }
    }
    #[doc = "Layerx Burst Length Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Blcr(pub u32);
    impl Blcr {
        #[doc = "Burst Length."]
        #[inline(always)]
        pub const fn bl(&self) -> super::vals::Bl {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Bl::from_bits(val as u8)
        }
        #[doc = "Burst Length."]
        #[inline(always)]
        pub fn set_bl(&mut self, val: super::vals::Bl) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Blcr {
        #[inline(always)]
        fn default() -> Blcr {
            Blcr(0)
        }
    }
    impl core::fmt::Debug for Blcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Blcr").field("bl", &self.bl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Blcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Blcr {{ bl: {:?} }}", self.bl())
        }
    }
    #[doc = "Ltdc Back Porch Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcr(pub u32);
    impl Bpcr {
        #[doc = "Accumulated Vertical Back Porch (In Units Of Horizontal Scan Line)."]
        #[inline(always)]
        pub const fn avbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Accumulated Vertical Back Porch (In Units Of Horizontal Scan Line)."]
        #[inline(always)]
        pub fn set_avbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Accumulated Horizontal Back Porch (In Units Of Pixel Clock Period)."]
        #[inline(always)]
        pub const fn ahbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Accumulated Horizontal Back Porch (In Units Of Pixel Clock Period)."]
        #[inline(always)]
        pub fn set_ahbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Bpcr {
        #[inline(always)]
        fn default() -> Bpcr {
            Bpcr(0)
        }
    }
    impl core::fmt::Debug for Bpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bpcr")
                .field("avbp", &self.avbp())
                .field("ahbp", &self.ahbp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bpcr {{ avbp: {=u16:?}, ahbp: {=u16:?} }}", self.avbp(), self.ahbp())
        }
    }
    #[doc = "Layerx Configuration 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C0r(pub u32);
    impl C0r {
        #[doc = "Color Key Transparency Ability."]
        #[inline(always)]
        pub const fn ckta(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Color Key Transparency Ability."]
        #[inline(always)]
        pub fn set_ckta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color Frame Buffer Duplication Ability."]
        #[inline(always)]
        pub const fn cfbda(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Color Frame Buffer Duplication Ability."]
        #[inline(always)]
        pub fn set_cfbda(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Color Frame Buffer Pitch Ability."]
        #[inline(always)]
        pub const fn cfbpa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Color Frame Buffer Pitch Ability."]
        #[inline(always)]
        pub fn set_cfbpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Alpha Plane Ability."]
        #[inline(always)]
        pub const fn apa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Alpha Plane Ability."]
        #[inline(always)]
        pub fn set_apa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Default Color Programmability."]
        #[inline(always)]
        pub const fn dcp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Default Color Programmability."]
        #[inline(always)]
        pub fn set_dcp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Windowing Ability."]
        #[inline(always)]
        pub const fn wina(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Windowing Ability."]
        #[inline(always)]
        pub fn set_wina(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clut Ability."]
        #[inline(always)]
        pub const fn cluta(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clut Ability."]
        #[inline(always)]
        pub fn set_cluta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Color Key Replace Ability."]
        #[inline(always)]
        pub const fn ckra(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Color Key Replace Ability."]
        #[inline(always)]
        pub fn set_ckra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Blending Factor 2, Ability For: 1.0."]
        #[inline(always)]
        pub const fn f21(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 2, Ability For: 1.0."]
        #[inline(always)]
        pub fn set_f21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Blending Factor 2, Ability For: 0.0."]
        #[inline(always)]
        pub const fn f20(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 2, Ability For: 0.0."]
        #[inline(always)]
        pub fn set_f20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Blending Factor 2, Ability For: Pixel_Alpha."]
        #[inline(always)]
        pub const fn f2p(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 2, Ability For: Pixel_Alpha."]
        #[inline(always)]
        pub fn set_f2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Blending Factor 2, Ability For: 1.0 - Pixel_Alpha."]
        #[inline(always)]
        pub const fn f21p(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 2, Ability For: 1.0 - Pixel_Alpha."]
        #[inline(always)]
        pub fn set_f21p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Blending Factor 2, Ability For: Constant_Alpha."]
        #[inline(always)]
        pub const fn f2c(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 2, Ability For: Constant_Alpha."]
        #[inline(always)]
        pub fn set_f2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Blending Factor 2, Ability For: 1.0 - Constant_Alpha."]
        #[inline(always)]
        pub const fn f21c(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 2, Ability For: 1.0 - Constant_Alpha."]
        #[inline(always)]
        pub fn set_f21c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Blending Factor 2, Ability For: Pixel_Alpha * Constant_Alpha."]
        #[inline(always)]
        pub const fn f2pc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 2, Ability For: Pixel_Alpha * Constant_Alpha."]
        #[inline(always)]
        pub fn set_f2pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Blending Factor 2, Ability For: 1.0 - (Pixel_Alpha * Constant_Alpha)."]
        #[inline(always)]
        pub const fn f21pc(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 2, Ability For: 1.0 - (Pixel_Alpha * Constant_Alpha)."]
        #[inline(always)]
        pub fn set_f21pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Blending Factor 1, Ability For: 1.0."]
        #[inline(always)]
        pub const fn f11(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 1, Ability For: 1.0."]
        #[inline(always)]
        pub fn set_f11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Blending Factor 1,Ability For: 0.0."]
        #[inline(always)]
        pub const fn f10(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 1,Ability For: 0.0."]
        #[inline(always)]
        pub fn set_f10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Blending Factor 1, Ability For: Pixel_Alpha."]
        #[inline(always)]
        pub const fn f1p(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 1, Ability For: Pixel_Alpha."]
        #[inline(always)]
        pub fn set_f1p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Blending Factor 1, Ability For: 1.0 - Pixel_Alpha."]
        #[inline(always)]
        pub const fn f11p(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 1, Ability For: 1.0 - Pixel_Alpha."]
        #[inline(always)]
        pub fn set_f11p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Blending Factor 1, Ability For: Constant_Alpha."]
        #[inline(always)]
        pub const fn f1c(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 1, Ability For: Constant_Alpha."]
        #[inline(always)]
        pub fn set_f1c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Blending Factor 1, Ability For: 1.0 - Constant_Alpha."]
        #[inline(always)]
        pub const fn f11c(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 1, Ability For: 1.0 - Constant_Alpha."]
        #[inline(always)]
        pub fn set_f11c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Blending Factor 1, Ability For: Pixel_Alpha * Constant_Alpha."]
        #[inline(always)]
        pub const fn f1pc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 1, Ability For: Pixel_Alpha * Constant_Alpha."]
        #[inline(always)]
        pub fn set_f1pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Blending Factor 1, Ability For: 1.0 - (Pixel_Alpha * Constant_Alpha)."]
        #[inline(always)]
        pub const fn f11pc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Blending Factor 1, Ability For: 1.0 - (Pixel_Alpha * Constant_Alpha)."]
        #[inline(always)]
        pub fn set_f11pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Flexible Pixel Format, Ability."]
        #[inline(always)]
        pub const fn ff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible Pixel Format, Ability."]
        #[inline(always)]
        pub fn set_ff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Pixel Format, Ability For Rgb888."]
        #[inline(always)]
        pub const fn rgb888(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel Format, Ability For Rgb888."]
        #[inline(always)]
        pub fn set_rgb888(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Pixel Format, Ability For Bgr565."]
        #[inline(always)]
        pub const fn bgr565(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel Format, Ability For Bgr565."]
        #[inline(always)]
        pub fn set_bgr565(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Pixel Format, Ability For Rgb565."]
        #[inline(always)]
        pub const fn rgb565(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel Format, Ability For Rgb565."]
        #[inline(always)]
        pub fn set_rgb565(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Pixel Format, Ability For Bgra8888."]
        #[inline(always)]
        pub const fn bgra8888(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel Format, Ability For Bgra8888."]
        #[inline(always)]
        pub fn set_bgra8888(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Pixel Format, Ability For Rgba8888."]
        #[inline(always)]
        pub const fn rgba8888(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel Format, Ability For Rgba8888."]
        #[inline(always)]
        pub fn set_rgba8888(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Pixel Format, Ability For Abgr8888."]
        #[inline(always)]
        pub const fn abgr8888(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel Format, Ability For Abgr8888."]
        #[inline(always)]
        pub fn set_abgr8888(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Pixel Format, Ability For Argb8888."]
        #[inline(always)]
        pub const fn argb8888(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel Format, Ability For Argb8888."]
        #[inline(always)]
        pub fn set_argb8888(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C0r {
        #[inline(always)]
        fn default() -> C0r {
            C0r(0)
        }
    }
    impl core::fmt::Debug for C0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C0r")
                .field("ckta", &self.ckta())
                .field("cfbda", &self.cfbda())
                .field("cfbpa", &self.cfbpa())
                .field("apa", &self.apa())
                .field("dcp", &self.dcp())
                .field("wina", &self.wina())
                .field("cluta", &self.cluta())
                .field("ckra", &self.ckra())
                .field("f21", &self.f21())
                .field("f20", &self.f20())
                .field("f2p", &self.f2p())
                .field("f21p", &self.f21p())
                .field("f2c", &self.f2c())
                .field("f21c", &self.f21c())
                .field("f2pc", &self.f2pc())
                .field("f21pc", &self.f21pc())
                .field("f11", &self.f11())
                .field("f10", &self.f10())
                .field("f1p", &self.f1p())
                .field("f11p", &self.f11p())
                .field("f1c", &self.f1c())
                .field("f11c", &self.f11c())
                .field("f1pc", &self.f1pc())
                .field("f11pc", &self.f11pc())
                .field("ff", &self.ff())
                .field("rgb888", &self.rgb888())
                .field("bgr565", &self.bgr565())
                .field("rgb565", &self.rgb565())
                .field("bgra8888", &self.bgra8888())
                .field("rgba8888", &self.rgba8888())
                .field("abgr8888", &self.abgr8888())
                .field("argb8888", &self.argb8888())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C0r {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "C0r {{ ckta: {=bool:?}, cfbda: {=bool:?}, cfbpa: {=bool:?}, apa: {=bool:?}, dcp: {=bool:?}, wina: {=bool:?}, cluta: {=bool:?}, ckra: {=bool:?}, f21: {=bool:?}, f20: {=bool:?}, f2p: {=bool:?}, f21p: {=bool:?}, f2c: {=bool:?}, f21c: {=bool:?}, f2pc: {=bool:?}, f21pc: {=bool:?}, f11: {=bool:?}, f10: {=bool:?}, f1p: {=bool:?}, f11p: {=bool:?}, f1c: {=bool:?}, f11c: {=bool:?}, f1pc: {=bool:?}, f11pc: {=bool:?}, ff: {=bool:?}, rgb888: {=bool:?}, bgr565: {=bool:?}, rgb565: {=bool:?}, bgra8888: {=bool:?}, rgba8888: {=bool:?}, abgr8888: {=bool:?}, argb8888: {=bool:?} }}" , self . ckta () , self . cfbda () , self . cfbpa () , self . apa () , self . dcp () , self . wina () , self . cluta () , self . ckra () , self . f21 () , self . f20 () , self . f2p () , self . f21p () , self . f2c () , self . f21c () , self . f2pc () , self . f21pc () , self . f11 () , self . f10 () , self . f1p () , self . f11p () , self . f1c () , self . f11c () , self . f1pc () , self . f11pc () , self . ff () , self . rgb888 () , self . bgr565 () , self . rgb565 () , self . bgra8888 () , self . rgba8888 () , self . abgr8888 () , self . argb8888 ())
        }
    }
    #[doc = "Layerx Configuration 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1r(pub u32);
    impl C1r {
        #[doc = "Ycbcr 422 Interleaved Ability For That Layer."]
        #[inline(always)]
        pub const fn yia(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ycbcr 422 Interleaved Ability For That Layer."]
        #[inline(always)]
        pub fn set_yia(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Ycbcr 420 Semi-Planar Ability For That Layer."]
        #[inline(always)]
        pub const fn yspa(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Ycbcr 420 Semi-Planar Ability For That Layer."]
        #[inline(always)]
        pub fn set_yspa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Ycbcr 420 Full-Planar Ability For That Layer."]
        #[inline(always)]
        pub const fn yfpa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Ycbcr 420 Full-Planar Ability For That Layer."]
        #[inline(always)]
        pub fn set_yfpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Scaling Ability For That Layer."]
        #[inline(always)]
        pub const fn sca(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Scaling Ability For That Layer."]
        #[inline(always)]
        pub fn set_sca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C1r {
        #[inline(always)]
        fn default() -> C1r {
            C1r(0)
        }
    }
    impl core::fmt::Debug for C1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1r")
                .field("yia", &self.yia())
                .field("yspa", &self.yspa())
                .field("yfpa", &self.yfpa())
                .field("sca", &self.sca())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "C1r {{ yia: {=bool:?}, yspa: {=bool:?}, yfpa: {=bool:?}, sca: {=bool:?} }}",
                self.yia(),
                self.yspa(),
                self.yfpa(),
                self.sca()
            )
        }
    }
    #[doc = "Layerx Constant Alpha Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cacr(pub u32);
    impl Cacr {
        #[doc = "Constant Alpha."]
        #[inline(always)]
        pub const fn consta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Constant Alpha."]
        #[inline(always)]
        pub fn set_consta(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cacr {
        #[inline(always)]
        fn default() -> Cacr {
            Cacr(0)
        }
    }
    impl core::fmt::Debug for Cacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cacr").field("consta", &self.consta()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cacr {{ consta: {=u8:?} }}", self.consta())
        }
    }
    #[doc = "Ltdc Computed Crc Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccrcr(pub u32);
    impl Ccrcr {
        #[doc = "Computed Crc Of Frame."]
        #[inline(always)]
        pub const fn ccrc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Computed Crc Of Frame."]
        #[inline(always)]
        pub fn set_ccrc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccrcr {
        #[inline(always)]
        fn default() -> Ccrcr {
            Ccrcr(0)
        }
    }
    impl core::fmt::Debug for Ccrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccrcr").field("ccrc", &self.ccrc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccrcr {{ ccrc: {=u16:?} }}", self.ccrc())
        }
    }
    #[doc = "Ltdc Current Display Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdsr(pub u32);
    impl Cdsr {
        #[doc = "Vertical Data Enable Display Status."]
        #[inline(always)]
        pub const fn vdes(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Data Enable Display Status."]
        #[inline(always)]
        pub fn set_vdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal Data Enable Display Status."]
        #[inline(always)]
        pub const fn hdes(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Data Enable Display Status."]
        #[inline(always)]
        pub fn set_hdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Vertical Synchronization Display Status."]
        #[inline(always)]
        pub const fn vsyncs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Synchronization Display Status."]
        #[inline(always)]
        pub fn set_vsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Horizontal Synchronization Display Status."]
        #[inline(always)]
        pub const fn hsyncs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Synchronization Display Status."]
        #[inline(always)]
        pub fn set_hsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cdsr {
        #[inline(always)]
        fn default() -> Cdsr {
            Cdsr(0)
        }
    }
    impl core::fmt::Debug for Cdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdsr")
                .field("vdes", &self.vdes())
                .field("hdes", &self.hdes())
                .field("vsyncs", &self.vsyncs())
                .field("hsyncs", &self.hsyncs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cdsr {{ vdes: {=bool:?}, hdes: {=bool:?}, vsyncs: {=bool:?}, hsyncs: {=bool:?} }}",
                self.vdes(),
                self.hdes(),
                self.vsyncs(),
                self.hsyncs()
            )
        }
    }
    #[doc = "Layerx Color Frame Buffer Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfbar(pub u32);
    impl Cfbar {
        #[doc = "Color Frame Buffer Start Address."]
        #[inline(always)]
        pub const fn cfbadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Color Frame Buffer Start Address."]
        #[inline(always)]
        pub fn set_cfbadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cfbar {
        #[inline(always)]
        fn default() -> Cfbar {
            Cfbar(0)
        }
    }
    impl core::fmt::Debug for Cfbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfbar").field("cfbadd", &self.cfbadd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfbar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfbar {{ cfbadd: {=u32:?} }}", self.cfbadd())
        }
    }
    #[doc = "Layerx Color Frame Buffer Line Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfblnr(pub u32);
    impl Cfblnr {
        #[doc = "Frame Buffer Line Number."]
        #[inline(always)]
        pub const fn cfblnbr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Frame Buffer Line Number."]
        #[inline(always)]
        pub fn set_cfblnbr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cfblnr {
        #[inline(always)]
        fn default() -> Cfblnr {
            Cfblnr(0)
        }
    }
    impl core::fmt::Debug for Cfblnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfblnr").field("cfblnbr", &self.cfblnbr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfblnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfblnr {{ cfblnbr: {=u16:?} }}", self.cfblnbr())
        }
    }
    #[doc = "Layerx Color Frame Buffer Length Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfblr(pub u32);
    impl Cfblr {
        #[doc = "Color Frame Buffer Line Length."]
        #[inline(always)]
        pub const fn cfbll(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Color Frame Buffer Line Length."]
        #[inline(always)]
        pub fn set_cfbll(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Color Frame Buffer Pitch In Bytes."]
        #[inline(always)]
        pub const fn cfbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Color Frame Buffer Pitch In Bytes."]
        #[inline(always)]
        pub fn set_cfbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cfblr {
        #[inline(always)]
        fn default() -> Cfblr {
            Cfblr(0)
        }
    }
    impl core::fmt::Debug for Cfblr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfblr")
                .field("cfbll", &self.cfbll())
                .field("cfbp", &self.cfbp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfblr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfblr {{ cfbll: {=u16:?}, cfbp: {=u16:?} }}",
                self.cfbll(),
                self.cfbp()
            )
        }
    }
    #[doc = "Layerx Color Keying Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckcr(pub u32);
    impl Ckcr {
        #[doc = "Color Key Blue Value."]
        #[inline(always)]
        pub const fn ckblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Blue Value."]
        #[inline(always)]
        pub fn set_ckblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Color Key Green Value."]
        #[inline(always)]
        pub const fn ckgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Green Value."]
        #[inline(always)]
        pub fn set_ckgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Color Key Red Value."]
        #[inline(always)]
        pub const fn ckred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Red Value."]
        #[inline(always)]
        pub fn set_ckred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ckcr {
        #[inline(always)]
        fn default() -> Ckcr {
            Ckcr(0)
        }
    }
    impl core::fmt::Debug for Ckcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckcr")
                .field("ckblue", &self.ckblue())
                .field("ckgreen", &self.ckgreen())
                .field("ckred", &self.ckred())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ckcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ckcr {{ ckblue: {=u8:?}, ckgreen: {=u8:?}, ckred: {=u8:?} }}",
                self.ckblue(),
                self.ckgreen(),
                self.ckred()
            )
        }
    }
    #[doc = "Layerx Clut Write Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clutwr(pub u32);
    impl Clutwr {
        #[doc = "Blue Value."]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue Value."]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green Value."]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green Value."]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red Value."]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red Value."]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Clut Address."]
        #[inline(always)]
        pub const fn clutadd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Clut Address."]
        #[inline(always)]
        pub fn set_clutadd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Clutwr {
        #[inline(always)]
        fn default() -> Clutwr {
            Clutwr(0)
        }
    }
    impl core::fmt::Debug for Clutwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clutwr")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .field("clutadd", &self.clutadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clutwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clutwr {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?}, clutadd: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red(),
                self.clutadd()
            )
        }
    }
    #[doc = "Ltdc Current Position Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cpsr(pub u32);
    impl Cpsr {
        #[doc = "Current Y Position."]
        #[inline(always)]
        pub const fn cypos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Current Y Position."]
        #[inline(always)]
        pub fn set_cypos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Current X Position."]
        #[inline(always)]
        pub const fn cxpos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Current X Position."]
        #[inline(always)]
        pub fn set_cxpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cpsr {
        #[inline(always)]
        fn default() -> Cpsr {
            Cpsr(0)
        }
    }
    impl core::fmt::Debug for Cpsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cpsr")
                .field("cypos", &self.cypos())
                .field("cxpos", &self.cxpos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cpsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cpsr {{ cypos: {=u16:?}, cxpos: {=u16:?} }}",
                self.cypos(),
                self.cxpos()
            )
        }
    }
    #[doc = "Layerx Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Layer Enable."]
        #[inline(always)]
        pub const fn len(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer Enable."]
        #[inline(always)]
        pub fn set_len(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color Keying Enable."]
        #[inline(always)]
        pub const fn cken(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Color Keying Enable."]
        #[inline(always)]
        pub fn set_cken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Color Look-Up Table Enable."]
        #[inline(always)]
        pub const fn cluten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Color Look-Up Table Enable."]
        #[inline(always)]
        pub fn set_cluten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Horizontal Mirroring Enable."]
        #[inline(always)]
        pub const fn hmen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Mirroring Enable."]
        #[inline(always)]
        pub fn set_hmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Default Color Blending Enable."]
        #[inline(always)]
        pub const fn dcben(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Default Color Blending Enable."]
        #[inline(always)]
        pub fn set_dcben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("len", &self.len())
                .field("cken", &self.cken())
                .field("cluten", &self.cluten())
                .field("hmen", &self.hmen())
                .field("dcben", &self.dcben())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ len: {=bool:?}, cken: {=bool:?}, cluten: {=bool:?}, hmen: {=bool:?}, dcben: {=bool:?} }}",
                self.len(),
                self.cken(),
                self.cluten(),
                self.hmen(),
                self.dcben()
            )
        }
    }
    #[doc = "Layerx Conversion Ycbcr Rgb 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cyr0r(pub u32);
    impl Cyr0r {
        #[doc = "Cr-To-Red Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals."]
        #[inline(always)]
        pub const fn cr2r(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Cr-To-Red Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals."]
        #[inline(always)]
        pub fn set_cr2r(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Cb-To-Blue Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals."]
        #[inline(always)]
        pub const fn cb2b(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Cb-To-Blue Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals."]
        #[inline(always)]
        pub fn set_cb2b(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Cyr0r {
        #[inline(always)]
        fn default() -> Cyr0r {
            Cyr0r(0)
        }
    }
    impl core::fmt::Debug for Cyr0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cyr0r")
                .field("cr2r", &self.cr2r())
                .field("cb2b", &self.cb2b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cyr0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cyr0r {{ cr2r: {=u16:?}, cb2b: {=u16:?} }}",
                self.cr2r(),
                self.cb2b()
            )
        }
    }
    #[doc = "Layerx Conversion Ycbcr Rgb 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cyr1r(pub u32);
    impl Cyr1r {
        #[doc = "Cr-To-Green Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals."]
        #[inline(always)]
        pub const fn cr2g(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Cr-To-Green Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals."]
        #[inline(always)]
        pub fn set_cr2g(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Cb-To-Green Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals."]
        #[inline(always)]
        pub const fn cb2g(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Cb-To-Green Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals."]
        #[inline(always)]
        pub fn set_cb2g(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Cyr1r {
        #[inline(always)]
        fn default() -> Cyr1r {
            Cyr1r(0)
        }
    }
    impl core::fmt::Debug for Cyr1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cyr1r")
                .field("cr2g", &self.cr2g())
                .field("cb2g", &self.cb2g())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cyr1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cyr1r {{ cr2g: {=u16:?}, cb2g: {=u16:?} }}",
                self.cr2g(),
                self.cb2g()
            )
        }
    }
    #[doc = "Layerx Default Color Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dccr(pub u32);
    impl Dccr {
        #[doc = "Default Color Blue."]
        #[inline(always)]
        pub const fn dcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Blue."]
        #[inline(always)]
        pub fn set_dcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Default Color Green."]
        #[inline(always)]
        pub const fn dcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Green."]
        #[inline(always)]
        pub fn set_dcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Default Color Red."]
        #[inline(always)]
        pub const fn dcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Red."]
        #[inline(always)]
        pub fn set_dcred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Default Color Alpha."]
        #[inline(always)]
        pub const fn dcalpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Alpha."]
        #[inline(always)]
        pub fn set_dcalpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dccr {
        #[inline(always)]
        fn default() -> Dccr {
            Dccr(0)
        }
    }
    impl core::fmt::Debug for Dccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dccr")
                .field("dcblue", &self.dcblue())
                .field("dcgreen", &self.dcgreen())
                .field("dcred", &self.dcred())
                .field("dcalpha", &self.dcalpha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dccr {{ dcblue: {=u8:?}, dcgreen: {=u8:?}, dcred: {=u8:?}, dcalpha: {=u8:?} }}",
                self.dcblue(),
                self.dcgreen(),
                self.dcred(),
                self.dcalpha()
            )
        }
    }
    #[doc = "Ltdc Expected Crc Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecrcr(pub u32);
    impl Ecrcr {
        #[doc = "Expected Crc Of Frame."]
        #[inline(always)]
        pub const fn ecrc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Expected Crc Of Frame."]
        #[inline(always)]
        pub fn set_ecrc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ecrcr {
        #[inline(always)]
        fn default() -> Ecrcr {
            Ecrcr(0)
        }
    }
    impl core::fmt::Debug for Ecrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecrcr").field("ecrc", &self.ecrc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ecrcr {{ ecrc: {=u16:?} }}", self.ecrc())
        }
    }
    #[doc = "Ltdc External Display Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Edcr(pub u32);
    impl Edcr {
        #[doc = "Output Conversion To Ycbcr 422 Enable."]
        #[inline(always)]
        pub const fn ocyen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Output Conversion To Ycbcr 422 Enable."]
        #[inline(always)]
        pub fn set_ocyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Output Conversion To Ycbcr 422."]
        #[inline(always)]
        pub const fn ocysel(&self) -> super::vals::Ocysel {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Ocysel::from_bits(val as u8)
        }
        #[doc = "Output Conversion To Ycbcr 422."]
        #[inline(always)]
        pub fn set_ocysel(&mut self, val: super::vals::Ocysel) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "Output Conversion To Ycbcr 422."]
        #[inline(always)]
        pub const fn ocyco(&self) -> super::vals::Ocyco {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::Ocyco::from_bits(val as u8)
        }
        #[doc = "Output Conversion To Ycbcr 422."]
        #[inline(always)]
        pub fn set_ocyco(&mut self, val: super::vals::Ocyco) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Edcr {
        #[inline(always)]
        fn default() -> Edcr {
            Edcr(0)
        }
    }
    impl core::fmt::Debug for Edcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Edcr")
                .field("ocyen", &self.ocyen())
                .field("ocysel", &self.ocysel())
                .field("ocyco", &self.ocyco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Edcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Edcr {{ ocyen: {=bool:?}, ocysel: {:?}, ocyco: {:?} }}",
                self.ocyen(),
                self.ocysel(),
                self.ocyco()
            )
        }
    }
    #[doc = "Layerx Flexible Pixel Format 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpf0r(pub u32);
    impl Fpf0r {
        #[doc = "Location Of The Alpha Component Inside The Pixel Memory Word (In Bits)."]
        #[inline(always)]
        pub const fn apos(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Location Of The Alpha Component Inside The Pixel Memory Word (In Bits)."]
        #[inline(always)]
        pub fn set_apos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Width Of The Alpha Component (In Bits)."]
        #[inline(always)]
        pub const fn alen(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "Width Of The Alpha Component (In Bits)."]
        #[inline(always)]
        pub fn set_alen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Location Of The Red Component Inside The Pixel Memory Word (In Bits)."]
        #[inline(always)]
        pub const fn rpos(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x1f;
            val as u8
        }
        #[doc = "Location Of The Red Component Inside The Pixel Memory Word (In Bits)."]
        #[inline(always)]
        pub fn set_rpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
        }
        #[doc = "Width Of The Red Component (In Bits)."]
        #[inline(always)]
        pub const fn rlen(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[doc = "Width Of The Red Component (In Bits)."]
        #[inline(always)]
        pub fn set_rlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
    }
    impl Default for Fpf0r {
        #[inline(always)]
        fn default() -> Fpf0r {
            Fpf0r(0)
        }
    }
    impl core::fmt::Debug for Fpf0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpf0r")
                .field("apos", &self.apos())
                .field("alen", &self.alen())
                .field("rpos", &self.rpos())
                .field("rlen", &self.rlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fpf0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fpf0r {{ apos: {=u8:?}, alen: {=u8:?}, rpos: {=u8:?}, rlen: {=u8:?} }}",
                self.apos(),
                self.alen(),
                self.rpos(),
                self.rlen()
            )
        }
    }
    #[doc = "Layerx Flexible Pixel Format 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpf1r(pub u32);
    impl Fpf1r {
        #[doc = "Location Of The Green Component Inside The Pixel Memory Word (In Bits)."]
        #[inline(always)]
        pub const fn gpos(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Location Of The Green Component Inside The Pixel Memory Word (In Bits)."]
        #[inline(always)]
        pub fn set_gpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Width Of The Green Component (In Bits)."]
        #[inline(always)]
        pub const fn glen(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "Width Of The Green Component (In Bits)."]
        #[inline(always)]
        pub fn set_glen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Location Of The Blue Component Inside The Pixel Memory Word (In Bits)."]
        #[inline(always)]
        pub const fn bpos(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x1f;
            val as u8
        }
        #[doc = "Location Of The Blue Component Inside The Pixel Memory Word (In Bits)."]
        #[inline(always)]
        pub fn set_bpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
        }
        #[doc = "Width Of The Blue Component (In Bits)."]
        #[inline(always)]
        pub const fn blen(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[doc = "Width Of The Blue Component (In Bits)."]
        #[inline(always)]
        pub fn set_blen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
        #[doc = "Pixel Size (In Bytes)."]
        #[inline(always)]
        pub const fn psize(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "Pixel Size (In Bytes)."]
        #[inline(always)]
        pub fn set_psize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
    }
    impl Default for Fpf1r {
        #[inline(always)]
        fn default() -> Fpf1r {
            Fpf1r(0)
        }
    }
    impl core::fmt::Debug for Fpf1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpf1r")
                .field("gpos", &self.gpos())
                .field("glen", &self.glen())
                .field("bpos", &self.bpos())
                .field("blen", &self.blen())
                .field("psize", &self.psize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fpf1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fpf1r {{ gpos: {=u8:?}, glen: {=u8:?}, bpos: {=u8:?}, blen: {=u8:?}, psize: {=u8:?} }}",
                self.gpos(),
                self.glen(),
                self.bpos(),
                self.blen(),
                self.psize()
            )
        }
    }
    #[doc = "Ltdc Fifo Underrun Threshold Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Futr(pub u32);
    impl Futr {
        #[doc = "Threshold To Trigger A Fifo Underrun Interrupt (Per Fifo Word, 64 Bits)."]
        #[inline(always)]
        pub const fn thre(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold To Trigger A Fifo Underrun Interrupt (Per Fifo Word, 64 Bits)."]
        #[inline(always)]
        pub fn set_thre(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Futr {
        #[inline(always)]
        fn default() -> Futr {
            Futr(0)
        }
    }
    impl core::fmt::Debug for Futr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Futr").field("thre", &self.thre()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Futr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Futr {{ thre: {=u16:?} }}", self.thre())
        }
    }
    #[doc = "Ltdc Global Configuration 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gc1r(pub u32);
    impl Gc1r {
        #[doc = "Width Of Blue Channel Output."]
        #[inline(always)]
        pub const fn wbch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Width Of Blue Channel Output."]
        #[inline(always)]
        pub fn set_wbch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Width Of Green Channel Output."]
        #[inline(always)]
        pub const fn wgch(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Width Of Green Channel Output."]
        #[inline(always)]
        pub fn set_wgch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Width Of Red Channel Output."]
        #[inline(always)]
        pub const fn wrch(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Width Of Red Channel Output."]
        #[inline(always)]
        pub fn set_wrch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Precise Blending Ability."]
        #[inline(always)]
        pub const fn prba(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Precise Blending Ability."]
        #[inline(always)]
        pub fn set_prba(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Dithering Technique Implemented."]
        #[inline(always)]
        pub const fn dt(&self) -> super::vals::Dt {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Dt::from_bits(val as u8)
        }
        #[doc = "Dithering Technique Implemented."]
        #[inline(always)]
        pub fn set_dt(&mut self, val: super::vals::Dt) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Gamma Correction Technique Implemented."]
        #[inline(always)]
        pub const fn gct(&self) -> super::vals::Gct {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::Gct::from_bits(val as u8)
        }
        #[doc = "Gamma Correction Technique Implemented."]
        #[inline(always)]
        pub fn set_gct(&mut self, val: super::vals::Gct) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
        #[doc = "Shadow Registers Ability."]
        #[inline(always)]
        pub const fn shra(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow Registers Ability."]
        #[inline(always)]
        pub fn set_shra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Background Color Programmability (Unique Color Blended As Background)."]
        #[inline(always)]
        pub const fn bcp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Background Color Programmability (Unique Color Blended As Background)."]
        #[inline(always)]
        pub fn set_bcp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Background Blending Ability."]
        #[inline(always)]
        pub const fn bba(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Background Blending Ability."]
        #[inline(always)]
        pub fn set_bba(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Line-Irq: Line Position Programmability."]
        #[inline(always)]
        pub const fn lnip(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Line-Irq: Line Position Programmability."]
        #[inline(always)]
        pub fn set_lnip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Timing Programmability."]
        #[inline(always)]
        pub const fn tp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Timing Programmability."]
        #[inline(always)]
        pub fn set_tp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Sync Polarity Programmability."]
        #[inline(always)]
        pub const fn spp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Polarity Programmability."]
        #[inline(always)]
        pub fn set_spp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Dither Width Programmability."]
        #[inline(always)]
        pub const fn dwp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Dither Width Programmability."]
        #[inline(always)]
        pub fn set_dwp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Status Register Ability."]
        #[inline(always)]
        pub const fn stra(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Status Register Ability."]
        #[inline(always)]
        pub fn set_stra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Configuration Reading Mode Ability."]
        #[inline(always)]
        pub const fn crma(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Reading Mode Ability."]
        #[inline(always)]
        pub fn set_crma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Blind Mode Ability."]
        #[inline(always)]
        pub const fn bma(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Blind Mode Ability."]
        #[inline(always)]
        pub fn set_bma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gc1r {
        #[inline(always)]
        fn default() -> Gc1r {
            Gc1r(0)
        }
    }
    impl core::fmt::Debug for Gc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gc1r")
                .field("wbch", &self.wbch())
                .field("wgch", &self.wgch())
                .field("wrch", &self.wrch())
                .field("prba", &self.prba())
                .field("dt", &self.dt())
                .field("gct", &self.gct())
                .field("shra", &self.shra())
                .field("bcp", &self.bcp())
                .field("bba", &self.bba())
                .field("lnip", &self.lnip())
                .field("tp", &self.tp())
                .field("spp", &self.spp())
                .field("dwp", &self.dwp())
                .field("stra", &self.stra())
                .field("crma", &self.crma())
                .field("bma", &self.bma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gc1r {{ wbch: {=u8:?}, wgch: {=u8:?}, wrch: {=u8:?}, prba: {=bool:?}, dt: {:?}, gct: {:?}, shra: {=bool:?}, bcp: {=bool:?}, bba: {=bool:?}, lnip: {=bool:?}, tp: {=bool:?}, spp: {=bool:?}, dwp: {=bool:?}, stra: {=bool:?}, crma: {=bool:?}, bma: {=bool:?} }}" , self . wbch () , self . wgch () , self . wrch () , self . prba () , self . dt () , self . gct () , self . shra () , self . bcp () , self . bba () , self . lnip () , self . tp () , self . spp () , self . dwp () , self . stra () , self . crma () , self . bma ())
        }
    }
    #[doc = "Ltdc Global Configuration 2 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gc2r(pub u32);
    impl Gc2r {
        #[doc = "Background Layer Ability (Pixels Of Background Layer Are Read From Memory)."]
        #[inline(always)]
        pub const fn bla(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Background Layer Ability (Pixels Of Background Layer Are Read From Memory)."]
        #[inline(always)]
        pub fn set_bla(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Slave Timings Synchronization Ability."]
        #[inline(always)]
        pub const fn stsa(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Slave Timings Synchronization Ability."]
        #[inline(always)]
        pub fn set_stsa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Dual-View Ability."]
        #[inline(always)]
        pub const fn dva(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Dual-View Ability."]
        #[inline(always)]
        pub fn set_dva(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secondary Rgb Output Port Ability."]
        #[inline(always)]
        pub const fn dpa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secondary Rgb Output Port Ability."]
        #[inline(always)]
        pub fn set_dpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Bus Width (Log2 Of Number Of Bytes)."]
        #[inline(always)]
        pub const fn bw(&self) -> super::vals::Bw {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Bw::from_bits(val as u8)
        }
        #[doc = "Bus Width (Log2 Of Number Of Bytes)."]
        #[inline(always)]
        pub fn set_bw(&mut self, val: super::vals::Bw) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "External Display Control Ability."]
        #[inline(always)]
        pub const fn edca(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "External Display Control Ability."]
        #[inline(always)]
        pub fn set_edca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Output Conversion Ability (Rgb To Ycbcr)."]
        #[inline(always)]
        pub const fn oca(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Output Conversion Ability (Rgb To Ycbcr)."]
        #[inline(always)]
        pub fn set_oca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Axiid Ability."]
        #[inline(always)]
        pub const fn axiida(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Axiid Ability."]
        #[inline(always)]
        pub fn set_axiida(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Rotation Support Ability."]
        #[inline(always)]
        pub const fn rota(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Rotation Support Ability."]
        #[inline(always)]
        pub fn set_rota(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Second Interrupt Set Ability."]
        #[inline(always)]
        pub const fn sisa(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Second Interrupt Set Ability."]
        #[inline(always)]
        pub fn set_sisa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Single Frame Mode Ability."]
        #[inline(always)]
        pub const fn sfa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Single Frame Mode Ability."]
        #[inline(always)]
        pub fn set_sfa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Crc Ability."]
        #[inline(always)]
        pub const fn crca(&self) -> super::vals::Crca {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Crca::from_bits(val as u8)
        }
        #[doc = "Crc Ability."]
        #[inline(always)]
        pub fn set_crca(&mut self, val: super::vals::Crca) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Blending Order Ability."]
        #[inline(always)]
        pub const fn boa(&self) -> super::vals::Boa {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Boa::from_bits(val as u8)
        }
        #[doc = "Blending Order Ability."]
        #[inline(always)]
        pub fn set_boa(&mut self, val: super::vals::Boa) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Gc2r {
        #[inline(always)]
        fn default() -> Gc2r {
            Gc2r(0)
        }
    }
    impl core::fmt::Debug for Gc2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gc2r")
                .field("bla", &self.bla())
                .field("stsa", &self.stsa())
                .field("dva", &self.dva())
                .field("dpa", &self.dpa())
                .field("bw", &self.bw())
                .field("edca", &self.edca())
                .field("oca", &self.oca())
                .field("axiida", &self.axiida())
                .field("rota", &self.rota())
                .field("sisa", &self.sisa())
                .field("sfa", &self.sfa())
                .field("crca", &self.crca())
                .field("boa", &self.boa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gc2r {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gc2r {{ bla: {=bool:?}, stsa: {=bool:?}, dva: {=bool:?}, dpa: {=bool:?}, bw: {:?}, edca: {=bool:?}, oca: {=bool:?}, axiida: {=bool:?}, rota: {=bool:?}, sisa: {=bool:?}, sfa: {=bool:?}, crca: {:?}, boa: {:?} }}" , self . bla () , self . stsa () , self . dva () , self . dpa () , self . bw () , self . edca () , self . oca () , self . axiida () , self . rota () , self . sisa () , self . sfa () , self . crca () , self . boa ())
        }
    }
    #[doc = "Ltdc Gamma Correction Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gccr(pub u32);
    impl Gccr {
        #[doc = "Address Of The R,G,B Table Where The Comp Component Is Written."]
        #[inline(always)]
        pub const fn addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Address Of The R,G,B Table Where The Comp Component Is Written."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Color Component To Be Written, In Either (Or All) The R,G,B Tables."]
        #[inline(always)]
        pub const fn comp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Color Component To Be Written, In Either (Or All) The R,G,B Tables."]
        #[inline(always)]
        pub fn set_comp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Write Trigger To The Blue Table."]
        #[inline(always)]
        pub const fn blueen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Write Trigger To The Blue Table."]
        #[inline(always)]
        pub fn set_blueen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Write Trigger To The Green Table."]
        #[inline(always)]
        pub const fn greenen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Write Trigger To The Green Table."]
        #[inline(always)]
        pub fn set_greenen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Write Trigger To The Red Table."]
        #[inline(always)]
        pub const fn reden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Write Trigger To The Red Table."]
        #[inline(always)]
        pub fn set_reden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Gccr {
        #[inline(always)]
        fn default() -> Gccr {
            Gccr(0)
        }
    }
    impl core::fmt::Debug for Gccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gccr")
                .field("addr", &self.addr())
                .field("comp", &self.comp())
                .field("blueen", &self.blueen())
                .field("greenen", &self.greenen())
                .field("reden", &self.reden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gccr {{ addr: {=u8:?}, comp: {=u8:?}, blueen: {=bool:?}, greenen: {=bool:?}, reden: {=bool:?} }}",
                self.addr(),
                self.comp(),
                self.blueen(),
                self.greenen(),
                self.reden()
            )
        }
    }
    #[doc = "Ltdc Global Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "Ltdc Global Enable."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ltdc Global Enable."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Gamma Correction Enable."]
        #[inline(always)]
        pub const fn gamen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Gamma Correction Enable."]
        #[inline(always)]
        pub fn set_gamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Dither Blue Width."]
        #[inline(always)]
        pub const fn dbw(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Blue Width."]
        #[inline(always)]
        pub fn set_dbw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Dither Green Width."]
        #[inline(always)]
        pub const fn dgw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Green Width."]
        #[inline(always)]
        pub fn set_dgw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Dither Red Width."]
        #[inline(always)]
        pub const fn drw(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Red Width."]
        #[inline(always)]
        pub fn set_drw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Dither Enable."]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Dither Enable."]
        #[inline(always)]
        pub fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Crc Enable."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Crc Enable."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Single-Frame Mode: Mode Enable."]
        #[inline(always)]
        pub const fn sfen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Single-Frame Mode: Mode Enable."]
        #[inline(always)]
        pub fn set_sfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Single-Frame Mode: Software Trigger."]
        #[inline(always)]
        pub const fn sfswtr(&self) -> super::vals::Sfswtr {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Sfswtr::from_bits(val as u8)
        }
        #[doc = "Single-Frame Mode: Software Trigger."]
        #[inline(always)]
        pub fn set_sfswtr(&mut self, val: super::vals::Sfswtr) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Pixel Clock Polarity."]
        #[inline(always)]
        pub const fn pcpol(&self) -> super::vals::Pcpol {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Pcpol::from_bits(val as u8)
        }
        #[doc = "Pixel Clock Polarity."]
        #[inline(always)]
        pub fn set_pcpol(&mut self, val: super::vals::Pcpol) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "Blanking (No Data/Pixel) Polarity."]
        #[inline(always)]
        pub const fn depol(&self) -> super::vals::Depol {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Depol::from_bits(val as u8)
        }
        #[doc = "Blanking (No Data/Pixel) Polarity."]
        #[inline(always)]
        pub fn set_depol(&mut self, val: super::vals::Depol) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Vertical Synchronization Polarity."]
        #[inline(always)]
        pub const fn vspol(&self) -> super::vals::Vspol {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Vspol::from_bits(val as u8)
        }
        #[doc = "Vertical Synchronization Polarity."]
        #[inline(always)]
        pub fn set_vspol(&mut self, val: super::vals::Vspol) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Horizontal Synchronization Polarity."]
        #[inline(always)]
        pub const fn hspol(&self) -> super::vals::Hspol {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Hspol::from_bits(val as u8)
        }
        #[doc = "Horizontal Synchronization Polarity."]
        #[inline(always)]
        pub fn set_hspol(&mut self, val: super::vals::Hspol) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("ltdcen", &self.ltdcen())
                .field("gamen", &self.gamen())
                .field("dbw", &self.dbw())
                .field("dgw", &self.dgw())
                .field("drw", &self.drw())
                .field("den", &self.den())
                .field("crcen", &self.crcen())
                .field("sfen", &self.sfen())
                .field("sfswtr", &self.sfswtr())
                .field("pcpol", &self.pcpol())
                .field("depol", &self.depol())
                .field("vspol", &self.vspol())
                .field("hspol", &self.hspol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gcr {{ ltdcen: {=bool:?}, gamen: {=bool:?}, dbw: {=u8:?}, dgw: {=u8:?}, drw: {=u8:?}, den: {=bool:?}, crcen: {=bool:?}, sfen: {=bool:?}, sfswtr: {:?}, pcpol: {:?}, depol: {:?}, vspol: {:?}, hspol: {:?} }}" , self . ltdcen () , self . gamen () , self . dbw () , self . dgw () , self . drw () , self . den () , self . crcen () , self . sfen () , self . sfswtr () , self . pcpol () , self . depol () , self . vspol () , self . hspol ())
        }
    }
    #[doc = "Ltdc Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Clears The Line Interrupt Flag."]
        #[inline(always)]
        pub const fn clif(&self) -> super::vals::Clif {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Clif::from_bits(val as u8)
        }
        #[doc = "Clears The Line Interrupt Flag."]
        #[inline(always)]
        pub fn set_clif(&mut self, val: super::vals::Clif) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clears The Fifo Underrun Warning Interrupt Flag."]
        #[inline(always)]
        pub const fn cfuwif(&self) -> super::vals::Cfuwif {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cfuwif::from_bits(val as u8)
        }
        #[doc = "Clears The Fifo Underrun Warning Interrupt Flag."]
        #[inline(always)]
        pub fn set_cfuwif(&mut self, val: super::vals::Cfuwif) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clears The Transfer Error Interrupt Flag."]
        #[inline(always)]
        pub const fn cterrif(&self) -> super::vals::Cterrif {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Cterrif::from_bits(val as u8)
        }
        #[doc = "Clears The Transfer Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_cterrif(&mut self, val: super::vals::Cterrif) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clears Register Reload Interrupt Flag."]
        #[inline(always)]
        pub const fn crrif(&self) -> super::vals::Crrif {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Crrif::from_bits(val as u8)
        }
        #[doc = "Clears Register Reload Interrupt Flag."]
        #[inline(always)]
        pub fn set_crrif(&mut self, val: super::vals::Crrif) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Clears The Fifo Underrun Interrupt Flag."]
        #[inline(always)]
        pub const fn cfuif(&self) -> super::vals::Cfuif {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Cfuif::from_bits(val as u8)
        }
        #[doc = "Clears The Fifo Underrun Interrupt Flag."]
        #[inline(always)]
        pub fn set_cfuif(&mut self, val: super::vals::Cfuif) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Clears The Crc Error Interrupt Flag."]
        #[inline(always)]
        pub const fn ccrcif(&self) -> super::vals::Ccrcif {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ccrcif::from_bits(val as u8)
        }
        #[doc = "Clears The Crc Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_ccrcif(&mut self, val: super::vals::Ccrcif) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
                .field("clif", &self.clif())
                .field("cfuwif", &self.cfuwif())
                .field("cterrif", &self.cterrif())
                .field("crrif", &self.crrif())
                .field("cfuif", &self.cfuif())
                .field("ccrcif", &self.ccrcif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ clif: {:?}, cfuwif: {:?}, cterrif: {:?}, crrif: {:?}, cfuif: {:?}, ccrcif: {:?} }}",
                self.clif(),
                self.cfuwif(),
                self.cterrif(),
                self.crrif(),
                self.cfuif(),
                self.ccrcif()
            )
        }
    }
    #[doc = "Ltdc Interrupt Clear Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr2(pub u32);
    impl Icr2 {
        #[doc = "Clears The Line Interrupt Flag."]
        #[inline(always)]
        pub const fn clif(&self) -> super::vals::Clif {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Clif::from_bits(val as u8)
        }
        #[doc = "Clears The Line Interrupt Flag."]
        #[inline(always)]
        pub fn set_clif(&mut self, val: super::vals::Clif) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clears The Fifo Underrun Warning Interrupt Flag."]
        #[inline(always)]
        pub const fn cfuwif(&self) -> super::vals::Cfuwif {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cfuwif::from_bits(val as u8)
        }
        #[doc = "Clears The Fifo Underrun Warning Interrupt Flag."]
        #[inline(always)]
        pub fn set_cfuwif(&mut self, val: super::vals::Cfuwif) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clears The Transfer Error Interrupt Flag."]
        #[inline(always)]
        pub const fn cterrif(&self) -> super::vals::Cterrif {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Cterrif::from_bits(val as u8)
        }
        #[doc = "Clears The Transfer Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_cterrif(&mut self, val: super::vals::Cterrif) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clears Register Reload Interrupt Flag."]
        #[inline(always)]
        pub const fn crrif(&self) -> super::vals::Crrif {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Crrif::from_bits(val as u8)
        }
        #[doc = "Clears Register Reload Interrupt Flag."]
        #[inline(always)]
        pub fn set_crrif(&mut self, val: super::vals::Crrif) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Clears The Fifo Underrun Interrupt Flag."]
        #[inline(always)]
        pub const fn cfuif(&self) -> super::vals::Cfuif {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Cfuif::from_bits(val as u8)
        }
        #[doc = "Clears The Fifo Underrun Interrupt Flag."]
        #[inline(always)]
        pub fn set_cfuif(&mut self, val: super::vals::Cfuif) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Clears The Crc Error Interrupt Flag."]
        #[inline(always)]
        pub const fn ccrcif(&self) -> super::vals::Ccrcif {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ccrcif::from_bits(val as u8)
        }
        #[doc = "Clears The Crc Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_ccrcif(&mut self, val: super::vals::Ccrcif) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Icr2 {
        #[inline(always)]
        fn default() -> Icr2 {
            Icr2(0)
        }
    }
    impl core::fmt::Debug for Icr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr2")
                .field("clif", &self.clif())
                .field("cfuwif", &self.cfuwif())
                .field("cterrif", &self.cterrif())
                .field("crrif", &self.crrif())
                .field("cfuif", &self.cfuif())
                .field("ccrcif", &self.ccrcif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr2 {{ clif: {:?}, cfuwif: {:?}, cterrif: {:?}, crrif: {:?}, cfuif: {:?}, ccrcif: {:?} }}",
                self.clif(),
                self.cfuwif(),
                self.cterrif(),
                self.crrif(),
                self.cfuif(),
                self.ccrcif()
            )
        }
    }
    #[doc = "Ltdc Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Line Interrupt Enable."]
        #[inline(always)]
        pub const fn lie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt Enable."]
        #[inline(always)]
        pub fn set_lie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fifo Underrun Warning Interrupt Enable."]
        #[inline(always)]
        pub const fn fuwie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Underrun Warning Interrupt Enable."]
        #[inline(always)]
        pub fn set_fuwie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer Error Interrupt Enable."]
        #[inline(always)]
        pub const fn terrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_terrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Register Reload Interrupt Enable."]
        #[inline(always)]
        pub const fn rrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload Interrupt Enable."]
        #[inline(always)]
        pub fn set_rrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fifo Underrun Interrupt Enable."]
        #[inline(always)]
        pub const fn fuie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Underrun Interrupt Enable."]
        #[inline(always)]
        pub fn set_fuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Crc Error Interrupt Enable."]
        #[inline(always)]
        pub const fn crcie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Crc Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_crcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("lie", &self.lie())
                .field("fuwie", &self.fuwie())
                .field("terrie", &self.terrie())
                .field("rrie", &self.rrie())
                .field("fuie", &self.fuie())
                .field("crcie", &self.crcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ lie: {=bool:?}, fuwie: {=bool:?}, terrie: {=bool:?}, rrie: {=bool:?}, fuie: {=bool:?}, crcie: {=bool:?} }}" , self . lie () , self . fuwie () , self . terrie () , self . rrie () , self . fuie () , self . crcie ())
        }
    }
    #[doc = "Ltdc Interrupt Enable Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier2(pub u32);
    impl Ier2 {
        #[doc = "Line Interrupt Enable."]
        #[inline(always)]
        pub const fn lie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt Enable."]
        #[inline(always)]
        pub fn set_lie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fifo Underrun Warning Interrupt Enable."]
        #[inline(always)]
        pub const fn fuwie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Underrun Warning Interrupt Enable."]
        #[inline(always)]
        pub fn set_fuwie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer Error Interrupt Enable."]
        #[inline(always)]
        pub const fn terrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_terrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Register Reload Interrupt Enable."]
        #[inline(always)]
        pub const fn rrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload Interrupt Enable."]
        #[inline(always)]
        pub fn set_rrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fifo Underrun Interrupt Enable."]
        #[inline(always)]
        pub const fn fuie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Underrun Interrupt Enable."]
        #[inline(always)]
        pub fn set_fuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Crc Error Interrupt Enable."]
        #[inline(always)]
        pub const fn crcie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Crc Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_crcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ier2 {
        #[inline(always)]
        fn default() -> Ier2 {
            Ier2(0)
        }
    }
    impl core::fmt::Debug for Ier2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier2")
                .field("lie", &self.lie())
                .field("fuwie", &self.fuwie())
                .field("terrie", &self.terrie())
                .field("rrie", &self.rrie())
                .field("fuie", &self.fuie())
                .field("crcie", &self.crcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier2 {{ lie: {=bool:?}, fuwie: {=bool:?}, terrie: {=bool:?}, rrie: {=bool:?}, fuie: {=bool:?}, crcie: {=bool:?} }}" , self . lie () , self . fuwie () , self . terrie () , self . rrie () , self . fuie () , self . crcie ())
        }
    }
    #[doc = "Ltdc Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Line Interrupt Flag."]
        #[inline(always)]
        pub const fn lif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt Flag."]
        #[inline(always)]
        pub fn set_lif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fifo Underrun Warning Interrupt Flag."]
        #[inline(always)]
        pub const fn fuwif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Underrun Warning Interrupt Flag."]
        #[inline(always)]
        pub fn set_fuwif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer Error Interrupt Flag."]
        #[inline(always)]
        pub const fn terrif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_terrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Register Reload Interrupt Flag."]
        #[inline(always)]
        pub const fn rrif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload Interrupt Flag."]
        #[inline(always)]
        pub fn set_rrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fifo Underrun Interrupt Flag."]
        #[inline(always)]
        pub const fn fuif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Underrun Interrupt Flag."]
        #[inline(always)]
        pub fn set_fuif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Crc Error Interrupt Flag."]
        #[inline(always)]
        pub const fn crcif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Crc Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_crcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("lif", &self.lif())
                .field("fuwif", &self.fuwif())
                .field("terrif", &self.terrif())
                .field("rrif", &self.rrif())
                .field("fuif", &self.fuif())
                .field("crcif", &self.crcif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ lif: {=bool:?}, fuwif: {=bool:?}, terrif: {=bool:?}, rrif: {=bool:?}, fuif: {=bool:?}, crcif: {=bool:?} }}" , self . lif () , self . fuwif () , self . terrif () , self . rrif () , self . fuif () , self . crcif ())
        }
    }
    #[doc = "Ltdc Interrupt Status Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr2(pub u32);
    impl Isr2 {
        #[doc = "Line Interrupt Flag."]
        #[inline(always)]
        pub const fn lif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt Flag."]
        #[inline(always)]
        pub fn set_lif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fifo Underrun Warning Interrupt Flag."]
        #[inline(always)]
        pub const fn fuwif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Underrun Warning Interrupt Flag."]
        #[inline(always)]
        pub fn set_fuwif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer Error Interrupt Flag."]
        #[inline(always)]
        pub const fn terrif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_terrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Register Reload Interrupt Flag."]
        #[inline(always)]
        pub const fn rrif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload Interrupt Flag."]
        #[inline(always)]
        pub fn set_rrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fifo Underrun Interrupt Flag."]
        #[inline(always)]
        pub const fn fuif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Underrun Interrupt Flag."]
        #[inline(always)]
        pub fn set_fuif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Crc Error Interrupt Flag."]
        #[inline(always)]
        pub const fn crcif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Crc Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_crcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Isr2 {
        #[inline(always)]
        fn default() -> Isr2 {
            Isr2(0)
        }
    }
    impl core::fmt::Debug for Isr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr2")
                .field("lif", &self.lif())
                .field("fuwif", &self.fuwif())
                .field("terrif", &self.terrif())
                .field("rrif", &self.rrif())
                .field("fuif", &self.fuif())
                .field("crcif", &self.crcif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr2 {{ lif: {=bool:?}, fuwif: {=bool:?}, terrif: {=bool:?}, rrif: {=bool:?}, fuif: {=bool:?}, crcif: {=bool:?} }}" , self . lif () , self . fuwif () , self . terrif () , self . rrif () , self . fuif () , self . crcif ())
        }
    }
    #[doc = "Line Interrupt Position Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lipcr(pub u32);
    impl Lipcr {
        #[doc = "Line Interrupt Position."]
        #[inline(always)]
        pub const fn lipos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Line Interrupt Position."]
        #[inline(always)]
        pub fn set_lipos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lipcr {
        #[inline(always)]
        fn default() -> Lipcr {
            Lipcr(0)
        }
    }
    impl core::fmt::Debug for Lipcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lipcr").field("lipos", &self.lipos()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lipcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lipcr {{ lipos: {=u16:?} }}", self.lipos())
        }
    }
    #[doc = "Line Interrupt Position Configuration Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lipcr2(pub u32);
    impl Lipcr2 {
        #[doc = "Line Interrupt Position."]
        #[inline(always)]
        pub const fn lipos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Line Interrupt Position."]
        #[inline(always)]
        pub fn set_lipos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lipcr2 {
        #[inline(always)]
        fn default() -> Lipcr2 {
            Lipcr2(0)
        }
    }
    impl core::fmt::Debug for Lipcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lipcr2").field("lipos", &self.lipos()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lipcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lipcr2 {{ lipos: {=u16:?} }}", self.lipos())
        }
    }
    #[doc = "Layerx Planar Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Ycbcr-To-Rgb Conversion Enable."]
        #[inline(always)]
        pub const fn ycen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Ycbcr-To-Rgb Conversion Enable."]
        #[inline(always)]
        pub fn set_ycen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Ycbcr Conversion Mode."]
        #[inline(always)]
        pub const fn ycm(&self) -> super::vals::Ycm {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ycm::from_bits(val as u8)
        }
        #[doc = "Ycbcr Conversion Mode."]
        #[inline(always)]
        pub fn set_ycm(&mut self, val: super::vals::Ycm) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Y Component First."]
        #[inline(always)]
        pub const fn yf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Y Component First."]
        #[inline(always)]
        pub fn set_yf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Cb Component First."]
        #[inline(always)]
        pub const fn cbf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Cb Component First."]
        #[inline(always)]
        pub fn set_cbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Odd Pixel First."]
        #[inline(always)]
        pub const fn of(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Odd Pixel First."]
        #[inline(always)]
        pub fn set_of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Y Rescale Enable For The Color Dynamic Range."]
        #[inline(always)]
        pub const fn yren(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Y Rescale Enable For The Color Dynamic Range."]
        #[inline(always)]
        pub fn set_yren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("ycen", &self.ycen())
                .field("ycm", &self.ycm())
                .field("yf", &self.yf())
                .field("cbf", &self.cbf())
                .field("of", &self.of())
                .field("yren", &self.yren())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcr {{ ycen: {=bool:?}, ycm: {:?}, yf: {=bool:?}, cbf: {=bool:?}, of: {=bool:?}, yren: {=bool:?} }}",
                self.ycen(),
                self.ycm(),
                self.yf(),
                self.cbf(),
                self.of(),
                self.yren()
            )
        }
    }
    #[doc = "Layerx Pixel Format Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfcr(pub u32);
    impl Pfcr {
        #[doc = "Pixel Format."]
        #[inline(always)]
        pub const fn pf(&self) -> super::vals::Pf {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Pf::from_bits(val as u8)
        }
        #[doc = "Pixel Format."]
        #[inline(always)]
        pub fn set_pf(&mut self, val: super::vals::Pf) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Pfcr {
        #[inline(always)]
        fn default() -> Pfcr {
            Pfcr(0)
        }
    }
    impl core::fmt::Debug for Pfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfcr").field("pf", &self.pf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pfcr {{ pf: {:?} }}", self.pf())
        }
    }
    #[doc = "Layerx Reload Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr(pub u32);
    impl Rcr {
        #[doc = "Immediate Reload Trigger."]
        #[inline(always)]
        pub const fn imr(&self) -> super::vals::Imr {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Imr::from_bits(val as u8)
        }
        #[doc = "Immediate Reload Trigger."]
        #[inline(always)]
        pub fn set_imr(&mut self, val: super::vals::Imr) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Vertical Blanking Reload Request."]
        #[inline(always)]
        pub const fn vbr(&self) -> super::vals::Vbr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Vbr::from_bits(val as u8)
        }
        #[doc = "Vertical Blanking Reload Request."]
        #[inline(always)]
        pub fn set_vbr(&mut self, val: super::vals::Vbr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Shadow Reload Control, Global (Centralized) Reload Masked."]
        #[inline(always)]
        pub const fn grmsk(&self) -> super::vals::Grmsk {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Grmsk::from_bits(val as u8)
        }
        #[doc = "Shadow Reload Control, Global (Centralized) Reload Masked."]
        #[inline(always)]
        pub fn set_grmsk(&mut self, val: super::vals::Grmsk) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Rcr {
        #[inline(always)]
        fn default() -> Rcr {
            Rcr(0)
        }
    }
    impl core::fmt::Debug for Rcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcr")
                .field("imr", &self.imr())
                .field("vbr", &self.vbr())
                .field("grmsk", &self.grmsk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rcr {{ imr: {:?}, vbr: {:?}, grmsk: {:?} }}",
                self.imr(),
                self.vbr(),
                self.grmsk()
            )
        }
    }
    #[doc = "Ltdc Shadow Reload Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcr(pub u32);
    impl Srcr {
        #[doc = "Immediate Reload Trigger."]
        #[inline(always)]
        pub const fn imr(&self) -> super::vals::Imr {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Imr::from_bits(val as u8)
        }
        #[doc = "Immediate Reload Trigger."]
        #[inline(always)]
        pub fn set_imr(&mut self, val: super::vals::Imr) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Vertical Blanking Reload Request."]
        #[inline(always)]
        pub const fn vbr(&self) -> super::vals::Vbr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Vbr::from_bits(val as u8)
        }
        #[doc = "Vertical Blanking Reload Request."]
        #[inline(always)]
        pub fn set_vbr(&mut self, val: super::vals::Vbr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Srcr {
        #[inline(always)]
        fn default() -> Srcr {
            Srcr(0)
        }
    }
    impl core::fmt::Debug for Srcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcr")
                .field("imr", &self.imr())
                .field("vbr", &self.vbr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Srcr {{ imr: {:?}, vbr: {:?} }}", self.imr(), self.vbr())
        }
    }
    #[doc = "Ltdc Synchronization Size Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sscr(pub u32);
    impl Sscr {
        #[doc = "Vertical Synchronization Height (In Units Of Horizontal Scan Line)."]
        #[inline(always)]
        pub const fn vsh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Vertical Synchronization Height (In Units Of Horizontal Scan Line)."]
        #[inline(always)]
        pub fn set_vsh(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Horizontal Synchronization Width (In Units Of Pixel Clock Period)."]
        #[inline(always)]
        pub const fn hsw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Horizontal Synchronization Width (In Units Of Pixel Clock Period)."]
        #[inline(always)]
        pub fn set_hsw(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Sscr {
        #[inline(always)]
        fn default() -> Sscr {
            Sscr(0)
        }
    }
    impl core::fmt::Debug for Sscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sscr")
                .field("vsh", &self.vsh())
                .field("hsw", &self.hsw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sscr {{ vsh: {=u16:?}, hsw: {=u16:?} }}", self.vsh(), self.hsw())
        }
    }
    #[doc = "Ltdc Total Width Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twcr(pub u32);
    impl Twcr {
        #[doc = "Total Height (In Units Of Horizontal Scan Line)."]
        #[inline(always)]
        pub const fn totalh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Total Height (In Units Of Horizontal Scan Line)."]
        #[inline(always)]
        pub fn set_totalh(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Total Width (In Units Of Pixel Clock Period)."]
        #[inline(always)]
        pub const fn totalw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Total Width (In Units Of Pixel Clock Period)."]
        #[inline(always)]
        pub fn set_totalw(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Twcr {
        #[inline(always)]
        fn default() -> Twcr {
            Twcr(0)
        }
    }
    impl core::fmt::Debug for Twcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Twcr")
                .field("totalh", &self.totalh())
                .field("totalw", &self.totalw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Twcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Twcr {{ totalh: {=u16:?}, totalw: {=u16:?} }}",
                self.totalh(),
                self.totalw()
            )
        }
    }
    #[doc = "Layerx Window Horizontal Position Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Whpcr(pub u32);
    impl Whpcr {
        #[doc = "Window Horizontal Start Position."]
        #[inline(always)]
        pub const fn whstpos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Window Horizontal Start Position."]
        #[inline(always)]
        pub fn set_whstpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Window Horizontal Stop Position."]
        #[inline(always)]
        pub const fn whsppos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Window Horizontal Stop Position."]
        #[inline(always)]
        pub fn set_whsppos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Whpcr {
        #[inline(always)]
        fn default() -> Whpcr {
            Whpcr(0)
        }
    }
    impl core::fmt::Debug for Whpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Whpcr")
                .field("whstpos", &self.whstpos())
                .field("whsppos", &self.whsppos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Whpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Whpcr {{ whstpos: {=u16:?}, whsppos: {=u16:?} }}",
                self.whstpos(),
                self.whsppos()
            )
        }
    }
    #[doc = "Layerx Window Vertical Position Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wvpcr(pub u32);
    impl Wvpcr {
        #[doc = "Window Vertical Start Position."]
        #[inline(always)]
        pub const fn wvstpos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Window Vertical Start Position."]
        #[inline(always)]
        pub fn set_wvstpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Window Vertical Stop Position."]
        #[inline(always)]
        pub const fn wvsppos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Window Vertical Stop Position."]
        #[inline(always)]
        pub fn set_wvsppos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Wvpcr {
        #[inline(always)]
        fn default() -> Wvpcr {
            Wvpcr(0)
        }
    }
    impl core::fmt::Debug for Wvpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wvpcr")
                .field("wvstpos", &self.wvstpos())
                .field("wvsppos", &self.wvsppos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wvpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wvpcr {{ wvstpos: {=u16:?}, wvsppos: {=u16:?} }}",
                self.wvstpos(),
                self.wvsppos()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bf1 {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "constant alpha."]
        CONSTANT = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "pixel alpha x constant alpha."]
        PIXEL = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Bf1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bf1 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bf1 {
        #[inline(always)]
        fn from(val: u8) -> Bf1 {
            Bf1::from_bits(val)
        }
    }
    impl From<Bf1> for u8 {
        #[inline(always)]
        fn from(val: Bf1) -> u8 {
            Bf1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bf2 {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "1 - constant alpha."]
        CONSTANT = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "1 - (pixel alpha x constant alpha)."]
        PIXEL = 0x07,
    }
    impl Bf2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bf2 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bf2 {
        #[inline(always)]
        fn from(val: u8) -> Bf2 {
            Bf2::from_bits(val)
        }
    }
    impl From<Bf2> for u8 {
        #[inline(always)]
        fn from(val: Bf2) -> u8 {
            Bf2::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bl(u8);
    impl Bl {
        #[doc = "maximum burst length (16 words 64 bits, thus 128 Bytes)."]
        pub const MAXIMUM: Self = Self(0x0);
        #[doc = "1 word (of 64 bits) per burst."]
        pub const WORD1: Self = Self(0x01);
        #[doc = "16 words (of 64 bits) per burst."]
        pub const WORD16: Self = Self(0x10);
    }
    impl Bl {
        pub const fn from_bits(val: u8) -> Bl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Bl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("MAXIMUM"),
                0x01 => f.write_str("WORD1"),
                0x10 => f.write_str("WORD16"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "MAXIMUM"),
                0x01 => defmt::write!(f, "WORD1"),
                0x10 => defmt::write!(f, "WORD16"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Bl {
        #[inline(always)]
        fn from(val: u8) -> Bl {
            Bl::from_bits(val)
        }
    }
    impl From<Bl> for u8 {
        #[inline(always)]
        fn from(val: Bl) -> u8 {
            Bl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Boa {
        #[doc = "Blending Order Fixed."]
        FIXED = 0x0,
        #[doc = "Blending Order Configurable."]
        CONFIGURABLE = 0x01,
    }
    impl Boa {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Boa {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Boa {
        #[inline(always)]
        fn from(val: u8) -> Boa {
            Boa::from_bits(val)
        }
    }
    impl From<Boa> for u8 {
        #[inline(always)]
        fn from(val: Boa) -> u8 {
            Boa::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bor {
        #[doc = "layer set in background."]
        BACKGROUND = 0x0,
        #[doc = "layer set in foreground."]
        FOREGROUND = 0x01,
    }
    impl Bor {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bor {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bor {
        #[inline(always)]
        fn from(val: u8) -> Bor {
            Bor::from_bits(val)
        }
    }
    impl From<Bor> for u8 {
        #[inline(always)]
        fn from(val: Bor) -> u8 {
            Bor::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bw {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "32-Bit Bus."]
        BUS32 = 0x02,
        #[doc = "64-Bit Bus."]
        BUS64 = 0x03,
        #[doc = "128-Bit Bus."]
        BUS128 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Bw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bw {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bw {
        #[inline(always)]
        fn from(val: u8) -> Bw {
            Bw::from_bits(val)
        }
    }
    impl From<Bw> for u8 {
        #[inline(always)]
        fn from(val: Bw) -> u8 {
            Bw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ccrcif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears The Crcif Flag In Isrx."]
        CLEAR = 0x01,
    }
    impl Ccrcif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccrcif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccrcif {
        #[inline(always)]
        fn from(val: u8) -> Ccrcif {
            Ccrcif::from_bits(val)
        }
    }
    impl From<Ccrcif> for u8 {
        #[inline(always)]
        fn from(val: Ccrcif) -> u8 {
            Ccrcif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfuif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears The Fuif Flag In Isrx"]
        CLEAR = 0x01,
    }
    impl Cfuif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfuif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfuif {
        #[inline(always)]
        fn from(val: u8) -> Cfuif {
            Cfuif::from_bits(val)
        }
    }
    impl From<Cfuif> for u8 {
        #[inline(always)]
        fn from(val: Cfuif) -> u8 {
            Cfuif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfuwif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears The Fuwif Flag In Isrx."]
        CLEAR = 0x01,
    }
    impl Cfuwif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfuwif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfuwif {
        #[inline(always)]
        fn from(val: u8) -> Cfuwif {
            Cfuwif::from_bits(val)
        }
    }
    impl From<Cfuwif> for u8 {
        #[inline(always)]
        fn from(val: Cfuwif) -> u8 {
            Cfuwif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears The Lif Flag In Isrx."]
        CLEAR = 0x01,
    }
    impl Clif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clif {
        #[inline(always)]
        fn from(val: u8) -> Clif {
            Clif::from_bits(val)
        }
    }
    impl From<Clif> for u8 {
        #[inline(always)]
        fn from(val: Clif) -> u8 {
            Clif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Crca {
        #[doc = "Crc No Computation Available."]
        NOT_AVAILABLE = 0x0,
        #[doc = "Crc Computation Available."]
        AVAILABLE = 0x01,
    }
    impl Crca {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crca {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crca {
        #[inline(always)]
        fn from(val: u8) -> Crca {
            Crca::from_bits(val)
        }
    }
    impl From<Crca> for u8 {
        #[inline(always)]
        fn from(val: Crca) -> u8 {
            Crca::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Crrif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears The Rrif Flag In Isrx."]
        CLEAR = 0x01,
    }
    impl Crrif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crrif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crrif {
        #[inline(always)]
        fn from(val: u8) -> Crrif {
            Crrif::from_bits(val)
        }
    }
    impl From<Crrif> for u8 {
        #[inline(always)]
        fn from(val: Crrif) -> u8 {
            Crrif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cterrif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears The Terrif Flag In Isrx."]
        CLEAR = 0x01,
    }
    impl Cterrif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cterrif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cterrif {
        #[inline(always)]
        fn from(val: u8) -> Cterrif {
            Cterrif::from_bits(val)
        }
    }
    impl From<Cterrif> for u8 {
        #[inline(always)]
        fn from(val: Cterrif) -> u8 {
            Cterrif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Depol {
        #[doc = "Blanking (No Data/Pixel) Polarity Is Active Low."]
        ACTIVE_LOW = 0x0,
        #[doc = "Blanking (No Data/Pixel) Polarity Is Active High."]
        ACTIVE_HIGH = 0x01,
    }
    impl Depol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Depol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Depol {
        #[inline(always)]
        fn from(val: u8) -> Depol {
            Depol::from_bits(val)
        }
    }
    impl From<Depol> for u8 {
        #[inline(always)]
        fn from(val: Depol) -> u8 {
            Depol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dt {
        #[doc = "No Dithering."]
        NO_DITHERING = 0x0,
        #[doc = "Ordered 4X4 Bayer."]
        ORDERED = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Pseudo-Random Lfsr."]
        PSEUDO_RANDOM = 0x03,
    }
    impl Dt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dt {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dt {
        #[inline(always)]
        fn from(val: u8) -> Dt {
            Dt::from_bits(val)
        }
    }
    impl From<Dt> for u8 {
        #[inline(always)]
        fn from(val: Dt) -> u8 {
            Dt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Gct {
        #[doc = "No Gamma."]
        NO_GAMMA = 0x0,
        #[doc = "Gamma With 256 Samples."]
        GAMMA_SAMPLES = 0x01,
        #[doc = "Gamma With 8 Interpolated Segments."]
        GAMMA_INTERPOLATED = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Gct {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Gct {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Gct {
        #[inline(always)]
        fn from(val: u8) -> Gct {
            Gct::from_bits(val)
        }
    }
    impl From<Gct> for u8 {
        #[inline(always)]
        fn from(val: Gct) -> u8 {
            Gct::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Grmsk {
        #[doc = "global reload active for this layer (control from LTDC_SRCR enabled)."]
        ACTIVE = 0x0,
        #[doc = "global reload masked for this layer (control from LTDC_SRCR disabled)."]
        MASKED = 0x01,
    }
    impl Grmsk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Grmsk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Grmsk {
        #[inline(always)]
        fn from(val: u8) -> Grmsk {
            Grmsk::from_bits(val)
        }
    }
    impl From<Grmsk> for u8 {
        #[inline(always)]
        fn from(val: Grmsk) -> u8 {
            Grmsk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hspol {
        #[doc = "Horizontal Synchronization Polarity Is Active Low."]
        ACTIVE_LOW = 0x0,
        #[doc = "Horizontal Synchronization Polarity Is Active High."]
        ACTIVE_HIGH = 0x01,
    }
    impl Hspol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hspol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hspol {
        #[inline(always)]
        fn from(val: u8) -> Hspol {
            Hspol::from_bits(val)
        }
    }
    impl From<Hspol> for u8 {
        #[inline(always)]
        fn from(val: Hspol) -> u8 {
            Hspol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Imr {
        #[doc = "No Effect"]
        NO_EFFECT = 0x0,
        #[doc = "The Shadow Registers Are Reloaded Immediately."]
        RELOAD = 0x01,
    }
    impl Imr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Imr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Imr {
        #[inline(always)]
        fn from(val: u8) -> Imr {
            Imr::from_bits(val)
        }
    }
    impl From<Imr> for u8 {
        #[inline(always)]
        fn from(val: Imr) -> u8 {
            Imr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ocyco {
        #[doc = "Cb Is Output First (Y0Cb, Then Y1Cr, Y2Cb And So On)."]
        CB_FIRST = 0x0,
        #[doc = "Cr Is Output First (Y0Cr, Then Y1Cb, Y2Cr And So On)."]
        CR_FIRST = 0x01,
    }
    impl Ocyco {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ocyco {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ocyco {
        #[inline(always)]
        fn from(val: u8) -> Ocyco {
            Ocyco::from_bits(val)
        }
    }
    impl From<Ocyco> for u8 {
        #[inline(always)]
        fn from(val: Ocyco) -> u8 {
            Ocyco::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ocysel {
        #[doc = "Use Itu-R Bt.601 Set (For Typically Sdtv Analog-Like Displays)."]
        BT601 = 0x0,
        #[doc = "Use Itu-R Bt.709 Set (For Typically Hdtv Digital-Like Displays)."]
        BT709 = 0x01,
    }
    impl Ocysel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ocysel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ocysel {
        #[inline(always)]
        fn from(val: u8) -> Ocysel {
            Ocysel::from_bits(val)
        }
    }
    impl From<Ocysel> for u8 {
        #[inline(always)]
        fn from(val: Ocysel) -> u8 {
            Ocysel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pcpol {
        #[doc = "The Pixel And Sync Data Are Generated At The Rising-Edge Of The Output Lcd_Clk Clock."]
        RISING_EDGE = 0x0,
        #[doc = "The Pixel And Sync Data Are Generated At The Falling-Edge Of The Output Lcd_Clk Clock."]
        FALLING_EDGE = 0x01,
    }
    impl Pcpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pcpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pcpol {
        #[inline(always)]
        fn from(val: u8) -> Pcpol {
            Pcpol::from_bits(val)
        }
    }
    impl From<Pcpol> for u8 {
        #[inline(always)]
        fn from(val: Pcpol) -> u8 {
            Pcpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pf {
        #[doc = "ARGB8888 (32 bpp)."]
        ARGB8888 = 0x0,
        #[doc = "ABGR8888 (32 bpp)."]
        ABGR8888 = 0x01,
        #[doc = "RGBA8888 (32 bpp)."]
        RGBA8888 = 0x02,
        #[doc = "BGRA8888 (32 bpp)."]
        BGRA8888 = 0x03,
        #[doc = "RGB565 (16 bpp, A = 255)."]
        RGB565 = 0x04,
        #[doc = "BGR565 (16 bpp, A = 255)."]
        BGR565 = 0x05,
        #[doc = "RGB888 (24 bpp packed, A = 255)."]
        RGB888 = 0x06,
        #[doc = "Flexible pixel format selected."]
        FLEXIBLE = 0x07,
    }
    impl Pf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pf {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pf {
        #[inline(always)]
        fn from(val: u8) -> Pf {
            Pf::from_bits(val)
        }
    }
    impl From<Pf> for u8 {
        #[inline(always)]
        fn from(val: Pf) -> u8 {
            Pf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sfswtr {
        #[doc = "No Action."]
        NO_ACTION = 0x0,
        #[doc = "Triggers One Frame."]
        ONE_FRAME = 0x01,
    }
    impl Sfswtr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sfswtr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sfswtr {
        #[inline(always)]
        fn from(val: u8) -> Sfswtr {
            Sfswtr::from_bits(val)
        }
    }
    impl From<Sfswtr> for u8 {
        #[inline(always)]
        fn from(val: Sfswtr) -> u8 {
            Sfswtr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbr {
        #[doc = "No Effect"]
        NO_EFFECT = 0x0,
        #[doc = "The Shadow Registers Are Reloaded During The Vertical Blanking Period (At The Beginning Of The First Line After The Active Display Area)."]
        RELOAD = 0x01,
    }
    impl Vbr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbr {
        #[inline(always)]
        fn from(val: u8) -> Vbr {
            Vbr::from_bits(val)
        }
    }
    impl From<Vbr> for u8 {
        #[inline(always)]
        fn from(val: Vbr) -> u8 {
            Vbr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vspol {
        #[doc = "Vertical Synchronization Is Active Low."]
        ACTIVE_LOW = 0x0,
        #[doc = "Vertical Synchronization Is Active High."]
        ACTIVE_HIGH = 0x01,
    }
    impl Vspol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vspol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vspol {
        #[inline(always)]
        fn from(val: u8) -> Vspol {
            Vspol::from_bits(val)
        }
    }
    impl From<Vspol> for u8 {
        #[inline(always)]
        fn from(val: Vspol) -> u8 {
            Vspol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ycm {
        #[doc = "interleaved 422 (Cb and Cr component are replicated horizontally for pixels P and P+1)."]
        INTERLEAVED = 0x0,
        #[doc = "semi-Planar 420: (Cb and Cr component are replicated horizontally and vertically.The layer main configuration defines the access to the Y buffer, and auxiliary registers define the access to the Cb and Cr buffers)."]
        SEMI_PLANAR = 0x01,
        #[doc = "full-Planar 420: (Cb and Cr component are replicated horizontally and vertically. The layer main configuration defines the access to the Y buffer, and auxiliary registers define the access to the Cb and Cr buffers)."]
        FULL_PLANAR = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ycm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ycm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ycm {
        #[inline(always)]
        fn from(val: u8) -> Ycm {
            Ycm::from_bits(val)
        }
    }
    impl From<Ycm> for u8 {
        #[inline(always)]
        fn from(val: Ycm) -> u8 {
            Ycm::to_bits(val)
        }
    }
}
