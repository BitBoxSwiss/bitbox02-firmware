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
    pub const fn apb1lfz1(self) -> crate::common::Reg<regs::Apb1lfz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DBGMCU APB1H peripheral freeze register."]
    #[inline(always)]
    pub const fn apb1hfz1(self) -> crate::common::Reg<regs::Apb1hfz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[inline(always)]
    pub const fn apb2fz1(self) -> crate::common::Reg<regs::Apb2fz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DBGMCU APB4 peripheral freeze register."]
    #[inline(always)]
    pub const fn apb4fz1(self) -> crate::common::Reg<regs::Apb4fz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "DBGMCU APB5 peripheral freeze register."]
    #[inline(always)]
    pub const fn apb5fz1(self) -> crate::common::Reg<regs::Apb5fz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[inline(always)]
    pub const fn ahb1fz1(self) -> crate::common::Reg<regs::Ahb1fz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "DBGMCU AHB5 peripheral freeze register."]
    #[inline(always)]
    pub const fn ahb5fz1(self) -> crate::common::Reg<regs::Ahb5fz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "DBGMCU status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "DBGMCU host authentication register."]
    #[inline(always)]
    pub const fn dbg_auth_host(self) -> crate::common::Reg<regs::DbgAuthHost, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "DBGMCU device authentication register."]
    #[inline(always)]
    pub const fn dbg_auth_dev(self) -> crate::common::Reg<regs::DbgAuthDev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "DBGMCU message read acknowledge authentication register."]
    #[inline(always)]
    pub const fn dbg_auth_ack(self) -> crate::common::Reg<regs::DbgAuthAck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
}
pub mod regs {
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1fz1(pub u32);
    impl Ahb1fz1 {
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch0_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch1_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch2_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch3_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch4_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch5_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch6_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch7_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch8_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch9_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch10_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch11_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch12_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch12_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch13_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch13_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch14_stop(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch14_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch15_stop(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ahb1fz1 {
        #[inline(always)]
        fn default() -> Ahb1fz1 {
            Ahb1fz1(0)
        }
    }
    impl core::fmt::Debug for Ahb1fz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1fz1")
                .field("dbg_gpdma1_ch0_stop", &self.dbg_gpdma1_ch0_stop())
                .field("dbg_gpdma1_ch1_stop", &self.dbg_gpdma1_ch1_stop())
                .field("dbg_gpdma1_ch2_stop", &self.dbg_gpdma1_ch2_stop())
                .field("dbg_gpdma1_ch3_stop", &self.dbg_gpdma1_ch3_stop())
                .field("dbg_gpdma1_ch4_stop", &self.dbg_gpdma1_ch4_stop())
                .field("dbg_gpdma1_ch5_stop", &self.dbg_gpdma1_ch5_stop())
                .field("dbg_gpdma1_ch6_stop", &self.dbg_gpdma1_ch6_stop())
                .field("dbg_gpdma1_ch7_stop", &self.dbg_gpdma1_ch7_stop())
                .field("dbg_gpdma1_ch8_stop", &self.dbg_gpdma1_ch8_stop())
                .field("dbg_gpdma1_ch9_stop", &self.dbg_gpdma1_ch9_stop())
                .field("dbg_gpdma1_ch10_stop", &self.dbg_gpdma1_ch10_stop())
                .field("dbg_gpdma1_ch11_stop", &self.dbg_gpdma1_ch11_stop())
                .field("dbg_gpdma1_ch12_stop", &self.dbg_gpdma1_ch12_stop())
                .field("dbg_gpdma1_ch13_stop", &self.dbg_gpdma1_ch13_stop())
                .field("dbg_gpdma1_ch14_stop", &self.dbg_gpdma1_ch14_stop())
                .field("dbg_gpdma1_ch15_stop", &self.dbg_gpdma1_ch15_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1fz1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1fz1 {{ dbg_gpdma1_ch0_stop: {=bool:?}, dbg_gpdma1_ch1_stop: {=bool:?}, dbg_gpdma1_ch2_stop: {=bool:?}, dbg_gpdma1_ch3_stop: {=bool:?}, dbg_gpdma1_ch4_stop: {=bool:?}, dbg_gpdma1_ch5_stop: {=bool:?}, dbg_gpdma1_ch6_stop: {=bool:?}, dbg_gpdma1_ch7_stop: {=bool:?}, dbg_gpdma1_ch8_stop: {=bool:?}, dbg_gpdma1_ch9_stop: {=bool:?}, dbg_gpdma1_ch10_stop: {=bool:?}, dbg_gpdma1_ch11_stop: {=bool:?}, dbg_gpdma1_ch12_stop: {=bool:?}, dbg_gpdma1_ch13_stop: {=bool:?}, dbg_gpdma1_ch14_stop: {=bool:?}, dbg_gpdma1_ch15_stop: {=bool:?} }}" , self . dbg_gpdma1_ch0_stop () , self . dbg_gpdma1_ch1_stop () , self . dbg_gpdma1_ch2_stop () , self . dbg_gpdma1_ch3_stop () , self . dbg_gpdma1_ch4_stop () , self . dbg_gpdma1_ch5_stop () , self . dbg_gpdma1_ch6_stop () , self . dbg_gpdma1_ch7_stop () , self . dbg_gpdma1_ch8_stop () , self . dbg_gpdma1_ch9_stop () , self . dbg_gpdma1_ch10_stop () , self . dbg_gpdma1_ch11_stop () , self . dbg_gpdma1_ch12_stop () , self . dbg_gpdma1_ch13_stop () , self . dbg_gpdma1_ch14_stop () , self . dbg_gpdma1_ch15_stop ())
        }
    }
    #[doc = "DBGMCU AHB5 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5fz1(pub u32);
    impl Ahb5fz1 {
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch0_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch1_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch2_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch3_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch4_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch5_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch6_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch7_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch8_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch9_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch10_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch11_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch12_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch12_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch13_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch13_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch14_stop(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch14_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub const fn dbg_hpdma1_ch15_stop(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA3_CHn suspend in debug."]
        #[inline(always)]
        pub fn set_dbg_hpdma1_ch15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "NPU stop in debug mode."]
        #[inline(always)]
        pub const fn npu_dbg_freeze(&self) -> super::vals::NpuDbgFreeze {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::NpuDbgFreeze::from_bits(val as u8)
        }
        #[doc = "NPU stop in debug mode."]
        #[inline(always)]
        pub fn set_npu_dbg_freeze(&mut self, val: super::vals::NpuDbgFreeze) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ahb5fz1 {
        #[inline(always)]
        fn default() -> Ahb5fz1 {
            Ahb5fz1(0)
        }
    }
    impl core::fmt::Debug for Ahb5fz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb5fz1")
                .field("dbg_hpdma1_ch0_stop", &self.dbg_hpdma1_ch0_stop())
                .field("dbg_hpdma1_ch1_stop", &self.dbg_hpdma1_ch1_stop())
                .field("dbg_hpdma1_ch2_stop", &self.dbg_hpdma1_ch2_stop())
                .field("dbg_hpdma1_ch3_stop", &self.dbg_hpdma1_ch3_stop())
                .field("dbg_hpdma1_ch4_stop", &self.dbg_hpdma1_ch4_stop())
                .field("dbg_hpdma1_ch5_stop", &self.dbg_hpdma1_ch5_stop())
                .field("dbg_hpdma1_ch6_stop", &self.dbg_hpdma1_ch6_stop())
                .field("dbg_hpdma1_ch7_stop", &self.dbg_hpdma1_ch7_stop())
                .field("dbg_hpdma1_ch8_stop", &self.dbg_hpdma1_ch8_stop())
                .field("dbg_hpdma1_ch9_stop", &self.dbg_hpdma1_ch9_stop())
                .field("dbg_hpdma1_ch10_stop", &self.dbg_hpdma1_ch10_stop())
                .field("dbg_hpdma1_ch11_stop", &self.dbg_hpdma1_ch11_stop())
                .field("dbg_hpdma1_ch12_stop", &self.dbg_hpdma1_ch12_stop())
                .field("dbg_hpdma1_ch13_stop", &self.dbg_hpdma1_ch13_stop())
                .field("dbg_hpdma1_ch14_stop", &self.dbg_hpdma1_ch14_stop())
                .field("dbg_hpdma1_ch15_stop", &self.dbg_hpdma1_ch15_stop())
                .field("npu_dbg_freeze", &self.npu_dbg_freeze())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5fz1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb5fz1 {{ dbg_hpdma1_ch0_stop: {=bool:?}, dbg_hpdma1_ch1_stop: {=bool:?}, dbg_hpdma1_ch2_stop: {=bool:?}, dbg_hpdma1_ch3_stop: {=bool:?}, dbg_hpdma1_ch4_stop: {=bool:?}, dbg_hpdma1_ch5_stop: {=bool:?}, dbg_hpdma1_ch6_stop: {=bool:?}, dbg_hpdma1_ch7_stop: {=bool:?}, dbg_hpdma1_ch8_stop: {=bool:?}, dbg_hpdma1_ch9_stop: {=bool:?}, dbg_hpdma1_ch10_stop: {=bool:?}, dbg_hpdma1_ch11_stop: {=bool:?}, dbg_hpdma1_ch12_stop: {=bool:?}, dbg_hpdma1_ch13_stop: {=bool:?}, dbg_hpdma1_ch14_stop: {=bool:?}, dbg_hpdma1_ch15_stop: {=bool:?}, npu_dbg_freeze: {:?} }}" , self . dbg_hpdma1_ch0_stop () , self . dbg_hpdma1_ch1_stop () , self . dbg_hpdma1_ch2_stop () , self . dbg_hpdma1_ch3_stop () , self . dbg_hpdma1_ch4_stop () , self . dbg_hpdma1_ch5_stop () , self . dbg_hpdma1_ch6_stop () , self . dbg_hpdma1_ch7_stop () , self . dbg_hpdma1_ch8_stop () , self . dbg_hpdma1_ch9_stop () , self . dbg_hpdma1_ch10_stop () , self . dbg_hpdma1_ch11_stop () , self . dbg_hpdma1_ch12_stop () , self . dbg_hpdma1_ch13_stop () , self . dbg_hpdma1_ch14_stop () , self . dbg_hpdma1_ch15_stop () , self . npu_dbg_freeze ())
        }
    }
    #[doc = "DBGMCU APB1H peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hfz1(pub u32);
    impl Apb1hfz1 {
        #[doc = "FDCAN stop in debug."]
        #[inline(always)]
        pub const fn dbg_fdcan_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN stop in debug."]
        #[inline(always)]
        pub fn set_dbg_fdcan_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Apb1hfz1 {
        #[inline(always)]
        fn default() -> Apb1hfz1 {
            Apb1hfz1(0)
        }
    }
    impl core::fmt::Debug for Apb1hfz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1hfz1")
                .field("dbg_fdcan_stop", &self.dbg_fdcan_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hfz1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb1hfz1 {{ dbg_fdcan_stop: {=bool:?} }}", self.dbg_fdcan_stop())
        }
    }
    #[doc = "DBGMCU APB1L peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lfz1(pub u32);
    impl Apb1lfz1 {
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
        #[doc = "TIM4 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim4_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim5_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
        #[doc = "TIM12 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim12_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim12_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim13_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim13_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim14_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim14_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 stop in debug."]
        #[inline(always)]
        pub const fn dbg_lptim1_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_lptim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG1 stop in debug."]
        #[inline(always)]
        pub const fn dbg_wwdg1_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_wwdg1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM10 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim10_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM11 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim11_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "I2C1 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn dbg_i2c1_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_dbg_i2c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn dbg_i2c2_stop(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_dbg_i2c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn dbg_i2c3_stop(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_dbg_i2c3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I3C1 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn dbg_i3c1_stop(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_dbg_i3c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "I3C2 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn dbg_i3c2_stop(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I3C2 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub fn set_dbg_i3c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1lfz1 {
        #[inline(always)]
        fn default() -> Apb1lfz1 {
            Apb1lfz1(0)
        }
    }
    impl core::fmt::Debug for Apb1lfz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lfz1")
                .field("dbg_tim2_stop", &self.dbg_tim2_stop())
                .field("dbg_tim3_stop", &self.dbg_tim3_stop())
                .field("dbg_tim4_stop", &self.dbg_tim4_stop())
                .field("dbg_tim5_stop", &self.dbg_tim5_stop())
                .field("dbg_tim6_stop", &self.dbg_tim6_stop())
                .field("dbg_tim7_stop", &self.dbg_tim7_stop())
                .field("dbg_tim12_stop", &self.dbg_tim12_stop())
                .field("dbg_tim13_stop", &self.dbg_tim13_stop())
                .field("dbg_tim14_stop", &self.dbg_tim14_stop())
                .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
                .field("dbg_wwdg1_stop", &self.dbg_wwdg1_stop())
                .field("dbg_tim10_stop", &self.dbg_tim10_stop())
                .field("dbg_tim11_stop", &self.dbg_tim11_stop())
                .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
                .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
                .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
                .field("dbg_i3c1_stop", &self.dbg_i3c1_stop())
                .field("dbg_i3c2_stop", &self.dbg_i3c2_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lfz1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1lfz1 {{ dbg_tim2_stop: {=bool:?}, dbg_tim3_stop: {=bool:?}, dbg_tim4_stop: {=bool:?}, dbg_tim5_stop: {=bool:?}, dbg_tim6_stop: {=bool:?}, dbg_tim7_stop: {=bool:?}, dbg_tim12_stop: {=bool:?}, dbg_tim13_stop: {=bool:?}, dbg_tim14_stop: {=bool:?}, dbg_lptim1_stop: {=bool:?}, dbg_wwdg1_stop: {=bool:?}, dbg_tim10_stop: {=bool:?}, dbg_tim11_stop: {=bool:?}, dbg_i2c1_stop: {=bool:?}, dbg_i2c2_stop: {=bool:?}, dbg_i2c3_stop: {=bool:?}, dbg_i3c1_stop: {=bool:?}, dbg_i3c2_stop: {=bool:?} }}" , self . dbg_tim2_stop () , self . dbg_tim3_stop () , self . dbg_tim4_stop () , self . dbg_tim5_stop () , self . dbg_tim6_stop () , self . dbg_tim7_stop () , self . dbg_tim12_stop () , self . dbg_tim13_stop () , self . dbg_tim14_stop () , self . dbg_lptim1_stop () , self . dbg_wwdg1_stop () , self . dbg_tim10_stop () , self . dbg_tim11_stop () , self . dbg_i2c1_stop () , self . dbg_i2c2_stop () , self . dbg_i2c3_stop () , self . dbg_i3c1_stop () , self . dbg_i3c2_stop ())
        }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fz1(pub u32);
    impl Apb2fz1 {
        #[doc = "TIM1 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim1_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim8_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM18 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim18_stop(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "TIM18 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim18_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "TIM17 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim17_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim17_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 stop in debug."]
        #[inline(always)]
        pub const fn dbg_tim9_stop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_tim9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Apb2fz1 {
        #[inline(always)]
        fn default() -> Apb2fz1 {
            Apb2fz1(0)
        }
    }
    impl core::fmt::Debug for Apb2fz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2fz1")
                .field("dbg_tim1_stop", &self.dbg_tim1_stop())
                .field("dbg_tim8_stop", &self.dbg_tim8_stop())
                .field("dbg_tim18_stop", &self.dbg_tim18_stop())
                .field("dbg_tim15_stop", &self.dbg_tim15_stop())
                .field("dbg_tim16_stop", &self.dbg_tim16_stop())
                .field("dbg_tim17_stop", &self.dbg_tim17_stop())
                .field("dbg_tim9_stop", &self.dbg_tim9_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fz1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2fz1 {{ dbg_tim1_stop: {=bool:?}, dbg_tim8_stop: {=bool:?}, dbg_tim18_stop: {=bool:?}, dbg_tim15_stop: {=bool:?}, dbg_tim16_stop: {=bool:?}, dbg_tim17_stop: {=bool:?}, dbg_tim9_stop: {=bool:?} }}" , self . dbg_tim1_stop () , self . dbg_tim8_stop () , self . dbg_tim18_stop () , self . dbg_tim15_stop () , self . dbg_tim16_stop () , self . dbg_tim17_stop () , self . dbg_tim9_stop ())
        }
    }
    #[doc = "DBGMCU APB4 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4fz1(pub u32);
    impl Apb4fz1 {
        #[doc = "I2C4 stop in debug."]
        #[inline(always)]
        pub const fn dbg_i2c4_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_i2c4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM2 stop in debug."]
        #[inline(always)]
        pub const fn dbg_lptim2_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_lptim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 stop in debug."]
        #[inline(always)]
        pub const fn dbg_lptim3_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_lptim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 stop in debug."]
        #[inline(always)]
        pub const fn dbg_lptim4_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_lptim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 stop in debug."]
        #[inline(always)]
        pub const fn dbg_lptim5_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 stop in debug."]
        #[inline(always)]
        pub fn set_dbg_lptim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RTC clock is suspended in debug."]
        #[inline(always)]
        pub const fn dbg_rtc_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock is suspended in debug."]
        #[inline(always)]
        pub fn set_dbg_rtc_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "WWDG stop in debug."]
        #[inline(always)]
        pub const fn dbg_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG stop in debug."]
        #[inline(always)]
        pub fn set_dbg_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Apb4fz1 {
        #[inline(always)]
        fn default() -> Apb4fz1 {
            Apb4fz1(0)
        }
    }
    impl core::fmt::Debug for Apb4fz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb4fz1")
                .field("dbg_i2c4_stop", &self.dbg_i2c4_stop())
                .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
                .field("dbg_lptim3_stop", &self.dbg_lptim3_stop())
                .field("dbg_lptim4_stop", &self.dbg_lptim4_stop())
                .field("dbg_lptim5_stop", &self.dbg_lptim5_stop())
                .field("dbg_rtc_stop", &self.dbg_rtc_stop())
                .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4fz1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb4fz1 {{ dbg_i2c4_stop: {=bool:?}, dbg_lptim2_stop: {=bool:?}, dbg_lptim3_stop: {=bool:?}, dbg_lptim4_stop: {=bool:?}, dbg_lptim5_stop: {=bool:?}, dbg_rtc_stop: {=bool:?}, dbg_iwdg_stop: {=bool:?} }}" , self . dbg_i2c4_stop () , self . dbg_lptim2_stop () , self . dbg_lptim3_stop () , self . dbg_lptim4_stop () , self . dbg_lptim5_stop () , self . dbg_rtc_stop () , self . dbg_iwdg_stop ())
        }
    }
    #[doc = "DBGMCU APB5 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb5fz1(pub u32);
    impl Apb5fz1 {
        #[doc = "GFXTIM stop in debug."]
        #[inline(always)]
        pub const fn dbg_gfxtim_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM stop in debug."]
        #[inline(always)]
        pub fn set_dbg_gfxtim_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb5fz1 {
        #[inline(always)]
        fn default() -> Apb5fz1 {
            Apb5fz1(0)
        }
    }
    impl core::fmt::Debug for Apb5fz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb5fz1")
                .field("dbg_gfxtim_stop", &self.dbg_gfxtim_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb5fz1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb5fz1 {{ dbg_gfxtim_stop: {=bool:?} }}", self.dbg_gfxtim_stop())
        }
    }
    #[doc = "DBGMCU configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Allow debug in Sleep mode."]
        #[inline(always)]
        pub const fn dbg_sleep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Allow debug in Sleep mode."]
        #[inline(always)]
        pub fn set_dbg_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Allow debug in Stop mode."]
        #[inline(always)]
        pub const fn dbg_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Allow debug in Stop mode."]
        #[inline(always)]
        pub fn set_dbg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Allow debug in Standby mode."]
        #[inline(always)]
        pub const fn dbg_standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Allow debug in Standby mode."]
        #[inline(always)]
        pub fn set_dbg_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Debug clock enable through software."]
        #[inline(always)]
        pub const fn dbgclken(&self) -> super::vals::Dbgclken {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Dbgclken::from_bits(val as u8)
        }
        #[doc = "Debug clock enable through software."]
        #[inline(always)]
        pub fn set_dbgclken(&mut self, val: super::vals::Dbgclken) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "TPIU export clock enable through software."]
        #[inline(always)]
        pub const fn traceclken(&self) -> super::vals::Traceclken {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Traceclken::from_bits(val as u8)
        }
        #[doc = "TPIU export clock enable through software."]
        #[inline(always)]
        pub fn set_traceclken(&mut self, val: super::vals::Traceclken) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "DBTRGIO connection control."]
        #[inline(always)]
        pub const fn dbtrgoen(&self) -> super::vals::Dbtrgoen {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Dbtrgoen::from_bits(val as u8)
        }
        #[doc = "DBTRGIO connection control."]
        #[inline(always)]
        pub fn set_dbtrgoen(&mut self, val: super::vals::Dbtrgoen) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "TSGEN halt enable."]
        #[inline(always)]
        pub const fn hlt_tsgen_en(&self) -> super::vals::HltTsgenEn {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::HltTsgenEn::from_bits(val as u8)
        }
        #[doc = "TSGEN halt enable."]
        #[inline(always)]
        pub fn set_hlt_tsgen_en(&mut self, val: super::vals::HltTsgenEn) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                .field("dbgclken", &self.dbgclken())
                .field("traceclken", &self.traceclken())
                .field("dbtrgoen", &self.dbtrgoen())
                .field("hlt_tsgen_en", &self.hlt_tsgen_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ dbg_sleep: {=bool:?}, dbg_stop: {=bool:?}, dbg_standby: {=bool:?}, dbgclken: {:?}, traceclken: {:?}, dbtrgoen: {:?}, hlt_tsgen_en: {:?} }}" , self . dbg_sleep () , self . dbg_stop () , self . dbg_standby () , self . dbgclken () , self . traceclken () , self . dbtrgoen () , self . hlt_tsgen_en ())
        }
    }
    #[doc = "DBGMCU message read acknowledge authentication register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgAuthAck(pub u32);
    impl DbgAuthAck {
        #[doc = "Access status to DBG_AUTH_HOST register."]
        #[inline(always)]
        pub const fn host_ack(&self) -> super::vals::HostAck {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::HostAck::from_bits(val as u8)
        }
        #[doc = "Access status to DBG_AUTH_HOST register."]
        #[inline(always)]
        pub fn set_host_ack(&mut self, val: super::vals::HostAck) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Access status to DBG_AUTH_DEV register."]
        #[inline(always)]
        pub const fn device_ack(&self) -> super::vals::DeviceAck {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::DeviceAck::from_bits(val as u8)
        }
        #[doc = "Access status to DBG_AUTH_DEV register."]
        #[inline(always)]
        pub fn set_device_ack(&mut self, val: super::vals::DeviceAck) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DbgAuthAck {
        #[inline(always)]
        fn default() -> DbgAuthAck {
            DbgAuthAck(0)
        }
    }
    impl core::fmt::Debug for DbgAuthAck {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgAuthAck")
                .field("host_ack", &self.host_ack())
                .field("device_ack", &self.device_ack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgAuthAck {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgAuthAck {{ host_ack: {:?}, device_ack: {:?} }}",
                self.host_ack(),
                self.device_ack()
            )
        }
    }
    #[doc = "DBGMCU device authentication register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgAuthDev(pub u32);
    impl DbgAuthDev {
        #[doc = "Mailbox between debugger and processor."]
        #[inline(always)]
        pub const fn message(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mailbox between debugger and processor."]
        #[inline(always)]
        pub fn set_message(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgAuthDev {
        #[inline(always)]
        fn default() -> DbgAuthDev {
            DbgAuthDev(0)
        }
    }
    impl core::fmt::Debug for DbgAuthDev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgAuthDev").field("message", &self.message()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgAuthDev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgAuthDev {{ message: {=u32:?} }}", self.message())
        }
    }
    #[doc = "DBGMCU host authentication register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgAuthHost(pub u32);
    impl DbgAuthHost {
        #[doc = "Mailbox between debugger and processor."]
        #[inline(always)]
        pub const fn message(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mailbox between debugger and processor."]
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
    #[doc = "DBGMCU identity code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "Device ID."]
        #[inline(always)]
        pub const fn dev_id(&self) -> super::vals::DevId {
            let val = (self.0 >> 0usize) & 0x0fff;
            super::vals::DevId::from_bits(val as u16)
        }
        #[doc = "Device ID."]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: super::vals::DevId) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Revision."]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision."]
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
                "Idcode {{ dev_id: {:?}, rev_id: {=u16:?} }}",
                self.dev_id(),
                self.rev_id()
            )
        }
    }
    #[doc = "DBGMCU status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Access point 0 presence."]
        #[inline(always)]
        pub const fn ap0_present(&self) -> super::vals::Ap0Present {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ap0Present::from_bits(val as u8)
        }
        #[doc = "Access point 0 presence."]
        #[inline(always)]
        pub fn set_ap0_present(&mut self, val: super::vals::Ap0Present) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Access point 1 presence."]
        #[inline(always)]
        pub const fn ap1_present(&self) -> super::vals::Ap1Present {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Ap1Present::from_bits(val as u8)
        }
        #[doc = "Access point 1 presence."]
        #[inline(always)]
        pub fn set_ap1_present(&mut self, val: super::vals::Ap1Present) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Access point 0 enable."]
        #[inline(always)]
        pub const fn ap0_enable(&self) -> super::vals::Ap0Enable {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Ap0Enable::from_bits(val as u8)
        }
        #[doc = "Access point 0 enable."]
        #[inline(always)]
        pub fn set_ap0_enable(&mut self, val: super::vals::Ap0Enable) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Access point 1 enable."]
        #[inline(always)]
        pub const fn ap1_enable(&self) -> super::vals::Ap1Enable {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ap1Enable::from_bits(val as u8)
        }
        #[doc = "Access point 1 enable."]
        #[inline(always)]
        pub fn set_ap1_enable(&mut self, val: super::vals::Ap1Enable) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
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
                .field("ap0_present", &self.ap0_present())
                .field("ap1_present", &self.ap1_present())
                .field("ap0_enable", &self.ap0_enable())
                .field("ap1_enable", &self.ap1_enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ ap0_present: {:?}, ap1_present: {:?}, ap0_enable: {:?}, ap1_enable: {:?} }}",
                self.ap0_present(),
                self.ap1_present(),
                self.ap0_enable(),
                self.ap1_enable()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ap0Enable {
        _RESERVED_0 = 0x0,
        #[doc = "Always enable."]
        B_0X1 = 0x01,
    }
    impl Ap0Enable {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ap0Enable {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ap0Enable {
        #[inline(always)]
        fn from(val: u8) -> Ap0Enable {
            Ap0Enable::from_bits(val)
        }
    }
    impl From<Ap0Enable> for u8 {
        #[inline(always)]
        fn from(val: Ap0Enable) -> u8 {
            Ap0Enable::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ap0Present {
        _RESERVED_0 = 0x0,
        #[doc = "AP present."]
        B_0X1 = 0x01,
    }
    impl Ap0Present {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ap0Present {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ap0Present {
        #[inline(always)]
        fn from(val: u8) -> Ap0Present {
            Ap0Present::from_bits(val)
        }
    }
    impl From<Ap0Present> for u8 {
        #[inline(always)]
        fn from(val: Ap0Present) -> u8 {
            Ap0Present::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ap1Enable {
        #[doc = "AP disabled (debug access locked)."]
        B_0X0 = 0x0,
        #[doc = "AP enabled (debug access open)."]
        B_0X1 = 0x01,
    }
    impl Ap1Enable {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ap1Enable {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ap1Enable {
        #[inline(always)]
        fn from(val: u8) -> Ap1Enable {
            Ap1Enable::from_bits(val)
        }
    }
    impl From<Ap1Enable> for u8 {
        #[inline(always)]
        fn from(val: Ap1Enable) -> u8 {
            Ap1Enable::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ap1Present {
        _RESERVED_0 = 0x0,
        #[doc = "AP present."]
        B_0X1 = 0x01,
    }
    impl Ap1Present {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ap1Present {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ap1Present {
        #[inline(always)]
        fn from(val: u8) -> Ap1Present {
            Ap1Present::from_bits(val)
        }
    }
    impl From<Ap1Present> for u8 {
        #[inline(always)]
        fn from(val: Ap1Present) -> u8 {
            Ap1Present::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dbgclken {
        #[doc = "Debug clock is off."]
        B_0X0 = 0x0,
        #[doc = "Debug clock is on."]
        B_0X1 = 0x01,
    }
    impl Dbgclken {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dbgclken {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dbgclken {
        #[inline(always)]
        fn from(val: u8) -> Dbgclken {
            Dbgclken::from_bits(val)
        }
    }
    impl From<Dbgclken> for u8 {
        #[inline(always)]
        fn from(val: Dbgclken) -> u8 {
            Dbgclken::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dbtrgoen {
        #[doc = "DBTRGIO connected to DBTRGIN."]
        B_0X0 = 0x0,
        #[doc = "DBTRGIO connected to DBTRGOUT."]
        B_0X1 = 0x01,
    }
    impl Dbtrgoen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dbtrgoen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dbtrgoen {
        #[inline(always)]
        fn from(val: u8) -> Dbtrgoen {
            Dbtrgoen::from_bits(val)
        }
    }
    impl From<Dbtrgoen> for u8 {
        #[inline(always)]
        fn from(val: Dbtrgoen) -> u8 {
            Dbtrgoen::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DevId(u16);
    impl DevId {
        #[doc = "STM32N6xx."]
        pub const B_0X486: Self = Self(0x0486);
    }
    impl DevId {
        pub const fn from_bits(val: u16) -> DevId {
            Self(val & 0x0fff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for DevId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0486 => f.write_str("B_0X486"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DevId {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0486 => defmt::write!(f, "B_0X486"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for DevId {
        #[inline(always)]
        fn from(val: u16) -> DevId {
            DevId::from_bits(val)
        }
    }
    impl From<DevId> for u16 {
        #[inline(always)]
        fn from(val: DevId) -> u16 {
            DevId::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DeviceAck {
        #[doc = "Debugger has read DBG_AUTH_DEV."]
        B_0X0 = 0x0,
        #[doc = "Processor has written DBG_AUTH_DEV."]
        B_0X1 = 0x01,
    }
    impl DeviceAck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DeviceAck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DeviceAck {
        #[inline(always)]
        fn from(val: u8) -> DeviceAck {
            DeviceAck::from_bits(val)
        }
    }
    impl From<DeviceAck> for u8 {
        #[inline(always)]
        fn from(val: DeviceAck) -> u8 {
            DeviceAck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HltTsgenEn {
        #[doc = "TSGEN keeps on counting when processor is in halt."]
        B_0X0 = 0x0,
        #[doc = "TSGEN stops counting when processor is in halt."]
        B_0X1 = 0x01,
    }
    impl HltTsgenEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HltTsgenEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HltTsgenEn {
        #[inline(always)]
        fn from(val: u8) -> HltTsgenEn {
            HltTsgenEn::from_bits(val)
        }
    }
    impl From<HltTsgenEn> for u8 {
        #[inline(always)]
        fn from(val: HltTsgenEn) -> u8 {
            HltTsgenEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HostAck {
        #[doc = "Processor has read DBG_AUTH_HOST."]
        B_0X0 = 0x0,
        #[doc = "Debugger has written DBG_AUTH_HOST."]
        B_0X1 = 0x01,
    }
    impl HostAck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HostAck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HostAck {
        #[inline(always)]
        fn from(val: u8) -> HostAck {
            HostAck::from_bits(val)
        }
    }
    impl From<HostAck> for u8 {
        #[inline(always)]
        fn from(val: HostAck) -> u8 {
            HostAck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum NpuDbgFreeze {
        #[doc = "Normal operation. The NPU continues to operate while Cortex-M55 is in debug mode."]
        B_0X0 = 0x0,
        #[doc = "Stop in debug. NPU is suspended while Cortex-M55 is in debug mode."]
        B_0X1 = 0x01,
    }
    impl NpuDbgFreeze {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NpuDbgFreeze {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NpuDbgFreeze {
        #[inline(always)]
        fn from(val: u8) -> NpuDbgFreeze {
            NpuDbgFreeze::from_bits(val)
        }
    }
    impl From<NpuDbgFreeze> for u8 {
        #[inline(always)]
        fn from(val: NpuDbgFreeze) -> u8 {
            NpuDbgFreeze::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Traceclken {
        #[doc = "TPIU clock is off."]
        B_0X0 = 0x0,
        #[doc = "TPIU clock is on."]
        B_0X1 = 0x01,
    }
    impl Traceclken {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Traceclken {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Traceclken {
        #[inline(always)]
        fn from(val: u8) -> Traceclken {
            Traceclken::from_bits(val)
        }
    }
    impl From<Traceclken> for u8 {
        #[inline(always)]
        fn from(val: Traceclken) -> u8 {
            Traceclken::to_bits(val)
        }
    }
}
