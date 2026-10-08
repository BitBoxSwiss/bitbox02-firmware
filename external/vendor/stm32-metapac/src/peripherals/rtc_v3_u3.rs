#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RTC register block."]
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
    #[doc = "RTC time register."]
    #[inline(always)]
    pub const fn tr(self) -> crate::common::Reg<regs::Tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RTC date register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RTC subsecond register."]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RTC initialization control and status register."]
    #[inline(always)]
    pub const fn icsr(self) -> crate::common::Reg<regs::Icsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RTC prescaler register."]
    #[inline(always)]
    pub const fn prer(self) -> crate::common::Reg<regs::Prer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RTC wake-up timer register."]
    #[inline(always)]
    pub const fn wutr(self) -> crate::common::Reg<regs::Wutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "RTC control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "RTC privilege mode control register."]
    #[inline(always)]
    pub const fn privcr(self) -> crate::common::Reg<regs::Privcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "RTC secure configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RTC write protection register."]
    #[inline(always)]
    pub const fn wpr(self) -> crate::common::Reg<regs::Wpr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RTC calibration register."]
    #[inline(always)]
    pub const fn calr(self) -> crate::common::Reg<regs::Calr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RTC shift control register."]
    #[inline(always)]
    pub const fn shiftr(self) -> crate::common::Reg<regs::Shiftr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "RTC timestamp time register."]
    #[inline(always)]
    pub const fn tstr(self) -> crate::common::Reg<regs::Tstr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "RTC timestamp date register."]
    #[inline(always)]
    pub const fn tsdr(self) -> crate::common::Reg<regs::Tsdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "RTC timestamp subsecond register."]
    #[inline(always)]
    pub const fn tsssr(self) -> crate::common::Reg<regs::Tsssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "RTC alarm register."]
    #[inline(always)]
    pub const fn alrmr(self, n: usize) -> crate::common::Reg<regs::Alrmr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 8usize) as _) }
    }
    #[doc = "RTC alarm subsecond register."]
    #[inline(always)]
    pub const fn alrmssr(self, n: usize) -> crate::common::Reg<regs::Alrmssr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize + n * 8usize) as _) }
    }
    #[doc = "RTC status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RTC nonsecure masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RTC secure masked interrupt status register."]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Smisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RTC status clear register."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "RTC timestamp on tamper control register."]
    #[inline(always)]
    pub const fn tamptscr(self) -> crate::common::Reg<regs::Tamptscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RTC timestamp status register."]
    #[inline(always)]
    pub const fn tsidr(self) -> crate::common::Reg<regs::Tsidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "RTC alarm binary mode register."]
    #[inline(always)]
    pub const fn alrbinr(self, n: usize) -> crate::common::Reg<regs::Alrbinr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "RTC alarm binary mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrbinr(pub u32);
    impl Alrbinr {
        #[doc = "Synchronous counter alarm value in Binary mode."]
        #[inline(always)]
        pub const fn ss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Synchronous counter alarm value in Binary mode."]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Alrbinr {
        #[inline(always)]
        fn default() -> Alrbinr {
            Alrbinr(0)
        }
    }
    impl core::fmt::Debug for Alrbinr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrbinr").field("ss", &self.ss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alrbinr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alrbinr {{ ss: {=u32:?} }}", self.ss())
        }
    }
    #[doc = "RTC alarm register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrmr(pub u32);
    impl Alrmr {
        #[doc = "Second units in BCD format."]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format."]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format."]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format."]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Alarm seconds mask."]
        #[inline(always)]
        pub const fn msk1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm seconds mask."]
        #[inline(always)]
        pub fn set_msk1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Minute units in BCD format."]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format."]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format."]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format."]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Alarm A minutes mask."]
        #[inline(always)]
        pub const fn msk2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm A minutes mask."]
        #[inline(always)]
        pub fn set_msk2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Hour units in BCD format."]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format."]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format."]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format."]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation."]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::Ampm {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ampm::from_bits(val as u8)
        }
        #[doc = "AM/PM notation."]
        #[inline(always)]
        pub fn set_pm(&mut self, val: super::vals::Ampm) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Alarm hours mask."]
        #[inline(always)]
        pub const fn msk3(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm hours mask."]
        #[inline(always)]
        pub fn set_msk3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Date units or day in BCD format."]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units or day in BCD format."]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Date tens in BCD format."]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format."]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Week day selection."]
        #[inline(always)]
        pub const fn wdsel(&self) -> super::vals::AlrmrWdsel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::AlrmrWdsel::from_bits(val as u8)
        }
        #[doc = "Week day selection."]
        #[inline(always)]
        pub fn set_wdsel(&mut self, val: super::vals::AlrmrWdsel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Alarm date mask."]
        #[inline(always)]
        pub const fn msk4(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm date mask."]
        #[inline(always)]
        pub fn set_msk4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            defmt :: write ! (f , "Alrmr {{ su: {=u8:?}, st: {=u8:?}, msk1: {=bool:?}, mnu: {=u8:?}, mnt: {=u8:?}, msk2: {=bool:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {:?}, msk3: {=bool:?}, du: {=u8:?}, dt: {=u8:?}, wdsel: {:?}, msk4: {=bool:?} }}" , self . su () , self . st () , self . msk1 () , self . mnu () , self . mnt () , self . msk2 () , self . hu () , self . ht () , self . pm () , self . msk3 () , self . du () , self . dt () , self . wdsel () , self . msk4 ())
        }
    }
    #[doc = "RTC alarm A subsecond register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrmssr(pub u32);
    impl Alrmssr {
        #[doc = "Subseconds value."]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Subseconds value."]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Mask the most-significant bits starting at this bit."]
        #[inline(always)]
        pub const fn maskss(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask the most-significant bits starting at this bit."]
        #[inline(always)]
        pub fn set_maskss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Clear synchronous counter on alarm (Binary mode only)."]
        #[inline(always)]
        pub const fn ssclr(&self) -> super::vals::AlrmssrSsclr {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::AlrmssrSsclr::from_bits(val as u8)
        }
        #[doc = "Clear synchronous counter on alarm (Binary mode only)."]
        #[inline(always)]
        pub fn set_ssclr(&mut self, val: super::vals::AlrmssrSsclr) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                .field("ssclr", &self.ssclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alrmssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alrmssr {{ ss: {=u16:?}, maskss: {=u8:?}, ssclr: {:?} }}",
                self.ss(),
                self.maskss(),
                self.ssclr()
            )
        }
    }
    #[doc = "RTC calibration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calr(pub u32);
    impl Calr {
        #[doc = "Calibration minus."]
        #[inline(always)]
        pub const fn calm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Calibration minus."]
        #[inline(always)]
        pub fn set_calm(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "RTC low-power mode."]
        #[inline(always)]
        pub const fn lpcal(&self) -> super::vals::Lpcal {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Lpcal::from_bits(val as u8)
        }
        #[doc = "RTC low-power mode."]
        #[inline(always)]
        pub fn set_lpcal(&mut self, val: super::vals::Lpcal) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Use a 16-second calibration cycle period."]
        #[inline(always)]
        pub const fn calw16(&self) -> super::vals::Calw16 {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Calw16::from_bits(val as u8)
        }
        #[doc = "Use a 16-second calibration cycle period."]
        #[inline(always)]
        pub fn set_calw16(&mut self, val: super::vals::Calw16) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Use an 8-second calibration cycle period."]
        #[inline(always)]
        pub const fn calw8(&self) -> super::vals::Calw8 {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Calw8::from_bits(val as u8)
        }
        #[doc = "Use an 8-second calibration cycle period."]
        #[inline(always)]
        pub fn set_calw8(&mut self, val: super::vals::Calw8) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Increase frequency of RTC by 488."]
        #[inline(always)]
        pub const fn calp(&self) -> super::vals::Calp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Calp::from_bits(val as u8)
        }
        #[doc = "Increase frequency of RTC by 488."]
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
                .field("lpcal", &self.lpcal())
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
                "Calr {{ calm: {=u16:?}, lpcal: {:?}, calw16: {:?}, calw8: {:?}, calp: {:?} }}",
                self.calm(),
                self.lpcal(),
                self.calw16(),
                self.calw8(),
                self.calp()
            )
        }
    }
    #[doc = "RTC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "ck_wut wake-up clock selection."]
        #[inline(always)]
        pub const fn wucksel(&self) -> super::vals::Wucksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Wucksel::from_bits(val as u8)
        }
        #[doc = "ck_wut wake-up clock selection."]
        #[inline(always)]
        pub fn set_wucksel(&mut self, val: super::vals::Wucksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Timestamp event active edge."]
        #[inline(always)]
        pub const fn tsedge(&self) -> super::vals::Tsedge {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Tsedge::from_bits(val as u8)
        }
        #[doc = "Timestamp event active edge."]
        #[inline(always)]
        pub fn set_tsedge(&mut self, val: super::vals::Tsedge) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "REFIN reference clock detection enable (50 or 60Hz)."]
        #[inline(always)]
        pub const fn refckon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "REFIN reference clock detection enable (50 or 60Hz)."]
        #[inline(always)]
        pub fn set_refckon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bypass the shadow registers."]
        #[inline(always)]
        pub const fn bypshad(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass the shadow registers."]
        #[inline(always)]
        pub fn set_bypshad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Hour format."]
        #[inline(always)]
        pub const fn fmt(&self) -> super::vals::Fmt {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Fmt::from_bits(val as u8)
        }
        #[doc = "Hour format."]
        #[inline(always)]
        pub fn set_fmt(&mut self, val: super::vals::Fmt) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "SSR underflow interrupt enable."]
        #[inline(always)]
        pub const fn ssruie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SSR underflow interrupt enable."]
        #[inline(always)]
        pub fn set_ssruie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Alarm enable."]
        #[inline(always)]
        pub const fn alre(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm enable."]
        #[inline(always)]
        pub fn set_alre(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wake-up timer enable."]
        #[inline(always)]
        pub const fn wute(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up timer enable."]
        #[inline(always)]
        pub fn set_wute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "timestamp enable."]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "timestamp enable."]
        #[inline(always)]
        pub fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Alarm interrupt enable."]
        #[inline(always)]
        pub const fn alrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm interrupt enable."]
        #[inline(always)]
        pub fn set_alrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wake-up timer interrupt enable."]
        #[inline(always)]
        pub const fn wutie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up timer interrupt enable."]
        #[inline(always)]
        pub fn set_wutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timestamp interrupt enable."]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp interrupt enable."]
        #[inline(always)]
        pub fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Add 1 hour (summer time change)."]
        #[inline(always)]
        pub const fn add1h(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Add 1 hour (summer time change)."]
        #[inline(always)]
        pub fn set_add1h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Subtract 1 hour (winter time change)."]
        #[inline(always)]
        pub const fn sub1h(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Subtract 1 hour (winter time change)."]
        #[inline(always)]
        pub fn set_sub1h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Backup."]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Backup."]
        #[inline(always)]
        pub fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Calibration output selection."]
        #[inline(always)]
        pub const fn cosel(&self) -> super::vals::Cosel {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Cosel::from_bits(val as u8)
        }
        #[doc = "Calibration output selection."]
        #[inline(always)]
        pub fn set_cosel(&mut self, val: super::vals::Cosel) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Output polarity."]
        #[inline(always)]
        pub const fn pol(&self) -> super::vals::Pol {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "Output polarity."]
        #[inline(always)]
        pub fn set_pol(&mut self, val: super::vals::Pol) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Output selection."]
        #[inline(always)]
        pub const fn osel(&self) -> super::vals::Osel {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Osel::from_bits(val as u8)
        }
        #[doc = "Output selection."]
        #[inline(always)]
        pub fn set_osel(&mut self, val: super::vals::Osel) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "Calibration output enable."]
        #[inline(always)]
        pub const fn coe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration output enable."]
        #[inline(always)]
        pub fn set_coe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "timestamp on internal event enable."]
        #[inline(always)]
        pub const fn itse(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "timestamp on internal event enable."]
        #[inline(always)]
        pub fn set_itse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Activate timestamp on tamper detection event."]
        #[inline(always)]
        pub const fn tampts(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Activate timestamp on tamper detection event."]
        #[inline(always)]
        pub fn set_tampts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Tamper detection output enable on TAMPALRM."]
        #[inline(always)]
        pub const fn tampoe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection output enable on TAMPALRM."]
        #[inline(always)]
        pub fn set_tampoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Alarm A flag automatic clear."]
        #[inline(always)]
        pub const fn alrfclr(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 27usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm A flag automatic clear."]
        #[inline(always)]
        pub fn set_alrfclr(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 27usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "TAMPALRM pull-up enable."]
        #[inline(always)]
        pub const fn tampalrm_pu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TAMPALRM pull-up enable."]
        #[inline(always)]
        pub fn set_tampalrm_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "TAMPALRM output type."]
        #[inline(always)]
        pub const fn tampalrm_type(&self) -> super::vals::TampalrmType {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::TampalrmType::from_bits(val as u8)
        }
        #[doc = "TAMPALRM output type."]
        #[inline(always)]
        pub fn set_tampalrm_type(&mut self, val: super::vals::TampalrmType) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "OUT2 output enable."]
        #[inline(always)]
        pub const fn out2en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "OUT2 output enable."]
        #[inline(always)]
        pub fn set_out2en(&mut self, val: bool) {
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
                .field("wucksel", &self.wucksel())
                .field("tsedge", &self.tsedge())
                .field("refckon", &self.refckon())
                .field("bypshad", &self.bypshad())
                .field("fmt", &self.fmt())
                .field("ssruie", &self.ssruie())
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
                .field("itse", &self.itse())
                .field("tampts", &self.tampts())
                .field("tampoe", &self.tampoe())
                .field("alrfclr[0]", &self.alrfclr(0usize))
                .field("alrfclr[1]", &self.alrfclr(1usize))
                .field("tampalrm_pu", &self.tampalrm_pu())
                .field("tampalrm_type", &self.tampalrm_type())
                .field("out2en", &self.out2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ wucksel: {:?}, tsedge: {:?}, refckon: {=bool:?}, bypshad: {=bool:?}, fmt: {:?}, ssruie: {=bool:?}, alre[0]: {=bool:?}, alre[1]: {=bool:?}, wute: {=bool:?}, tse: {=bool:?}, alrie[0]: {=bool:?}, alrie[1]: {=bool:?}, wutie: {=bool:?}, tsie: {=bool:?}, add1h: {=bool:?}, sub1h: {=bool:?}, bkp: {=bool:?}, cosel: {:?}, pol: {:?}, osel: {:?}, coe: {=bool:?}, itse: {=bool:?}, tampts: {=bool:?}, tampoe: {=bool:?}, alrfclr[0]: {=bool:?}, alrfclr[1]: {=bool:?}, tampalrm_pu: {=bool:?}, tampalrm_type: {:?}, out2en: {=bool:?} }}" , self . wucksel () , self . tsedge () , self . refckon () , self . bypshad () , self . fmt () , self . ssruie () , self . alre (0usize) , self . alre (1usize) , self . wute () , self . tse () , self . alrie (0usize) , self . alrie (1usize) , self . wutie () , self . tsie () , self . add1h () , self . sub1h () , self . bkp () , self . cosel () , self . pol () , self . osel () , self . coe () , self . itse () , self . tampts () , self . tampoe () , self . alrfclr (0usize) , self . alrfclr (1usize) , self . tampalrm_pu () , self . tampalrm_type () , self . out2en ())
        }
    }
    #[doc = "RTC date register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Date units in BCD format."]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units in BCD format."]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Date tens in BCD format."]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format."]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Month units in BCD format."]
        #[inline(always)]
        pub const fn mu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Month units in BCD format."]
        #[inline(always)]
        pub fn set_mu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Month tens in BCD format."]
        #[inline(always)]
        pub const fn mt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Month tens in BCD format."]
        #[inline(always)]
        pub fn set_mt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Week day units."]
        #[inline(always)]
        pub const fn wdu(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Week day units."]
        #[inline(always)]
        pub fn set_wdu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Year units in BCD format."]
        #[inline(always)]
        pub const fn yu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Year units in BCD format."]
        #[inline(always)]
        pub fn set_yu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Year tens in BCD format."]
        #[inline(always)]
        pub const fn yt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Year tens in BCD format."]
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
    #[doc = "RTC initialization control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icsr(pub u32);
    impl Icsr {
        #[doc = "Wake-up timer write flag."]
        #[inline(always)]
        pub const fn wutwf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up timer write flag."]
        #[inline(always)]
        pub fn set_wutwf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Shift operation pending."]
        #[inline(always)]
        pub const fn shpf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Shift operation pending."]
        #[inline(always)]
        pub fn set_shpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Initialization status flag."]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization status flag."]
        #[inline(always)]
        pub fn set_inits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Registers synchronization flag."]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Registers synchronization flag."]
        #[inline(always)]
        pub fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Initialization flag."]
        #[inline(always)]
        pub const fn initf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization flag."]
        #[inline(always)]
        pub fn set_initf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Initialization mode."]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization mode."]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Binary mode."]
        #[inline(always)]
        pub const fn bin(&self) -> super::vals::Bin {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Bin::from_bits(val as u8)
        }
        #[doc = "Binary mode."]
        #[inline(always)]
        pub fn set_bin(&mut self, val: super::vals::Bin) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "BCD update (BIN = 10 or 11)."]
        #[inline(always)]
        pub const fn bcdu(&self) -> super::vals::Bcdu {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Bcdu::from_bits(val as u8)
        }
        #[doc = "BCD update (BIN = 10 or 11)."]
        #[inline(always)]
        pub fn set_bcdu(&mut self, val: super::vals::Bcdu) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "Recalibration pending Flag."]
        #[inline(always)]
        pub const fn recalpf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Recalibration pending Flag."]
        #[inline(always)]
        pub fn set_recalpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Icsr {
        #[inline(always)]
        fn default() -> Icsr {
            Icsr(0)
        }
    }
    impl core::fmt::Debug for Icsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icsr")
                .field("wutwf", &self.wutwf())
                .field("shpf", &self.shpf())
                .field("inits", &self.inits())
                .field("rsf", &self.rsf())
                .field("initf", &self.initf())
                .field("init", &self.init())
                .field("bin", &self.bin())
                .field("bcdu", &self.bcdu())
                .field("recalpf", &self.recalpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icsr {{ wutwf: {=bool:?}, shpf: {=bool:?}, inits: {=bool:?}, rsf: {=bool:?}, initf: {=bool:?}, init: {=bool:?}, bin: {:?}, bcdu: {:?}, recalpf: {=bool:?} }}" , self . wutwf () , self . shpf () , self . inits () , self . rsf () , self . initf () , self . init () , self . bin () , self . bcdu () , self . recalpf ())
        }
    }
    #[doc = "RTC nonsecure masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Alarm A masked flag."]
        #[inline(always)]
        pub const fn alrmf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm A masked flag."]
        #[inline(always)]
        pub fn set_alrmf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wake-up timer nonsecure masked flag."]
        #[inline(always)]
        pub const fn wutmf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up timer nonsecure masked flag."]
        #[inline(always)]
        pub fn set_wutmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp nonsecure masked flag."]
        #[inline(always)]
        pub const fn tsmf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp nonsecure masked flag."]
        #[inline(always)]
        pub fn set_tsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timestamp overflow nonsecure masked flag."]
        #[inline(always)]
        pub const fn tsovmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp overflow nonsecure masked flag."]
        #[inline(always)]
        pub fn set_tsovmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Internal timestamp nonsecure masked flag."]
        #[inline(always)]
        pub const fn itsmf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Internal timestamp nonsecure masked flag."]
        #[inline(always)]
        pub fn set_itsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SSR underflow nonsecure masked flag."]
        #[inline(always)]
        pub const fn ssrumf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SSR underflow nonsecure masked flag."]
        #[inline(always)]
        pub fn set_ssrumf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("alrmf[0]", &self.alrmf(0usize))
                .field("alrmf[1]", &self.alrmf(1usize))
                .field("wutmf", &self.wutmf())
                .field("tsmf", &self.tsmf())
                .field("tsovmf", &self.tsovmf())
                .field("itsmf", &self.itsmf())
                .field("ssrumf", &self.ssrumf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Misr {{ alrmf[0]: {=bool:?}, alrmf[1]: {=bool:?}, wutmf: {=bool:?}, tsmf: {=bool:?}, tsovmf: {=bool:?}, itsmf: {=bool:?}, ssrumf: {=bool:?} }}" , self . alrmf (0usize) , self . alrmf (1usize) , self . wutmf () , self . tsmf () , self . tsovmf () , self . itsmf () , self . ssrumf ())
        }
    }
    #[doc = "RTC prescaler register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prer(pub u32);
    impl Prer {
        #[doc = "Synchronous prescaler factor."]
        #[inline(always)]
        pub const fn prediv_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Synchronous prescaler factor."]
        #[inline(always)]
        pub fn set_prediv_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Asynchronous prescaler factor."]
        #[inline(always)]
        pub const fn prediv_a(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Asynchronous prescaler factor."]
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
    #[doc = "RTC privilege mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcr(pub u32);
    impl Privcr {
        #[doc = "Alarm and SSR underflow privilege protection."]
        #[inline(always)]
        pub const fn alrpriv(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm and SSR underflow privilege protection."]
        #[inline(always)]
        pub fn set_alrpriv(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wake-up timer privilege protection."]
        #[inline(always)]
        pub const fn wutpriv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up timer privilege protection."]
        #[inline(always)]
        pub fn set_wutpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp privilege protection."]
        #[inline(always)]
        pub const fn tspriv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp privilege protection."]
        #[inline(always)]
        pub fn set_tspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Shift register, Delight saving, calibration and reference clock privilege protection."]
        #[inline(always)]
        pub const fn calpriv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Shift register, Delight saving, calibration and reference clock privilege protection."]
        #[inline(always)]
        pub fn set_calpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Initialization privilege protection."]
        #[inline(always)]
        pub const fn initpriv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization privilege protection."]
        #[inline(always)]
        pub fn set_initpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "RTC privilege protection."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC privilege protection."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Privcr {
        #[inline(always)]
        fn default() -> Privcr {
            Privcr(0)
        }
    }
    impl core::fmt::Debug for Privcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcr")
                .field("alrpriv[0]", &self.alrpriv(0usize))
                .field("alrpriv[1]", &self.alrpriv(1usize))
                .field("wutpriv", &self.wutpriv())
                .field("tspriv", &self.tspriv())
                .field("calpriv", &self.calpriv())
                .field("initpriv", &self.initpriv())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privcr {{ alrpriv[0]: {=bool:?}, alrpriv[1]: {=bool:?}, wutpriv: {=bool:?}, tspriv: {=bool:?}, calpriv: {=bool:?}, initpriv: {=bool:?}, priv_: {=bool:?} }}" , self . alrpriv (0usize) , self . alrpriv (1usize) , self . wutpriv () , self . tspriv () , self . calpriv () , self . initpriv () , self . priv_ ())
        }
    }
    #[doc = "RTC status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear alarm A flag."]
        #[inline(always)]
        pub const fn calrf(&self, n: usize) -> super::vals::Calrf {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear alarm A flag."]
        #[inline(always)]
        pub fn set_calrf(&mut self, n: usize, val: super::vals::Calrf) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Clear wake-up timer flag."]
        #[inline(always)]
        pub const fn cwutf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear wake-up timer flag."]
        #[inline(always)]
        pub fn set_cwutf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear timestamp flag."]
        #[inline(always)]
        pub const fn ctsf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear timestamp flag."]
        #[inline(always)]
        pub fn set_ctsf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear timestamp overflow flag."]
        #[inline(always)]
        pub const fn ctsovf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear timestamp overflow flag."]
        #[inline(always)]
        pub fn set_ctsovf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear internal timestamp flag."]
        #[inline(always)]
        pub const fn citsf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear internal timestamp flag."]
        #[inline(always)]
        pub fn set_citsf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Clear SSR underflow flag."]
        #[inline(always)]
        pub const fn cssruf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear SSR underflow flag."]
        #[inline(always)]
        pub fn set_cssruf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field("calrf[0]", &self.calrf(0usize))
                .field("calrf[1]", &self.calrf(1usize))
                .field("cwutf", &self.cwutf())
                .field("ctsf", &self.ctsf())
                .field("ctsovf", &self.ctsovf())
                .field("citsf", &self.citsf())
                .field("cssruf", &self.cssruf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Scr {{ calrf[0]: {:?}, calrf[1]: {:?}, cwutf: {:?}, ctsf: {:?}, ctsovf: {:?}, citsf: {:?}, cssruf: {:?} }}" , self . calrf (0usize) , self . calrf (1usize) , self . cwutf () , self . ctsf () , self . ctsovf () , self . citsf () , self . cssruf ())
        }
    }
    #[doc = "RTC secure configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Alarm A and SSR underflow protection."]
        #[inline(always)]
        pub const fn alrasec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm A and SSR underflow protection."]
        #[inline(always)]
        pub fn set_alrasec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Alarm B protection."]
        #[inline(always)]
        pub const fn alrbsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm B protection."]
        #[inline(always)]
        pub fn set_alrbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wake-up timer protection."]
        #[inline(always)]
        pub const fn wutsec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up timer protection."]
        #[inline(always)]
        pub fn set_wutsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp protection."]
        #[inline(always)]
        pub const fn tssec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp protection."]
        #[inline(always)]
        pub fn set_tssec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Shift register, daylight saving, calibration and reference clock protection."]
        #[inline(always)]
        pub const fn calsec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Shift register, daylight saving, calibration and reference clock protection."]
        #[inline(always)]
        pub fn set_calsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Initialization protection."]
        #[inline(always)]
        pub const fn initsec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization protection."]
        #[inline(always)]
        pub fn set_initsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "RTC global protection."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC global protection."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("alrasec", &self.alrasec())
                .field("alrbsec", &self.alrbsec())
                .field("wutsec", &self.wutsec())
                .field("tssec", &self.tssec())
                .field("calsec", &self.calsec())
                .field("initsec", &self.initsec())
                .field("sec", &self.sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr {{ alrasec: {=bool:?}, alrbsec: {=bool:?}, wutsec: {=bool:?}, tssec: {=bool:?}, calsec: {=bool:?}, initsec: {=bool:?}, sec: {=bool:?} }}" , self . alrasec () , self . alrbsec () , self . wutsec () , self . tssec () , self . calsec () , self . initsec () , self . sec ())
        }
    }
    #[doc = "RTC shift control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shiftr(pub u32);
    impl Shiftr {
        #[doc = "Subtract a fraction of a second."]
        #[inline(always)]
        pub const fn subfs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Subtract a fraction of a second."]
        #[inline(always)]
        pub fn set_subfs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Add one second."]
        #[inline(always)]
        pub const fn add1s(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Add one second."]
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
    #[doc = "RTC secure masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smisr(pub u32);
    impl Smisr {
        #[doc = "Alarm A interrupt secure masked flag."]
        #[inline(always)]
        pub const fn alrmf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm A interrupt secure masked flag."]
        #[inline(always)]
        pub fn set_alrmf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wake-up timer interrupt secure masked flag."]
        #[inline(always)]
        pub const fn wutmf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up timer interrupt secure masked flag."]
        #[inline(always)]
        pub fn set_wutmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp interrupt secure masked flag."]
        #[inline(always)]
        pub const fn tsmf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp interrupt secure masked flag."]
        #[inline(always)]
        pub fn set_tsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timestamp overflow interrupt secure masked flag."]
        #[inline(always)]
        pub const fn tsovmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp overflow interrupt secure masked flag."]
        #[inline(always)]
        pub fn set_tsovmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Internal timestamp interrupt secure masked flag."]
        #[inline(always)]
        pub const fn itsmf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Internal timestamp interrupt secure masked flag."]
        #[inline(always)]
        pub fn set_itsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SSR underflow secure masked flag."]
        #[inline(always)]
        pub const fn ssrumf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SSR underflow secure masked flag."]
        #[inline(always)]
        pub fn set_ssrumf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Smisr {
        #[inline(always)]
        fn default() -> Smisr {
            Smisr(0)
        }
    }
    impl core::fmt::Debug for Smisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smisr")
                .field("alrmf[0]", &self.alrmf(0usize))
                .field("alrmf[1]", &self.alrmf(1usize))
                .field("wutmf", &self.wutmf())
                .field("tsmf", &self.tsmf())
                .field("tsovmf", &self.tsovmf())
                .field("itsmf", &self.itsmf())
                .field("ssrumf", &self.ssrumf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smisr {{ alrmf[0]: {=bool:?}, alrmf[1]: {=bool:?}, wutmf: {=bool:?}, tsmf: {=bool:?}, tsovmf: {=bool:?}, itsmf: {=bool:?}, ssrumf: {=bool:?} }}" , self . alrmf (0usize) , self . alrmf (1usize) , self . wutmf () , self . tsmf () , self . tsovmf () , self . itsmf () , self . ssrumf ())
        }
    }
    #[doc = "RTC status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Alarm flag."]
        #[inline(always)]
        pub const fn alrf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm flag."]
        #[inline(always)]
        pub fn set_alrf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wake-up timer flag."]
        #[inline(always)]
        pub const fn wutf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up timer flag."]
        #[inline(always)]
        pub fn set_wutf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp flag."]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp flag."]
        #[inline(always)]
        pub fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timestamp overflow flag."]
        #[inline(always)]
        pub const fn tsovf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp overflow flag."]
        #[inline(always)]
        pub fn set_tsovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Internal timestamp flag."]
        #[inline(always)]
        pub const fn itsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Internal timestamp flag."]
        #[inline(always)]
        pub fn set_itsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SSR underflow flag."]
        #[inline(always)]
        pub const fn ssruf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SSR underflow flag."]
        #[inline(always)]
        pub fn set_ssruf(&mut self, val: bool) {
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
                .field("alrf[0]", &self.alrf(0usize))
                .field("alrf[1]", &self.alrf(1usize))
                .field("wutf", &self.wutf())
                .field("tsf", &self.tsf())
                .field("tsovf", &self.tsovf())
                .field("itsf", &self.itsf())
                .field("ssruf", &self.ssruf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ alrf[0]: {=bool:?}, alrf[1]: {=bool:?}, wutf: {=bool:?}, tsf: {=bool:?}, tsovf: {=bool:?}, itsf: {=bool:?}, ssruf: {=bool:?} }}" , self . alrf (0usize) , self . alrf (1usize) , self . wutf () , self . tsf () , self . tsovf () , self . itsf () , self . ssruf ())
        }
    }
    #[doc = "RTC subsecond register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssr(pub u32);
    impl Ssr {
        #[doc = "Synchronous binary counter."]
        #[inline(always)]
        pub const fn ss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Synchronous binary counter."]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            defmt::write!(f, "Ssr {{ ss: {=u32:?} }}", self.ss())
        }
    }
    #[doc = "RTC timestamp on tamper control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tamptscr(pub u32);
    impl Tamptscr {
        #[doc = "Timestamp on external tamper TAMP1 event."]
        #[inline(always)]
        pub const fn tampts(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timestamp on external tamper TAMP1 event."]
        #[inline(always)]
        pub fn set_tampts(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timestamp on internal tamper event."]
        #[inline(always)]
        pub const fn itampts(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp on internal tamper event."]
        #[inline(always)]
        pub fn set_itampts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Tamptscr {
        #[inline(always)]
        fn default() -> Tamptscr {
            Tamptscr(0)
        }
    }
    impl core::fmt::Debug for Tamptscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tamptscr")
                .field("tampts[0]", &self.tampts(0usize))
                .field("tampts[1]", &self.tampts(1usize))
                .field("tampts[2]", &self.tampts(2usize))
                .field("tampts[3]", &self.tampts(3usize))
                .field("tampts[4]", &self.tampts(4usize))
                .field("itampts", &self.itampts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tamptscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Tamptscr {{ tampts[0]: {=bool:?}, tampts[1]: {=bool:?}, tampts[2]: {=bool:?}, tampts[3]: {=bool:?}, tampts[4]: {=bool:?}, itampts: {=bool:?} }}" , self . tampts (0usize) , self . tampts (1usize) , self . tampts (2usize) , self . tampts (3usize) , self . tampts (4usize) , self . itampts ())
        }
    }
    #[doc = "RTC time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tr(pub u32);
    impl Tr {
        #[doc = "Second units in BCD format."]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format."]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format."]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format."]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Minute units in BCD format."]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format."]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format."]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format."]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Hour units in BCD format."]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format."]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format."]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format."]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation."]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::Ampm {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ampm::from_bits(val as u8)
        }
        #[doc = "AM/PM notation."]
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
    #[doc = "RTC timestamp date register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsdr(pub u32);
    impl Tsdr {
        #[doc = "Date units in BCD format."]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units in BCD format."]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Date tens in BCD format."]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format."]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Month units in BCD format."]
        #[inline(always)]
        pub const fn mu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Month units in BCD format."]
        #[inline(always)]
        pub fn set_mu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Month tens in BCD format."]
        #[inline(always)]
        pub const fn mt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Month tens in BCD format."]
        #[inline(always)]
        pub fn set_mt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Week day units."]
        #[inline(always)]
        pub const fn wdu(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Week day units."]
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
    #[doc = "RTC timestamp status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsidr(pub u32);
    impl Tsidr {
        #[doc = "Timestamp flag source identification."]
        #[inline(always)]
        pub const fn tsid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Timestamp flag source identification."]
        #[inline(always)]
        pub fn set_tsid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Tsidr {
        #[inline(always)]
        fn default() -> Tsidr {
            Tsidr(0)
        }
    }
    impl core::fmt::Debug for Tsidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsidr").field("tsid", &self.tsid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tsidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tsidr {{ tsid: {=u8:?} }}", self.tsid())
        }
    }
    #[doc = "RTC timestamp subsecond register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsssr(pub u32);
    impl Tsssr {
        #[doc = "Subsecond value/synchronous binary counter values."]
        #[inline(always)]
        pub const fn ss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Subsecond value/synchronous binary counter values."]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            defmt::write!(f, "Tsssr {{ ss: {=u32:?} }}", self.ss())
        }
    }
    #[doc = "RTC timestamp time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tstr(pub u32);
    impl Tstr {
        #[doc = "Second units in BCD format."]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format."]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format."]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format."]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Minute units in BCD format."]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format."]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format."]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format."]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Hour units in BCD format."]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format."]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format."]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format."]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation."]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::Ampm {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ampm::from_bits(val as u8)
        }
        #[doc = "AM/PM notation."]
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
    #[doc = "RTC write protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpr(pub u32);
    impl Wpr {
        #[doc = "Write protection key."]
        #[inline(always)]
        pub const fn key(&self) -> super::vals::Key {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Key::from_bits(val as u8)
        }
        #[doc = "Write protection key."]
        #[inline(always)]
        pub fn set_key(&mut self, val: super::vals::Key) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
            defmt::write!(f, "Wpr {{ key: {:?} }}", self.key())
        }
    }
    #[doc = "RTC wake-up timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wutr(pub u32);
    impl Wutr {
        #[doc = "Wake-up auto-reload value bits."]
        #[inline(always)]
        pub const fn wut(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Wake-up auto-reload value bits."]
        #[inline(always)]
        pub fn set_wut(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Wake-up auto-reload output clear value."]
        #[inline(always)]
        pub const fn wutoclr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Wake-up auto-reload output clear value."]
        #[inline(always)]
        pub fn set_wutoclr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            f.debug_struct("Wutr")
                .field("wut", &self.wut())
                .field("wutoclr", &self.wutoclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wutr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wutr {{ wut: {=u16:?}, wutoclr: {=u16:?} }}",
                self.wut(),
                self.wutoclr()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AlrmrWdsel {
        #[doc = "DU\\[3:0\\]
represents the date units."]
        DATE_UNITS = 0x0,
        #[doc = "DU\\[3:0\\]
represents the week day."]
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
    pub enum AlrmssrSsclr {
        #[doc = "The synchronous binary counter (SS\\[31:0\\]
in SSR) is free-running."]
        FREE_RUNNING = 0x0,
        #[doc = "The synchronous binary counter (SS\\[31:0\\]
in SSR) is running from 0xFFFFFFFF to ALRBBINR."]
        ALRMBINR = 0x01,
    }
    impl AlrmssrSsclr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AlrmssrSsclr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AlrmssrSsclr {
        #[inline(always)]
        fn from(val: u8) -> AlrmssrSsclr {
            AlrmssrSsclr::from_bits(val)
        }
    }
    impl From<AlrmssrSsclr> for u8 {
        #[inline(always)]
        fn from(val: AlrmssrSsclr) -> u8 {
            AlrmssrSsclr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ampm {
        #[doc = "AM or 24-hour format."]
        AM = 0x0,
        #[doc = "PM."]
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
    pub enum Bcdu {
        #[doc = "1s calendar increment is generated each time SS\\[7:0\\]
= 0."]
        BIT7 = 0x0,
        #[doc = "1s calendar increment is generated each time SS\\[8:0\\]
= 0."]
        BIT8 = 0x01,
        #[doc = "1s calendar increment is generated each time SS\\[9:0\\]
= 0."]
        BIT9 = 0x02,
        #[doc = "1s calendar increment is generated each time SS\\[10:0\\]
= 0."]
        BIT10 = 0x03,
        #[doc = "1s calendar increment is generated each time SS\\[11:0\\]
= 0."]
        BIT11 = 0x04,
        #[doc = "1s calendar increment is generated each time SS\\[12:0\\]
= 0."]
        BIT12 = 0x05,
        #[doc = "1s calendar increment is generated each time SS\\[13:0\\]
= 0."]
        BIT13 = 0x06,
        #[doc = "1s calendar increment is generated each time SS\\[14:0\\]
= 0."]
        BIT14 = 0x07,
    }
    impl Bcdu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bcdu {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bcdu {
        #[inline(always)]
        fn from(val: u8) -> Bcdu {
            Bcdu::from_bits(val)
        }
    }
    impl From<Bcdu> for u8 {
        #[inline(always)]
        fn from(val: Bcdu) -> u8 {
            Bcdu::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bin {
        #[doc = "Free running BCD calendar mode (Binary mode disabled)."]
        BCD = 0x0,
        #[doc = "Free running Binary mode (BCD mode disabled)."]
        BINARY = 0x01,
        #[doc = "Free running BCD calendar and Binary modes."]
        BIN_BCD = 0x02,
        #[doc = "Free running BCD calendar and Binary modes."]
        BIN_BCD2 = 0x03,
    }
    impl Bin {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bin {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bin {
        #[inline(always)]
        fn from(val: u8) -> Bin {
            Bin::from_bits(val)
        }
    }
    impl From<Bin> for u8 {
        #[inline(always)]
        fn from(val: Bin) -> u8 {
            Bin::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calp {
        #[doc = "No RTCCLK pulses are added."]
        NO_CHANGE = 0x0,
        #[doc = "One RTCCLK pulse is effectively inserted every 2less thansup>11less than/sup> pulses (frequency increased by 488."]
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
    pub enum Calrf {
        _RESERVED_0 = 0x0,
        #[doc = "Clear interrupt flag by writing 1"]
        CLEAR = 0x01,
    }
    impl Calrf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calrf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calrf {
        #[inline(always)]
        fn from(val: u8) -> Calrf {
            Calrf::from_bits(val)
        }
    }
    impl From<Calrf> for u8 {
        #[inline(always)]
        fn from(val: Calrf) -> u8 {
            Calrf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calw16 {
        _RESERVED_0 = 0x0,
        #[doc = "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
        SIXTEEN_SECONDS = 0x01,
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
        EIGHT_SECONDS = 0x01,
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
        #[doc = "Calibration output is 512Hz."]
        CAL_FREQ_512HZ = 0x0,
        #[doc = "Calibration output is 1Hz."]
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
        #[doc = "24 hour/day format."]
        TWENTY_FOUR_HOUR = 0x0,
        #[doc = "AM/PM hour format."]
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key(u8);
    impl Key {
        #[doc = "Activate write protection (any value that is not the keys)"]
        pub const ACTIVATE: Self = Self(0x0);
        #[doc = "Key 2"]
        pub const DEACTIVATE2: Self = Self(0x53);
        #[doc = "Key 1"]
        pub const DEACTIVATE1: Self = Self(0xca);
    }
    impl Key {
        pub const fn from_bits(val: u8) -> Key {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Key {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("ACTIVATE"),
                0x53 => f.write_str("DEACTIVATE2"),
                0xca => f.write_str("DEACTIVATE1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "ACTIVATE"),
                0x53 => defmt::write!(f, "DEACTIVATE2"),
                0xca => defmt::write!(f, "DEACTIVATE1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Key {
        #[inline(always)]
        fn from(val: u8) -> Key {
            Key::from_bits(val)
        }
    }
    impl From<Key> for u8 {
        #[inline(always)]
        fn from(val: Key) -> u8 {
            Key::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpcal {
        #[doc = "Calibration window is 2less thansup>20less than/sup> RTCCLK, which is a high-consumption mode."]
        RTCCLK = 0x0,
        #[doc = "Calibration window is 2less thansup>20less than/sup> ck_apre, which is the required configuration for ultra-low consumption mode."]
        CK_APRE = 0x01,
    }
    impl Lpcal {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpcal {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpcal {
        #[inline(always)]
        fn from(val: u8) -> Lpcal {
            Lpcal::from_bits(val)
        }
    }
    impl From<Lpcal> for u8 {
        #[inline(always)]
        fn from(val: Lpcal) -> u8 {
            Lpcal::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Osel {
        #[doc = "Output disabled."]
        DISABLED = 0x0,
        #[doc = "Alarm A output enabled."]
        ALARM_A = 0x01,
        #[doc = "Alarm B output enabled."]
        ALARM_B = 0x02,
        #[doc = "Wake-up output enabled."]
        WAKE_UP = 0x03,
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
        #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
        HIGH = 0x0,
        #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
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
    pub enum TampalrmType {
        #[doc = "TAMPALRM is push-pull output."]
        PUSH_PULL = 0x0,
        #[doc = "TAMPALRM is open-drain output."]
        OPEN_DRAIN = 0x01,
    }
    impl TampalrmType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TampalrmType {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TampalrmType {
        #[inline(always)]
        fn from(val: u8) -> TampalrmType {
            TampalrmType::from_bits(val)
        }
    }
    impl From<TampalrmType> for u8 {
        #[inline(always)]
        fn from(val: TampalrmType) -> u8 {
            TampalrmType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tsedge {
        #[doc = "TS input rising edge generates a timestamp event."]
        RISING_EDGE = 0x0,
        #[doc = "TS input falling edge generates a timestamp event."]
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
        #[doc = "RTC/16 clock is selected."]
        DIV16 = 0x0,
        #[doc = "RTC/8 clock is selected."]
        DIV8 = 0x01,
        #[doc = "RTC/4 clock is selected."]
        DIV4 = 0x02,
        #[doc = "RTC/2 clock is selected."]
        DIV2 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
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
