#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Microcontroller debug unit."]
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
    #[doc = "DBGMCU identity code register."]
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::Idcode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DBGMCU configuration register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DBGMCU APB1L peripheral freeze register."]
    #[inline(always)]
    pub const fn apb1lfzr(self) -> crate::common::Reg<regs::Apb1lfzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DBGMCU APB1H peripheral freeze register."]
    #[inline(always)]
    pub const fn apb1hfzr(self) -> crate::common::Reg<regs::Apb1hfzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[inline(always)]
    pub const fn apb2fzr(self) -> crate::common::Reg<regs::Apb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DBGMCU APB3 peripheral freeze register."]
    #[inline(always)]
    pub const fn apb3fzr(self) -> crate::common::Reg<regs::Apb3fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[inline(always)]
    pub const fn ahb1fzr(self) -> crate::common::Reg<regs::Ahb1fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DBGMCU status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox host register."]
    #[inline(always)]
    pub const fn auth_host(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox device register."]
    #[inline(always)]
    pub const fn auth_device(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox acknowledge register."]
    #[inline(always)]
    pub const fn auth_ack(self) -> crate::common::Reg<regs::AuthAck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 4."]
    #[inline(always)]
    pub const fn pidr4(self) -> crate::common::Reg<regs::Pidr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 0."]
    #[inline(always)]
    pub const fn pidr0(self) -> crate::common::Reg<regs::Pidr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 1."]
    #[inline(always)]
    pub const fn pidr1(self) -> crate::common::Reg<regs::Pidr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe4usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 2."]
    #[inline(always)]
    pub const fn pidr2(self) -> crate::common::Reg<regs::Pidr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 3."]
    #[inline(always)]
    pub const fn pidr3(self) -> crate::common::Reg<regs::Pidr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 0."]
    #[inline(always)]
    pub const fn cidr0(self) -> crate::common::Reg<regs::Cidr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 1."]
    #[inline(always)]
    pub const fn cidr1(self) -> crate::common::Reg<regs::Cidr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 2."]
    #[inline(always)]
    pub const fn cidr2(self) -> crate::common::Reg<regs::Cidr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 3."]
    #[inline(always)]
    pub const fn cidr3(self) -> crate::common::Reg<regs::Cidr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1fzr(pub u32);
    impl Ahb1fzr {
        #[doc = "GPDMA1 channel 0 stop in debug."]
        #[inline(always)]
        pub const fn gpdma1_stop(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 channel 0 stop in debug."]
        #[inline(always)]
        pub fn set_gpdma1_stop(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "GPDMA2 channel 0 stop in debug."]
        #[inline(always)]
        pub const fn gpdma2_stop(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPDMA2 channel 0 stop in debug."]
        #[inline(always)]
        pub fn set_gpdma2_stop(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ahb1fzr {
        #[inline(always)]
        fn default() -> Ahb1fzr {
            Ahb1fzr(0)
        }
    }
    impl core::fmt::Debug for Ahb1fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1fzr")
                .field("gpdma1_stop[0]", &self.gpdma1_stop(0usize))
                .field("gpdma1_stop[1]", &self.gpdma1_stop(1usize))
                .field("gpdma1_stop[2]", &self.gpdma1_stop(2usize))
                .field("gpdma1_stop[3]", &self.gpdma1_stop(3usize))
                .field("gpdma1_stop[4]", &self.gpdma1_stop(4usize))
                .field("gpdma1_stop[5]", &self.gpdma1_stop(5usize))
                .field("gpdma1_stop[6]", &self.gpdma1_stop(6usize))
                .field("gpdma1_stop[7]", &self.gpdma1_stop(7usize))
                .field("gpdma1_stop[8]", &self.gpdma1_stop(8usize))
                .field("gpdma1_stop[9]", &self.gpdma1_stop(9usize))
                .field("gpdma1_stop[10]", &self.gpdma1_stop(10usize))
                .field("gpdma1_stop[11]", &self.gpdma1_stop(11usize))
                .field("gpdma1_stop[12]", &self.gpdma1_stop(12usize))
                .field("gpdma1_stop[13]", &self.gpdma1_stop(13usize))
                .field("gpdma1_stop[14]", &self.gpdma1_stop(14usize))
                .field("gpdma1_stop[15]", &self.gpdma1_stop(15usize))
                .field("gpdma2_stop[0]", &self.gpdma2_stop(0usize))
                .field("gpdma2_stop[1]", &self.gpdma2_stop(1usize))
                .field("gpdma2_stop[2]", &self.gpdma2_stop(2usize))
                .field("gpdma2_stop[3]", &self.gpdma2_stop(3usize))
                .field("gpdma2_stop[4]", &self.gpdma2_stop(4usize))
                .field("gpdma2_stop[5]", &self.gpdma2_stop(5usize))
                .field("gpdma2_stop[6]", &self.gpdma2_stop(6usize))
                .field("gpdma2_stop[7]", &self.gpdma2_stop(7usize))
                .field("gpdma2_stop[8]", &self.gpdma2_stop(8usize))
                .field("gpdma2_stop[9]", &self.gpdma2_stop(9usize))
                .field("gpdma2_stop[10]", &self.gpdma2_stop(10usize))
                .field("gpdma2_stop[11]", &self.gpdma2_stop(11usize))
                .field("gpdma2_stop[12]", &self.gpdma2_stop(12usize))
                .field("gpdma2_stop[13]", &self.gpdma2_stop(13usize))
                .field("gpdma2_stop[14]", &self.gpdma2_stop(14usize))
                .field("gpdma2_stop[15]", &self.gpdma2_stop(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1fzr {{ gpdma1_stop[0]: {=bool:?}, gpdma1_stop[1]: {=bool:?}, gpdma1_stop[2]: {=bool:?}, gpdma1_stop[3]: {=bool:?}, gpdma1_stop[4]: {=bool:?}, gpdma1_stop[5]: {=bool:?}, gpdma1_stop[6]: {=bool:?}, gpdma1_stop[7]: {=bool:?}, gpdma1_stop[8]: {=bool:?}, gpdma1_stop[9]: {=bool:?}, gpdma1_stop[10]: {=bool:?}, gpdma1_stop[11]: {=bool:?}, gpdma1_stop[12]: {=bool:?}, gpdma1_stop[13]: {=bool:?}, gpdma1_stop[14]: {=bool:?}, gpdma1_stop[15]: {=bool:?}, gpdma2_stop[0]: {=bool:?}, gpdma2_stop[1]: {=bool:?}, gpdma2_stop[2]: {=bool:?}, gpdma2_stop[3]: {=bool:?}, gpdma2_stop[4]: {=bool:?}, gpdma2_stop[5]: {=bool:?}, gpdma2_stop[6]: {=bool:?}, gpdma2_stop[7]: {=bool:?}, gpdma2_stop[8]: {=bool:?}, gpdma2_stop[9]: {=bool:?}, gpdma2_stop[10]: {=bool:?}, gpdma2_stop[11]: {=bool:?}, gpdma2_stop[12]: {=bool:?}, gpdma2_stop[13]: {=bool:?}, gpdma2_stop[14]: {=bool:?}, gpdma2_stop[15]: {=bool:?} }}" , self . gpdma1_stop (0usize) , self . gpdma1_stop (1usize) , self . gpdma1_stop (2usize) , self . gpdma1_stop (3usize) , self . gpdma1_stop (4usize) , self . gpdma1_stop (5usize) , self . gpdma1_stop (6usize) , self . gpdma1_stop (7usize) , self . gpdma1_stop (8usize) , self . gpdma1_stop (9usize) , self . gpdma1_stop (10usize) , self . gpdma1_stop (11usize) , self . gpdma1_stop (12usize) , self . gpdma1_stop (13usize) , self . gpdma1_stop (14usize) , self . gpdma1_stop (15usize) , self . gpdma2_stop (0usize) , self . gpdma2_stop (1usize) , self . gpdma2_stop (2usize) , self . gpdma2_stop (3usize) , self . gpdma2_stop (4usize) , self . gpdma2_stop (5usize) , self . gpdma2_stop (6usize) , self . gpdma2_stop (7usize) , self . gpdma2_stop (8usize) , self . gpdma2_stop (9usize) , self . gpdma2_stop (10usize) , self . gpdma2_stop (11usize) , self . gpdma2_stop (12usize) , self . gpdma2_stop (13usize) , self . gpdma2_stop (14usize) , self . gpdma2_stop (15usize))
        }
    }
    #[doc = "DBGMCU APB1H peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hfzr(pub u32);
    impl Apb1hfzr {
        #[doc = "LPTIM2 stop in debug."]
        #[inline(always)]
        pub const fn lptim2_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 stop in debug."]
        #[inline(always)]
        pub fn set_lptim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Apb1hfzr {
        #[inline(always)]
        fn default() -> Apb1hfzr {
            Apb1hfzr(0)
        }
    }
    impl core::fmt::Debug for Apb1hfzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1hfzr")
                .field("lptim2_stop", &self.lptim2_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hfzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb1hfzr {{ lptim2_stop: {=bool:?} }}", self.lptim2_stop())
        }
    }
    #[doc = "DBGMCU APB1L peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lfzr(pub u32);
    impl Apb1lfzr {
        #[doc = "TIM2 stop in debug."]
        #[inline(always)]
        pub const fn tim2_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 stop in debug."]
        #[inline(always)]
        pub fn set_tim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 stop in debug."]
        #[inline(always)]
        pub const fn tim3_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 stop in debug."]
        #[inline(always)]
        pub fn set_tim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 stop in debug."]
        #[inline(always)]
        pub const fn tim4_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 stop in debug."]
        #[inline(always)]
        pub fn set_tim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 stop in debug."]
        #[inline(always)]
        pub const fn tim5_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 stop in debug."]
        #[inline(always)]
        pub fn set_tim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 stop in debug."]
        #[inline(always)]
        pub const fn tim6_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 stop in debug."]
        #[inline(always)]
        pub fn set_tim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 stop in debug."]
        #[inline(always)]
        pub const fn tim7_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 stop in debug."]
        #[inline(always)]
        pub fn set_tim7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 stop in debug."]
        #[inline(always)]
        pub const fn tim12_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 stop in debug."]
        #[inline(always)]
        pub fn set_tim12_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 stop in debug."]
        #[inline(always)]
        pub const fn tim13_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 stop in debug."]
        #[inline(always)]
        pub fn set_tim13_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 stop in debug."]
        #[inline(always)]
        pub const fn tim14_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 stop in debug."]
        #[inline(always)]
        pub fn set_tim14_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "WWDG stop in debug."]
        #[inline(always)]
        pub const fn wwdg_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG stop in debug."]
        #[inline(always)]
        pub fn set_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IWDG stop in debug."]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG stop in debug."]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "I2C1 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn i2c1_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_i2c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn i2c2_stop(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_i2c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 SCL stall counter stop in debug."]
        #[inline(always)]
        pub const fn i3c1_stop(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 SCL stall counter stop in debug."]
        #[inline(always)]
        pub fn set_i3c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1lfzr {
        #[inline(always)]
        fn default() -> Apb1lfzr {
            Apb1lfzr(0)
        }
    }
    impl core::fmt::Debug for Apb1lfzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lfzr")
                .field("tim2_stop", &self.tim2_stop())
                .field("tim3_stop", &self.tim3_stop())
                .field("tim4_stop", &self.tim4_stop())
                .field("tim5_stop", &self.tim5_stop())
                .field("tim6_stop", &self.tim6_stop())
                .field("tim7_stop", &self.tim7_stop())
                .field("tim12_stop", &self.tim12_stop())
                .field("tim13_stop", &self.tim13_stop())
                .field("tim14_stop", &self.tim14_stop())
                .field("wwdg_stop", &self.wwdg_stop())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("i2c1_stop", &self.i2c1_stop())
                .field("i2c2_stop", &self.i2c2_stop())
                .field("i3c1_stop", &self.i3c1_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lfzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1lfzr {{ tim2_stop: {=bool:?}, tim3_stop: {=bool:?}, tim4_stop: {=bool:?}, tim5_stop: {=bool:?}, tim6_stop: {=bool:?}, tim7_stop: {=bool:?}, tim12_stop: {=bool:?}, tim13_stop: {=bool:?}, tim14_stop: {=bool:?}, wwdg_stop: {=bool:?}, iwdg_stop: {=bool:?}, i2c1_stop: {=bool:?}, i2c2_stop: {=bool:?}, i3c1_stop: {=bool:?} }}" , self . tim2_stop () , self . tim3_stop () , self . tim4_stop () , self . tim5_stop () , self . tim6_stop () , self . tim7_stop () , self . tim12_stop () , self . tim13_stop () , self . tim14_stop () , self . wwdg_stop () , self . iwdg_stop () , self . i2c1_stop () , self . i2c2_stop () , self . i3c1_stop ())
        }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr(pub u32);
    impl Apb2fzr {
        #[doc = "TIM1 stop in debug."]
        #[inline(always)]
        pub const fn tim1_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 stop in debug."]
        #[inline(always)]
        pub fn set_tim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM8 stop in debug."]
        #[inline(always)]
        pub const fn tim8_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 stop in debug."]
        #[inline(always)]
        pub fn set_tim8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 stop in debug."]
        #[inline(always)]
        pub const fn tim15_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 stop in debug."]
        #[inline(always)]
        pub fn set_tim15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 stop in debug."]
        #[inline(always)]
        pub const fn tim16_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 stop in debug."]
        #[inline(always)]
        pub fn set_tim16_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 stop in debug."]
        #[inline(always)]
        pub const fn tim17_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 stop in debug."]
        #[inline(always)]
        pub fn set_tim17_stop(&mut self, val: bool) {
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
                .field("tim1_stop", &self.tim1_stop())
                .field("tim8_stop", &self.tim8_stop())
                .field("tim15_stop", &self.tim15_stop())
                .field("tim16_stop", &self.tim16_stop())
                .field("tim17_stop", &self.tim17_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2fzr {{ tim1_stop: {=bool:?}, tim8_stop: {=bool:?}, tim15_stop: {=bool:?}, tim16_stop: {=bool:?}, tim17_stop: {=bool:?} }}" , self . tim1_stop () , self . tim8_stop () , self . tim15_stop () , self . tim16_stop () , self . tim17_stop ())
        }
    }
    #[doc = "DBGMCU APB3 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3fzr(pub u32);
    impl Apb3fzr {
        #[doc = "I2C3 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn i2c3_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_i2c3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "I2C4 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn i2c4_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_i2c4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM1 stop in debug."]
        #[inline(always)]
        pub const fn lptim1_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 stop in debug."]
        #[inline(always)]
        pub fn set_lptim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "LPTIM3 stop in debug."]
        #[inline(always)]
        pub const fn lptim3_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 stop in debug."]
        #[inline(always)]
        pub fn set_lptim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LPTIM4 stop in debug."]
        #[inline(always)]
        pub const fn lptim4_stop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 stop in debug."]
        #[inline(always)]
        pub fn set_lptim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "LPTIM5 stop in debug."]
        #[inline(always)]
        pub const fn lptim5_stop(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 stop in debug."]
        #[inline(always)]
        pub fn set_lptim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "LPTIM6 stop in debug."]
        #[inline(always)]
        pub const fn lptim6_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM6 stop in debug."]
        #[inline(always)]
        pub fn set_lptim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "RTC stop in debug."]
        #[inline(always)]
        pub const fn rtc_stop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RTC stop in debug."]
        #[inline(always)]
        pub fn set_rtc_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb3fzr {
        #[inline(always)]
        fn default() -> Apb3fzr {
            Apb3fzr(0)
        }
    }
    impl core::fmt::Debug for Apb3fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3fzr")
                .field("i2c3_stop", &self.i2c3_stop())
                .field("i2c4_stop", &self.i2c4_stop())
                .field("lptim1_stop", &self.lptim1_stop())
                .field("lptim3_stop", &self.lptim3_stop())
                .field("lptim4_stop", &self.lptim4_stop())
                .field("lptim5_stop", &self.lptim5_stop())
                .field("lptim6_stop", &self.lptim6_stop())
                .field("rtc_stop", &self.rtc_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb3fzr {{ i2c3_stop: {=bool:?}, i2c4_stop: {=bool:?}, lptim1_stop: {=bool:?}, lptim3_stop: {=bool:?}, lptim4_stop: {=bool:?}, lptim5_stop: {=bool:?}, lptim6_stop: {=bool:?}, rtc_stop: {=bool:?} }}" , self . i2c3_stop () , self . i2c4_stop () , self . lptim1_stop () , self . lptim3_stop () , self . lptim4_stop () , self . lptim5_stop () , self . lptim6_stop () , self . rtc_stop ())
        }
    }
    #[doc = "DBGMCU debug authentication mailbox acknowledge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AuthAck(pub u32);
    impl AuthAck {
        #[doc = "Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message."]
        #[inline(always)]
        pub const fn host_ack(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message."]
        #[inline(always)]
        pub fn set_host_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message."]
        #[inline(always)]
        pub const fn dev_ack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message."]
        #[inline(always)]
        pub fn set_dev_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for AuthAck {
        #[inline(always)]
        fn default() -> AuthAck {
            AuthAck(0)
        }
    }
    impl core::fmt::Debug for AuthAck {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AuthAck")
                .field("host_ack", &self.host_ack())
                .field("dev_ack", &self.dev_ack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AuthAck {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AuthAck {{ host_ack: {=bool:?}, dev_ack: {=bool:?} }}",
                self.host_ack(),
                self.dev_ack()
            )
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr0(pub u32);
    impl Cidr0 {
        #[doc = "component identification bits \\[7:0\\]."]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "component identification bits \\[7:0\\]."]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr0 {
        #[inline(always)]
        fn default() -> Cidr0 {
            Cidr0(0)
        }
    }
    impl core::fmt::Debug for Cidr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cidr0").field("preamble", &self.preamble()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cidr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cidr0 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr1(pub u32);
    impl Cidr1 {
        #[doc = "component identification bits \\[11:8\\]."]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "component identification bits \\[11:8\\]."]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "component identification bits \\[15:12\\]
- component class."]
        #[inline(always)]
        pub const fn class(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "component identification bits \\[15:12\\]
- component class."]
        #[inline(always)]
        pub fn set_class(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cidr1 {
        #[inline(always)]
        fn default() -> Cidr1 {
            Cidr1(0)
        }
    }
    impl core::fmt::Debug for Cidr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cidr1")
                .field("preamble", &self.preamble())
                .field("class", &self.class())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cidr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cidr1 {{ preamble: {=u8:?}, class: {=u8:?} }}",
                self.preamble(),
                self.class()
            )
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr2(pub u32);
    impl Cidr2 {
        #[doc = "component identification bits \\[23:16\\]."]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "component identification bits \\[23:16\\]."]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr2 {
        #[inline(always)]
        fn default() -> Cidr2 {
            Cidr2(0)
        }
    }
    impl core::fmt::Debug for Cidr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cidr2").field("preamble", &self.preamble()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cidr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cidr2 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr3(pub u32);
    impl Cidr3 {
        #[doc = "component identification bits \\[31:24\\]."]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "component identification bits \\[31:24\\]."]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr3 {
        #[inline(always)]
        fn default() -> Cidr3 {
            Cidr3(0)
        }
    }
    impl core::fmt::Debug for Cidr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cidr3").field("preamble", &self.preamble()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cidr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cidr3 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
        #[inline(always)]
        pub fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
        #[inline(always)]
        pub const fn standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
        #[inline(always)]
        pub fn set_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "trace pin enable."]
        #[inline(always)]
        pub const fn trace_ioen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "trace pin enable."]
        #[inline(always)]
        pub fn set_trace_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "trace port and clock enable. This bit enables the trace port clock, TRACECK."]
        #[inline(always)]
        pub const fn trace_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "trace port and clock enable. This bit enables the trace port clock, TRACECK."]
        #[inline(always)]
        pub fn set_trace_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "trace pin assignment."]
        #[inline(always)]
        pub const fn trace_mode(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "trace pin assignment."]
        #[inline(always)]
        pub fn set_trace_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials."]
        #[inline(always)]
        pub const fn dcrt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials."]
        #[inline(always)]
        pub fn set_dcrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("stop", &self.stop())
                .field("standby", &self.standby())
                .field("trace_ioen", &self.trace_ioen())
                .field("trace_en", &self.trace_en())
                .field("trace_mode", &self.trace_mode())
                .field("dcrt", &self.dcrt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ stop: {=bool:?}, standby: {=bool:?}, trace_ioen: {=bool:?}, trace_en: {=bool:?}, trace_mode: {=u8:?}, dcrt: {=bool:?} }}" , self . stop () , self . standby () , self . trace_ioen () , self . trace_en () , self . trace_mode () , self . dcrt ())
        }
    }
    #[doc = "DBGMCU identity code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "device identification."]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "device identification."]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "revision."]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "revision."]
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
    #[doc = "DBGMCU CoreSight peripheral identity register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr0(pub u32);
    impl Pidr0 {
        #[doc = "part number bits \\[7:0\\]."]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "part number bits \\[7:0\\]."]
        #[inline(always)]
        pub fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Pidr0 {
        #[inline(always)]
        fn default() -> Pidr0 {
            Pidr0(0)
        }
    }
    impl core::fmt::Debug for Pidr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr0").field("partnum", &self.partnum()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pidr0 {{ partnum: {=u8:?} }}", self.partnum())
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr1(pub u32);
    impl Pidr1 {
        #[doc = "part number bits \\[11:8\\]."]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "part number bits \\[11:8\\]."]
        #[inline(always)]
        pub fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "JEP106 identity code bits \\[3:0\\]."]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "JEP106 identity code bits \\[3:0\\]."]
        #[inline(always)]
        pub fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr1 {
        #[inline(always)]
        fn default() -> Pidr1 {
            Pidr1(0)
        }
    }
    impl core::fmt::Debug for Pidr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr1")
                .field("partnum", &self.partnum())
                .field("jep106id", &self.jep106id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr1 {{ partnum: {=u8:?}, jep106id: {=u8:?} }}",
                self.partnum(),
                self.jep106id()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr2(pub u32);
    impl Pidr2 {
        #[doc = "JEP106 identity code bits \\[6:4\\]."]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "JEP106 identity code bits \\[6:4\\]."]
        #[inline(always)]
        pub fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "JEDEC assigned value."]
        #[inline(always)]
        pub const fn jedec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JEDEC assigned value."]
        #[inline(always)]
        pub fn set_jedec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "component revision number."]
        #[inline(always)]
        pub const fn revision(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "component revision number."]
        #[inline(always)]
        pub fn set_revision(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr2 {
        #[inline(always)]
        fn default() -> Pidr2 {
            Pidr2(0)
        }
    }
    impl core::fmt::Debug for Pidr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr2")
                .field("jep106id", &self.jep106id())
                .field("jedec", &self.jedec())
                .field("revision", &self.revision())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr2 {{ jep106id: {=u8:?}, jedec: {=bool:?}, revision: {=u8:?} }}",
                self.jep106id(),
                self.jedec(),
                self.revision()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr3(pub u32);
    impl Pidr3 {
        #[doc = "customer modified."]
        #[inline(always)]
        pub const fn cmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "customer modified."]
        #[inline(always)]
        pub fn set_cmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "metal fix version."]
        #[inline(always)]
        pub const fn revand(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "metal fix version."]
        #[inline(always)]
        pub fn set_revand(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr3 {
        #[inline(always)]
        fn default() -> Pidr3 {
            Pidr3(0)
        }
    }
    impl core::fmt::Debug for Pidr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr3")
                .field("cmod", &self.cmod())
                .field("revand", &self.revand())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr3 {{ cmod: {=u8:?}, revand: {=u8:?} }}",
                self.cmod(),
                self.revand()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr4(pub u32);
    impl Pidr4 {
        #[doc = "JEP106 continuation code."]
        #[inline(always)]
        pub const fn jep106con(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "JEP106 continuation code."]
        #[inline(always)]
        pub fn set_jep106con(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "register file size."]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "register file size."]
        #[inline(always)]
        pub fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr4 {
        #[inline(always)]
        fn default() -> Pidr4 {
            Pidr4(0)
        }
    }
    impl core::fmt::Debug for Pidr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr4")
                .field("jep106con", &self.jep106con())
                .field("size", &self.size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr4 {{ jep106con: {=u8:?}, size: {=u8:?} }}",
                self.jep106con(),
                self.size()
            )
        }
    }
    #[doc = "DBGMCU status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present."]
        #[inline(always)]
        pub const fn ap_present(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present."]
        #[inline(always)]
        pub fn set_ap_present(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled."]
        #[inline(always)]
        pub const fn ap_enabled(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled."]
        #[inline(always)]
        pub fn set_ap_enabled(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("ap_present", &self.ap_present())
                .field("ap_enabled", &self.ap_enabled())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ ap_present: {=u16:?}, ap_enabled: {=u16:?} }}",
                self.ap_present(),
                self.ap_enabled()
            )
        }
    }
}
