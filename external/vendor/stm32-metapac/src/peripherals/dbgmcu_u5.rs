#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MCU debug component"]
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
    #[doc = "Debug MCU APB1L peripheral freeze register"]
    #[inline(always)]
    pub const fn apb1lfzr(self) -> crate::common::Reg<regs::Apb1lfzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Debug MCU APB1H peripheral freeze register"]
    #[inline(always)]
    pub const fn apb1hfzr(self) -> crate::common::Reg<regs::Apb1hfzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Debug MCU APB2 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb2fzr(self) -> crate::common::Reg<regs::Apb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Debug MCU APB3 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb3fzr(self) -> crate::common::Reg<regs::Apb3fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Debug MCU AHB1 peripheral freeze register"]
    #[inline(always)]
    pub const fn ahb1fzr(self) -> crate::common::Reg<regs::Ahb1fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Debug MCU AHB3 peripheral freeze register"]
    #[inline(always)]
    pub const fn ahb3fzr(self) -> crate::common::Reg<regs::Ahb3fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "DBGMCU status register"]
    #[inline(always)]
    pub const fn dbgmcu_sr(self) -> crate::common::Reg<regs::DbgmcuSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "DBGMCU debug host authentication register"]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_host(self) -> crate::common::Reg<regs::DbgmcuDbgAuthHost, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "DBGMCU debug device authentication register"]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_device(self) -> crate::common::Reg<regs::DbgmcuDbgAuthDevice, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Debug MCU CoreSight peripheral identity register 4"]
    #[inline(always)]
    pub const fn pidr4(self) -> crate::common::Reg<regs::Pidr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
    #[doc = "Debug MCU CoreSight peripheral identity register 0"]
    #[inline(always)]
    pub const fn pidr0(self) -> crate::common::Reg<regs::Pidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe0usize) as _) }
    }
    #[doc = "Debug MCU CoreSight peripheral identity register 1"]
    #[inline(always)]
    pub const fn pidr1(self) -> crate::common::Reg<regs::Pidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe4usize) as _) }
    }
    #[doc = "Debug MCU CoreSight peripheral identity register 2"]
    #[inline(always)]
    pub const fn pidr2(self) -> crate::common::Reg<regs::Pidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[doc = "Debug MCU CoreSight peripheral identity register 3"]
    #[inline(always)]
    pub const fn pidr3(self) -> crate::common::Reg<regs::Pidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[doc = "Debug MCU CoreSight component identity register 0"]
    #[inline(always)]
    pub const fn cidr0(self) -> crate::common::Reg<regs::Cidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
    }
    #[doc = "Debug MCU CoreSight component identity register 1"]
    #[inline(always)]
    pub const fn cidr1(self) -> crate::common::Reg<regs::Cidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[doc = "Debug MCU CoreSight component identity register 2"]
    #[inline(always)]
    pub const fn cidr2(self) -> crate::common::Reg<regs::Cidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[doc = "Debug MCU CoreSight component identity register 3"]
    #[inline(always)]
    pub const fn cidr3(self) -> crate::common::Reg<regs::Cidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Debug MCU AHB1 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1fzr(pub u32);
    impl Ahb1fzr {
        #[doc = "GPDMA channel 0 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma0_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 0 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA channel 1 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma1_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 1 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPDMA channel 2 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma2_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 2 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPDMA channel 3 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma3_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 3 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPDMA channel 4 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma4_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 4 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPDMA channel 5 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma5_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 5 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPDMA channel 6 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma6_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 6 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPDMA channel 7 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma7_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 7 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPDMA channel 8 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma8_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 8 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPDMA channel 9 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma9_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 9 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPDMA channel 10 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma10_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 10 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GPDMA channel 11 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma11_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 11 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "GPDMA channel 12 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma12_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 12 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma12_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPDMA channel 13 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma13_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 13 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma13_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPDMA channel 14 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma14_stop(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 14 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma14_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPDMA channel 15 stop in debug"]
        #[inline(always)]
        pub const fn dbg_gpdma15_stop(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA channel 15 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_gpdma15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("dbg_gpdma12_stop", &self.dbg_gpdma12_stop())
                .field("dbg_gpdma13_stop", &self.dbg_gpdma13_stop())
                .field("dbg_gpdma14_stop", &self.dbg_gpdma14_stop())
                .field("dbg_gpdma15_stop", &self.dbg_gpdma15_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1fzr {{ dbg_gpdma0_stop: {=bool:?}, dbg_gpdma1_stop: {=bool:?}, dbg_gpdma2_stop: {=bool:?}, dbg_gpdma3_stop: {=bool:?}, dbg_gpdma4_stop: {=bool:?}, dbg_gpdma5_stop: {=bool:?}, dbg_gpdma6_stop: {=bool:?}, dbg_gpdma7_stop: {=bool:?}, dbg_gpdma8_stop: {=bool:?}, dbg_gpdma9_stop: {=bool:?}, dbg_gpdma10_stop: {=bool:?}, dbg_gpdma11_stop: {=bool:?}, dbg_gpdma12_stop: {=bool:?}, dbg_gpdma13_stop: {=bool:?}, dbg_gpdma14_stop: {=bool:?}, dbg_gpdma15_stop: {=bool:?} }}" , self . dbg_gpdma0_stop () , self . dbg_gpdma1_stop () , self . dbg_gpdma2_stop () , self . dbg_gpdma3_stop () , self . dbg_gpdma4_stop () , self . dbg_gpdma5_stop () , self . dbg_gpdma6_stop () , self . dbg_gpdma7_stop () , self . dbg_gpdma8_stop () , self . dbg_gpdma9_stop () , self . dbg_gpdma10_stop () , self . dbg_gpdma11_stop () , self . dbg_gpdma12_stop () , self . dbg_gpdma13_stop () , self . dbg_gpdma14_stop () , self . dbg_gpdma15_stop ())
        }
    }
    #[doc = "Debug MCU AHB3 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3fzr(pub u32);
    impl Ahb3fzr {
        #[doc = "LPDMA channel 0 stop in debug"]
        #[inline(always)]
        pub const fn dbg_lpdma0_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA channel 0 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_lpdma0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPDMA channel 1 stop in debug"]
        #[inline(always)]
        pub const fn dbg_lpdma1_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA channel 1 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_lpdma1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPDMA channel 2 stop in debug"]
        #[inline(always)]
        pub const fn dbg_lpdma2_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA channel 2 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_lpdma2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LPDMA channel 3 stop in debug"]
        #[inline(always)]
        pub const fn dbg_lpdma3_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA channel 3 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_lpdma3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ahb3fzr {
        #[inline(always)]
        fn default() -> Ahb3fzr {
            Ahb3fzr(0)
        }
    }
    impl core::fmt::Debug for Ahb3fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3fzr")
                .field("dbg_lpdma0_stop", &self.dbg_lpdma0_stop())
                .field("dbg_lpdma1_stop", &self.dbg_lpdma1_stop())
                .field("dbg_lpdma2_stop", &self.dbg_lpdma2_stop())
                .field("dbg_lpdma3_stop", &self.dbg_lpdma3_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb3fzr {{ dbg_lpdma0_stop: {=bool:?}, dbg_lpdma1_stop: {=bool:?}, dbg_lpdma2_stop: {=bool:?}, dbg_lpdma3_stop: {=bool:?} }}" , self . dbg_lpdma0_stop () , self . dbg_lpdma1_stop () , self . dbg_lpdma2_stop () , self . dbg_lpdma3_stop ())
        }
    }
    #[doc = "Debug MCU APB1H peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hfzr(pub u32);
    impl Apb1hfzr {
        #[doc = "I2C4 stop in debug"]
        #[inline(always)]
        pub const fn dbg_i2c4_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_i2c4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 stop in debug"]
        #[inline(always)]
        pub const fn dbg_lptim2_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 stop in debug"]
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
                .field("dbg_i2c4_stop", &self.dbg_i2c4_stop())
                .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hfzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1hfzr {{ dbg_i2c4_stop: {=bool:?}, dbg_lptim2_stop: {=bool:?} }}",
                self.dbg_i2c4_stop(),
                self.dbg_lptim2_stop()
            )
        }
    }
    #[doc = "Debug MCU APB1L peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lfzr(pub u32);
    impl Apb1lfzr {
        #[doc = "TIM2 stop in debug"]
        #[inline(always)]
        pub const fn dbg_tim2_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_tim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 stop in debug"]
        #[inline(always)]
        pub const fn dbg_tim3_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_tim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 stop in debug"]
        #[inline(always)]
        pub const fn dbg_tim4_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_tim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 stop in debug"]
        #[inline(always)]
        pub const fn dbg_tim5_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_tim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 stop in debug"]
        #[inline(always)]
        pub const fn dbg_tim6_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_tim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 stop in debug"]
        #[inline(always)]
        pub const fn dbg_tim7_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_tim7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Window watchdog counter stop in debug"]
        #[inline(always)]
        pub const fn dbg_wwdg_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog counter stop in debug"]
        #[inline(always)]
        pub fn set_dbg_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Independent watchdog counter stop in debug"]
        #[inline(always)]
        pub const fn dbg_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter stop in debug"]
        #[inline(always)]
        pub fn set_dbg_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "I2C1 SMBUS timeout stop in debug"]
        #[inline(always)]
        pub const fn dbg_i2c1_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 SMBUS timeout stop in debug"]
        #[inline(always)]
        pub fn set_dbg_i2c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 SMBUS timeout stop in debug"]
        #[inline(always)]
        pub const fn dbg_i2c2_stop(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 SMBUS timeout stop in debug"]
        #[inline(always)]
        pub fn set_dbg_i2c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("dbg_tim5_stop", &self.dbg_tim5_stop())
                .field("dbg_tim6_stop", &self.dbg_tim6_stop())
                .field("dbg_tim7_stop", &self.dbg_tim7_stop())
                .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
                .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
                .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
                .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lfzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1lfzr {{ dbg_tim2_stop: {=bool:?}, dbg_tim3_stop: {=bool:?}, dbg_tim4_stop: {=bool:?}, dbg_tim5_stop: {=bool:?}, dbg_tim6_stop: {=bool:?}, dbg_tim7_stop: {=bool:?}, dbg_wwdg_stop: {=bool:?}, dbg_iwdg_stop: {=bool:?}, dbg_i2c1_stop: {=bool:?}, dbg_i2c2_stop: {=bool:?} }}" , self . dbg_tim2_stop () , self . dbg_tim3_stop () , self . dbg_tim4_stop () , self . dbg_tim5_stop () , self . dbg_tim6_stop () , self . dbg_tim7_stop () , self . dbg_wwdg_stop () , self . dbg_iwdg_stop () , self . dbg_i2c1_stop () , self . dbg_i2c2_stop ())
        }
    }
    #[doc = "Debug MCU APB2 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr(pub u32);
    impl Apb2fzr {
        #[doc = "TIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim1_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM8 stop in debug"]
        #[inline(always)]
        pub const fn dbg_tim8_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_tim8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim15_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 counter stopped when core is halted"]
        #[inline(always)]
        pub const fn dbg_tim16_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 counter stopped when core is halted"]
        #[inline(always)]
        pub fn set_dbg_tim16_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DBG_TIM17_STOP"]
        #[inline(always)]
        pub const fn dbg_tim17_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DBG_TIM17_STOP"]
        #[inline(always)]
        pub fn set_dbg_tim17_stop(&mut self, val: bool) {
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
                .field("dbg_tim8_stop", &self.dbg_tim8_stop())
                .field("dbg_tim15_stop", &self.dbg_tim15_stop())
                .field("dbg_tim16_stop", &self.dbg_tim16_stop())
                .field("dbg_tim17_stop", &self.dbg_tim17_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2fzr {{ dbg_tim1_stop: {=bool:?}, dbg_tim8_stop: {=bool:?}, dbg_tim15_stop: {=bool:?}, dbg_tim16_stop: {=bool:?}, dbg_tim17_stop: {=bool:?} }}" , self . dbg_tim1_stop () , self . dbg_tim8_stop () , self . dbg_tim15_stop () , self . dbg_tim16_stop () , self . dbg_tim17_stop ())
        }
    }
    #[doc = "Debug MCU APB3 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3fzr(pub u32);
    impl Apb3fzr {
        #[doc = "I2C3 stop in debug"]
        #[inline(always)]
        pub const fn dbg_i2c3_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_i2c3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM1 stop in debug"]
        #[inline(always)]
        pub const fn dbg_lptim1_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_lptim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "LPTIM3 stop in debug"]
        #[inline(always)]
        pub const fn dbg_lptim3_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_lptim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LPTIM4 stop in debug"]
        #[inline(always)]
        pub const fn dbg_lptim4_stop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 stop in debug"]
        #[inline(always)]
        pub fn set_dbg_lptim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "RTC stop in debug"]
        #[inline(always)]
        pub const fn dbg_rtc_stop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RTC stop in debug"]
        #[inline(always)]
        pub fn set_dbg_rtc_stop(&mut self, val: bool) {
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
                .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
                .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
                .field("dbg_lptim3_stop", &self.dbg_lptim3_stop())
                .field("dbg_lptim4_stop", &self.dbg_lptim4_stop())
                .field("dbg_rtc_stop", &self.dbg_rtc_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb3fzr {{ dbg_i2c3_stop: {=bool:?}, dbg_lptim1_stop: {=bool:?}, dbg_lptim3_stop: {=bool:?}, dbg_lptim4_stop: {=bool:?}, dbg_rtc_stop: {=bool:?} }}" , self . dbg_i2c3_stop () , self . dbg_lptim1_stop () , self . dbg_lptim3_stop () , self . dbg_lptim4_stop () , self . dbg_rtc_stop ())
        }
    }
    #[doc = "Debug MCU CoreSight component identity register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr0(pub u32);
    impl Cidr0 {
        #[doc = "component identification bits \\[7:0\\]"]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "component identification bits \\[7:0\\]"]
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
    #[doc = "Debug MCU CoreSight component identity register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr1(pub u32);
    impl Cidr1 {
        #[doc = "component identification bits \\[11:8\\]"]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "component identification bits \\[11:8\\]"]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "component identification bits \\[15:12\\]
- component class"]
        #[inline(always)]
        pub const fn class(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "component identification bits \\[15:12\\]
- component class"]
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
    #[doc = "Debug MCU CoreSight component identity register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr2(pub u32);
    impl Cidr2 {
        #[doc = "component identification bits \\[23:16\\]"]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "component identification bits \\[23:16\\]"]
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
    #[doc = "Debug MCU CoreSight component identity register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr3(pub u32);
    impl Cidr3 {
        #[doc = "component identification bits \\[31:24\\]"]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "component identification bits \\[31:24\\]"]
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
    #[doc = "Debug MCU configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
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
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Trace pin assignment control"]
        #[inline(always)]
        pub fn set_trace_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "trace port and clock enable"]
        #[inline(always)]
        pub const fn trace_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "trace port and clock enable"]
        #[inline(always)]
        pub fn set_trace_en(&mut self, val: bool) {
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
    #[doc = "DBGMCU debug device authentication register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgAuthDevice(pub u32);
    impl DbgmcuDbgAuthDevice {
        #[doc = "Device specific ID Device specific ID used for RDP regression."]
        #[inline(always)]
        pub const fn auth_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device specific ID Device specific ID used for RDP regression."]
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
    #[doc = "DBGMCU debug host authentication register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgAuthHost(pub u32);
    impl DbgmcuDbgAuthHost {
        #[doc = "Device authentication key The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory."]
        #[inline(always)]
        pub const fn auth_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device authentication key The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory."]
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
    #[doc = "DBGMCU status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuSr(pub u32);
    impl DbgmcuSr {
        #[doc = "Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present"]
        #[inline(always)]
        pub const fn ap_present(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present"]
        #[inline(always)]
        pub fn set_ap_present(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "DECLARATION TO BE CONFIRMED by PRODUCT OWNER! Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled"]
        #[inline(always)]
        pub const fn ap_locked(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "DECLARATION TO BE CONFIRMED by PRODUCT OWNER! Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled"]
        #[inline(always)]
        pub fn set_ap_locked(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
                .field("ap_locked", &self.ap_locked())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuSr {{ ap_present: {=u8:?}, ap_locked: {=u8:?} }}",
                self.ap_present(),
                self.ap_locked()
            )
        }
    }
    #[doc = "DBGMCU_IDCODE"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "Device dentification"]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Device dentification"]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Revision"]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision"]
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
    #[doc = "Debug MCU CoreSight peripheral identity register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr0(pub u32);
    impl Pidr0 {
        #[doc = "part number bits \\[7:0\\]"]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "part number bits \\[7:0\\]"]
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
    #[doc = "Debug MCU CoreSight peripheral identity register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr1(pub u32);
    impl Pidr1 {
        #[doc = "part number bits \\[11:8\\]"]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "part number bits \\[11:8\\]"]
        #[inline(always)]
        pub fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "JEP106 identity code bits \\[3:0\\]"]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "JEP106 identity code bits \\[3:0\\]"]
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
    #[doc = "Debug MCU CoreSight peripheral identity register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr2(pub u32);
    impl Pidr2 {
        #[doc = "JEP106 identity code bits \\[6:4\\]"]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "JEP106 identity code bits \\[6:4\\]"]
        #[inline(always)]
        pub fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "JEDEC assigned value"]
        #[inline(always)]
        pub const fn jedec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JEDEC assigned value"]
        #[inline(always)]
        pub fn set_jedec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "component revision number"]
        #[inline(always)]
        pub const fn revision(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "component revision number"]
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
    #[doc = "Debug MCU CoreSight peripheral identity register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr3(pub u32);
    impl Pidr3 {
        #[doc = "customer modified"]
        #[inline(always)]
        pub const fn cmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "customer modified"]
        #[inline(always)]
        pub fn set_cmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "metal fix version"]
        #[inline(always)]
        pub const fn revand(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "metal fix version"]
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
    #[doc = "Debug MCU CoreSight peripheral identity register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr4(pub u32);
    impl Pidr4 {
        #[doc = "JEP106 continuation code"]
        #[inline(always)]
        pub const fn jep106con(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "JEP106 continuation code"]
        #[inline(always)]
        pub fn set_jep106con(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "register file size"]
        #[inline(always)]
        pub const fn kcount_4(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "register file size"]
        #[inline(always)]
        pub fn set_kcount_4(&mut self, val: u8) {
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
                .field("kcount_4", &self.kcount_4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr4 {{ jep106con: {=u8:?}, kcount_4: {=u8:?} }}",
                self.jep106con(),
                self.kcount_4()
            )
        }
    }
}
