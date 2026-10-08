#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Global privilege controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtzc1 {
    ptr: *mut u8,
}
unsafe impl Send for Gtzc1 {}
unsafe impl Sync for Gtzc1 {}
impl Gtzc1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GTZC1 TZSC privilege configuration register 1."]
    #[inline(always)]
    pub const fn tzsc_privcfgr1(self) -> crate::common::Reg<regs::TzscPrivcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 2."]
    #[inline(always)]
    pub const fn tzsc_privcfgr2(self) -> crate::common::Reg<regs::TzscPrivcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 3."]
    #[inline(always)]
    pub const fn tzsc_privcfgr3(self) -> crate::common::Reg<regs::TzscPrivcfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark configuration register."]
    #[inline(always)]
    pub const fn tzsc_mpcwm4acfgr(self) -> crate::common::Reg<regs::TzscMpcwm4acfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark register."]
    #[inline(always)]
    pub const fn tzsc_mpcwm4ar(self) -> crate::common::Reg<regs::TzscMpcwm4ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register."]
    #[inline(always)]
    pub const fn tzsc_mpcwm4bcfgr(self) -> crate::common::Reg<regs::TzscMpcwm4bcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark register."]
    #[inline(always)]
    pub const fn tzsc_mpcwm4br(self) -> crate::common::Reg<regs::TzscMpcwm4br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register."]
    #[inline(always)]
    pub const fn mpcbb1_privcfgr(self, n: usize) -> crate::common::Reg<regs::Mpcbb1Privcfgr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register."]
    #[inline(always)]
    pub const fn mpcbb2_privcfgr(self, n: usize) -> crate::common::Reg<regs::Mpcbb2Privcfgr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcbb1Privcfgr(pub u32);
    impl Mpcbb1Privcfgr {
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Mpcbb1Privcfgr {
        #[inline(always)]
        fn default() -> Mpcbb1Privcfgr {
            Mpcbb1Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcbb1Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcbb1Privcfgr")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcbb1Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mpcbb1Privcfgr {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcbb2Privcfgr(pub u32);
    impl Mpcbb2Privcfgr {
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub const fn priv31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
        #[inline(always)]
        pub fn set_priv31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Mpcbb2Privcfgr {
        #[inline(always)]
        fn default() -> Mpcbb2Privcfgr {
            Mpcbb2Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcbb2Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcbb2Privcfgr")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("priv8", &self.priv8())
                .field("priv9", &self.priv9())
                .field("priv10", &self.priv10())
                .field("priv11", &self.priv11())
                .field("priv12", &self.priv12())
                .field("priv13", &self.priv13())
                .field("priv14", &self.priv14())
                .field("priv15", &self.priv15())
                .field("priv16", &self.priv16())
                .field("priv17", &self.priv17())
                .field("priv18", &self.priv18())
                .field("priv19", &self.priv19())
                .field("priv20", &self.priv20())
                .field("priv21", &self.priv21())
                .field("priv22", &self.priv22())
                .field("priv23", &self.priv23())
                .field("priv24", &self.priv24())
                .field("priv25", &self.priv25())
                .field("priv26", &self.priv26())
                .field("priv27", &self.priv27())
                .field("priv28", &self.priv28())
                .field("priv29", &self.priv29())
                .field("priv30", &self.priv30())
                .field("priv31", &self.priv31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcbb2Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mpcbb2Privcfgr {{ priv0: {=bool:?}, priv1: {=bool:?}, priv2: {=bool:?}, priv3: {=bool:?}, priv4: {=bool:?}, priv5: {=bool:?}, priv6: {=bool:?}, priv7: {=bool:?}, priv8: {=bool:?}, priv9: {=bool:?}, priv10: {=bool:?}, priv11: {=bool:?}, priv12: {=bool:?}, priv13: {=bool:?}, priv14: {=bool:?}, priv15: {=bool:?}, priv16: {=bool:?}, priv17: {=bool:?}, priv18: {=bool:?}, priv19: {=bool:?}, priv20: {=bool:?}, priv21: {=bool:?}, priv22: {=bool:?}, priv23: {=bool:?}, priv24: {=bool:?}, priv25: {=bool:?}, priv26: {=bool:?}, priv27: {=bool:?}, priv28: {=bool:?}, priv29: {=bool:?}, priv30: {=bool:?}, priv31: {=bool:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . priv8 () , self . priv9 () , self . priv10 () , self . priv11 () , self . priv12 () , self . priv13 () , self . priv14 () , self . priv15 () , self . priv16 () , self . priv17 () , self . priv18 () , self . priv19 () , self . priv20 () , self . priv21 () , self . priv22 () , self . priv23 () , self . priv24 () , self . priv25 () , self . priv26 () , self . priv27 () , self . priv28 () , self . priv29 () , self . priv30 () , self . priv31 ())
        }
    }
    #[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscMpcwm4acfgr(pub u32);
    impl TzscMpcwm4acfgr {
        #[doc = "Sub-region z enable."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region z enable."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region z lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region z lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Privileged sub-region z This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region z This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for TzscMpcwm4acfgr {
        #[inline(always)]
        fn default() -> TzscMpcwm4acfgr {
            TzscMpcwm4acfgr(0)
        }
    }
    impl core::fmt::Debug for TzscMpcwm4acfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscMpcwm4acfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscMpcwm4acfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TzscMpcwm4acfgr {{ sren: {=bool:?}, srlock: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscMpcwm4ar(pub u32);
    impl TzscMpcwm4ar {
        #[doc = "Start of sub-region A This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table 16."]
        #[inline(always)]
        pub const fn suba_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region A This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table 16."]
        #[inline(always)]
        pub fn set_suba_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region A This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table 16. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled (SREN bit in TZSC_MPCMWACFGR is cleared)."]
        #[inline(always)]
        pub const fn suba_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region A This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table 16. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled (SREN bit in TZSC_MPCMWACFGR is cleared)."]
        #[inline(always)]
        pub fn set_suba_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for TzscMpcwm4ar {
        #[inline(always)]
        fn default() -> TzscMpcwm4ar {
            TzscMpcwm4ar(0)
        }
    }
    impl core::fmt::Debug for TzscMpcwm4ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscMpcwm4ar")
                .field("suba_start", &self.suba_start())
                .field("suba_length", &self.suba_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscMpcwm4ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TzscMpcwm4ar {{ suba_start: {=u16:?}, suba_length: {=u16:?} }}",
                self.suba_start(),
                self.suba_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscMpcwm4bcfgr(pub u32);
    impl TzscMpcwm4bcfgr {
        #[doc = "Sub-region z enable."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region z enable."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region z lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region z lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Privileged sub-region z This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region z This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for TzscMpcwm4bcfgr {
        #[inline(always)]
        fn default() -> TzscMpcwm4bcfgr {
            TzscMpcwm4bcfgr(0)
        }
    }
    impl core::fmt::Debug for TzscMpcwm4bcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscMpcwm4bcfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscMpcwm4bcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TzscMpcwm4bcfgr {{ sren: {=bool:?}, srlock: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscMpcwm4br(pub u32);
    impl TzscMpcwm4br {
        #[doc = "Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16."]
        #[inline(always)]
        pub const fn subb_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16."]
        #[inline(always)]
        pub fn set_subb_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in TZSC_MPCMWBCFGR is cleared)."]
        #[inline(always)]
        pub const fn subb_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in TZSC_MPCMWBCFGR is cleared)."]
        #[inline(always)]
        pub fn set_subb_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for TzscMpcwm4br {
        #[inline(always)]
        fn default() -> TzscMpcwm4br {
            TzscMpcwm4br(0)
        }
    }
    impl core::fmt::Debug for TzscMpcwm4br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscMpcwm4br")
                .field("subb_start", &self.subb_start())
                .field("subb_length", &self.subb_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscMpcwm4br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TzscMpcwm4br {{ subb_start: {=u16:?}, subb_length: {=u16:?} }}",
                self.subb_start(),
                self.subb_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscPrivcfgr1(pub u32);
    impl TzscPrivcfgr1 {
        #[doc = "privileged access mode for TIM2."]
        #[inline(always)]
        pub const fn tim2priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM2."]
        #[inline(always)]
        pub fn set_tim2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "privileged access mode for TIM3."]
        #[inline(always)]
        pub const fn tim3priv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM3."]
        #[inline(always)]
        pub fn set_tim3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "privileged access mode for TIM6."]
        #[inline(always)]
        pub const fn tim6priv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM6."]
        #[inline(always)]
        pub fn set_tim6priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "privileged access mode for TIM7."]
        #[inline(always)]
        pub const fn tim7priv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM7."]
        #[inline(always)]
        pub fn set_tim7priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "privileged access mode for WWDG."]
        #[inline(always)]
        pub const fn wwdgpriv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for WWDG."]
        #[inline(always)]
        pub fn set_wwdgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "privileged access mode for IWDG."]
        #[inline(always)]
        pub const fn iwdgpriv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for IWDG."]
        #[inline(always)]
        pub fn set_iwdgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "privileged access mode for SPI2."]
        #[inline(always)]
        pub const fn spi2priv(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI2."]
        #[inline(always)]
        pub fn set_spi2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "privileged access mode for SPI3."]
        #[inline(always)]
        pub const fn spi3priv(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI3."]
        #[inline(always)]
        pub fn set_spi3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "privileged access mode for USART2."]
        #[inline(always)]
        pub const fn usart2priv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART2."]
        #[inline(always)]
        pub fn set_usart2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "privileged access mode for USART3."]
        #[inline(always)]
        pub const fn usart3priv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART3."]
        #[inline(always)]
        pub fn set_usart3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "privileged access mode for I2C1."]
        #[inline(always)]
        pub const fn i2c1priv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I2C1."]
        #[inline(always)]
        pub fn set_i2c1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "privileged access mode for I2C2."]
        #[inline(always)]
        pub const fn i2c2priv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I2C2."]
        #[inline(always)]
        pub fn set_i2c2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "privileged access mode for I3C1."]
        #[inline(always)]
        pub const fn i3c1priv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I3C1."]
        #[inline(always)]
        pub fn set_i3c1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "privileged access mode for CRS."]
        #[inline(always)]
        pub const fn crspriv(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for CRS."]
        #[inline(always)]
        pub fn set_crspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "privileged access mode for DAC1."]
        #[inline(always)]
        pub const fn dac1priv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for DAC1."]
        #[inline(always)]
        pub fn set_dac1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "privileged access mode for DTS."]
        #[inline(always)]
        pub const fn dtspriv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for DTS."]
        #[inline(always)]
        pub fn set_dtspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "privileged access mode for LPTIM2."]
        #[inline(always)]
        pub const fn lptim2priv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPTIM2."]
        #[inline(always)]
        pub fn set_lptim2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TzscPrivcfgr1 {
        #[inline(always)]
        fn default() -> TzscPrivcfgr1 {
            TzscPrivcfgr1(0)
        }
    }
    impl core::fmt::Debug for TzscPrivcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscPrivcfgr1")
                .field("tim2priv", &self.tim2priv())
                .field("tim3priv", &self.tim3priv())
                .field("tim6priv", &self.tim6priv())
                .field("tim7priv", &self.tim7priv())
                .field("wwdgpriv", &self.wwdgpriv())
                .field("iwdgpriv", &self.iwdgpriv())
                .field("spi2priv", &self.spi2priv())
                .field("spi3priv", &self.spi3priv())
                .field("usart2priv", &self.usart2priv())
                .field("usart3priv", &self.usart3priv())
                .field("i2c1priv", &self.i2c1priv())
                .field("i2c2priv", &self.i2c2priv())
                .field("i3c1priv", &self.i3c1priv())
                .field("crspriv", &self.crspriv())
                .field("dac1priv", &self.dac1priv())
                .field("dtspriv", &self.dtspriv())
                .field("lptim2priv", &self.lptim2priv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscPrivcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscPrivcfgr1 {{ tim2priv: {=bool:?}, tim3priv: {=bool:?}, tim6priv: {=bool:?}, tim7priv: {=bool:?}, wwdgpriv: {=bool:?}, iwdgpriv: {=bool:?}, spi2priv: {=bool:?}, spi3priv: {=bool:?}, usart2priv: {=bool:?}, usart3priv: {=bool:?}, i2c1priv: {=bool:?}, i2c2priv: {=bool:?}, i3c1priv: {=bool:?}, crspriv: {=bool:?}, dac1priv: {=bool:?}, dtspriv: {=bool:?}, lptim2priv: {=bool:?} }}" , self . tim2priv () , self . tim3priv () , self . tim6priv () , self . tim7priv () , self . wwdgpriv () , self . iwdgpriv () , self . spi2priv () , self . spi3priv () , self . usart2priv () , self . usart3priv () , self . i2c1priv () , self . i2c2priv () , self . i3c1priv () , self . crspriv () , self . dac1priv () , self . dtspriv () , self . lptim2priv ())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscPrivcfgr2(pub u32);
    impl TzscPrivcfgr2 {
        #[doc = "privileged access mode for FDCAN1."]
        #[inline(always)]
        pub const fn fdcan1priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for FDCAN1."]
        #[inline(always)]
        pub fn set_fdcan1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "privileged access mode for OPAMP."]
        #[inline(always)]
        pub const fn opamppriv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for OPAMP."]
        #[inline(always)]
        pub fn set_opamppriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "privileged access mode for COMP."]
        #[inline(always)]
        pub const fn comppriv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for COMP."]
        #[inline(always)]
        pub fn set_comppriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "privileged access mode for TIM1."]
        #[inline(always)]
        pub const fn tim1priv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM1."]
        #[inline(always)]
        pub fn set_tim1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "privileged access mode for SPI1."]
        #[inline(always)]
        pub const fn spi1priv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI1."]
        #[inline(always)]
        pub fn set_spi1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "privileged access mode for USART1."]
        #[inline(always)]
        pub const fn usart1priv(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART1."]
        #[inline(always)]
        pub fn set_usart1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "privileged access mode for USBSF."]
        #[inline(always)]
        pub const fn usbfspriv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USBSF."]
        #[inline(always)]
        pub fn set_usbfspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "privileged access mode for LPUART."]
        #[inline(always)]
        pub const fn lpuart1priv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPUART."]
        #[inline(always)]
        pub fn set_lpuart1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "privileged access mode for LPTIM1."]
        #[inline(always)]
        pub const fn lptim1priv(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPTIM1."]
        #[inline(always)]
        pub fn set_lptim1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for TzscPrivcfgr2 {
        #[inline(always)]
        fn default() -> TzscPrivcfgr2 {
            TzscPrivcfgr2(0)
        }
    }
    impl core::fmt::Debug for TzscPrivcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscPrivcfgr2")
                .field("fdcan1priv", &self.fdcan1priv())
                .field("opamppriv", &self.opamppriv())
                .field("comppriv", &self.comppriv())
                .field("tim1priv", &self.tim1priv())
                .field("spi1priv", &self.spi1priv())
                .field("usart1priv", &self.usart1priv())
                .field("usbfspriv", &self.usbfspriv())
                .field("lpuart1priv", &self.lpuart1priv())
                .field("lptim1priv", &self.lptim1priv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscPrivcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscPrivcfgr2 {{ fdcan1priv: {=bool:?}, opamppriv: {=bool:?}, comppriv: {=bool:?}, tim1priv: {=bool:?}, spi1priv: {=bool:?}, usart1priv: {=bool:?}, usbfspriv: {=bool:?}, lpuart1priv: {=bool:?}, lptim1priv: {=bool:?} }}" , self . fdcan1priv () , self . opamppriv () , self . comppriv () , self . tim1priv () , self . spi1priv () , self . usart1priv () , self . usbfspriv () , self . lpuart1priv () , self . lptim1priv ())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscPrivcfgr3(pub u32);
    impl TzscPrivcfgr3 {
        #[doc = "privileged access mode for I3C2."]
        #[inline(always)]
        pub const fn i3c2priv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I3C2."]
        #[inline(always)]
        pub fn set_i3c2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "privileged access mode for CRC."]
        #[inline(always)]
        pub const fn crcpriv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for CRC."]
        #[inline(always)]
        pub fn set_crcpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "privileged access mode for ICACHE."]
        #[inline(always)]
        pub const fn icachepriv(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for ICACHE."]
        #[inline(always)]
        pub fn set_icachepriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "privileged access mode for ADC1."]
        #[inline(always)]
        pub const fn adc1priv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for ADC1."]
        #[inline(always)]
        pub fn set_adc1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "privileged access mode for HASH."]
        #[inline(always)]
        pub const fn hashpriv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for HASH."]
        #[inline(always)]
        pub fn set_hashpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "privileged access mode for RNG."]
        #[inline(always)]
        pub const fn rngpriv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for RNG."]
        #[inline(always)]
        pub fn set_rngpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "privileged access mode for RAMSCFG."]
        #[inline(always)]
        pub const fn ramcfgpriv(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for RAMSCFG."]
        #[inline(always)]
        pub fn set_ramcfgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for TzscPrivcfgr3 {
        #[inline(always)]
        fn default() -> TzscPrivcfgr3 {
            TzscPrivcfgr3(0)
        }
    }
    impl core::fmt::Debug for TzscPrivcfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscPrivcfgr3")
                .field("i3c2priv", &self.i3c2priv())
                .field("crcpriv", &self.crcpriv())
                .field("icachepriv", &self.icachepriv())
                .field("adc1priv", &self.adc1priv())
                .field("hashpriv", &self.hashpriv())
                .field("rngpriv", &self.rngpriv())
                .field("ramcfgpriv", &self.ramcfgpriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscPrivcfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscPrivcfgr3 {{ i3c2priv: {=bool:?}, crcpriv: {=bool:?}, icachepriv: {=bool:?}, adc1priv: {=bool:?}, hashpriv: {=bool:?}, rngpriv: {=bool:?}, ramcfgpriv: {=bool:?} }}" , self . i3c2priv () , self . crcpriv () , self . icachepriv () , self . adc1priv () , self . hashpriv () , self . rngpriv () , self . ramcfgpriv ())
        }
    }
}
