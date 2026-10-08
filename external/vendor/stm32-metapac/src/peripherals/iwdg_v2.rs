#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Independent watchdog"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdg {
    ptr: *mut u8,
}
unsafe impl Send for Iwdg {}
unsafe impl Sync for Iwdg {}
impl Iwdg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Key register"]
    #[inline(always)]
    pub const fn kr(self) -> crate::common::Reg<regs::Kr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Prescaler register"]
    #[inline(always)]
    pub const fn pr(self) -> crate::common::Reg<regs::Pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Reload register"]
    #[inline(always)]
    pub const fn rlr(self) -> crate::common::Reg<regs::Rlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Window register"]
    #[inline(always)]
    pub const fn winr(self) -> crate::common::Reg<regs::Winr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Kr(pub u32);
    impl Kr {
        #[doc = "Key value (write only, read 0000h)"]
        #[inline(always)]
        pub const fn key(&self) -> super::vals::Key {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Key::from_bits(val as u16)
        }
        #[doc = "Key value (write only, read 0000h)"]
        #[inline(always)]
        pub fn set_key(&mut self, val: super::vals::Key) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Kr {
        #[inline(always)]
        fn default() -> Kr {
            Kr(0)
        }
    }
    impl core::fmt::Debug for Kr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Kr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Kr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Kr {{ key: {:?} }}", self.key())
        }
    }
    #[doc = "Prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pr(pub u32);
    impl Pr {
        #[doc = "Prescaler divider"]
        #[inline(always)]
        pub const fn pr(&self) -> super::vals::Pr {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Pr::from_bits(val as u8)
        }
        #[doc = "Prescaler divider"]
        #[inline(always)]
        pub fn set_pr(&mut self, val: super::vals::Pr) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Pr {
        #[inline(always)]
        fn default() -> Pr {
            Pr(0)
        }
    }
    impl core::fmt::Debug for Pr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pr").field("pr", &self.pr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pr {{ pr: {:?} }}", self.pr())
        }
    }
    #[doc = "Reload register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rlr(pub u32);
    impl Rlr {
        #[doc = "Watchdog counter reload value"]
        #[inline(always)]
        pub const fn rl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Watchdog counter reload value"]
        #[inline(always)]
        pub fn set_rl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
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
            f.debug_struct("Rlr").field("rl", &self.rl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rlr {{ rl: {=u16:?} }}", self.rl())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Watchdog prescaler value update"]
        #[inline(always)]
        pub const fn pvu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog prescaler value update"]
        #[inline(always)]
        pub fn set_pvu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Watchdog counter reload value update"]
        #[inline(always)]
        pub const fn rvu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog counter reload value update"]
        #[inline(always)]
        pub fn set_rvu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Watchdog counter window value update"]
        #[inline(always)]
        pub const fn wvu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog counter window value update"]
        #[inline(always)]
        pub fn set_wvu(&mut self, val: bool) {
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
                .field("pvu", &self.pvu())
                .field("rvu", &self.rvu())
                .field("wvu", &self.wvu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ pvu: {=bool:?}, rvu: {=bool:?}, wvu: {=bool:?} }}",
                self.pvu(),
                self.rvu(),
                self.wvu()
            )
        }
    }
    #[doc = "Window register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Winr(pub u32);
    impl Winr {
        #[doc = "Watchdog counter window value"]
        #[inline(always)]
        pub const fn win(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Watchdog counter window value"]
        #[inline(always)]
        pub fn set_win(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Winr {
        #[inline(always)]
        fn default() -> Winr {
            Winr(0)
        }
    }
    impl core::fmt::Debug for Winr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Winr").field("win", &self.win()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Winr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Winr {{ win: {=u16:?} }}", self.win())
        }
    }
}
pub mod vals {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key(u16);
    impl Key {
        #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
        pub const ENABLE: Self = Self(0x5555);
        #[doc = "Reset the watchdog value (0xAAAA)"]
        pub const RESET: Self = Self(0xaaaa);
        #[doc = "Start the watchdog (0xCCCC)"]
        pub const START: Self = Self(0xcccc);
    }
    impl Key {
        pub const fn from_bits(val: u16) -> Key {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Key {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x5555 => f.write_str("ENABLE"),
                0xaaaa => f.write_str("RESET"),
                0xcccc => f.write_str("START"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x5555 => defmt::write!(f, "ENABLE"),
                0xaaaa => defmt::write!(f, "RESET"),
                0xcccc => defmt::write!(f, "START"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Key {
        #[inline(always)]
        fn from(val: u16) -> Key {
            Key::from_bits(val)
        }
    }
    impl From<Key> for u16 {
        #[inline(always)]
        fn from(val: Key) -> u16 {
            Key::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pr {
        #[doc = "Divider /4"]
        DIVIDE_BY4 = 0x0,
        #[doc = "Divider /8"]
        DIVIDE_BY8 = 0x01,
        #[doc = "Divider /16"]
        DIVIDE_BY16 = 0x02,
        #[doc = "Divider /32"]
        DIVIDE_BY32 = 0x03,
        #[doc = "Divider /64"]
        DIVIDE_BY64 = 0x04,
        #[doc = "Divider /128"]
        DIVIDE_BY128 = 0x05,
        #[doc = "Divider /256"]
        DIVIDE_BY256 = 0x06,
        #[doc = "Divider /256"]
        DIVIDE_BY256BIS = 0x07,
    }
    impl Pr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pr {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pr {
        #[inline(always)]
        fn from(val: u8) -> Pr {
            Pr::from_bits(val)
        }
    }
    impl From<Pr> for u8 {
        #[inline(always)]
        fn from(val: Pr) -> u8 {
            Pr::to_bits(val)
        }
    }
}
