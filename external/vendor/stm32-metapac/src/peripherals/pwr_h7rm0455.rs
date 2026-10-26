#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWR"]
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
    #[doc = "PWR control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PWR control status register 1"]
    #[inline(always)]
    pub const fn csr1(self) -> crate::common::Reg<regs::Csr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "This register allows controlling CPU1 power."]
    #[inline(always)]
    pub const fn cpucr(self) -> crate::common::Reg<regs::Cpucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
    #[inline(always)]
    pub const fn d3cr(self) -> crate::common::Reg<regs::D3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
    #[inline(always)]
    pub const fn wkupcr(self) -> crate::common::Reg<regs::Wkupcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "reset only by system reset, not reset by wakeup from Standby mode"]
    #[inline(always)]
    pub const fn wkupfr(self) -> crate::common::Reg<regs::Wkupfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Reset only by system reset, not reset by wakeup from Standby mode"]
    #[inline(always)]
    pub const fn wkupepr(self) -> crate::common::Reg<regs::Wkupepr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "This register allows controlling CPU1 power."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cpucr(pub u32);
    impl Cpucr {
        #[doc = "D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
        #[inline(always)]
        pub const fn pdds_d1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
        #[inline(always)]
        pub fn set_pdds_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
        #[inline(always)]
        pub const fn pdds_d2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
        #[inline(always)]
        pub fn set_pdds_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
        #[inline(always)]
        pub const fn pdds_d3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
        #[inline(always)]
        pub fn set_pdds_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
        #[inline(always)]
        pub const fn sbf_d1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
        #[inline(always)]
        pub fn set_sbf_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
        #[inline(always)]
        pub const fn sbf_d2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
        #[inline(always)]
        pub fn set_sbf_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
        #[inline(always)]
        pub const fn run_d3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
        #[inline(always)]
        pub fn set_run_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Cpucr {
        #[inline(always)]
        fn default() -> Cpucr {
            Cpucr(0)
        }
    }
    impl core::fmt::Debug for Cpucr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cpucr")
                .field("pdds_d1", &self.pdds_d1())
                .field("pdds_d2", &self.pdds_d2())
                .field("pdds_d3", &self.pdds_d3())
                .field("stopf", &self.stopf())
                .field("sbf", &self.sbf())
                .field("sbf_d1", &self.sbf_d1())
                .field("sbf_d2", &self.sbf_d2())
                .field("cssf", &self.cssf())
                .field("run_d3", &self.run_d3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cpucr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cpucr {{ pdds_d1: {=bool:?}, pdds_d2: {=bool:?}, pdds_d3: {=bool:?}, stopf: {=bool:?}, sbf: {=bool:?}, sbf_d1: {=bool:?}, sbf_d2: {=bool:?}, cssf: {=bool:?}, run_d3: {=bool:?} }}" , self . pdds_d1 () , self . pdds_d2 () , self . pdds_d3 () , self . stopf () , self . sbf () , self . sbf_d1 () , self . sbf_d2 () , self . cssf () , self . run_d3 ())
        }
    }
    #[doc = "PWR control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
        #[inline(always)]
        pub const fn lpds(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
        #[inline(always)]
        pub fn set_lpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Programmable voltage detector enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detector enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
        #[inline(always)]
        pub const fn pls(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
        #[inline(always)]
        pub fn set_pls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
        #[inline(always)]
        pub const fn flps(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
        #[inline(always)]
        pub fn set_flps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
        #[inline(always)]
        pub const fn svos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
        #[inline(always)]
        pub fn set_svos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Peripheral voltage monitor on VDDA enable"]
        #[inline(always)]
        pub const fn avden(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitor on VDDA enable"]
        #[inline(always)]
        pub fn set_avden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
        #[inline(always)]
        pub const fn als(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
        #[inline(always)]
        pub fn set_als(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
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
                .field("lpds", &self.lpds())
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("dbp", &self.dbp())
                .field("flps", &self.flps())
                .field("svos", &self.svos())
                .field("avden", &self.avden())
                .field("als", &self.als())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ lpds: {=bool:?}, pvde: {=bool:?}, pls: {=u8:?}, dbp: {=bool:?}, flps: {=bool:?}, svos: {=u8:?}, avden: {=bool:?}, als: {=u8:?} }}" , self . lpds () , self . pvde () , self . pls () , self . dbp () , self . flps () , self . svos () , self . avden () , self . als ())
        }
    }
    #[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
        #[inline(always)]
        pub const fn bren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
        #[inline(always)]
        pub fn set_bren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
        #[inline(always)]
        pub const fn monen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
        #[inline(always)]
        pub fn set_monen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
        #[inline(always)]
        pub const fn brrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
        #[inline(always)]
        pub fn set_brrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VBAT level monitoring versus low threshold"]
        #[inline(always)]
        pub const fn vbatl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT level monitoring versus low threshold"]
        #[inline(always)]
        pub fn set_vbatl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "VBAT level monitoring versus high threshold"]
        #[inline(always)]
        pub const fn vbath(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT level monitoring versus high threshold"]
        #[inline(always)]
        pub fn set_vbath(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Temperature level monitoring versus low threshold"]
        #[inline(always)]
        pub const fn templ(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature level monitoring versus low threshold"]
        #[inline(always)]
        pub fn set_templ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature level monitoring versus high threshold"]
        #[inline(always)]
        pub const fn temph(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature level monitoring versus high threshold"]
        #[inline(always)]
        pub fn set_temph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("bren", &self.bren())
                .field("monen", &self.monen())
                .field("brrdy", &self.brrdy())
                .field("vbatl", &self.vbatl())
                .field("vbath", &self.vbath())
                .field("templ", &self.templ())
                .field("temph", &self.temph())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ bren: {=bool:?}, monen: {=bool:?}, brrdy: {=bool:?}, vbatl: {=bool:?}, vbath: {=bool:?}, templ: {=bool:?}, temph: {=bool:?} }}" , self . bren () , self . monen () , self . brrdy () , self . vbatl () , self . vbath () , self . templ () , self . temph ())
        }
    }
    #[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Power management unit bypass"]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power management unit bypass"]
        #[inline(always)]
        pub fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Low drop-out regulator enable"]
        #[inline(always)]
        pub const fn ldoen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Low drop-out regulator enable"]
        #[inline(always)]
        pub fn set_ldoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SD converter Enable"]
        #[inline(always)]
        pub const fn sden(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SD converter Enable"]
        #[inline(always)]
        pub fn set_sden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Step-down converter forced ON and in High Power MR mode"]
        #[inline(always)]
        pub const fn sdexthp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Step-down converter forced ON and in High Power MR mode"]
        #[inline(always)]
        pub fn set_sdexthp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Step-down converter voltage output level selection"]
        #[inline(always)]
        pub const fn sdlevel(&self) -> super::vals::Sdlevel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Sdlevel::from_bits(val as u8)
        }
        #[doc = "Step-down converter voltage output level selection"]
        #[inline(always)]
        pub fn set_sdlevel(&mut self, val: super::vals::Sdlevel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "VBAT charging enable"]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT charging enable"]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VBAT charging resistor selection"]
        #[inline(always)]
        pub const fn vbrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT charging resistor selection"]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SMPS step-down converter external supply ready"]
        #[inline(always)]
        pub const fn sdextrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS step-down converter external supply ready"]
        #[inline(always)]
        pub fn set_sdextrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VDD33USB voltage level detector enable."]
        #[inline(always)]
        pub const fn usb33den(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VDD33USB voltage level detector enable."]
        #[inline(always)]
        pub fn set_usb33den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USB regulator enable."]
        #[inline(always)]
        pub const fn usbregen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB regulator enable."]
        #[inline(always)]
        pub fn set_usbregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB supply ready."]
        #[inline(always)]
        pub const fn usb33rdy(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB supply ready."]
        #[inline(always)]
        pub fn set_usb33rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("bypass", &self.bypass())
                .field("ldoen", &self.ldoen())
                .field("sden", &self.sden())
                .field("sdexthp", &self.sdexthp())
                .field("sdlevel", &self.sdlevel())
                .field("vbe", &self.vbe())
                .field("vbrs", &self.vbrs())
                .field("sdextrdy", &self.sdextrdy())
                .field("usb33den", &self.usb33den())
                .field("usbregen", &self.usbregen())
                .field("usb33rdy", &self.usb33rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ bypass: {=bool:?}, ldoen: {=bool:?}, sden: {=bool:?}, sdexthp: {=bool:?}, sdlevel: {:?}, vbe: {=bool:?}, vbrs: {=bool:?}, sdextrdy: {=bool:?}, usb33den: {=bool:?}, usbregen: {=bool:?}, usb33rdy: {=bool:?} }}" , self . bypass () , self . ldoen () , self . sden () , self . sdexthp () , self . sdlevel () , self . vbe () , self . vbrs () , self . sdextrdy () , self . usb33den () , self . usbregen () , self . usb33rdy ())
        }
    }
    #[doc = "PWR control status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr1(pub u32);
    impl Csr1 {
        #[doc = "Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Voltage levels ready bit for currently used VOS and SDLEVEL This bit is set to 1 by hardware when the voltage regulator and the SD converter are both disabled and Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage levels ready bit for currently used VOS and SDLEVEL This bit is set to 1 by hardware when the voltage regulator and the SD converter are both disabled and Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
        #[inline(always)]
        pub fn set_actvosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "VOS currently applied for VCORE voltage scaling selection. These bits reflect the last VOS value applied to the PMU."]
        #[inline(always)]
        pub const fn actvos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "VOS currently applied for VCORE voltage scaling selection. These bits reflect the last VOS value applied to the PMU."]
        #[inline(always)]
        pub fn set_actvos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
        #[inline(always)]
        pub const fn avdo(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
        #[inline(always)]
        pub fn set_avdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Csr1 {
        #[inline(always)]
        fn default() -> Csr1 {
            Csr1(0)
        }
    }
    impl core::fmt::Debug for Csr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr1")
                .field("pvdo", &self.pvdo())
                .field("actvosrdy", &self.actvosrdy())
                .field("actvos", &self.actvos())
                .field("avdo", &self.avdo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr1 {{ pvdo: {=bool:?}, actvosrdy: {=bool:?}, actvos: {=u8:?}, avdo: {=bool:?} }}",
                self.pvdo(),
                self.actvosrdy(),
                self.actvos(),
                self.avdo()
            )
        }
    }
    #[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3cr(pub u32);
    impl D3cr {
        #[doc = "VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
        #[inline(always)]
        pub fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
    }
    impl Default for D3cr {
        #[inline(always)]
        fn default() -> D3cr {
            D3cr(0)
        }
    }
    impl core::fmt::Debug for D3cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D3cr")
                .field("vosrdy", &self.vosrdy())
                .field("vos", &self.vos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "D3cr {{ vosrdy: {=bool:?}, vos: {:?} }}", self.vosrdy(), self.vos())
        }
    }
    #[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupcr(pub u32);
    impl Wkupcr {
        #[doc = "Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
        #[inline(always)]
        pub const fn wkupc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
        #[inline(always)]
        pub fn set_wkupc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Wkupcr {
        #[inline(always)]
        fn default() -> Wkupcr {
            Wkupcr(0)
        }
    }
    impl core::fmt::Debug for Wkupcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupcr").field("wkupc", &self.wkupc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wkupcr {{ wkupc: {=u8:?} }}", self.wkupc())
        }
    }
    #[doc = "Reset only by system reset, not reset by wakeup from Standby mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupepr(pub u32);
    impl Wkupepr {
        #[doc = "Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
        #[inline(always)]
        pub const fn wkupen(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
        #[inline(always)]
        pub fn set_wkupen(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
        #[inline(always)]
        pub const fn wkupp(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
        #[inline(always)]
        pub fn set_wkupp(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup pin pull configuration"]
        #[inline(always)]
        pub const fn wkuppupd(&self, n: usize) -> super::vals::Wkuppupd {
            assert!(n < 6usize);
            let offs = 16usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Wkuppupd::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration"]
        #[inline(always)]
        pub fn set_wkuppupd(&mut self, n: usize, val: super::vals::Wkuppupd) {
            assert!(n < 6usize);
            let offs = 16usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Wkupepr {
        #[inline(always)]
        fn default() -> Wkupepr {
            Wkupepr(0)
        }
    }
    impl core::fmt::Debug for Wkupepr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupepr")
                .field("wkupen[0]", &self.wkupen(0usize))
                .field("wkupen[1]", &self.wkupen(1usize))
                .field("wkupen[2]", &self.wkupen(2usize))
                .field("wkupen[3]", &self.wkupen(3usize))
                .field("wkupen[4]", &self.wkupen(4usize))
                .field("wkupen[5]", &self.wkupen(5usize))
                .field("wkupp[0]", &self.wkupp(0usize))
                .field("wkupp[1]", &self.wkupp(1usize))
                .field("wkupp[2]", &self.wkupp(2usize))
                .field("wkupp[3]", &self.wkupp(3usize))
                .field("wkupp[4]", &self.wkupp(4usize))
                .field("wkupp[5]", &self.wkupp(5usize))
                .field("wkuppupd[0]", &self.wkuppupd(0usize))
                .field("wkuppupd[1]", &self.wkuppupd(1usize))
                .field("wkuppupd[2]", &self.wkuppupd(2usize))
                .field("wkuppupd[3]", &self.wkuppupd(3usize))
                .field("wkuppupd[4]", &self.wkuppupd(4usize))
                .field("wkuppupd[5]", &self.wkuppupd(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupepr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wkupepr {{ wkupen[0]: {=bool:?}, wkupen[1]: {=bool:?}, wkupen[2]: {=bool:?}, wkupen[3]: {=bool:?}, wkupen[4]: {=bool:?}, wkupen[5]: {=bool:?}, wkupp[0]: {=bool:?}, wkupp[1]: {=bool:?}, wkupp[2]: {=bool:?}, wkupp[3]: {=bool:?}, wkupp[4]: {=bool:?}, wkupp[5]: {=bool:?}, wkuppupd[0]: {:?}, wkuppupd[1]: {:?}, wkuppupd[2]: {:?}, wkuppupd[3]: {:?}, wkuppupd[4]: {:?}, wkuppupd[5]: {:?} }}" , self . wkupen (0usize) , self . wkupen (1usize) , self . wkupen (2usize) , self . wkupen (3usize) , self . wkupen (4usize) , self . wkupen (5usize) , self . wkupp (0usize) , self . wkupp (1usize) , self . wkupp (2usize) , self . wkupp (3usize) , self . wkupp (4usize) , self . wkupp (5usize) , self . wkuppupd (0usize) , self . wkuppupd (1usize) , self . wkuppupd (2usize) , self . wkuppupd (3usize) , self . wkuppupd (4usize) , self . wkuppupd (5usize))
        }
    }
    #[doc = "reset only by system reset, not reset by wakeup from Standby mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupfr(pub u32);
    impl Wkupfr {
        #[doc = "Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
        #[inline(always)]
        pub const fn wkupf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
        #[inline(always)]
        pub fn set_wkupf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Wkupfr {
        #[inline(always)]
        fn default() -> Wkupfr {
            Wkupfr(0)
        }
    }
    impl core::fmt::Debug for Wkupfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupfr")
                .field("wkupf[0]", &self.wkupf(0usize))
                .field("wkupf[1]", &self.wkupf(1usize))
                .field("wkupf[2]", &self.wkupf(2usize))
                .field("wkupf[3]", &self.wkupf(3usize))
                .field("wkupf[4]", &self.wkupf(4usize))
                .field("wkupf[5]", &self.wkupf(5usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupfr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wkupfr {{ wkupf[0]: {=bool:?}, wkupf[1]: {=bool:?}, wkupf[2]: {=bool:?}, wkupf[3]: {=bool:?}, wkupf[4]: {=bool:?}, wkupf[5]: {=bool:?} }}" , self . wkupf (0usize) , self . wkupf (1usize) , self . wkupf (2usize) , self . wkupf (3usize) , self . wkupf (4usize) , self . wkupf (5usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdlevel {
        RESET = 0x0,
        V1_8 = 0x01,
        V2_5 = 0x02,
        V2_5_ALT = 0x03,
    }
    impl Sdlevel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdlevel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdlevel {
        #[inline(always)]
        fn from(val: u8) -> Sdlevel {
            Sdlevel::from_bits(val)
        }
    }
    impl From<Sdlevel> for u8 {
        #[inline(always)]
        fn from(val: Sdlevel) -> u8 {
            Sdlevel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vos {
        SCALE3 = 0x0,
        SCALE2 = 0x01,
        SCALE1 = 0x02,
        SCALE0 = 0x03,
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
    pub enum Wkuppupd {
        #[doc = "No pull-up."]
        NO_PULL = 0x0,
        #[doc = "Pull-up."]
        PULL_UP = 0x01,
        #[doc = "Pull-down."]
        PULL_DOWN = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd {
            Wkuppupd::from_bits(val)
        }
    }
    impl From<Wkuppupd> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd) -> u8 {
            Wkuppupd::to_bits(val)
        }
    }
}
