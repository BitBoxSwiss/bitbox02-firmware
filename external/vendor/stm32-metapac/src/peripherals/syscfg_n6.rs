#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller."]
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
    #[doc = "SYSCFG boot pin control register."]
    #[inline(always)]
    pub const fn bootcr(self) -> crate::common::Reg<regs::Bootcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SYSCFG Cortex-M55 control register."]
    #[inline(always)]
    pub const fn cm55cr(self) -> crate::common::Reg<regs::Cm55cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SYSCFG Cortex-M55 TCM control register."]
    #[inline(always)]
    pub const fn cm55tcmcr(self) -> crate::common::Reg<regs::Cm55tcmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SYSCFG Cortex-CM55 memory RW margin register."]
    #[inline(always)]
    pub const fn cm55rwmcr(self) -> crate::common::Reg<regs::Cm55rwmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SYSCFG Cortex-M55 SVTOR control register."]
    #[inline(always)]
    pub const fn initsvtorcr(self) -> crate::common::Reg<regs::Initsvtorcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SYSCFG Cortex-M55 NSVTOR control register."]
    #[inline(always)]
    pub const fn initnsvtorcr(self) -> crate::common::Reg<regs::Initnsvtorcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SYSCFG Cortex-M55 reset type control register."]
    #[inline(always)]
    pub const fn cm55rstcr(self) -> crate::common::Reg<regs::Cm55rstcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SYSCFG Cortex-M55 P-AHB write posting control register."]
    #[inline(always)]
    pub const fn cm55pahbwpr(self) -> crate::common::Reg<regs::Cm55pahbwpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SYSCFG VENCRAM control register."]
    #[inline(always)]
    pub const fn vencramcr(self) -> crate::common::Reg<regs::Vencramcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SYSCFG potential tamper reset register."]
    #[inline(always)]
    pub const fn pottamprstcr(self) -> crate::common::Reg<regs::Pottamprstcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SYSCFG AHB-AXI bridge early write response control register."]
    #[inline(always)]
    pub const fn icnewrcr(self) -> crate::common::Reg<regs::Icnewrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SYSCFG ICN clock gating control register."]
    #[inline(always)]
    pub const fn icncgcr(self) -> crate::common::Reg<regs::Icncgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "SYSCFG ICN bandwidth regulator control register."]
    #[inline(always)]
    pub const fn icnbwrcr(self) -> crate::common::Reg<regs::Icnbwrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "SYSCFG /O control register."]
    #[inline(always)]
    pub const fn iocr(self) -> crate::common::Reg<regs::Iocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "SYSCFG VDDIO1 compensation cell control register."]
    #[inline(always)]
    pub const fn vddio1cccr(self) -> crate::common::Reg<regs::Vddio1cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "SYSCFG VDDIO1 compensation cell status register."]
    #[inline(always)]
    pub const fn vddio1ccsr(self) -> crate::common::Reg<regs::Vddio1ccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "SYSCFG VDDIO2 compensation cell control register."]
    #[inline(always)]
    pub const fn vddio2cccr(self) -> crate::common::Reg<regs::Vddio2cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "SYSCFG VDDIO2 compensation cell status register."]
    #[inline(always)]
    pub const fn vddio2ccsr(self) -> crate::common::Reg<regs::Vddio2ccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "SYSCFG VDDIO3 compensation cell control register."]
    #[inline(always)]
    pub const fn vddio3cccr(self) -> crate::common::Reg<regs::Vddio3cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "SYSCFG VDDIO3 compensation cell status register."]
    #[inline(always)]
    pub const fn vddio3ccsr(self) -> crate::common::Reg<regs::Vddio3ccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "SYSCFG VDDIO4 compensation cell control register."]
    #[inline(always)]
    pub const fn vddio4cccr(self) -> crate::common::Reg<regs::Vddio4cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "SYSCFG VDDIO4 compensation cell status register."]
    #[inline(always)]
    pub const fn vddio4ccsr(self) -> crate::common::Reg<regs::Vddio4ccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "SYSCFG VDDIO compensation cell control register."]
    #[inline(always)]
    pub const fn vddiocccr(self) -> crate::common::Reg<regs::Vddiocccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "SYSCFG VDDIO compensation cell status register."]
    #[inline(always)]
    pub const fn vddioccsr(self) -> crate::common::Reg<regs::Vddioccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "SYSCFG control timer break register."]
    #[inline(always)]
    pub const fn cbr(self) -> crate::common::Reg<regs::Cbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "SYSCFG DMA CID secure control register."]
    #[inline(always)]
    pub const fn sec_aidcr(self) -> crate::common::Reg<regs::SecAidcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "SYSCFG FMC retiming logic control register."]
    #[inline(always)]
    pub const fn fmc_retimecr(self) -> crate::common::Reg<regs::FmcRetimecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "SYSCFG NPU RAM interleaving control register."]
    #[inline(always)]
    pub const fn npu_icncr(self) -> crate::common::Reg<regs::NpuIcncr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "SYSCFG boot pin status register."]
    #[inline(always)]
    pub const fn bootsr(self) -> crate::common::Reg<regs::Bootsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "SYSCFG AHB write posting address error register."]
    #[inline(always)]
    pub const fn ahbwp_error_sr(self) -> crate::common::Reg<regs::AhbwpErrorSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "SYSCFG SMPS observable signals through HDP selection configuration register."]
    #[inline(always)]
    pub const fn smpshdpcr(self) -> crate::common::Reg<regs::Smpshdpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "SYSCFG DMA CID non-secure control register."]
    #[inline(always)]
    pub const fn nonsec_aidcr(self) -> crate::common::Reg<regs::NonsecAidcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
}
pub mod regs {
    #[doc = "SYSCFG AHB write posting address error register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AhbwpErrorSr(pub u32);
    impl AhbwpErrorSr {
        #[doc = "Reports address of the first error in P-AHB write-posting buffer."]
        #[inline(always)]
        pub const fn pahb_error_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Reports address of the first error in P-AHB write-posting buffer."]
        #[inline(always)]
        pub fn set_pahb_error_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AhbwpErrorSr {
        #[inline(always)]
        fn default() -> AhbwpErrorSr {
            AhbwpErrorSr(0)
        }
    }
    impl core::fmt::Debug for AhbwpErrorSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AhbwpErrorSr")
                .field("pahb_error_addr", &self.pahb_error_addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AhbwpErrorSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AhbwpErrorSr {{ pahb_error_addr: {=u32:?} }}",
                self.pahb_error_addr()
            )
        }
    }
    #[doc = "SYSCFG boot pin control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bootcr(pub u32);
    impl Bootcr {
        #[doc = "BOOT0 pin pull-down disable."]
        #[inline(always)]
        pub const fn boot0_pd(&self) -> super::vals::Boot0Pd {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Boot0Pd::from_bits(val as u8)
        }
        #[doc = "BOOT0 pin pull-down disable."]
        #[inline(always)]
        pub fn set_boot0_pd(&mut self, val: super::vals::Boot0Pd) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "BOOT1 pin pull-down disable."]
        #[inline(always)]
        pub const fn boot1_pd(&self) -> super::vals::Boot1Pd {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Boot1Pd::from_bits(val as u8)
        }
        #[doc = "BOOT1 pin pull-down disable."]
        #[inline(always)]
        pub fn set_boot1_pd(&mut self, val: super::vals::Boot1Pd) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Bootcr {
        #[inline(always)]
        fn default() -> Bootcr {
            Bootcr(0)
        }
    }
    impl core::fmt::Debug for Bootcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bootcr")
                .field("boot0_pd", &self.boot0_pd())
                .field("boot1_pd", &self.boot1_pd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bootcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bootcr {{ boot0_pd: {:?}, boot1_pd: {:?} }}",
                self.boot0_pd(),
                self.boot1_pd()
            )
        }
    }
    #[doc = "SYSCFG boot pin status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bootsr(pub u32);
    impl Bootsr {
        #[doc = "BOOT0 pin value."]
        #[inline(always)]
        pub const fn boot0(&self) -> super::vals::Boot0 {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Boot0::from_bits(val as u8)
        }
        #[doc = "BOOT0 pin value."]
        #[inline(always)]
        pub fn set_boot0(&mut self, val: super::vals::Boot0) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "BOOT1 pin value."]
        #[inline(always)]
        pub const fn boot1(&self) -> super::vals::Boot1 {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Boot1::from_bits(val as u8)
        }
        #[doc = "BOOT1 pin value."]
        #[inline(always)]
        pub fn set_boot1(&mut self, val: super::vals::Boot1) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Bootsr {
        #[inline(always)]
        fn default() -> Bootsr {
            Bootsr(0)
        }
    }
    impl core::fmt::Debug for Bootsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bootsr")
                .field("boot0", &self.boot0())
                .field("boot1", &self.boot1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bootsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bootsr {{ boot0: {:?}, boot1: {:?} }}", self.boot0(), self.boot1())
        }
    }
    #[doc = "SYSCFG control timer break register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cbr(pub u32);
    impl Cbr {
        #[doc = "CM55 lockup lock enable."]
        #[inline(always)]
        pub const fn cm55l(&self) -> super::vals::Cm55l {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Cm55l::from_bits(val as u8)
        }
        #[doc = "CM55 lockup lock enable."]
        #[inline(always)]
        pub fn set_cm55l(&mut self, val: super::vals::Cm55l) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "PVD lock enable."]
        #[inline(always)]
        pub const fn pvdl_lock(&self) -> super::vals::PvdlLock {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::PvdlLock::from_bits(val as u8)
        }
        #[doc = "PVD lock enable."]
        #[inline(always)]
        pub fn set_pvdl_lock(&mut self, val: super::vals::PvdlLock) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Backup SRAM double ECC error lock."]
        #[inline(always)]
        pub const fn bkpraml(&self) -> super::vals::Bkpraml {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Bkpraml::from_bits(val as u8)
        }
        #[doc = "Backup SRAM double ECC error lock."]
        #[inline(always)]
        pub fn set_bkpraml(&mut self, val: super::vals::Bkpraml) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "CM55 cache double ECC error lock."]
        #[inline(always)]
        pub const fn cm55cachel(&self) -> super::vals::Cm55cachel {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Cm55cachel::from_bits(val as u8)
        }
        #[doc = "CM55 cache double ECC error lock."]
        #[inline(always)]
        pub fn set_cm55cachel(&mut self, val: super::vals::Cm55cachel) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CM55 TCM double ECC error lock."]
        #[inline(always)]
        pub const fn cm55tcml(&self) -> super::vals::Cm55tcml {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Cm55tcml::from_bits(val as u8)
        }
        #[doc = "CM55 TCM double ECC error lock."]
        #[inline(always)]
        pub fn set_cm55tcml(&mut self, val: super::vals::Cm55tcml) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Cbr {
        #[inline(always)]
        fn default() -> Cbr {
            Cbr(0)
        }
    }
    impl core::fmt::Debug for Cbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cbr")
                .field("cm55l", &self.cm55l())
                .field("pvdl_lock", &self.pvdl_lock())
                .field("bkpraml", &self.bkpraml())
                .field("cm55cachel", &self.cm55cachel())
                .field("cm55tcml", &self.cm55tcml())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cbr {{ cm55l: {:?}, pvdl_lock: {:?}, bkpraml: {:?}, cm55cachel: {:?}, cm55tcml: {:?} }}",
                self.cm55l(),
                self.pvdl_lock(),
                self.bkpraml(),
                self.cm55cachel(),
                self.cm55tcml()
            )
        }
    }
    #[doc = "SYSCFG Cortex-M55 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cm55cr(pub u32);
    impl Cm55cr {
        #[doc = "Enable FPU exception."]
        #[inline(always)]
        pub const fn fpu_it_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Enable FPU exception."]
        #[inline(always)]
        pub fn set_fpu_it_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Prevent changes to:."]
        #[inline(always)]
        pub const fn locksvtaircr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Prevent changes to:."]
        #[inline(always)]
        pub fn set_locksvtaircr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Prevent changes to the non-secure vector table base address."]
        #[inline(always)]
        pub const fn locknsvtor(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Prevent changes to the non-secure vector table base address."]
        #[inline(always)]
        pub fn set_locknsvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Prevent changes to programmed secure MPU memory regions."]
        #[inline(always)]
        pub const fn locksmpu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Prevent changes to programmed secure MPU memory regions."]
        #[inline(always)]
        pub fn set_locksmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Prevent changes to non-secure MPU memory regions already programmed."]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Prevent changes to non-secure MPU memory regions already programmed."]
        #[inline(always)]
        pub fn set_locknsmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Prevent changes to secure SAU memory regions already programmed."]
        #[inline(always)]
        pub const fn locksau(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Prevent changes to secure SAU memory regions already programmed."]
        #[inline(always)]
        pub fn set_locksau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Disable access to the instruction cache direct cache access registers DCAICLR and DCAICRR."]
        #[inline(always)]
        pub const fn lockdcaic(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Disable access to the instruction cache direct cache access registers DCAICLR and DCAICRR."]
        #[inline(always)]
        pub fn set_lockdcaic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Cm55cr {
        #[inline(always)]
        fn default() -> Cm55cr {
            Cm55cr(0)
        }
    }
    impl core::fmt::Debug for Cm55cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cm55cr")
                .field("fpu_it_en", &self.fpu_it_en())
                .field("locksvtaircr", &self.locksvtaircr())
                .field("locknsvtor", &self.locknsvtor())
                .field("locksmpu", &self.locksmpu())
                .field("locknsmpu", &self.locknsmpu())
                .field("locksau", &self.locksau())
                .field("lockdcaic", &self.lockdcaic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cm55cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cm55cr {{ fpu_it_en: {=u8:?}, locksvtaircr: {=bool:?}, locknsvtor: {=bool:?}, locksmpu: {=bool:?}, locknsmpu: {=bool:?}, locksau: {=bool:?}, lockdcaic: {=bool:?} }}" , self . fpu_it_en () , self . locksvtaircr () , self . locknsvtor () , self . locksmpu () , self . locknsmpu () , self . locksau () , self . lockdcaic ())
        }
    }
    #[doc = "SYSCFG Cortex-M55 P-AHB write posting control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cm55pahbwpr(pub u32);
    impl Cm55pahbwpr {
        #[doc = "Error capture in write posting buffer."]
        #[inline(always)]
        pub const fn pahb_error_ack(&self) -> super::vals::PahbErrorAck {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::PahbErrorAck::from_bits(val as u8)
        }
        #[doc = "Error capture in write posting buffer."]
        #[inline(always)]
        pub fn set_pahb_error_ack(&mut self, val: super::vals::PahbErrorAck) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Cm55pahbwpr {
        #[inline(always)]
        fn default() -> Cm55pahbwpr {
            Cm55pahbwpr(0)
        }
    }
    impl core::fmt::Debug for Cm55pahbwpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cm55pahbwpr")
                .field("pahb_error_ack", &self.pahb_error_ack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cm55pahbwpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cm55pahbwpr {{ pahb_error_ack: {:?} }}", self.pahb_error_ack())
        }
    }
    #[doc = "SYSCFG Cortex-M55 reset type control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cm55rstcr(pub u32);
    impl Cm55rstcr {
        #[doc = "Select reset to apply on core upon SYSRESETREQ."]
        #[inline(always)]
        pub const fn core_reset_type(&self) -> super::vals::CoreResetType {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::CoreResetType::from_bits(val as u8)
        }
        #[doc = "Select reset to apply on core upon SYSRESETREQ."]
        #[inline(always)]
        pub fn set_core_reset_type(&mut self, val: super::vals::CoreResetType) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Select action to perform on a lockup state on the core."]
        #[inline(always)]
        pub const fn lockup_rst_en(&self) -> super::vals::LockupRstEn {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::LockupRstEn::from_bits(val as u8)
        }
        #[doc = "Select action to perform on a lockup state on the core."]
        #[inline(always)]
        pub fn set_lockup_rst_en(&mut self, val: super::vals::LockupRstEn) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Select action to perform on a lockup state on the core."]
        #[inline(always)]
        pub const fn lockup_nmi_en(&self) -> super::vals::LockupNmiEn {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::LockupNmiEn::from_bits(val as u8)
        }
        #[doc = "Select action to perform on a lockup state on the core."]
        #[inline(always)]
        pub fn set_lockup_nmi_en(&mut self, val: super::vals::LockupNmiEn) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Cm55rstcr {
        #[inline(always)]
        fn default() -> Cm55rstcr {
            Cm55rstcr(0)
        }
    }
    impl core::fmt::Debug for Cm55rstcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cm55rstcr")
                .field("core_reset_type", &self.core_reset_type())
                .field("lockup_rst_en", &self.lockup_rst_en())
                .field("lockup_nmi_en", &self.lockup_nmi_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cm55rstcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cm55rstcr {{ core_reset_type: {:?}, lockup_rst_en: {:?}, lockup_nmi_en: {:?} }}",
                self.core_reset_type(),
                self.lockup_rst_en(),
                self.lockup_nmi_en()
            )
        }
    }
    #[doc = "SYSCFG Cortex-CM55 memory RW margin register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cm55rwmcr(pub u32);
    impl Cm55rwmcr {
        #[doc = "RW margin enable input for TCM memories."]
        #[inline(always)]
        pub const fn rme_tcm(&self) -> super::vals::RmeTcm {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::RmeTcm::from_bits(val as u8)
        }
        #[doc = "RW margin enable input for TCM memories."]
        #[inline(always)]
        pub fn set_rme_tcm(&mut self, val: super::vals::RmeTcm) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "External RW margin inputs for TCM memories."]
        #[inline(always)]
        pub const fn rm_tcm(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "External RW margin inputs for TCM memories."]
        #[inline(always)]
        pub fn set_rm_tcm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "Biasing level adjust input recommended for Vnom."]
        #[inline(always)]
        pub const fn bc1_tcm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Biasing level adjust input recommended for Vnom."]
        #[inline(always)]
        pub fn set_bc1_tcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Biasing level adjust input recommended for Vnom + 10%."]
        #[inline(always)]
        pub const fn bc2_tcm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Biasing level adjust input recommended for Vnom + 10%."]
        #[inline(always)]
        pub fn set_bc2_tcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RW margin enable input for caches memories."]
        #[inline(always)]
        pub const fn rme_cache(&self) -> super::vals::RmeCache {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::RmeCache::from_bits(val as u8)
        }
        #[doc = "RW margin enable input for caches memories."]
        #[inline(always)]
        pub fn set_rme_cache(&mut self, val: super::vals::RmeCache) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "External read/write (RW) margin inputs for caches memories."]
        #[inline(always)]
        pub const fn rm_cache(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "External read/write (RW) margin inputs for caches memories."]
        #[inline(always)]
        pub fn set_rm_cache(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Biasing level adjust input recommended for Vnom."]
        #[inline(always)]
        pub const fn bc1_cache(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Biasing level adjust input recommended for Vnom."]
        #[inline(always)]
        pub fn set_bc1_cache(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Biasing level adjust input recommended for Vnom + 10%."]
        #[inline(always)]
        pub const fn bc2_cache(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Biasing level adjust input recommended for Vnom + 10%."]
        #[inline(always)]
        pub fn set_bc2_cache(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Cm55rwmcr {
        #[inline(always)]
        fn default() -> Cm55rwmcr {
            Cm55rwmcr(0)
        }
    }
    impl core::fmt::Debug for Cm55rwmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cm55rwmcr")
                .field("rme_tcm", &self.rme_tcm())
                .field("rm_tcm", &self.rm_tcm())
                .field("bc1_tcm", &self.bc1_tcm())
                .field("bc2_tcm", &self.bc2_tcm())
                .field("rme_cache", &self.rme_cache())
                .field("rm_cache", &self.rm_cache())
                .field("bc1_cache", &self.bc1_cache())
                .field("bc2_cache", &self.bc2_cache())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cm55rwmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cm55rwmcr {{ rme_tcm: {:?}, rm_tcm: {=u8:?}, bc1_tcm: {=bool:?}, bc2_tcm: {=bool:?}, rme_cache: {:?}, rm_cache: {=u8:?}, bc1_cache: {=bool:?}, bc2_cache: {=bool:?} }}" , self . rme_tcm () , self . rm_tcm () , self . bc1_tcm () , self . bc2_tcm () , self . rme_cache () , self . rm_cache () , self . bc1_cache () , self . bc2_cache ())
        }
    }
    #[doc = "SYSCFG Cortex-M55 TCM control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cm55tcmcr(pub u32);
    impl Cm55tcmcr {
        #[doc = "Select ITCM memory size."]
        #[inline(always)]
        pub const fn cfgitcmsz(&self) -> super::vals::Cfgitcmsz {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Cfgitcmsz::from_bits(val as u8)
        }
        #[doc = "Select ITCM memory size."]
        #[inline(always)]
        pub fn set_cfgitcmsz(&mut self, val: super::vals::Cfgitcmsz) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Select DTCM memory size."]
        #[inline(always)]
        pub const fn cfgdtcmsz(&self) -> super::vals::Cfgdtcmsz {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Cfgdtcmsz::from_bits(val as u8)
        }
        #[doc = "Select DTCM memory size."]
        #[inline(always)]
        pub fn set_cfgdtcmsz(&mut self, val: super::vals::Cfgdtcmsz) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Disable writes to registers associated with the TCM region."]
        #[inline(always)]
        pub const fn locktcm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Disable writes to registers associated with the TCM region."]
        #[inline(always)]
        pub fn set_locktcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Disable writes to registers associated with the ITCM interface security gating."]
        #[inline(always)]
        pub const fn lockitgu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Disable writes to registers associated with the ITCM interface security gating."]
        #[inline(always)]
        pub fn set_lockitgu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Disable writes to registers associated with the DTCM interface security gating."]
        #[inline(always)]
        pub const fn lockdtgu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Disable writes to registers associated with the DTCM interface security gating."]
        #[inline(always)]
        pub fn set_lockdtgu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Disable wait-state applied by default on extended ITCM memory."]
        #[inline(always)]
        pub const fn itcmwsdisable(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Disable wait-state applied by default on extended ITCM memory."]
        #[inline(always)]
        pub fn set_itcmwsdisable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Disable wait-state applied by default on extended DTCM memory."]
        #[inline(always)]
        pub const fn dtcmwsdisable(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Disable wait-state applied by default on extended DTCM memory."]
        #[inline(always)]
        pub fn set_dtcmwsdisable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Cm55tcmcr {
        #[inline(always)]
        fn default() -> Cm55tcmcr {
            Cm55tcmcr(0)
        }
    }
    impl core::fmt::Debug for Cm55tcmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cm55tcmcr")
                .field("cfgitcmsz", &self.cfgitcmsz())
                .field("cfgdtcmsz", &self.cfgdtcmsz())
                .field("locktcm", &self.locktcm())
                .field("lockitgu", &self.lockitgu())
                .field("lockdtgu", &self.lockdtgu())
                .field("itcmwsdisable", &self.itcmwsdisable())
                .field("dtcmwsdisable", &self.dtcmwsdisable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cm55tcmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cm55tcmcr {{ cfgitcmsz: {:?}, cfgdtcmsz: {:?}, locktcm: {=bool:?}, lockitgu: {=bool:?}, lockdtgu: {=bool:?}, itcmwsdisable: {=bool:?}, dtcmwsdisable: {=bool:?} }}" , self . cfgitcmsz () , self . cfgdtcmsz () , self . locktcm () , self . lockitgu () , self . lockdtgu () , self . itcmwsdisable () , self . dtcmwsdisable ())
        }
    }
    #[doc = "SYSCFG FMC retiming logic control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmcRetimecr(pub u32);
    impl FmcRetimecr {
        #[doc = "Retiming on Rx path."]
        #[inline(always)]
        pub const fn cfg_retime_rx(&self) -> super::vals::CfgRetimeRx {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::CfgRetimeRx::from_bits(val as u8)
        }
        #[doc = "Retiming on Rx path."]
        #[inline(always)]
        pub fn set_cfg_retime_rx(&mut self, val: super::vals::CfgRetimeRx) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Retiming on Tx path."]
        #[inline(always)]
        pub const fn cfg_retime_tx(&self) -> super::vals::CfgRetimeTx {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::CfgRetimeTx::from_bits(val as u8)
        }
        #[doc = "Retiming on Tx path."]
        #[inline(always)]
        pub fn set_cfg_retime_tx(&mut self, val: super::vals::CfgRetimeTx) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Delay on feedback clock."]
        #[inline(always)]
        pub const fn sdfbclk_180(&self) -> super::vals::Sdfbclk180 {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Sdfbclk180::from_bits(val as u8)
        }
        #[doc = "Delay on feedback clock."]
        #[inline(always)]
        pub fn set_sdfbclk_180(&mut self, val: super::vals::Sdfbclk180) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
    }
    impl Default for FmcRetimecr {
        #[inline(always)]
        fn default() -> FmcRetimecr {
            FmcRetimecr(0)
        }
    }
    impl core::fmt::Debug for FmcRetimecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmcRetimecr")
                .field("cfg_retime_rx", &self.cfg_retime_rx())
                .field("cfg_retime_tx", &self.cfg_retime_tx())
                .field("sdfbclk_180", &self.sdfbclk_180())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmcRetimecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmcRetimecr {{ cfg_retime_rx: {:?}, cfg_retime_tx: {:?}, sdfbclk_180: {:?} }}",
                self.cfg_retime_rx(),
                self.cfg_retime_tx(),
                self.sdfbclk_180()
            )
        }
    }
    #[doc = "SYSCFG ICN bandwidth regulator control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icnbwrcr(pub u32);
    impl Icnbwrcr {
        #[doc = "Bandwidth regulator control bits."]
        #[inline(always)]
        pub const fn icnbwrcr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bandwidth regulator control bits."]
        #[inline(always)]
        pub fn set_icnbwrcr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Icnbwrcr {
        #[inline(always)]
        fn default() -> Icnbwrcr {
            Icnbwrcr(0)
        }
    }
    impl core::fmt::Debug for Icnbwrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icnbwrcr").field("icnbwrcr", &self.icnbwrcr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icnbwrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Icnbwrcr {{ icnbwrcr: {=u32:?} }}", self.icnbwrcr())
        }
    }
    #[doc = "SYSCFG ICN clock gating control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icncgcr(pub u32);
    impl Icncgcr {
        #[doc = "When bit\\[i\\]
is set to 1, ICN clock gating\\[i\\]
is OFF."]
        #[inline(always)]
        pub const fn icncgcr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "When bit\\[i\\]
is set to 1, ICN clock gating\\[i\\]
is OFF."]
        #[inline(always)]
        pub fn set_icncgcr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Icncgcr {
        #[inline(always)]
        fn default() -> Icncgcr {
            Icncgcr(0)
        }
    }
    impl core::fmt::Debug for Icncgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icncgcr").field("icncgcr", &self.icncgcr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icncgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Icncgcr {{ icncgcr: {=u32:?} }}", self.icncgcr())
        }
    }
    #[doc = "SYSCFG AHB-AXI bridge early write response control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icnewrcr(pub u32);
    impl Icnewrcr {
        #[doc = "None."]
        #[inline(always)]
        pub const fn sdmmc1_early_wr_rsp_enable(&self) -> super::vals::Sdmmc1EarlyWrRspEnable {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Sdmmc1EarlyWrRspEnable::from_bits(val as u8)
        }
        #[doc = "None."]
        #[inline(always)]
        pub fn set_sdmmc1_early_wr_rsp_enable(&mut self, val: super::vals::Sdmmc1EarlyWrRspEnable) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn sdmmc2_early_wr_rsp_enable(&self) -> super::vals::Sdmmc2EarlyWrRspEnable {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Sdmmc2EarlyWrRspEnable::from_bits(val as u8)
        }
        #[doc = "None."]
        #[inline(always)]
        pub fn set_sdmmc2_early_wr_rsp_enable(&mut self, val: super::vals::Sdmmc2EarlyWrRspEnable) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn usb1_early_wr_rsp_enable(&self) -> super::vals::Usb1EarlyWrRspEnable {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Usb1EarlyWrRspEnable::from_bits(val as u8)
        }
        #[doc = "None."]
        #[inline(always)]
        pub fn set_usb1_early_wr_rsp_enable(&mut self, val: super::vals::Usb1EarlyWrRspEnable) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn usb2_early_wr_rsp_enable(&self) -> super::vals::Usb2EarlyWrRspEnable {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Usb2EarlyWrRspEnable::from_bits(val as u8)
        }
        #[doc = "None."]
        #[inline(always)]
        pub fn set_usb2_early_wr_rsp_enable(&mut self, val: super::vals::Usb2EarlyWrRspEnable) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Icnewrcr {
        #[inline(always)]
        fn default() -> Icnewrcr {
            Icnewrcr(0)
        }
    }
    impl core::fmt::Debug for Icnewrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icnewrcr")
                .field("sdmmc1_early_wr_rsp_enable", &self.sdmmc1_early_wr_rsp_enable())
                .field("sdmmc2_early_wr_rsp_enable", &self.sdmmc2_early_wr_rsp_enable())
                .field("usb1_early_wr_rsp_enable", &self.usb1_early_wr_rsp_enable())
                .field("usb2_early_wr_rsp_enable", &self.usb2_early_wr_rsp_enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icnewrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icnewrcr {{ sdmmc1_early_wr_rsp_enable: {:?}, sdmmc2_early_wr_rsp_enable: {:?}, usb1_early_wr_rsp_enable: {:?}, usb2_early_wr_rsp_enable: {:?} }}" , self . sdmmc1_early_wr_rsp_enable () , self . sdmmc2_early_wr_rsp_enable () , self . usb1_early_wr_rsp_enable () , self . usb2_early_wr_rsp_enable ())
        }
    }
    #[doc = "SYSCFG Cortex-M55 NSVTOR control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Initnsvtorcr(pub u32);
    impl Initnsvtorcr {
        #[doc = "Non-secure vector table base address."]
        #[inline(always)]
        pub const fn nsvtor_addr(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Non-secure vector table base address."]
        #[inline(always)]
        pub fn set_nsvtor_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Initnsvtorcr {
        #[inline(always)]
        fn default() -> Initnsvtorcr {
            Initnsvtorcr(0)
        }
    }
    impl core::fmt::Debug for Initnsvtorcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Initnsvtorcr")
                .field("nsvtor_addr", &self.nsvtor_addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Initnsvtorcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Initnsvtorcr {{ nsvtor_addr: {=u32:?} }}", self.nsvtor_addr())
        }
    }
    #[doc = "SYSCFG Cortex-M55 SVTOR control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Initsvtorcr(pub u32);
    impl Initsvtorcr {
        #[doc = "Secure vector table base address."]
        #[inline(always)]
        pub const fn svtor_addr(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Secure vector table base address."]
        #[inline(always)]
        pub fn set_svtor_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Initsvtorcr {
        #[inline(always)]
        fn default() -> Initsvtorcr {
            Initsvtorcr(0)
        }
    }
    impl core::fmt::Debug for Initsvtorcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Initsvtorcr")
                .field("svtor_addr", &self.svtor_addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Initsvtorcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Initsvtorcr {{ svtor_addr: {=u32:?} }}", self.svtor_addr())
        }
    }
    #[doc = "SYSCFG /O control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iocr(pub u32);
    impl Iocr {
        #[doc = "Digital or analog pins."]
        #[inline(always)]
        pub const fn iocr(&self) -> super::vals::Iocr {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Iocr::from_bits(val as u32)
        }
        #[doc = "Digital or analog pins."]
        #[inline(always)]
        pub fn set_iocr(&mut self, val: super::vals::Iocr) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iocr {
        #[inline(always)]
        fn default() -> Iocr {
            Iocr(0)
        }
    }
    impl core::fmt::Debug for Iocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iocr").field("iocr", &self.iocr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iocr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iocr {{ iocr: {:?} }}", self.iocr())
        }
    }
    #[doc = "SYSCFG DMA CID non-secure control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NonsecAidcr(pub u32);
    impl NonsecAidcr {
        #[doc = "Non-secure OS allocates specific CID to DMA channel through these bits."]
        #[inline(always)]
        pub const fn dmacid_nonsec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Non-secure OS allocates specific CID to DMA channel through these bits."]
        #[inline(always)]
        pub fn set_dmacid_nonsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for NonsecAidcr {
        #[inline(always)]
        fn default() -> NonsecAidcr {
            NonsecAidcr(0)
        }
    }
    impl core::fmt::Debug for NonsecAidcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NonsecAidcr")
                .field("dmacid_nonsec", &self.dmacid_nonsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NonsecAidcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "NonsecAidcr {{ dmacid_nonsec: {=u8:?} }}", self.dmacid_nonsec())
        }
    }
    #[doc = "SYSCFG NPU RAM interleaving control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NpuIcncr(pub u32);
    impl NpuIcncr {
        #[doc = "Control interleaving on NPU RAMs."]
        #[inline(always)]
        pub const fn interleaving_active(&self) -> super::vals::InterleavingActive {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::InterleavingActive::from_bits(val as u8)
        }
        #[doc = "Control interleaving on NPU RAMs."]
        #[inline(always)]
        pub fn set_interleaving_active(&mut self, val: super::vals::InterleavingActive) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
    }
    impl Default for NpuIcncr {
        #[inline(always)]
        fn default() -> NpuIcncr {
            NpuIcncr(0)
        }
    }
    impl core::fmt::Debug for NpuIcncr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NpuIcncr")
                .field("interleaving_active", &self.interleaving_active())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NpuIcncr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "NpuIcncr {{ interleaving_active: {:?} }}",
                self.interleaving_active()
            )
        }
    }
    #[doc = "SYSCFG potential tamper reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pottamprstcr(pub u32);
    impl Pottamprstcr {
        #[doc = "This bit can be set by software to mask PKA, SAES, CRYP1/2, and HASH reset, in case of potential tamper."]
        #[inline(always)]
        pub const fn pottampersetmask(&self) -> super::vals::Pottampersetmask {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Pottampersetmask::from_bits(val as u8)
        }
        #[doc = "This bit can be set by software to mask PKA, SAES, CRYP1/2, and HASH reset, in case of potential tamper."]
        #[inline(always)]
        pub fn set_pottampersetmask(&mut self, val: super::vals::Pottampersetmask) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Pottamprstcr {
        #[inline(always)]
        fn default() -> Pottamprstcr {
            Pottamprstcr(0)
        }
    }
    impl core::fmt::Debug for Pottamprstcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pottamprstcr")
                .field("pottampersetmask", &self.pottampersetmask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pottamprstcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pottamprstcr {{ pottampersetmask: {:?} }}", self.pottampersetmask())
        }
    }
    #[doc = "SYSCFG DMA CID secure control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecAidcr(pub u32);
    impl SecAidcr {
        #[doc = "Secure OS allocates specific CID to DMA channel through these bits."]
        #[inline(always)]
        pub const fn dmacid_sec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Secure OS allocates specific CID to DMA channel through these bits."]
        #[inline(always)]
        pub fn set_dmacid_sec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for SecAidcr {
        #[inline(always)]
        fn default() -> SecAidcr {
            SecAidcr(0)
        }
    }
    impl core::fmt::Debug for SecAidcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SecAidcr")
                .field("dmacid_sec", &self.dmacid_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SecAidcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SecAidcr {{ dmacid_sec: {=u8:?} }}", self.dmacid_sec())
        }
    }
    #[doc = "SYSCFG SMPS observable signals through HDP selection configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpshdpcr(pub u32);
    impl Smpshdpcr {
        #[doc = "Others: Reserved."]
        #[inline(always)]
        pub const fn smpshdpsel(&self) -> super::vals::Smpshdpsel {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Smpshdpsel::from_bits(val as u8)
        }
        #[doc = "Others: Reserved."]
        #[inline(always)]
        pub fn set_smpshdpsel(&mut self, val: super::vals::Smpshdpsel) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Smpshdpcr {
        #[inline(always)]
        fn default() -> Smpshdpcr {
            Smpshdpcr(0)
        }
    }
    impl core::fmt::Debug for Smpshdpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smpshdpcr")
                .field("smpshdpsel", &self.smpshdpsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpshdpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Smpshdpcr {{ smpshdpsel: {:?} }}", self.smpshdpsel())
        }
    }
    #[doc = "SYSCFG VDDIO1 compensation cell control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddio1cccr(pub u32);
    impl Vddio1cccr {
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1."]
        #[inline(always)]
        pub const fn ransrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1."]
        #[inline(always)]
        pub fn set_ransrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub const fn rapsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub fn set_rapsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn en(&self) -> super::vals::Vddio1cccrEn {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio1cccrEn::from_bits(val as u8)
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_en(&mut self, val: super::vals::Vddio1cccrEn) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn cs(&self) -> super::vals::Vddio1cccrCs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vddio1cccrCs::from_bits(val as u8)
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_cs(&mut self, val: super::vals::Vddio1cccrCs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Vddio1cccr {
        #[inline(always)]
        fn default() -> Vddio1cccr {
            Vddio1cccr(0)
        }
    }
    impl core::fmt::Debug for Vddio1cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddio1cccr")
                .field("ransrc", &self.ransrc())
                .field("rapsrc", &self.rapsrc())
                .field("en", &self.en())
                .field("cs", &self.cs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddio1cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddio1cccr {{ ransrc: {=u8:?}, rapsrc: {=u8:?}, en: {:?}, cs: {:?} }}",
                self.ransrc(),
                self.rapsrc(),
                self.en(),
                self.cs()
            )
        }
    }
    #[doc = "SYSCFG VDDIO1 compensation cell status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddio1ccsr(pub u32);
    impl Vddio1ccsr {
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub const fn ansrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub fn set_ansrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub const fn apsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub fn set_apsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn ready(&self) -> super::vals::Vddio1ccsrReady {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio1ccsrReady::from_bits(val as u8)
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_ready(&mut self, val: super::vals::Vddio1ccsrReady) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Vddio1ccsr {
        #[inline(always)]
        fn default() -> Vddio1ccsr {
            Vddio1ccsr(0)
        }
    }
    impl core::fmt::Debug for Vddio1ccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddio1ccsr")
                .field("ansrc", &self.ansrc())
                .field("apsrc", &self.apsrc())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddio1ccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddio1ccsr {{ ansrc: {=u8:?}, apsrc: {=u8:?}, ready: {:?} }}",
                self.ansrc(),
                self.apsrc(),
                self.ready()
            )
        }
    }
    #[doc = "SYSCFG VDDIO2 compensation cell control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddio2cccr(pub u32);
    impl Vddio2cccr {
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1."]
        #[inline(always)]
        pub const fn ransrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1."]
        #[inline(always)]
        pub fn set_ransrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub const fn rapsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub fn set_rapsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn en(&self) -> super::vals::Vddio2cccrEn {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio2cccrEn::from_bits(val as u8)
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_en(&mut self, val: super::vals::Vddio2cccrEn) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn cs(&self) -> super::vals::Vddio2cccrCs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vddio2cccrCs::from_bits(val as u8)
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_cs(&mut self, val: super::vals::Vddio2cccrCs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Vddio2cccr {
        #[inline(always)]
        fn default() -> Vddio2cccr {
            Vddio2cccr(0)
        }
    }
    impl core::fmt::Debug for Vddio2cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddio2cccr")
                .field("ransrc", &self.ransrc())
                .field("rapsrc", &self.rapsrc())
                .field("en", &self.en())
                .field("cs", &self.cs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddio2cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddio2cccr {{ ransrc: {=u8:?}, rapsrc: {=u8:?}, en: {:?}, cs: {:?} }}",
                self.ransrc(),
                self.rapsrc(),
                self.en(),
                self.cs()
            )
        }
    }
    #[doc = "SYSCFG VDDIO2 compensation cell status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddio2ccsr(pub u32);
    impl Vddio2ccsr {
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub const fn ansrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub fn set_ansrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub const fn apsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub fn set_apsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn ready(&self) -> super::vals::Vddio2ccsrReady {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio2ccsrReady::from_bits(val as u8)
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_ready(&mut self, val: super::vals::Vddio2ccsrReady) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Vddio2ccsr {
        #[inline(always)]
        fn default() -> Vddio2ccsr {
            Vddio2ccsr(0)
        }
    }
    impl core::fmt::Debug for Vddio2ccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddio2ccsr")
                .field("ansrc", &self.ansrc())
                .field("apsrc", &self.apsrc())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddio2ccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddio2ccsr {{ ansrc: {=u8:?}, apsrc: {=u8:?}, ready: {:?} }}",
                self.ansrc(),
                self.apsrc(),
                self.ready()
            )
        }
    }
    #[doc = "SYSCFG VDDIO3 compensation cell control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddio3cccr(pub u32);
    impl Vddio3cccr {
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1."]
        #[inline(always)]
        pub const fn ransrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1."]
        #[inline(always)]
        pub fn set_ransrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub const fn rapsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub fn set_rapsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn en(&self) -> super::vals::Vddio3cccrEn {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio3cccrEn::from_bits(val as u8)
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_en(&mut self, val: super::vals::Vddio3cccrEn) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn cs(&self) -> super::vals::Vddio3cccrCs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vddio3cccrCs::from_bits(val as u8)
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_cs(&mut self, val: super::vals::Vddio3cccrCs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Vddio3cccr {
        #[inline(always)]
        fn default() -> Vddio3cccr {
            Vddio3cccr(0)
        }
    }
    impl core::fmt::Debug for Vddio3cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddio3cccr")
                .field("ransrc", &self.ransrc())
                .field("rapsrc", &self.rapsrc())
                .field("en", &self.en())
                .field("cs", &self.cs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddio3cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddio3cccr {{ ransrc: {=u8:?}, rapsrc: {=u8:?}, en: {:?}, cs: {:?} }}",
                self.ransrc(),
                self.rapsrc(),
                self.en(),
                self.cs()
            )
        }
    }
    #[doc = "SYSCFG VDDIO3 compensation cell status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddio3ccsr(pub u32);
    impl Vddio3ccsr {
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub const fn ansrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub fn set_ansrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub const fn apsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub fn set_apsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn ready(&self) -> super::vals::Vddio3ccsrReady {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio3ccsrReady::from_bits(val as u8)
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_ready(&mut self, val: super::vals::Vddio3ccsrReady) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Vddio3ccsr {
        #[inline(always)]
        fn default() -> Vddio3ccsr {
            Vddio3ccsr(0)
        }
    }
    impl core::fmt::Debug for Vddio3ccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddio3ccsr")
                .field("ansrc", &self.ansrc())
                .field("apsrc", &self.apsrc())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddio3ccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddio3ccsr {{ ansrc: {=u8:?}, apsrc: {=u8:?}, ready: {:?} }}",
                self.ansrc(),
                self.apsrc(),
                self.ready()
            )
        }
    }
    #[doc = "SYSCFG VDDIO4 compensation cell control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddio4cccr(pub u32);
    impl Vddio4cccr {
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1."]
        #[inline(always)]
        pub const fn ransrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1."]
        #[inline(always)]
        pub fn set_ransrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub const fn rapsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub fn set_rapsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn en(&self) -> super::vals::Vddio4cccrEn {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio4cccrEn::from_bits(val as u8)
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_en(&mut self, val: super::vals::Vddio4cccrEn) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn cs(&self) -> super::vals::Vddio4cccrCs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vddio4cccrCs::from_bits(val as u8)
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_cs(&mut self, val: super::vals::Vddio4cccrCs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Vddio4cccr {
        #[inline(always)]
        fn default() -> Vddio4cccr {
            Vddio4cccr(0)
        }
    }
    impl core::fmt::Debug for Vddio4cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddio4cccr")
                .field("ransrc", &self.ransrc())
                .field("rapsrc", &self.rapsrc())
                .field("en", &self.en())
                .field("cs", &self.cs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddio4cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddio4cccr {{ ransrc: {=u8:?}, rapsrc: {=u8:?}, en: {:?}, cs: {:?} }}",
                self.ransrc(),
                self.rapsrc(),
                self.en(),
                self.cs()
            )
        }
    }
    #[doc = "SYSCFG VDDIO4 compensation cell status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddio4ccsr(pub u32);
    impl Vddio4ccsr {
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub const fn ansrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub fn set_ansrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub const fn apsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub fn set_apsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub const fn ready(&self) -> super::vals::Vddio4ccsrReady {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio4ccsrReady::from_bits(val as u8)
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIOx."]
        #[inline(always)]
        pub fn set_ready(&mut self, val: super::vals::Vddio4ccsrReady) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Vddio4ccsr {
        #[inline(always)]
        fn default() -> Vddio4ccsr {
            Vddio4ccsr(0)
        }
    }
    impl core::fmt::Debug for Vddio4ccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddio4ccsr")
                .field("ansrc", &self.ansrc())
                .field("apsrc", &self.apsrc())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddio4ccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddio4ccsr {{ ansrc: {=u8:?}, apsrc: {=u8:?}, ready: {:?} }}",
                self.ansrc(),
                self.apsrc(),
                self.ready()
            )
        }
    }
    #[doc = "SYSCFG VDDIO compensation cell control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddiocccr(pub u32);
    impl Vddiocccr {
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub const fn ransrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub fn set_ransrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub const fn rapsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1."]
        #[inline(always)]
        pub fn set_rapsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIO."]
        #[inline(always)]
        pub const fn en(&self) -> super::vals::VddiocccrEn {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::VddiocccrEn::from_bits(val as u8)
        }
        #[doc = "Enables the compensation cell of I/Os supplied by VDDIO."]
        #[inline(always)]
        pub fn set_en(&mut self, val: super::vals::VddiocccrEn) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIO."]
        #[inline(always)]
        pub const fn cs(&self) -> super::vals::VddiocccrCs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::VddiocccrCs::from_bits(val as u8)
        }
        #[doc = "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIO."]
        #[inline(always)]
        pub fn set_cs(&mut self, val: super::vals::VddiocccrCs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Vddiocccr {
        #[inline(always)]
        fn default() -> Vddiocccr {
            Vddiocccr(0)
        }
    }
    impl core::fmt::Debug for Vddiocccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddiocccr")
                .field("ransrc", &self.ransrc())
                .field("rapsrc", &self.rapsrc())
                .field("en", &self.en())
                .field("cs", &self.cs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddiocccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddiocccr {{ ransrc: {=u8:?}, rapsrc: {=u8:?}, en: {:?}, cs: {:?} }}",
                self.ransrc(),
                self.rapsrc(),
                self.en(),
                self.cs()
            )
        }
    }
    #[doc = "SYSCFG VDDIO compensation cell status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vddioccsr(pub u32);
    impl Vddioccsr {
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub const fn ansrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors."]
        #[inline(always)]
        pub fn set_ansrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub const fn apsrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors."]
        #[inline(always)]
        pub fn set_apsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIO."]
        #[inline(always)]
        pub const fn ready(&self) -> super::vals::VddioccsrReady {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::VddioccsrReady::from_bits(val as u8)
        }
        #[doc = "Provides the compensation cell status of I/Os supplied by VDDIO."]
        #[inline(always)]
        pub fn set_ready(&mut self, val: super::vals::VddioccsrReady) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Vddioccsr {
        #[inline(always)]
        fn default() -> Vddioccsr {
            Vddioccsr(0)
        }
    }
    impl core::fmt::Debug for Vddioccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vddioccsr")
                .field("ansrc", &self.ansrc())
                .field("apsrc", &self.apsrc())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vddioccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vddioccsr {{ ansrc: {=u8:?}, apsrc: {=u8:?}, ready: {:?} }}",
                self.ansrc(),
                self.apsrc(),
                self.ready()
            )
        }
    }
    #[doc = "SYSCFG VENCRAM control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vencramcr(pub u32);
    impl Vencramcr {
        #[doc = "VENCRAM allocation VENC if active, or to system (if VENC inactive)."]
        #[inline(always)]
        pub const fn vencram_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VENCRAM allocation VENC if active, or to system (if VENC inactive)."]
        #[inline(always)]
        pub fn set_vencram_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Vencramcr {
        #[inline(always)]
        fn default() -> Vencramcr {
            Vencramcr(0)
        }
    }
    impl core::fmt::Debug for Vencramcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vencramcr")
                .field("vencram_en", &self.vencram_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vencramcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vencramcr {{ vencram_en: {=bool:?} }}", self.vencram_en())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bkpraml {
        #[doc = "Backup SRAM double ECC error signal disconnected from TIM1/8/15/16/17 break inputs."]
        B_0X0 = 0x0,
        #[doc = "Backup SRAM double ECC error signal connected to TIM1/8/15/16/17 break inputs."]
        B_0X1 = 0x01,
    }
    impl Bkpraml {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bkpraml {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bkpraml {
        #[inline(always)]
        fn from(val: u8) -> Bkpraml {
            Bkpraml::from_bits(val)
        }
    }
    impl From<Bkpraml> for u8 {
        #[inline(always)]
        fn from(val: Bkpraml) -> u8 {
            Bkpraml::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Boot0 {
        #[doc = "BOOT0 pin connected to VSS (or left open if BOOT0_PD = 0)."]
        B_0X0 = 0x0,
        #[doc = "BOOT0 pin connected to VDD."]
        B_0X1 = 0x01,
    }
    impl Boot0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Boot0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Boot0 {
        #[inline(always)]
        fn from(val: u8) -> Boot0 {
            Boot0::from_bits(val)
        }
    }
    impl From<Boot0> for u8 {
        #[inline(always)]
        fn from(val: Boot0) -> u8 {
            Boot0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Boot0Pd {
        #[doc = "Pull-down enabled. The BOOT0 pin can be left open and takes a value of 0 if open."]
        B_0X0 = 0x0,
        #[doc = "Pull-down disabled. The BOOT0 pin must not be left open."]
        B_0X1 = 0x01,
    }
    impl Boot0Pd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Boot0Pd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Boot0Pd {
        #[inline(always)]
        fn from(val: u8) -> Boot0Pd {
            Boot0Pd::from_bits(val)
        }
    }
    impl From<Boot0Pd> for u8 {
        #[inline(always)]
        fn from(val: Boot0Pd) -> u8 {
            Boot0Pd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Boot1 {
        #[doc = "BOOT1 pin connected to VSS (or left open if BOOT1_PD = 0 in BOOTCR)."]
        B_0X0 = 0x0,
        #[doc = "BOOT1 pin connected to VDD."]
        B_0X1 = 0x01,
    }
    impl Boot1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Boot1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Boot1 {
        #[inline(always)]
        fn from(val: u8) -> Boot1 {
            Boot1::from_bits(val)
        }
    }
    impl From<Boot1> for u8 {
        #[inline(always)]
        fn from(val: Boot1) -> u8 {
            Boot1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Boot1Pd {
        #[doc = "Pull-down enabled. The BOOT1 pin can be left open and takes a value of 0 if open."]
        B_0X0 = 0x0,
        #[doc = "Pull-down disabled. The BOOT1 pin must not be left open."]
        B_0X1 = 0x01,
    }
    impl Boot1Pd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Boot1Pd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Boot1Pd {
        #[inline(always)]
        fn from(val: u8) -> Boot1Pd {
            Boot1Pd::from_bits(val)
        }
    }
    impl From<Boot1Pd> for u8 {
        #[inline(always)]
        fn from(val: Boot1Pd) -> u8 {
            Boot1Pd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CfgRetimeRx {
        #[doc = "No retiming on Rx path."]
        B_0X0 = 0x0,
        #[doc = "Retiming on Rx path."]
        B_0X1 = 0x01,
    }
    impl CfgRetimeRx {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CfgRetimeRx {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CfgRetimeRx {
        #[inline(always)]
        fn from(val: u8) -> CfgRetimeRx {
            CfgRetimeRx::from_bits(val)
        }
    }
    impl From<CfgRetimeRx> for u8 {
        #[inline(always)]
        fn from(val: CfgRetimeRx) -> u8 {
            CfgRetimeRx::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CfgRetimeTx {
        #[doc = "No retiming on Tx path."]
        B_0X0 = 0x0,
        #[doc = "Retiming on Tx path."]
        B_0X1 = 0x01,
    }
    impl CfgRetimeTx {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CfgRetimeTx {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CfgRetimeTx {
        #[inline(always)]
        fn from(val: u8) -> CfgRetimeTx {
            CfgRetimeTx::from_bits(val)
        }
    }
    impl From<CfgRetimeTx> for u8 {
        #[inline(always)]
        fn from(val: CfgRetimeTx) -> u8 {
            CfgRetimeTx::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfgdtcmsz {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "128 Kbytes (default value)."]
        B_0X8 = 0x08,
        #[doc = "256 Kbytes."]
        B_0X9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Cfgdtcmsz {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfgdtcmsz {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfgdtcmsz {
        #[inline(always)]
        fn from(val: u8) -> Cfgdtcmsz {
            Cfgdtcmsz::from_bits(val)
        }
    }
    impl From<Cfgdtcmsz> for u8 {
        #[inline(always)]
        fn from(val: Cfgdtcmsz) -> u8 {
            Cfgdtcmsz::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfgitcmsz {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "64 KB (default value)."]
        B_0X7 = 0x07,
        #[doc = "128 Kbytes."]
        B_0X8 = 0x08,
        #[doc = "256 Kbytes."]
        B_0X9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Cfgitcmsz {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfgitcmsz {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfgitcmsz {
        #[inline(always)]
        fn from(val: u8) -> Cfgitcmsz {
            Cfgitcmsz::from_bits(val)
        }
    }
    impl From<Cfgitcmsz> for u8 {
        #[inline(always)]
        fn from(val: Cfgitcmsz) -> u8 {
            Cfgitcmsz::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cm55cachel {
        #[doc = "Cortex-M55 cache double ECC error signal disconnected from TIM1/8/15/16/17 break inputs."]
        B_0X0 = 0x0,
        #[doc = "Cortex-M55 cache double ECC error signal connected to TIM1/8/15/16/17 break inputs."]
        B_0X1 = 0x01,
    }
    impl Cm55cachel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cm55cachel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cm55cachel {
        #[inline(always)]
        fn from(val: u8) -> Cm55cachel {
            Cm55cachel::from_bits(val)
        }
    }
    impl From<Cm55cachel> for u8 {
        #[inline(always)]
        fn from(val: Cm55cachel) -> u8 {
            Cm55cachel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cm55l {
        #[doc = "Cortex-M55 lockup output disconnected from TIM1/8/15/16/17 break inputs."]
        B_0X0 = 0x0,
        #[doc = "Cortex-M55 lockup output disconnected from TIM1/8/15/16/17 break inputs."]
        B_0X1 = 0x01,
    }
    impl Cm55l {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cm55l {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cm55l {
        #[inline(always)]
        fn from(val: u8) -> Cm55l {
            Cm55l::from_bits(val)
        }
    }
    impl From<Cm55l> for u8 {
        #[inline(always)]
        fn from(val: Cm55l) -> u8 {
            Cm55l::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cm55tcml {
        #[doc = "Cortex-M55 TCM double ECC error signal disconnected from TIM1/8/15/16/17 break inputs."]
        B_0X0 = 0x0,
        #[doc = "Cortex-M55 TCM double ECC error signal connected to TIM1/8/15/16/17 break inputs."]
        B_0X1 = 0x01,
    }
    impl Cm55tcml {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cm55tcml {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cm55tcml {
        #[inline(always)]
        fn from(val: u8) -> Cm55tcml {
            Cm55tcml::from_bits(val)
        }
    }
    impl From<Cm55tcml> for u8 {
        #[inline(always)]
        fn from(val: Cm55tcml) -> u8 {
            Cm55tcml::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CoreResetType {
        #[doc = "Warm reset (default value)."]
        B_0X0 = 0x0,
        #[doc = "Power-on reset."]
        B_0X1 = 0x01,
    }
    impl CoreResetType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CoreResetType {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CoreResetType {
        #[inline(always)]
        fn from(val: u8) -> CoreResetType {
            CoreResetType::from_bits(val)
        }
    }
    impl From<CoreResetType> for u8 {
        #[inline(always)]
        fn from(val: CoreResetType) -> u8 {
            CoreResetType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum InterleavingActive {
        #[doc = "Interleaving disabled."]
        B_0X0 = 0x0,
        #[doc = "Interleaving enabled."]
        B_0X1 = 0x01,
    }
    impl InterleavingActive {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> InterleavingActive {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for InterleavingActive {
        #[inline(always)]
        fn from(val: u8) -> InterleavingActive {
            InterleavingActive::from_bits(val)
        }
    }
    impl From<InterleavingActive> for u8 {
        #[inline(always)]
        fn from(val: InterleavingActive) -> u8 {
            InterleavingActive::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocr(u32);
    impl Iocr {
        #[doc = "High-speed mode disabled, or use ADC ANA pin."]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "High-speed mode enabled, or connect internal ADC ANA signal to GPIO."]
        pub const B_0X1: Self = Self(0x01);
    }
    impl Iocr {
        pub const fn from_bits(val: u32) -> Iocr {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Iocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                0x01 => f.write_str("B_0X1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iocr {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                0x01 => defmt::write!(f, "B_0X1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Iocr {
        #[inline(always)]
        fn from(val: u32) -> Iocr {
            Iocr::from_bits(val)
        }
    }
    impl From<Iocr> for u32 {
        #[inline(always)]
        fn from(val: Iocr) -> u32 {
            Iocr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LockupNmiEn {
        #[doc = "Lockup state must be recovered from NVIC interrupt (default value)."]
        B_0X0 = 0x0,
        #[doc = "Lockup generates a NMI on the core."]
        B_0X1 = 0x01,
    }
    impl LockupNmiEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LockupNmiEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LockupNmiEn {
        #[inline(always)]
        fn from(val: u8) -> LockupNmiEn {
            LockupNmiEn::from_bits(val)
        }
    }
    impl From<LockupNmiEn> for u8 {
        #[inline(always)]
        fn from(val: LockupNmiEn) -> u8 {
            LockupNmiEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LockupRstEn {
        #[doc = "Lockup state shall be recovered from interrupt (default value)."]
        B_0X0 = 0x0,
        #[doc = "Lockup requests a warm reset to the RCC."]
        B_0X1 = 0x01,
    }
    impl LockupRstEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LockupRstEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LockupRstEn {
        #[inline(always)]
        fn from(val: u8) -> LockupRstEn {
            LockupRstEn::from_bits(val)
        }
    }
    impl From<LockupRstEn> for u8 {
        #[inline(always)]
        fn from(val: LockupRstEn) -> u8 {
            LockupRstEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PahbErrorAck {
        #[doc = "Error capture."]
        B_0X0 = 0x0,
        #[doc = "Clean error."]
        B_0X1 = 0x01,
    }
    impl PahbErrorAck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PahbErrorAck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PahbErrorAck {
        #[inline(always)]
        fn from(val: u8) -> PahbErrorAck {
            PahbErrorAck::from_bits(val)
        }
    }
    impl From<PahbErrorAck> for u8 {
        #[inline(always)]
        fn from(val: PahbErrorAck) -> u8 {
            PahbErrorAck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pottampersetmask {
        #[doc = "PKA, SAES, CRYP1/2, and HASH reset in case of potential tamper."]
        B_0X0 = 0x0,
        #[doc = "PKA, SAES, CRYP1/2, and HASH not reset in case of potential tamper."]
        B_0X1 = 0x01,
    }
    impl Pottampersetmask {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pottampersetmask {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pottampersetmask {
        #[inline(always)]
        fn from(val: u8) -> Pottampersetmask {
            Pottampersetmask::from_bits(val)
        }
    }
    impl From<Pottampersetmask> for u8 {
        #[inline(always)]
        fn from(val: Pottampersetmask) -> u8 {
            Pottampersetmask::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PvdlLock {
        #[doc = "PVD interrupt disconnected from TIM1/8/15/16/17 break input. PVDE bits can be programmed by the application."]
        B_0X0 = 0x0,
        #[doc = "PVD interrupt connected to TIM1/8/15/16/17 break input. PVDE and bits are read only."]
        B_0X1 = 0x01,
    }
    impl PvdlLock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PvdlLock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PvdlLock {
        #[inline(always)]
        fn from(val: u8) -> PvdlLock {
            PvdlLock::from_bits(val)
        }
    }
    impl From<PvdlLock> for u8 {
        #[inline(always)]
        fn from(val: PvdlLock) -> u8 {
            PvdlLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RmeCache {
        #[doc = "Default RW margin settings."]
        B_0X0 = 0x0,
        #[doc = "Use external pin RW margin setting."]
        B_0X1 = 0x01,
    }
    impl RmeCache {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RmeCache {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RmeCache {
        #[inline(always)]
        fn from(val: u8) -> RmeCache {
            RmeCache::from_bits(val)
        }
    }
    impl From<RmeCache> for u8 {
        #[inline(always)]
        fn from(val: RmeCache) -> u8 {
            RmeCache::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RmeTcm {
        #[doc = "Default RW margin settings."]
        B_0X0 = 0x0,
        #[doc = "Use external pin RW margin setting."]
        B_0X1 = 0x01,
    }
    impl RmeTcm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RmeTcm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RmeTcm {
        #[inline(always)]
        fn from(val: u8) -> RmeTcm {
            RmeTcm::from_bits(val)
        }
    }
    impl From<RmeTcm> for u8 {
        #[inline(always)]
        fn from(val: RmeTcm) -> u8 {
            RmeTcm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdfbclk180 {
        #[doc = "No delay on the feedback clock."]
        B_0X0 = 0x0,
        #[doc = "Half a cycle delay on the feedback clock."]
        B_0X1 = 0x01,
    }
    impl Sdfbclk180 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdfbclk180 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdfbclk180 {
        #[inline(always)]
        fn from(val: u8) -> Sdfbclk180 {
            Sdfbclk180::from_bits(val)
        }
    }
    impl From<Sdfbclk180> for u8 {
        #[inline(always)]
        fn from(val: Sdfbclk180) -> u8 {
            Sdfbclk180::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdmmc1EarlyWrRspEnable {
        #[doc = "Early-write response disabled. The last AHB write data beat receives the AXI buffered response for the complete AHB transaction."]
        B_0X0 = 0x0,
        #[doc = "Early-write response enabled. AHB-Lite write data beats receive an automatic OK response from the AHB-to-AXI bridge, whatever the B-channel AXI response."]
        B_0X1 = 0x01,
    }
    impl Sdmmc1EarlyWrRspEnable {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdmmc1EarlyWrRspEnable {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdmmc1EarlyWrRspEnable {
        #[inline(always)]
        fn from(val: u8) -> Sdmmc1EarlyWrRspEnable {
            Sdmmc1EarlyWrRspEnable::from_bits(val)
        }
    }
    impl From<Sdmmc1EarlyWrRspEnable> for u8 {
        #[inline(always)]
        fn from(val: Sdmmc1EarlyWrRspEnable) -> u8 {
            Sdmmc1EarlyWrRspEnable::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdmmc2EarlyWrRspEnable {
        #[doc = "Early-write response disabled. The last AHB write data beat receives the AXI buffered response for the complete AHB transaction."]
        B_0X0 = 0x0,
        #[doc = "Early-write response enabled. AHB-Lite write data beats receive an automatic OK response from the AHB-to-AXI bridge, whatever the B-channel AXI response."]
        B_0X1 = 0x01,
    }
    impl Sdmmc2EarlyWrRspEnable {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdmmc2EarlyWrRspEnable {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdmmc2EarlyWrRspEnable {
        #[inline(always)]
        fn from(val: u8) -> Sdmmc2EarlyWrRspEnable {
            Sdmmc2EarlyWrRspEnable::from_bits(val)
        }
    }
    impl From<Sdmmc2EarlyWrRspEnable> for u8 {
        #[inline(always)]
        fn from(val: Sdmmc2EarlyWrRspEnable) -> u8 {
            Sdmmc2EarlyWrRspEnable::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpshdpsel {
        #[doc = "Standard run mode (no HDP)."]
        B_0X0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        #[doc = "Analyze fsm mode analysis."]
        B_0X_C = 0x0c,
        #[doc = "Analyze fsm mos analysis."]
        B_0X_D = 0x0d,
        #[doc = "Analyze fsm rampe analysis."]
        B_0X_E = 0x0e,
        #[doc = "Analyze fsm mode analysis."]
        B_0X_F = 0x0f,
    }
    impl Smpshdpsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpshdpsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpshdpsel {
        #[inline(always)]
        fn from(val: u8) -> Smpshdpsel {
            Smpshdpsel::from_bits(val)
        }
    }
    impl From<Smpshdpsel> for u8 {
        #[inline(always)]
        fn from(val: Smpshdpsel) -> u8 {
            Smpshdpsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usb1EarlyWrRspEnable {
        #[doc = "Early-write response disabled. The last AHB write data beat receives the AXI buffered response for the complete AHB transaction."]
        B_0X0 = 0x0,
        #[doc = "Early-write response enabled. AHB-Lite write data beats receive an automatic OK response from the AHB-to-AXI bridge, whatever the B-channel AXI response."]
        B_0X1 = 0x01,
    }
    impl Usb1EarlyWrRspEnable {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usb1EarlyWrRspEnable {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usb1EarlyWrRspEnable {
        #[inline(always)]
        fn from(val: u8) -> Usb1EarlyWrRspEnable {
            Usb1EarlyWrRspEnable::from_bits(val)
        }
    }
    impl From<Usb1EarlyWrRspEnable> for u8 {
        #[inline(always)]
        fn from(val: Usb1EarlyWrRspEnable) -> u8 {
            Usb1EarlyWrRspEnable::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usb2EarlyWrRspEnable {
        #[doc = "Early-write response disabled. The last AHB write data beat receives the AXI buffered response for the complete AHB transaction."]
        B_0X0 = 0x0,
        #[doc = "Early-write response enabled. AHB-Lite write data beats receive an automatic OK response from the AHB-to-AXI bridge, whatever the B-channel AXI response."]
        B_0X1 = 0x01,
    }
    impl Usb2EarlyWrRspEnable {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usb2EarlyWrRspEnable {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usb2EarlyWrRspEnable {
        #[inline(always)]
        fn from(val: u8) -> Usb2EarlyWrRspEnable {
            Usb2EarlyWrRspEnable::from_bits(val)
        }
    }
    impl From<Usb2EarlyWrRspEnable> for u8 {
        #[inline(always)]
        fn from(val: Usb2EarlyWrRspEnable) -> u8 {
            Usb2EarlyWrRspEnable::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio1cccrCs {
        #[doc = "VDDIOx I/O code from the cell (available in the VDDIOxCCSR)."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O code from RANSRC\\[3:0\\]
and RAPSRC\\[3:0\\]
in this register."]
        B_0X1 = 0x01,
    }
    impl Vddio1cccrCs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio1cccrCs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio1cccrCs {
        #[inline(always)]
        fn from(val: u8) -> Vddio1cccrCs {
            Vddio1cccrCs::from_bits(val)
        }
    }
    impl From<Vddio1cccrCs> for u8 {
        #[inline(always)]
        fn from(val: Vddio1cccrCs) -> u8 {
            Vddio1cccrCs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio1cccrEn {
        #[doc = "VDDIOx I/O compensation cell disabled."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O compensation cell enabled."]
        B_0X1 = 0x01,
    }
    impl Vddio1cccrEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio1cccrEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio1cccrEn {
        #[inline(always)]
        fn from(val: u8) -> Vddio1cccrEn {
            Vddio1cccrEn::from_bits(val)
        }
    }
    impl From<Vddio1cccrEn> for u8 {
        #[inline(always)]
        fn from(val: Vddio1cccrEn) -> u8 {
            Vddio1cccrEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio1ccsrReady {
        #[doc = "VDDIOx I/O compensation cell not ready."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O compensation cell ready."]
        B_0X1 = 0x01,
    }
    impl Vddio1ccsrReady {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio1ccsrReady {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio1ccsrReady {
        #[inline(always)]
        fn from(val: u8) -> Vddio1ccsrReady {
            Vddio1ccsrReady::from_bits(val)
        }
    }
    impl From<Vddio1ccsrReady> for u8 {
        #[inline(always)]
        fn from(val: Vddio1ccsrReady) -> u8 {
            Vddio1ccsrReady::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio2cccrCs {
        #[doc = "VDDIOx I/O code from the cell (available in the VDDIOxCCSR)."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O code from RANSRC\\[3:0\\]
and RAPSRC\\[3:0\\]
in this register."]
        B_0X1 = 0x01,
    }
    impl Vddio2cccrCs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio2cccrCs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio2cccrCs {
        #[inline(always)]
        fn from(val: u8) -> Vddio2cccrCs {
            Vddio2cccrCs::from_bits(val)
        }
    }
    impl From<Vddio2cccrCs> for u8 {
        #[inline(always)]
        fn from(val: Vddio2cccrCs) -> u8 {
            Vddio2cccrCs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio2cccrEn {
        #[doc = "VDDIOx I/O compensation cell disabled."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O compensation cell enabled."]
        B_0X1 = 0x01,
    }
    impl Vddio2cccrEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio2cccrEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio2cccrEn {
        #[inline(always)]
        fn from(val: u8) -> Vddio2cccrEn {
            Vddio2cccrEn::from_bits(val)
        }
    }
    impl From<Vddio2cccrEn> for u8 {
        #[inline(always)]
        fn from(val: Vddio2cccrEn) -> u8 {
            Vddio2cccrEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio2ccsrReady {
        #[doc = "VDDIOx I/O compensation cell not ready."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O compensation cell ready."]
        B_0X1 = 0x01,
    }
    impl Vddio2ccsrReady {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio2ccsrReady {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio2ccsrReady {
        #[inline(always)]
        fn from(val: u8) -> Vddio2ccsrReady {
            Vddio2ccsrReady::from_bits(val)
        }
    }
    impl From<Vddio2ccsrReady> for u8 {
        #[inline(always)]
        fn from(val: Vddio2ccsrReady) -> u8 {
            Vddio2ccsrReady::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio3cccrCs {
        #[doc = "VDDIOx I/O code from the cell (available in the VDDIOxCCSR)."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O code from RANSRC\\[3:0\\]
and RAPSRC\\[3:0\\]
in this register."]
        B_0X1 = 0x01,
    }
    impl Vddio3cccrCs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio3cccrCs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio3cccrCs {
        #[inline(always)]
        fn from(val: u8) -> Vddio3cccrCs {
            Vddio3cccrCs::from_bits(val)
        }
    }
    impl From<Vddio3cccrCs> for u8 {
        #[inline(always)]
        fn from(val: Vddio3cccrCs) -> u8 {
            Vddio3cccrCs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio3cccrEn {
        #[doc = "VDDIOx I/O compensation cell disabled."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O compensation cell enabled."]
        B_0X1 = 0x01,
    }
    impl Vddio3cccrEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio3cccrEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio3cccrEn {
        #[inline(always)]
        fn from(val: u8) -> Vddio3cccrEn {
            Vddio3cccrEn::from_bits(val)
        }
    }
    impl From<Vddio3cccrEn> for u8 {
        #[inline(always)]
        fn from(val: Vddio3cccrEn) -> u8 {
            Vddio3cccrEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio3ccsrReady {
        #[doc = "VDDIOx I/O compensation cell not ready."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O compensation cell ready."]
        B_0X1 = 0x01,
    }
    impl Vddio3ccsrReady {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio3ccsrReady {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio3ccsrReady {
        #[inline(always)]
        fn from(val: u8) -> Vddio3ccsrReady {
            Vddio3ccsrReady::from_bits(val)
        }
    }
    impl From<Vddio3ccsrReady> for u8 {
        #[inline(always)]
        fn from(val: Vddio3ccsrReady) -> u8 {
            Vddio3ccsrReady::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio4cccrCs {
        #[doc = "VDDIOx I/O code from the cell (available in the VDDIOxCCSR)."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O code from RANSRC\\[3:0\\]
and RAPSRC\\[3:0\\]
in this register."]
        B_0X1 = 0x01,
    }
    impl Vddio4cccrCs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio4cccrCs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio4cccrCs {
        #[inline(always)]
        fn from(val: u8) -> Vddio4cccrCs {
            Vddio4cccrCs::from_bits(val)
        }
    }
    impl From<Vddio4cccrCs> for u8 {
        #[inline(always)]
        fn from(val: Vddio4cccrCs) -> u8 {
            Vddio4cccrCs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio4cccrEn {
        #[doc = "VDDIOx I/O compensation cell disabled."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O compensation cell enabled."]
        B_0X1 = 0x01,
    }
    impl Vddio4cccrEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio4cccrEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio4cccrEn {
        #[inline(always)]
        fn from(val: u8) -> Vddio4cccrEn {
            Vddio4cccrEn::from_bits(val)
        }
    }
    impl From<Vddio4cccrEn> for u8 {
        #[inline(always)]
        fn from(val: Vddio4cccrEn) -> u8 {
            Vddio4cccrEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio4ccsrReady {
        #[doc = "VDDIOx I/O compensation cell not ready."]
        B_0X0 = 0x0,
        #[doc = "VDDIOx I/O compensation cell ready."]
        B_0X1 = 0x01,
    }
    impl Vddio4ccsrReady {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio4ccsrReady {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio4ccsrReady {
        #[inline(always)]
        fn from(val: u8) -> Vddio4ccsrReady {
            Vddio4ccsrReady::from_bits(val)
        }
    }
    impl From<Vddio4ccsrReady> for u8 {
        #[inline(always)]
        fn from(val: Vddio4ccsrReady) -> u8 {
            Vddio4ccsrReady::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VddiocccrCs {
        #[doc = "VDDIO I/O code from the cell (available in the VDDIOCCSR)."]
        B_0X0 = 0x0,
        #[doc = "VDDIO I/O code from RANSRC\\[3:0\\]
and RAPSRC\\[3:0\\]."]
        B_0X1 = 0x01,
    }
    impl VddiocccrCs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VddiocccrCs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VddiocccrCs {
        #[inline(always)]
        fn from(val: u8) -> VddiocccrCs {
            VddiocccrCs::from_bits(val)
        }
    }
    impl From<VddiocccrCs> for u8 {
        #[inline(always)]
        fn from(val: VddiocccrCs) -> u8 {
            VddiocccrCs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VddiocccrEn {
        #[doc = "VDDIO I/O compensation cell disabled."]
        B_0X0 = 0x0,
        #[doc = "VDDIO I/O compensation cell enabled."]
        B_0X1 = 0x01,
    }
    impl VddiocccrEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VddiocccrEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VddiocccrEn {
        #[inline(always)]
        fn from(val: u8) -> VddiocccrEn {
            VddiocccrEn::from_bits(val)
        }
    }
    impl From<VddiocccrEn> for u8 {
        #[inline(always)]
        fn from(val: VddiocccrEn) -> u8 {
            VddiocccrEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VddioccsrReady {
        #[doc = "VDDIO I/O compensation cell not ready."]
        B_0X0 = 0x0,
        #[doc = "VDDIO I/O compensation cell ready."]
        B_0X1 = 0x01,
    }
    impl VddioccsrReady {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VddioccsrReady {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VddioccsrReady {
        #[inline(always)]
        fn from(val: u8) -> VddioccsrReady {
            VddioccsrReady::from_bits(val)
        }
    }
    impl From<VddioccsrReady> for u8 {
        #[inline(always)]
        fn from(val: VddioccsrReady) -> u8 {
            VddioccsrReady::to_bits(val)
        }
    }
}
