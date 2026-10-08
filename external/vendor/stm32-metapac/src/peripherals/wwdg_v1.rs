#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Window watchdog"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wwdg {
    ptr: *mut u8,
}
unsafe impl Send for Wwdg {}
unsafe impl Sync for Wwdg {}
impl Wwdg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn cfr(self) -> crate::common::Reg<regs::Cfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "Configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfr(pub u32);
    impl Cfr {
        #[doc = "7-bit window value"]
        #[inline(always)]
        pub const fn w(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit window value"]
        #[inline(always)]
        pub fn set_w(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Timer base"]
        #[inline(always)]
        pub const fn wdgtb(&self) -> super::vals::Wdgtb {
            let val = (self.0 >> 7usize) & 0x03;
            super::vals::Wdgtb::from_bits(val as u8)
        }
        #[doc = "Timer base"]
        #[inline(always)]
        pub fn set_wdgtb(&mut self, val: super::vals::Wdgtb) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
        }
        #[doc = "Early wakeup interrupt"]
        #[inline(always)]
        pub const fn ewi(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Early wakeup interrupt"]
        #[inline(always)]
        pub fn set_ewi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cfr {
        #[inline(always)]
        fn default() -> Cfr {
            Cfr(0)
        }
    }
    impl core::fmt::Debug for Cfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfr")
                .field("w", &self.w())
                .field("wdgtb", &self.wdgtb())
                .field("ewi", &self.ewi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfr {{ w: {=u8:?}, wdgtb: {:?}, ewi: {=bool:?} }}",
                self.w(),
                self.wdgtb(),
                self.ewi()
            )
        }
    }
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "7-bit counter (MSB to LSB)"]
        #[inline(always)]
        pub const fn t(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit counter (MSB to LSB)"]
        #[inline(always)]
        pub fn set_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Watchdog activated"]
        #[inline(always)]
        pub const fn wdga(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog activated"]
        #[inline(always)]
        pub fn set_wdga(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("t", &self.t())
                .field("wdga", &self.wdga())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr {{ t: {=u8:?}, wdga: {=bool:?} }}", self.t(), self.wdga())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Early wakeup interrupt flag"]
        #[inline(always)]
        pub const fn ewif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Early wakeup interrupt flag"]
        #[inline(always)]
        pub fn set_ewif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Sr").field("ewif", &self.ewif()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sr {{ ewif: {=bool:?} }}", self.ewif())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wdgtb {
        #[doc = "Counter clock (PCLK1 div 4096) div 1"]
        DIV1 = 0x0,
        #[doc = "Counter clock (PCLK1 div 4096) div 2"]
        DIV2 = 0x01,
        #[doc = "Counter clock (PCLK1 div 4096) div 4"]
        DIV4 = 0x02,
        #[doc = "Counter clock (PCLK1 div 4096) div 8"]
        DIV8 = 0x03,
    }
    impl Wdgtb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wdgtb {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wdgtb {
        #[inline(always)]
        fn from(val: u8) -> Wdgtb {
            Wdgtb::from_bits(val)
        }
    }
    impl From<Wdgtb> for u8 {
        #[inline(always)]
        fn from(val: Wdgtb) -> u8 {
            Wdgtb::to_bits(val)
        }
    }
}
