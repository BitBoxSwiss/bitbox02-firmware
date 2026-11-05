#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "debug support"]
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
    #[doc = "DBGMCU_IDCODE"]
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::Idcode, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Debug MCU configuration register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Debug MCU APB1 freeze register1"]
    #[inline(always)]
    pub const fn apb1_fz(self) -> crate::common::Reg<regs::Apb1Fz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Debug MCU APB1 freeze register 2"]
    #[inline(always)]
    pub const fn apb2_fz(self) -> crate::common::Reg<regs::Apb2Fz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Debug MCU APB1 freeze register1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1Fz(pub u32);
    impl Apb1Fz {
        #[doc = "TIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim2_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim3_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim4_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim5_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim6_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim7_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Debug RTC stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_rtc_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Debug RTC stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_rtc_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Debug window watchdog stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_wwdg_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Debug window watchdog stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Debug independent watchdog stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Debug independent watchdog stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SMBUS timeout mode stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_i2c1_smbus_timeout(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SMBUS timeout mode stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_i2c1_smbus_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SMBUS timeout mode stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_i2c2_smbus_timeout(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SMBUS timeout mode stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_i2c2_smbus_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Apb1Fz {
        #[inline(always)]
        fn default() -> Apb1Fz {
            Apb1Fz(0)
        }
    }
    impl core::fmt::Debug for Apb1Fz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1Fz")
                .field("dbg_tim2_stop", &self.dbg_tim2_stop())
                .field("dbg_tim3_stop", &self.dbg_tim3_stop())
                .field("dbg_tim4_stop", &self.dbg_tim4_stop())
                .field("dbg_tim5_stop", &self.dbg_tim5_stop())
                .field("dbg_tim6_stop", &self.dbg_tim6_stop())
                .field("dbg_tim7_stop", &self.dbg_tim7_stop())
                .field("dbg_rtc_stop", &self.dbg_rtc_stop())
                .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
                .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
                .field("dbg_i2c1_smbus_timeout", &self.dbg_i2c1_smbus_timeout())
                .field("dbg_i2c2_smbus_timeout", &self.dbg_i2c2_smbus_timeout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1Fz {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1Fz {{ dbg_tim2_stop: {=bool:?}, dbg_tim3_stop: {=bool:?}, dbg_tim4_stop: {=bool:?}, dbg_tim5_stop: {=bool:?}, dbg_tim6_stop: {=bool:?}, dbg_tim7_stop: {=bool:?}, dbg_rtc_stop: {=bool:?}, dbg_wwdg_stop: {=bool:?}, dbg_iwdg_stop: {=bool:?}, dbg_i2c1_smbus_timeout: {=bool:?}, dbg_i2c2_smbus_timeout: {=bool:?} }}" , self . dbg_tim2_stop () , self . dbg_tim3_stop () , self . dbg_tim4_stop () , self . dbg_tim5_stop () , self . dbg_tim6_stop () , self . dbg_tim7_stop () , self . dbg_rtc_stop () , self . dbg_wwdg_stop () , self . dbg_iwdg_stop () , self . dbg_i2c1_smbus_timeout () , self . dbg_i2c2_smbus_timeout ())
        }
    }
    #[doc = "Debug MCU APB1 freeze register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2Fz(pub u32);
    impl Apb2Fz {
        #[doc = "TIM counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim9_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim10_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim11_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb2Fz {
        #[inline(always)]
        fn default() -> Apb2Fz {
            Apb2Fz(0)
        }
    }
    impl core::fmt::Debug for Apb2Fz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2Fz")
                .field("dbg_tim9_stop", &self.dbg_tim9_stop())
                .field("dbg_tim10_stop", &self.dbg_tim10_stop())
                .field("dbg_tim11_stop", &self.dbg_tim11_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2Fz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2Fz {{ dbg_tim9_stop: {=bool:?}, dbg_tim10_stop: {=bool:?}, dbg_tim11_stop: {=bool:?} }}",
                self.dbg_tim9_stop(),
                self.dbg_tim10_stop(),
                self.dbg_tim11_stop()
            )
        }
    }
    #[doc = "Debug MCU configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Debug Sleep mode"]
        #[inline(always)]
        pub const fn dbg_sleep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Sleep mode"]
        #[inline(always)]
        pub fn set_dbg_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Debug Stop mode"]
        #[inline(always)]
        pub const fn dbg_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Stop mode"]
        #[inline(always)]
        pub fn set_dbg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Debug Standby mode"]
        #[inline(always)]
        pub const fn dbg_standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Standby mode"]
        #[inline(always)]
        pub fn set_dbg_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Trace pin assignment control"]
        #[inline(always)]
        pub const fn trace_ioen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trace pin assignment control"]
        #[inline(always)]
        pub fn set_trace_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trace pin assignment control"]
        #[inline(always)]
        pub const fn trace_mode(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Trace pin assignment control"]
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
    #[doc = "DBGMCU_IDCODE"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "Device identifier"]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Device identifier"]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Revision identifie"]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision identifie"]
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
