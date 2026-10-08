#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Instruction Cache Control Registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icache {
    ptr: *mut u8,
}
unsafe impl Send for Icache {}
unsafe impl Sync for Icache {}
impl Icache {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ICACHE control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ICACHE status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ICACHE interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ICACHE flag clear register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ICACHE hit monitor register."]
    #[inline(always)]
    pub const fn hmonr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "ICACHE miss monitor register."]
    #[inline(always)]
    pub const fn mmonr(self) -> crate::common::Reg<regs::Mmonr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "ICACHE control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "EN."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EN."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect."]
        #[inline(always)]
        pub const fn cacheinv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_cacheinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
        #[inline(always)]
        pub const fn waysel(&self) -> super::vals::Waysel {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Waysel::from_bits(val as u8)
        }
        #[doc = "This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
        #[inline(always)]
        pub fn set_waysel(&mut self, val: super::vals::Waysel) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Hit monitor enable."]
        #[inline(always)]
        pub const fn hitmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Hit monitor enable."]
        #[inline(always)]
        pub fn set_hitmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Miss monitor enable."]
        #[inline(always)]
        pub const fn missmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Miss monitor enable."]
        #[inline(always)]
        pub fn set_missmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Hit monitor reset."]
        #[inline(always)]
        pub const fn hitmrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Hit monitor reset."]
        #[inline(always)]
        pub fn set_hitmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Miss monitor reset."]
        #[inline(always)]
        pub const fn missmrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Miss monitor reset."]
        #[inline(always)]
        pub fn set_missmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("en", &self.en())
                .field("cacheinv", &self.cacheinv())
                .field("waysel", &self.waysel())
                .field("hitmen", &self.hitmen())
                .field("missmen", &self.missmen())
                .field("hitmrst", &self.hitmrst())
                .field("missmrst", &self.missmrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, cacheinv: {=bool:?}, waysel: {:?}, hitmen: {=bool:?}, missmen: {=bool:?}, hitmrst: {=bool:?}, missmrst: {=bool:?} }}" , self . en () , self . cacheinv () , self . waysel () , self . hitmen () , self . missmen () , self . hitmrst () , self . missmrst ())
        }
    }
    #[doc = "ICACHE flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "Clear busy end flag."]
        #[inline(always)]
        pub const fn cbsyendf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear busy end flag."]
        #[inline(always)]
        pub fn set_cbsyendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear ERRF flag in SR."]
        #[inline(always)]
        pub const fn cerrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ERRF flag in SR."]
        #[inline(always)]
        pub fn set_cerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("cbsyendf", &self.cbsyendf())
                .field("cerrf", &self.cerrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fcr {{ cbsyendf: {=bool:?}, cerrf: {=bool:?} }}",
                self.cbsyendf(),
                self.cerrf()
            )
        }
    }
    #[doc = "ICACHE interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Interrupt enable on busy end."]
        #[inline(always)]
        pub const fn bsyendie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable on busy end."]
        #[inline(always)]
        pub fn set_bsyendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Error interrupt on cache error."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt on cache error."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("bsyendie", &self.bsyendie())
                .field("errie", &self.errie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ bsyendie: {=bool:?}, errie: {=bool:?} }}",
                self.bsyendie(),
                self.errie()
            )
        }
    }
    #[doc = "ICACHE miss monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmonr(pub u32);
    impl Mmonr {
        #[doc = "Miss monitor register."]
        #[inline(always)]
        pub const fn missmon(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Miss monitor register."]
        #[inline(always)]
        pub fn set_missmon(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mmonr {
        #[inline(always)]
        fn default() -> Mmonr {
            Mmonr(0)
        }
    }
    impl core::fmt::Debug for Mmonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmonr").field("missmon", &self.missmon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mmonr {{ missmon: {=u16:?} }}", self.missmon())
        }
    }
    #[doc = "ICACHE status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "cache busy executing a full invalidate CACHEINV operation."]
        #[inline(always)]
        pub const fn busyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "cache busy executing a full invalidate CACHEINV operation."]
        #[inline(always)]
        pub fn set_busyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "full invalidate CACHEINV operation finished."]
        #[inline(always)]
        pub const fn bsyendf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "full invalidate CACHEINV operation finished."]
        #[inline(always)]
        pub fn set_bsyendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "an error occurred during the operation."]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "an error occurred during the operation."]
        #[inline(always)]
        pub fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("busyf", &self.busyf())
                .field("bsyendf", &self.bsyendf())
                .field("errf", &self.errf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ busyf: {=bool:?}, bsyendf: {=bool:?}, errf: {=bool:?} }}",
                self.busyf(),
                self.bsyendf(),
                self.errf()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Waysel {
        #[doc = "direct mapped cache (1-way cache)"]
        DIRECT_MAPPED = 0x0,
        #[doc = "n-way set associative cache (reset value)"]
        NWAY_SET_ASSOCIATIVE = 0x01,
    }
    impl Waysel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waysel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waysel {
        #[inline(always)]
        fn from(val: u8) -> Waysel {
            Waysel::from_bits(val)
        }
    }
    impl From<Waysel> for u8 {
        #[inline(always)]
        fn from(val: Waysel) -> u8 {
            Waysel::to_bits(val)
        }
    }
}
