#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Factory programmed device identification registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Desig {
    ptr: *mut u8,
}
unsafe impl Send for Desig {}
unsafe impl Sync for Desig {}
impl Desig {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "package data register"]
    #[inline(always)]
    pub const fn pkgr(self) -> crate::common::Reg<regs::Pkgr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "temperature calibration 1 register"]
    #[inline(always)]
    pub const fn tscal1r(self) -> crate::common::Reg<regs::Tscal1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "temperature calibration 2 register"]
    #[inline(always)]
    pub const fn tscal2r(self) -> crate::common::Reg<regs::Tscal2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[doc = "FLASH size data register"]
    #[inline(always)]
    pub const fn flashsizer(self) -> crate::common::Reg<regs::Flashsizer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[doc = "voltage reference buffer calibration register"]
    #[inline(always)]
    pub const fn vrefbufcalr(self) -> crate::common::Reg<regs::Vrefbufcalr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a8usize) as _) }
    }
    #[doc = "resistor calibration register"]
    #[inline(always)]
    pub const fn rcalr(self) -> crate::common::Reg<regs::Rcalr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f0usize) as _) }
    }
    #[doc = "radio gain calibration register"]
    #[inline(always)]
    pub const fn rfgaincalr(self) -> crate::common::Reg<regs::Rfgaincalr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f4usize) as _) }
    }
    #[doc = "IEEE 64-bit unique device ID register 1"]
    #[inline(always)]
    pub const fn uid64r1(self) -> crate::common::Reg<regs::Uid64r1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "IEEE 64-bit unique device ID register 2"]
    #[inline(always)]
    pub const fn uid64r2(self) -> crate::common::Reg<regs::Uid64r2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
}
pub mod regs {
    #[doc = "FLASH size data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flashsizer(pub u32);
    impl Flashsizer {
        #[doc = "Flash memory size in Kbytes"]
        #[inline(always)]
        pub const fn flash_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Flash memory size in Kbytes"]
        #[inline(always)]
        pub fn set_flash_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "RAM memory size in Kbytes"]
        #[inline(always)]
        pub const fn ram_size(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "RAM memory size in Kbytes"]
        #[inline(always)]
        pub fn set_ram_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Flashsizer {
        #[inline(always)]
        fn default() -> Flashsizer {
            Flashsizer(0)
        }
    }
    impl core::fmt::Debug for Flashsizer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flashsizer")
                .field("flash_size", &self.flash_size())
                .field("ram_size", &self.ram_size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flashsizer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Flashsizer {{ flash_size: {=u16:?}, ram_size: {=u16:?} }}",
                self.flash_size(),
                self.ram_size()
            )
        }
    }
    #[doc = "package data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pkgr(pub u32);
    impl Pkgr {
        #[doc = "Package type"]
        #[inline(always)]
        pub const fn pkg(&self) -> super::vals::Pkg {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Pkg::from_bits(val as u8)
        }
        #[doc = "Package type"]
        #[inline(always)]
        pub fn set_pkg(&mut self, val: super::vals::Pkg) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "Reserved, must be kept at reset value"]
        #[inline(always)]
        pub const fn reserved(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "Reserved, must be kept at reset value"]
        #[inline(always)]
        pub fn set_reserved(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Pkgr {
        #[inline(always)]
        fn default() -> Pkgr {
            Pkgr(0)
        }
    }
    impl core::fmt::Debug for Pkgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pkgr")
                .field("pkg", &self.pkg())
                .field("reserved", &self.reserved())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pkgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pkgr {{ pkg: {:?}, reserved: {=u32:?} }}",
                self.pkg(),
                self.reserved()
            )
        }
    }
    #[doc = "resistor calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcalr(pub u32);
    impl Rcalr {
        #[doc = "Resistor calibration value"]
        #[inline(always)]
        pub const fn r_cal(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Resistor calibration value"]
        #[inline(always)]
        pub fn set_r_cal(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rcalr {
        #[inline(always)]
        fn default() -> Rcalr {
            Rcalr(0)
        }
    }
    impl core::fmt::Debug for Rcalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcalr").field("r_cal", &self.r_cal()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rcalr {{ r_cal: {=u8:?} }}", self.r_cal())
        }
    }
    #[doc = "radio gain calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfgaincalr(pub u32);
    impl Rfgaincalr {
        #[doc = "Radio gain 1 calibration value"]
        #[inline(always)]
        pub const fn gain1_cal(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Radio gain 1 calibration value"]
        #[inline(always)]
        pub fn set_gain1_cal(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Radio gain 2 calibration value"]
        #[inline(always)]
        pub const fn gain2_cal(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Radio gain 2 calibration value"]
        #[inline(always)]
        pub fn set_gain2_cal(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Radio gain 3 calibration value"]
        #[inline(always)]
        pub const fn gain3_cal(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Radio gain 3 calibration value"]
        #[inline(always)]
        pub fn set_gain3_cal(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Radio gain 4 calibration value"]
        #[inline(always)]
        pub const fn gain4_cal(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Radio gain 4 calibration value"]
        #[inline(always)]
        pub fn set_gain4_cal(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rfgaincalr {
        #[inline(always)]
        fn default() -> Rfgaincalr {
            Rfgaincalr(0)
        }
    }
    impl core::fmt::Debug for Rfgaincalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfgaincalr")
                .field("gain1_cal", &self.gain1_cal())
                .field("gain2_cal", &self.gain2_cal())
                .field("gain3_cal", &self.gain3_cal())
                .field("gain4_cal", &self.gain4_cal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfgaincalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfgaincalr {{ gain1_cal: {=u8:?}, gain2_cal: {=u8:?}, gain3_cal: {=u8:?}, gain4_cal: {=u8:?} }}",
                self.gain1_cal(),
                self.gain2_cal(),
                self.gain3_cal(),
                self.gain4_cal()
            )
        }
    }
    #[doc = "temperature calibration 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscal1r(pub u32);
    impl Tscal1r {
        #[doc = "Factory temperature sensor calibration 1 value for ADC4"]
        #[inline(always)]
        pub const fn ts_cal1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Factory temperature sensor calibration 1 value for ADC4"]
        #[inline(always)]
        pub fn set_ts_cal1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Tscal1r {
        #[inline(always)]
        fn default() -> Tscal1r {
            Tscal1r(0)
        }
    }
    impl core::fmt::Debug for Tscal1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tscal1r").field("ts_cal1", &self.ts_cal1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tscal1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tscal1r {{ ts_cal1: {=u16:?} }}", self.ts_cal1())
        }
    }
    #[doc = "temperature calibration 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscal2r(pub u32);
    impl Tscal2r {
        #[doc = "Factory temperature sensor calibration 2 value for ADC4"]
        #[inline(always)]
        pub const fn ts_cal2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Factory temperature sensor calibration 2 value for ADC4"]
        #[inline(always)]
        pub fn set_ts_cal2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Tscal2r {
        #[inline(always)]
        fn default() -> Tscal2r {
            Tscal2r(0)
        }
    }
    impl core::fmt::Debug for Tscal2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tscal2r").field("ts_cal2", &self.ts_cal2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tscal2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tscal2r {{ ts_cal2: {=u16:?} }}", self.ts_cal2())
        }
    }
    #[doc = "IEEE 64-bit unique device ID register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uid64r1(pub u32);
    impl Uid64r1 {
        #[doc = "Device number"]
        #[inline(always)]
        pub const fn devnum(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device number"]
        #[inline(always)]
        pub fn set_devnum(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uid64r1 {
        #[inline(always)]
        fn default() -> Uid64r1 {
            Uid64r1(0)
        }
    }
    impl core::fmt::Debug for Uid64r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uid64r1").field("devnum", &self.devnum()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Uid64r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Uid64r1 {{ devnum: {=u32:?} }}", self.devnum())
        }
    }
    #[doc = "IEEE 64-bit unique device ID register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uid64r2(pub u32);
    impl Uid64r2 {
        #[doc = "Device ID"]
        #[inline(always)]
        pub const fn devid(&self) -> super::vals::Devid {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Devid::from_bits(val as u8)
        }
        #[doc = "Device ID"]
        #[inline(always)]
        pub fn set_devid(&mut self, val: super::vals::Devid) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Company ID"]
        #[inline(always)]
        pub const fn stid(&self) -> super::vals::Stid {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            super::vals::Stid::from_bits(val as u32)
        }
        #[doc = "Company ID"]
        #[inline(always)]
        pub fn set_stid(&mut self, val: super::vals::Stid) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val.to_bits() as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Uid64r2 {
        #[inline(always)]
        fn default() -> Uid64r2 {
            Uid64r2(0)
        }
    }
    impl core::fmt::Debug for Uid64r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uid64r2")
                .field("devid", &self.devid())
                .field("stid", &self.stid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Uid64r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Uid64r2 {{ devid: {:?}, stid: {:?} }}", self.devid(), self.stid())
        }
    }
    #[doc = "voltage reference buffer calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vrefbufcalr(pub u32);
    impl Vrefbufcalr {
        #[doc = "VREFBUF0 factory voltage reference buffer calibration value"]
        #[inline(always)]
        pub const fn vrefbuf0_trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "VREFBUF0 factory voltage reference buffer calibration value"]
        #[inline(always)]
        pub fn set_vrefbuf0_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "VREFBUF1 factory voltage reference buffer calibration value"]
        #[inline(always)]
        pub const fn vrefbuf1_trim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "VREFBUF1 factory voltage reference buffer calibration value"]
        #[inline(always)]
        pub fn set_vrefbuf1_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "VREFBUF2 factory voltage reference buffer calibration value"]
        #[inline(always)]
        pub const fn vrefbuf2_trim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "VREFBUF2 factory voltage reference buffer calibration value"]
        #[inline(always)]
        pub fn set_vrefbuf2_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "VREFBUF3 factory voltage reference buffer calibration value"]
        #[inline(always)]
        pub const fn vrefbuf3_trim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "VREFBUF3 factory voltage reference buffer calibration value"]
        #[inline(always)]
        pub fn set_vrefbuf3_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Vrefbufcalr {
        #[inline(always)]
        fn default() -> Vrefbufcalr {
            Vrefbufcalr(0)
        }
    }
    impl core::fmt::Debug for Vrefbufcalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vrefbufcalr")
                .field("vrefbuf0_trim", &self.vrefbuf0_trim())
                .field("vrefbuf1_trim", &self.vrefbuf1_trim())
                .field("vrefbuf2_trim", &self.vrefbuf2_trim())
                .field("vrefbuf3_trim", &self.vrefbuf3_trim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vrefbufcalr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vrefbufcalr {{ vrefbuf0_trim: {=u8:?}, vrefbuf1_trim: {=u8:?}, vrefbuf2_trim: {=u8:?}, vrefbuf3_trim: {=u8:?} }}" , self . vrefbuf0_trim () , self . vrefbuf1_trim () , self . vrefbuf2_trim () , self . vrefbuf3_trim ())
        }
    }
}
pub mod vals {
    #[doc = "IEEE device ID encoding for STM32WBA devices"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Devid(u8);
    impl Devid {
        #[doc = "STM32WBA5xxx"]
        pub const STM32WBA5: Self = Self(0x2a);
        #[doc = "STM32WBA6xxx"]
        pub const STM32WBA6: Self = Self(0x2c);
    }
    impl Devid {
        pub const fn from_bits(val: u8) -> Devid {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Devid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x2a => f.write_str("STM32WBA5"),
                0x2c => f.write_str("STM32WBA6"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Devid {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x2a => defmt::write!(f, "STM32WBA5"),
                0x2c => defmt::write!(f, "STM32WBA6"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Devid {
        #[inline(always)]
        fn from(val: u8) -> Devid {
            Devid::from_bits(val)
        }
    }
    impl From<Devid> for u8 {
        #[inline(always)]
        fn from(val: Devid) -> u8 {
            Devid::to_bits(val)
        }
    }
    #[doc = "Package encoding for STM32WBA devices"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pkg {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "UFQFPN48 USB"]
        UFQFPN48_USB = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "WLSCP88 USB"]
        WLSCP88_USB = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "UFBGA121 USB"]
        UFBGA121_USB = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        #[doc = "UFQFPN48 SMPS"]
        UFQFPN48_SMPS = 0x0a,
        #[doc = "UFQFPN48 SMPS USB"]
        UFQFPN48_SMPS_USB = 0x0b,
        #[doc = "VFQFPN68 SMPS USB"]
        VFQFPN68_SMPS_USB = 0x0c,
        #[doc = "WLSCP88 SMPS USB"]
        WLSCP88_SMPS_USB = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "UFBGA121 SMPS USB"]
        UFBGA121_SMPS_USB = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Pkg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pkg {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pkg {
        #[inline(always)]
        fn from(val: u8) -> Pkg {
            Pkg::from_bits(val)
        }
    }
    impl From<Pkg> for u8 {
        #[inline(always)]
        fn from(val: Pkg) -> u8 {
            Pkg::to_bits(val)
        }
    }
    #[doc = "IEEE company ID assignment"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stid(u32);
    impl Stid {
        #[doc = "STMicroelectronics"]
        pub const STMICROELECTRONICS: Self = Self(0x80e1);
    }
    impl Stid {
        pub const fn from_bits(val: u32) -> Stid {
            Self(val & 0x00ff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Stid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x80e1 => f.write_str("STMICROELECTRONICS"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Stid {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x80e1 => defmt::write!(f, "STMICROELECTRONICS"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Stid {
        #[inline(always)]
        fn from(val: u32) -> Stid {
            Stid::from_bits(val)
        }
    }
    impl From<Stid> for u32 {
        #[inline(always)]
        fn from(val: Stid) -> u32 {
            Stid::to_bits(val)
        }
    }
}
