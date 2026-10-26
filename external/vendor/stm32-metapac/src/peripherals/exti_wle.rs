#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "External interrupt/event controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exti {
    ptr: *mut u8,
}
unsafe impl Send for Exti {}
unsafe impl Sync for Exti {}
impl Exti {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Rising Trigger selection register"]
    #[inline(always)]
    pub const fn rtsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Falling Trigger selection register"]
    #[inline(always)]
    pub const fn ftsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 32usize) as _) }
    }
    #[doc = "Software interrupt event register"]
    #[inline(always)]
    pub const fn swier(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 32usize) as _) }
    }
    #[doc = "Pending register"]
    #[inline(always)]
    pub const fn pr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 32usize) as _) }
    }
    #[doc = "Interrupt mask register"]
    #[inline(always)]
    pub const fn imr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 16usize) as _) }
    }
    #[doc = "Event mask register"]
    #[inline(always)]
    pub const fn emr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize + n * 16usize) as _) }
    }
}
pub mod regs {
    #[doc = "EXTI lines register, 1 bit per line"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lines(pub u32);
    impl Lines {
        #[doc = "EXTI line"]
        #[inline(always)]
        pub const fn line(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "EXTI line"]
        #[inline(always)]
        pub fn set_line(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Lines {
        #[inline(always)]
        fn default() -> Lines {
            Lines(0)
        }
    }
    impl core::fmt::Debug for Lines {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lines")
                .field("line[0]", &self.line(0usize))
                .field("line[1]", &self.line(1usize))
                .field("line[2]", &self.line(2usize))
                .field("line[3]", &self.line(3usize))
                .field("line[4]", &self.line(4usize))
                .field("line[5]", &self.line(5usize))
                .field("line[6]", &self.line(6usize))
                .field("line[7]", &self.line(7usize))
                .field("line[8]", &self.line(8usize))
                .field("line[9]", &self.line(9usize))
                .field("line[10]", &self.line(10usize))
                .field("line[11]", &self.line(11usize))
                .field("line[12]", &self.line(12usize))
                .field("line[13]", &self.line(13usize))
                .field("line[14]", &self.line(14usize))
                .field("line[15]", &self.line(15usize))
                .field("line[16]", &self.line(16usize))
                .field("line[17]", &self.line(17usize))
                .field("line[18]", &self.line(18usize))
                .field("line[19]", &self.line(19usize))
                .field("line[20]", &self.line(20usize))
                .field("line[21]", &self.line(21usize))
                .field("line[22]", &self.line(22usize))
                .field("line[23]", &self.line(23usize))
                .field("line[24]", &self.line(24usize))
                .field("line[25]", &self.line(25usize))
                .field("line[26]", &self.line(26usize))
                .field("line[27]", &self.line(27usize))
                .field("line[28]", &self.line(28usize))
                .field("line[29]", &self.line(29usize))
                .field("line[30]", &self.line(30usize))
                .field("line[31]", &self.line(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lines {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Lines {{ line[0]: {=bool:?}, line[1]: {=bool:?}, line[2]: {=bool:?}, line[3]: {=bool:?}, line[4]: {=bool:?}, line[5]: {=bool:?}, line[6]: {=bool:?}, line[7]: {=bool:?}, line[8]: {=bool:?}, line[9]: {=bool:?}, line[10]: {=bool:?}, line[11]: {=bool:?}, line[12]: {=bool:?}, line[13]: {=bool:?}, line[14]: {=bool:?}, line[15]: {=bool:?}, line[16]: {=bool:?}, line[17]: {=bool:?}, line[18]: {=bool:?}, line[19]: {=bool:?}, line[20]: {=bool:?}, line[21]: {=bool:?}, line[22]: {=bool:?}, line[23]: {=bool:?}, line[24]: {=bool:?}, line[25]: {=bool:?}, line[26]: {=bool:?}, line[27]: {=bool:?}, line[28]: {=bool:?}, line[29]: {=bool:?}, line[30]: {=bool:?}, line[31]: {=bool:?} }}" , self . line (0usize) , self . line (1usize) , self . line (2usize) , self . line (3usize) , self . line (4usize) , self . line (5usize) , self . line (6usize) , self . line (7usize) , self . line (8usize) , self . line (9usize) , self . line (10usize) , self . line (11usize) , self . line (12usize) , self . line (13usize) , self . line (14usize) , self . line (15usize) , self . line (16usize) , self . line (17usize) , self . line (18usize) , self . line (19usize) , self . line (20usize) , self . line (21usize) , self . line (22usize) , self . line (23usize) , self . line (24usize) , self . line (25usize) , self . line (26usize) , self . line (27usize) , self . line (28usize) , self . line (29usize) , self . line (30usize) , self . line (31usize))
        }
    }
}
