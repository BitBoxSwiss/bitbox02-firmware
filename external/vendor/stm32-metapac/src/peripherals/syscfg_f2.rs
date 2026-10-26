#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "memory remap register"]
    #[inline(always)]
    pub const fn memrmp(self) -> crate::common::Reg<regs::Memrmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "peripheral mode configuration register"]
    #[inline(always)]
    pub const fn pmc(self) -> crate::common::Reg<regs::Pmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "Compensation cell control register"]
    #[inline(always)]
    pub const fn cmpcr(self) -> crate::common::Reg<regs::Cmpcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Compensation cell control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmpcr(pub u32);
    impl Cmpcr {
        #[doc = "Compensation cell power-down"]
        #[inline(always)]
        pub const fn cmp_pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Compensation cell power-down"]
        #[inline(always)]
        pub fn set_cmp_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Compensation cell ready flag"]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Compensation cell ready flag"]
        #[inline(always)]
        pub fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cmpcr {
        #[inline(always)]
        fn default() -> Cmpcr {
            Cmpcr(0)
        }
    }
    impl core::fmt::Debug for Cmpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmpcr")
                .field("cmp_pd", &self.cmp_pd())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmpcr {{ cmp_pd: {=bool:?}, ready: {=bool:?} }}",
                self.cmp_pd(),
                self.ready()
            )
        }
    }
    #[doc = "external interrupt configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI x configuration (x = 0 to 3)"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI x configuration (x = 0 to 3)"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "memory remap register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memrmp(pub u32);
    impl Memrmp {
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> super::vals::MemMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::MemMode::from_bits(val as u8)
        }
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: super::vals::MemMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Memrmp {
        #[inline(always)]
        fn default() -> Memrmp {
            Memrmp(0)
        }
    }
    impl core::fmt::Debug for Memrmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Memrmp").field("mem_mode", &self.mem_mode()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Memrmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Memrmp {{ mem_mode: {:?} }}", self.mem_mode())
        }
    }
    #[doc = "peripheral mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmc(pub u32);
    impl Pmc {
        #[doc = "Ethernet PHY interface selection"]
        #[inline(always)]
        pub const fn mii_rmii_sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PHY interface selection"]
        #[inline(always)]
        pub fn set_mii_rmii_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Pmc {
        #[inline(always)]
        fn default() -> Pmc {
            Pmc(0)
        }
    }
    impl core::fmt::Debug for Pmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmc")
                .field("mii_rmii_sel", &self.mii_rmii_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pmc {{ mii_rmii_sel: {=bool:?} }}", self.mii_rmii_sel())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MemMode {
        #[doc = "Main Flash memory mapped at 0x0000_0000"]
        MAIN_FLASH = 0x0,
        #[doc = "System Flash memory mapped at 0x0000_0000"]
        SYSTEM_FLASH = 0x01,
        #[doc = "FSMC Bank1 (NOR/PSRAM 1 and 2) mapped at 0x0000_0000"]
        FSMC = 0x02,
        #[doc = "Embedded SRAM mapped at 0x0000_0000"]
        SRAM = 0x03,
    }
    impl MemMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemMode {
        #[inline(always)]
        fn from(val: u8) -> MemMode {
            MemMode::from_bits(val)
        }
    }
    impl From<MemMode> for u8 {
        #[inline(always)]
        fn from(val: MemMode) -> u8 {
            MemMode::to_bits(val)
        }
    }
}
