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
    #[doc = "IDCODE"]
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::Idcode, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Debug MCU APB1 Freeze register"]
    #[inline(always)]
    pub const fn apb1fzr(self) -> crate::common::Reg<regs::Apb1fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Debug MCU APB2 Freeze register"]
    #[inline(always)]
    pub const fn apb2fzr(self) -> crate::common::Reg<regs::Apb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Debug MCU APB1 Freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1fzr(pub u32);
    impl Apb1fzr {
        #[doc = "TIM2"]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2"]
        #[inline(always)]
        pub fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3"]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3"]
        #[inline(always)]
        pub fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4"]
        #[inline(always)]
        pub const fn tim4(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4"]
        #[inline(always)]
        pub fn set_tim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5"]
        #[inline(always)]
        pub const fn tim5(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5"]
        #[inline(always)]
        pub fn set_tim5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6"]
        #[inline(always)]
        pub const fn tim6(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6"]
        #[inline(always)]
        pub fn set_tim6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7"]
        #[inline(always)]
        pub const fn tim7(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7"]
        #[inline(always)]
        pub fn set_tim7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12"]
        #[inline(always)]
        pub const fn tim12(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12"]
        #[inline(always)]
        pub fn set_tim12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13"]
        #[inline(always)]
        pub const fn tim13(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13"]
        #[inline(always)]
        pub fn set_tim13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14"]
        #[inline(always)]
        pub const fn tim14(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14"]
        #[inline(always)]
        pub fn set_tim14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1"]
        #[inline(always)]
        pub const fn lptim1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1"]
        #[inline(always)]
        pub fn set_lptim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RTC"]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "WWDG"]
        #[inline(always)]
        pub const fn wwdg(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG"]
        #[inline(always)]
        pub fn set_wwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IWDG"]
        #[inline(always)]
        pub const fn iwdg(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG"]
        #[inline(always)]
        pub fn set_iwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CAN3"]
        #[inline(always)]
        pub const fn can3(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CAN3"]
        #[inline(always)]
        pub fn set_can3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DBG_I2C1_SMBUS_TIMEOUT"]
        #[inline(always)]
        pub const fn dbg_i2c1_smbus_timeout(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DBG_I2C1_SMBUS_TIMEOUT"]
        #[inline(always)]
        pub fn set_dbg_i2c1_smbus_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DBG_I2C2_SMBUS_TIMEOUT"]
        #[inline(always)]
        pub const fn dbg_i2c2_smbus_timeout(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DBG_I2C2_SMBUS_TIMEOUT"]
        #[inline(always)]
        pub fn set_dbg_i2c2_smbus_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DBG_I2C3_SMBUS_TIMEOUT"]
        #[inline(always)]
        pub const fn dbg_i2c3_smbus_timeout(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "DBG_I2C3_SMBUS_TIMEOUT"]
        #[inline(always)]
        pub fn set_dbg_i2c3_smbus_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "DBG_I2C4SMBUS_TIMEOUT"]
        #[inline(always)]
        pub const fn dbg_i2c4_smbus_timeout(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DBG_I2C4SMBUS_TIMEOUT"]
        #[inline(always)]
        pub fn set_dbg_i2c4_smbus_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN1"]
        #[inline(always)]
        pub const fn can1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1"]
        #[inline(always)]
        pub fn set_can1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN2"]
        #[inline(always)]
        pub const fn can2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2"]
        #[inline(always)]
        pub fn set_can2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb1fzr {
        #[inline(always)]
        fn default() -> Apb1fzr {
            Apb1fzr(0)
        }
    }
    impl core::fmt::Debug for Apb1fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1fzr")
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
                .field("rtc", &self.rtc())
                .field("wwdg", &self.wwdg())
                .field("iwdg", &self.iwdg())
                .field("can3", &self.can3())
                .field("dbg_i2c1_smbus_timeout", &self.dbg_i2c1_smbus_timeout())
                .field("dbg_i2c2_smbus_timeout", &self.dbg_i2c2_smbus_timeout())
                .field("dbg_i2c3_smbus_timeout", &self.dbg_i2c3_smbus_timeout())
                .field("dbg_i2c4_smbus_timeout", &self.dbg_i2c4_smbus_timeout())
                .field("can1", &self.can1())
                .field("can2", &self.can2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1fzr {{ tim2: {=bool:?}, tim3: {=bool:?}, tim4: {=bool:?}, tim5: {=bool:?}, tim6: {=bool:?}, tim7: {=bool:?}, tim12: {=bool:?}, tim13: {=bool:?}, tim14: {=bool:?}, lptim1: {=bool:?}, rtc: {=bool:?}, wwdg: {=bool:?}, iwdg: {=bool:?}, can3: {=bool:?}, dbg_i2c1_smbus_timeout: {=bool:?}, dbg_i2c2_smbus_timeout: {=bool:?}, dbg_i2c3_smbus_timeout: {=bool:?}, dbg_i2c4_smbus_timeout: {=bool:?}, can1: {=bool:?}, can2: {=bool:?} }}" , self . tim2 () , self . tim3 () , self . tim4 () , self . tim5 () , self . tim6 () , self . tim7 () , self . tim12 () , self . tim13 () , self . tim14 () , self . lptim1 () , self . rtc () , self . wwdg () , self . iwdg () , self . can3 () , self . dbg_i2c1_smbus_timeout () , self . dbg_i2c2_smbus_timeout () , self . dbg_i2c3_smbus_timeout () , self . dbg_i2c4_smbus_timeout () , self . can1 () , self . can2 ())
        }
    }
    #[doc = "Debug MCU APB2 Freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr(pub u32);
    impl Apb2fzr {
        #[doc = "TIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim8(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM9 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim9(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM10 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim10(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM11 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim11(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim11(&mut self, val: bool) {
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
                .field("tim8", &self.tim8())
                .field("tim9", &self.tim9())
                .field("tim10", &self.tim10())
                .field("tim11", &self.tim11())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2fzr {{ tim1: {=bool:?}, tim8: {=bool:?}, tim9: {=bool:?}, tim10: {=bool:?}, tim11: {=bool:?} }}",
                self.tim1(),
                self.tim8(),
                self.tim9(),
                self.tim10(),
                self.tim11()
            )
        }
    }
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "DBG_SLEEP"]
        #[inline(always)]
        pub const fn dbg_sleep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DBG_SLEEP"]
        #[inline(always)]
        pub fn set_dbg_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DBG_STOP"]
        #[inline(always)]
        pub const fn dbg_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DBG_STOP"]
        #[inline(always)]
        pub fn set_dbg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DBG_STANDBY"]
        #[inline(always)]
        pub const fn dbg_standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DBG_STANDBY"]
        #[inline(always)]
        pub fn set_dbg_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TRACE_IOEN"]
        #[inline(always)]
        pub const fn trace_ioen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TRACE_IOEN"]
        #[inline(always)]
        pub fn set_trace_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TRACE_MODE"]
        #[inline(always)]
        pub const fn trace_mode(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "TRACE_MODE"]
        #[inline(always)]
        pub fn set_trace_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
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
                .field("trace_mode", &self.trace_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ dbg_sleep: {=bool:?}, dbg_stop: {=bool:?}, dbg_standby: {=bool:?}, trace_ioen: {=bool:?}, trace_mode: {=u8:?} }}" , self . dbg_sleep () , self . dbg_stop () , self . dbg_standby () , self . trace_ioen () , self . trace_mode ())
        }
    }
    #[doc = "IDCODE"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "DEV_ID"]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DEV_ID"]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "REV_ID"]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "REV_ID"]
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
