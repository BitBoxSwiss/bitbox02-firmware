#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Liquid crystal display controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcd {
    ptr: *mut u8,
}
unsafe impl Send for Lcd {}
unsafe impl Sync for Lcd {}
impl Lcd {
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
    #[doc = "frame control register"]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "clear register"]
    #[inline(always)]
    pub const fn clr(self) -> crate::common::Reg<regs::Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "display memory"]
    #[inline(always)]
    pub const fn ram_com(self, n: usize) -> RamCom {
        assert!(n < 8usize);
        unsafe { RamCom::from_ptr(self.ptr.add(0x14usize + n * 8usize) as _) }
    }
}
#[doc = "display memory"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamCom {
    ptr: *mut u8,
}
unsafe impl Send for RamCom {}
unsafe impl Sync for RamCom {}
impl RamCom {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "display memory low word"]
    #[inline(always)]
    pub const fn low(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "display memory high word"]
    #[inline(always)]
    pub const fn high(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clr(pub u32);
    impl Clr {
        #[doc = "Start of frame flag clear"]
        #[inline(always)]
        pub const fn sofc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of frame flag clear"]
        #[inline(always)]
        pub fn set_sofc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update display done clear"]
        #[inline(always)]
        pub const fn uddc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Update display done clear"]
        #[inline(always)]
        pub fn set_uddc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Clr {
        #[inline(always)]
        fn default() -> Clr {
            Clr(0)
        }
    }
    impl core::fmt::Debug for Clr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clr")
                .field("sofc", &self.sofc())
                .field("uddc", &self.uddc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clr {{ sofc: {=bool:?}, uddc: {=bool:?} }}",
                self.sofc(),
                self.uddc()
            )
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "LCD controller enable"]
        #[inline(always)]
        pub const fn lcden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LCD controller enable"]
        #[inline(always)]
        pub fn set_lcden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Voltage source selection"]
        #[inline(always)]
        pub const fn vsel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage source selection"]
        #[inline(always)]
        pub fn set_vsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Duty selection"]
        #[inline(always)]
        pub const fn duty(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "Duty selection"]
        #[inline(always)]
        pub fn set_duty(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "Bias selector"]
        #[inline(always)]
        pub const fn bias(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Bias selector"]
        #[inline(always)]
        pub fn set_bias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Mux segment enable"]
        #[inline(always)]
        pub const fn mux_seg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Mux segment enable"]
        #[inline(always)]
        pub fn set_mux_seg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("lcden", &self.lcden())
                .field("vsel", &self.vsel())
                .field("duty", &self.duty())
                .field("bias", &self.bias())
                .field("mux_seg", &self.mux_seg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ lcden: {=bool:?}, vsel: {=bool:?}, duty: {=u8:?}, bias: {=u8:?}, mux_seg: {=bool:?} }}",
                self.lcden(),
                self.vsel(),
                self.duty(),
                self.bias(),
                self.mux_seg()
            )
        }
    }
    #[doc = "frame control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "High drive enable"]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "High drive enable"]
        #[inline(always)]
        pub fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start of frame interrupt enable"]
        #[inline(always)]
        pub const fn sofie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of frame interrupt enable"]
        #[inline(always)]
        pub fn set_sofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update display done interrupt enable"]
        #[inline(always)]
        pub const fn uddie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Update display done interrupt enable"]
        #[inline(always)]
        pub fn set_uddie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pulse ON duration"]
        #[inline(always)]
        pub const fn pon(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Pulse ON duration"]
        #[inline(always)]
        pub fn set_pon(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Dead time duration"]
        #[inline(always)]
        pub const fn dead(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x07;
            val as u8
        }
        #[doc = "Dead time duration"]
        #[inline(always)]
        pub fn set_dead(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
        }
        #[doc = "Contrast control"]
        #[inline(always)]
        pub const fn cc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Contrast control"]
        #[inline(always)]
        pub fn set_cc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Blink frequency selection"]
        #[inline(always)]
        pub const fn blinkf(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Blink frequency selection"]
        #[inline(always)]
        pub fn set_blinkf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Blink mode selection"]
        #[inline(always)]
        pub const fn blink(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Blink mode selection"]
        #[inline(always)]
        pub fn set_blink(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "DIV clock divider"]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "DIV clock divider"]
        #[inline(always)]
        pub fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "PS 16-bit prescaler"]
        #[inline(always)]
        pub const fn ps(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "PS 16-bit prescaler"]
        #[inline(always)]
        pub fn set_ps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
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
                .field("hd", &self.hd())
                .field("sofie", &self.sofie())
                .field("uddie", &self.uddie())
                .field("pon", &self.pon())
                .field("dead", &self.dead())
                .field("cc", &self.cc())
                .field("blinkf", &self.blinkf())
                .field("blink", &self.blink())
                .field("div", &self.div())
                .field("ps", &self.ps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fcr {{ hd: {=bool:?}, sofie: {=bool:?}, uddie: {=bool:?}, pon: {=u8:?}, dead: {=u8:?}, cc: {=u8:?}, blinkf: {=u8:?}, blink: {=u8:?}, div: {=u8:?}, ps: {=u8:?} }}" , self . hd () , self . sofie () , self . uddie () , self . pon () , self . dead () , self . cc () , self . blinkf () , self . blink () , self . div () , self . ps ())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "LCD enabled status"]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LCD enabled status"]
        #[inline(always)]
        pub fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start of frame flag"]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of frame flag"]
        #[inline(always)]
        pub fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update display request"]
        #[inline(always)]
        pub const fn udr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Update display request"]
        #[inline(always)]
        pub fn set_udr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Update Display Done"]
        #[inline(always)]
        pub const fn udd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Update Display Done"]
        #[inline(always)]
        pub fn set_udd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Ready flag"]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Ready flag"]
        #[inline(always)]
        pub fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LCD Frame Control Register Synchronization flag"]
        #[inline(always)]
        pub const fn fcrsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LCD Frame Control Register Synchronization flag"]
        #[inline(always)]
        pub fn set_fcrsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("ens", &self.ens())
                .field("sof", &self.sof())
                .field("udr", &self.udr())
                .field("udd", &self.udd())
                .field("rdy", &self.rdy())
                .field("fcrsf", &self.fcrsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ ens: {=bool:?}, sof: {=bool:?}, udr: {=bool:?}, udd: {=bool:?}, rdy: {=bool:?}, fcrsf: {=bool:?} }}" , self . ens () , self . sof () , self . udr () , self . udd () , self . rdy () , self . fcrsf ())
        }
    }
}
