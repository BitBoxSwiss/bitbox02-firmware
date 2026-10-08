#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Extended interrupts and event controller."]
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
    #[doc = "EXTI rising trigger selection register."]
    #[inline(always)]
    pub const fn rtsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "EXTI falling trigger selection register."]
    #[inline(always)]
    pub const fn ftsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 32usize) as _) }
    }
    #[doc = "EXTI software interrupt event register."]
    #[inline(always)]
    pub const fn swier(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 32usize) as _) }
    }
    #[doc = "EXTI rising edge pending register."]
    #[inline(always)]
    pub const fn rpr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 32usize) as _) }
    }
    #[doc = "EXTI falling edge pending register."]
    #[inline(always)]
    pub const fn fpr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 32usize) as _) }
    }
    #[doc = "EXTI security configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self, n: usize) -> crate::common::Reg<regs::Sec, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 32usize) as _) }
    }
    #[doc = "EXTI privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self, n: usize) -> crate::common::Reg<regs::Priv, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 32usize) as _) }
    }
    #[doc = "EXTI external interrupt selection register 1."]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exti, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "EXTI lock register."]
    #[inline(always)]
    pub const fn lockr(self) -> crate::common::Reg<regs::Lockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "EXTI CPU wake-up with interrupt mask register 1."]
    #[inline(always)]
    pub const fn imr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 16usize) as _) }
    }
    #[doc = "EXTI CPU wake-up with event mask register 1."]
    #[inline(always)]
    pub const fn emr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize + n * 16usize) as _) }
    }
}
pub mod regs {
    #[doc = "EXTI external interrupt selection register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exti(pub u32);
    impl Exti {
        #[doc = "EXTI GPIO port selection."]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "EXTI GPIO port selection."]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Exti {
        #[inline(always)]
        fn default() -> Exti {
            Exti(0)
        }
    }
    impl core::fmt::Debug for Exti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exti")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exti {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
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
    #[doc = "EXTI lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lockr(pub u32);
    impl Lockr {
        #[doc = "Global security privilege SECCFGRx/PRIVCFGRx."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global security privilege SECCFGRx/PRIVCFGRx."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Lockr {
        #[inline(always)]
        fn default() -> Lockr {
            Lockr(0)
        }
    }
    impl core::fmt::Debug for Lockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lockr").field("lock", &self.lock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lockr {{ lock: {=bool:?} }}", self.lock())
        }
    }
    #[doc = "EXTI privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv(pub u32);
    impl Priv {
        #[doc = "Privilege enable on event input x."]
        #[inline(always)]
        pub const fn priv_(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Privilege enable on event input x."]
        #[inline(always)]
        pub fn set_priv_(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Priv {
        #[inline(always)]
        fn default() -> Priv {
            Priv(0)
        }
    }
    impl core::fmt::Debug for Priv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv")
                .field("priv_[0]", &self.priv_(0usize))
                .field("priv_[1]", &self.priv_(1usize))
                .field("priv_[2]", &self.priv_(2usize))
                .field("priv_[3]", &self.priv_(3usize))
                .field("priv_[4]", &self.priv_(4usize))
                .field("priv_[5]", &self.priv_(5usize))
                .field("priv_[6]", &self.priv_(6usize))
                .field("priv_[7]", &self.priv_(7usize))
                .field("priv_[8]", &self.priv_(8usize))
                .field("priv_[9]", &self.priv_(9usize))
                .field("priv_[10]", &self.priv_(10usize))
                .field("priv_[11]", &self.priv_(11usize))
                .field("priv_[12]", &self.priv_(12usize))
                .field("priv_[13]", &self.priv_(13usize))
                .field("priv_[14]", &self.priv_(14usize))
                .field("priv_[15]", &self.priv_(15usize))
                .field("priv_[16]", &self.priv_(16usize))
                .field("priv_[17]", &self.priv_(17usize))
                .field("priv_[18]", &self.priv_(18usize))
                .field("priv_[19]", &self.priv_(19usize))
                .field("priv_[20]", &self.priv_(20usize))
                .field("priv_[21]", &self.priv_(21usize))
                .field("priv_[22]", &self.priv_(22usize))
                .field("priv_[23]", &self.priv_(23usize))
                .field("priv_[24]", &self.priv_(24usize))
                .field("priv_[25]", &self.priv_(25usize))
                .field("priv_[26]", &self.priv_(26usize))
                .field("priv_[27]", &self.priv_(27usize))
                .field("priv_[28]", &self.priv_(28usize))
                .field("priv_[29]", &self.priv_(29usize))
                .field("priv_[30]", &self.priv_(30usize))
                .field("priv_[31]", &self.priv_(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Priv {{ priv_[0]: {=bool:?}, priv_[1]: {=bool:?}, priv_[2]: {=bool:?}, priv_[3]: {=bool:?}, priv_[4]: {=bool:?}, priv_[5]: {=bool:?}, priv_[6]: {=bool:?}, priv_[7]: {=bool:?}, priv_[8]: {=bool:?}, priv_[9]: {=bool:?}, priv_[10]: {=bool:?}, priv_[11]: {=bool:?}, priv_[12]: {=bool:?}, priv_[13]: {=bool:?}, priv_[14]: {=bool:?}, priv_[15]: {=bool:?}, priv_[16]: {=bool:?}, priv_[17]: {=bool:?}, priv_[18]: {=bool:?}, priv_[19]: {=bool:?}, priv_[20]: {=bool:?}, priv_[21]: {=bool:?}, priv_[22]: {=bool:?}, priv_[23]: {=bool:?}, priv_[24]: {=bool:?}, priv_[25]: {=bool:?}, priv_[26]: {=bool:?}, priv_[27]: {=bool:?}, priv_[28]: {=bool:?}, priv_[29]: {=bool:?}, priv_[30]: {=bool:?}, priv_[31]: {=bool:?} }}" , self . priv_ (0usize) , self . priv_ (1usize) , self . priv_ (2usize) , self . priv_ (3usize) , self . priv_ (4usize) , self . priv_ (5usize) , self . priv_ (6usize) , self . priv_ (7usize) , self . priv_ (8usize) , self . priv_ (9usize) , self . priv_ (10usize) , self . priv_ (11usize) , self . priv_ (12usize) , self . priv_ (13usize) , self . priv_ (14usize) , self . priv_ (15usize) , self . priv_ (16usize) , self . priv_ (17usize) , self . priv_ (18usize) , self . priv_ (19usize) , self . priv_ (20usize) , self . priv_ (21usize) , self . priv_ (22usize) , self . priv_ (23usize) , self . priv_ (24usize) , self . priv_ (25usize) , self . priv_ (26usize) , self . priv_ (27usize) , self . priv_ (28usize) , self . priv_ (29usize) , self . priv_ (30usize) , self . priv_ (31usize))
        }
    }
    #[doc = "EXTI security configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec(pub u32);
    impl Sec {
        #[doc = "Security enable on event input x."]
        #[inline(always)]
        pub const fn sec(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Security enable on event input x."]
        #[inline(always)]
        pub fn set_sec(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sec {
        #[inline(always)]
        fn default() -> Sec {
            Sec(0)
        }
    }
    impl core::fmt::Debug for Sec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec")
                .field("sec[0]", &self.sec(0usize))
                .field("sec[1]", &self.sec(1usize))
                .field("sec[2]", &self.sec(2usize))
                .field("sec[3]", &self.sec(3usize))
                .field("sec[4]", &self.sec(4usize))
                .field("sec[5]", &self.sec(5usize))
                .field("sec[6]", &self.sec(6usize))
                .field("sec[7]", &self.sec(7usize))
                .field("sec[8]", &self.sec(8usize))
                .field("sec[9]", &self.sec(9usize))
                .field("sec[10]", &self.sec(10usize))
                .field("sec[11]", &self.sec(11usize))
                .field("sec[12]", &self.sec(12usize))
                .field("sec[13]", &self.sec(13usize))
                .field("sec[14]", &self.sec(14usize))
                .field("sec[15]", &self.sec(15usize))
                .field("sec[16]", &self.sec(16usize))
                .field("sec[17]", &self.sec(17usize))
                .field("sec[18]", &self.sec(18usize))
                .field("sec[19]", &self.sec(19usize))
                .field("sec[20]", &self.sec(20usize))
                .field("sec[21]", &self.sec(21usize))
                .field("sec[22]", &self.sec(22usize))
                .field("sec[23]", &self.sec(23usize))
                .field("sec[24]", &self.sec(24usize))
                .field("sec[25]", &self.sec(25usize))
                .field("sec[26]", &self.sec(26usize))
                .field("sec[27]", &self.sec(27usize))
                .field("sec[28]", &self.sec(28usize))
                .field("sec[29]", &self.sec(29usize))
                .field("sec[30]", &self.sec(30usize))
                .field("sec[31]", &self.sec(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sec {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sec {{ sec[0]: {=bool:?}, sec[1]: {=bool:?}, sec[2]: {=bool:?}, sec[3]: {=bool:?}, sec[4]: {=bool:?}, sec[5]: {=bool:?}, sec[6]: {=bool:?}, sec[7]: {=bool:?}, sec[8]: {=bool:?}, sec[9]: {=bool:?}, sec[10]: {=bool:?}, sec[11]: {=bool:?}, sec[12]: {=bool:?}, sec[13]: {=bool:?}, sec[14]: {=bool:?}, sec[15]: {=bool:?}, sec[16]: {=bool:?}, sec[17]: {=bool:?}, sec[18]: {=bool:?}, sec[19]: {=bool:?}, sec[20]: {=bool:?}, sec[21]: {=bool:?}, sec[22]: {=bool:?}, sec[23]: {=bool:?}, sec[24]: {=bool:?}, sec[25]: {=bool:?}, sec[26]: {=bool:?}, sec[27]: {=bool:?}, sec[28]: {=bool:?}, sec[29]: {=bool:?}, sec[30]: {=bool:?}, sec[31]: {=bool:?} }}" , self . sec (0usize) , self . sec (1usize) , self . sec (2usize) , self . sec (3usize) , self . sec (4usize) , self . sec (5usize) , self . sec (6usize) , self . sec (7usize) , self . sec (8usize) , self . sec (9usize) , self . sec (10usize) , self . sec (11usize) , self . sec (12usize) , self . sec (13usize) , self . sec (14usize) , self . sec (15usize) , self . sec (16usize) , self . sec (17usize) , self . sec (18usize) , self . sec (19usize) , self . sec (20usize) , self . sec (21usize) , self . sec (22usize) , self . sec (23usize) , self . sec (24usize) , self . sec (25usize) , self . sec (26usize) , self . sec (27usize) , self . sec (28usize) , self . sec (29usize) , self . sec (30usize) , self . sec (31usize))
        }
    }
}
