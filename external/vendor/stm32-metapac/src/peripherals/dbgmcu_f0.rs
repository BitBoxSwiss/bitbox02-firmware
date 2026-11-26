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
    #[doc = "Debug MCU APB1 freeze register"]
    #[inline(always)]
    pub const fn apb1_fz(self) -> crate::common::Reg<regs::Apb1Fz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Debug MCU APB2 freeze register"]
    #[inline(always)]
    pub const fn apb2_fz(self) -> crate::common::Reg<regs::Apb2Fz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Debug MCU APB1 freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1Fz(pub u32);
    impl Apb1Fz {
        #[doc = "TIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM6 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim6(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim7(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM14 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim14(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Debug RTC stopped when core is halted"]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Debug RTC stopped when core is halted"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Debug window watchdog stopped when core is halted"]
        #[inline(always)]
        pub const fn wwdg(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Debug window watchdog stopped when core is halted"]
        #[inline(always)]
        pub fn set_wwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Debug independent watchdog stopped when core is halted"]
        #[inline(always)]
        pub const fn iwdg(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Debug independent watchdog stopped when core is halted"]
        #[inline(always)]
        pub fn set_iwdg(&mut self, val: bool) {
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
        #[doc = "CAN stopped when core is halted"]
        #[inline(always)]
        pub const fn can(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN stopped when core is halted"]
        #[inline(always)]
        pub fn set_can(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
                .field("tim2", &self.tim2())
                .field("tim3", &self.tim3())
                .field("tim6", &self.tim6())
                .field("tim7", &self.tim7())
                .field("tim14", &self.tim14())
                .field("rtc", &self.rtc())
                .field("wwdg", &self.wwdg())
                .field("iwdg", &self.iwdg())
                .field("dbg_i2c1_smbus_timeout", &self.dbg_i2c1_smbus_timeout())
                .field("can", &self.can())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1Fz {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1Fz {{ tim2: {=bool:?}, tim3: {=bool:?}, tim6: {=bool:?}, tim7: {=bool:?}, tim14: {=bool:?}, rtc: {=bool:?}, wwdg: {=bool:?}, iwdg: {=bool:?}, dbg_i2c1_smbus_timeout: {=bool:?}, can: {=bool:?} }}" , self . tim2 () , self . tim3 () , self . tim6 () , self . tim7 () , self . tim14 () , self . rtc () , self . wwdg () , self . iwdg () , self . dbg_i2c1_smbus_timeout () , self . can ())
        }
    }
    #[doc = "Debug MCU APB2 freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2Fz(pub u32);
    impl Apb2Fz {
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
        #[doc = "TIM15 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn tim15(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_tim15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
    impl Default for Apb2Fz {
        #[inline(always)]
        fn default() -> Apb2Fz {
            Apb2Fz(0)
        }
    }
    impl core::fmt::Debug for Apb2Fz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2Fz")
                .field("tim1", &self.tim1())
                .field("tim15", &self.tim15())
                .field("tim16", &self.tim16())
                .field("tim17", &self.tim17())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2Fz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2Fz {{ tim1: {=bool:?}, tim15: {=bool:?}, tim16: {=bool:?}, tim17: {=bool:?} }}",
                self.tim1(),
                self.tim15(),
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
                .field("dbg_stop", &self.dbg_stop())
                .field("dbg_standby", &self.dbg_standby())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ dbg_stop: {=bool:?}, dbg_standby: {=bool:?} }}",
                self.dbg_stop(),
                self.dbg_standby()
            )
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
        #[doc = "Division Identifier"]
        #[inline(always)]
        pub const fn div_id(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Division Identifier"]
        #[inline(always)]
        pub fn set_div_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
                .field("div_id", &self.div_id())
                .field("rev_id", &self.rev_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idcode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idcode {{ dev_id: {=u16:?}, div_id: {=u8:?}, rev_id: {=u16:?} }}",
                self.dev_id(),
                self.div_id(),
                self.rev_id()
            )
        }
    }
}
