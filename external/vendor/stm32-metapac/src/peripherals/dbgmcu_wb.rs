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
    #[doc = "MCU Device ID Code Register"]
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::Idcode, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Debug MCU Configuration Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "APB1 Low Freeze Register CPU1"]
    #[inline(always)]
    pub const fn apb1fzr1(self) -> crate::common::Reg<regs::Apb1fzr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "APB1 Low Freeze Register CPU2"]
    #[inline(always)]
    pub const fn c2ap_b1fzr1(self) -> crate::common::Reg<regs::C2apB1fzr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "APB1 High Freeze Register CPU1"]
    #[inline(always)]
    pub const fn apb1fzr2(self) -> crate::common::Reg<regs::Apb1fzr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "APB1 High Freeze Register CPU2"]
    #[inline(always)]
    pub const fn c2apb1fzr2(self) -> crate::common::Reg<regs::C2apb1fzr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "APB2 Freeze Register CPU2"]
    #[inline(always)]
    pub const fn c2apb2fzr(self) -> crate::common::Reg<regs::C2apb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "APB2 Freeze Register CPU1"]
    #[inline(always)]
    pub const fn apb2fzr(self) -> crate::common::Reg<regs::Apb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "APB1 Low Freeze Register CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1fzr1(pub u32);
    impl Apb1fzr1 {
        #[doc = "Debug Timer 2 stopped when Core is halted"]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Timer 2 stopped when Core is halted"]
        #[inline(always)]
        pub fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTC counter stopped when core is halted"]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "WWDG counter stopped when core is halted"]
        #[inline(always)]
        pub const fn wwdg(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_wwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IWDG counter stopped when core is halted"]
        #[inline(always)]
        pub const fn iwdg(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_iwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Debug I2C1 SMBUS timeout stopped when Core is halted"]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Debug I2C1 SMBUS timeout stopped when Core is halted"]
        #[inline(always)]
        pub fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Debug I2C3 SMBUS timeout stopped when core is halted"]
        #[inline(always)]
        pub const fn i2c3(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Debug I2C3 SMBUS timeout stopped when core is halted"]
        #[inline(always)]
        pub fn set_i2c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Debug LPTIM1 stopped when Core is halted"]
        #[inline(always)]
        pub const fn lptim1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Debug LPTIM1 stopped when Core is halted"]
        #[inline(always)]
        pub fn set_lptim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1fzr1 {
        #[inline(always)]
        fn default() -> Apb1fzr1 {
            Apb1fzr1(0)
        }
    }
    impl core::fmt::Debug for Apb1fzr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1fzr1")
                .field("tim2", &self.tim2())
                .field("rtc", &self.rtc())
                .field("wwdg", &self.wwdg())
                .field("iwdg", &self.iwdg())
                .field("i2c1", &self.i2c1())
                .field("i2c3", &self.i2c3())
                .field("lptim1", &self.lptim1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1fzr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1fzr1 {{ tim2: {=bool:?}, rtc: {=bool:?}, wwdg: {=bool:?}, iwdg: {=bool:?}, i2c1: {=bool:?}, i2c3: {=bool:?}, lptim1: {=bool:?} }}" , self . tim2 () , self . rtc () , self . wwdg () , self . iwdg () , self . i2c1 () , self . i2c3 () , self . lptim1 ())
        }
    }
    #[doc = "APB1 High Freeze Register CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1fzr2(pub u32);
    impl Apb1fzr2 {
        #[doc = "LPTIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn lptim2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_lptim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Apb1fzr2 {
        #[inline(always)]
        fn default() -> Apb1fzr2 {
            Apb1fzr2(0)
        }
    }
    impl core::fmt::Debug for Apb1fzr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1fzr2").field("lptim2", &self.lptim2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1fzr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb1fzr2 {{ lptim2: {=bool:?} }}", self.lptim2())
        }
    }
    #[doc = "APB2 Freeze Register CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr(pub u32);
    impl Apb2fzr {
        #[doc = "TIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM16 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim16(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim17(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Apb2fzr {
        #[inline(always)]
        fn default() -> Apb2fzr {
            Apb2fzr(0)
        }
    }
    impl core::fmt::Debug for Apb2fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2fzr")
                .field("tim1", &self.tim1())
                .field("tim16", &self.tim16())
                .field("tim17", &self.tim17())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2fzr {{ tim1: {=bool:?}, tim16: {=bool:?}, tim17: {=bool:?} }}",
                self.tim1(),
                self.tim16(),
                self.tim17()
            )
        }
    }
    #[doc = "APB1 Low Freeze Register CPU2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2apB1fzr1(pub u32);
    impl C2apB1fzr1 {
        #[doc = "LPTIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn lptim2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_lptim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTC counter stopped when core is halted"]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "IWDG stopped when core is halted"]
        #[inline(always)]
        pub const fn iwdg(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG stopped when core is halted"]
        #[inline(always)]
        pub fn set_iwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "I2C1 SMBUS timeout stopped when core is halted"]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 SMBUS timeout stopped when core is halted"]
        #[inline(always)]
        pub fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C3 SMBUS timeout stopped when core is halted"]
        #[inline(always)]
        pub const fn i2c3(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 SMBUS timeout stopped when core is halted"]
        #[inline(always)]
        pub fn set_i2c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "LPTIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn lptim1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_lptim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C2apB1fzr1 {
        #[inline(always)]
        fn default() -> C2apB1fzr1 {
            C2apB1fzr1(0)
        }
    }
    impl core::fmt::Debug for C2apB1fzr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C2apB1fzr1")
                .field("lptim2", &self.lptim2())
                .field("rtc", &self.rtc())
                .field("iwdg", &self.iwdg())
                .field("i2c1", &self.i2c1())
                .field("i2c3", &self.i2c3())
                .field("lptim1", &self.lptim1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C2apB1fzr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "C2apB1fzr1 {{ lptim2: {=bool:?}, rtc: {=bool:?}, iwdg: {=bool:?}, i2c1: {=bool:?}, i2c3: {=bool:?}, lptim1: {=bool:?} }}" , self . lptim2 () , self . rtc () , self . iwdg () , self . i2c1 () , self . i2c3 () , self . lptim1 ())
        }
    }
    #[doc = "APB1 High Freeze Register CPU2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2apb1fzr2(pub u32);
    impl C2apb1fzr2 {
        #[doc = "LPTIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn lptim2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_lptim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for C2apb1fzr2 {
        #[inline(always)]
        fn default() -> C2apb1fzr2 {
            C2apb1fzr2(0)
        }
    }
    impl core::fmt::Debug for C2apb1fzr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C2apb1fzr2").field("lptim2", &self.lptim2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C2apb1fzr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "C2apb1fzr2 {{ lptim2: {=bool:?} }}", self.lptim2())
        }
    }
    #[doc = "APB2 Freeze Register CPU2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2apb2fzr(pub u32);
    impl C2apb2fzr {
        #[doc = "TIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM16 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim16(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim17(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for C2apb2fzr {
        #[inline(always)]
        fn default() -> C2apb2fzr {
            C2apb2fzr(0)
        }
    }
    impl core::fmt::Debug for C2apb2fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C2apb2fzr")
                .field("tim1", &self.tim1())
                .field("tim16", &self.tim16())
                .field("tim17", &self.tim17())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C2apb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "C2apb2fzr {{ tim1: {=bool:?}, tim16: {=bool:?}, tim17: {=bool:?} }}",
                self.tim1(),
                self.tim16(),
                self.tim17()
            )
        }
    }
    #[doc = "Debug MCU Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Debug Sleep Mode"]
        #[inline(always)]
        pub const fn dbg_sleep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Sleep Mode"]
        #[inline(always)]
        pub fn set_dbg_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Debug Stop Mode"]
        #[inline(always)]
        pub const fn dbg_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Stop Mode"]
        #[inline(always)]
        pub fn set_dbg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Debug Standby Mode"]
        #[inline(always)]
        pub const fn dbg_standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Standby Mode"]
        #[inline(always)]
        pub fn set_dbg_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Trace port and clock enable"]
        #[inline(always)]
        pub const fn trace_ioen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trace port and clock enable"]
        #[inline(always)]
        pub fn set_trace_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("dbg_sleep", &self.dbg_sleep())
                .field("dbg_stop", &self.dbg_stop())
                .field("dbg_standby", &self.dbg_standby())
                .field("trace_ioen", &self.trace_ioen())
                .field("trgoen", &self.trgoen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ dbg_sleep: {=bool:?}, dbg_stop: {=bool:?}, dbg_standby: {=bool:?}, trace_ioen: {=bool:?}, trgoen: {=bool:?} }}" , self . dbg_sleep () , self . dbg_stop () , self . dbg_standby () , self . trace_ioen () , self . trgoen ())
        }
    }
    #[doc = "MCU Device ID Code Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "Device Identifier"]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Device Identifier"]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Revision Identifier"]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision Identifier"]
        #[inline(always)]
        pub fn set_rev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Idcode {
        #[inline(always)]
        fn default() -> Idcode {
            Idcode(0)
        }
    }
    impl core::fmt::Debug for Idcode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idcode")
                .field("dev_id", &self.dev_id())
                .field("rev_id", &self.rev_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idcode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idcode {{ dev_id: {=u16:?}, rev_id: {=u16:?} }}",
                self.dev_id(),
                self.rev_id()
            )
        }
    }
}
