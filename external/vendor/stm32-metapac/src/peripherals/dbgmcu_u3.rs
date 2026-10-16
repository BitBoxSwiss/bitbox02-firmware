#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DBGMCU Address block."]
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
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::Idcode, crate::common::R> {
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
    pub const fn dbgmcu_sr(self) -> crate::common::Reg<regs::DbgmcuSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "DBGMCU debug host authentication register."]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_host(self) -> crate::common::Reg<regs::DbgmcuDbgAuthHost, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "DBGMCU debug device authentication register."]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_device(self) -> crate::common::Reg<regs::DbgmcuDbgAuthDevice, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 4."]
    #[inline(always)]
    pub const fn pidr4(self) -> crate::common::Reg<regs::Pidr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 0."]
    #[inline(always)]
    pub const fn pidr0(self) -> crate::common::Reg<regs::Pidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 1."]
    #[inline(always)]
    pub const fn pidr1(self) -> crate::common::Reg<regs::Pidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe4usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 2."]
    #[inline(always)]
    pub const fn pidr2(self) -> crate::common::Reg<regs::Pidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 3."]
    #[inline(always)]
    pub const fn pidr3(self) -> crate::common::Reg<regs::Pidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 0."]
    #[inline(always)]
    pub const fn cidr0(self) -> crate::common::Reg<regs::Cidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 1."]
    #[inline(always)]
    pub const fn cidr1(self) -> crate::common::Reg<regs::Cidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 2."]
    #[inline(always)]
    pub const fn cidr2(self) -> crate::common::Reg<regs::Cidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 3."]
    #[inline(always)]
    pub const fn cidr3(self) -> crate::common::Reg<regs::Cidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1fzr(pub u32);
    impl Ahb1fzr {
        #[doc = "None 0: normal operation. GPDMA channel 0 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 0 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma0_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 0 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 0 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 1 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 1 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma1_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 1 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 1 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 2 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 2 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma2_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 2 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 2 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 3 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 3 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma3_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 3 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 3 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 4 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 4 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma4_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 4 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 4 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 5 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 5 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma5_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 5 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 5 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 6 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 6 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma6_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 6 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 6 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 7 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 7 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma7_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 7 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 7 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 8 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 8 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma8_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 8 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 8 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 9 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 9 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma9_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 9 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 9 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 10 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 10 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma10_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 10 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 10 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "None 0: normal operation. GPDMA channel 11 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 11 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_gpdma11_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. GPDMA channel 11 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 11 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_gpdma11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("dbg_gpdma0_stop", &self.dbg_gpdma0_stop())
                .field("dbg_gpdma1_stop", &self.dbg_gpdma1_stop())
                .field("dbg_gpdma2_stop", &self.dbg_gpdma2_stop())
                .field("dbg_gpdma3_stop", &self.dbg_gpdma3_stop())
                .field("dbg_gpdma4_stop", &self.dbg_gpdma4_stop())
                .field("dbg_gpdma5_stop", &self.dbg_gpdma5_stop())
                .field("dbg_gpdma6_stop", &self.dbg_gpdma6_stop())
                .field("dbg_gpdma7_stop", &self.dbg_gpdma7_stop())
                .field("dbg_gpdma8_stop", &self.dbg_gpdma8_stop())
                .field("dbg_gpdma9_stop", &self.dbg_gpdma9_stop())
                .field("dbg_gpdma10_stop", &self.dbg_gpdma10_stop())
                .field("dbg_gpdma11_stop", &self.dbg_gpdma11_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1fzr {{ dbg_gpdma0_stop: {=bool:?}, dbg_gpdma1_stop: {=bool:?}, dbg_gpdma2_stop: {=bool:?}, dbg_gpdma3_stop: {=bool:?}, dbg_gpdma4_stop: {=bool:?}, dbg_gpdma5_stop: {=bool:?}, dbg_gpdma6_stop: {=bool:?}, dbg_gpdma7_stop: {=bool:?}, dbg_gpdma8_stop: {=bool:?}, dbg_gpdma9_stop: {=bool:?}, dbg_gpdma10_stop: {=bool:?}, dbg_gpdma11_stop: {=bool:?} }}" , self . dbg_gpdma0_stop () , self . dbg_gpdma1_stop () , self . dbg_gpdma2_stop () , self . dbg_gpdma3_stop () , self . dbg_gpdma4_stop () , self . dbg_gpdma5_stop () , self . dbg_gpdma6_stop () , self . dbg_gpdma7_stop () , self . dbg_gpdma8_stop () , self . dbg_gpdma9_stop () , self . dbg_gpdma10_stop () , self . dbg_gpdma11_stop ())
        }
    }
    #[doc = "DBGMCU APB1H peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hfzr(pub u32);
    impl Apb1hfzr {
        #[doc = "None 0: normal operation. LPTIM2 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM2 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_lptim2_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. LPTIM2 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM2 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_lptim2_stop(&mut self, val: bool) {
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
                .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hfzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb1hfzr {{ dbg_lptim2_stop: {=bool:?} }}", self.dbg_lptim2_stop())
        }
    }
    #[doc = "DBGMCU APB1L peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lfzr(pub u32);
    impl Apb1lfzr {
        #[doc = "None 0: normal operation. TIM2 continues to operate while CPU is in debug mode. 1: stop in debug. TIM2 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim2_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM2 continues to operate while CPU is in debug mode. 1: stop in debug. TIM2 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "None 0: normal operation. TIM3 continues to operate while CPU is in debug mode. 1: stop in debug. TIM3 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim3_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM3 continues to operate while CPU is in debug mode. 1: stop in debug. TIM3 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "None 0: normal operation. TIM4 continues to operate while CPU is in debug mode. 1: stop in debug. TIM4 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim4_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM4 continues to operate while CPU is in debug mode. 1: stop in debug. TIM4 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "None 0: normal operation. TIM6 continues to operate while CPU is in debug mode. 1: stop in debug. TIM6 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim6_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM6 continues to operate while CPU is in debug mode. 1: stop in debug. TIM6 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "None 0: normal operation. TIM7 continues to operate while CPU is in debug mode. 1: stop in debug. TIM7 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim7_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM7 continues to operate while CPU is in debug mode. 1: stop in debug. TIM7 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "None 0: normal operation. WWDG continues to operate while CPU is in debug mode. 1: stop in debug. WWDG is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_wwdg_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. WWDG continues to operate while CPU is in debug mode. 1: stop in debug. WWDG is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "None 0: normal operation. IWDG continues to operate while CPU is in debug mode. 1: stop in debug. IWDG is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. IWDG continues to operate while CPU is in debug mode. 1: stop in debug. IWDG is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "None 0: normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode. 1: stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_i2c1_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode. 1: stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_i2c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "None 0: normal operation. I2C2 SMBUS timeout continues to operate while CPU is in debug mode. 1: stop in debug. I2C2 SMBUS timeout is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_i2c2_stop(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. I2C2 SMBUS timeout continues to operate while CPU is in debug mode. 1: stop in debug. I2C2 SMBUS timeout is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_i2c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "None 0: normal operation. I3C1 timeout continues to operate while CPU is in debug mode. 1: stop in debug. I3C1 timeout is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_i3c1_stop(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. I3C1 timeout continues to operate while CPU is in debug mode. 1: stop in debug. I3C1 timeout is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_i3c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "None 0: normal operation. RTC continues to operate while CPU is in debug mode. 1: stop in debug. RTC is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_rtc_stop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. RTC continues to operate while CPU is in debug mode. 1: stop in debug. RTC is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_rtc_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("dbg_tim2_stop", &self.dbg_tim2_stop())
                .field("dbg_tim3_stop", &self.dbg_tim3_stop())
                .field("dbg_tim4_stop", &self.dbg_tim4_stop())
                .field("dbg_tim6_stop", &self.dbg_tim6_stop())
                .field("dbg_tim7_stop", &self.dbg_tim7_stop())
                .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
                .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
                .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
                .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
                .field("dbg_i3c1_stop", &self.dbg_i3c1_stop())
                .field("dbg_rtc_stop", &self.dbg_rtc_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lfzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1lfzr {{ dbg_tim2_stop: {=bool:?}, dbg_tim3_stop: {=bool:?}, dbg_tim4_stop: {=bool:?}, dbg_tim6_stop: {=bool:?}, dbg_tim7_stop: {=bool:?}, dbg_wwdg_stop: {=bool:?}, dbg_iwdg_stop: {=bool:?}, dbg_i2c1_stop: {=bool:?}, dbg_i2c2_stop: {=bool:?}, dbg_i3c1_stop: {=bool:?}, dbg_rtc_stop: {=bool:?} }}" , self . dbg_tim2_stop () , self . dbg_tim3_stop () , self . dbg_tim4_stop () , self . dbg_tim6_stop () , self . dbg_tim7_stop () , self . dbg_wwdg_stop () , self . dbg_iwdg_stop () , self . dbg_i2c1_stop () , self . dbg_i2c2_stop () , self . dbg_i3c1_stop () , self . dbg_rtc_stop ())
        }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr(pub u32);
    impl Apb2fzr {
        #[doc = "None 0: normal operation. TIM1 continues to operate while CPU is in debug mode. 1: stop in debug. TIM1 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim1_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM1 continues to operate while CPU is in debug mode. 1: stop in debug. TIM1 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "None 0: normal operation. TIM15 continues to operate while CPU is in debug mode. 1: stop in debug. TIM15 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim15_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM15 continues to operate while CPU is in debug mode. 1: stop in debug. TIM15 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "None 0: normal operation. TIM16 continues to operate while CPU is in debug mode. 1: stop in debug. TIM16 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim16_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM16 continues to operate while CPU is in debug mode. 1: stop in debug. TIM16 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim16_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "None 0: normal operation. TIM17 continues to operate while CPU is in debug mode. 1: stop in debug. TIM17 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_tim17_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. TIM17 continues to operate while CPU is in debug mode. 1: stop in debug. TIM17 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_tim17_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "None 0: normal operation. I3C2 timeout continues to operate while CPU is in debug mode. 1: stop in debug. I3C2 timeout is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_i3c2_stop(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. I3C2 timeout continues to operate while CPU is in debug mode. 1: stop in debug. I3C2 timeout is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_i3c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("dbg_tim17_stop", &self.dbg_tim17_stop())
                .field("dbg_i3c2_stop", &self.dbg_i3c2_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2fzr {{ dbg_tim1_stop: {=bool:?}, dbg_tim15_stop: {=bool:?}, dbg_tim16_stop: {=bool:?}, dbg_tim17_stop: {=bool:?}, dbg_i3c2_stop: {=bool:?} }}" , self . dbg_tim1_stop () , self . dbg_tim15_stop () , self . dbg_tim16_stop () , self . dbg_tim17_stop () , self . dbg_i3c2_stop ())
        }
    }
    #[doc = "DBGMCU APB3 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3fzr(pub u32);
    impl Apb3fzr {
        #[doc = "None 0: normal operation. I2C3 continues to operate while CPU is in debug mode. 1: stop in debug. I2C3 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_i2c3_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. I2C3 continues to operate while CPU is in debug mode. 1: stop in debug. I2C3 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_i2c3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "None 0: normal operation. LPTIM1 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM1 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_lptim1_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. LPTIM1 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM1 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_lptim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "None 0: normal operation. LPTIM3 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM3 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_lptim3_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. LPTIM3 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM3 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_lptim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "None 0: normal operation. LPTIM4 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM4 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub const fn dbg_lptim4_stop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: normal operation. LPTIM4 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM4 is frozen while CPU is in debug mode."]
        #[inline(always)]
        pub fn set_dbg_lptim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
                .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
                .field("dbg_lptim3_stop", &self.dbg_lptim3_stop())
                .field("dbg_lptim4_stop", &self.dbg_lptim4_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb3fzr {{ dbg_i2c3_stop: {=bool:?}, dbg_lptim1_stop: {=bool:?}, dbg_lptim3_stop: {=bool:?}, dbg_lptim4_stop: {=bool:?} }}" , self . dbg_i2c3_stop () , self . dbg_lptim1_stop () , self . dbg_lptim3_stop () , self . dbg_lptim4_stop ())
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr0(pub u32);
    impl Cidr0 {
        #[doc = "None 0x0D: common identification value."]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "None 0x0D: common identification value."]
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
        #[doc = "None 0x0: common identification value."]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0x0: common identification value."]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "None 0xF: Non-CoreSight component."]
        #[inline(always)]
        pub const fn class(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0xF: Non-CoreSight component."]
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
        #[doc = "None 0x05: common identification value."]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "None 0x05: common identification value."]
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
        #[doc = "None 0xB1: common identification value."]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "None 0xB1: common identification value."]
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
        #[doc = "All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state. 0: normal operation 1: automatic clock stop disabled."]
        #[inline(always)]
        pub const fn dbg_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state. 0: normal operation 1: automatic clock stop disabled."]
        #[inline(always)]
        pub fn set_dbg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed. 0: normal operation 1: automatic clock stop/power down disabled."]
        #[inline(always)]
        pub const fn dbg_standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed. 0: normal operation 1: automatic clock stop/power down disabled."]
        #[inline(always)]
        pub fn set_dbg_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "None 0: disabled-trace pins not assigned 1: enabled-trace pins assigned according to the value of TRACE_MODE field."]
        #[inline(always)]
        pub const fn trace_ioen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: disabled-trace pins not assigned 1: enabled-trace pins assigned according to the value of TRACE_MODE field."]
        #[inline(always)]
        pub fn set_trace_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "This bit enables the trace port clock, TRACECK. 0: disabled 1: enabled."]
        #[inline(always)]
        pub const fn trace_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "This bit enables the trace port clock, TRACECK. 0: disabled 1: enabled."]
        #[inline(always)]
        pub fn set_trace_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "None 0x0: trace pins assigned for asynchronous mode (TRACESWO) 0x1: trace pins assigned for synchronous mode with a port width of 1 (TRACECK, TRACED0) 0x2: trace pins assigned for synchronous mode with a port width of 2 ((TRACECK, TRACED0-1) 0x3: trace pins assigned for synchronous mode with a port width of 4 ((TRACECK, TRACED0-3)."]
        #[inline(always)]
        pub const fn trace_mode(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "None 0x0: trace pins assigned for asynchronous mode (TRACESWO) 0x1: trace pins assigned for synchronous mode with a port width of 1 (TRACECK, TRACED0) 0x2: trace pins assigned for synchronous mode with a port width of 2 ((TRACECK, TRACED0-1) 0x3: trace pins assigned for synchronous mode with a port width of 4 ((TRACECK, TRACED0-3)."]
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
                .field("dbg_stop", &self.dbg_stop())
                .field("dbg_standby", &self.dbg_standby())
                .field("trace_ioen", &self.trace_ioen())
                .field("trace_en", &self.trace_en())
                .field("trace_mode", &self.trace_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ dbg_stop: {=bool:?}, dbg_standby: {=bool:?}, trace_ioen: {=bool:?}, trace_en: {=bool:?}, trace_mode: {=u8:?} }}" , self . dbg_stop () , self . dbg_standby () , self . trace_ioen () , self . trace_en () , self . trace_mode ())
        }
    }
    #[doc = "DBGMCU debug device authentication register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgAuthDevice(pub u32);
    impl DbgmcuDbgAuthDevice {
        #[doc = "Device specific ID used for RDP regression."]
        #[inline(always)]
        pub const fn auth_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device specific ID used for RDP regression."]
        #[inline(always)]
        pub fn set_auth_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgmcuDbgAuthDevice {
        #[inline(always)]
        fn default() -> DbgmcuDbgAuthDevice {
            DbgmcuDbgAuthDevice(0)
        }
    }
    impl core::fmt::Debug for DbgmcuDbgAuthDevice {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuDbgAuthDevice")
                .field("auth_id", &self.auth_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuDbgAuthDevice {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuDbgAuthDevice {{ auth_id: {=u32:?} }}", self.auth_id())
        }
    }
    #[doc = "DBGMCU debug host authentication register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgAuthHost(pub u32);
    impl DbgmcuDbgAuthHost {
        #[doc = "The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory."]
        #[inline(always)]
        pub const fn auth_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory."]
        #[inline(always)]
        pub fn set_auth_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgmcuDbgAuthHost {
        #[inline(always)]
        fn default() -> DbgmcuDbgAuthHost {
            DbgmcuDbgAuthHost(0)
        }
    }
    impl core::fmt::Debug for DbgmcuDbgAuthHost {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuDbgAuthHost")
                .field("auth_key", &self.auth_key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuDbgAuthHost {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuDbgAuthHost {{ auth_key: {=u32:?} }}", self.auth_key())
        }
    }
    #[doc = "DBGMCU status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuSr(pub u32);
    impl DbgmcuSr {
        #[doc = "Bit n=0: APn absent Bit n=1: APn present."]
        #[inline(always)]
        pub const fn ap_present(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit n=0: APn absent Bit n=1: APn present."]
        #[inline(always)]
        pub fn set_ap_present(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Bit n=0: APn locked Bit n=1: APn enabled."]
        #[inline(always)]
        pub const fn ap_enabled(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit n=0: APn locked Bit n=1: APn enabled."]
        #[inline(always)]
        pub fn set_ap_enabled(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DbgmcuSr {
        #[inline(always)]
        fn default() -> DbgmcuSr {
            DbgmcuSr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuSr")
                .field("ap_present", &self.ap_present())
                .field("ap_enabled", &self.ap_enabled())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuSr {{ ap_present: {=u16:?}, ap_enabled: {=u16:?} }}",
                self.ap_present(),
                self.ap_enabled()
            )
        }
    }
    #[doc = "DBGMCU identity code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "None 0x454: STM32U375/385."]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "None 0x454: STM32U375/385."]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "None 0x0001: revision A."]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "None 0x0001: revision A."]
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
        #[doc = "None 0x00: DBGMCU part number."]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "None 0x00: DBGMCU part number."]
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
        #[doc = "None 0x0: DBGMCU part number."]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0x0: DBGMCU part number."]
        #[inline(always)]
        pub fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "None 0x0: STMicroelectronics JEDEC code."]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0x0: STMicroelectronics JEDEC code."]
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
        #[doc = "None 0x2: STMicroelectronics JEDEC code."]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "None 0x2: STMicroelectronics JEDEC code."]
        #[inline(always)]
        pub fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "None 0x1: designer identification specified by JEDEC."]
        #[inline(always)]
        pub const fn jedec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "None 0x1: designer identification specified by JEDEC."]
        #[inline(always)]
        pub fn set_jedec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "None 0x0: r0p0."]
        #[inline(always)]
        pub const fn revision(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0x0: r0p0."]
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
        #[doc = "None 0x0: no customer modifications."]
        #[inline(always)]
        pub const fn cmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0x0: no customer modifications."]
        #[inline(always)]
        pub fn set_cmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "None 0x0: no metal fix."]
        #[inline(always)]
        pub const fn revand(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0x0: no metal fix."]
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
        #[doc = "None 0x0: STMicroelectronics JEDEC code."]
        #[inline(always)]
        pub const fn jep106con(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0x0: STMicroelectronics JEDEC code."]
        #[inline(always)]
        pub fn set_jep106con(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "None 0x0: The register file occupies a single 4-Kbyte region."]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "None 0x0: The register file occupies a single 4-Kbyte region."]
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
}
