#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWR Address block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PWR control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PWR control register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PWR control register 3."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PWR voltage scaling register."]
    #[inline(always)]
    pub const fn vosr(self) -> crate::common::Reg<regs::Vosr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PWR supply voltage monitoring control register."]
    #[inline(always)]
    pub const fn svmcr(self) -> crate::common::Reg<regs::Svmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PWR wakeup control register 1."]
    #[inline(always)]
    pub const fn wucr1(self) -> crate::common::Reg<regs::Wucr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PWR wakeup control register 2."]
    #[inline(always)]
    pub const fn wucr2(self) -> crate::common::Reg<regs::Wucr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "PWR wakeup control register 3."]
    #[inline(always)]
    pub const fn wucr3(self) -> crate::common::Reg<regs::Wucr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "PWR Backup domain control register."]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PWR disable Backup domain register."]
    #[inline(always)]
    pub const fn dbpcr(self) -> crate::common::Reg<regs::Dbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PWR security configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "PWR privilege control register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "PWR status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "PWR supply voltage monitoring status register."]
    #[inline(always)]
    pub const fn svmsr(self) -> crate::common::Reg<regs::Svmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "PWR wakeup status register."]
    #[inline(always)]
    pub const fn wusr(self) -> crate::common::Reg<regs::Wusr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "PWR wakeup status clear register."]
    #[inline(always)]
    pub const fn wuscr(self) -> crate::common::Reg<regs::Wuscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "PWR apply pull configuration register."]
    #[inline(always)]
    pub const fn apcr(self) -> crate::common::Reg<regs::Apcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "PWR port A pull-up control register."]
    #[inline(always)]
    pub const fn pucra(self) -> crate::common::Reg<regs::Pucra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "PWR port A pull-down control register."]
    #[inline(always)]
    pub const fn pdcra(self) -> crate::common::Reg<regs::Pdcra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "PWR port B pull-up control register."]
    #[inline(always)]
    pub const fn pucrb(self) -> crate::common::Reg<regs::Pucrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "PWR port B pull-down control register."]
    #[inline(always)]
    pub const fn pdcrb(self) -> crate::common::Reg<regs::Pdcrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "PWR port C pull-up control register."]
    #[inline(always)]
    pub const fn pucrc(self) -> crate::common::Reg<regs::Pucrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "PWR port C pull-down control register."]
    #[inline(always)]
    pub const fn pdcrc(self) -> crate::common::Reg<regs::Pdcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "PWR port D pull-up control register."]
    #[inline(always)]
    pub const fn pucrd(self) -> crate::common::Reg<regs::Pucrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "PWR port D pull-down control register."]
    #[inline(always)]
    pub const fn pdcrd(self) -> crate::common::Reg<regs::Pdcrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "PWR port E pull-up control register."]
    #[inline(always)]
    pub const fn pucre(self) -> crate::common::Reg<regs::Pucre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "PWR port E pull-down control register."]
    #[inline(always)]
    pub const fn pdcre(self) -> crate::common::Reg<regs::Pdcre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "PWR port G pull-up control register."]
    #[inline(always)]
    pub const fn pucrg(self) -> crate::common::Reg<regs::Pucrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "PWR port G pull-down control register."]
    #[inline(always)]
    pub const fn pdcrg(self) -> crate::common::Reg<regs::Pdcrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "PWR port H pull-up control register."]
    #[inline(always)]
    pub const fn pucrh(self) -> crate::common::Reg<regs::Pucrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "PWR port H pull-down control register."]
    #[inline(always)]
    pub const fn pdcrh(self) -> crate::common::Reg<regs::Pdcrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "PWR I3C pull-up control register 1."]
    #[inline(always)]
    pub const fn i3cpucr1(self) -> crate::common::Reg<regs::I3cpucr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "PWR I3C pull-up control register 2."]
    #[inline(always)]
    pub const fn i3cpucr2(self) -> crate::common::Reg<regs::I3cpucr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWR apply pull configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apcr(pub u32);
    impl Apcr {
        #[doc = "When this bit is set, the I/O pull-up and pull-down configurations defined in PUCRx and PDCRx are applied. When this bit is cleared, PUCRx and PDCRx are not applied to the I/Os."]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When this bit is set, the I/O pull-up and pull-down configurations defined in PUCRx and PDCRx are applied. When this bit is cleared, PUCRx and PDCRx are not applied to the I/Os."]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Apcr {
        #[inline(always)]
        fn default() -> Apcr {
            Apcr(0)
        }
    }
    impl core::fmt::Debug for Apcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apcr").field("apc", &self.apc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apcr {{ apc: {=bool:?} }}", self.apc())
        }
    }
    #[doc = "PWR Backup domain control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "None 0: VBAT battery charging disabled 1: VBAT battery charging enabled."]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VBAT battery charging disabled 1: VBAT battery charging enabled."]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "None 0: Charge VBAT through a 5 k ohm resistor 1: Charge VBAT through a 1.5 k ohm resistor."]
        #[inline(always)]
        pub const fn vbrs(&self) -> super::vals::Vbrs {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Vbrs::from_bits(val as u8)
        }
        #[doc = "None 0: Charge VBAT through a 5 k ohm resistor 1: Charge VBAT through a 1.5 k ohm resistor."]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: super::vals::Vbrs) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    impl core::fmt::Debug for Bdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr")
                .field("vbe", &self.vbe())
                .field("vbrs", &self.vbrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bdcr {{ vbe: {=bool:?}, vbrs: {:?} }}", self.vbe(), self.vbrs())
        }
    }
    #[doc = "PWR control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 000: Stop 0 mode 001: Stop 1 mode 010: Stop 2 mode 011: Stop 3 mode 100-101: Standby mode 110-111: Shutdown mode."]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 000: Stop 0 mode 001: Stop 1 mode 010: Stop 2 mode 011: Stop 3 mode 100-101: Standby mode 110-111: Shutdown mode."]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "This bit is used to keep the SRAM2 page 1 content in Standby mode. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). 0: SRAM2 page1 content not retained in Standby mode 1: SRAM2 page1 content retained in Standby mode Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub const fn rrsb1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is used to keep the SRAM2 page 1 content in Standby mode. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). 0: SRAM2 page1 content not retained in Standby mode 1: SRAM2 page1 content retained in Standby mode Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub fn set_rrsb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "This bit is used to keep the SRAM2 page 2 content in Standby mode. The SRAM2 page 2 corresponds to the 24 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0x7FFF). 0: SRAM2 page2 content not retained in Standby mode 1: SRAM2 page2 content retained in Standby mode Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub const fn rrsb2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is used to keep the SRAM2 page 2 content in Standby mode. The SRAM2 page 2 corresponds to the 24 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0x7FFF). 0: SRAM2 page2 content not retained in Standby mode 1: SRAM2 page2 content retained in Standby mode Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub fn set_rrsb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "This bit is used to keep the SRAM2 page 3 content in Standby mode. The SRAM2 page 3 corresponds to the last 32 Kbytes of the SRAM2 (from SRAM2 base address + 0x8000 to SRAM2 base address + 0xFFFF). 0: SRAM2 page3 content not retained in Standby mode 1: SRAM2 page3 content retained in Standby mode Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub const fn rrsb3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is used to keep the SRAM2 page 3 content in Standby mode. The SRAM2 page 3 corresponds to the last 32 Kbytes of the SRAM2 (from SRAM2 base address + 0x8000 to SRAM2 base address + 0xFFFF). 0: SRAM2 page3 content not retained in Standby mode 1: SRAM2 page3 content retained in Standby mode Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub fn set_rrsb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit has effect only when the BOR level 0 is selected and when the device is in Standby mode. This bit must be set to reach the lowest power consumption in Standby mode. 0: BOR level 0 operating in continuous (normal) mode in Standby mode 1: BOR level 0 operating in discontinuous (ultra-low power) mode in Standby mode."]
        #[inline(always)]
        pub const fn ulpmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit has effect only when the BOR level 0 is selected and when the device is in Standby mode. This bit must be set to reach the lowest power consumption in Standby mode. 0: BOR level 0 operating in continuous (normal) mode in Standby mode 1: BOR level 0 operating in discontinuous (ultra-low power) mode in Standby mode."]
        #[inline(always)]
        pub fn set_ulpmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "This bit is used to reduce the consumption by powering off the SRAM1. 0: SRAM1 powered on 1: SRAM1 powered off Note: When this bit is cleared to 0, wait for more than 1.6us before accessing the SRAM."]
        #[inline(always)]
        pub const fn sram1pd(&self) -> super::vals::Srampd {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Srampd::from_bits(val as u8)
        }
        #[doc = "This bit is used to reduce the consumption by powering off the SRAM1. 0: SRAM1 powered on 1: SRAM1 powered off Note: When this bit is cleared to 0, wait for more than 1.6us before accessing the SRAM."]
        #[inline(always)]
        pub fn set_sram1pd(&mut self, val: super::vals::Srampd) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "This bit is used to reduce the consumption by powering off the SRAM2. 0: SRAM2 powered on 1: SRAM2 powered off Note: When this bit is cleared to 0, wait for more than 1.6us before accessing the SRAM."]
        #[inline(always)]
        pub const fn sram2pd(&self) -> super::vals::Srampd {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Srampd::from_bits(val as u8)
        }
        #[doc = "This bit is used to reduce the consumption by powering off the SRAM2. 0: SRAM2 powered on 1: SRAM2 powered off Note: When this bit is cleared to 0, wait for more than 1.6us before accessing the SRAM."]
        #[inline(always)]
        pub fn set_sram2pd(&mut self, val: super::vals::Srampd) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("lpms", &self.lpms())
                .field("rrsb1", &self.rrsb1())
                .field("rrsb2", &self.rrsb2())
                .field("rrsb3", &self.rrsb3())
                .field("ulpmen", &self.ulpmen())
                .field("sram1pd", &self.sram1pd())
                .field("sram2pd", &self.sram2pd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ lpms: {:?}, rrsb1: {=bool:?}, rrsb2: {=bool:?}, rrsb3: {=bool:?}, ulpmen: {=bool:?}, sram1pd: {:?}, sram2pd: {:?} }}" , self . lpms () , self . rrsb1 () , self . rrsb2 () , self . rrsb3 () , self . ulpmen () , self . sram1pd () , self . sram2pd ())
        }
    }
    #[doc = "PWR control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "None 0: SRAM1 page 1 content retained in Stop modes 1: SRAM1 page 1 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub const fn sram1pds1(&self) -> super::vals::Pds {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM1 page 1 content retained in Stop modes 1: SRAM1 page 1 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub fn set_sram1pds1(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "None 0: SRAM1 page 2 content retained in Stop modes 1: SRAM1 page 2 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub const fn sram1pds2(&self) -> super::vals::Pds {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM1 page 2 content retained in Stop modes 1: SRAM1 page 2 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub fn set_sram1pds2(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "None 0: SRAM1 page 3 content retained in Stop modes 1: SRAM1 page 3 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub const fn sram1pds3(&self) -> super::vals::Pds {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM1 page 3 content retained in Stop modes 1: SRAM1 page 3 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub fn set_sram1pds3(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "None 0: SRAM1 page 4 content retained in Stop modes 1: SRAM1 page 4 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub const fn sram1pds4(&self) -> super::vals::Pds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM1 page 4 content retained in Stop modes 1: SRAM1 page 4 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub fn set_sram1pds4(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "None 0: SRAM1 page 5 content retained in Stop modes 1: SRAM1 page 5 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub const fn sram1pds5(&self) -> super::vals::Pds {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM1 page 5 content retained in Stop modes 1: SRAM1 page 5 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub fn set_sram1pds5(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "None 0: SRAM1 page 6 content retained in Stop modes 1: SRAM1 page 6 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub const fn sram1pds6(&self) -> super::vals::Pds {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM1 page 6 content retained in Stop modes 1: SRAM1 page 6 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub fn set_sram1pds6(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "None 0: SRAM1 page 7 content retained in Stop modes 1: SRAM1 page 7 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub const fn sram1pds7(&self) -> super::vals::Pds {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM1 page 7 content retained in Stop modes 1: SRAM1 page 7 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes."]
        #[inline(always)]
        pub fn set_sram1pds7(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "None 0: SRAM2 page 1 content retained in Stop modes 1: SRAM2 page 1 content lost in Stop modes."]
        #[inline(always)]
        pub const fn sram2pds1(&self) -> super::vals::Pds {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM2 page 1 content retained in Stop modes 1: SRAM2 page 1 content lost in Stop modes."]
        #[inline(always)]
        pub fn set_sram2pds1(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "None 0: SRAM2 page 2 content retained in Stop modes 1: SRAM2 page 2 content lost in Stop modes."]
        #[inline(always)]
        pub const fn sram2pds2(&self) -> super::vals::Pds {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM2 page 2 content retained in Stop modes 1: SRAM2 page 2 content lost in Stop modes."]
        #[inline(always)]
        pub fn set_sram2pds2(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "None 0: SRAM2 page 3 content retained in Stop modes 1: SRAM2 page 3 content lost in Stop modes."]
        #[inline(always)]
        pub const fn sram2pds3(&self) -> super::vals::Pds {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: SRAM2 page 3 content retained in Stop modes 1: SRAM2 page 3 content lost in Stop modes."]
        #[inline(always)]
        pub fn set_sram2pds3(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "None 0: ICACHE SRAM content retained in Stop modes 1: ICACHE SRAM content lost in Stop modes."]
        #[inline(always)]
        pub const fn icrampds(&self) -> super::vals::Pds {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: ICACHE SRAM content retained in Stop modes 1: ICACHE SRAM content lost in Stop modes."]
        #[inline(always)]
        pub fn set_icrampds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "None 0: FDCAN and USB peripherals SRAM content retained in Stop modes 1: FDCAN and USB peripherals SRAM content lost in Stop modes."]
        #[inline(always)]
        pub const fn prampds(&self) -> super::vals::Pds {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: FDCAN and USB peripherals SRAM content retained in Stop modes 1: FDCAN and USB peripherals SRAM content lost in Stop modes."]
        #[inline(always)]
        pub fn set_prampds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "None 0: PKA SRAM content retained in Stop modes 1: PKA SRAM content lost in Stop modes."]
        #[inline(always)]
        pub const fn pkarampds(&self) -> super::vals::Pds {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "None 0: PKA SRAM content retained in Stop modes 1: PKA SRAM content lost in Stop modes."]
        #[inline(always)]
        pub fn set_pkarampds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAMs wakeup time increases the wakeup time when exiting Stop 0 and 1 modes, and also increases the GPDMA1 access time to SRAMs during Stop modes. 0: SRAMs enters low-power mode in Stop 0 and Stop 1 modes (source biasing for lower-power consumption). 1: SRAMs remains in normal mode in Stop 0 and Stop 1 modes (higher consumption but no SRAM wakeup time). Note: in case one or several SRAMs are configured to be in power-down in Stop mode, setting SRAMFWU bit has no effect."]
        #[inline(always)]
        pub const fn sramfwu(&self) -> super::vals::Sramfwu {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Sramfwu::from_bits(val as u8)
        }
        #[doc = "This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAMs wakeup time increases the wakeup time when exiting Stop 0 and 1 modes, and also increases the GPDMA1 access time to SRAMs during Stop modes. 0: SRAMs enters low-power mode in Stop 0 and Stop 1 modes (source biasing for lower-power consumption). 1: SRAMs remains in normal mode in Stop 0 and Stop 1 modes (higher consumption but no SRAM wakeup time). Note: in case one or several SRAMs are configured to be in power-down in Stop mode, setting SRAMFWU bit has no effect."]
        #[inline(always)]
        pub fn set_sramfwu(&mut self, val: super::vals::Sramfwu) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption. 0: Flash memory enters low-power mode in Stop 0 and Stop 1 modes (lower-power consumption). 1: Flash memory remains in normal mode in Stop 0 and Stop 1 modes (faster wakeup time)."]
        #[inline(always)]
        pub const fn flashfwu(&self) -> super::vals::Flashfwu {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Flashfwu::from_bits(val as u8)
        }
        #[doc = "This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption. 0: Flash memory enters low-power mode in Stop 0 and Stop 1 modes (lower-power consumption). 1: Flash memory remains in normal mode in Stop 0 and Stop 1 modes (faster wakeup time)."]
        #[inline(always)]
        pub fn set_flashfwu(&mut self, val: super::vals::Flashfwu) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("sram1pds1", &self.sram1pds1())
                .field("sram1pds2", &self.sram1pds2())
                .field("sram1pds3", &self.sram1pds3())
                .field("sram1pds4", &self.sram1pds4())
                .field("sram1pds5", &self.sram1pds5())
                .field("sram1pds6", &self.sram1pds6())
                .field("sram1pds7", &self.sram1pds7())
                .field("sram2pds1", &self.sram2pds1())
                .field("sram2pds2", &self.sram2pds2())
                .field("sram2pds3", &self.sram2pds3())
                .field("icrampds", &self.icrampds())
                .field("prampds", &self.prampds())
                .field("pkarampds", &self.pkarampds())
                .field("sramfwu", &self.sramfwu())
                .field("flashfwu", &self.flashfwu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ sram1pds1: {:?}, sram1pds2: {:?}, sram1pds3: {:?}, sram1pds4: {:?}, sram1pds5: {:?}, sram1pds6: {:?}, sram1pds7: {:?}, sram2pds1: {:?}, sram2pds2: {:?}, sram2pds3: {:?}, icrampds: {:?}, prampds: {:?}, pkarampds: {:?}, sramfwu: {:?}, flashfwu: {:?} }}" , self . sram1pds1 () , self . sram1pds2 () , self . sram1pds3 () , self . sram1pds4 () , self . sram1pds5 () , self . sram1pds6 () , self . sram1pds7 () , self . sram2pds1 () , self . sram2pds2 () , self . sram2pds3 () , self . icrampds () , self . prampds () , self . pkarampds () , self . sramfwu () , self . flashfwu ())
        }
    }
    #[doc = "PWR control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "None 0: LDO selected 1: SMPS selected Note: REGSEL is reserved and must be kept at reset value in packages without SMPS."]
        #[inline(always)]
        pub const fn regsel(&self) -> super::vals::Regsel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Regsel::from_bits(val as u8)
        }
        #[doc = "None 0: LDO selected 1: SMPS selected Note: REGSEL is reserved and must be kept at reset value in packages without SMPS."]
        #[inline(always)]
        pub fn set_regsel(&mut self, val: super::vals::Regsel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "None 0: LDO/SMPS fast startup disabled (limited inrush current) 1: LDO/SMPS fast startup enabled."]
        #[inline(always)]
        pub const fn fsten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: LDO/SMPS fast startup disabled (limited inrush current) 1: LDO/SMPS fast startup enabled."]
        #[inline(always)]
        pub fn set_fsten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("regsel", &self.regsel())
                .field("fsten", &self.fsten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr3 {{ regsel: {:?}, fsten: {=bool:?} }}",
                self.regsel(),
                self.fsten()
            )
        }
    }
    #[doc = "PWR disable Backup domain register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbpcr(pub u32);
    impl Dbpcr {
        #[doc = "In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers. 0: Write access to Backup domain disabled 1: Write access to Backup domain enabled."]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers. 0: Write access to Backup domain disabled 1: Write access to Backup domain enabled."]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Dbpcr {
        #[inline(always)]
        fn default() -> Dbpcr {
            Dbpcr(0)
        }
    }
    impl core::fmt::Debug for Dbpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbpcr").field("dbp", &self.dbp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dbpcr {{ dbp: {=bool:?} }}", self.dbp())
        }
    }
    #[doc = "PWR I3C pull-up control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cpucr1(pub u32);
    impl I3cpucr1 {
        #[doc = "When set, the bit activates the I3C pull-up on PA1."]
        #[inline(always)]
        pub const fn pa1_i3cpu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PA1."]
        #[inline(always)]
        pub fn set_pa1_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PA6."]
        #[inline(always)]
        pub const fn pa6_i3cpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PA6."]
        #[inline(always)]
        pub fn set_pa6_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PA7."]
        #[inline(always)]
        pub const fn pa7_i3cpu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PA7."]
        #[inline(always)]
        pub fn set_pa7_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB2."]
        #[inline(always)]
        pub const fn pb2_i3cpu(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB2."]
        #[inline(always)]
        pub fn set_pb2_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB6."]
        #[inline(always)]
        pub const fn pb6_i3cpu(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB6."]
        #[inline(always)]
        pub fn set_pb6_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB8."]
        #[inline(always)]
        pub const fn pb8_i3cpu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB8."]
        #[inline(always)]
        pub fn set_pb8_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB9."]
        #[inline(always)]
        pub const fn pb9_i3cpu(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB9."]
        #[inline(always)]
        pub fn set_pb9_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB10."]
        #[inline(always)]
        pub const fn pb10_i3cpu(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB10."]
        #[inline(always)]
        pub fn set_pb10_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB12."]
        #[inline(always)]
        pub const fn pb12_i3cpu(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB12."]
        #[inline(always)]
        pub fn set_pb12_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB13."]
        #[inline(always)]
        pub const fn pb13_i3cpu(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB13."]
        #[inline(always)]
        pub fn set_pb13_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB14."]
        #[inline(always)]
        pub const fn pb14_i3cpu(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PB14."]
        #[inline(always)]
        pub fn set_pb14_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for I3cpucr1 {
        #[inline(always)]
        fn default() -> I3cpucr1 {
            I3cpucr1(0)
        }
    }
    impl core::fmt::Debug for I3cpucr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cpucr1")
                .field("pa1_i3cpu", &self.pa1_i3cpu())
                .field("pa6_i3cpu", &self.pa6_i3cpu())
                .field("pa7_i3cpu", &self.pa7_i3cpu())
                .field("pb2_i3cpu", &self.pb2_i3cpu())
                .field("pb6_i3cpu", &self.pb6_i3cpu())
                .field("pb8_i3cpu", &self.pb8_i3cpu())
                .field("pb9_i3cpu", &self.pb9_i3cpu())
                .field("pb10_i3cpu", &self.pb10_i3cpu())
                .field("pb12_i3cpu", &self.pb12_i3cpu())
                .field("pb13_i3cpu", &self.pb13_i3cpu())
                .field("pb14_i3cpu", &self.pb14_i3cpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cpucr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "I3cpucr1 {{ pa1_i3cpu: {=bool:?}, pa6_i3cpu: {=bool:?}, pa7_i3cpu: {=bool:?}, pb2_i3cpu: {=bool:?}, pb6_i3cpu: {=bool:?}, pb8_i3cpu: {=bool:?}, pb9_i3cpu: {=bool:?}, pb10_i3cpu: {=bool:?}, pb12_i3cpu: {=bool:?}, pb13_i3cpu: {=bool:?}, pb14_i3cpu: {=bool:?} }}" , self . pa1_i3cpu () , self . pa6_i3cpu () , self . pa7_i3cpu () , self . pb2_i3cpu () , self . pb6_i3cpu () , self . pb8_i3cpu () , self . pb9_i3cpu () , self . pb10_i3cpu () , self . pb12_i3cpu () , self . pb13_i3cpu () , self . pb14_i3cpu ())
        }
    }
    #[doc = "PWR I3C pull-up control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cpucr2(pub u32);
    impl I3cpucr2 {
        #[doc = "When set, the bit activates the I3C pull-up on PC0."]
        #[inline(always)]
        pub const fn pc0_i3cpu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PC0."]
        #[inline(always)]
        pub fn set_pc0_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PC1."]
        #[inline(always)]
        pub const fn pc1_i3cpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PC1."]
        #[inline(always)]
        pub fn set_pc1_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PD12."]
        #[inline(always)]
        pub const fn pd12_i3cpu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PD12."]
        #[inline(always)]
        pub fn set_pd12_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PD13."]
        #[inline(always)]
        pub const fn pd13_i3cpu(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PD13."]
        #[inline(always)]
        pub fn set_pd13_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PG7."]
        #[inline(always)]
        pub const fn pg7_i3cpu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PG7."]
        #[inline(always)]
        pub fn set_pg7_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PG8."]
        #[inline(always)]
        pub const fn pg8_i3cpu(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PG8."]
        #[inline(always)]
        pub fn set_pg8_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PG13."]
        #[inline(always)]
        pub const fn pg13_i3cpu(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PG13."]
        #[inline(always)]
        pub fn set_pg13_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PG14."]
        #[inline(always)]
        pub const fn pg14_i3cpu(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PG14."]
        #[inline(always)]
        pub fn set_pg14_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, the bit activates the I3C pull-up on PH3."]
        #[inline(always)]
        pub const fn ph3_i3cpu(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, the bit activates the I3C pull-up on PH3."]
        #[inline(always)]
        pub fn set_ph3_i3cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for I3cpucr2 {
        #[inline(always)]
        fn default() -> I3cpucr2 {
            I3cpucr2(0)
        }
    }
    impl core::fmt::Debug for I3cpucr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cpucr2")
                .field("pc0_i3cpu", &self.pc0_i3cpu())
                .field("pc1_i3cpu", &self.pc1_i3cpu())
                .field("pd12_i3cpu", &self.pd12_i3cpu())
                .field("pd13_i3cpu", &self.pd13_i3cpu())
                .field("pg7_i3cpu", &self.pg7_i3cpu())
                .field("pg8_i3cpu", &self.pg8_i3cpu())
                .field("pg13_i3cpu", &self.pg13_i3cpu())
                .field("pg14_i3cpu", &self.pg14_i3cpu())
                .field("ph3_i3cpu", &self.ph3_i3cpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cpucr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "I3cpucr2 {{ pc0_i3cpu: {=bool:?}, pc1_i3cpu: {=bool:?}, pd12_i3cpu: {=bool:?}, pd13_i3cpu: {=bool:?}, pg7_i3cpu: {=bool:?}, pg8_i3cpu: {=bool:?}, pg13_i3cpu: {=bool:?}, pg14_i3cpu: {=bool:?}, ph3_i3cpu: {=bool:?} }}" , self . pc0_i3cpu () , self . pc1_i3cpu () , self . pd12_i3cpu () , self . pd13_i3cpu () , self . pg7_i3cpu () , self . pg8_i3cpu () , self . pg13_i3cpu () , self . pg14_i3cpu () , self . ph3_i3cpu ())
        }
    }
    #[doc = "PWR port A pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcra(pub u32);
    impl Pdcra {
        #[doc = "When set, each bit activates the pull-down on PA0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-down on PA14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PA14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Pdcra {
        #[inline(always)]
        fn default() -> Pdcra {
            Pdcra(0)
        }
    }
    impl core::fmt::Debug for Pdcra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcra")
                .field("pd0", &self.pd0())
                .field("pd1", &self.pd1())
                .field("pd2", &self.pd2())
                .field("pd3", &self.pd3())
                .field("pd4", &self.pd4())
                .field("pd5", &self.pd5())
                .field("pd6", &self.pd6())
                .field("pd7", &self.pd7())
                .field("pd8", &self.pd8())
                .field("pd9", &self.pd9())
                .field("pd10", &self.pd10())
                .field("pd11", &self.pd11())
                .field("pd12", &self.pd12())
                .field("pd14", &self.pd14())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcra {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pdcra {{ pd0: {=bool:?}, pd1: {=bool:?}, pd2: {=bool:?}, pd3: {=bool:?}, pd4: {=bool:?}, pd5: {=bool:?}, pd6: {=bool:?}, pd7: {=bool:?}, pd8: {=bool:?}, pd9: {=bool:?}, pd10: {=bool:?}, pd11: {=bool:?}, pd12: {=bool:?}, pd14: {=bool:?} }}" , self . pd0 () , self . pd1 () , self . pd2 () , self . pd3 () , self . pd4 () , self . pd5 () , self . pd6 () , self . pd7 () , self . pd8 () , self . pd9 () , self . pd10 () , self . pd11 () , self . pd12 () , self . pd14 ())
        }
    }
    #[doc = "PWR port B pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrb(pub u32);
    impl Pdcrb {
        #[doc = "When set, each bit activates the pull-down on PB0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-down on PB15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PB15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pdcrb {
        #[inline(always)]
        fn default() -> Pdcrb {
            Pdcrb(0)
        }
    }
    impl core::fmt::Debug for Pdcrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcrb")
                .field("pd0", &self.pd0())
                .field("pd1", &self.pd1())
                .field("pd2", &self.pd2())
                .field("pd3", &self.pd3())
                .field("pd5", &self.pd5())
                .field("pd6", &self.pd6())
                .field("pd7", &self.pd7())
                .field("pd8", &self.pd8())
                .field("pd9", &self.pd9())
                .field("pd10", &self.pd10())
                .field("pd11", &self.pd11())
                .field("pd12", &self.pd12())
                .field("pd13", &self.pd13())
                .field("pd14", &self.pd14())
                .field("pd15", &self.pd15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcrb {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pdcrb {{ pd0: {=bool:?}, pd1: {=bool:?}, pd2: {=bool:?}, pd3: {=bool:?}, pd5: {=bool:?}, pd6: {=bool:?}, pd7: {=bool:?}, pd8: {=bool:?}, pd9: {=bool:?}, pd10: {=bool:?}, pd11: {=bool:?}, pd12: {=bool:?}, pd13: {=bool:?}, pd14: {=bool:?}, pd15: {=bool:?} }}" , self . pd0 () , self . pd1 () , self . pd2 () , self . pd3 () , self . pd5 () , self . pd6 () , self . pd7 () , self . pd8 () , self . pd9 () , self . pd10 () , self . pd11 () , self . pd12 () , self . pd13 () , self . pd14 () , self . pd15 ())
        }
    }
    #[doc = "PWR port C pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrc(pub u32);
    impl Pdcrc {
        #[doc = "When set, each bit activates the pull-down on PC0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-down on PC15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PC15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pdcrc {
        #[inline(always)]
        fn default() -> Pdcrc {
            Pdcrc(0)
        }
    }
    impl core::fmt::Debug for Pdcrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcrc")
                .field("pd0", &self.pd0())
                .field("pd1", &self.pd1())
                .field("pd2", &self.pd2())
                .field("pd3", &self.pd3())
                .field("pd4", &self.pd4())
                .field("pd5", &self.pd5())
                .field("pd6", &self.pd6())
                .field("pd7", &self.pd7())
                .field("pd8", &self.pd8())
                .field("pd9", &self.pd9())
                .field("pd10", &self.pd10())
                .field("pd11", &self.pd11())
                .field("pd12", &self.pd12())
                .field("pd13", &self.pd13())
                .field("pd14", &self.pd14())
                .field("pd15", &self.pd15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcrc {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pdcrc {{ pd0: {=bool:?}, pd1: {=bool:?}, pd2: {=bool:?}, pd3: {=bool:?}, pd4: {=bool:?}, pd5: {=bool:?}, pd6: {=bool:?}, pd7: {=bool:?}, pd8: {=bool:?}, pd9: {=bool:?}, pd10: {=bool:?}, pd11: {=bool:?}, pd12: {=bool:?}, pd13: {=bool:?}, pd14: {=bool:?}, pd15: {=bool:?} }}" , self . pd0 () , self . pd1 () , self . pd2 () , self . pd3 () , self . pd4 () , self . pd5 () , self . pd6 () , self . pd7 () , self . pd8 () , self . pd9 () , self . pd10 () , self . pd11 () , self . pd12 () , self . pd13 () , self . pd14 () , self . pd15 ())
        }
    }
    #[doc = "PWR port D pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrd(pub u32);
    impl Pdcrd {
        #[doc = "When set, each bit activates the pull-down on PD0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-down on PD15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PD15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pdcrd {
        #[inline(always)]
        fn default() -> Pdcrd {
            Pdcrd(0)
        }
    }
    impl core::fmt::Debug for Pdcrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcrd")
                .field("pd0", &self.pd0())
                .field("pd1", &self.pd1())
                .field("pd2", &self.pd2())
                .field("pd3", &self.pd3())
                .field("pd4", &self.pd4())
                .field("pd5", &self.pd5())
                .field("pd6", &self.pd6())
                .field("pd7", &self.pd7())
                .field("pd8", &self.pd8())
                .field("pd9", &self.pd9())
                .field("pd10", &self.pd10())
                .field("pd11", &self.pd11())
                .field("pd12", &self.pd12())
                .field("pd13", &self.pd13())
                .field("pd14", &self.pd14())
                .field("pd15", &self.pd15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcrd {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pdcrd {{ pd0: {=bool:?}, pd1: {=bool:?}, pd2: {=bool:?}, pd3: {=bool:?}, pd4: {=bool:?}, pd5: {=bool:?}, pd6: {=bool:?}, pd7: {=bool:?}, pd8: {=bool:?}, pd9: {=bool:?}, pd10: {=bool:?}, pd11: {=bool:?}, pd12: {=bool:?}, pd13: {=bool:?}, pd14: {=bool:?}, pd15: {=bool:?} }}" , self . pd0 () , self . pd1 () , self . pd2 () , self . pd3 () , self . pd4 () , self . pd5 () , self . pd6 () , self . pd7 () , self . pd8 () , self . pd9 () , self . pd10 () , self . pd11 () , self . pd12 () , self . pd13 () , self . pd14 () , self . pd15 ())
        }
    }
    #[doc = "PWR port E pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcre(pub u32);
    impl Pdcre {
        #[doc = "When set, each bit activates the pull-down on PE0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-down on PE15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PE15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pdcre {
        #[inline(always)]
        fn default() -> Pdcre {
            Pdcre(0)
        }
    }
    impl core::fmt::Debug for Pdcre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcre")
                .field("pd0", &self.pd0())
                .field("pd1", &self.pd1())
                .field("pd2", &self.pd2())
                .field("pd3", &self.pd3())
                .field("pd4", &self.pd4())
                .field("pd5", &self.pd5())
                .field("pd6", &self.pd6())
                .field("pd7", &self.pd7())
                .field("pd8", &self.pd8())
                .field("pd9", &self.pd9())
                .field("pd10", &self.pd10())
                .field("pd11", &self.pd11())
                .field("pd12", &self.pd12())
                .field("pd13", &self.pd13())
                .field("pd14", &self.pd14())
                .field("pd15", &self.pd15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcre {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pdcre {{ pd0: {=bool:?}, pd1: {=bool:?}, pd2: {=bool:?}, pd3: {=bool:?}, pd4: {=bool:?}, pd5: {=bool:?}, pd6: {=bool:?}, pd7: {=bool:?}, pd8: {=bool:?}, pd9: {=bool:?}, pd10: {=bool:?}, pd11: {=bool:?}, pd12: {=bool:?}, pd13: {=bool:?}, pd14: {=bool:?}, pd15: {=bool:?} }}" , self . pd0 () , self . pd1 () , self . pd2 () , self . pd3 () , self . pd4 () , self . pd5 () , self . pd6 () , self . pd7 () , self . pd8 () , self . pd9 () , self . pd10 () , self . pd11 () , self . pd12 () , self . pd13 () , self . pd14 () , self . pd15 ())
        }
    }
    #[doc = "PWR port G pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrg(pub u32);
    impl Pdcrg {
        #[doc = "When set, each bit activates the pull-down on PG2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-down on PG15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PG15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pdcrg {
        #[inline(always)]
        fn default() -> Pdcrg {
            Pdcrg(0)
        }
    }
    impl core::fmt::Debug for Pdcrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcrg")
                .field("pd2", &self.pd2())
                .field("pd3", &self.pd3())
                .field("pd4", &self.pd4())
                .field("pd5", &self.pd5())
                .field("pd6", &self.pd6())
                .field("pd7", &self.pd7())
                .field("pd8", &self.pd8())
                .field("pd9", &self.pd9())
                .field("pd10", &self.pd10())
                .field("pd11", &self.pd11())
                .field("pd12", &self.pd12())
                .field("pd13", &self.pd13())
                .field("pd14", &self.pd14())
                .field("pd15", &self.pd15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcrg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pdcrg {{ pd2: {=bool:?}, pd3: {=bool:?}, pd4: {=bool:?}, pd5: {=bool:?}, pd6: {=bool:?}, pd7: {=bool:?}, pd8: {=bool:?}, pd9: {=bool:?}, pd10: {=bool:?}, pd11: {=bool:?}, pd12: {=bool:?}, pd13: {=bool:?}, pd14: {=bool:?}, pd15: {=bool:?} }}" , self . pd2 () , self . pd3 () , self . pd4 () , self . pd5 () , self . pd6 () , self . pd7 () , self . pd8 () , self . pd9 () , self . pd10 () , self . pd11 () , self . pd12 () , self . pd13 () , self . pd14 () , self . pd15 ())
        }
    }
    #[doc = "PWR port H pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrh(pub u32);
    impl Pdcrh {
        #[doc = "When set, each bit activates the pull-down on PH0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PH0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-down on PH1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PH1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-down on PH3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-down on PH3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Pdcrh {
        #[inline(always)]
        fn default() -> Pdcrh {
            Pdcrh(0)
        }
    }
    impl core::fmt::Debug for Pdcrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcrh")
                .field("pd0", &self.pd0())
                .field("pd1", &self.pd1())
                .field("pd3", &self.pd3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcrh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdcrh {{ pd0: {=bool:?}, pd1: {=bool:?}, pd3: {=bool:?} }}",
                self.pd0(),
                self.pd1(),
                self.pd3()
            )
        }
    }
    #[doc = "PWR privilege control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "This bit is set and reset by software. It can be written only by a secure privileged access. 0: Read and write to PWR secure functions can be done by privileged or unprivileged access. 1: Read and write to PWR secure functions can be done by privileged access only."]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set and reset by software. It can be written only by a secure privileged access. 0: Read and write to PWR secure functions can be done by privileged or unprivileged access. 1: Read and write to PWR secure functions can be done by privileged access only."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This bit is set and reset by software. It can be written only by privileged access, secure or non-secure. 0: Read and write to PWR non-secure functions can be done by privileged or unprivileged access. 1: Read and write to PWR non-secure functions can be done by privileged access only."]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set and reset by software. It can be written only by privileged access, secure or non-secure. 0: Read and write to PWR non-secure functions can be done by privileged or unprivileged access. 1: Read and write to PWR non-secure functions can be done by privileged access only."]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("spriv", &self.spriv())
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {=bool:?}, nspriv: {=bool:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[doc = "PWR port A pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucra(pub u32);
    impl Pucra {
        #[doc = "When set, each bit activates the pull-up on PA0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-up on PA15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PA15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucra {
        #[inline(always)]
        fn default() -> Pucra {
            Pucra(0)
        }
    }
    impl core::fmt::Debug for Pucra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucra")
                .field("pu0", &self.pu0())
                .field("pu1", &self.pu1())
                .field("pu2", &self.pu2())
                .field("pu3", &self.pu3())
                .field("pu4", &self.pu4())
                .field("pu5", &self.pu5())
                .field("pu6", &self.pu6())
                .field("pu7", &self.pu7())
                .field("pu8", &self.pu8())
                .field("pu9", &self.pu9())
                .field("pu10", &self.pu10())
                .field("pu11", &self.pu11())
                .field("pu12", &self.pu12())
                .field("pu13", &self.pu13())
                .field("pu15", &self.pu15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucra {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pucra {{ pu0: {=bool:?}, pu1: {=bool:?}, pu2: {=bool:?}, pu3: {=bool:?}, pu4: {=bool:?}, pu5: {=bool:?}, pu6: {=bool:?}, pu7: {=bool:?}, pu8: {=bool:?}, pu9: {=bool:?}, pu10: {=bool:?}, pu11: {=bool:?}, pu12: {=bool:?}, pu13: {=bool:?}, pu15: {=bool:?} }}" , self . pu0 () , self . pu1 () , self . pu2 () , self . pu3 () , self . pu4 () , self . pu5 () , self . pu6 () , self . pu7 () , self . pu8 () , self . pu9 () , self . pu10 () , self . pu11 () , self . pu12 () , self . pu13 () , self . pu15 ())
        }
    }
    #[doc = "PWR port B pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrb(pub u32);
    impl Pucrb {
        #[doc = "When set, each bit activates the pull-up on PB0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-up on PB15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PB15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucrb {
        #[inline(always)]
        fn default() -> Pucrb {
            Pucrb(0)
        }
    }
    impl core::fmt::Debug for Pucrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucrb")
                .field("pu0", &self.pu0())
                .field("pu1", &self.pu1())
                .field("pu2", &self.pu2())
                .field("pu3", &self.pu3())
                .field("pu4", &self.pu4())
                .field("pu5", &self.pu5())
                .field("pu6", &self.pu6())
                .field("pu7", &self.pu7())
                .field("pu8", &self.pu8())
                .field("pu9", &self.pu9())
                .field("pu10", &self.pu10())
                .field("pu11", &self.pu11())
                .field("pu12", &self.pu12())
                .field("pu13", &self.pu13())
                .field("pu14", &self.pu14())
                .field("pu15", &self.pu15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucrb {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pucrb {{ pu0: {=bool:?}, pu1: {=bool:?}, pu2: {=bool:?}, pu3: {=bool:?}, pu4: {=bool:?}, pu5: {=bool:?}, pu6: {=bool:?}, pu7: {=bool:?}, pu8: {=bool:?}, pu9: {=bool:?}, pu10: {=bool:?}, pu11: {=bool:?}, pu12: {=bool:?}, pu13: {=bool:?}, pu14: {=bool:?}, pu15: {=bool:?} }}" , self . pu0 () , self . pu1 () , self . pu2 () , self . pu3 () , self . pu4 () , self . pu5 () , self . pu6 () , self . pu7 () , self . pu8 () , self . pu9 () , self . pu10 () , self . pu11 () , self . pu12 () , self . pu13 () , self . pu14 () , self . pu15 ())
        }
    }
    #[doc = "PWR port C pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrc(pub u32);
    impl Pucrc {
        #[doc = "When set, each bit activates the pull-up on PC0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-up on PC15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PC15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucrc {
        #[inline(always)]
        fn default() -> Pucrc {
            Pucrc(0)
        }
    }
    impl core::fmt::Debug for Pucrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucrc")
                .field("pu0", &self.pu0())
                .field("pu1", &self.pu1())
                .field("pu2", &self.pu2())
                .field("pu3", &self.pu3())
                .field("pu4", &self.pu4())
                .field("pu5", &self.pu5())
                .field("pu6", &self.pu6())
                .field("pu7", &self.pu7())
                .field("pu8", &self.pu8())
                .field("pu9", &self.pu9())
                .field("pu10", &self.pu10())
                .field("pu11", &self.pu11())
                .field("pu12", &self.pu12())
                .field("pu13", &self.pu13())
                .field("pu14", &self.pu14())
                .field("pu15", &self.pu15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucrc {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pucrc {{ pu0: {=bool:?}, pu1: {=bool:?}, pu2: {=bool:?}, pu3: {=bool:?}, pu4: {=bool:?}, pu5: {=bool:?}, pu6: {=bool:?}, pu7: {=bool:?}, pu8: {=bool:?}, pu9: {=bool:?}, pu10: {=bool:?}, pu11: {=bool:?}, pu12: {=bool:?}, pu13: {=bool:?}, pu14: {=bool:?}, pu15: {=bool:?} }}" , self . pu0 () , self . pu1 () , self . pu2 () , self . pu3 () , self . pu4 () , self . pu5 () , self . pu6 () , self . pu7 () , self . pu8 () , self . pu9 () , self . pu10 () , self . pu11 () , self . pu12 () , self . pu13 () , self . pu14 () , self . pu15 ())
        }
    }
    #[doc = "PWR port D pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrd(pub u32);
    impl Pucrd {
        #[doc = "When set, each bit activates the pull-up on PD0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-up on PD15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PD15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucrd {
        #[inline(always)]
        fn default() -> Pucrd {
            Pucrd(0)
        }
    }
    impl core::fmt::Debug for Pucrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucrd")
                .field("pu0", &self.pu0())
                .field("pu1", &self.pu1())
                .field("pu2", &self.pu2())
                .field("pu3", &self.pu3())
                .field("pu4", &self.pu4())
                .field("pu5", &self.pu5())
                .field("pu6", &self.pu6())
                .field("pu7", &self.pu7())
                .field("pu8", &self.pu8())
                .field("pu9", &self.pu9())
                .field("pu10", &self.pu10())
                .field("pu11", &self.pu11())
                .field("pu12", &self.pu12())
                .field("pu13", &self.pu13())
                .field("pu14", &self.pu14())
                .field("pu15", &self.pu15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucrd {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pucrd {{ pu0: {=bool:?}, pu1: {=bool:?}, pu2: {=bool:?}, pu3: {=bool:?}, pu4: {=bool:?}, pu5: {=bool:?}, pu6: {=bool:?}, pu7: {=bool:?}, pu8: {=bool:?}, pu9: {=bool:?}, pu10: {=bool:?}, pu11: {=bool:?}, pu12: {=bool:?}, pu13: {=bool:?}, pu14: {=bool:?}, pu15: {=bool:?} }}" , self . pu0 () , self . pu1 () , self . pu2 () , self . pu3 () , self . pu4 () , self . pu5 () , self . pu6 () , self . pu7 () , self . pu8 () , self . pu9 () , self . pu10 () , self . pu11 () , self . pu12 () , self . pu13 () , self . pu14 () , self . pu15 ())
        }
    }
    #[doc = "PWR port E pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucre(pub u32);
    impl Pucre {
        #[doc = "When set, each bit activates the pull-up on PE0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-up on PE15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PE15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucre {
        #[inline(always)]
        fn default() -> Pucre {
            Pucre(0)
        }
    }
    impl core::fmt::Debug for Pucre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucre")
                .field("pu0", &self.pu0())
                .field("pu1", &self.pu1())
                .field("pu2", &self.pu2())
                .field("pu3", &self.pu3())
                .field("pu4", &self.pu4())
                .field("pu5", &self.pu5())
                .field("pu6", &self.pu6())
                .field("pu7", &self.pu7())
                .field("pu8", &self.pu8())
                .field("pu9", &self.pu9())
                .field("pu10", &self.pu10())
                .field("pu11", &self.pu11())
                .field("pu12", &self.pu12())
                .field("pu13", &self.pu13())
                .field("pu14", &self.pu14())
                .field("pu15", &self.pu15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucre {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pucre {{ pu0: {=bool:?}, pu1: {=bool:?}, pu2: {=bool:?}, pu3: {=bool:?}, pu4: {=bool:?}, pu5: {=bool:?}, pu6: {=bool:?}, pu7: {=bool:?}, pu8: {=bool:?}, pu9: {=bool:?}, pu10: {=bool:?}, pu11: {=bool:?}, pu12: {=bool:?}, pu13: {=bool:?}, pu14: {=bool:?}, pu15: {=bool:?} }}" , self . pu0 () , self . pu1 () , self . pu2 () , self . pu3 () , self . pu4 () , self . pu5 () , self . pu6 () , self . pu7 () , self . pu8 () , self . pu9 () , self . pu10 () , self . pu11 () , self . pu12 () , self . pu13 () , self . pu14 () , self . pu15 ())
        }
    }
    #[doc = "PWR port G pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrg(pub u32);
    impl Pucrg {
        #[doc = "When set, each bit activates the pull-up on PG2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "When set, each bit activates the pull-up on PG15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PG15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucrg {
        #[inline(always)]
        fn default() -> Pucrg {
            Pucrg(0)
        }
    }
    impl core::fmt::Debug for Pucrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucrg")
                .field("pu2", &self.pu2())
                .field("pu3", &self.pu3())
                .field("pu4", &self.pu4())
                .field("pu5", &self.pu5())
                .field("pu6", &self.pu6())
                .field("pu7", &self.pu7())
                .field("pu8", &self.pu8())
                .field("pu9", &self.pu9())
                .field("pu10", &self.pu10())
                .field("pu11", &self.pu11())
                .field("pu12", &self.pu12())
                .field("pu13", &self.pu13())
                .field("pu14", &self.pu14())
                .field("pu15", &self.pu15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucrg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pucrg {{ pu2: {=bool:?}, pu3: {=bool:?}, pu4: {=bool:?}, pu5: {=bool:?}, pu6: {=bool:?}, pu7: {=bool:?}, pu8: {=bool:?}, pu9: {=bool:?}, pu10: {=bool:?}, pu11: {=bool:?}, pu12: {=bool:?}, pu13: {=bool:?}, pu14: {=bool:?}, pu15: {=bool:?} }}" , self . pu2 () , self . pu3 () , self . pu4 () , self . pu5 () , self . pu6 () , self . pu7 () , self . pu8 () , self . pu9 () , self . pu10 () , self . pu11 () , self . pu12 () , self . pu13 () , self . pu14 () , self . pu15 ())
        }
    }
    #[doc = "PWR port H pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrh(pub u32);
    impl Pucrh {
        #[doc = "When set, each bit activates the pull-up on PH0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PH0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, each bit activates the pull-up on PH1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PH1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When set, each bit activates the pull-up on PH3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When set, each bit activates the pull-up on PH3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Pucrh {
        #[inline(always)]
        fn default() -> Pucrh {
            Pucrh(0)
        }
    }
    impl core::fmt::Debug for Pucrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucrh")
                .field("pu0", &self.pu0())
                .field("pu1", &self.pu1())
                .field("pu3", &self.pu3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucrh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pucrh {{ pu0: {=bool:?}, pu1: {=bool:?}, pu3: {=bool:?} }}",
                self.pu0(),
                self.pu1(),
                self.pu3()
            )
        }
    }
    #[doc = "PWR security configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "None 0: Bits related to the WKUP1 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP1 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup1sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP1 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP1 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "None 0: Bits related to the WKUP2 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP2 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup2sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP2 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP2 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "None 0: Bits related to the WKUP3 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP3 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup3sec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP3 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP3 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "None 0: Bits related to the WKUP4 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP4 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup4sec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP4 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP4 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "None 0: Bits related to the WKUP5 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP5 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup5sec(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP5 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP5 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup5sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "None 0: Bits related to the WKUP6 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP6 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup6sec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP6 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP6 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup6sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "None 0: Bits related to the WKUP7 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP7 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup7sec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP7 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP7 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup7sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "None 0: Bits related to the WKUP8 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP8 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup8sec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP8 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP8 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup8sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "None 0: Bits related to the WKUP9 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP9 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup9sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP9 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP9 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup9sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "None 0: Bits related to the WKUP10 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP10 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn wup10sec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Bits related to the WKUP10 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP10 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_wup10sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "None 0: CR1, CR2 and CSSF in the SR can be read and written with secure or non-secure access. 1: CR1, CR2, and CSSF in the SR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn lpmsec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: CR1, CR2 and CSSF in the SR can be read and written with secure or non-secure access. 1: CR1, CR2, and CSSF in the SR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_lpmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "None 0: SVMCR and CR3 can be read and written with secure or non-secure access. 1: SVMCR and CR3 can be read and written only with secure access."]
        #[inline(always)]
        pub const fn vdmsec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: SVMCR and CR3 can be read and written with secure or non-secure access. 1: SVMCR and CR3 can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_vdmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "None 0: BDCR and DBPR can be read and written with secure or non-secure access. 1: BDCR and DBPR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn vbsec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: BDCR and DBPR can be read and written with secure or non-secure access. 1: BDCR and DBPR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_vbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "None 0: APCR can be read and written with secure or non-secure access. 1: APCR can be read and written only with secure access."]
        #[inline(always)]
        pub const fn apcsec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: APCR can be read and written with secure or non-secure access. 1: APCR can be read and written only with secure access."]
        #[inline(always)]
        pub fn set_apcsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("wup1sec", &self.wup1sec())
                .field("wup2sec", &self.wup2sec())
                .field("wup3sec", &self.wup3sec())
                .field("wup4sec", &self.wup4sec())
                .field("wup5sec", &self.wup5sec())
                .field("wup6sec", &self.wup6sec())
                .field("wup7sec", &self.wup7sec())
                .field("wup8sec", &self.wup8sec())
                .field("wup9sec", &self.wup9sec())
                .field("wup10sec", &self.wup10sec())
                .field("lpmsec", &self.lpmsec())
                .field("vdmsec", &self.vdmsec())
                .field("vbsec", &self.vbsec())
                .field("apcsec", &self.apcsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr {{ wup1sec: {=bool:?}, wup2sec: {=bool:?}, wup3sec: {=bool:?}, wup4sec: {=bool:?}, wup5sec: {=bool:?}, wup6sec: {=bool:?}, wup7sec: {=bool:?}, wup8sec: {=bool:?}, wup9sec: {=bool:?}, wup10sec: {=bool:?}, lpmsec: {=bool:?}, vdmsec: {=bool:?}, vbsec: {=bool:?}, apcsec: {=bool:?} }}" , self . wup1sec () , self . wup2sec () , self . wup3sec () , self . wup4sec () , self . wup5sec () , self . wup6sec () , self . wup7sec () , self . wup8sec () , self . wup9sec () , self . wup10sec () , self . lpmsec () , self . vdmsec () , self . vbsec () , self . apcsec ())
        }
    }
    #[doc = "PWR status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "This bit is protected against non-secure access when LPMSEC=1 in SECCFGR. This bit is protected against unprivileged access when LPMSEC=1 and SPRIV=1 in PRIVCFGR, or when LPMSEC=0 and NSPRIV=1. Writing 1 to this bit clears the STOPF and SBF flags."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is protected against non-secure access when LPMSEC=1 in SECCFGR. This bit is protected against unprivileged access when LPMSEC=1 and SPRIV=1 in PRIVCFGR, or when LPMSEC=0 and NSPRIV=1. Writing 1 to this bit clears the STOPF and SBF flags."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit. 0: The device did not enter any Stop mode. 1: The device entered a Stop mode."]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit. 0: The device did not enter any Stop mode. 1: The device entered a Stop mode."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset. 0: The device did not enter Standby mode. 1: The device entered Standby mode."]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset. 0: The device did not enter Standby mode. 1: The device entered Standby mode."]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
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
                .field("cssf", &self.cssf())
                .field("stopf", &self.stopf())
                .field("sbf", &self.sbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ cssf: {=bool:?}, stopf: {=bool:?}, sbf: {=bool:?} }}",
                self.cssf(),
                self.stopf(),
                self.sbf()
            )
        }
    }
    #[doc = "PWR supply voltage monitoring control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmcr(pub u32);
    impl Svmcr {
        #[doc = "None 0: PVD disabled 1: PVD enabled."]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: PVD disabled 1: PVD enabled."]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "These bits select the voltage threshold detected by the PVD: 000: VPVD0 around 2.0V 001: VPVD1 around 2.2V 010: VPVD2 around 2.4V 011: VPVD3 around 2.5V 100: VPVD4 around 2.6V 101: VPVD5 around 2.8V 110: VPVD6 around 2.9V 111: External input analog voltage PVD_IN (compared internally to VREFINT)."]
        #[inline(always)]
        pub const fn pvdls(&self) -> super::vals::Pvdls {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Pvdls::from_bits(val as u8)
        }
        #[doc = "These bits select the voltage threshold detected by the PVD: 000: VPVD0 around 2.0V 001: VPVD1 around 2.2V 010: VPVD2 around 2.4V 011: VPVD3 around 2.5V 100: VPVD4 around 2.6V 101: VPVD5 around 2.8V 110: VPVD6 around 2.9V 111: External input analog voltage PVD_IN (compared internally to VREFINT)."]
        #[inline(always)]
        pub fn set_pvdls(&mut self, val: super::vals::Pvdls) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "None 0: VDDUSB voltage monitor disabled 1: VDDUSB voltage monitor enabled."]
        #[inline(always)]
        pub const fn uvmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VDDUSB voltage monitor disabled 1: VDDUSB voltage monitor enabled."]
        #[inline(always)]
        pub fn set_uvmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "None 0: VDDIO2 voltage monitor disabled 1: VDDIO2 voltage monitor enabled."]
        #[inline(always)]
        pub const fn io2vmen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VDDIO2 voltage monitor disabled 1: VDDIO2 voltage monitor enabled."]
        #[inline(always)]
        pub fn set_io2vmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "None 0: VDDA voltage monitor 1 disabled 1: VDDA voltage monitor 1 enabled."]
        #[inline(always)]
        pub const fn avm1en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VDDA voltage monitor 1 disabled 1: VDDA voltage monitor 1 enabled."]
        #[inline(always)]
        pub fn set_avm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "None 0: VDDA voltage monitor 2 disabled 1: VDDA voltage monitor 2 enabled."]
        #[inline(always)]
        pub const fn avm2en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VDDA voltage monitor 2 disabled 1: VDDA voltage monitor 2 enabled."]
        #[inline(always)]
        pub fn set_avm2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB peripheral. If VDDUSB is not always present in the application, the VDDUSB voltage monitor can be used to determine whether this supply is ready or not. 0: VDDUSB not present: logical and electrical isolation is applied to ignore this supply. 1: VDDUSB valid."]
        #[inline(always)]
        pub const fn usv(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB peripheral. If VDDUSB is not always present in the application, the VDDUSB voltage monitor can be used to determine whether this supply is ready or not. 0: VDDUSB not present: logical and electrical isolation is applied to ignore this supply. 1: VDDUSB valid."]
        #[inline(always)]
        pub fn set_usv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\\[15:2\\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not. 0: VDDIO2 not present: logical and electrical isolation is applied to ignore this supply. 1: VDDIO2 valid."]
        #[inline(always)]
        pub const fn io2sv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\\[15:2\\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not. 0: VDDIO2 not present: logical and electrical isolation is applied to ignore this supply. 1: VDDIO2 valid."]
        #[inline(always)]
        pub fn set_io2sv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "This bit is used to validate the VDDA supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in the application, the VDDA voltage monitor can be used to determine whether this supply is ready or not. 0: VDDA not present: logical and electrical isolation is applied to ignore this supply. 1: VDDA valid."]
        #[inline(always)]
        pub const fn asv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is used to validate the VDDA supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in the application, the VDDA voltage monitor can be used to determine whether this supply is ready or not. 0: VDDA not present: logical and electrical isolation is applied to ignore this supply. 1: VDDA valid."]
        #[inline(always)]
        pub fn set_asv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Svmcr {
        #[inline(always)]
        fn default() -> Svmcr {
            Svmcr(0)
        }
    }
    impl core::fmt::Debug for Svmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Svmcr")
                .field("pvde", &self.pvde())
                .field("pvdls", &self.pvdls())
                .field("uvmen", &self.uvmen())
                .field("io2vmen", &self.io2vmen())
                .field("avm1en", &self.avm1en())
                .field("avm2en", &self.avm2en())
                .field("usv", &self.usv())
                .field("io2sv", &self.io2sv())
                .field("asv", &self.asv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Svmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Svmcr {{ pvde: {=bool:?}, pvdls: {:?}, uvmen: {=bool:?}, io2vmen: {=bool:?}, avm1en: {=bool:?}, avm2en: {=bool:?}, usv: {=bool:?}, io2sv: {=bool:?}, asv: {=bool:?} }}" , self . pvde () , self . pvdls () , self . uvmen () , self . io2vmen () , self . avm1en () , self . avm2en () , self . usv () , self . io2sv () , self . asv ())
        }
    }
    #[doc = "PWR supply voltage monitoring status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmsr(pub u32);
    impl Svmsr {
        #[doc = "None 0: LDO selected 1: SMPS selected."]
        #[inline(always)]
        pub const fn regs(&self) -> super::vals::Regsel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Regsel::from_bits(val as u8)
        }
        #[doc = "None 0: LDO selected 1: SMPS selected."]
        #[inline(always)]
        pub fn set_regs(&mut self, val: super::vals::Regsel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "None 0: VDD is equal or above the PVD threshold selected by PVDLS\\[2:0\\]. 1: VDD is below the PVD threshold selected by PVDLS\\[2:0\\]."]
        #[inline(always)]
        pub const fn pvdo(&self) -> super::vals::Pvdo {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Pvdo::from_bits(val as u8)
        }
        #[doc = "None 0: VDD is equal or above the PVD threshold selected by PVDLS\\[2:0\\]. 1: VDD is below the PVD threshold selected by PVDLS\\[2:0\\]."]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: super::vals::Pvdo) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "None 0: VDDUSB is below the threshold of the VDDUSB voltage monitor. 1: VDDUSB is equal or above the threshold of the VDDUSB voltage monitor."]
        #[inline(always)]
        pub const fn vddusbrdy(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VDDUSB is below the threshold of the VDDUSB voltage monitor. 1: VDDUSB is equal or above the threshold of the VDDUSB voltage monitor."]
        #[inline(always)]
        pub fn set_vddusbrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "None 0: VDDIO2 is below the threshold of the VDDIO2 voltage monitor. 1: VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor."]
        #[inline(always)]
        pub const fn vddio2rdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VDDIO2 is below the threshold of the VDDIO2 voltage monitor. 1: VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor."]
        #[inline(always)]
        pub fn set_vddio2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "None 0: VDDA is below the threshold of the VDDA voltage monitor 1 (around 1.6V). 1: VDDA is equal or above the threshold of the VDDA voltage monitor 1 (around 1.6V)."]
        #[inline(always)]
        pub const fn vdda1rdy(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VDDA is below the threshold of the VDDA voltage monitor 1 (around 1.6V). 1: VDDA is equal or above the threshold of the VDDA voltage monitor 1 (around 1.6V)."]
        #[inline(always)]
        pub fn set_vdda1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "None 0: VDDA is below the threshold of the VDDA voltage monitor 2 (around 1.8V). 1: VDDA is equal or above the threshold of the VDDA voltage monitor 2 (around 1.8V)."]
        #[inline(always)]
        pub const fn vdda2rdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: VDDA is below the threshold of the VDDA voltage monitor 2 (around 1.8V). 1: VDDA is equal or above the threshold of the VDDA voltage monitor 2 (around 1.8V)."]
        #[inline(always)]
        pub fn set_vdda2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Svmsr {
        #[inline(always)]
        fn default() -> Svmsr {
            Svmsr(0)
        }
    }
    impl core::fmt::Debug for Svmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Svmsr")
                .field("regs", &self.regs())
                .field("pvdo", &self.pvdo())
                .field("vddusbrdy", &self.vddusbrdy())
                .field("vddio2rdy", &self.vddio2rdy())
                .field("vdda1rdy", &self.vdda1rdy())
                .field("vdda2rdy", &self.vdda2rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Svmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Svmsr {{ regs: {:?}, pvdo: {:?}, vddusbrdy: {=bool:?}, vddio2rdy: {=bool:?}, vdda1rdy: {=bool:?}, vdda2rdy: {=bool:?} }}" , self . regs () , self . pvdo () , self . vddusbrdy () , self . vddio2rdy () , self . vdda1rdy () , self . vdda2rdy ())
        }
    }
    #[doc = "PWR voltage scaling register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vosr(pub u32);
    impl Vosr {
        #[doc = "This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. 0: Voltage scaling range 1 disabled 1: Voltage scaling range 1 enabled Note: R1EN and R2EN must be at opposite value. Any attempt to write R1EN and R2EN to same value is ignored. Modifying R1EN and R2EN is possible only when current range is ready (R1RDY=R1EN and R2RDY=R2EN)."]
        #[inline(always)]
        pub const fn r1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. 0: Voltage scaling range 1 disabled 1: Voltage scaling range 1 enabled Note: R1EN and R2EN must be at opposite value. Any attempt to write R1EN and R2EN to same value is ignored. Modifying R1EN and R2EN is possible only when current range is ready (R1RDY=R1EN and R2RDY=R2EN)."]
        #[inline(always)]
        pub fn set_r1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. 0: Voltage scaling range 2 disabled 1: Voltage scaling range 2 enabled Note: R1EN and R2EN must be at opposite value. Any attempt to write R1EN and R2EN to same value is ignored. Modifying R1EN and R2EN is possible only when current range is ready (R1RDY=R1EN and R2RDY=R2EN)."]
        #[inline(always)]
        pub const fn r2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. 0: Voltage scaling range 2 disabled 1: Voltage scaling range 2 enabled Note: R1EN and R2EN must be at opposite value. Any attempt to write R1EN and R2EN to same value is ignored. Modifying R1EN and R2EN is possible only when current range is ready (R1RDY=R1EN and R2RDY=R2EN)."]
        #[inline(always)]
        pub fn set_r2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bit is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. This bit must be set in Range 1, and before increasing the system clock frequency above 24 MHz in Range 2. The booster clock must be configured before setting this bit, and must not be disabled as long as the booster is enabled. 0: Booster disabled 1: Booster enabled."]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. This bit must be set in Range 1, and before increasing the system clock frequency above 24 MHz in Range 2. The booster clock must be configured before setting this bit, and must not be disabled as long as the booster is enabled. 0: Booster disabled 1: Booster enabled."]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "None 0: Range 1 not ready: voltage level less than VOS range 1 level 1: Range 1 ready: voltage level greater or equal VOS range 1 level Note: R1RDY and R2RDY cannot be set at the same time."]
        #[inline(always)]
        pub const fn r1rdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Range 1 not ready: voltage level less than VOS range 1 level 1: Range 1 ready: voltage level greater or equal VOS range 1 level Note: R1RDY and R2RDY cannot be set at the same time."]
        #[inline(always)]
        pub fn set_r1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "None 0: Range 2 not ready: voltage level less than VOS range 2 level 1: Range 2 ready: voltage level greater or equal VOS range 2 level Note: R1RDY and R2RDY cannot be set at the same time."]
        #[inline(always)]
        pub const fn r2rdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Range 2 not ready: voltage level less than VOS range 2 level 1: Range 2 ready: voltage level greater or equal VOS range 2 level Note: R1RDY and R2RDY cannot be set at the same time."]
        #[inline(always)]
        pub fn set_r2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 24 MHz only after this bit is set. Disabling the booster clock when the booster is ready is forbidden. 0: Power booster not ready 1: Power booster ready."]
        #[inline(always)]
        pub const fn boostrdy(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 24 MHz only after this bit is set. Disabling the booster clock when the booster is ready is forbidden. 0: Power booster not ready 1: Power booster ready."]
        #[inline(always)]
        pub fn set_boostrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Vosr {
        #[inline(always)]
        fn default() -> Vosr {
            Vosr(0)
        }
    }
    impl core::fmt::Debug for Vosr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vosr")
                .field("r1en", &self.r1en())
                .field("r2en", &self.r2en())
                .field("boosten", &self.boosten())
                .field("r1rdy", &self.r1rdy())
                .field("r2rdy", &self.r2rdy())
                .field("boostrdy", &self.boostrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vosr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vosr {{ r1en: {=bool:?}, r2en: {=bool:?}, boosten: {=bool:?}, r1rdy: {=bool:?}, r2rdy: {=bool:?}, boostrdy: {=bool:?} }}" , self . r1en () , self . r2en () , self . boosten () , self . r1rdy () , self . r2rdy () , self . boostrdy ())
        }
    }
    #[doc = "PWR wakeup control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr1(pub u32);
    impl Wucr1 {
        #[doc = "None 0: Wakeup line WKUP1 disabled 1: Wakeup line WKUP1 enabled."]
        #[inline(always)]
        pub const fn wupen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP1 disabled 1: Wakeup line WKUP1 enabled."]
        #[inline(always)]
        pub fn set_wupen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "None 0: Wakeup line WKUP2 disabled 1: Wakeup line WKUP2 enabled."]
        #[inline(always)]
        pub const fn wupen2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP2 disabled 1: Wakeup line WKUP2 enabled."]
        #[inline(always)]
        pub fn set_wupen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "None 0: Wakeup line WKUP3 disabled 1: Wakeup line WKUP3 enabled."]
        #[inline(always)]
        pub const fn wupen3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP3 disabled 1: Wakeup line WKUP3 enabled."]
        #[inline(always)]
        pub fn set_wupen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "None 0: Wakeup line WKUP4 disabled 1: Wakeup line WKUP4 enabled."]
        #[inline(always)]
        pub const fn wupen4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP4 disabled 1: Wakeup line WKUP4 enabled."]
        #[inline(always)]
        pub fn set_wupen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "None 0: Wakeup line WKUP5 disabled 1: Wakeup line WKUP5 enabled."]
        #[inline(always)]
        pub const fn wupen5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP5 disabled 1: Wakeup line WKUP5 enabled."]
        #[inline(always)]
        pub fn set_wupen5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "None 0: Wakeup line WKUP6 disabled 1: Wakeup line WKUP6 enabled."]
        #[inline(always)]
        pub const fn wupen6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP6 disabled 1: Wakeup line WKUP6 enabled."]
        #[inline(always)]
        pub fn set_wupen6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "None 0: Wakeup line WKUP7 disabled 1: Wakeup line WKUP7 enabled."]
        #[inline(always)]
        pub const fn wupen7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP7 disabled 1: Wakeup line WKUP7 enabled."]
        #[inline(always)]
        pub fn set_wupen7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "None 0: Wakeup line WKUP8 disabled 1: Wakeup line WKUP8 enabled."]
        #[inline(always)]
        pub const fn wupen8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP8 disabled 1: Wakeup line WKUP8 enabled."]
        #[inline(always)]
        pub fn set_wupen8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "None 0: Wakeup line WKUP9 disabled 1: Wakeup line WKUP9 enabled."]
        #[inline(always)]
        pub const fn wupen9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP9 disabled 1: Wakeup line WKUP9 enabled."]
        #[inline(always)]
        pub fn set_wupen9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "None 0: Wakeup line WKUP10 disabled 1: Wakeup line WKUP10 enabled."]
        #[inline(always)]
        pub const fn wupen10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "None 0: Wakeup line WKUP10 disabled 1: Wakeup line WKUP10 enabled."]
        #[inline(always)]
        pub fn set_wupen10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Wucr1 {
        #[inline(always)]
        fn default() -> Wucr1 {
            Wucr1(0)
        }
    }
    impl core::fmt::Debug for Wucr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wucr1")
                .field("wupen1", &self.wupen1())
                .field("wupen2", &self.wupen2())
                .field("wupen3", &self.wupen3())
                .field("wupen4", &self.wupen4())
                .field("wupen5", &self.wupen5())
                .field("wupen6", &self.wupen6())
                .field("wupen7", &self.wupen7())
                .field("wupen8", &self.wupen8())
                .field("wupen9", &self.wupen9())
                .field("wupen10", &self.wupen10())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wucr1 {{ wupen1: {=bool:?}, wupen2: {=bool:?}, wupen3: {=bool:?}, wupen4: {=bool:?}, wupen5: {=bool:?}, wupen6: {=bool:?}, wupen7: {=bool:?}, wupen8: {=bool:?}, wupen9: {=bool:?}, wupen10: {=bool:?} }}" , self . wupen1 () , self . wupen2 () , self . wupen3 () , self . wupen4 () , self . wupen5 () , self . wupen6 () , self . wupen7 () , self . wupen8 () , self . wupen9 () , self . wupen10 ())
        }
    }
    #[doc = "PWR wakeup control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr2(pub u32);
    impl Wucr2 {
        #[doc = "This bit must be configured when WUPEN1 = 0. It has no effect when WUSEL1 = 11. 0: Detection on high level (rising edge) 1: Detection on low level (falling edge)."]
        #[inline(always)]
        pub const fn wupp(&self, n: usize) -> super::vals::Wupp {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "This bit must be configured when WUPEN1 = 0. It has no effect when WUSEL1 = 11. 0: Detection on high level (rising edge) 1: Detection on low level (falling edge)."]
        #[inline(always)]
        pub fn set_wupp(&mut self, n: usize, val: super::vals::Wupp) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Wucr2 {
        #[inline(always)]
        fn default() -> Wucr2 {
            Wucr2(0)
        }
    }
    impl core::fmt::Debug for Wucr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wucr2")
                .field("wupp[0]", &self.wupp(0usize))
                .field("wupp[1]", &self.wupp(1usize))
                .field("wupp[2]", &self.wupp(2usize))
                .field("wupp[3]", &self.wupp(3usize))
                .field("wupp[4]", &self.wupp(4usize))
                .field("wupp[5]", &self.wupp(5usize))
                .field("wupp[6]", &self.wupp(6usize))
                .field("wupp[7]", &self.wupp(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wucr2 {{ wupp[0]: {:?}, wupp[1]: {:?}, wupp[2]: {:?}, wupp[3]: {:?}, wupp[4]: {:?}, wupp[5]: {:?}, wupp[6]: {:?}, wupp[7]: {:?} }}" , self . wupp (0usize) , self . wupp (1usize) , self . wupp (2usize) , self . wupp (3usize) , self . wupp (4usize) , self . wupp (5usize) , self . wupp (6usize) , self . wupp (7usize))
        }
    }
    #[doc = "PWR wakeup control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr3(pub u32);
    impl Wucr3 {
        #[doc = "This field must be configured when WUPEN1 = 0. 00: WKUP1_0 01: WKUP1_1 10: WKUP1_2 11: WKUP1_3."]
        #[inline(always)]
        pub const fn wusel1(&self) -> super::vals::Wusel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "This field must be configured when WUPEN1 = 0. 00: WKUP1_0 01: WKUP1_1 10: WKUP1_2 11: WKUP1_3."]
        #[inline(always)]
        pub fn set_wusel1(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "This field must be configured when WUPEN2 = 0. 00: WKUP2_0 01: WKUP2_1 10: WKUP2_2 11: WKUP2_3."]
        #[inline(always)]
        pub const fn wusel2(&self) -> super::vals::Wusel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "This field must be configured when WUPEN2 = 0. 00: WKUP2_0 01: WKUP2_1 10: WKUP2_2 11: WKUP2_3."]
        #[inline(always)]
        pub fn set_wusel2(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "This field must be configured when WUPEN3 = 0. 00: WKUP3_0 01: WKUP3_1 10: WKUP3_2 11: WKUP3_3."]
        #[inline(always)]
        pub const fn wusel3(&self) -> super::vals::Wusel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "This field must be configured when WUPEN3 = 0. 00: WKUP3_0 01: WKUP3_1 10: WKUP3_2 11: WKUP3_3."]
        #[inline(always)]
        pub fn set_wusel3(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "This field must be configured when WUPEN4 = 0. 00: WKUP4_0 01: WKUP4_1 10: WKUP4_2 11: WKUP4_3."]
        #[inline(always)]
        pub const fn wusel4(&self) -> super::vals::Wusel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "This field must be configured when WUPEN4 = 0. 00: WKUP4_0 01: WKUP4_1 10: WKUP4_2 11: WKUP4_3."]
        #[inline(always)]
        pub fn set_wusel4(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "This field must be configured when WUPEN5 = 0. 00: WKUP5_0 01: WKUP5_1 10: WKUP5_2 11: WKUP5_3."]
        #[inline(always)]
        pub const fn wusel5(&self) -> super::vals::Wusel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "This field must be configured when WUPEN5 = 0. 00: WKUP5_0 01: WKUP5_1 10: WKUP5_2 11: WKUP5_3."]
        #[inline(always)]
        pub fn set_wusel5(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "This field must be configured when WUPEN6 = 0. 00: WKUP6_0 01: WKUP6_1 10: WKUP6_2 11: WKUP6_3."]
        #[inline(always)]
        pub const fn wusel6(&self) -> super::vals::Wusel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "This field must be configured when WUPEN6 = 0. 00: WKUP6_0 01: WKUP6_1 10: WKUP6_2 11: WKUP6_3."]
        #[inline(always)]
        pub fn set_wusel6(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "This field must be configured when WUPEN7 = 0. 00: WKUP7_0 01: WKUP7_1 10: WKUP7_2 11: WKUP7_3."]
        #[inline(always)]
        pub const fn wusel7(&self) -> super::vals::Wusel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "This field must be configured when WUPEN7 = 0. 00: WKUP7_0 01: WKUP7_1 10: WKUP7_2 11: WKUP7_3."]
        #[inline(always)]
        pub fn set_wusel7(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "This field must be configured when WUPEN8 = 0. 00: WKUP8_0 01: WKUP8_1 10: WKUP8_2 11: WKUP8_3."]
        #[inline(always)]
        pub const fn wusel8(&self) -> super::vals::Wusel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "This field must be configured when WUPEN8 = 0. 00: WKUP8_0 01: WKUP8_1 10: WKUP8_2 11: WKUP8_3."]
        #[inline(always)]
        pub fn set_wusel8(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Wucr3 {
        #[inline(always)]
        fn default() -> Wucr3 {
            Wucr3(0)
        }
    }
    impl core::fmt::Debug for Wucr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wucr3")
                .field("wusel1", &self.wusel1())
                .field("wusel2", &self.wusel2())
                .field("wusel3", &self.wusel3())
                .field("wusel4", &self.wusel4())
                .field("wusel5", &self.wusel5())
                .field("wusel6", &self.wusel6())
                .field("wusel7", &self.wusel7())
                .field("wusel8", &self.wusel8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wucr3 {{ wusel1: {:?}, wusel2: {:?}, wusel3: {:?}, wusel4: {:?}, wusel5: {:?}, wusel6: {:?}, wusel7: {:?}, wusel8: {:?} }}" , self . wusel1 () , self . wusel2 () , self . wusel3 () , self . wusel4 () , self . wusel5 () , self . wusel6 () , self . wusel7 () , self . wusel8 ())
        }
    }
    #[doc = "PWR wakeup status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wuscr(pub u32);
    impl Wuscr {
        #[doc = "Writing 1 to this bit clears the WUF1 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF1 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF2 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF2 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF3 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF3 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF4 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF4 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF5 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF5 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF6 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF6 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF7 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF7 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF8 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF8 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF9 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF9 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Writing 1 to this bit clears the WUF10 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 to this bit clears the WUF10 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Wuscr {
        #[inline(always)]
        fn default() -> Wuscr {
            Wuscr(0)
        }
    }
    impl core::fmt::Debug for Wuscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wuscr")
                .field("cwuf1", &self.cwuf1())
                .field("cwuf2", &self.cwuf2())
                .field("cwuf3", &self.cwuf3())
                .field("cwuf4", &self.cwuf4())
                .field("cwuf5", &self.cwuf5())
                .field("cwuf6", &self.cwuf6())
                .field("cwuf7", &self.cwuf7())
                .field("cwuf8", &self.cwuf8())
                .field("cwuf9", &self.cwuf9())
                .field("cwuf10", &self.cwuf10())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wuscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wuscr {{ cwuf1: {=bool:?}, cwuf2: {=bool:?}, cwuf3: {=bool:?}, cwuf4: {=bool:?}, cwuf5: {=bool:?}, cwuf6: {=bool:?}, cwuf7: {=bool:?}, cwuf8: {=bool:?}, cwuf9: {=bool:?}, cwuf10: {=bool:?} }}" , self . cwuf1 () , self . cwuf2 () , self . cwuf3 () , self . cwuf4 () , self . cwuf5 () , self . cwuf6 () , self . cwuf7 () , self . cwuf8 () , self . cwuf9 () , self . cwuf10 ())
        }
    }
    #[doc = "PWR wakeup status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wusr(pub u32);
    impl Wusr {
        #[doc = "This bit is set when a wakeup event is detected on WKUP1 line. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN1=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP1 line. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN1=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP2 line. This bit is cleared by writing 1 in the CWUF2 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN2=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP2 line. This bit is cleared by writing 1 in the CWUF2 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN2=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP3 line. This bit is cleared by writing 1 in the CWUF3 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN3=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP3 line. This bit is cleared by writing 1 in the CWUF3 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN3=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP4 line. This bit is cleared by writing 1 in the CWUF4 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN4=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP4 line. This bit is cleared by writing 1 in the CWUF4 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN4=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP5 line. This bit is cleared by writing 1 in the CWUF5 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN5=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP5 line. This bit is cleared by writing 1 in the CWUF5 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN5=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP6 line. This bit is cleared by writing 1 in the CWUF6 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN6=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP6 line. This bit is cleared by writing 1 in the CWUF6 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN6=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP7 line. This bit is cleared by writing 1 in the CWUF7 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN7=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP7 line. This bit is cleared by writing 1 in the CWUF7 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN7=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP8 line. This bit is cleared by writing 1 in the CWUF8 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN8=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP8 line. This bit is cleared by writing 1 in the CWUF8 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN8=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP9 line. This bit is cleared by writing 1 in the CWUF9 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN9=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP9 line. This bit is cleared by writing 1 in the CWUF9 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN9=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP10 line. This bit is cleared by writing 1 in the CWUF10 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN10=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set when a wakeup event is detected on WKUP10 line. This bit is cleared by writing 1 in the CWUF10 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN10=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Wusr {
        #[inline(always)]
        fn default() -> Wusr {
            Wusr(0)
        }
    }
    impl core::fmt::Debug for Wusr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wusr")
                .field("wuf1", &self.wuf1())
                .field("wuf2", &self.wuf2())
                .field("wuf3", &self.wuf3())
                .field("wuf4", &self.wuf4())
                .field("wuf5", &self.wuf5())
                .field("wuf6", &self.wuf6())
                .field("wuf7", &self.wuf7())
                .field("wuf8", &self.wuf8())
                .field("wuf9", &self.wuf9())
                .field("wuf10", &self.wuf10())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wusr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wusr {{ wuf1: {=bool:?}, wuf2: {=bool:?}, wuf3: {=bool:?}, wuf4: {=bool:?}, wuf5: {=bool:?}, wuf6: {=bool:?}, wuf7: {=bool:?}, wuf8: {=bool:?}, wuf9: {=bool:?}, wuf10: {=bool:?} }}" , self . wuf1 () , self . wuf2 () , self . wuf3 () , self . wuf4 () , self . wuf5 () , self . wuf6 () , self . wuf7 () , self . wuf8 () , self . wuf9 () , self . wuf10 ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Flashfwu {
        #[doc = "Flash memory enters low-power mode in Stop 0 and Stop 1 modes (lower-power consumption)."]
        LOW_POWER = 0x0,
        #[doc = "Flash memory remains in normal mode in Stop 0 and Stop 1 modes (faster wakeup time)."]
        NORMAL = 0x01,
    }
    impl Flashfwu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Flashfwu {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Flashfwu {
        #[inline(always)]
        fn from(val: u8) -> Flashfwu {
            Flashfwu::from_bits(val)
        }
    }
    impl From<Flashfwu> for u8 {
        #[inline(always)]
        fn from(val: Flashfwu) -> u8 {
            Flashfwu::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpms {
        #[doc = "Stop 0 mode."]
        STOP0 = 0x0,
        #[doc = "Stop 1 mode."]
        STOP1 = 0x01,
        #[doc = "Stop 2 mode."]
        STOP2 = 0x02,
        #[doc = "Stop 3 mode."]
        STOP3 = 0x03,
        #[doc = "Standby mode."]
        STOP4 = 0x04,
        #[doc = "Standby mode."]
        STOP5 = 0x05,
        #[doc = "Shutdown mode."]
        STOP6 = 0x06,
        #[doc = "Shutdown mode."]
        STOP7 = 0x07,
    }
    impl Lpms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpms {
        #[inline(always)]
        fn from(val: u8) -> Lpms {
            Lpms::from_bits(val)
        }
    }
    impl From<Lpms> for u8 {
        #[inline(always)]
        fn from(val: Lpms) -> u8 {
            Lpms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pds {
        #[doc = "Contents retained in Stop modes."]
        RETAINED = 0x0,
        #[doc = "Content lost in Stop modes."]
        LOST = 0x01,
    }
    impl Pds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pds {
        #[inline(always)]
        fn from(val: u8) -> Pds {
            Pds::from_bits(val)
        }
    }
    impl From<Pds> for u8 {
        #[inline(always)]
        fn from(val: Pds) -> u8 {
            Pds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pvdls {
        #[doc = "VPVD0 around 2.0V."]
        V20 = 0x0,
        #[doc = "VPVD1 around 2.2V."]
        V22 = 0x01,
        #[doc = "VPVD2 around 2.4V."]
        V24 = 0x02,
        #[doc = "VPVD3 around 2.5V."]
        V25 = 0x03,
        #[doc = "VPVD4 around 2.6V."]
        V26 = 0x04,
        #[doc = "VPVD5 around 2.8V."]
        V28 = 0x05,
        #[doc = "VPVD6 around 2.9V."]
        V29 = 0x06,
        #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)."]
        PVD_IN = 0x07,
    }
    impl Pvdls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvdls {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvdls {
        #[inline(always)]
        fn from(val: u8) -> Pvdls {
            Pvdls::from_bits(val)
        }
    }
    impl From<Pvdls> for u8 {
        #[inline(always)]
        fn from(val: Pvdls) -> u8 {
            Pvdls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pvdo {
        #[doc = "VDD is equal or above the PVD threshold selected by PVDLS\\[2:0\\]."]
        ABOVE_OR_EQUAL = 0x0,
        #[doc = "VDD is below the PVD threshold selected by PVDLS\\[2:0\\]."]
        BELOW = 0x01,
    }
    impl Pvdo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvdo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvdo {
        #[inline(always)]
        fn from(val: u8) -> Pvdo {
            Pvdo::from_bits(val)
        }
    }
    impl From<Pvdo> for u8 {
        #[inline(always)]
        fn from(val: Pvdo) -> u8 {
            Pvdo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Regsel {
        #[doc = "LDO selected."]
        LDO = 0x0,
        #[doc = "SMPS selected."]
        SMPS = 0x01,
    }
    impl Regsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Regsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Regsel {
        #[inline(always)]
        fn from(val: u8) -> Regsel {
            Regsel::from_bits(val)
        }
    }
    impl From<Regsel> for u8 {
        #[inline(always)]
        fn from(val: Regsel) -> u8 {
            Regsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sramfwu {
        #[doc = "SRAMs enters low-power mode in Stop 0 and Stop 1 modes (source biasing for lower-power consumption)."]
        LOW_POWER = 0x0,
        #[doc = "SRAMs remains in normal mode in Stop 0 and Stop 1 modes (higher consumption but no SRAM wakeup time)."]
        NORMAL = 0x01,
    }
    impl Sramfwu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sramfwu {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sramfwu {
        #[inline(always)]
        fn from(val: u8) -> Sramfwu {
            Sramfwu::from_bits(val)
        }
    }
    impl From<Sramfwu> for u8 {
        #[inline(always)]
        fn from(val: Sramfwu) -> u8 {
            Sramfwu::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Srampd {
        #[doc = "SRAM1 powered on."]
        POWERED_ON = 0x0,
        #[doc = "SRAM1 powered off."]
        POWERED_OFF = 0x01,
    }
    impl Srampd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Srampd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Srampd {
        #[inline(always)]
        fn from(val: u8) -> Srampd {
            Srampd::from_bits(val)
        }
    }
    impl From<Srampd> for u8 {
        #[inline(always)]
        fn from(val: Srampd) -> u8 {
            Srampd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbrs {
        #[doc = "Charge VBAT through a 5 k ohm resistor."]
        CHARGE_5K = 0x0,
        #[doc = "Charge VBAT through a 1.5 k ohm resistor."]
        CHARGE_1_5K = 0x01,
    }
    impl Vbrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbrs {
        #[inline(always)]
        fn from(val: u8) -> Vbrs {
            Vbrs::from_bits(val)
        }
    }
    impl From<Vbrs> for u8 {
        #[inline(always)]
        fn from(val: Vbrs) -> u8 {
            Vbrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wupp {
        #[doc = "Detection on high level (rising edge)."]
        HIGH = 0x0,
        #[doc = "Detection on low level (falling edge)."]
        LOW = 0x01,
    }
    impl Wupp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wupp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wupp {
        #[inline(always)]
        fn from(val: u8) -> Wupp {
            Wupp::from_bits(val)
        }
    }
    impl From<Wupp> for u8 {
        #[inline(always)]
        fn from(val: Wupp) -> u8 {
            Wupp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wusel {
        #[doc = "WKUPx_0."]
        WKUPX_0 = 0x0,
        #[doc = "WKUPx_1."]
        WKUPX_1 = 0x01,
        #[doc = "WKUPx_2."]
        WKUPX_2 = 0x02,
        #[doc = "WKUPx_3."]
        WKUPX_3 = 0x03,
    }
    impl Wusel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wusel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wusel {
        #[inline(always)]
        fn from(val: u8) -> Wusel {
            Wusel::from_bits(val)
        }
    }
    impl From<Wusel> for u8 {
        #[inline(always)]
        fn from(val: Wusel) -> u8 {
            Wusel::to_bits(val)
        }
    }
}
