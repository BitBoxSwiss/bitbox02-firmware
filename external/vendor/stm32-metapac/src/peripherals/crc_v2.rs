#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cyclic Redundancy Check calculation unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
unsafe impl Send for Crc {}
unsafe impl Sync for Crc {}
impl Crc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data register - half-word sized"]
    #[inline(always)]
    pub const fn dr16(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr32(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Data register - byte sized"]
    #[inline(always)]
    pub const fn dr8(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Independent Data register"]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Initial CRC value"]
    #[inline(always)]
    pub const fn init(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "RESET bit"]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RESET bit"]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Polynomial size"]
        #[inline(always)]
        pub const fn polysize(&self) -> super::vals::Polysize {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Polysize::from_bits(val as u8)
        }
        #[doc = "Polynomial size"]
        #[inline(always)]
        pub fn set_polysize(&mut self, val: super::vals::Polysize) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Reverse input data"]
        #[inline(always)]
        pub const fn rev_in(&self) -> super::vals::RevIn {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::RevIn::from_bits(val as u8)
        }
        #[doc = "Reverse input data"]
        #[inline(always)]
        pub fn set_rev_in(&mut self, val: super::vals::RevIn) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Reverse output data"]
        #[inline(always)]
        pub const fn rev_out(&self) -> super::vals::RevOut {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::RevOut::from_bits(val as u8)
        }
        #[doc = "Reverse output data"]
        #[inline(always)]
        pub fn set_rev_out(&mut self, val: super::vals::RevOut) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
                .field("reset", &self.reset())
                .field("polysize", &self.polysize())
                .field("rev_in", &self.rev_in())
                .field("rev_out", &self.rev_out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ reset: {=bool:?}, polysize: {:?}, rev_in: {:?}, rev_out: {:?} }}",
                self.reset(),
                self.polysize(),
                self.rev_in(),
                self.rev_out()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Polysize {
        #[doc = "32-bit polynomial"]
        POLYSIZE32 = 0x0,
        #[doc = "16-bit polynomial"]
        POLYSIZE16 = 0x01,
        #[doc = "8-bit polynomial"]
        POLYSIZE8 = 0x02,
        #[doc = "7-bit polynomial"]
        POLYSIZE7 = 0x03,
    }
    impl Polysize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Polysize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Polysize {
        #[inline(always)]
        fn from(val: u8) -> Polysize {
            Polysize::from_bits(val)
        }
    }
    impl From<Polysize> for u8 {
        #[inline(always)]
        fn from(val: Polysize) -> u8 {
            Polysize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RevIn {
        #[doc = "Bit order not affected"]
        NORMAL = 0x0,
        #[doc = "Bit reversal done by byte"]
        BYTE = 0x01,
        #[doc = "Bit reversal done by half-word"]
        HALF_WORD = 0x02,
        #[doc = "Bit reversal done by word"]
        WORD = 0x03,
    }
    impl RevIn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RevIn {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RevIn {
        #[inline(always)]
        fn from(val: u8) -> RevIn {
            RevIn::from_bits(val)
        }
    }
    impl From<RevIn> for u8 {
        #[inline(always)]
        fn from(val: RevIn) -> u8 {
            RevIn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RevOut {
        #[doc = "Bit order not affected"]
        NORMAL = 0x0,
        #[doc = "Bit reversed output"]
        REVERSED = 0x01,
    }
    impl RevOut {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RevOut {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RevOut {
        #[inline(always)]
        fn from(val: u8) -> RevOut {
            RevOut::from_bits(val)
        }
    }
    impl From<RevOut> for u8 {
        #[inline(always)]
        fn from(val: RevOut) -> u8 {
            RevOut::to_bits(val)
        }
    }
}
