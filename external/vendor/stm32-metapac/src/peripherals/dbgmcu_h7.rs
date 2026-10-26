#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Debug support"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgmcu {
    ptr: *mut u8,
}
unsafe impl Send for Dbgmcu {}
unsafe impl Sync for Dbgmcu {}
impl Dbgmcu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Identity code"]
    #[inline(always)]
    pub const fn idc(self) -> crate::common::Reg<regs::Idc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "APB3 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb3fzr1(self) -> crate::common::Reg<regs::Apb3fzr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "APB1L peripheral freeze register"]
    #[inline(always)]
    pub const fn apb1lfzr1(self) -> crate::common::Reg<regs::Apb1lfzr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "APB2 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb2fzr1(self) -> crate::common::Reg<regs::Apb2fzr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "APB4 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb4fzr1(self) -> crate::common::Reg<regs::Apb4fzr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
}
pub mod regs {
    #[doc = "APB1L peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lfzr1(pub u32);
    impl Apb1lfzr1 {
        #[doc = "TIM2 stop in debug mode"]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 stop in debug mode"]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 stop in debug mode"]
        #[inline(always)]
        pub const fn tim4(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 stop in debug mode"]
        #[inline(always)]
        pub const fn tim5(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 stop in debug mode"]
        #[inline(always)]
        pub const fn tim6(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 stop in debug mode"]
        #[inline(always)]
        pub const fn tim7(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 stop in debug mode"]
        #[inline(always)]
        pub const fn tim12(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 stop in debug mode"]
        #[inline(always)]
        pub const fn tim13(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 stop in debug mode"]
        #[inline(always)]
        pub const fn tim14(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 stop in debug mode"]
        #[inline(always)]
        pub const fn lptim1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 stop in debug mode"]
        #[inline(always)]
        pub fn set_lptim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "I2C1 SMBUS timeout stop in debug mode"]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 SMBUS timeout stop in debug mode"]
        #[inline(always)]
        pub fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 SMBUS timeout stop in debug mode"]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 SMBUS timeout stop in debug mode"]
        #[inline(always)]
        pub fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 SMBUS timeout stop in debug mode"]
        #[inline(always)]
        pub const fn i2c3(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 SMBUS timeout stop in debug mode"]
        #[inline(always)]
        pub fn set_i2c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1lfzr1 {
        #[inline(always)]
        fn default() -> Apb1lfzr1 {
            Apb1lfzr1(0)
        }
    }
    impl core::fmt::Debug for Apb1lfzr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lfzr1")
                .field("tim2", &self.tim2())
                .field("tim3", &self.tim3())
                .field("tim4", &self.tim4())
                .field("tim5", &self.tim5())
                .field("tim6", &self.tim6())
                .field("tim7", &self.tim7())
                .field("tim12", &self.tim12())
                .field("tim13", &self.tim13())
                .field("tim14", &self.tim14())
                .field("lptim1", &self.lptim1())
                .field("i2c1", &self.i2c1())
                .field("i2c2", &self.i2c2())
                .field("i2c3", &self.i2c3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lfzr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1lfzr1 {{ tim2: {=bool:?}, tim3: {=bool:?}, tim4: {=bool:?}, tim5: {=bool:?}, tim6: {=bool:?}, tim7: {=bool:?}, tim12: {=bool:?}, tim13: {=bool:?}, tim14: {=bool:?}, lptim1: {=bool:?}, i2c1: {=bool:?}, i2c2: {=bool:?}, i2c3: {=bool:?} }}" , self . tim2 () , self . tim3 () , self . tim4 () , self . tim5 () , self . tim6 () , self . tim7 () , self . tim12 () , self . tim13 () , self . tim14 () , self . lptim1 () , self . i2c1 () , self . i2c2 () , self . i2c3 ())
        }
    }
    #[doc = "APB2 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr1(pub u32);
    impl Apb2fzr1 {
        #[doc = "TIM1 stop in debug mode"]
        #[inline(always)]
        pub const fn tim1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 stop in debug mode"]
        #[inline(always)]
        pub const fn tim8(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM15 stop in debug mode"]
        #[inline(always)]
        pub const fn tim15(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 stop in debug mode"]
        #[inline(always)]
        pub const fn tim16(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 stop in debug mode"]
        #[inline(always)]
        pub const fn tim17(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 stop in debug mode"]
        #[inline(always)]
        pub fn set_tim17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HRTIM stop in debug mode"]
        #[inline(always)]
        pub const fn hrtim(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM stop in debug mode"]
        #[inline(always)]
        pub fn set_hrtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb2fzr1 {
        #[inline(always)]
        fn default() -> Apb2fzr1 {
            Apb2fzr1(0)
        }
    }
    impl core::fmt::Debug for Apb2fzr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2fzr1")
                .field("tim1", &self.tim1())
                .field("tim8", &self.tim8())
                .field("tim15", &self.tim15())
                .field("tim16", &self.tim16())
                .field("tim17", &self.tim17())
                .field("hrtim", &self.hrtim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fzr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2fzr1 {{ tim1: {=bool:?}, tim8: {=bool:?}, tim15: {=bool:?}, tim16: {=bool:?}, tim17: {=bool:?}, hrtim: {=bool:?} }}" , self . tim1 () , self . tim8 () , self . tim15 () , self . tim16 () , self . tim17 () , self . hrtim ())
        }
    }
    #[doc = "APB3 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3fzr1(pub u32);
    impl Apb3fzr1 {
        #[doc = "WWDG1 stop in debug mode"]
        #[inline(always)]
        pub const fn wwdg1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 stop in debug mode"]
        #[inline(always)]
        pub fn set_wwdg1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Apb3fzr1 {
        #[inline(always)]
        fn default() -> Apb3fzr1 {
            Apb3fzr1(0)
        }
    }
    impl core::fmt::Debug for Apb3fzr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3fzr1").field("wwdg1", &self.wwdg1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3fzr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb3fzr1 {{ wwdg1: {=bool:?} }}", self.wwdg1())
        }
    }
    #[doc = "APB4 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4fzr1(pub u32);
    impl Apb4fzr1 {
        #[doc = "I2C4 SMBUS timeout stop in debug mode"]
        #[inline(always)]
        pub const fn i2c4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 SMBUS timeout stop in debug mode"]
        #[inline(always)]
        pub fn set_i2c4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 stop in debug mode"]
        #[inline(always)]
        pub const fn lptim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 stop in debug mode"]
        #[inline(always)]
        pub fn set_lptim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 stop in debug mode"]
        #[inline(always)]
        pub const fn lptim3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 stop in debug mode"]
        #[inline(always)]
        pub fn set_lptim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 stop in debug mode"]
        #[inline(always)]
        pub const fn lptim4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 stop in debug mode"]
        #[inline(always)]
        pub fn set_lptim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 stop in debug mode"]
        #[inline(always)]
        pub const fn lptim5(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 stop in debug mode"]
        #[inline(always)]
        pub fn set_lptim5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RTC stop in debug mode"]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC stop in debug mode"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Independent watchdog for D1 stop in debug mode"]
        #[inline(always)]
        pub const fn iwdg1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog for D1 stop in debug mode"]
        #[inline(always)]
        pub fn set_iwdg1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Apb4fzr1 {
        #[inline(always)]
        fn default() -> Apb4fzr1 {
            Apb4fzr1(0)
        }
    }
    impl core::fmt::Debug for Apb4fzr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb4fzr1")
                .field("i2c4", &self.i2c4())
                .field("lptim2", &self.lptim2())
                .field("lptim3", &self.lptim3())
                .field("lptim4", &self.lptim4())
                .field("lptim5", &self.lptim5())
                .field("rtc", &self.rtc())
                .field("iwdg1", &self.iwdg1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4fzr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb4fzr1 {{ i2c4: {=bool:?}, lptim2: {=bool:?}, lptim3: {=bool:?}, lptim4: {=bool:?}, lptim5: {=bool:?}, rtc: {=bool:?}, iwdg1: {=bool:?} }}" , self . i2c4 () , self . lptim2 () , self . lptim3 () , self . lptim4 () , self . lptim5 () , self . rtc () , self . iwdg1 ())
        }
    }
    #[doc = "Configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Allow debug in D1 Sleep mode"]
        #[inline(always)]
        pub const fn dbgsleep_d1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Allow debug in D1 Sleep mode"]
        #[inline(always)]
        pub fn set_dbgsleep_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Allow debug in D1 Stop mode"]
        #[inline(always)]
        pub const fn dbgstop_d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Allow debug in D1 Stop mode"]
        #[inline(always)]
        pub fn set_dbgstop_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Allow debug in D1 Standby mode"]
        #[inline(always)]
        pub const fn dbgstby_d1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Allow debug in D1 Standby mode"]
        #[inline(always)]
        pub fn set_dbgstby_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Trace clock enable enable"]
        #[inline(always)]
        pub const fn traceclken(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Trace clock enable enable"]
        #[inline(always)]
        pub fn set_traceclken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "D1 debug clock enable enable"]
        #[inline(always)]
        pub const fn d1dbgcken(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "D1 debug clock enable enable"]
        #[inline(always)]
        pub fn set_d1dbgcken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "D3 debug clock enable enable"]
        #[inline(always)]
        pub const fn d3dbgcken(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "D3 debug clock enable enable"]
        #[inline(always)]
        pub fn set_d3dbgcken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "External trigger output enable"]
        #[inline(always)]
        pub const fn trgoen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger output enable"]
        #[inline(always)]
        pub fn set_trgoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("dbgsleep_d1", &self.dbgsleep_d1())
                .field("dbgstop_d1", &self.dbgstop_d1())
                .field("dbgstby_d1", &self.dbgstby_d1())
                .field("traceclken", &self.traceclken())
                .field("d1dbgcken", &self.d1dbgcken())
                .field("d3dbgcken", &self.d3dbgcken())
                .field("trgoen", &self.trgoen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ dbgsleep_d1: {=bool:?}, dbgstop_d1: {=bool:?}, dbgstby_d1: {=bool:?}, traceclken: {=bool:?}, d1dbgcken: {=bool:?}, d3dbgcken: {=bool:?}, trgoen: {=bool:?} }}" , self . dbgsleep_d1 () , self . dbgstop_d1 () , self . dbgstby_d1 () , self . traceclken () , self . d1dbgcken () , self . d3dbgcken () , self . trgoen ())
        }
    }
    #[doc = "Identity code"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idc(pub u32);
    impl Idc {
        #[doc = "Device ID"]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Device ID"]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Revision ID"]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision ID"]
        #[inline(always)]
        pub fn set_rev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Idc {
        #[inline(always)]
        fn default() -> Idc {
            Idc(0)
        }
    }
    impl core::fmt::Debug for Idc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idc")
                .field("dev_id", &self.dev_id())
                .field("rev_id", &self.rev_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idc {{ dev_id: {=u16:?}, rev_id: {=u16:?} }}",
                self.dev_id(),
                self.rev_id()
            )
        }
    }
}
