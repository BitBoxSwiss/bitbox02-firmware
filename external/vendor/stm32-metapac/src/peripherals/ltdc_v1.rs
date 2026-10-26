#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
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
    #[doc = "Layerx Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Layerx Window Horizontal Position Configuration Register"]
    #[inline(always)]
    pub const fn whpcr(self) -> crate::common::Reg<regs::Whpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Layerx Window Vertical Position Configuration Register"]
    #[inline(always)]
    pub const fn wvpcr(self) -> crate::common::Reg<regs::Wvpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Layerx Color Keying Configuration Register"]
    #[inline(always)]
    pub const fn ckcr(self) -> crate::common::Reg<regs::Ckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Layerx Pixel Format Configuration Register"]
    #[inline(always)]
    pub const fn pfcr(self) -> crate::common::Reg<regs::Pfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Layerx Constant Alpha Configuration Register"]
    #[inline(always)]
    pub const fn cacr(self) -> crate::common::Reg<regs::Cacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Layerx Default Color Configuration Register"]
    #[inline(always)]
    pub const fn dccr(self) -> crate::common::Reg<regs::Dccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Layerx Blending Factors Configuration Register"]
    #[inline(always)]
    pub const fn bfcr(self) -> crate::common::Reg<regs::Bfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Address Register"]
    #[inline(always)]
    pub const fn cfbar(self) -> crate::common::Reg<regs::Cfbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Length Register"]
    #[inline(always)]
    pub const fn cfblr(self) -> crate::common::Reg<regs::Cfblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Layerx ColorFrame Buffer Line Number Register"]
    #[inline(always)]
    pub const fn cfblnr(self) -> crate::common::Reg<regs::Cfblnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Layerx CLUT Write Register"]
    #[inline(always)]
    pub const fn clutwr(self) -> crate::common::Reg<regs::Clutwr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
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
    #[doc = "Synchronization Size Configuration Register"]
    #[inline(always)]
    pub const fn sscr(self) -> crate::common::Reg<regs::Sscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Back Porch Configuration Register"]
    #[inline(always)]
    pub const fn bpcr(self) -> crate::common::Reg<regs::Bpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Active Width Configuration Register"]
    #[inline(always)]
    pub const fn awcr(self) -> crate::common::Reg<regs::Awcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Total Width Configuration Register"]
    #[inline(always)]
    pub const fn twcr(self) -> crate::common::Reg<regs::Twcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Global Control Register"]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Shadow Reload Configuration Register"]
    #[inline(always)]
    pub const fn srcr(self) -> crate::common::Reg<regs::Srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Background Color Configuration Register"]
    #[inline(always)]
    pub const fn bccr(self) -> crate::common::Reg<regs::Bccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Line Interrupt Position Configuration Register"]
    #[inline(always)]
    pub const fn lipcr(self) -> crate::common::Reg<regs::Lipcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Current Position Status Register"]
    #[inline(always)]
    pub const fn cpsr(self) -> crate::common::Reg<regs::Cpsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Current Display Status Register"]
    #[inline(always)]
    pub const fn cdsr(self) -> crate::common::Reg<regs::Cdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    #[inline(always)]
    pub const fn layer(self, n: usize) -> Layer {
        assert!(n < 2usize);
        unsafe { Layer::from_ptr(self.ptr.add(0x84usize + n * 128usize) as _) }
    }
}
pub mod regs {
    #[doc = "Active Width Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awcr(pub u32);
    impl Awcr {
        #[doc = "Accumulated Active Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub const fn aah(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Accumulated Active Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub fn set_aah(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Accumulated Active Width (in units of pixel clock period)"]
        #[inline(always)]
        pub const fn aaw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Accumulated Active Width (in units of pixel clock period)"]
        #[inline(always)]
        pub fn set_aaw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
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
    #[doc = "Background Color Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bccr(pub u32);
    impl Bccr {
        #[doc = "Background color blue value"]
        #[inline(always)]
        pub const fn bcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Background color blue value"]
        #[inline(always)]
        pub fn set_bcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Background color green value"]
        #[inline(always)]
        pub const fn bcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Background color green value"]
        #[inline(always)]
        pub fn set_bcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Background color red value"]
        #[inline(always)]
        pub const fn bcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Background color red value"]
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
    #[doc = "Layerx Blending Factors Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bfcr(pub u32);
    impl Bfcr {
        #[doc = "Blending Factor 2"]
        #[inline(always)]
        pub const fn bf2(&self) -> super::vals::Bf2 {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Bf2::from_bits(val as u8)
        }
        #[doc = "Blending Factor 2"]
        #[inline(always)]
        pub fn set_bf2(&mut self, val: super::vals::Bf2) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Blending Factor 1"]
        #[inline(always)]
        pub const fn bf1(&self) -> super::vals::Bf1 {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Bf1::from_bits(val as u8)
        }
        #[doc = "Blending Factor 1"]
        #[inline(always)]
        pub fn set_bf1(&mut self, val: super::vals::Bf1) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bfcr {{ bf2: {:?}, bf1: {:?} }}", self.bf2(), self.bf1())
        }
    }
    #[doc = "Back Porch Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcr(pub u32);
    impl Bpcr {
        #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)"]
        #[inline(always)]
        pub const fn avbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)"]
        #[inline(always)]
        pub fn set_avbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)"]
        #[inline(always)]
        pub const fn ahbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)"]
        #[inline(always)]
        pub fn set_ahbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
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
    #[doc = "Layerx Constant Alpha Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cacr(pub u32);
    impl Cacr {
        #[doc = "Constant Alpha"]
        #[inline(always)]
        pub const fn consta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Constant Alpha"]
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
    #[doc = "Current Display Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdsr(pub u32);
    impl Cdsr {
        #[doc = "Vertical Data Enable display Status"]
        #[inline(always)]
        pub const fn vdes(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Data Enable display Status"]
        #[inline(always)]
        pub fn set_vdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal Data Enable display Status"]
        #[inline(always)]
        pub const fn hdes(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Data Enable display Status"]
        #[inline(always)]
        pub fn set_hdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Vertical Synchronization display Status"]
        #[inline(always)]
        pub const fn vsyncs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Synchronization display Status"]
        #[inline(always)]
        pub fn set_vsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Horizontal Synchronization display Status"]
        #[inline(always)]
        pub const fn hsyncs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Synchronization display Status"]
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
    #[doc = "Layerx Color Frame Buffer Address Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfbar(pub u32);
    impl Cfbar {
        #[doc = "Color Frame Buffer Start Address"]
        #[inline(always)]
        pub const fn cfbadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Color Frame Buffer Start Address"]
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
    #[doc = "Layerx ColorFrame Buffer Line Number Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfblnr(pub u32);
    impl Cfblnr {
        #[doc = "Frame Buffer Line Number"]
        #[inline(always)]
        pub const fn cfblnbr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Frame Buffer Line Number"]
        #[inline(always)]
        pub fn set_cfblnbr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
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
    #[doc = "Layerx Color Frame Buffer Length Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfblr(pub u32);
    impl Cfblr {
        #[doc = "Color Frame Buffer Line Length"]
        #[inline(always)]
        pub const fn cfbll(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Color Frame Buffer Line Length"]
        #[inline(always)]
        pub fn set_cfbll(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Color Frame Buffer Pitch in bytes"]
        #[inline(always)]
        pub const fn cfbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Color Frame Buffer Pitch in bytes"]
        #[inline(always)]
        pub fn set_cfbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
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
    #[doc = "Layerx Color Keying Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckcr(pub u32);
    impl Ckcr {
        #[doc = "Color Key Blue value"]
        #[inline(always)]
        pub const fn ckblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Blue value"]
        #[inline(always)]
        pub fn set_ckblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Color Key Green value"]
        #[inline(always)]
        pub const fn ckgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Green value"]
        #[inline(always)]
        pub fn set_ckgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Color Key Red value"]
        #[inline(always)]
        pub const fn ckred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Red value"]
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
    #[doc = "Layerx CLUT Write Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clutwr(pub u32);
    impl Clutwr {
        #[doc = "Blue value"]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue value"]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green value"]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green value"]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red value"]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red value"]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "CLUT Address"]
        #[inline(always)]
        pub const fn clutadd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "CLUT Address"]
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
    #[doc = "Current Position Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cpsr(pub u32);
    impl Cpsr {
        #[doc = "Current Y Position"]
        #[inline(always)]
        pub const fn cypos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Current Y Position"]
        #[inline(always)]
        pub fn set_cypos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Current X Position"]
        #[inline(always)]
        pub const fn cxpos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Current X Position"]
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
    #[doc = "Layerx Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Layer Enable"]
        #[inline(always)]
        pub const fn len(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer Enable"]
        #[inline(always)]
        pub fn set_len(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color Keying Enable"]
        #[inline(always)]
        pub const fn colken(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Color Keying Enable"]
        #[inline(always)]
        pub fn set_colken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Color Look-Up Table Enable"]
        #[inline(always)]
        pub const fn cluten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Color Look-Up Table Enable"]
        #[inline(always)]
        pub fn set_cluten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("colken", &self.colken())
                .field("cluten", &self.cluten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ len: {=bool:?}, colken: {=bool:?}, cluten: {=bool:?} }}",
                self.len(),
                self.colken(),
                self.cluten()
            )
        }
    }
    #[doc = "Layerx Default Color Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dccr(pub u32);
    impl Dccr {
        #[doc = "Default Color Blue"]
        #[inline(always)]
        pub const fn dcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Blue"]
        #[inline(always)]
        pub fn set_dcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Default Color Green"]
        #[inline(always)]
        pub const fn dcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Green"]
        #[inline(always)]
        pub fn set_dcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Default Color Red"]
        #[inline(always)]
        pub const fn dcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Red"]
        #[inline(always)]
        pub fn set_dcred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Default Color Alpha"]
        #[inline(always)]
        pub const fn dcalpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Alpha"]
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
    #[doc = "Global Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "LCD-TFT controller enable bit"]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LCD-TFT controller enable bit"]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Dither Blue Width"]
        #[inline(always)]
        pub const fn dbw(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Blue Width"]
        #[inline(always)]
        pub fn set_dbw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Dither Green Width"]
        #[inline(always)]
        pub const fn dgw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Green Width"]
        #[inline(always)]
        pub fn set_dgw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Dither Red Width"]
        #[inline(always)]
        pub const fn drw(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Red Width"]
        #[inline(always)]
        pub fn set_drw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Dither Enable"]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Dither Enable"]
        #[inline(always)]
        pub fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Pixel Clock Polarity"]
        #[inline(always)]
        pub const fn pcpol(&self) -> super::vals::Pcpol {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Pcpol::from_bits(val as u8)
        }
        #[doc = "Pixel Clock Polarity"]
        #[inline(always)]
        pub fn set_pcpol(&mut self, val: super::vals::Pcpol) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "Data Enable Polarity"]
        #[inline(always)]
        pub const fn depol(&self) -> super::vals::Depol {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Depol::from_bits(val as u8)
        }
        #[doc = "Data Enable Polarity"]
        #[inline(always)]
        pub fn set_depol(&mut self, val: super::vals::Depol) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Vertical Synchronization Polarity"]
        #[inline(always)]
        pub const fn vspol(&self) -> super::vals::Vspol {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Vspol::from_bits(val as u8)
        }
        #[doc = "Vertical Synchronization Polarity"]
        #[inline(always)]
        pub fn set_vspol(&mut self, val: super::vals::Vspol) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Horizontal Synchronization Polarity"]
        #[inline(always)]
        pub const fn hspol(&self) -> super::vals::Hspol {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Hspol::from_bits(val as u8)
        }
        #[doc = "Horizontal Synchronization Polarity"]
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
                .field("dbw", &self.dbw())
                .field("dgw", &self.dgw())
                .field("drw", &self.drw())
                .field("den", &self.den())
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
            defmt :: write ! (f , "Gcr {{ ltdcen: {=bool:?}, dbw: {=u8:?}, dgw: {=u8:?}, drw: {=u8:?}, den: {=bool:?}, pcpol: {:?}, depol: {:?}, vspol: {:?}, hspol: {:?} }}" , self . ltdcen () , self . dbw () , self . dgw () , self . drw () , self . den () , self . pcpol () , self . depol () , self . vspol () , self . hspol ())
        }
    }
    #[doc = "Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Clears the Line Interrupt Flag"]
        #[inline(always)]
        pub const fn clif(&self) -> super::vals::Clif {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Clif::from_bits(val as u8)
        }
        #[doc = "Clears the Line Interrupt Flag"]
        #[inline(always)]
        pub fn set_clif(&mut self, val: super::vals::Clif) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clears the FIFO Underrun Interrupt flag"]
        #[inline(always)]
        pub const fn cfuif(&self) -> super::vals::Cfuif {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cfuif::from_bits(val as u8)
        }
        #[doc = "Clears the FIFO Underrun Interrupt flag"]
        #[inline(always)]
        pub fn set_cfuif(&mut self, val: super::vals::Cfuif) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clears the Transfer Error Interrupt Flag"]
        #[inline(always)]
        pub const fn cterrif(&self) -> super::vals::Cterrif {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Cterrif::from_bits(val as u8)
        }
        #[doc = "Clears the Transfer Error Interrupt Flag"]
        #[inline(always)]
        pub fn set_cterrif(&mut self, val: super::vals::Cterrif) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clears Register Reload Interrupt Flag"]
        #[inline(always)]
        pub const fn crrif(&self) -> super::vals::Crrif {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Crrif::from_bits(val as u8)
        }
        #[doc = "Clears Register Reload Interrupt Flag"]
        #[inline(always)]
        pub fn set_crrif(&mut self, val: super::vals::Crrif) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
                .field("cfuif", &self.cfuif())
                .field("cterrif", &self.cterrif())
                .field("crrif", &self.crrif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ clif: {:?}, cfuif: {:?}, cterrif: {:?}, crrif: {:?} }}",
                self.clif(),
                self.cfuif(),
                self.cterrif(),
                self.crrif()
            )
        }
    }
    #[doc = "Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Line Interrupt Enable"]
        #[inline(always)]
        pub const fn lie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt Enable"]
        #[inline(always)]
        pub fn set_lie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO Underrun Interrupt Enable"]
        #[inline(always)]
        pub const fn fuie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Underrun Interrupt Enable"]
        #[inline(always)]
        pub fn set_fuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer Error Interrupt Enable"]
        #[inline(always)]
        pub const fn terrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Interrupt Enable"]
        #[inline(always)]
        pub fn set_terrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Register Reload interrupt enable"]
        #[inline(always)]
        pub const fn rrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload interrupt enable"]
        #[inline(always)]
        pub fn set_rrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("fuie", &self.fuie())
                .field("terrie", &self.terrie())
                .field("rrie", &self.rrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ lie: {=bool:?}, fuie: {=bool:?}, terrie: {=bool:?}, rrie: {=bool:?} }}",
                self.lie(),
                self.fuie(),
                self.terrie(),
                self.rrie()
            )
        }
    }
    #[doc = "Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Line Interrupt flag"]
        #[inline(always)]
        pub const fn lif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt flag"]
        #[inline(always)]
        pub fn set_lif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO Underrun Interrupt flag"]
        #[inline(always)]
        pub const fn fuif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Underrun Interrupt flag"]
        #[inline(always)]
        pub fn set_fuif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer Error interrupt flag"]
        #[inline(always)]
        pub const fn terrif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error interrupt flag"]
        #[inline(always)]
        pub fn set_terrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Register Reload Interrupt Flag"]
        #[inline(always)]
        pub const fn rrif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload Interrupt Flag"]
        #[inline(always)]
        pub fn set_rrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("fuif", &self.fuif())
                .field("terrif", &self.terrif())
                .field("rrif", &self.rrif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ lif: {=bool:?}, fuif: {=bool:?}, terrif: {=bool:?}, rrif: {=bool:?} }}",
                self.lif(),
                self.fuif(),
                self.terrif(),
                self.rrif()
            )
        }
    }
    #[doc = "Line Interrupt Position Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lipcr(pub u32);
    impl Lipcr {
        #[doc = "Line Interrupt Position"]
        #[inline(always)]
        pub const fn lipos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Line Interrupt Position"]
        #[inline(always)]
        pub fn set_lipos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
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
    #[doc = "Layerx Pixel Format Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfcr(pub u32);
    impl Pfcr {
        #[doc = "Pixel Format"]
        #[inline(always)]
        pub const fn pf(&self) -> super::vals::Pf {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Pf::from_bits(val as u8)
        }
        #[doc = "Pixel Format"]
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
    #[doc = "Shadow Reload Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcr(pub u32);
    impl Srcr {
        #[doc = "Immediate Reload"]
        #[inline(always)]
        pub const fn imr(&self) -> super::vals::Imr {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Imr::from_bits(val as u8)
        }
        #[doc = "Immediate Reload"]
        #[inline(always)]
        pub fn set_imr(&mut self, val: super::vals::Imr) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Vertical Blanking Reload"]
        #[inline(always)]
        pub const fn vbr(&self) -> super::vals::Vbr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Vbr::from_bits(val as u8)
        }
        #[doc = "Vertical Blanking Reload"]
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
    #[doc = "Synchronization Size Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sscr(pub u32);
    impl Sscr {
        #[doc = "Vertical Synchronization Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub const fn vsh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Vertical Synchronization Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub fn set_vsh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Horizontal Synchronization Width (in units of pixel clock period)"]
        #[inline(always)]
        pub const fn hsw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal Synchronization Width (in units of pixel clock period)"]
        #[inline(always)]
        pub fn set_hsw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
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
    #[doc = "Total Width Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twcr(pub u32);
    impl Twcr {
        #[doc = "Total Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub const fn totalh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Total Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub fn set_totalh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Total Width (in units of pixel clock period)"]
        #[inline(always)]
        pub const fn totalw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Total Width (in units of pixel clock period)"]
        #[inline(always)]
        pub fn set_totalw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
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
    #[doc = "Layerx Window Horizontal Position Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Whpcr(pub u32);
    impl Whpcr {
        #[doc = "Window Horizontal Start Position"]
        #[inline(always)]
        pub const fn whstpos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Window Horizontal Start Position"]
        #[inline(always)]
        pub fn set_whstpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Window Horizontal Stop Position"]
        #[inline(always)]
        pub const fn whsppos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Window Horizontal Stop Position"]
        #[inline(always)]
        pub fn set_whsppos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
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
    #[doc = "Layerx Window Vertical Position Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wvpcr(pub u32);
    impl Wvpcr {
        #[doc = "Window Vertical Start Position"]
        #[inline(always)]
        pub const fn wvstpos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Window Vertical Start Position"]
        #[inline(always)]
        pub fn set_wvstpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Window Vertical Stop Position"]
        #[inline(always)]
        pub const fn wvsppos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Window Vertical Stop Position"]
        #[inline(always)]
        pub fn set_wvsppos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
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
        #[doc = "BF1 = constant alpha"]
        CONSTANT = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "BF1 = pixel alpha * constant alpha"]
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
        #[doc = "BF2 = 1 - constant alpha"]
        CONSTANT = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "BF2 = 1 - pixel alpha * constant alpha"]
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfuif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears the FUIF flag in the ISR register"]
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
    pub enum Clif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears the LIF flag in the ISR register"]
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
    pub enum Crrif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears the RRIF flag in the ISR register"]
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
        #[doc = "Clears the TERRIF flag in the ISR register"]
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
        #[doc = "Data enable polarity is active low"]
        ACTIVE_LOW = 0x0,
        #[doc = "Data enable polarity is active high"]
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
    pub enum Hspol {
        #[doc = "Horizontal synchronization polarity is active low"]
        ACTIVE_LOW = 0x0,
        #[doc = "Horizontal synchronization polarity is active high"]
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
        #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
        NO_EFFECT = 0x0,
        #[doc = "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
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
    pub enum Pcpol {
        #[doc = "Pixel clock on rising edge"]
        RISING_EDGE = 0x0,
        #[doc = "Pixel clock on falling edge"]
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
        #[doc = "L8 (8-bit luminance)"]
        L8 = 0x05,
        #[doc = "AL44 (4-bit alpha, 4-bit luminance)"]
        AL44 = 0x06,
        #[doc = "AL88 (8-bit alpha, 8-bit luminance)"]
        AL88 = 0x07,
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
    pub enum Vbr {
        #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
        NO_EFFECT = 0x0,
        #[doc = "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
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
        #[doc = "Vertical synchronization polarity is active low"]
        ACTIVE_LOW = 0x0,
        #[doc = "Vertical synchronization polarity is active high"]
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
}
