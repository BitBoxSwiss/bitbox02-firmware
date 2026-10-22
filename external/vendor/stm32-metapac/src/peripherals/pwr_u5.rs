#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Power control"]
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
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "voltage scaling register"]
    #[inline(always)]
    pub const fn vosr(self) -> crate::common::Reg<regs::Vosr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "supply voltage monitoring control register"]
    #[inline(always)]
    pub const fn svmcr(self) -> crate::common::Reg<regs::Svmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "wakeup control register 1"]
    #[inline(always)]
    pub const fn wucr1(self) -> crate::common::Reg<regs::Wucr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "wakeup control register 2"]
    #[inline(always)]
    pub const fn wucr2(self) -> crate::common::Reg<regs::Wucr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "wakeup control register 3"]
    #[inline(always)]
    pub const fn wucr3(self) -> crate::common::Reg<regs::Wucr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Backup domain control register 1"]
    #[inline(always)]
    pub const fn bdcr1(self) -> crate::common::Reg<regs::Bdcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Backup domain control register 2"]
    #[inline(always)]
    pub const fn bdcr2(self) -> crate::common::Reg<regs::Bdcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "disable Backup domain register"]
    #[inline(always)]
    pub const fn dbpcr(self) -> crate::common::Reg<regs::Dbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "USB Type-C™ and Power Delivery register"]
    #[inline(always)]
    pub const fn ucpdr(self) -> crate::common::Reg<regs::Ucpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "security configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "privilege control register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn svmsr(self) -> crate::common::Reg<regs::Svmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Backup domain status register"]
    #[inline(always)]
    pub const fn bdsr(self) -> crate::common::Reg<regs::Bdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "wakeup status register"]
    #[inline(always)]
    pub const fn wusr(self) -> crate::common::Reg<regs::Wusr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "wakeup status clear register"]
    #[inline(always)]
    pub const fn wuscr(self) -> crate::common::Reg<regs::Wuscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "apply pull configuration register"]
    #[inline(always)]
    pub const fn apcr(self) -> crate::common::Reg<regs::Apcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Power Port pull-up control register"]
    #[inline(always)]
    pub const fn pucr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 9usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 8usize) as _) }
    }
    #[doc = "Power Port pull-down control register"]
    #[inline(always)]
    pub const fn pdcr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 9usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "apply pull configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apcr(pub u32);
    impl Apcr {
        #[doc = "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PUCRx and PDCRx are applied. When this bit is cleared, PUCRx and PDCRx are not applied to the I/Os."]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PUCRx and PDCRx are applied. When this bit is cleared, PUCRx and PDCRx are not applied to the I/Os."]
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
    #[doc = "Backup domain control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr1(pub u32);
    impl Bdcr1 {
        #[doc = "Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode."]
        #[inline(always)]
        pub const fn bren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode."]
        #[inline(always)]
        pub fn set_bren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Backup domain voltage and temperature monitoring enable"]
        #[inline(always)]
        pub const fn monen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain voltage and temperature monitoring enable"]
        #[inline(always)]
        pub fn set_monen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Bdcr1 {
        #[inline(always)]
        fn default() -> Bdcr1 {
            Bdcr1(0)
        }
    }
    impl core::fmt::Debug for Bdcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr1")
                .field("bren", &self.bren())
                .field("monen", &self.monen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdcr1 {{ bren: {=bool:?}, monen: {=bool:?} }}",
                self.bren(),
                self.monen()
            )
        }
    }
    #[doc = "Backup domain control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr2(pub u32);
    impl Bdcr2 {
        #[doc = "VBAT charging enable"]
        #[inline(always)]
        pub const fn vbe(&self) -> super::vals::Vbe {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Vbe::from_bits(val as u8)
        }
        #[doc = "VBAT charging enable"]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: super::vals::Vbe) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "VBAT charging resistor selection"]
        #[inline(always)]
        pub const fn vbrs(&self) -> super::vals::Vbrs {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Vbrs::from_bits(val as u8)
        }
        #[doc = "VBAT charging resistor selection"]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: super::vals::Vbrs) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Bdcr2 {
        #[inline(always)]
        fn default() -> Bdcr2 {
            Bdcr2(0)
        }
    }
    impl core::fmt::Debug for Bdcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr2")
                .field("vbe", &self.vbe())
                .field("vbrs", &self.vbrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bdcr2 {{ vbe: {:?}, vbrs: {:?} }}", self.vbe(), self.vbrs())
        }
    }
    #[doc = "Backup domain status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdsr(pub u32);
    impl Bdsr {
        #[doc = "Backup domain voltage level monitoring versus high threshold"]
        #[inline(always)]
        pub const fn vbath(&self) -> super::vals::Vbath {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Vbath::from_bits(val as u8)
        }
        #[doc = "Backup domain voltage level monitoring versus high threshold"]
        #[inline(always)]
        pub fn set_vbath(&mut self, val: super::vals::Vbath) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Temperature level monitoring versus low threshold"]
        #[inline(always)]
        pub const fn templ(&self) -> super::vals::Templ {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Templ::from_bits(val as u8)
        }
        #[doc = "Temperature level monitoring versus low threshold"]
        #[inline(always)]
        pub fn set_templ(&mut self, val: super::vals::Templ) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Temperature level monitoring versus high threshold"]
        #[inline(always)]
        pub const fn temph(&self) -> super::vals::Temph {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Temph::from_bits(val as u8)
        }
        #[doc = "Temperature level monitoring versus high threshold"]
        #[inline(always)]
        pub fn set_temph(&mut self, val: super::vals::Temph) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Bdsr {
        #[inline(always)]
        fn default() -> Bdsr {
            Bdsr(0)
        }
    }
    impl core::fmt::Debug for Bdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdsr")
                .field("vbath", &self.vbath())
                .field("templ", &self.templ())
                .field("temph", &self.temph())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdsr {{ vbath: {:?}, templ: {:?}, temph: {:?} }}",
                self.vbath(),
                self.templ(),
                self.temph()
            )
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS=11X in CR1 with BREN=1 in BDCR1) 11x: Shutdown mode if BREN = 0 in BDCR1"]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS=11X in CR1 with BREN=1 in BDCR1) 11x: Shutdown mode if BREN = 0 in BDCR1"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub const fn rrsb1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub fn set_rrsb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub const fn rrsb2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode."]
        #[inline(always)]
        pub fn set_rrsb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes."]
        #[inline(always)]
        pub const fn ulpmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes."]
        #[inline(always)]
        pub fn set_ulpmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1."]
        #[inline(always)]
        pub const fn sram1pd(&self) -> super::vals::Srampd {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Srampd::from_bits(val as u8)
        }
        #[doc = "SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1."]
        #[inline(always)]
        pub fn set_sram1pd(&mut self, val: super::vals::Srampd) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2."]
        #[inline(always)]
        pub const fn sram2pd(&self) -> super::vals::Srampd {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Srampd::from_bits(val as u8)
        }
        #[doc = "SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2."]
        #[inline(always)]
        pub fn set_sram2pd(&mut self, val: super::vals::Srampd) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3."]
        #[inline(always)]
        pub const fn sram3pd(&self) -> super::vals::Srampd {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Srampd::from_bits(val as u8)
        }
        #[doc = "SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3."]
        #[inline(always)]
        pub fn set_sram3pd(&mut self, val: super::vals::Srampd) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4."]
        #[inline(always)]
        pub const fn sram4pd(&self) -> super::vals::Srampd {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Srampd::from_bits(val as u8)
        }
        #[doc = "SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4."]
        #[inline(always)]
        pub fn set_sram4pd(&mut self, val: super::vals::Srampd) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
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
                .field("ulpmen", &self.ulpmen())
                .field("sram1pd", &self.sram1pd())
                .field("sram2pd", &self.sram2pd())
                .field("sram3pd", &self.sram3pd())
                .field("sram4pd", &self.sram4pd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ lpms: {:?}, rrsb1: {=bool:?}, rrsb2: {=bool:?}, ulpmen: {=bool:?}, sram1pd: {:?}, sram2pd: {:?}, sram3pd: {:?}, sram4pd: {:?} }}" , self . lpms () , self . rrsb1 () , self . rrsb2 () , self . ulpmen () , self . sram1pd () , self . sram2pd () , self . sram3pd () , self . sram4pd ())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram1pds1(&self) -> super::vals::Pds {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram1pds1(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram1pds2(&self) -> super::vals::Pds {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram1pds2(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram1pds3(&self) -> super::vals::Pds {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram1pds3(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in CR1."]
        #[inline(always)]
        pub const fn sram2pds1(&self) -> super::vals::Pds {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in CR1."]
        #[inline(always)]
        pub fn set_sram2pds1(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in CR1."]
        #[inline(always)]
        pub const fn sram2pds2(&self) -> super::vals::Pds {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in CR1."]
        #[inline(always)]
        pub fn set_sram2pds2(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram4pds(&self) -> super::vals::Pds {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram4pds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn icrampds(&self) -> super::vals::Pds {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_icrampds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn dc1rampds(&self) -> super::vals::Pds {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_dc1rampds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn dma2drampds(&self) -> super::vals::Pds {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_dma2drampds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop0,1,2,3)"]
        #[inline(always)]
        pub const fn prampds(&self) -> super::vals::Pds {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop0,1,2,3)"]
        #[inline(always)]
        pub fn set_prampds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "PKA SRAM power-down"]
        #[inline(always)]
        pub const fn pkarampds(&self) -> super::vals::Pds {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "PKA SRAM power-down"]
        #[inline(always)]
        pub fn set_pkarampds(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes."]
        #[inline(always)]
        pub const fn sram4fwu(&self) -> super::vals::Sramfwu {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Sramfwu::from_bits(val as u8)
        }
        #[doc = "SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes."]
        #[inline(always)]
        pub fn set_sram4fwu(&mut self, val: super::vals::Sramfwu) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption."]
        #[inline(always)]
        pub const fn flashfwu(&self) -> super::vals::Flashfwu {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Flashfwu::from_bits(val as u8)
        }
        #[doc = "Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption."]
        #[inline(always)]
        pub fn set_flashfwu(&mut self, val: super::vals::Flashfwu) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram3pds1(&self) -> super::vals::Pds {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram3pds1(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram3pds2(&self) -> super::vals::Pds {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram3pds2(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram3pds3(&self) -> super::vals::Pds {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram3pds3(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram3pds4(&self) -> super::vals::Pds {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram3pds4(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram3pds5(&self) -> super::vals::Pds {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram3pds5(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram3pds6(&self) -> super::vals::Pds {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram3pds6(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram3pds7(&self) -> super::vals::Pds {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram3pds7(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub const fn sram3pds8(&self) -> super::vals::Pds {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Pds::from_bits(val as u8)
        }
        #[doc = "SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)"]
        #[inline(always)]
        pub fn set_sram3pds8(&mut self, val: super::vals::Pds) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "SmartRun domain in Run mode"]
        #[inline(always)]
        pub const fn srdrun(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SmartRun domain in Run mode"]
        #[inline(always)]
        pub fn set_srdrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("sram2pds1", &self.sram2pds1())
                .field("sram2pds2", &self.sram2pds2())
                .field("sram4pds", &self.sram4pds())
                .field("icrampds", &self.icrampds())
                .field("dc1rampds", &self.dc1rampds())
                .field("dma2drampds", &self.dma2drampds())
                .field("prampds", &self.prampds())
                .field("pkarampds", &self.pkarampds())
                .field("sram4fwu", &self.sram4fwu())
                .field("flashfwu", &self.flashfwu())
                .field("sram3pds1", &self.sram3pds1())
                .field("sram3pds2", &self.sram3pds2())
                .field("sram3pds3", &self.sram3pds3())
                .field("sram3pds4", &self.sram3pds4())
                .field("sram3pds5", &self.sram3pds5())
                .field("sram3pds6", &self.sram3pds6())
                .field("sram3pds7", &self.sram3pds7())
                .field("sram3pds8", &self.sram3pds8())
                .field("srdrun", &self.srdrun())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ sram1pds1: {:?}, sram1pds2: {:?}, sram1pds3: {:?}, sram2pds1: {:?}, sram2pds2: {:?}, sram4pds: {:?}, icrampds: {:?}, dc1rampds: {:?}, dma2drampds: {:?}, prampds: {:?}, pkarampds: {:?}, sram4fwu: {:?}, flashfwu: {:?}, sram3pds1: {:?}, sram3pds2: {:?}, sram3pds3: {:?}, sram3pds4: {:?}, sram3pds5: {:?}, sram3pds6: {:?}, sram3pds7: {:?}, sram3pds8: {:?}, srdrun: {=bool:?} }}" , self . sram1pds1 () , self . sram1pds2 () , self . sram1pds3 () , self . sram2pds1 () , self . sram2pds2 () , self . sram4pds () , self . icrampds () , self . dc1rampds () , self . dma2drampds () , self . prampds () , self . pkarampds () , self . sram4fwu () , self . flashfwu () , self . sram3pds1 () , self . sram3pds2 () , self . sram3pds3 () , self . sram3pds4 () , self . sram3pds5 () , self . sram3pds6 () , self . sram3pds7 () , self . sram3pds8 () , self . srdrun ())
        }
    }
    #[doc = "control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS."]
        #[inline(always)]
        pub const fn regsel(&self) -> super::vals::Regsel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Regsel::from_bits(val as u8)
        }
        #[doc = "Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS."]
        #[inline(always)]
        pub fn set_regsel(&mut self, val: super::vals::Regsel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Fast soft start"]
        #[inline(always)]
        pub const fn fsten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Fast soft start"]
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
    #[doc = "disable Backup domain register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbpcr(pub u32);
    impl Dbpcr {
        #[doc = "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers."]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers."]
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
    #[doc = "Power Port pull control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub const fn p(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_p(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    impl core::fmt::Debug for Pcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcr")
                .field("p[0]", &self.p(0usize))
                .field("p[1]", &self.p(1usize))
                .field("p[2]", &self.p(2usize))
                .field("p[3]", &self.p(3usize))
                .field("p[4]", &self.p(4usize))
                .field("p[5]", &self.p(5usize))
                .field("p[6]", &self.p(6usize))
                .field("p[7]", &self.p(7usize))
                .field("p[8]", &self.p(8usize))
                .field("p[9]", &self.p(9usize))
                .field("p[10]", &self.p(10usize))
                .field("p[11]", &self.p(11usize))
                .field("p[12]", &self.p(12usize))
                .field("p[13]", &self.p(13usize))
                .field("p[14]", &self.p(14usize))
                .field("p[15]", &self.p(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pcr {{ p[0]: {=bool:?}, p[1]: {=bool:?}, p[2]: {=bool:?}, p[3]: {=bool:?}, p[4]: {=bool:?}, p[5]: {=bool:?}, p[6]: {=bool:?}, p[7]: {=bool:?}, p[8]: {=bool:?}, p[9]: {=bool:?}, p[10]: {=bool:?}, p[11]: {=bool:?}, p[12]: {=bool:?}, p[13]: {=bool:?}, p[14]: {=bool:?}, p[15]: {=bool:?} }}" , self . p (0usize) , self . p (1usize) , self . p (2usize) , self . p (3usize) , self . p (4usize) , self . p (5usize) , self . p (6usize) , self . p (7usize) , self . p (8usize) , self . p (9usize) , self . p (10usize) , self . p (11usize) , self . p (12usize) , self . p (13usize) , self . p (14usize) , self . p (15usize))
        }
    }
    #[doc = "privilege control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "secure functions privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access."]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "secure functions privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "non-secure functions privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure."]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "non-secure functions privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure."]
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
    #[doc = "security configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "WUP1 secure protection"]
        #[inline(always)]
        pub const fn wup1sec(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "WUP1 secure protection"]
        #[inline(always)]
        pub fn set_wup1sec(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Low-power modes secure protection"]
        #[inline(always)]
        pub const fn lpmsec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power modes secure protection"]
        #[inline(always)]
        pub fn set_lpmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Voltage detection and monitoring secure protection"]
        #[inline(always)]
        pub const fn vdmsec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage detection and monitoring secure protection"]
        #[inline(always)]
        pub fn set_vdmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Backup domain secure protection"]
        #[inline(always)]
        pub const fn vbsec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain secure protection"]
        #[inline(always)]
        pub fn set_vbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Pull-up/pull-down secure protection"]
        #[inline(always)]
        pub const fn apcsec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Pull-up/pull-down secure protection"]
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
                .field("wup1sec[0]", &self.wup1sec(0usize))
                .field("wup1sec[1]", &self.wup1sec(1usize))
                .field("wup1sec[2]", &self.wup1sec(2usize))
                .field("wup1sec[3]", &self.wup1sec(3usize))
                .field("wup1sec[4]", &self.wup1sec(4usize))
                .field("wup1sec[5]", &self.wup1sec(5usize))
                .field("wup1sec[6]", &self.wup1sec(6usize))
                .field("wup1sec[7]", &self.wup1sec(7usize))
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
            defmt :: write ! (f , "Seccfgr {{ wup1sec[0]: {=bool:?}, wup1sec[1]: {=bool:?}, wup1sec[2]: {=bool:?}, wup1sec[3]: {=bool:?}, wup1sec[4]: {=bool:?}, wup1sec[5]: {=bool:?}, wup1sec[6]: {=bool:?}, wup1sec[7]: {=bool:?}, lpmsec: {=bool:?}, vdmsec: {=bool:?}, vbsec: {=bool:?}, apcsec: {=bool:?} }}" , self . wup1sec (0usize) , self . wup1sec (1usize) , self . wup1sec (2usize) , self . wup1sec (3usize) , self . wup1sec (4usize) , self . wup1sec (5usize) , self . wup1sec (6usize) , self . wup1sec (7usize) , self . lpmsec () , self . vdmsec () , self . vbsec () , self . apcsec ())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC=1 in SECCFGR. This bit is protected against unprivileged access when LPMSEC=1 and SPRIV=1 in PRIVCFGR, or when LPMSEC=0 and NSPRIV=1. Writing 1 to this bit clears the STOPF and SBF flags."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC=1 in SECCFGR. This bit is protected against unprivileged access when LPMSEC=1 and SPRIV=1 in PRIVCFGR, or when LPMSEC=0 and NSPRIV=1. Writing 1 to this bit clears the STOPF and SBF flags."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit."]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset."]
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
    #[doc = "supply voltage monitoring control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmcr(pub u32);
    impl Svmcr {
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:"]
        #[inline(always)]
        pub const fn pvdls(&self) -> super::vals::Pvdls {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Pvdls::from_bits(val as u8)
        }
        #[doc = "Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:"]
        #[inline(always)]
        pub fn set_pvdls(&mut self, val: super::vals::Pvdls) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "VDDUSB independent USB voltage monitor enable"]
        #[inline(always)]
        pub const fn uvmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VDDUSB independent USB voltage monitor enable"]
        #[inline(always)]
        pub fn set_uvmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "VDDIO2 independent I/Os voltage monitor enable"]
        #[inline(always)]
        pub const fn io2vmen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 independent I/Os voltage monitor enable"]
        #[inline(always)]
        pub fn set_io2vmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "VDDA independent analog supply voltage monitor 1 enable (1.6V threshold)"]
        #[inline(always)]
        pub const fn avm1en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "VDDA independent analog supply voltage monitor 1 enable (1.6V threshold)"]
        #[inline(always)]
        pub fn set_avm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "VDDA independent analog supply voltage monitor 2 enable (1.8V threshold)"]
        #[inline(always)]
        pub const fn avm2en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "VDDA independent analog supply voltage monitor 2 enable (1.8V threshold)"]
        #[inline(always)]
        pub fn set_avm2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "VDDUSB independent USB supply valid"]
        #[inline(always)]
        pub const fn usv(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "VDDUSB independent USB supply valid"]
        #[inline(always)]
        pub fn set_usv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\\[15:2\\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not."]
        #[inline(always)]
        pub const fn io2sv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\\[15:2\\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not."]
        #[inline(always)]
        pub fn set_io2sv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "VDDA independent analog supply valid"]
        #[inline(always)]
        pub const fn asv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "VDDA independent analog supply valid"]
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmsr(pub u32);
    impl Svmsr {
        #[doc = "Regulator selection"]
        #[inline(always)]
        pub const fn regs(&self) -> super::vals::Regsel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Regsel::from_bits(val as u8)
        }
        #[doc = "Regulator selection"]
        #[inline(always)]
        pub fn set_regs(&mut self, val: super::vals::Regsel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "VDD voltage detector output"]
        #[inline(always)]
        pub const fn pvdo(&self) -> super::vals::Pvdo {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Pvdo::from_bits(val as u8)
        }
        #[doc = "VDD voltage detector output"]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: super::vals::Pvdo) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Voltage level ready for currently used VOS"]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage level ready for currently used VOS"]
        #[inline(always)]
        pub fn set_actvosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VOS currently applied to VCORE This field provides the last VOS value."]
        #[inline(always)]
        pub const fn actvos(&self) -> super::vals::Actvos {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Actvos::from_bits(val as u8)
        }
        #[doc = "VOS currently applied to VCORE This field provides the last VOS value."]
        #[inline(always)]
        pub fn set_actvos(&mut self, val: super::vals::Actvos) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "VDDUSB ready"]
        #[inline(always)]
        pub const fn vddusbrdy(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VDDUSB ready"]
        #[inline(always)]
        pub fn set_vddusbrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "VDDIO2 ready"]
        #[inline(always)]
        pub const fn vddio2rdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 ready"]
        #[inline(always)]
        pub fn set_vddio2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "VDDA ready versus 1.6V voltage monitor"]
        #[inline(always)]
        pub const fn vdda1rdy(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "VDDA ready versus 1.6V voltage monitor"]
        #[inline(always)]
        pub fn set_vdda1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "VDDA ready versus 1.8V voltage monitor"]
        #[inline(always)]
        pub const fn vdda2rdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "VDDA ready versus 1.8V voltage monitor"]
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
                .field("actvosrdy", &self.actvosrdy())
                .field("actvos", &self.actvos())
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
            defmt :: write ! (f , "Svmsr {{ regs: {:?}, pvdo: {:?}, actvosrdy: {=bool:?}, actvos: {:?}, vddusbrdy: {=bool:?}, vddio2rdy: {=bool:?}, vdda1rdy: {=bool:?}, vdda2rdy: {=bool:?} }}" , self . regs () , self . pvdo () , self . actvosrdy () , self . actvos () , self . vddusbrdy () , self . vddio2rdy () , self . vdda1rdy () , self . vdda2rdy ())
        }
    }
    #[doc = "USB Type-C™ and Power Delivery register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucpdr(pub u32);
    impl Ucpdr {
        #[doc = "UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable)."]
        #[inline(always)]
        pub const fn ucpd_dbdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable)."]
        #[inline(always)]
        pub fn set_ucpd_dbdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers."]
        #[inline(always)]
        pub const fn ucpd_stby(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers."]
        #[inline(always)]
        pub fn set_ucpd_stby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ucpdr {
        #[inline(always)]
        fn default() -> Ucpdr {
            Ucpdr(0)
        }
    }
    impl core::fmt::Debug for Ucpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ucpdr")
                .field("ucpd_dbdis", &self.ucpd_dbdis())
                .field("ucpd_stby", &self.ucpd_stby())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ucpdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ucpdr {{ ucpd_dbdis: {=bool:?}, ucpd_stby: {=bool:?} }}",
                self.ucpd_dbdis(),
                self.ucpd_stby()
            )
        }
    }
    #[doc = "voltage scaling register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vosr(pub u32);
    impl Vosr {
        #[doc = "OTG_HS EPOD booster ready This bit is set to one by hardware when the power booster startup time is reached. The OTG_HS clock can be provided only after this bit is set."]
        #[inline(always)]
        pub const fn usbboostrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS EPOD booster ready This bit is set to one by hardware when the power booster startup time is reached. The OTG_HS clock can be provided only after this bit is set."]
        #[inline(always)]
        pub fn set_usbboostrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set."]
        #[inline(always)]
        pub const fn boostrdy(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set."]
        #[inline(always)]
        pub fn set_boostrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ready bit for VCORE voltage scaling output selection"]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ready bit for VCORE voltage scaling output selection"]
        #[inline(always)]
        pub fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1."]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1."]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "EPOD booster enable"]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "EPOD booster enable"]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "OTG_HS power enable"]
        #[inline(always)]
        pub const fn usbpwren(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS power enable"]
        #[inline(always)]
        pub fn set_usbpwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OTG_HS EPOD booster enable"]
        #[inline(always)]
        pub const fn usbboosten(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS EPOD booster enable"]
        #[inline(always)]
        pub fn set_usbboosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OTG_HS VDD11USB disable"]
        #[inline(always)]
        pub const fn vdd11usbdis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS VDD11USB disable"]
        #[inline(always)]
        pub fn set_vdd11usbdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("usbboostrdy", &self.usbboostrdy())
                .field("boostrdy", &self.boostrdy())
                .field("vosrdy", &self.vosrdy())
                .field("vos", &self.vos())
                .field("boosten", &self.boosten())
                .field("usbpwren", &self.usbpwren())
                .field("usbboosten", &self.usbboosten())
                .field("vdd11usbdis", &self.vdd11usbdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vosr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vosr {{ usbboostrdy: {=bool:?}, boostrdy: {=bool:?}, vosrdy: {=bool:?}, vos: {:?}, boosten: {=bool:?}, usbpwren: {=bool:?}, usbboosten: {=bool:?}, vdd11usbdis: {=bool:?} }}" , self . usbboostrdy () , self . boostrdy () , self . vosrdy () , self . vos () , self . boosten () , self . usbpwren () , self . usbboosten () , self . vdd11usbdis ())
        }
    }
    #[doc = "wakeup control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr1(pub u32);
    impl Wucr1 {
        #[doc = "Wakeup pin WKUP1 enable"]
        #[inline(always)]
        pub const fn wupen(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP1 enable"]
        #[inline(always)]
        pub fn set_wupen(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("wupen[0]", &self.wupen(0usize))
                .field("wupen[1]", &self.wupen(1usize))
                .field("wupen[2]", &self.wupen(2usize))
                .field("wupen[3]", &self.wupen(3usize))
                .field("wupen[4]", &self.wupen(4usize))
                .field("wupen[5]", &self.wupen(5usize))
                .field("wupen[6]", &self.wupen(6usize))
                .field("wupen[7]", &self.wupen(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wucr1 {{ wupen[0]: {=bool:?}, wupen[1]: {=bool:?}, wupen[2]: {=bool:?}, wupen[3]: {=bool:?}, wupen[4]: {=bool:?}, wupen[5]: {=bool:?}, wupen[6]: {=bool:?}, wupen[7]: {=bool:?} }}" , self . wupen (0usize) , self . wupen (1usize) , self . wupen (2usize) , self . wupen (3usize) , self . wupen (4usize) , self . wupen (5usize) , self . wupen (6usize) , self . wupen (7usize))
        }
    }
    #[doc = "wakeup control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr2(pub u32);
    impl Wucr2 {
        #[doc = "Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0."]
        #[inline(always)]
        pub const fn wupp(&self, n: usize) -> super::vals::Wupp {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0."]
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
    #[doc = "wakeup control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr3(pub u32);
    impl Wucr3 {
        #[doc = "Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0."]
        #[inline(always)]
        pub const fn wusel1(&self) -> super::vals::Wusel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0."]
        #[inline(always)]
        pub fn set_wusel1(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0."]
        #[inline(always)]
        pub const fn wusel2(&self) -> super::vals::Wusel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0."]
        #[inline(always)]
        pub fn set_wusel2(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0."]
        #[inline(always)]
        pub const fn wusel3(&self) -> super::vals::Wusel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0."]
        #[inline(always)]
        pub fn set_wusel3(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0."]
        #[inline(always)]
        pub const fn wusel4(&self) -> super::vals::Wusel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0."]
        #[inline(always)]
        pub fn set_wusel4(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0."]
        #[inline(always)]
        pub const fn wusel5(&self) -> super::vals::Wusel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0."]
        #[inline(always)]
        pub fn set_wusel5(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0."]
        #[inline(always)]
        pub const fn wusel6(&self) -> super::vals::Wusel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0."]
        #[inline(always)]
        pub fn set_wusel6(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0."]
        #[inline(always)]
        pub const fn wusel7(&self) -> super::vals::Wusel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0."]
        #[inline(always)]
        pub fn set_wusel7(&mut self, val: super::vals::Wusel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0."]
        #[inline(always)]
        pub const fn wusel8(&self) -> super::vals::Wusel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0."]
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
    #[doc = "wakeup status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wuscr(pub u32);
    impl Wuscr {
        #[doc = "Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wakeup flag 2 Writing 1 to this bit clears the WUF2 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 2 Writing 1 to this bit clears the WUF2 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup flag 3 Writing 1 to this bit clears the WUF3 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 3 Writing 1 to this bit clears the WUF3 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup flag 4 Writing 1 to this bit clears the WUF4 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 4 Writing 1 to this bit clears the WUF4 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wakeup flag 5 Writing 1 to this bit clears the WUF5 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 5 Writing 1 to this bit clears the WUF5 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Wakeup flag 6 Writing 1 to this bit clears the WUF6 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 6 Writing 1 to this bit clears the WUF6 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Wakeup flag 7 Writing 1 to this bit clears the WUF7 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 7 Writing 1 to this bit clears the WUF7 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Wakeup flag 8 Writing 1 to this bit clears the WUF8 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 8 Writing 1 to this bit clears the WUF8 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wuscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wuscr {{ cwuf1: {=bool:?}, cwuf2: {=bool:?}, cwuf3: {=bool:?}, cwuf4: {=bool:?}, cwuf5: {=bool:?}, cwuf6: {=bool:?}, cwuf7: {=bool:?}, cwuf8: {=bool:?} }}" , self . cwuf1 () , self . cwuf2 () , self . cwuf3 () , self . cwuf4 () , self . cwuf5 () , self . cwuf6 () , self . cwuf7 () , self . cwuf8 ())
        }
    }
    #[doc = "wakeup status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wusr(pub u32);
    impl Wusr {
        #[doc = "Wakeup flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN1=0."]
        #[inline(always)]
        pub const fn wuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN1=0."]
        #[inline(always)]
        pub fn set_wuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wakeup flag 2 This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN2=0."]
        #[inline(always)]
        pub const fn wuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 2 This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN2=0."]
        #[inline(always)]
        pub fn set_wuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup flag 3 This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN3=0."]
        #[inline(always)]
        pub const fn wuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 3 This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN3=0."]
        #[inline(always)]
        pub fn set_wuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup flag 4 This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN4=0."]
        #[inline(always)]
        pub const fn wuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 4 This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN4=0."]
        #[inline(always)]
        pub fn set_wuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wakeup flag 5 This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN5=0."]
        #[inline(always)]
        pub const fn wuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 5 This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN5=0."]
        #[inline(always)]
        pub fn set_wuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Wakeup flag 6 This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN6=0. If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 6 This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN6=0. If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Wakeup flag 7 This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN7=0. If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 7 This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN7=0. If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Wakeup flag 8 This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN8=0. If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub const fn wuf8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 8 This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN8=0. If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared."]
        #[inline(always)]
        pub fn set_wuf8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wusr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wusr {{ wuf1: {=bool:?}, wuf2: {=bool:?}, wuf3: {=bool:?}, wuf4: {=bool:?}, wuf5: {=bool:?}, wuf6: {=bool:?}, wuf7: {=bool:?}, wuf8: {=bool:?} }}" , self . wuf1 () , self . wuf2 () , self . wuf3 () , self . wuf4 () , self . wuf5 () , self . wuf6 () , self . wuf7 () , self . wuf8 ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Actvos {
        #[doc = "Range 4 (lowest power)"]
        RANGE4 = 0x0,
        #[doc = "Range 3"]
        RANGE3 = 0x01,
        #[doc = "Range 2"]
        RANGE2 = 0x02,
        #[doc = "Range 1 (highest frequency)"]
        RANGE1 = 0x03,
    }
    impl Actvos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Actvos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Actvos {
        #[inline(always)]
        fn from(val: u8) -> Actvos {
            Actvos::from_bits(val)
        }
    }
    impl From<Actvos> for u8 {
        #[inline(always)]
        fn from(val: Actvos) -> u8 {
            Actvos::to_bits(val)
        }
    }
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
        #[doc = "Stop 0 mode"]
        STOP0 = 0x0,
        #[doc = "Stop 1 mode"]
        STOP1 = 0x01,
        #[doc = "Stop 2 mode"]
        STOP2 = 0x02,
        #[doc = "Stop 3 mode"]
        STOP3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
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
        #[doc = "Content retained in Stop modes"]
        RETAINED = 0x0,
        #[doc = "Content lost in Stop modes"]
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
        #[doc = "VPVD0 around 2.0 V"]
        V20 = 0x0,
        #[doc = "VPVD1 around 2.2 V"]
        V22 = 0x01,
        #[doc = "VPVD2 around 2.4 V"]
        V24 = 0x02,
        #[doc = "VPVD3 around 2.5 V"]
        V25 = 0x03,
        #[doc = "VPVD4 around 2.6 V"]
        V26 = 0x04,
        #[doc = "VPVD5 around 2.8 V"]
        V28 = 0x05,
        #[doc = "VPVD6 around 2.9 V"]
        V29 = 0x06,
        #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
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
        #[doc = "LDO selected"]
        LDO = 0x0,
        #[doc = "SMPS selected"]
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
        #[doc = "SRAM4 enters low-power mode in Stop 0, 1 and 2 modes (source biasing for lower-power consumption)."]
        B_0X0 = 0x0,
        #[doc = "SRAM4 remains in normal mode in Stop 0, 1 and 2 modes (higher consumption but no SRAM4 wakeup time)."]
        B_0X1 = 0x01,
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
        #[doc = "SRAM1 powered on"]
        POWERED_ON = 0x0,
        #[doc = "SRAM1 powered off"]
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
    pub enum Temph {
        #[doc = "Temperature < high threshold"]
        B_0X0 = 0x0,
        #[doc = "Temperature ≥ high threshold"]
        B_0X1 = 0x01,
    }
    impl Temph {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Temph {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Temph {
        #[inline(always)]
        fn from(val: u8) -> Temph {
            Temph::from_bits(val)
        }
    }
    impl From<Temph> for u8 {
        #[inline(always)]
        fn from(val: Temph) -> u8 {
            Temph::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Templ {
        #[doc = "Temperature > low threshold"]
        B_0X0 = 0x0,
        #[doc = "Temperature ≤ low threshold"]
        B_0X1 = 0x01,
    }
    impl Templ {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Templ {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Templ {
        #[inline(always)]
        fn from(val: u8) -> Templ {
            Templ::from_bits(val)
        }
    }
    impl From<Templ> for u8 {
        #[inline(always)]
        fn from(val: Templ) -> u8 {
            Templ::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbath {
        #[doc = "Backup domain voltage level < high threshold"]
        B_0X0 = 0x0,
        #[doc = "Backup domain voltage level ≥ high threshold"]
        B_0X1 = 0x01,
    }
    impl Vbath {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbath {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbath {
        #[inline(always)]
        fn from(val: u8) -> Vbath {
            Vbath::from_bits(val)
        }
    }
    impl From<Vbath> for u8 {
        #[inline(always)]
        fn from(val: Vbath) -> u8 {
            Vbath::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbe {
        #[doc = "VBAT battery charging disabled"]
        B_0X0 = 0x0,
        #[doc = "VBAT battery charging enabled"]
        B_0X1 = 0x01,
    }
    impl Vbe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbe {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbe {
        #[inline(always)]
        fn from(val: u8) -> Vbe {
            Vbe::from_bits(val)
        }
    }
    impl From<Vbe> for u8 {
        #[inline(always)]
        fn from(val: Vbe) -> u8 {
            Vbe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbrs {
        #[doc = "Charge VBAT through a 5 kΩ resistor"]
        B_0X0 = 0x0,
        #[doc = "Charge VBAT through a 1.5 kΩ resistor"]
        B_0X1 = 0x01,
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
    pub enum Vos {
        #[doc = "Range 4 (lowest power)"]
        RANGE4 = 0x0,
        #[doc = "Range 3"]
        RANGE3 = 0x01,
        #[doc = "Range 2"]
        RANGE2 = 0x02,
        #[doc = "Range 1 (highest frequency). This value cannot be written when VCOREMEN = 1 in TAMP_OR register."]
        RANGE1 = 0x03,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vos {
        #[inline(always)]
        fn from(val: u8) -> Vos {
            Vos::from_bits(val)
        }
    }
    impl From<Vos> for u8 {
        #[inline(always)]
        fn from(val: Vos) -> u8 {
            Vos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wupp {
        #[doc = "Detection on high level (rising edge)"]
        HIGH = 0x0,
        #[doc = "Detection on low level (falling edge)"]
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
        #[doc = "WKUP7_0"]
        B_0X0 = 0x0,
        #[doc = "WKUP7_1"]
        B_0X1 = 0x01,
        #[doc = "WKUP7_2"]
        B_0X2 = 0x02,
        #[doc = "WKUP7_3"]
        B_0X3 = 0x03,
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
