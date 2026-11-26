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
}
pub mod regs {
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
        #[doc = "channel output buffer disable"]
        #[inline(always)]
        pub const fn boff(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel output buffer disable"]
        #[inline(always)]
        pub fn set_boff(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel trigger enable"]
        #[inline(always)]
        pub const fn ten(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel trigger enable"]
        #[inline(always)]
        pub fn set_ten(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel trigger selection"]
        #[inline(always)]
        pub const fn tsel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 3usize + n * 16usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "channel trigger selection"]
        #[inline(always)]
        pub fn set_tsel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 3usize + n * 16usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
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
                .field("boff[0]", &self.boff(0usize))
                .field("boff[1]", &self.boff(1usize))
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en[0]: {=bool:?}, en[1]: {=bool:?}, boff[0]: {=bool:?}, boff[1]: {=bool:?}, ten[0]: {=bool:?}, ten[1]: {=bool:?}, tsel[0]: {=u8:?}, tsel[1]: {=u8:?}, wave[0]: {:?}, wave[1]: {:?}, mamp[0]: {=u8:?}, mamp[1]: {=u8:?}, dmaen[0]: {=bool:?}, dmaen[1]: {=bool:?} }}" , self . en (0usize) , self . en (1usize) , self . boff (0usize) , self . boff (1usize) , self . ten (0usize) , self . ten (1usize) , self . tsel (0usize) , self . tsel (1usize) , self . wave (0usize) , self . wave (1usize) , self . mamp (0usize) , self . mamp (1usize) , self . dmaen (0usize) , self . dmaen (1usize))
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
    }
    impl Default for Dhr12l {
        #[inline(always)]
        fn default() -> Dhr12l {
            Dhr12l(0)
        }
    }
    impl core::fmt::Debug for Dhr12l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12l").field("dhr", &self.dhr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12l {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr12l {{ dhr: {=u16:?} }}", self.dhr())
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
    }
    impl Default for Dhr12r {
        #[inline(always)]
        fn default() -> Dhr12r {
            Dhr12r(0)
        }
    }
    impl core::fmt::Debug for Dhr12r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12r").field("dhr", &self.dhr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr12r {{ dhr: {=u16:?} }}", self.dhr())
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
    }
    impl Default for Dhr8r {
        #[inline(always)]
        fn default() -> Dhr8r {
            Dhr8r(0)
        }
    }
    impl core::fmt::Debug for Dhr8r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr8r").field("dhr", &self.dhr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr8r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr8r {{ dhr: {=u8:?} }}", self.dhr())
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
    }
    impl Default for Dor {
        #[inline(always)]
        fn default() -> Dor {
            Dor(0)
        }
    }
    impl core::fmt::Debug for Dor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dor").field("dor", &self.dor()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dor {{ dor: {=u16:?} }}", self.dor())
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swtrigr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swtrigr {{ swtrig[0]: {=bool:?}, swtrig[1]: {=bool:?} }}",
                self.swtrig(0usize),
                self.swtrig(1usize)
            )
        }
    }
}
pub mod vals {
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
        _RESERVED_3 = 0x03,
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
