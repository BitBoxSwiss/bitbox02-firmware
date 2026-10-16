#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RAMCFG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ramcfg {
    ptr: *mut u8,
}
unsafe impl Send for Ramcfg {}
unsafe impl Sync for Ramcfg {}
impl Ramcfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[inline(always)]
    pub const fn ram1cr(self) -> crate::common::Reg<regs::Ram1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[inline(always)]
    pub const fn ram1isr(self) -> crate::common::Reg<regs::Ram1isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RAMCFG SRAM x erase key register."]
    #[inline(always)]
    pub const fn ram1erkeyr(self) -> crate::common::Reg<regs::Ram1erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[inline(always)]
    pub const fn ram2cr(self) -> crate::common::Reg<regs::Ram2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "RAMCFG SRAM x interrupt enable register."]
    #[inline(always)]
    pub const fn ram2ier(self) -> crate::common::Reg<regs::Ram2ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[inline(always)]
    pub const fn ram2isr(self) -> crate::common::Reg<regs::Ram2isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "RAMCFG RAM x ECC single error address register."]
    #[inline(always)]
    pub const fn ram2sear(self) -> crate::common::Reg<regs::Ram2sear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "RAMCFG RAM x ECC double error address register."]
    #[inline(always)]
    pub const fn ram2dear(self) -> crate::common::Reg<regs::Ram2dear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RAMCFG RAM x interrupt clear register x."]
    #[inline(always)]
    pub const fn ram2icr(self) -> crate::common::Reg<regs::Ram2icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 write protection register 1."]
    #[inline(always)]
    pub const fn ram2wpr1(self) -> crate::common::Reg<regs::Ram2wpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 write protection register 2."]
    #[inline(always)]
    pub const fn ram2wpr2(self) -> crate::common::Reg<regs::Ram2wpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "RAMCFG SRAM x ECC key register."]
    #[inline(always)]
    pub const fn ram2ecckeyr(self) -> crate::common::Reg<regs::Ram2ecckeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RAMCFG SRAM x erase key register."]
    #[inline(always)]
    pub const fn ram2erkeyr(self) -> crate::common::Reg<regs::Ram2erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[inline(always)]
    pub const fn ram3cr(self) -> crate::common::Reg<regs::Ram3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "RAMCFG SRAM x interrupt enable register."]
    #[inline(always)]
    pub const fn ram3ier(self) -> crate::common::Reg<regs::Ram3ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[inline(always)]
    pub const fn ram3isr(self) -> crate::common::Reg<regs::Ram3isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "RAMCFG RAM x ECC single error address register."]
    #[inline(always)]
    pub const fn ram3sear(self) -> crate::common::Reg<regs::Ram3sear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "RAMCFG RAM x ECC double error address register."]
    #[inline(always)]
    pub const fn ram3dear(self) -> crate::common::Reg<regs::Ram3dear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "RAMCFG RAM x interrupt clear register x."]
    #[inline(always)]
    pub const fn ram3icr(self) -> crate::common::Reg<regs::Ram3icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RAMCFG SRAM x ECC key register."]
    #[inline(always)]
    pub const fn ram3ecckeyr(self) -> crate::common::Reg<regs::Ram3ecckeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "RAMCFG SRAM x erase key register."]
    #[inline(always)]
    pub const fn ram3erkeyr(self) -> crate::common::Reg<regs::Ram3erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[inline(always)]
    pub const fn ram4cr(self) -> crate::common::Reg<regs::Ram4cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[inline(always)]
    pub const fn ram4isr(self) -> crate::common::Reg<regs::Ram4isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "RAMCFG SRAM x erase key register."]
    #[inline(always)]
    pub const fn ram4erkeyr(self) -> crate::common::Reg<regs::Ram4erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[inline(always)]
    pub const fn ram5cr(self) -> crate::common::Reg<regs::Ram5cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "RAMCFG SRAM x interrupt enable register."]
    #[inline(always)]
    pub const fn ram5ier(self) -> crate::common::Reg<regs::Ram5ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[inline(always)]
    pub const fn ram5isr(self) -> crate::common::Reg<regs::Ram5isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "RAMCFG RAM x ECC single error address register."]
    #[inline(always)]
    pub const fn ram5sear(self) -> crate::common::Reg<regs::Ram5sear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "RAMCFG RAM x ECC double error address register."]
    #[inline(always)]
    pub const fn ram5dear(self) -> crate::common::Reg<regs::Ram5dear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "RAMCFG RAM x interrupt clear register x."]
    #[inline(always)]
    pub const fn ram5icr(self) -> crate::common::Reg<regs::Ram5icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
}
pub mod regs {
    #[doc = "RAMCFG SRAM x control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1cr(pub u32);
    impl Ram1cr {
        #[doc = "ECCE."]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECCE."]
        #[inline(always)]
        pub fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub const fn wsc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub fn set_wsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Ram1cr {
        #[inline(always)]
        fn default() -> Ram1cr {
            Ram1cr(0)
        }
    }
    impl core::fmt::Debug for Ram1cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .field("wsc", &self.wsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram1cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram1cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?}, wsc: {=u8:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer(),
                self.wsc()
            )
        }
    }
    #[doc = "RAMCFG SRAM x erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1erkeyr(pub u32);
    impl Ram1erkeyr {
        #[doc = "ERASEKEY."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ERASEKEY."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ram1erkeyr {
        #[inline(always)]
        fn default() -> Ram1erkeyr {
            Ram1erkeyr(0)
        }
    }
    impl core::fmt::Debug for Ram1erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1erkeyr")
                .field("erasekey", &self.erasekey())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram1erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram1erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1isr(pub u32);
    impl Ram1isr {
        #[doc = "SEDC."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEDC."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DED."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DED."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ram1isr {
        #[inline(always)]
        fn default() -> Ram1isr {
            Ram1isr(0)
        }
    }
    impl core::fmt::Debug for Ram1isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram1isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram1isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2cr(pub u32);
    impl Ram2cr {
        #[doc = "ECCE."]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECCE."]
        #[inline(always)]
        pub fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub const fn wsc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub fn set_wsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Ram2cr {
        #[inline(always)]
        fn default() -> Ram2cr {
            Ram2cr(0)
        }
    }
    impl core::fmt::Debug for Ram2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .field("wsc", &self.wsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram2cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?}, wsc: {=u8:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer(),
                self.wsc()
            )
        }
    }
    #[doc = "RAMCFG RAM x ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2dear(pub u32);
    impl Ram2dear {
        #[doc = "EDEA."]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "EDEA."]
        #[inline(always)]
        pub fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ram2dear {
        #[inline(always)]
        fn default() -> Ram2dear {
            Ram2dear(0)
        }
    }
    impl core::fmt::Debug for Ram2dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram2dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG SRAM x ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2ecckeyr(pub u32);
    impl Ram2ecckeyr {
        #[doc = "ECCKEY."]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECCKEY."]
        #[inline(always)]
        pub fn set_ecckey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ram2ecckeyr {
        #[inline(always)]
        fn default() -> Ram2ecckeyr {
            Ram2ecckeyr(0)
        }
    }
    impl core::fmt::Debug for Ram2ecckeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2ecckeyr").field("ecckey", &self.ecckey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2ecckeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram2ecckeyr {{ ecckey: {=u8:?} }}", self.ecckey())
        }
    }
    #[doc = "RAMCFG SRAM x erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2erkeyr(pub u32);
    impl Ram2erkeyr {
        #[doc = "ERASEKEY."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ERASEKEY."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ram2erkeyr {
        #[inline(always)]
        fn default() -> Ram2erkeyr {
            Ram2erkeyr(0)
        }
    }
    impl core::fmt::Debug for Ram2erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2erkeyr")
                .field("erasekey", &self.erasekey())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram2erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG RAM x interrupt clear register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2icr(pub u32);
    impl Ram2icr {
        #[doc = "CSEDC."]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CSEDC."]
        #[inline(always)]
        pub fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CDED."]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CDED."]
        #[inline(always)]
        pub fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ram2icr {
        #[inline(always)]
        fn default() -> Ram2icr {
            Ram2icr(0)
        }
    }
    impl core::fmt::Debug for Ram2icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram2icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG SRAM x interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2ier(pub u32);
    impl Ram2ier {
        #[doc = "SEIE."]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEIE."]
        #[inline(always)]
        pub fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DEIE."]
        #[inline(always)]
        pub const fn deie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DEIE."]
        #[inline(always)]
        pub fn set_deie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECCNMI."]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECCNMI."]
        #[inline(always)]
        pub fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ram2ier {
        #[inline(always)]
        fn default() -> Ram2ier {
            Ram2ier(0)
        }
    }
    impl core::fmt::Debug for Ram2ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram2ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2isr(pub u32);
    impl Ram2isr {
        #[doc = "SEDC."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEDC."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DED."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DED."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ram2isr {
        #[inline(always)]
        fn default() -> Ram2isr {
            Ram2isr(0)
        }
    }
    impl core::fmt::Debug for Ram2isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram2isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG RAM x ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2sear(pub u32);
    impl Ram2sear {
        #[doc = "ESEA."]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ESEA."]
        #[inline(always)]
        pub fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ram2sear {
        #[inline(always)]
        fn default() -> Ram2sear {
            Ram2sear(0)
        }
    }
    impl core::fmt::Debug for Ram2sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram2sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
    #[doc = "RAMCFG SRAM2 write protection register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2wpr1(pub u32);
    impl Ram2wpr1 {
        #[doc = "P0WP."]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "P0WP."]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ram2wpr1 {
        #[inline(always)]
        fn default() -> Ram2wpr1 {
            Ram2wpr1(0)
        }
    }
    impl core::fmt::Debug for Ram2wpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2wpr1")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2wpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ram2wpr1 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}" , self . pwp (0usize) , self . pwp (1usize) , self . pwp (2usize) , self . pwp (3usize) , self . pwp (4usize) , self . pwp (5usize) , self . pwp (6usize) , self . pwp (7usize) , self . pwp (8usize) , self . pwp (9usize) , self . pwp (10usize) , self . pwp (11usize) , self . pwp (12usize) , self . pwp (13usize) , self . pwp (14usize) , self . pwp (15usize) , self . pwp (16usize) , self . pwp (17usize) , self . pwp (18usize) , self . pwp (19usize) , self . pwp (20usize) , self . pwp (21usize) , self . pwp (22usize) , self . pwp (23usize) , self . pwp (24usize) , self . pwp (25usize) , self . pwp (26usize) , self . pwp (27usize) , self . pwp (28usize) , self . pwp (29usize) , self . pwp (30usize) , self . pwp (31usize))
        }
    }
    #[doc = "RAMCFG SRAM2 write protection register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2wpr2(pub u32);
    impl Ram2wpr2 {
        #[doc = "P32WP."]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "P32WP."]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ram2wpr2 {
        #[inline(always)]
        fn default() -> Ram2wpr2 {
            Ram2wpr2(0)
        }
    }
    impl core::fmt::Debug for Ram2wpr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2wpr2")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2wpr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ram2wpr2 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}" , self . pwp (0usize) , self . pwp (1usize) , self . pwp (2usize) , self . pwp (3usize) , self . pwp (4usize) , self . pwp (5usize) , self . pwp (6usize) , self . pwp (7usize) , self . pwp (8usize) , self . pwp (9usize) , self . pwp (10usize) , self . pwp (11usize) , self . pwp (12usize) , self . pwp (13usize) , self . pwp (14usize) , self . pwp (15usize) , self . pwp (16usize) , self . pwp (17usize) , self . pwp (18usize) , self . pwp (19usize) , self . pwp (20usize) , self . pwp (21usize) , self . pwp (22usize) , self . pwp (23usize) , self . pwp (24usize) , self . pwp (25usize) , self . pwp (26usize) , self . pwp (27usize) , self . pwp (28usize) , self . pwp (29usize) , self . pwp (30usize) , self . pwp (31usize))
        }
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3cr(pub u32);
    impl Ram3cr {
        #[doc = "ECCE."]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECCE."]
        #[inline(always)]
        pub fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub const fn wsc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub fn set_wsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Ram3cr {
        #[inline(always)]
        fn default() -> Ram3cr {
            Ram3cr(0)
        }
    }
    impl core::fmt::Debug for Ram3cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .field("wsc", &self.wsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram3cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?}, wsc: {=u8:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer(),
                self.wsc()
            )
        }
    }
    #[doc = "RAMCFG RAM x ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3dear(pub u32);
    impl Ram3dear {
        #[doc = "EDEA."]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "EDEA."]
        #[inline(always)]
        pub fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ram3dear {
        #[inline(always)]
        fn default() -> Ram3dear {
            Ram3dear(0)
        }
    }
    impl core::fmt::Debug for Ram3dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram3dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG SRAM x ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3ecckeyr(pub u32);
    impl Ram3ecckeyr {
        #[doc = "ECCKEY."]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECCKEY."]
        #[inline(always)]
        pub fn set_ecckey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ram3ecckeyr {
        #[inline(always)]
        fn default() -> Ram3ecckeyr {
            Ram3ecckeyr(0)
        }
    }
    impl core::fmt::Debug for Ram3ecckeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3ecckeyr").field("ecckey", &self.ecckey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3ecckeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram3ecckeyr {{ ecckey: {=u8:?} }}", self.ecckey())
        }
    }
    #[doc = "RAMCFG SRAM x erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3erkeyr(pub u32);
    impl Ram3erkeyr {
        #[doc = "ERASEKEY."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ERASEKEY."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ram3erkeyr {
        #[inline(always)]
        fn default() -> Ram3erkeyr {
            Ram3erkeyr(0)
        }
    }
    impl core::fmt::Debug for Ram3erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3erkeyr")
                .field("erasekey", &self.erasekey())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram3erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG RAM x interrupt clear register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3icr(pub u32);
    impl Ram3icr {
        #[doc = "CSEDC."]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CSEDC."]
        #[inline(always)]
        pub fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CDED."]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CDED."]
        #[inline(always)]
        pub fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ram3icr {
        #[inline(always)]
        fn default() -> Ram3icr {
            Ram3icr(0)
        }
    }
    impl core::fmt::Debug for Ram3icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram3icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG SRAM x interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3ier(pub u32);
    impl Ram3ier {
        #[doc = "SEIE."]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEIE."]
        #[inline(always)]
        pub fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DEIE."]
        #[inline(always)]
        pub const fn deie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DEIE."]
        #[inline(always)]
        pub fn set_deie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECCNMI."]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECCNMI."]
        #[inline(always)]
        pub fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ram3ier {
        #[inline(always)]
        fn default() -> Ram3ier {
            Ram3ier(0)
        }
    }
    impl core::fmt::Debug for Ram3ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram3ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3isr(pub u32);
    impl Ram3isr {
        #[doc = "SEDC."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEDC."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DED."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DED."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ram3isr {
        #[inline(always)]
        fn default() -> Ram3isr {
            Ram3isr(0)
        }
    }
    impl core::fmt::Debug for Ram3isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram3isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG RAM x ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3sear(pub u32);
    impl Ram3sear {
        #[doc = "ESEA."]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ESEA."]
        #[inline(always)]
        pub fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ram3sear {
        #[inline(always)]
        fn default() -> Ram3sear {
            Ram3sear(0)
        }
    }
    impl core::fmt::Debug for Ram3sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram3sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram4cr(pub u32);
    impl Ram4cr {
        #[doc = "ECCE."]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECCE."]
        #[inline(always)]
        pub fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub const fn wsc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub fn set_wsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Ram4cr {
        #[inline(always)]
        fn default() -> Ram4cr {
            Ram4cr(0)
        }
    }
    impl core::fmt::Debug for Ram4cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram4cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .field("wsc", &self.wsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram4cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram4cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?}, wsc: {=u8:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer(),
                self.wsc()
            )
        }
    }
    #[doc = "RAMCFG SRAM x erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram4erkeyr(pub u32);
    impl Ram4erkeyr {
        #[doc = "ERASEKEY."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ERASEKEY."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ram4erkeyr {
        #[inline(always)]
        fn default() -> Ram4erkeyr {
            Ram4erkeyr(0)
        }
    }
    impl core::fmt::Debug for Ram4erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram4erkeyr")
                .field("erasekey", &self.erasekey())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram4erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram4erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram4isr(pub u32);
    impl Ram4isr {
        #[doc = "SEDC."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEDC."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DED."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DED."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ram4isr {
        #[inline(always)]
        fn default() -> Ram4isr {
            Ram4isr(0)
        }
    }
    impl core::fmt::Debug for Ram4isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram4isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram4isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram4isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG SRAM x control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram5cr(pub u32);
    impl Ram5cr {
        #[doc = "ECCE."]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECCE."]
        #[inline(always)]
        pub fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ALE."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMER."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub const fn wsc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "WSC."]
        #[inline(always)]
        pub fn set_wsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Ram5cr {
        #[inline(always)]
        fn default() -> Ram5cr {
            Ram5cr(0)
        }
    }
    impl core::fmt::Debug for Ram5cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram5cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .field("wsc", &self.wsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram5cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram5cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?}, wsc: {=u8:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer(),
                self.wsc()
            )
        }
    }
    #[doc = "RAMCFG RAM x ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram5dear(pub u32);
    impl Ram5dear {
        #[doc = "EDEA."]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "EDEA."]
        #[inline(always)]
        pub fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ram5dear {
        #[inline(always)]
        fn default() -> Ram5dear {
            Ram5dear(0)
        }
    }
    impl core::fmt::Debug for Ram5dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram5dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram5dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram5dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG RAM x interrupt clear register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram5icr(pub u32);
    impl Ram5icr {
        #[doc = "CSEDC."]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CSEDC."]
        #[inline(always)]
        pub fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CDED."]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CDED."]
        #[inline(always)]
        pub fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ram5icr {
        #[inline(always)]
        fn default() -> Ram5icr {
            Ram5icr(0)
        }
    }
    impl core::fmt::Debug for Ram5icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram5icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram5icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram5icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG SRAM x interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram5ier(pub u32);
    impl Ram5ier {
        #[doc = "SEIE."]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEIE."]
        #[inline(always)]
        pub fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DEIE."]
        #[inline(always)]
        pub const fn deie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DEIE."]
        #[inline(always)]
        pub fn set_deie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECCNMI."]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECCNMI."]
        #[inline(always)]
        pub fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ram5ier {
        #[inline(always)]
        fn default() -> Ram5ier {
            Ram5ier(0)
        }
    }
    impl core::fmt::Debug for Ram5ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram5ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram5ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram5ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG RAMx interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram5isr(pub u32);
    impl Ram5isr {
        #[doc = "SEDC."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEDC."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DED."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DED."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAMBUSY."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ram5isr {
        #[inline(always)]
        fn default() -> Ram5isr {
            Ram5isr(0)
        }
    }
    impl core::fmt::Debug for Ram5isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram5isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram5isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram5isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG RAM x ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram5sear(pub u32);
    impl Ram5sear {
        #[doc = "ESEA."]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ESEA."]
        #[inline(always)]
        pub fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ram5sear {
        #[inline(always)]
        fn default() -> Ram5sear {
            Ram5sear(0)
        }
    }
    impl core::fmt::Debug for Ram5sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram5sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram5sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ram5sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
}
