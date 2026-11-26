#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Random number generator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng {
    ptr: *mut u8,
}
unsafe impl Send for Rng {}
unsafe impl Sync for Rng {}
impl Rng {
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
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RNG noise source control register."]
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::Nscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "health test control register"]
    #[inline(always)]
    pub const fn htcr(self, n: usize) -> crate::common::Reg<regs::Htcr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Random number generator enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt enable"]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable"]
        #[inline(always)]
        pub fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock error detection"]
        #[inline(always)]
        pub const fn ced(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error detection"]
        #[inline(always)]
        pub fn set_ced(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Auto reset disable"]
        #[inline(always)]
        pub const fn ardis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto reset disable"]
        #[inline(always)]
        pub fn set_ardis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RNG configuration 3"]
        #[inline(always)]
        pub const fn rng_config3(&self) -> super::vals::RngConfig3 {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::RngConfig3::from_bits(val as u8)
        }
        #[doc = "RNG configuration 3"]
        #[inline(always)]
        pub fn set_rng_config3(&mut self, val: super::vals::RngConfig3) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "Non NIST compliant"]
        #[inline(always)]
        pub const fn nistc(&self) -> super::vals::Nistc {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Nistc::from_bits(val as u8)
        }
        #[doc = "Non NIST compliant"]
        #[inline(always)]
        pub fn set_nistc(&mut self, val: super::vals::Nistc) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "RNG configuration 2"]
        #[inline(always)]
        pub const fn rng_config2(&self) -> super::vals::RngConfig2 {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::RngConfig2::from_bits(val as u8)
        }
        #[doc = "RNG configuration 2"]
        #[inline(always)]
        pub fn set_rng_config2(&mut self, val: super::vals::RngConfig2) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "Clock divider factor"]
        #[inline(always)]
        pub const fn clkdiv(&self) -> super::vals::Clkdiv {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Clkdiv::from_bits(val as u8)
        }
        #[doc = "Clock divider factor"]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: super::vals::Clkdiv) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "RNG configuration 1"]
        #[inline(always)]
        pub const fn rng_config1(&self) -> super::vals::RngConfig1 {
            let val = (self.0 >> 20usize) & 0xff;
            super::vals::RngConfig1::from_bits(val as u8)
        }
        #[doc = "RNG configuration 1"]
        #[inline(always)]
        pub fn set_rng_config1(&mut self, val: super::vals::RngConfig1) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
        }
        #[doc = "Conditioning soft reset"]
        #[inline(always)]
        pub const fn condrst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Conditioning soft reset"]
        #[inline(always)]
        pub fn set_condrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Config Lock"]
        #[inline(always)]
        pub const fn configlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Config Lock"]
        #[inline(always)]
        pub fn set_configlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("rngen", &self.rngen())
                .field("ie", &self.ie())
                .field("ced", &self.ced())
                .field("ardis", &self.ardis())
                .field("rng_config3", &self.rng_config3())
                .field("nistc", &self.nistc())
                .field("rng_config2", &self.rng_config2())
                .field("clkdiv", &self.clkdiv())
                .field("rng_config1", &self.rng_config1())
                .field("condrst", &self.condrst())
                .field("configlock", &self.configlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ rngen: {=bool:?}, ie: {=bool:?}, ced: {=bool:?}, ardis: {=bool:?}, rng_config3: {:?}, nistc: {:?}, rng_config2: {:?}, clkdiv: {:?}, rng_config1: {:?}, condrst: {=bool:?}, configlock: {=bool:?} }}" , self . rngen () , self . ie () , self . ced () , self . ardis () , self . rng_config3 () , self . nistc () , self . rng_config2 () , self . clkdiv () , self . rng_config1 () , self . condrst () , self . configlock ())
        }
    }
    #[doc = "Health test control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htcr(pub u32);
    impl Htcr {
        #[doc = "Health test configuration"]
        #[inline(always)]
        pub const fn htcfg(&self) -> super::vals::Htcfg {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Htcfg::from_bits(val as u32)
        }
        #[doc = "Health test configuration"]
        #[inline(always)]
        pub fn set_htcfg(&mut self, val: super::vals::Htcfg) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Htcr {
        #[inline(always)]
        fn default() -> Htcr {
            Htcr(0)
        }
    }
    impl core::fmt::Debug for Htcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Htcr").field("htcfg", &self.htcfg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Htcr {{ htcfg: {:?} }}", self.htcfg())
        }
    }
    #[doc = "RNG noise source control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr(pub u32);
    impl Nscr {
        #[doc = "When the RNG is enabled (RNGEN bit set), each bit of this bit field enables one of the three inputs from the oscillator instance number X."]
        #[inline(always)]
        pub const fn en_osc(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "When the RNG is enabled (RNGEN bit set), each bit of this bit field enables one of the three inputs from the oscillator instance number X."]
        #[inline(always)]
        pub fn set_en_osc(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
    }
    impl Default for Nscr {
        #[inline(always)]
        fn default() -> Nscr {
            Nscr(0)
        }
    }
    impl core::fmt::Debug for Nscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nscr")
                .field("en_osc[0]", &self.en_osc(0usize))
                .field("en_osc[1]", &self.en_osc(1usize))
                .field("en_osc[2]", &self.en_osc(2usize))
                .field("en_osc[3]", &self.en_osc(3usize))
                .field("en_osc[4]", &self.en_osc(4usize))
                .field("en_osc[5]", &self.en_osc(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nscr {{ en_osc[0]: {=u8:?}, en_osc[1]: {=u8:?}, en_osc[2]: {=u8:?}, en_osc[3]: {=u8:?}, en_osc[4]: {=u8:?}, en_osc[5]: {=u8:?} }}" , self . en_osc (0usize) , self . en_osc (1usize) , self . en_osc (2usize) , self . en_osc (3usize) , self . en_osc (4usize) , self . en_osc (5usize))
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Data ready"]
        #[inline(always)]
        pub const fn drdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data ready"]
        #[inline(always)]
        pub fn set_drdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock error current status"]
        #[inline(always)]
        pub const fn cecs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error current status"]
        #[inline(always)]
        pub fn set_cecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Seed error current status"]
        #[inline(always)]
        pub const fn secs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Seed error current status"]
        #[inline(always)]
        pub fn set_secs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Busy status"]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Busy status"]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clock error interrupt status"]
        #[inline(always)]
        pub const fn ceis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error interrupt status"]
        #[inline(always)]
        pub fn set_ceis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Seed error interrupt status"]
        #[inline(always)]
        pub const fn seis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Seed error interrupt status"]
        #[inline(always)]
        pub fn set_seis(&mut self, val: bool) {
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
                .field("drdy", &self.drdy())
                .field("cecs", &self.cecs())
                .field("secs", &self.secs())
                .field("busy", &self.busy())
                .field("ceis", &self.ceis())
                .field("seis", &self.seis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ drdy: {=bool:?}, cecs: {=bool:?}, secs: {=bool:?}, busy: {=bool:?}, ceis: {=bool:?}, seis: {=bool:?} }}" , self . drdy () , self . cecs () , self . secs () , self . busy () , self . ceis () , self . seis ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clkdiv {
        #[doc = "Internal RNG clock after divider is similar to incoming RNG clock"]
        NO_DIV = 0x0,
        #[doc = "Divide RNG clock by 2^1"]
        DIV_2_1 = 0x01,
        #[doc = "Divide RNG clock by 2^2"]
        DIV_2_2 = 0x02,
        #[doc = "Divide RNG clock by 2^3"]
        DIV_2_3 = 0x03,
        #[doc = "Divide RNG clock by 2^4"]
        DIV_2_4 = 0x04,
        #[doc = "Divide RNG clock by 2^5"]
        DIV_2_5 = 0x05,
        #[doc = "Divide RNG clock by 2^6"]
        DIV_2_6 = 0x06,
        #[doc = "Divide RNG clock by 2^7"]
        DIV_2_7 = 0x07,
        #[doc = "Divide RNG clock by 2^8"]
        DIV_2_8 = 0x08,
        #[doc = "Divide RNG clock by 2^9"]
        DIV_2_9 = 0x09,
        #[doc = "Divide RNG clock by 2^10"]
        DIV_2_10 = 0x0a,
        #[doc = "Divide RNG clock by 2^11"]
        DIV_2_11 = 0x0b,
        #[doc = "Divide RNG clock by 2^12"]
        DIV_2_12 = 0x0c,
        #[doc = "Divide RNG clock by 2^13"]
        DIV_2_13 = 0x0d,
        #[doc = "Divide RNG clock by 2^14"]
        DIV_2_14 = 0x0e,
        #[doc = "Divide RNG clock by 2^15"]
        DIV_2_15 = 0x0f,
    }
    impl Clkdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clkdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clkdiv {
        #[inline(always)]
        fn from(val: u8) -> Clkdiv {
            Clkdiv::from_bits(val)
        }
    }
    impl From<Clkdiv> for u8 {
        #[inline(always)]
        fn from(val: Clkdiv) -> u8 {
            Clkdiv::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htcfg(u32);
    impl Htcfg {
        #[doc = "Recommended value for RNG certification (0x0000_6688)"]
        pub const CONFIG_A: Self = Self(0x6688);
        #[doc = "Recommended value for config B and C (not NIST certifiable) (0x0000_AAC7)"]
        pub const CONFIG_B_C: Self = Self(0xaacf);
        #[doc = "Magic number to be written before any write (0x1759_0ABC)"]
        pub const MAGIC: Self = Self(0x1759_0abc);
    }
    impl Htcfg {
        pub const fn from_bits(val: u32) -> Htcfg {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Htcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x6688 => f.write_str("CONFIG_A"),
                0xaacf => f.write_str("CONFIG_B_C"),
                0x1759_0abc => f.write_str("MAGIC"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htcfg {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x6688 => defmt::write!(f, "CONFIG_A"),
                0xaacf => defmt::write!(f, "CONFIG_B_C"),
                0x1759_0abc => defmt::write!(f, "MAGIC"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Htcfg {
        #[inline(always)]
        fn from(val: u32) -> Htcfg {
            Htcfg::from_bits(val)
        }
    }
    impl From<Htcfg> for u32 {
        #[inline(always)]
        fn from(val: Htcfg) -> u32 {
            Htcfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nistc {
        #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used"]
        DEFAULT = 0x0,
        #[doc = "Custom values for NIST compliant RNG"]
        CUSTOM = 0x01,
    }
    impl Nistc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nistc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nistc {
        #[inline(always)]
        fn from(val: u8) -> Nistc {
            Nistc::from_bits(val)
        }
    }
    impl From<Nistc> for u8 {
        #[inline(always)]
        fn from(val: Nistc) -> u8 {
            Nistc::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RngConfig1(u8);
    impl RngConfig1 {
        #[doc = "Recommended value for config B (not NIST certifiable)"]
        pub const CONFIG_B: Self = Self(0x01);
        #[doc = "Recommended value for config A (NIST certifiable)"]
        pub const CONFIG_A: Self = Self(0x0f);
        #[doc = "Recommended value for config C (not NIST certifiable) (0x0000_82)"]
        pub const CONFIG_C: Self = Self(0x82);
    }
    impl RngConfig1 {
        pub const fn from_bits(val: u8) -> RngConfig1 {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for RngConfig1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x01 => f.write_str("CONFIG_B"),
                0x0f => f.write_str("CONFIG_A"),
                0x82 => f.write_str("CONFIG_C"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngConfig1 {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x01 => defmt::write!(f, "CONFIG_B"),
                0x0f => defmt::write!(f, "CONFIG_A"),
                0x82 => defmt::write!(f, "CONFIG_C"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for RngConfig1 {
        #[inline(always)]
        fn from(val: u8) -> RngConfig1 {
            RngConfig1::from_bits(val)
        }
    }
    impl From<RngConfig1> for u8 {
        #[inline(always)]
        fn from(val: RngConfig1) -> u8 {
            RngConfig1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RngConfig2 {
        #[doc = "Recommended value for config A and B"]
        CONFIG_A_B = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl RngConfig2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RngConfig2 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RngConfig2 {
        #[inline(always)]
        fn from(val: u8) -> RngConfig2 {
            RngConfig2::from_bits(val)
        }
    }
    impl From<RngConfig2> for u8 {
        #[inline(always)]
        fn from(val: RngConfig2) -> u8 {
            RngConfig2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RngConfig3 {
        #[doc = "Recommended value for config B (not NIST certifiable)"]
        CONFIG_B = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "Recommended value for config A (NIST certifiable)"]
        CONFIG_A = 0x0f,
    }
    impl RngConfig3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RngConfig3 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RngConfig3 {
        #[inline(always)]
        fn from(val: u8) -> RngConfig3 {
            RngConfig3::from_bits(val)
        }
    }
    impl From<RngConfig3> for u8 {
        #[inline(always)]
        fn from(val: RngConfig3) -> u8 {
            RngConfig3::to_bits(val)
        }
    }
}
