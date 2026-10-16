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
    pub const fn kr(self) -> crate::common::Reg<regs::Kr, crate::common::RW> {
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
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Window register"]
    #[inline(always)]
    pub const fn winr(self) -> crate::common::Reg<regs::Winr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "IWDG early wakeup interrupt register."]
    #[inline(always)]
    pub const fn ewcr(self) -> crate::common::Reg<regs::Ewcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "IWDG early wakeup interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ewcr(pub u32);
    impl Ewcr {
        #[doc = "Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]
- 1. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
        #[inline(always)]
        pub const fn ewit(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]
- 1. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
        #[inline(always)]
        pub fn set_ewit(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0."]
        #[inline(always)]
        pub const fn ewic(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0."]
        #[inline(always)]
        pub fn set_ewic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
        #[inline(always)]
        pub const fn ewie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
        #[inline(always)]
        pub fn set_ewie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ewcr {
        #[inline(always)]
        fn default() -> Ewcr {
            Ewcr(0)
        }
    }
    impl core::fmt::Debug for Ewcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ewcr")
                .field("ewit", &self.ewit())
                .field("ewic", &self.ewic())
                .field("ewie", &self.ewie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ewcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ewcr {{ ewit: {=u16:?}, ewic: {=bool:?}, ewie: {=bool:?} }}",
                self.ewit(),
                self.ewic(),
                self.ewie()
            )
        }
    }
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
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Pr::from_bits(val as u8)
        }
        #[doc = "Prescaler divider"]
        #[inline(always)]
        pub fn set_pr(&mut self, val: super::vals::Pr) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
        #[doc = "Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\\[11:0\\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\\[11:0\\]
and EWIE fields can be updated only when EWU bit is reset."]
        #[inline(always)]
        pub const fn ewu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\\[11:0\\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\\[11:0\\]
and EWIE fields can be updated only when EWU bit is reset."]
        #[inline(always)]
        pub fn set_ewu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Watchdog early interrupt flag This bit is set to ‘1’ by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to ‘1’."]
        #[inline(always)]
        pub const fn ewif(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog early interrupt flag This bit is set to ‘1’ by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to ‘1’."]
        #[inline(always)]
        pub fn set_ewif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("ewu", &self.ewu())
                .field("ewif", &self.ewif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ pvu: {=bool:?}, rvu: {=bool:?}, wvu: {=bool:?}, ewu: {=bool:?}, ewif: {=bool:?} }}",
                self.pvu(),
                self.rvu(),
                self.wvu(),
                self.ewu(),
                self.ewif()
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
        #[doc = "Divider /512"]
        DIVIDE_BY512 = 0x07,
        #[doc = "Divider /1024"]
        DIVIDE_BY1024 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Pr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pr {
            unsafe { core::mem::transmute(val & 0x0f) }
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
