#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Real-time clock"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register High"]
    #[inline(always)]
    pub const fn crh(self) -> crate::common::Reg<regs::Crh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control Register Low"]
    #[inline(always)]
    pub const fn crl(self) -> crate::common::Reg<regs::Crl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Prescaler Load Register High"]
    #[inline(always)]
    pub const fn prlh(self) -> crate::common::Reg<regs::Prlh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Prescaler Load Register Low"]
    #[inline(always)]
    pub const fn prll(self) -> crate::common::Reg<regs::Prll, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Prescaler Divider Register High"]
    #[inline(always)]
    pub const fn divh(self) -> crate::common::Reg<regs::Divh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Prescaler Divider Register Low"]
    #[inline(always)]
    pub const fn divl(self) -> crate::common::Reg<regs::Divl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Counter Register High"]
    #[inline(always)]
    pub const fn cnth(self) -> crate::common::Reg<regs::Cnth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Counter Register Low"]
    #[inline(always)]
    pub const fn cntl(self) -> crate::common::Reg<regs::Cntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Alarm Register High"]
    #[inline(always)]
    pub const fn alrh(self) -> crate::common::Reg<regs::Alrh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Alarm Register Low"]
    #[inline(always)]
    pub const fn alrl(self) -> crate::common::Reg<regs::Alrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "Alarm Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrh(pub u32);
    impl Alrh {
        #[doc = "Alarm register high"]
        #[inline(always)]
        pub const fn alrh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Alarm register high"]
        #[inline(always)]
        pub fn set_alrh(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Alrh {
        #[inline(always)]
        fn default() -> Alrh {
            Alrh(0)
        }
    }
    impl core::fmt::Debug for Alrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrh").field("alrh", &self.alrh()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alrh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alrh {{ alrh: {=u16:?} }}", self.alrh())
        }
    }
    #[doc = "Alarm Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrl(pub u32);
    impl Alrl {
        #[doc = "Alarm register low"]
        #[inline(always)]
        pub const fn alrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Alarm register low"]
        #[inline(always)]
        pub fn set_alrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Alrl {
        #[inline(always)]
        fn default() -> Alrl {
            Alrl(0)
        }
    }
    impl core::fmt::Debug for Alrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrl").field("alrl", &self.alrl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alrl {{ alrl: {=u16:?} }}", self.alrl())
        }
    }
    #[doc = "Counter Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnth(pub u32);
    impl Cnth {
        #[doc = "Counter register high"]
        #[inline(always)]
        pub const fn cnth(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter register high"]
        #[inline(always)]
        pub fn set_cnth(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnth {
        #[inline(always)]
        fn default() -> Cnth {
            Cnth(0)
        }
    }
    impl core::fmt::Debug for Cnth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnth").field("cnth", &self.cnth()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnth {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cnth {{ cnth: {=u16:?} }}", self.cnth())
        }
    }
    #[doc = "Counter Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntl(pub u32);
    impl Cntl {
        #[doc = "Counter register low"]
        #[inline(always)]
        pub const fn cntl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter register low"]
        #[inline(always)]
        pub fn set_cntl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntl {
        #[inline(always)]
        fn default() -> Cntl {
            Cntl(0)
        }
    }
    impl core::fmt::Debug for Cntl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntl").field("cntl", &self.cntl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cntl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cntl {{ cntl: {=u16:?} }}", self.cntl())
        }
    }
    #[doc = "Control Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crh(pub u32);
    impl Crh {
        #[doc = "Second interrupt enable"]
        #[inline(always)]
        pub const fn secie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Second interrupt enable"]
        #[inline(always)]
        pub fn set_secie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Alarm interrupt enable"]
        #[inline(always)]
        pub const fn alrie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm interrupt enable"]
        #[inline(always)]
        pub fn set_alrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Overflow interrupt enable"]
        #[inline(always)]
        pub const fn owie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow interrupt enable"]
        #[inline(always)]
        pub fn set_owie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Crh {
        #[inline(always)]
        fn default() -> Crh {
            Crh(0)
        }
    }
    impl core::fmt::Debug for Crh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crh")
                .field("secie", &self.secie())
                .field("alrie", &self.alrie())
                .field("owie", &self.owie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Crh {{ secie: {=bool:?}, alrie: {=bool:?}, owie: {=bool:?} }}",
                self.secie(),
                self.alrie(),
                self.owie()
            )
        }
    }
    #[doc = "Control Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crl(pub u32);
    impl Crl {
        #[doc = "Second flag"]
        #[inline(always)]
        pub const fn secf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Second flag"]
        #[inline(always)]
        pub fn set_secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Alarm flag"]
        #[inline(always)]
        pub const fn alrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm flag"]
        #[inline(always)]
        pub fn set_alrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Overflow flag"]
        #[inline(always)]
        pub const fn owf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow flag"]
        #[inline(always)]
        pub fn set_owf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Registers synchronized flag"]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Registers synchronized flag"]
        #[inline(always)]
        pub fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enter configuration mode"]
        #[inline(always)]
        pub const fn cnf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enter configuration mode"]
        #[inline(always)]
        pub fn set_cnf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RTC operation OFF"]
        #[inline(always)]
        pub const fn rtoff(&self) -> super::vals::Rtoff {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Rtoff::from_bits(val as u8)
        }
        #[doc = "RTC operation OFF"]
        #[inline(always)]
        pub fn set_rtoff(&mut self, val: super::vals::Rtoff) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Crl {
        #[inline(always)]
        fn default() -> Crl {
            Crl(0)
        }
    }
    impl core::fmt::Debug for Crl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crl")
                .field("secf", &self.secf())
                .field("alrf", &self.alrf())
                .field("owf", &self.owf())
                .field("rsf", &self.rsf())
                .field("cnf", &self.cnf())
                .field("rtoff", &self.rtoff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Crl {{ secf: {=bool:?}, alrf: {=bool:?}, owf: {=bool:?}, rsf: {=bool:?}, cnf: {=bool:?}, rtoff: {:?} }}" , self . secf () , self . alrf () , self . owf () , self . rsf () , self . cnf () , self . rtoff ())
        }
    }
    #[doc = "Prescaler Divider Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Divh(pub u32);
    impl Divh {
        #[doc = "Prescaler divider register high"]
        #[inline(always)]
        pub const fn divh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Prescaler divider register high"]
        #[inline(always)]
        pub fn set_divh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Divh {
        #[inline(always)]
        fn default() -> Divh {
            Divh(0)
        }
    }
    impl core::fmt::Debug for Divh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Divh").field("divh", &self.divh()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Divh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Divh {{ divh: {=u8:?} }}", self.divh())
        }
    }
    #[doc = "Prescaler Divider Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Divl(pub u32);
    impl Divl {
        #[doc = "Prescaler divider register low"]
        #[inline(always)]
        pub const fn divl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Prescaler divider register low"]
        #[inline(always)]
        pub fn set_divl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Divl {
        #[inline(always)]
        fn default() -> Divl {
            Divl(0)
        }
    }
    impl core::fmt::Debug for Divl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Divl").field("divl", &self.divl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Divl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Divl {{ divl: {=u16:?} }}", self.divl())
        }
    }
    #[doc = "Prescaler Load Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prlh(pub u32);
    impl Prlh {
        #[doc = "Prescaler load register high"]
        #[inline(always)]
        pub const fn prlh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Prescaler load register high"]
        #[inline(always)]
        pub fn set_prlh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Prlh {
        #[inline(always)]
        fn default() -> Prlh {
            Prlh(0)
        }
    }
    impl core::fmt::Debug for Prlh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prlh").field("prlh", &self.prlh()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prlh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Prlh {{ prlh: {=u8:?} }}", self.prlh())
        }
    }
    #[doc = "Prescaler Load Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prll(pub u32);
    impl Prll {
        #[doc = "Prescaler divider register low"]
        #[inline(always)]
        pub const fn prll(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Prescaler divider register low"]
        #[inline(always)]
        pub fn set_prll(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Prll {
        #[inline(always)]
        fn default() -> Prll {
            Prll(0)
        }
    }
    impl core::fmt::Debug for Prll {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prll").field("prll", &self.prll()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prll {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Prll {{ prll: {=u16:?} }}", self.prll())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtoff {
        #[doc = "Last write operation on RTC registers is still ongoing"]
        ONGOING = 0x0,
        #[doc = "Last write operation on RTC registers terminated"]
        TERMINATED = 0x01,
    }
    impl Rtoff {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtoff {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtoff {
        #[inline(always)]
        fn from(val: u8) -> Rtoff {
            Rtoff::from_bits(val)
        }
    }
    impl From<Rtoff> for u8 {
        #[inline(always)]
        fn from(val: Rtoff) -> u8 {
            Rtoff::to_bits(val)
        }
    }
}
