#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Digital-to-analog converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac {
    ptr: *mut u8,
}
unsafe impl Send for Dac {}
unsafe impl Sync for Dac {}
impl Dac {
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
    #[doc = "software trigger register"]
    #[inline(always)]
    pub const fn swtrigr(self) -> crate::common::Reg<regs::Swtrigr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "channel 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12r(self, n: usize) -> crate::common::Reg<regs::Dhr12r, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 12usize) as _) }
    }
    #[doc = "channel 12-bit left-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12l(self, n: usize) -> crate::common::Reg<regs::Dhr12l, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 12usize) as _) }
    }
    #[doc = "channel 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8r(self, n: usize) -> crate::common::Reg<regs::Dhr8r, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 12usize) as _) }
    }
    #[doc = "dual 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12rd(self) -> crate::common::Reg<regs::Dhr12rd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "dual 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12ld(self) -> crate::common::Reg<regs::Dhr12ld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "dual 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8rd(self) -> crate::common::Reg<regs::Dhr8rd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "channel data output register"]
    #[inline(always)]
    pub const fn dor(self, n: usize) -> crate::common::Reg<regs::Dor, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize + n * 4usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "calibration control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "mode control register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "sample and hold sample time register"]
    #[inline(always)]
    pub const fn shsr(self, n: usize) -> crate::common::Reg<regs::Shsr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "sample and hold hold time register"]
    #[inline(always)]
    pub const fn shhr(self) -> crate::common::Reg<regs::Shhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "sample and hold refresh time register"]
    #[inline(always)]
    pub const fn shrr(self) -> crate::common::Reg<regs::Shrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Sawtooth register"]
    #[inline(always)]
    pub const fn str(self, n: usize) -> crate::common::Reg<regs::Str, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize + n * 4usize) as _) }
    }
    #[doc = "Sawtooth Mode register"]
    #[inline(always)]
    pub const fn stmodr(self) -> crate::common::Reg<regs::Stmodr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
}
pub mod regs {
    #[doc = "calibration control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "channel offset trimming value"]
        #[inline(always)]
        pub const fn otrim(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "channel offset trimming value"]
        #[inline(always)]
        pub fn set_otrim(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr")
                .field("otrim[0]", &self.otrim(0usize))
                .field("otrim[1]", &self.otrim(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ otrim[0]: {=u8:?}, otrim[1]: {=u8:?} }}",
                self.otrim(0usize),
                self.otrim(1usize)
            )
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "channel enable"]
        #[inline(always)]
        pub const fn en(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel enable"]
        #[inline(always)]
        pub fn set_en(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel trigger enable"]
        #[inline(always)]
        pub const fn ten(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel trigger enable"]
        #[inline(always)]
        pub fn set_ten(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel trigger selection"]
        #[inline(always)]
        pub const fn tsel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "channel trigger selection"]
        #[inline(always)]
        pub fn set_tsel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "channel noise/triangle wave generation enable"]
        #[inline(always)]
        pub const fn wave(&self, n: usize) -> super::vals::Wave {
            assert!(n < 2usize);
            let offs = 6usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Wave::from_bits(val as u8)
        }
        #[doc = "channel noise/triangle wave generation enable"]
        #[inline(always)]
        pub fn set_wave(&mut self, n: usize, val: super::vals::Wave) {
            assert!(n < 2usize);
            let offs = 6usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "channel mask/amplitude selector"]
        #[inline(always)]
        pub const fn mamp(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "channel mask/amplitude selector"]
        #[inline(always)]
        pub fn set_mamp(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "channel DMA enable"]
        #[inline(always)]
        pub const fn dmaen(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel DMA enable"]
        #[inline(always)]
        pub fn set_dmaen(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel DMA Underrun Interrupt enable"]
        #[inline(always)]
        pub const fn dmaudrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 13usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel DMA Underrun Interrupt enable"]
        #[inline(always)]
        pub fn set_dmaudrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 13usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "DAC channel calibration enable"]
        #[inline(always)]
        pub const fn cen(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 14usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "DAC channel calibration enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 14usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("en[0]", &self.en(0usize))
                .field("en[1]", &self.en(1usize))
                .field("ten[0]", &self.ten(0usize))
                .field("ten[1]", &self.ten(1usize))
                .field("tsel[0]", &self.tsel(0usize))
                .field("tsel[1]", &self.tsel(1usize))
                .field("wave[0]", &self.wave(0usize))
                .field("wave[1]", &self.wave(1usize))
                .field("mamp[0]", &self.mamp(0usize))
                .field("mamp[1]", &self.mamp(1usize))
                .field("dmaen[0]", &self.dmaen(0usize))
                .field("dmaen[1]", &self.dmaen(1usize))
                .field("dmaudrie[0]", &self.dmaudrie(0usize))
                .field("dmaudrie[1]", &self.dmaudrie(1usize))
                .field("cen[0]", &self.cen(0usize))
                .field("cen[1]", &self.cen(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en[0]: {=bool:?}, en[1]: {=bool:?}, ten[0]: {=bool:?}, ten[1]: {=bool:?}, tsel[0]: {=u8:?}, tsel[1]: {=u8:?}, wave[0]: {:?}, wave[1]: {:?}, mamp[0]: {=u8:?}, mamp[1]: {=u8:?}, dmaen[0]: {=bool:?}, dmaen[1]: {=bool:?}, dmaudrie[0]: {=bool:?}, dmaudrie[1]: {=bool:?}, cen[0]: {=bool:?}, cen[1]: {=bool:?} }}" , self . en (0usize) , self . en (1usize) , self . ten (0usize) , self . ten (1usize) , self . tsel (0usize) , self . tsel (1usize) , self . wave (0usize) , self . wave (1usize) , self . mamp (0usize) , self . mamp (1usize) , self . dmaen (0usize) , self . dmaen (1usize) , self . dmaudrie (0usize) , self . dmaudrie (1usize) , self . cen (0usize) , self . cen (1usize))
        }
    }
    #[doc = "channel 12-bit left-aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12l(pub u32);
    impl Dhr12l {
        #[doc = "channel 12-bit left-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit left-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[doc = "channel 12-bit left-aligned data B"]
        #[inline(always)]
        pub const fn dhrb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit left-aligned data B"]
        #[inline(always)]
        pub fn set_dhrb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dhr12l {
        #[inline(always)]
        fn default() -> Dhr12l {
            Dhr12l(0)
        }
    }
    impl core::fmt::Debug for Dhr12l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12l")
                .field("dhr", &self.dhr())
                .field("dhrb", &self.dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12l {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr12l {{ dhr: {=u16:?}, dhrb: {=u16:?} }}", self.dhr(), self.dhrb())
        }
    }
    #[doc = "dual 12-bit left aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12ld(pub u32);
    impl Dhr12ld {
        #[doc = "channel 12-bit left-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self, n: usize) -> u16 {
            assert!(n < 2usize);
            let offs = 4usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit left-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, n: usize, val: u16) {
            assert!(n < 2usize);
            let offs = 4usize + n * 16usize;
            self.0 = (self.0 & !(0x0fff << offs)) | (((val as u32) & 0x0fff) << offs);
        }
    }
    impl Default for Dhr12ld {
        #[inline(always)]
        fn default() -> Dhr12ld {
            Dhr12ld(0)
        }
    }
    impl core::fmt::Debug for Dhr12ld {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12ld")
                .field("dhr[0]", &self.dhr(0usize))
                .field("dhr[1]", &self.dhr(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12ld {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dhr12ld {{ dhr[0]: {=u16:?}, dhr[1]: {=u16:?} }}",
                self.dhr(0usize),
                self.dhr(1usize)
            )
        }
    }
    #[doc = "channel 12-bit right-aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12r(pub u32);
    impl Dhr12r {
        #[doc = "channel 12-bit right-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit right-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "channel 12-bit right-aligned data B"]
        #[inline(always)]
        pub const fn dhrb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit right-aligned data B"]
        #[inline(always)]
        pub fn set_dhrb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Dhr12r {
        #[inline(always)]
        fn default() -> Dhr12r {
            Dhr12r(0)
        }
    }
    impl core::fmt::Debug for Dhr12r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12r")
                .field("dhr", &self.dhr())
                .field("dhrb", &self.dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr12r {{ dhr: {=u16:?}, dhrb: {=u16:?} }}", self.dhr(), self.dhrb())
        }
    }
    #[doc = "dual 12-bit right-aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12rd(pub u32);
    impl Dhr12rd {
        #[doc = "channel 12-bit right-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self, n: usize) -> u16 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit right-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, n: usize, val: u16) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x0fff << offs)) | (((val as u32) & 0x0fff) << offs);
        }
    }
    impl Default for Dhr12rd {
        #[inline(always)]
        fn default() -> Dhr12rd {
            Dhr12rd(0)
        }
    }
    impl core::fmt::Debug for Dhr12rd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12rd")
                .field("dhr[0]", &self.dhr(0usize))
                .field("dhr[1]", &self.dhr(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12rd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dhr12rd {{ dhr[0]: {=u16:?}, dhr[1]: {=u16:?} }}",
                self.dhr(0usize),
                self.dhr(1usize)
            )
        }
    }
    #[doc = "channel 8-bit right-aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr8r(pub u32);
    impl Dhr8r {
        #[doc = "channel 8-bit right-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "channel 8-bit right-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "channel 8-bit right-aligned data B"]
        #[inline(always)]
        pub const fn dhrb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "channel 8-bit right-aligned data B"]
        #[inline(always)]
        pub fn set_dhrb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Dhr8r {
        #[inline(always)]
        fn default() -> Dhr8r {
            Dhr8r(0)
        }
    }
    impl core::fmt::Debug for Dhr8r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr8r")
                .field("dhr", &self.dhr())
                .field("dhrb", &self.dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr8r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr8r {{ dhr: {=u8:?}, dhrb: {=u8:?} }}", self.dhr(), self.dhrb())
        }
    }
    #[doc = "dual 8-bit right aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr8rd(pub u32);
    impl Dhr8rd {
        #[doc = "channel 8-bit right-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "channel 8-bit right-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Dhr8rd {
        #[inline(always)]
        fn default() -> Dhr8rd {
            Dhr8rd(0)
        }
    }
    impl core::fmt::Debug for Dhr8rd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr8rd")
                .field("dhr[0]", &self.dhr(0usize))
                .field("dhr[1]", &self.dhr(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr8rd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dhr8rd {{ dhr[0]: {=u8:?}, dhr[1]: {=u8:?} }}",
                self.dhr(0usize),
                self.dhr(1usize)
            )
        }
    }
    #[doc = "channel data output register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dor(pub u32);
    impl Dor {
        #[doc = "channel data output"]
        #[inline(always)]
        pub const fn dor(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel data output"]
        #[inline(always)]
        pub fn set_dor(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "channel data output B"]
        #[inline(always)]
        pub const fn dorb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel data output B"]
        #[inline(always)]
        pub fn set_dorb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Dor {
        #[inline(always)]
        fn default() -> Dor {
            Dor(0)
        }
    }
    impl core::fmt::Debug for Dor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dor")
                .field("dor", &self.dor())
                .field("dorb", &self.dorb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dor {{ dor: {=u16:?}, dorb: {=u16:?} }}", self.dor(), self.dorb())
        }
    }
    #[doc = "mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "DAC channel mode"]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "DAC channel mode"]
        #[inline(always)]
        pub fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "channel DMA double data mode."]
        #[inline(always)]
        pub const fn dmadouble(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel DMA double data mode."]
        #[inline(always)]
        pub fn set_dmadouble(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "enable signed format for DAC channel"]
        #[inline(always)]
        pub const fn sinformat(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "enable signed format for DAC channel"]
        #[inline(always)]
        pub fn set_sinformat(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "high frequency interface mode selection"]
        #[inline(always)]
        pub const fn hfsel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "high frequency interface mode selection"]
        #[inline(always)]
        pub fn set_hfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    impl core::fmt::Debug for Mcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr")
                .field("mode[0]", &self.mode(0usize))
                .field("mode[1]", &self.mode(1usize))
                .field("dmadouble[0]", &self.dmadouble(0usize))
                .field("dmadouble[1]", &self.dmadouble(1usize))
                .field("sinformat[0]", &self.sinformat(0usize))
                .field("sinformat[1]", &self.sinformat(1usize))
                .field("hfsel", &self.hfsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mcr {{ mode[0]: {:?}, mode[1]: {:?}, dmadouble[0]: {=bool:?}, dmadouble[1]: {=bool:?}, sinformat[0]: {=bool:?}, sinformat[1]: {=bool:?}, hfsel: {=u8:?} }}" , self . mode (0usize) , self . mode (1usize) , self . dmadouble (0usize) , self . dmadouble (1usize) , self . sinformat (0usize) , self . sinformat (1usize) , self . hfsel ())
        }
    }
    #[doc = "sample and hold hold time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shhr(pub u32);
    impl Shhr {
        #[doc = "channel hold time"]
        #[inline(always)]
        pub const fn thold(&self, n: usize) -> u16 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03ff;
            val as u16
        }
        #[doc = "channel hold time"]
        #[inline(always)]
        pub fn set_thold(&mut self, n: usize, val: u16) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x03ff << offs)) | (((val as u32) & 0x03ff) << offs);
        }
    }
    impl Default for Shhr {
        #[inline(always)]
        fn default() -> Shhr {
            Shhr(0)
        }
    }
    impl core::fmt::Debug for Shhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Shhr")
                .field("thold[0]", &self.thold(0usize))
                .field("thold[1]", &self.thold(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Shhr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Shhr {{ thold[0]: {=u16:?}, thold[1]: {=u16:?} }}",
                self.thold(0usize),
                self.thold(1usize)
            )
        }
    }
    #[doc = "sample and hold refresh time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shrr(pub u32);
    impl Shrr {
        #[doc = "channel refresh time"]
        #[inline(always)]
        pub const fn trefresh(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "channel refresh time"]
        #[inline(always)]
        pub fn set_trefresh(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Shrr {
        #[inline(always)]
        fn default() -> Shrr {
            Shrr(0)
        }
    }
    impl core::fmt::Debug for Shrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Shrr")
                .field("trefresh[0]", &self.trefresh(0usize))
                .field("trefresh[1]", &self.trefresh(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Shrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Shrr {{ trefresh[0]: {=u8:?}, trefresh[1]: {=u8:?} }}",
                self.trefresh(0usize),
                self.trefresh(1usize)
            )
        }
    }
    #[doc = "sample and hold sample time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shsr(pub u32);
    impl Shsr {
        #[doc = "channel sample time"]
        #[inline(always)]
        pub const fn tsample(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "channel sample time"]
        #[inline(always)]
        pub fn set_tsample(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Shsr {
        #[inline(always)]
        fn default() -> Shsr {
            Shsr(0)
        }
    }
    impl core::fmt::Debug for Shsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Shsr").field("tsample", &self.tsample()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Shsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Shsr {{ tsample: {=u16:?} }}", self.tsample())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "channel ready status bit"]
        #[inline(always)]
        pub const fn dacrdy(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 11usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel ready status bit"]
        #[inline(always)]
        pub fn set_dacrdy(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 11usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel output register status bit"]
        #[inline(always)]
        pub const fn dorstat(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel output register status bit"]
        #[inline(always)]
        pub fn set_dorstat(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel DMA underrun flag"]
        #[inline(always)]
        pub const fn dmaudr(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 13usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel DMA underrun flag"]
        #[inline(always)]
        pub fn set_dmaudr(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 13usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel calibration offset status"]
        #[inline(always)]
        pub const fn cal_flag(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 14usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel calibration offset status"]
        #[inline(always)]
        pub fn set_cal_flag(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 14usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel busy writing sample time flag"]
        #[inline(always)]
        pub const fn bwst(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 15usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel busy writing sample time flag"]
        #[inline(always)]
        pub fn set_bwst(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 15usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("dacrdy[0]", &self.dacrdy(0usize))
                .field("dacrdy[1]", &self.dacrdy(1usize))
                .field("dorstat[0]", &self.dorstat(0usize))
                .field("dorstat[1]", &self.dorstat(1usize))
                .field("dmaudr[0]", &self.dmaudr(0usize))
                .field("dmaudr[1]", &self.dmaudr(1usize))
                .field("cal_flag[0]", &self.cal_flag(0usize))
                .field("cal_flag[1]", &self.cal_flag(1usize))
                .field("bwst[0]", &self.bwst(0usize))
                .field("bwst[1]", &self.bwst(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ dacrdy[0]: {=bool:?}, dacrdy[1]: {=bool:?}, dorstat[0]: {=bool:?}, dorstat[1]: {=bool:?}, dmaudr[0]: {=bool:?}, dmaudr[1]: {=bool:?}, cal_flag[0]: {=bool:?}, cal_flag[1]: {=bool:?}, bwst[0]: {=bool:?}, bwst[1]: {=bool:?} }}" , self . dacrdy (0usize) , self . dacrdy (1usize) , self . dorstat (0usize) , self . dorstat (1usize) , self . dmaudr (0usize) , self . dmaudr (1usize) , self . cal_flag (0usize) , self . cal_flag (1usize) , self . bwst (0usize) , self . bwst (1usize))
        }
    }
    #[doc = "sawtooth mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stmodr(pub u32);
    impl Stmodr {
        #[doc = "channel sawtooth reset trigger selection"]
        #[inline(always)]
        pub const fn strsttrigsel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "channel sawtooth reset trigger selection"]
        #[inline(always)]
        pub fn set_strsttrigsel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "channel sawtooth increment trigger selection"]
        #[inline(always)]
        pub const fn stinctrigsel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "channel sawtooth increment trigger selection"]
        #[inline(always)]
        pub fn set_stinctrigsel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Stmodr {
        #[inline(always)]
        fn default() -> Stmodr {
            Stmodr(0)
        }
    }
    impl core::fmt::Debug for Stmodr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stmodr")
                .field("strsttrigsel[0]", &self.strsttrigsel(0usize))
                .field("strsttrigsel[1]", &self.strsttrigsel(1usize))
                .field("stinctrigsel[0]", &self.stinctrigsel(0usize))
                .field("stinctrigsel[1]", &self.stinctrigsel(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Stmodr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Stmodr {{ strsttrigsel[0]: {=u8:?}, strsttrigsel[1]: {=u8:?}, stinctrigsel[0]: {=u8:?}, stinctrigsel[1]: {=u8:?} }}" , self . strsttrigsel (0usize) , self . strsttrigsel (1usize) , self . stinctrigsel (0usize) , self . stinctrigsel (1usize))
        }
    }
    #[doc = "sawtooth register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Str(pub u32);
    impl Str {
        #[doc = "channel sawtooth reset value."]
        #[inline(always)]
        pub const fn rstdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel sawtooth reset value."]
        #[inline(always)]
        pub fn set_rstdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "channel sawtooth direction setting"]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "channel sawtooth direction setting"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "channel sawtooth increment value (12.4 bit format)"]
        #[inline(always)]
        pub const fn incdata(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "channel sawtooth increment value (12.4 bit format)"]
        #[inline(always)]
        pub fn set_incdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Str {
        #[inline(always)]
        fn default() -> Str {
            Str(0)
        }
    }
    impl core::fmt::Debug for Str {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Str")
                .field("rstdata", &self.rstdata())
                .field("dir", &self.dir())
                .field("incdata", &self.incdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Str {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Str {{ rstdata: {=u16:?}, dir: {=bool:?}, incdata: {=u16:?} }}",
                self.rstdata(),
                self.dir(),
                self.incdata()
            )
        }
    }
    #[doc = "software trigger register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swtrigr(pub u32);
    impl Swtrigr {
        #[doc = "channel software trigger"]
        #[inline(always)]
        pub const fn swtrig(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel software trigger"]
        #[inline(always)]
        pub fn set_swtrig(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel software trigger B"]
        #[inline(always)]
        pub const fn swtrigb(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel software trigger B"]
        #[inline(always)]
        pub fn set_swtrigb(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swtrigr {
        #[inline(always)]
        fn default() -> Swtrigr {
            Swtrigr(0)
        }
    }
    impl core::fmt::Debug for Swtrigr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swtrigr")
                .field("swtrig[0]", &self.swtrig(0usize))
                .field("swtrig[1]", &self.swtrig(1usize))
                .field("swtrigb[0]", &self.swtrigb(0usize))
                .field("swtrigb[1]", &self.swtrigb(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swtrigr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Swtrigr {{ swtrig[0]: {=bool:?}, swtrig[1]: {=bool:?}, swtrigb[0]: {=bool:?}, swtrigb[1]: {=bool:?} }}" , self . swtrig (0usize) , self . swtrig (1usize) , self . swtrigb (0usize) , self . swtrigb (1usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Normal mode, external pin only, buffer enabled"]
        NORMAL_EXT_BUFEN = 0x0,
        #[doc = "Normal mode, external pin and internal peripherals, buffer enabled"]
        NORMAL_EXT_INT_BUFEN = 0x01,
        #[doc = "Normal mode, external pin only, buffer disabled"]
        NORMAL_EXT_BUFDIS = 0x02,
        #[doc = "Normal mode, internal peripherals only, buffer disabled"]
        NORMAL_INT_BUFDIS = 0x03,
        #[doc = "Sample and hold mode, external pin only, buffer enabled"]
        SAMPHOLD_EXT_BUFEN = 0x04,
        #[doc = "Sample and hold mode, external pin and internal peripherals, buffer enabled"]
        SAMPHOLD_EXT_INT_BUFEN = 0x05,
        #[doc = "Sample and hold mode, external pin and internal peripherals, buffer disabled"]
        SAMPHOLD_EXT_INT_BUFDIS = 0x06,
        #[doc = "Sample and hold mode, internal peripherals only, buffer disabled"]
        SAMPHOLD_INT_BUFDIS = 0x07,
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
    pub enum Wave {
        #[doc = "Wave generation disabled"]
        DISABLED = 0x0,
        #[doc = "Noise wave generation enabled"]
        NOISE = 0x01,
        #[doc = "Triangle wave generation enabled"]
        TRIANGLE = 0x02,
        #[doc = "Sawtooth wave generation enabled"]
        SAWTOOTH = 0x03,
    }
    impl Wave {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wave {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wave {
        #[inline(always)]
        fn from(val: u8) -> Wave {
            Wave::from_bits(val)
        }
    }
    impl From<Wave> for u8 {
        #[inline(always)]
        fn from(val: Wave) -> u8 {
            Wave::to_bits(val)
        }
    }
}
