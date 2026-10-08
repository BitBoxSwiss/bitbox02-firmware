#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Real-time clock"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Time register"]
    #[inline(always)]
    pub const fn tr(self) -> crate::common::Reg<regs::Tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Date register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Initialization and status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Prescaler register"]
    #[inline(always)]
    pub const fn prer(self) -> crate::common::Reg<regs::Prer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Wakeup timer register"]
    #[inline(always)]
    pub const fn wutr(self) -> crate::common::Reg<regs::Wutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Calibration register"]
    #[inline(always)]
    pub const fn calibr(self) -> crate::common::Reg<regs::Calibr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Alarm register"]
    #[inline(always)]
    pub const fn alrmr(self, n: usize) -> crate::common::Reg<regs::Alrmr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize + n * 4usize) as _) }
    }
    #[doc = "Write protection register"]
    #[inline(always)]
    pub const fn wpr(self) -> crate::common::Reg<regs::Wpr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Sub second register"]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Shift control register"]
    #[inline(always)]
    pub const fn shiftr(self) -> crate::common::Reg<regs::Shiftr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Timestamp time register"]
    #[inline(always)]
    pub const fn tstr(self) -> crate::common::Reg<regs::Tstr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Timestamp date register"]
    #[inline(always)]
    pub const fn tsdr(self) -> crate::common::Reg<regs::Tsdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Timestamp sub second register"]
    #[inline(always)]
    pub const fn tsssr(self) -> crate::common::Reg<regs::Tsssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Calibration register"]
    #[inline(always)]
    pub const fn calr(self) -> crate::common::Reg<regs::Calr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Tamper and alternate function configuration register"]
    #[inline(always)]
    pub const fn tafcr(self) -> crate::common::Reg<regs::Tafcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Alarm sub second register"]
    #[inline(always)]
    pub const fn alrmssr(self, n: usize) -> crate::common::Reg<regs::Alrmssr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize + n * 4usize) as _) }
    }
    #[doc = "Backup register"]
    #[inline(always)]
    pub const fn bkpr(self, n: usize) -> crate::common::Reg<regs::Bkpr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Alarm register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrmr(pub u32);
    impl Alrmr {
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Alarm seconds mask"]
        #[inline(always)]
        pub const fn msk1(&self) -> super::vals::AlrmrMsk {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::AlrmrMsk::from_bits(val as u8)
        }
        #[doc = "Alarm seconds mask"]
        #[inline(always)]
        pub fn set_msk1(&mut self, val: super::vals::AlrmrMsk) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Alarm minutes mask"]
        #[inline(always)]
        pub const fn msk2(&self) -> super::vals::AlrmrMsk {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::AlrmrMsk::from_bits(val as u8)
        }
        #[doc = "Alarm minutes mask"]
        #[inline(always)]
        pub fn set_msk2(&mut self, val: super::vals::AlrmrMsk) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::AlrmrPm {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::AlrmrPm::from_bits(val as u8)
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: super::vals::AlrmrPm) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Alarm hours mask"]
        #[inline(always)]
        pub const fn msk3(&self) -> super::vals::AlrmrMsk {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::AlrmrMsk::from_bits(val as u8)
        }
        #[doc = "Alarm hours mask"]
        #[inline(always)]
        pub fn set_msk3(&mut self, val: super::vals::AlrmrMsk) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Date units or day in BCD format"]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units or day in BCD format"]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Week day selection"]
        #[inline(always)]
        pub const fn wdsel(&self) -> super::vals::AlrmrWdsel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::AlrmrWdsel::from_bits(val as u8)
        }
        #[doc = "Week day selection"]
        #[inline(always)]
        pub fn set_wdsel(&mut self, val: super::vals::AlrmrWdsel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Alarm date mask"]
        #[inline(always)]
        pub const fn msk4(&self) -> super::vals::AlrmrMsk {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::AlrmrMsk::from_bits(val as u8)
        }
        #[doc = "Alarm date mask"]
        #[inline(always)]
        pub fn set_msk4(&mut self, val: super::vals::AlrmrMsk) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Alrmr {
        #[inline(always)]
        fn default() -> Alrmr {
            Alrmr(0)
        }
    }
    impl core::fmt::Debug for Alrmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrmr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("msk1", &self.msk1())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("msk2", &self.msk2())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .field("msk3", &self.msk3())
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("wdsel", &self.wdsel())
                .field("msk4", &self.msk4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alrmr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Alrmr {{ su: {=u8:?}, st: {=u8:?}, msk1: {:?}, mnu: {=u8:?}, mnt: {=u8:?}, msk2: {:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {:?}, msk3: {:?}, du: {=u8:?}, dt: {=u8:?}, wdsel: {:?}, msk4: {:?} }}" , self . su () , self . st () , self . msk1 () , self . mnu () , self . mnt () , self . msk2 () , self . hu () , self . ht () , self . pm () , self . msk3 () , self . du () , self . dt () , self . wdsel () , self . msk4 ())
        }
    }
    #[doc = "Alarm sub second register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrmssr(pub u32);
    impl Alrmssr {
        #[doc = "Sub seconds value"]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Sub seconds value"]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Mask the most-significant bits starting at this bit"]
        #[inline(always)]
        pub const fn maskss(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Mask the most-significant bits starting at this bit"]
        #[inline(always)]
        pub fn set_maskss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Alrmssr {
        #[inline(always)]
        fn default() -> Alrmssr {
            Alrmssr(0)
        }
    }
    impl core::fmt::Debug for Alrmssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrmssr")
                .field("ss", &self.ss())
                .field("maskss", &self.maskss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alrmssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alrmssr {{ ss: {=u16:?}, maskss: {=u8:?} }}",
                self.ss(),
                self.maskss()
            )
        }
    }
    #[doc = "Backup register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bkpr(pub u32);
    impl Bkpr {
        #[doc = "BKP"]
        #[inline(always)]
        pub const fn bkp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "BKP"]
        #[inline(always)]
        pub fn set_bkp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bkpr {
        #[inline(always)]
        fn default() -> Bkpr {
            Bkpr(0)
        }
    }
    impl core::fmt::Debug for Bkpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bkpr").field("bkp", &self.bkp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bkpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bkpr {{ bkp: {=u32:?} }}", self.bkp())
        }
    }
    #[doc = "Calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calibr(pub u32);
    impl Calibr {
        #[doc = "Digital calibration"]
        #[inline(always)]
        pub const fn dc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Digital calibration"]
        #[inline(always)]
        pub fn set_dc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Digital calibration sign"]
        #[inline(always)]
        pub const fn dcs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Digital calibration sign"]
        #[inline(always)]
        pub fn set_dcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Calibr {
        #[inline(always)]
        fn default() -> Calibr {
            Calibr(0)
        }
    }
    impl core::fmt::Debug for Calibr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calibr")
                .field("dc", &self.dc())
                .field("dcs", &self.dcs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calibr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Calibr {{ dc: {=u8:?}, dcs: {=bool:?} }}", self.dc(), self.dcs())
        }
    }
    #[doc = "Calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calr(pub u32);
    impl Calr {
        #[doc = "Calibration minus"]
        #[inline(always)]
        pub const fn calm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Calibration minus"]
        #[inline(always)]
        pub fn set_calm(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Use a 16-second calibration cycle period"]
        #[inline(always)]
        pub const fn calw16(&self) -> super::vals::Calw16 {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Calw16::from_bits(val as u8)
        }
        #[doc = "Use a 16-second calibration cycle period"]
        #[inline(always)]
        pub fn set_calw16(&mut self, val: super::vals::Calw16) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Use an 8-second calibration cycle period"]
        #[inline(always)]
        pub const fn calw8(&self) -> super::vals::Calw8 {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Calw8::from_bits(val as u8)
        }
        #[doc = "Use an 8-second calibration cycle period"]
        #[inline(always)]
        pub fn set_calw8(&mut self, val: super::vals::Calw8) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Increase frequency of RTC by 488.5 ppm"]
        #[inline(always)]
        pub const fn calp(&self) -> super::vals::Calp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Calp::from_bits(val as u8)
        }
        #[doc = "Increase frequency of RTC by 488.5 ppm"]
        #[inline(always)]
        pub fn set_calp(&mut self, val: super::vals::Calp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Calr {
        #[inline(always)]
        fn default() -> Calr {
            Calr(0)
        }
    }
    impl core::fmt::Debug for Calr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calr")
                .field("calm", &self.calm())
                .field("calw16", &self.calw16())
                .field("calw8", &self.calw8())
                .field("calp", &self.calp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calr {{ calm: {=u16:?}, calw16: {:?}, calw8: {:?}, calp: {:?} }}",
                self.calm(),
                self.calw16(),
                self.calw8(),
                self.calp()
            )
        }
    }
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Wakeup clock selection"]
        #[inline(always)]
        pub const fn wucksel(&self) -> super::vals::Wucksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Wucksel::from_bits(val as u8)
        }
        #[doc = "Wakeup clock selection"]
        #[inline(always)]
        pub fn set_wucksel(&mut self, val: super::vals::Wucksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Timestamp event active edge"]
        #[inline(always)]
        pub const fn tsedge(&self) -> super::vals::Tsedge {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Tsedge::from_bits(val as u8)
        }
        #[doc = "Timestamp event active edge"]
        #[inline(always)]
        pub fn set_tsedge(&mut self, val: super::vals::Tsedge) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Reference clock detection enable (50 or 60 Hz)"]
        #[inline(always)]
        pub const fn refckon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Reference clock detection enable (50 or 60 Hz)"]
        #[inline(always)]
        pub fn set_refckon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bypass the shadow registers"]
        #[inline(always)]
        pub const fn bypshad(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass the shadow registers"]
        #[inline(always)]
        pub fn set_bypshad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Hour format"]
        #[inline(always)]
        pub const fn fmt(&self) -> super::vals::Fmt {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Fmt::from_bits(val as u8)
        }
        #[doc = "Hour format"]
        #[inline(always)]
        pub fn set_fmt(&mut self, val: super::vals::Fmt) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Coarse digital calibration enable"]
        #[inline(always)]
        pub const fn dce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Coarse digital calibration enable"]
        #[inline(always)]
        pub fn set_dce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Alarm enable"]
        #[inline(always)]
        pub const fn alre(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm enable"]
        #[inline(always)]
        pub fn set_alre(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer enable"]
        #[inline(always)]
        pub const fn wute(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup timer enable"]
        #[inline(always)]
        pub fn set_wute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timestamp enable"]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp enable"]
        #[inline(always)]
        pub fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Alarm interrupt enable"]
        #[inline(always)]
        pub const fn alrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm interrupt enable"]
        #[inline(always)]
        pub fn set_alrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer interrupt enable"]
        #[inline(always)]
        pub const fn wutie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup timer interrupt enable"]
        #[inline(always)]
        pub fn set_wutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timestamp interrupt enable"]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp interrupt enable"]
        #[inline(always)]
        pub fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Add 1 hour (summer time change)"]
        #[inline(always)]
        pub const fn add1h(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Add 1 hour (summer time change)"]
        #[inline(always)]
        pub fn set_add1h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Subtract 1 hour (winter time change)"]
        #[inline(always)]
        pub const fn sub1h(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Subtract 1 hour (winter time change)"]
        #[inline(always)]
        pub fn set_sub1h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Backup"]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Backup"]
        #[inline(always)]
        pub fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Calibration output selection"]
        #[inline(always)]
        pub const fn cosel(&self) -> super::vals::Cosel {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Cosel::from_bits(val as u8)
        }
        #[doc = "Calibration output selection"]
        #[inline(always)]
        pub fn set_cosel(&mut self, val: super::vals::Cosel) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Output polarity"]
        #[inline(always)]
        pub const fn pol(&self) -> super::vals::Pol {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "Output polarity"]
        #[inline(always)]
        pub fn set_pol(&mut self, val: super::vals::Pol) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Output selection"]
        #[inline(always)]
        pub const fn osel(&self) -> super::vals::Osel {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Osel::from_bits(val as u8)
        }
        #[doc = "Output selection"]
        #[inline(always)]
        pub fn set_osel(&mut self, val: super::vals::Osel) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "Calibration output enable"]
        #[inline(always)]
        pub const fn coe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration output enable"]
        #[inline(always)]
        pub fn set_coe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("wucksel", &self.wucksel())
                .field("tsedge", &self.tsedge())
                .field("refckon", &self.refckon())
                .field("bypshad", &self.bypshad())
                .field("fmt", &self.fmt())
                .field("dce", &self.dce())
                .field("alre[0]", &self.alre(0usize))
                .field("alre[1]", &self.alre(1usize))
                .field("wute", &self.wute())
                .field("tse", &self.tse())
                .field("alrie[0]", &self.alrie(0usize))
                .field("alrie[1]", &self.alrie(1usize))
                .field("wutie", &self.wutie())
                .field("tsie", &self.tsie())
                .field("add1h", &self.add1h())
                .field("sub1h", &self.sub1h())
                .field("bkp", &self.bkp())
                .field("cosel", &self.cosel())
                .field("pol", &self.pol())
                .field("osel", &self.osel())
                .field("coe", &self.coe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ wucksel: {:?}, tsedge: {:?}, refckon: {=bool:?}, bypshad: {=bool:?}, fmt: {:?}, dce: {=bool:?}, alre[0]: {=bool:?}, alre[1]: {=bool:?}, wute: {=bool:?}, tse: {=bool:?}, alrie[0]: {=bool:?}, alrie[1]: {=bool:?}, wutie: {=bool:?}, tsie: {=bool:?}, add1h: {=bool:?}, sub1h: {=bool:?}, bkp: {=bool:?}, cosel: {:?}, pol: {:?}, osel: {:?}, coe: {=bool:?} }}" , self . wucksel () , self . tsedge () , self . refckon () , self . bypshad () , self . fmt () , self . dce () , self . alre (0usize) , self . alre (1usize) , self . wute () , self . tse () , self . alrie (0usize) , self . alrie (1usize) , self . wutie () , self . tsie () , self . add1h () , self . sub1h () , self . bkp () , self . cosel () , self . pol () , self . osel () , self . coe ())
        }
    }
    #[doc = "Date register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Date units in BCD format"]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units in BCD format"]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Month units in BCD format"]
        #[inline(always)]
        pub const fn mu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Month units in BCD format"]
        #[inline(always)]
        pub fn set_mu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Month tens in BCD format"]
        #[inline(always)]
        pub const fn mt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Month tens in BCD format"]
        #[inline(always)]
        pub fn set_mt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Week day units"]
        #[inline(always)]
        pub const fn wdu(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Week day units"]
        #[inline(always)]
        pub fn set_wdu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Year units in BCD format"]
        #[inline(always)]
        pub const fn yu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Year units in BCD format"]
        #[inline(always)]
        pub fn set_yu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Year tens in BCD format"]
        #[inline(always)]
        pub const fn yt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Year tens in BCD format"]
        #[inline(always)]
        pub fn set_yt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr")
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("mu", &self.mu())
                .field("mt", &self.mt())
                .field("wdu", &self.wdu())
                .field("yu", &self.yu())
                .field("yt", &self.yt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dr {{ du: {=u8:?}, dt: {=u8:?}, mu: {=u8:?}, mt: {=bool:?}, wdu: {=u8:?}, yu: {=u8:?}, yt: {=u8:?} }}",
                self.du(),
                self.dt(),
                self.mu(),
                self.mt(),
                self.wdu(),
                self.yu(),
                self.yt()
            )
        }
    }
    #[doc = "Initialization and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Alarm write enabled"]
        #[inline(always)]
        pub const fn alrwf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm write enabled"]
        #[inline(always)]
        pub fn set_alrwf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer write enabled"]
        #[inline(always)]
        pub const fn wutwf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup timer write enabled"]
        #[inline(always)]
        pub fn set_wutwf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Shift operation pending"]
        #[inline(always)]
        pub const fn shpf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Shift operation pending"]
        #[inline(always)]
        pub fn set_shpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Initialization status flag"]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization status flag"]
        #[inline(always)]
        pub fn set_inits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Registers synchronization flag"]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Registers synchronization flag"]
        #[inline(always)]
        pub fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Initialization flag"]
        #[inline(always)]
        pub const fn initf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization flag"]
        #[inline(always)]
        pub fn set_initf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enter Initialization mode"]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Initialization mode"]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Alarm flag"]
        #[inline(always)]
        pub const fn alrf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm flag"]
        #[inline(always)]
        pub fn set_alrf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer flag"]
        #[inline(always)]
        pub const fn wutf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup timer flag"]
        #[inline(always)]
        pub fn set_wutf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timestamp flag"]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp flag"]
        #[inline(always)]
        pub fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Timestamp overflow flag"]
        #[inline(always)]
        pub const fn tsovf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp overflow flag"]
        #[inline(always)]
        pub fn set_tsovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tamper detection flag"]
        #[inline(always)]
        pub const fn tampf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 13usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection flag"]
        #[inline(always)]
        pub fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 13usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Recalibration pending flag"]
        #[inline(always)]
        pub const fn recalpf(&self) -> super::vals::Recalpf {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Recalpf::from_bits(val as u8)
        }
        #[doc = "Recalibration pending flag"]
        #[inline(always)]
        pub fn set_recalpf(&mut self, val: super::vals::Recalpf) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
                .field("alrwf[0]", &self.alrwf(0usize))
                .field("alrwf[1]", &self.alrwf(1usize))
                .field("wutwf", &self.wutwf())
                .field("shpf", &self.shpf())
                .field("inits", &self.inits())
                .field("rsf", &self.rsf())
                .field("initf", &self.initf())
                .field("init", &self.init())
                .field("alrf[0]", &self.alrf(0usize))
                .field("alrf[1]", &self.alrf(1usize))
                .field("wutf", &self.wutf())
                .field("tsf", &self.tsf())
                .field("tsovf", &self.tsovf())
                .field("tampf[0]", &self.tampf(0usize))
                .field("tampf[1]", &self.tampf(1usize))
                .field("tampf[2]", &self.tampf(2usize))
                .field("recalpf", &self.recalpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ alrwf[0]: {=bool:?}, alrwf[1]: {=bool:?}, wutwf: {=bool:?}, shpf: {=bool:?}, inits: {=bool:?}, rsf: {=bool:?}, initf: {=bool:?}, init: {=bool:?}, alrf[0]: {=bool:?}, alrf[1]: {=bool:?}, wutf: {=bool:?}, tsf: {=bool:?}, tsovf: {=bool:?}, tampf[0]: {=bool:?}, tampf[1]: {=bool:?}, tampf[2]: {=bool:?}, recalpf: {:?} }}" , self . alrwf (0usize) , self . alrwf (1usize) , self . wutwf () , self . shpf () , self . inits () , self . rsf () , self . initf () , self . init () , self . alrf (0usize) , self . alrf (1usize) , self . wutf () , self . tsf () , self . tsovf () , self . tampf (0usize) , self . tampf (1usize) , self . tampf (2usize) , self . recalpf ())
        }
    }
    #[doc = "Prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prer(pub u32);
    impl Prer {
        #[doc = "Synchronous prescaler factor"]
        #[inline(always)]
        pub const fn prediv_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Synchronous prescaler factor"]
        #[inline(always)]
        pub fn set_prediv_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Asynchronous prescaler factor"]
        #[inline(always)]
        pub const fn prediv_a(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Asynchronous prescaler factor"]
        #[inline(always)]
        pub fn set_prediv_a(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Prer {
        #[inline(always)]
        fn default() -> Prer {
            Prer(0)
        }
    }
    impl core::fmt::Debug for Prer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prer")
                .field("prediv_s", &self.prediv_s())
                .field("prediv_a", &self.prediv_a())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prer {{ prediv_s: {=u16:?}, prediv_a: {=u8:?} }}",
                self.prediv_s(),
                self.prediv_a()
            )
        }
    }
    #[doc = "Shift control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shiftr(pub u32);
    impl Shiftr {
        #[doc = "Subtract a fraction of a second"]
        #[inline(always)]
        pub const fn subfs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Subtract a fraction of a second"]
        #[inline(always)]
        pub fn set_subfs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Add one second"]
        #[inline(always)]
        pub const fn add1s(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Add one second"]
        #[inline(always)]
        pub fn set_add1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Shiftr {
        #[inline(always)]
        fn default() -> Shiftr {
            Shiftr(0)
        }
    }
    impl core::fmt::Debug for Shiftr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Shiftr")
                .field("subfs", &self.subfs())
                .field("add1s", &self.add1s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Shiftr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Shiftr {{ subfs: {=u16:?}, add1s: {=bool:?} }}",
                self.subfs(),
                self.add1s()
            )
        }
    }
    #[doc = "Sub second register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssr(pub u32);
    impl Ssr {
        #[doc = "Sub second value"]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sub second value"]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ssr {
        #[inline(always)]
        fn default() -> Ssr {
            Ssr(0)
        }
    }
    impl core::fmt::Debug for Ssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ssr").field("ss", &self.ss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ssr {{ ss: {=u16:?} }}", self.ss())
        }
    }
    #[doc = "Tamper and alternate function configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tafcr(pub u32);
    impl Tafcr {
        #[doc = "Tamper detection enable"]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + ([0usize, 3usize, 5usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection enable"]
        #[inline(always)]
        pub fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + ([0usize, 3usize, 5usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Active level for tamper"]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> super::vals::Tamptrg {
            assert!(n < 3usize);
            let offs = 1usize + ([0usize, 3usize, 5usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            super::vals::Tamptrg::from_bits(val as u8)
        }
        #[doc = "Active level for tamper"]
        #[inline(always)]
        pub fn set_tamptrg(&mut self, n: usize, val: super::vals::Tamptrg) {
            assert!(n < 3usize);
            let offs = 1usize + ([0usize, 3usize, 5usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Tamper interrupt enable"]
        #[inline(always)]
        pub const fn tampie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper interrupt enable"]
        #[inline(always)]
        pub fn set_tampie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Activate timestamp on tamper detection event"]
        #[inline(always)]
        pub const fn tampts(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Activate timestamp on tamper detection event"]
        #[inline(always)]
        pub fn set_tampts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Tamper sampling frequency"]
        #[inline(always)]
        pub const fn tampfreq(&self) -> super::vals::Tampfreq {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Tampfreq::from_bits(val as u8)
        }
        #[doc = "Tamper sampling frequency"]
        #[inline(always)]
        pub fn set_tampfreq(&mut self, val: super::vals::Tampfreq) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Tamper filter count"]
        #[inline(always)]
        pub const fn tampflt(&self) -> super::vals::Tampflt {
            let val = (self.0 >> 11usize) & 0x03;
            super::vals::Tampflt::from_bits(val as u8)
        }
        #[doc = "Tamper filter count"]
        #[inline(always)]
        pub fn set_tampflt(&mut self, val: super::vals::Tampflt) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
        }
        #[doc = "Tamper precharge duration"]
        #[inline(always)]
        pub const fn tampprch(&self) -> super::vals::Tampprch {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Tampprch::from_bits(val as u8)
        }
        #[doc = "Tamper precharge duration"]
        #[inline(always)]
        pub fn set_tampprch(&mut self, val: super::vals::Tampprch) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
        #[doc = "Tamper pull-up disable"]
        #[inline(always)]
        pub const fn tamppudis(&self) -> super::vals::Tamppudis {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Tamppudis::from_bits(val as u8)
        }
        #[doc = "Tamper pull-up disable"]
        #[inline(always)]
        pub fn set_tamppudis(&mut self, val: super::vals::Tamppudis) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "AFO_ALARM output type"]
        #[inline(always)]
        pub const fn alarmouttype(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "AFO_ALARM output type"]
        #[inline(always)]
        pub fn set_alarmouttype(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Tafcr {
        #[inline(always)]
        fn default() -> Tafcr {
            Tafcr(0)
        }
    }
    impl core::fmt::Debug for Tafcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tafcr")
                .field("tampe[0]", &self.tampe(0usize))
                .field("tampe[1]", &self.tampe(1usize))
                .field("tampe[2]", &self.tampe(2usize))
                .field("tamptrg[0]", &self.tamptrg(0usize))
                .field("tamptrg[1]", &self.tamptrg(1usize))
                .field("tamptrg[2]", &self.tamptrg(2usize))
                .field("tampie", &self.tampie())
                .field("tampts", &self.tampts())
                .field("tampfreq", &self.tampfreq())
                .field("tampflt", &self.tampflt())
                .field("tampprch", &self.tampprch())
                .field("tamppudis", &self.tamppudis())
                .field("alarmouttype", &self.alarmouttype())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tafcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Tafcr {{ tampe[0]: {=bool:?}, tampe[1]: {=bool:?}, tampe[2]: {=bool:?}, tamptrg[0]: {:?}, tamptrg[1]: {:?}, tamptrg[2]: {:?}, tampie: {=bool:?}, tampts: {=bool:?}, tampfreq: {:?}, tampflt: {:?}, tampprch: {:?}, tamppudis: {:?}, alarmouttype: {=bool:?} }}" , self . tampe (0usize) , self . tampe (1usize) , self . tampe (2usize) , self . tamptrg (0usize) , self . tamptrg (1usize) , self . tamptrg (2usize) , self . tampie () , self . tampts () , self . tampfreq () , self . tampflt () , self . tampprch () , self . tamppudis () , self . alarmouttype ())
        }
    }
    #[doc = "Time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tr(pub u32);
    impl Tr {
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::Ampm {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ampm::from_bits(val as u8)
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: super::vals::Ampm) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Tr {
        #[inline(always)]
        fn default() -> Tr {
            Tr(0)
        }
    }
    impl core::fmt::Debug for Tr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tr {{ su: {=u8:?}, st: {=u8:?}, mnu: {=u8:?}, mnt: {=u8:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {:?} }}",
                self.su(),
                self.st(),
                self.mnu(),
                self.mnt(),
                self.hu(),
                self.ht(),
                self.pm()
            )
        }
    }
    #[doc = "Timestamp date register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsdr(pub u32);
    impl Tsdr {
        #[doc = "Date units in BCD format"]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units in BCD format"]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Month units in BCD format"]
        #[inline(always)]
        pub const fn mu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Month units in BCD format"]
        #[inline(always)]
        pub fn set_mu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Month tens in BCD format"]
        #[inline(always)]
        pub const fn mt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Month tens in BCD format"]
        #[inline(always)]
        pub fn set_mt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Week day units"]
        #[inline(always)]
        pub const fn wdu(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Week day units"]
        #[inline(always)]
        pub fn set_wdu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
    }
    impl Default for Tsdr {
        #[inline(always)]
        fn default() -> Tsdr {
            Tsdr(0)
        }
    }
    impl core::fmt::Debug for Tsdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsdr")
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("mu", &self.mu())
                .field("mt", &self.mt())
                .field("wdu", &self.wdu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tsdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tsdr {{ du: {=u8:?}, dt: {=u8:?}, mu: {=u8:?}, mt: {=bool:?}, wdu: {=u8:?} }}",
                self.du(),
                self.dt(),
                self.mu(),
                self.mt(),
                self.wdu()
            )
        }
    }
    #[doc = "Timestamp sub second register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsssr(pub u32);
    impl Tsssr {
        #[doc = "Sub second value"]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sub second value"]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tsssr {
        #[inline(always)]
        fn default() -> Tsssr {
            Tsssr(0)
        }
    }
    impl core::fmt::Debug for Tsssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsssr").field("ss", &self.ss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tsssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tsssr {{ ss: {=u16:?} }}", self.ss())
        }
    }
    #[doc = "Timestamp time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tstr(pub u32);
    impl Tstr {
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::Ampm {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ampm::from_bits(val as u8)
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: super::vals::Ampm) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Tstr {
        #[inline(always)]
        fn default() -> Tstr {
            Tstr(0)
        }
    }
    impl core::fmt::Debug for Tstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tstr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tstr {{ su: {=u8:?}, st: {=u8:?}, mnu: {=u8:?}, mnt: {=u8:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {:?} }}",
                self.su(),
                self.st(),
                self.mnu(),
                self.mnt(),
                self.hu(),
                self.ht(),
                self.pm()
            )
        }
    }
    #[doc = "Write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpr(pub u32);
    impl Wpr {
        #[doc = "Write protection key"]
        #[inline(always)]
        pub const fn key(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Write protection key"]
        #[inline(always)]
        pub fn set_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wpr {
        #[inline(always)]
        fn default() -> Wpr {
            Wpr(0)
        }
    }
    impl core::fmt::Debug for Wpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wpr {{ key: {=u8:?} }}", self.key())
        }
    }
    #[doc = "Wakeup timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wutr(pub u32);
    impl Wutr {
        #[doc = "Wakeup auto-reload value bits"]
        #[inline(always)]
        pub const fn wut(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Wakeup auto-reload value bits"]
        #[inline(always)]
        pub fn set_wut(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wutr {
        #[inline(always)]
        fn default() -> Wutr {
            Wutr(0)
        }
    }
    impl core::fmt::Debug for Wutr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wutr").field("wut", &self.wut()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wutr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wutr {{ wut: {=u16:?} }}", self.wut())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AlrmrMsk {
        #[doc = "Alarm set if the date/day match"]
        TO_MATCH = 0x0,
        #[doc = "Date/day don’t care in Alarm comparison"]
        NOT_MATCH = 0x01,
    }
    impl AlrmrMsk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AlrmrMsk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AlrmrMsk {
        #[inline(always)]
        fn from(val: u8) -> AlrmrMsk {
            AlrmrMsk::from_bits(val)
        }
    }
    impl From<AlrmrMsk> for u8 {
        #[inline(always)]
        fn from(val: AlrmrMsk) -> u8 {
            AlrmrMsk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AlrmrPm {
        #[doc = "AM or 24-hour format"]
        AM = 0x0,
        #[doc = "PM"]
        PM = 0x01,
    }
    impl AlrmrPm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AlrmrPm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AlrmrPm {
        #[inline(always)]
        fn from(val: u8) -> AlrmrPm {
            AlrmrPm::from_bits(val)
        }
    }
    impl From<AlrmrPm> for u8 {
        #[inline(always)]
        fn from(val: AlrmrPm) -> u8 {
            AlrmrPm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AlrmrWdsel {
        #[doc = "DU\\[3:0\\]
represents the date units"]
        DATE_UNITS = 0x0,
        #[doc = "DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is don’t care"]
        WEEK_DAY = 0x01,
    }
    impl AlrmrWdsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AlrmrWdsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AlrmrWdsel {
        #[inline(always)]
        fn from(val: u8) -> AlrmrWdsel {
            AlrmrWdsel::from_bits(val)
        }
    }
    impl From<AlrmrWdsel> for u8 {
        #[inline(always)]
        fn from(val: AlrmrWdsel) -> u8 {
            AlrmrWdsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ampm {
        #[doc = "AM or 24-hour format"]
        AM = 0x0,
        #[doc = "PM"]
        PM = 0x01,
    }
    impl Ampm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ampm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ampm {
        #[inline(always)]
        fn from(val: u8) -> Ampm {
            Ampm::from_bits(val)
        }
    }
    impl From<Ampm> for u8 {
        #[inline(always)]
        fn from(val: Ampm) -> u8 {
            Ampm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calp {
        #[doc = "No RTCCLK pulses are added"]
        NO_CHANGE = 0x0,
        #[doc = "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
        INCREASE_FREQ = 0x01,
    }
    impl Calp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calp {
        #[inline(always)]
        fn from(val: u8) -> Calp {
            Calp::from_bits(val)
        }
    }
    impl From<Calp> for u8 {
        #[inline(always)]
        fn from(val: Calp) -> u8 {
            Calp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calw16 {
        _RESERVED_0 = 0x0,
        #[doc = "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
        SIXTEEN_SECOND = 0x01,
    }
    impl Calw16 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calw16 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calw16 {
        #[inline(always)]
        fn from(val: u8) -> Calw16 {
            Calw16::from_bits(val)
        }
    }
    impl From<Calw16> for u8 {
        #[inline(always)]
        fn from(val: Calw16) -> u8 {
            Calw16::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calw8 {
        _RESERVED_0 = 0x0,
        #[doc = "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
        EIGHT_SECOND = 0x01,
    }
    impl Calw8 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calw8 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calw8 {
        #[inline(always)]
        fn from(val: u8) -> Calw8 {
            Calw8::from_bits(val)
        }
    }
    impl From<Calw8> for u8 {
        #[inline(always)]
        fn from(val: Calw8) -> u8 {
            Calw8::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cosel {
        #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
        CAL_FREQ_512HZ = 0x0,
        #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
        CAL_FREQ_1HZ = 0x01,
    }
    impl Cosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cosel {
        #[inline(always)]
        fn from(val: u8) -> Cosel {
            Cosel::from_bits(val)
        }
    }
    impl From<Cosel> for u8 {
        #[inline(always)]
        fn from(val: Cosel) -> u8 {
            Cosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fmt {
        #[doc = "24 hour/day format"]
        TWENTY_FOUR_HOUR = 0x0,
        #[doc = "AM/PM hour format"]
        AM_PM = 0x01,
    }
    impl Fmt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmt {
        #[inline(always)]
        fn from(val: u8) -> Fmt {
            Fmt::from_bits(val)
        }
    }
    impl From<Fmt> for u8 {
        #[inline(always)]
        fn from(val: Fmt) -> u8 {
            Fmt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Osel {
        #[doc = "Output disabled"]
        DISABLED = 0x0,
        #[doc = "Alarm A output enabled"]
        ALARM_A = 0x01,
        #[doc = "Alarm B output enabled"]
        ALARM_B = 0x02,
        #[doc = "Wakeup output enabled"]
        WAKEUP = 0x03,
    }
    impl Osel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Osel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Osel {
        #[inline(always)]
        fn from(val: u8) -> Osel {
            Osel::from_bits(val)
        }
    }
    impl From<Osel> for u8 {
        #[inline(always)]
        fn from(val: Osel) -> u8 {
            Osel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pol {
        #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
        HIGH = 0x0,
        #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
        LOW = 0x01,
    }
    impl Pol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pol {
        #[inline(always)]
        fn from(val: u8) -> Pol {
            Pol::from_bits(val)
        }
    }
    impl From<Pol> for u8 {
        #[inline(always)]
        fn from(val: Pol) -> u8 {
            Pol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Recalpf {
        _RESERVED_0 = 0x0,
        #[doc = "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
        PENDING = 0x01,
    }
    impl Recalpf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Recalpf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Recalpf {
        #[inline(always)]
        fn from(val: u8) -> Recalpf {
            Recalpf::from_bits(val)
        }
    }
    impl From<Recalpf> for u8 {
        #[inline(always)]
        fn from(val: Recalpf) -> u8 {
            Recalpf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tampflt {
        #[doc = "Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
        IMMEDIATE = 0x0,
        #[doc = "Tamper event is activated after 2 consecutive samples at the active level"]
        SAMPLES2 = 0x01,
        #[doc = "Tamper event is activated after 4 consecutive samples at the active level"]
        SAMPLES4 = 0x02,
        #[doc = "Tamper event is activated after 8 consecutive samples at the active level"]
        SAMPLES8 = 0x03,
    }
    impl Tampflt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampflt {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampflt {
        #[inline(always)]
        fn from(val: u8) -> Tampflt {
            Tampflt::from_bits(val)
        }
    }
    impl From<Tampflt> for u8 {
        #[inline(always)]
        fn from(val: Tampflt) -> u8 {
            Tampflt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tampfreq {
        #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
        DIV32768 = 0x0,
        #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
        DIV16384 = 0x01,
        #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
        DIV8192 = 0x02,
        #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
        DIV4096 = 0x03,
        #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
        DIV2048 = 0x04,
        #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
        DIV1024 = 0x05,
        #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
        DIV512 = 0x06,
        #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
        DIV256 = 0x07,
    }
    impl Tampfreq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampfreq {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampfreq {
        #[inline(always)]
        fn from(val: u8) -> Tampfreq {
            Tampfreq::from_bits(val)
        }
    }
    impl From<Tampfreq> for u8 {
        #[inline(always)]
        fn from(val: Tampfreq) -> u8 {
            Tampfreq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tampprch {
        #[doc = "1 RTCCLK cycle"]
        CYCLES1 = 0x0,
        #[doc = "2 RTCCLK cycles"]
        CYCLES2 = 0x01,
        #[doc = "4 RTCCLK cycles"]
        CYCLES4 = 0x02,
        #[doc = "8 RTCCLK cycles"]
        CYCLES8 = 0x03,
    }
    impl Tampprch {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampprch {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampprch {
        #[inline(always)]
        fn from(val: u8) -> Tampprch {
            Tampprch::from_bits(val)
        }
    }
    impl From<Tampprch> for u8 {
        #[inline(always)]
        fn from(val: Tampprch) -> u8 {
            Tampprch::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tamppudis {
        #[doc = "Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
        ENABLED = 0x0,
        #[doc = "Disable precharge of RTC_TAMPx pins"]
        DISABLED = 0x01,
    }
    impl Tamppudis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tamppudis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tamppudis {
        #[inline(always)]
        fn from(val: u8) -> Tamppudis {
            Tamppudis::from_bits(val)
        }
    }
    impl From<Tamppudis> for u8 {
        #[inline(always)]
        fn from(val: Tamppudis) -> u8 {
            Tamppudis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tamptrg {
        #[doc = "If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
        RISING_EDGE = 0x0,
        #[doc = "If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
        FALLING_EDGE = 0x01,
    }
    impl Tamptrg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tamptrg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tamptrg {
        #[inline(always)]
        fn from(val: u8) -> Tamptrg {
            Tamptrg::from_bits(val)
        }
    }
    impl From<Tamptrg> for u8 {
        #[inline(always)]
        fn from(val: Tamptrg) -> u8 {
            Tamptrg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tsedge {
        #[doc = "RTC_TS input rising edge generates a time-stamp event"]
        RISING_EDGE = 0x0,
        #[doc = "RTC_TS input falling edge generates a time-stamp event"]
        FALLING_EDGE = 0x01,
    }
    impl Tsedge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsedge {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsedge {
        #[inline(always)]
        fn from(val: u8) -> Tsedge {
            Tsedge::from_bits(val)
        }
    }
    impl From<Tsedge> for u8 {
        #[inline(always)]
        fn from(val: Tsedge) -> u8 {
            Tsedge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wucksel {
        #[doc = "RTC/16 clock is selected"]
        DIV16 = 0x0,
        #[doc = "RTC/8 clock is selected"]
        DIV8 = 0x01,
        #[doc = "RTC/4 clock is selected"]
        DIV4 = 0x02,
        #[doc = "RTC/2 clock is selected"]
        DIV2 = 0x03,
        #[doc = "ck_spre (usually 1 Hz) clock is selected"]
        CLOCK_SPARE = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
        CLOCK_SPARE_WITH_OFFSET = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Wucksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wucksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wucksel {
        #[inline(always)]
        fn from(val: u8) -> Wucksel {
            Wucksel::from_bits(val)
        }
    }
    impl From<Wucksel> for u8 {
        #[inline(always)]
        fn from(val: Wucksel) -> u8 {
            Wucksel::to_bits(val)
        }
    }
}
