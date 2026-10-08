#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DBGMCU register block."]
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
    #[doc = "DBGMCU device ID code register."]
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::Idcode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DBGMCU configuration register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DBGMCU APB1 freeze register."]
    #[inline(always)]
    pub const fn apb1fzr(self) -> crate::common::Reg<regs::Apb1fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DBG APB2 freeze register."]
    #[inline(always)]
    pub const fn apb2fzr(self) -> crate::common::Reg<regs::Apb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DBGMCU status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox host register."]
    #[inline(always)]
    pub const fn dbg_auth_host(self) -> crate::common::Reg<regs::DbgAuthHost, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox device register."]
    #[inline(always)]
    pub const fn dbg_auth_device(self) -> crate::common::Reg<regs::DbgAuthDevice, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
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
    #[doc = "DBGMCU APB1 freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1fzr(pub u32);
    impl Apb1fzr {
        #[doc = "TIM2 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim2_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim3_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM6 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim6_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim7_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RTC stop in debug."]
        #[inline(always)]
        pub const fn dbg_rtc_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC stop in debug."]
        #[inline(always)]
        pub fn set_dbg_rtc_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "WWDG stop in debug."]
        #[inline(always)]
        pub const fn dbg_wwdg_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG stop in debug."]
        #[inline(always)]
        pub fn set_dbg_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IWDG stop in debug."]
        #[inline(always)]
        pub const fn dbg_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG stop in debug."]
        #[inline(always)]
        pub fn set_dbg_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM2 stop in debug."]
        #[inline(always)]
        pub const fn dbg_lptim2_stop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_lptim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "LPTIM1 stop in debug."]
        #[inline(always)]
        pub const fn dbg_lptim1_stop(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_lptim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("dbg_tim2_stop", &self.dbg_tim2_stop())
                .field("dbg_tim3_stop", &self.dbg_tim3_stop())
                .field("dbg_tim6_stop", &self.dbg_tim6_stop())
                .field("dbg_tim7_stop", &self.dbg_tim7_stop())
                .field("dbg_rtc_stop", &self.dbg_rtc_stop())
                .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
                .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
                .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
                .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1fzr {{ dbg_tim2_stop: {=bool:?}, dbg_tim3_stop: {=bool:?}, dbg_tim6_stop: {=bool:?}, dbg_tim7_stop: {=bool:?}, dbg_rtc_stop: {=bool:?}, dbg_wwdg_stop: {=bool:?}, dbg_iwdg_stop: {=bool:?}, dbg_lptim2_stop: {=bool:?}, dbg_lptim1_stop: {=bool:?} }}" , self . dbg_tim2_stop () , self . dbg_tim3_stop () , self . dbg_tim6_stop () , self . dbg_tim7_stop () , self . dbg_rtc_stop () , self . dbg_wwdg_stop () , self . dbg_iwdg_stop () , self . dbg_lptim2_stop () , self . dbg_lptim1_stop ())
        }
    }
    #[doc = "DBG APB2 freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr(pub u32);
    impl Apb2fzr {
        #[doc = "TIM1 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim1_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM15 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim15_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim16_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim16_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "LPTIM3 stop in debug."]
        #[inline(always)]
        pub const fn dbg_lptim3_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_lptim3_stop(&mut self, val: bool) {
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
                .field("dbg_tim1_stop", &self.dbg_tim1_stop())
                .field("dbg_tim15_stop", &self.dbg_tim15_stop())
                .field("dbg_tim16_stop", &self.dbg_tim16_stop())
                .field("dbg_lptim3_stop", &self.dbg_lptim3_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2fzr {{ dbg_tim1_stop: {=bool:?}, dbg_tim15_stop: {=bool:?}, dbg_tim16_stop: {=bool:?}, dbg_lptim3_stop: {=bool:?} }}" , self . dbg_tim1_stop () , self . dbg_tim15_stop () , self . dbg_tim16_stop () , self . dbg_lptim3_stop ())
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
        #[doc = "Debug Stop mode Debug options in Stop mode."]
        #[inline(always)]
        pub const fn dbg_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Stop mode Debug options in Stop mode."]
        #[inline(always)]
        pub fn set_dbg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
        #[inline(always)]
        pub const fn dbg_standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
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
    #[doc = "DBGMCU debug authentication mailbox device register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgAuthDevice(pub u32);
    impl DbgAuthDevice {
        #[doc = "Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register."]
        #[inline(always)]
        pub const fn message(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register."]
        #[inline(always)]
        pub fn set_message(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgAuthDevice {
        #[inline(always)]
        fn default() -> DbgAuthDevice {
            DbgAuthDevice(0)
        }
    }
    impl core::fmt::Debug for DbgAuthDevice {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgAuthDevice")
                .field("message", &self.message())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgAuthDevice {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgAuthDevice {{ message: {=u32:?} }}", self.message())
        }
    }
    #[doc = "DBGMCU debug authentication mailbox host register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgAuthHost(pub u32);
    impl DbgAuthHost {
        #[doc = "Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
        #[inline(always)]
        pub const fn message(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
        #[inline(always)]
        pub fn set_message(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgAuthHost {
        #[inline(always)]
        fn default() -> DbgAuthHost {
            DbgAuthHost(0)
        }
    }
    impl core::fmt::Debug for DbgAuthHost {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgAuthHost").field("message", &self.message()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgAuthHost {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgAuthHost {{ message: {=u32:?} }}", self.message())
        }
    }
    #[doc = "DBGMCU device ID code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "Device identifier This field indicates the device ID."]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Device identifier This field indicates the device ID."]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Revision identifier This field indicates the revision of the device."]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision identifier This field indicates the revision of the device."]
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
        #[doc = "Identifies whether access port AP1 is present in device."]
        #[inline(always)]
        pub const fn ap1_present(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Identifies whether access port AP1 is present in device."]
        #[inline(always)]
        pub fn set_ap1_present(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Identifies whether access port AP0 is present in device."]
        #[inline(always)]
        pub const fn ap0_present(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Identifies whether access port AP0 is present in device."]
        #[inline(always)]
        pub fn set_ap0_present(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)."]
        #[inline(always)]
        pub const fn ap1_enabled(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)."]
        #[inline(always)]
        pub fn set_ap1_enabled(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)."]
        #[inline(always)]
        pub const fn ap0_enabled(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)."]
        #[inline(always)]
        pub fn set_ap0_enabled(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("ap1_present", &self.ap1_present())
                .field("ap0_present", &self.ap0_present())
                .field("ap1_enabled", &self.ap1_enabled())
                .field("ap0_enabled", &self.ap0_enabled())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ ap1_present: {=bool:?}, ap0_present: {=bool:?}, ap1_enabled: {=bool:?}, ap0_enabled: {=bool:?} }}" , self . ap1_present () , self . ap0_present () , self . ap1_enabled () , self . ap0_enabled ())
        }
    }
}
