#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Hardware semaphore (HSEM)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsem {
    ptr: *mut u8,
}
unsafe impl Send for Hsem {}
unsafe impl Sync for Hsem {}
impl Hsem {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "HSEM register HSEM_R%s HSEM_R31."]
    #[inline(always)]
    pub const fn r(self, n: usize) -> crate::common::Reg<regs::R, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "HSEM Read lock register."]
    #[inline(always)]
    pub const fn rlr(self, n: usize) -> crate::common::Reg<regs::Rlr, crate::common::R> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "HSEM Interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self, n: usize) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 16usize) as _) }
    }
    #[doc = "HSEM Interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self, n: usize) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 16usize) as _) }
    }
    #[doc = "HSEM Interrupt status register."]
    #[inline(always)]
    pub const fn isr(self, n: usize) -> crate::common::Reg<regs::Isr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize + n * 16usize) as _) }
    }
    #[doc = "HSEM Masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self, n: usize) -> crate::common::Reg<regs::Misr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize + n * 16usize) as _) }
    }
    #[doc = "HSEM Clear register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "HSEM Interrupt clear register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
}
pub mod regs {
    #[doc = "HSEM Clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "COREID of semaphores to be cleared."]
        #[inline(always)]
        pub const fn coreid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "COREID of semaphores to be cleared."]
        #[inline(always)]
        pub fn set_coreid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Semaphore clear Key."]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Semaphore clear Key."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("coreid", &self.coreid())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr {{ coreid: {=u8:?}, key: {=u16:?} }}", self.coreid(), self.key())
        }
    }
    #[doc = "HSEM Interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Interrupt semaphore x clear bit."]
        #[inline(always)]
        pub const fn isc(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore x clear bit."]
        #[inline(always)]
        pub fn set_isc(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("isc[0]", &self.isc(0usize))
                .field("isc[1]", &self.isc(1usize))
                .field("isc[2]", &self.isc(2usize))
                .field("isc[3]", &self.isc(3usize))
                .field("isc[4]", &self.isc(4usize))
                .field("isc[5]", &self.isc(5usize))
                .field("isc[6]", &self.isc(6usize))
                .field("isc[7]", &self.isc(7usize))
                .field("isc[8]", &self.isc(8usize))
                .field("isc[9]", &self.isc(9usize))
                .field("isc[10]", &self.isc(10usize))
                .field("isc[11]", &self.isc(11usize))
                .field("isc[12]", &self.isc(12usize))
                .field("isc[13]", &self.isc(13usize))
                .field("isc[14]", &self.isc(14usize))
                .field("isc[15]", &self.isc(15usize))
                .field("isc[16]", &self.isc(16usize))
                .field("isc[17]", &self.isc(17usize))
                .field("isc[18]", &self.isc(18usize))
                .field("isc[19]", &self.isc(19usize))
                .field("isc[20]", &self.isc(20usize))
                .field("isc[21]", &self.isc(21usize))
                .field("isc[22]", &self.isc(22usize))
                .field("isc[23]", &self.isc(23usize))
                .field("isc[24]", &self.isc(24usize))
                .field("isc[25]", &self.isc(25usize))
                .field("isc[26]", &self.isc(26usize))
                .field("isc[27]", &self.isc(27usize))
                .field("isc[28]", &self.isc(28usize))
                .field("isc[29]", &self.isc(29usize))
                .field("isc[30]", &self.isc(30usize))
                .field("isc[31]", &self.isc(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ isc[0]: {=bool:?}, isc[1]: {=bool:?}, isc[2]: {=bool:?}, isc[3]: {=bool:?}, isc[4]: {=bool:?}, isc[5]: {=bool:?}, isc[6]: {=bool:?}, isc[7]: {=bool:?}, isc[8]: {=bool:?}, isc[9]: {=bool:?}, isc[10]: {=bool:?}, isc[11]: {=bool:?}, isc[12]: {=bool:?}, isc[13]: {=bool:?}, isc[14]: {=bool:?}, isc[15]: {=bool:?}, isc[16]: {=bool:?}, isc[17]: {=bool:?}, isc[18]: {=bool:?}, isc[19]: {=bool:?}, isc[20]: {=bool:?}, isc[21]: {=bool:?}, isc[22]: {=bool:?}, isc[23]: {=bool:?}, isc[24]: {=bool:?}, isc[25]: {=bool:?}, isc[26]: {=bool:?}, isc[27]: {=bool:?}, isc[28]: {=bool:?}, isc[29]: {=bool:?}, isc[30]: {=bool:?}, isc[31]: {=bool:?} }}" , self . isc (0usize) , self . isc (1usize) , self . isc (2usize) , self . isc (3usize) , self . isc (4usize) , self . isc (5usize) , self . isc (6usize) , self . isc (7usize) , self . isc (8usize) , self . isc (9usize) , self . isc (10usize) , self . isc (11usize) , self . isc (12usize) , self . isc (13usize) , self . isc (14usize) , self . isc (15usize) , self . isc (16usize) , self . isc (17usize) , self . isc (18usize) , self . isc (19usize) , self . isc (20usize) , self . isc (21usize) , self . isc (22usize) , self . isc (23usize) , self . isc (24usize) , self . isc (25usize) , self . isc (26usize) , self . isc (27usize) , self . isc (28usize) , self . isc (29usize) , self . isc (30usize) , self . isc (31usize))
        }
    }
    #[doc = "HSEM Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Interrupt semaphore x enable bit."]
        #[inline(always)]
        pub const fn ise(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore x enable bit."]
        #[inline(always)]
        pub fn set_ise(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("ise[0]", &self.ise(0usize))
                .field("ise[1]", &self.ise(1usize))
                .field("ise[2]", &self.ise(2usize))
                .field("ise[3]", &self.ise(3usize))
                .field("ise[4]", &self.ise(4usize))
                .field("ise[5]", &self.ise(5usize))
                .field("ise[6]", &self.ise(6usize))
                .field("ise[7]", &self.ise(7usize))
                .field("ise[8]", &self.ise(8usize))
                .field("ise[9]", &self.ise(9usize))
                .field("ise[10]", &self.ise(10usize))
                .field("ise[11]", &self.ise(11usize))
                .field("ise[12]", &self.ise(12usize))
                .field("ise[13]", &self.ise(13usize))
                .field("ise[14]", &self.ise(14usize))
                .field("ise[15]", &self.ise(15usize))
                .field("ise[16]", &self.ise(16usize))
                .field("ise[17]", &self.ise(17usize))
                .field("ise[18]", &self.ise(18usize))
                .field("ise[19]", &self.ise(19usize))
                .field("ise[20]", &self.ise(20usize))
                .field("ise[21]", &self.ise(21usize))
                .field("ise[22]", &self.ise(22usize))
                .field("ise[23]", &self.ise(23usize))
                .field("ise[24]", &self.ise(24usize))
                .field("ise[25]", &self.ise(25usize))
                .field("ise[26]", &self.ise(26usize))
                .field("ise[27]", &self.ise(27usize))
                .field("ise[28]", &self.ise(28usize))
                .field("ise[29]", &self.ise(29usize))
                .field("ise[30]", &self.ise(30usize))
                .field("ise[31]", &self.ise(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ ise[0]: {=bool:?}, ise[1]: {=bool:?}, ise[2]: {=bool:?}, ise[3]: {=bool:?}, ise[4]: {=bool:?}, ise[5]: {=bool:?}, ise[6]: {=bool:?}, ise[7]: {=bool:?}, ise[8]: {=bool:?}, ise[9]: {=bool:?}, ise[10]: {=bool:?}, ise[11]: {=bool:?}, ise[12]: {=bool:?}, ise[13]: {=bool:?}, ise[14]: {=bool:?}, ise[15]: {=bool:?}, ise[16]: {=bool:?}, ise[17]: {=bool:?}, ise[18]: {=bool:?}, ise[19]: {=bool:?}, ise[20]: {=bool:?}, ise[21]: {=bool:?}, ise[22]: {=bool:?}, ise[23]: {=bool:?}, ise[24]: {=bool:?}, ise[25]: {=bool:?}, ise[26]: {=bool:?}, ise[27]: {=bool:?}, ise[28]: {=bool:?}, ise[29]: {=bool:?}, ise[30]: {=bool:?}, ise[31]: {=bool:?} }}" , self . ise (0usize) , self . ise (1usize) , self . ise (2usize) , self . ise (3usize) , self . ise (4usize) , self . ise (5usize) , self . ise (6usize) , self . ise (7usize) , self . ise (8usize) , self . ise (9usize) , self . ise (10usize) , self . ise (11usize) , self . ise (12usize) , self . ise (13usize) , self . ise (14usize) , self . ise (15usize) , self . ise (16usize) , self . ise (17usize) , self . ise (18usize) , self . ise (19usize) , self . ise (20usize) , self . ise (21usize) , self . ise (22usize) , self . ise (23usize) , self . ise (24usize) , self . ise (25usize) , self . ise (26usize) , self . ise (27usize) , self . ise (28usize) , self . ise (29usize) , self . ise (30usize) , self . ise (31usize))
        }
    }
    #[doc = "HSEM Interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Interrupt semaphore x status bit before enable (mask)."]
        #[inline(always)]
        pub const fn isf(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore x status bit before enable (mask)."]
        #[inline(always)]
        pub fn set_isf(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("isf[0]", &self.isf(0usize))
                .field("isf[1]", &self.isf(1usize))
                .field("isf[2]", &self.isf(2usize))
                .field("isf[3]", &self.isf(3usize))
                .field("isf[4]", &self.isf(4usize))
                .field("isf[5]", &self.isf(5usize))
                .field("isf[6]", &self.isf(6usize))
                .field("isf[7]", &self.isf(7usize))
                .field("isf[8]", &self.isf(8usize))
                .field("isf[9]", &self.isf(9usize))
                .field("isf[10]", &self.isf(10usize))
                .field("isf[11]", &self.isf(11usize))
                .field("isf[12]", &self.isf(12usize))
                .field("isf[13]", &self.isf(13usize))
                .field("isf[14]", &self.isf(14usize))
                .field("isf[15]", &self.isf(15usize))
                .field("isf[16]", &self.isf(16usize))
                .field("isf[17]", &self.isf(17usize))
                .field("isf[18]", &self.isf(18usize))
                .field("isf[19]", &self.isf(19usize))
                .field("isf[20]", &self.isf(20usize))
                .field("isf[21]", &self.isf(21usize))
                .field("isf[22]", &self.isf(22usize))
                .field("isf[23]", &self.isf(23usize))
                .field("isf[24]", &self.isf(24usize))
                .field("isf[25]", &self.isf(25usize))
                .field("isf[26]", &self.isf(26usize))
                .field("isf[27]", &self.isf(27usize))
                .field("isf[28]", &self.isf(28usize))
                .field("isf[29]", &self.isf(29usize))
                .field("isf[30]", &self.isf(30usize))
                .field("isf[31]", &self.isf(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ isf[0]: {=bool:?}, isf[1]: {=bool:?}, isf[2]: {=bool:?}, isf[3]: {=bool:?}, isf[4]: {=bool:?}, isf[5]: {=bool:?}, isf[6]: {=bool:?}, isf[7]: {=bool:?}, isf[8]: {=bool:?}, isf[9]: {=bool:?}, isf[10]: {=bool:?}, isf[11]: {=bool:?}, isf[12]: {=bool:?}, isf[13]: {=bool:?}, isf[14]: {=bool:?}, isf[15]: {=bool:?}, isf[16]: {=bool:?}, isf[17]: {=bool:?}, isf[18]: {=bool:?}, isf[19]: {=bool:?}, isf[20]: {=bool:?}, isf[21]: {=bool:?}, isf[22]: {=bool:?}, isf[23]: {=bool:?}, isf[24]: {=bool:?}, isf[25]: {=bool:?}, isf[26]: {=bool:?}, isf[27]: {=bool:?}, isf[28]: {=bool:?}, isf[29]: {=bool:?}, isf[30]: {=bool:?}, isf[31]: {=bool:?} }}" , self . isf (0usize) , self . isf (1usize) , self . isf (2usize) , self . isf (3usize) , self . isf (4usize) , self . isf (5usize) , self . isf (6usize) , self . isf (7usize) , self . isf (8usize) , self . isf (9usize) , self . isf (10usize) , self . isf (11usize) , self . isf (12usize) , self . isf (13usize) , self . isf (14usize) , self . isf (15usize) , self . isf (16usize) , self . isf (17usize) , self . isf (18usize) , self . isf (19usize) , self . isf (20usize) , self . isf (21usize) , self . isf (22usize) , self . isf (23usize) , self . isf (24usize) , self . isf (25usize) , self . isf (26usize) , self . isf (27usize) , self . isf (28usize) , self . isf (29usize) , self . isf (30usize) , self . isf (31usize))
        }
    }
    #[doc = "HSEM Interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "Semaphore Clear Key."]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Semaphore Clear Key."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    impl core::fmt::Debug for Keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Keyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Keyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Keyr {{ key: {=u16:?} }}", self.key())
        }
    }
    #[doc = "HSEM Masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "masked interrupt semaphore x status bit after enable (mask)."]
        #[inline(always)]
        pub const fn misf(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "masked interrupt semaphore x status bit after enable (mask)."]
        #[inline(always)]
        pub fn set_misf(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("misf[0]", &self.misf(0usize))
                .field("misf[1]", &self.misf(1usize))
                .field("misf[2]", &self.misf(2usize))
                .field("misf[3]", &self.misf(3usize))
                .field("misf[4]", &self.misf(4usize))
                .field("misf[5]", &self.misf(5usize))
                .field("misf[6]", &self.misf(6usize))
                .field("misf[7]", &self.misf(7usize))
                .field("misf[8]", &self.misf(8usize))
                .field("misf[9]", &self.misf(9usize))
                .field("misf[10]", &self.misf(10usize))
                .field("misf[11]", &self.misf(11usize))
                .field("misf[12]", &self.misf(12usize))
                .field("misf[13]", &self.misf(13usize))
                .field("misf[14]", &self.misf(14usize))
                .field("misf[15]", &self.misf(15usize))
                .field("misf[16]", &self.misf(16usize))
                .field("misf[17]", &self.misf(17usize))
                .field("misf[18]", &self.misf(18usize))
                .field("misf[19]", &self.misf(19usize))
                .field("misf[20]", &self.misf(20usize))
                .field("misf[21]", &self.misf(21usize))
                .field("misf[22]", &self.misf(22usize))
                .field("misf[23]", &self.misf(23usize))
                .field("misf[24]", &self.misf(24usize))
                .field("misf[25]", &self.misf(25usize))
                .field("misf[26]", &self.misf(26usize))
                .field("misf[27]", &self.misf(27usize))
                .field("misf[28]", &self.misf(28usize))
                .field("misf[29]", &self.misf(29usize))
                .field("misf[30]", &self.misf(30usize))
                .field("misf[31]", &self.misf(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Misr {{ misf[0]: {=bool:?}, misf[1]: {=bool:?}, misf[2]: {=bool:?}, misf[3]: {=bool:?}, misf[4]: {=bool:?}, misf[5]: {=bool:?}, misf[6]: {=bool:?}, misf[7]: {=bool:?}, misf[8]: {=bool:?}, misf[9]: {=bool:?}, misf[10]: {=bool:?}, misf[11]: {=bool:?}, misf[12]: {=bool:?}, misf[13]: {=bool:?}, misf[14]: {=bool:?}, misf[15]: {=bool:?}, misf[16]: {=bool:?}, misf[17]: {=bool:?}, misf[18]: {=bool:?}, misf[19]: {=bool:?}, misf[20]: {=bool:?}, misf[21]: {=bool:?}, misf[22]: {=bool:?}, misf[23]: {=bool:?}, misf[24]: {=bool:?}, misf[25]: {=bool:?}, misf[26]: {=bool:?}, misf[27]: {=bool:?}, misf[28]: {=bool:?}, misf[29]: {=bool:?}, misf[30]: {=bool:?}, misf[31]: {=bool:?} }}" , self . misf (0usize) , self . misf (1usize) , self . misf (2usize) , self . misf (3usize) , self . misf (4usize) , self . misf (5usize) , self . misf (6usize) , self . misf (7usize) , self . misf (8usize) , self . misf (9usize) , self . misf (10usize) , self . misf (11usize) , self . misf (12usize) , self . misf (13usize) , self . misf (14usize) , self . misf (15usize) , self . misf (16usize) , self . misf (17usize) , self . misf (18usize) , self . misf (19usize) , self . misf (20usize) , self . misf (21usize) , self . misf (22usize) , self . misf (23usize) , self . misf (24usize) , self . misf (25usize) , self . misf (26usize) , self . misf (27usize) , self . misf (28usize) , self . misf (29usize) , self . misf (30usize) , self . misf (31usize))
        }
    }
    #[doc = "HSEM register HSEM_R%s HSEM_R31."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R(pub u32);
    impl R {
        #[doc = "Semaphore ProcessID."]
        #[inline(always)]
        pub const fn procid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Semaphore ProcessID."]
        #[inline(always)]
        pub fn set_procid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Semaphore COREID."]
        #[inline(always)]
        pub const fn coreid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Semaphore COREID."]
        #[inline(always)]
        pub fn set_coreid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Lock indication."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock indication."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for R {
        #[inline(always)]
        fn default() -> R {
            R(0)
        }
    }
    impl core::fmt::Debug for R {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R")
                .field("procid", &self.procid())
                .field("coreid", &self.coreid())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R {{ procid: {=u8:?}, coreid: {=u8:?}, lock: {=bool:?} }}",
                self.procid(),
                self.coreid(),
                self.lock()
            )
        }
    }
    #[doc = "HSEM Read lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rlr(pub u32);
    impl Rlr {
        #[doc = "Semaphore ProcessID."]
        #[inline(always)]
        pub const fn procid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Semaphore ProcessID."]
        #[inline(always)]
        pub fn set_procid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Semaphore COREID."]
        #[inline(always)]
        pub const fn coreid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Semaphore COREID."]
        #[inline(always)]
        pub fn set_coreid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Lock indication."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock indication."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rlr {
        #[inline(always)]
        fn default() -> Rlr {
            Rlr(0)
        }
    }
    impl core::fmt::Debug for Rlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rlr")
                .field("procid", &self.procid())
                .field("coreid", &self.coreid())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rlr {{ procid: {=u8:?}, coreid: {=u8:?}, lock: {=bool:?} }}",
                self.procid(),
                self.coreid(),
                self.lock()
            )
        }
    }
}
