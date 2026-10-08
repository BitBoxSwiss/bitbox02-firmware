#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Power control."]
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
    #[doc = "PWR control status register 1."]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PWR control status register 1."]
    #[inline(always)]
    pub const fn csr1(self) -> crate::common::Reg<regs::Csr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PWR control register 2."]
    #[inline(always)]
    pub const fn csr2(self) -> crate::common::Reg<regs::Csr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PWR CPU control register 3."]
    #[inline(always)]
    pub const fn csr3(self) -> crate::common::Reg<regs::Csr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PWR control status register 4."]
    #[inline(always)]
    pub const fn csr4(self) -> crate::common::Reg<regs::Csr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PWR wakeup clear register."]
    #[inline(always)]
    pub const fn wkupcr(self) -> crate::common::Reg<regs::Wkupcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PWR wakeup flag register."]
    #[inline(always)]
    pub const fn wkupfr(self) -> crate::common::Reg<regs::Wkupfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PWR wakeup enable and polarity register."]
    #[inline(always)]
    pub const fn wkupepr(self) -> crate::common::Reg<regs::Wkupepr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PWR USB Type-C and Power Delivery register."]
    #[inline(always)]
    pub const fn ucpdr(self) -> crate::common::Reg<regs::Ucpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "PWR apply pull configuration register."]
    #[inline(always)]
    pub const fn apcr(self) -> crate::common::Reg<regs::Apcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "PWR port N pull-up control register."]
    #[inline(always)]
    pub const fn pucrn(self) -> crate::common::Reg<regs::Pucrn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "PWR port N pull-down control register."]
    #[inline(always)]
    pub const fn pdcrn(self) -> crate::common::Reg<regs::Pdcrn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "PWR port O pull-up control register."]
    #[inline(always)]
    pub const fn pucro(self) -> crate::common::Reg<regs::Pucro, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "PWR port O pull-down control register."]
    #[inline(always)]
    pub const fn pdcro(self) -> crate::common::Reg<regs::Pdcro, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "PWR port P pull-down control register."]
    #[inline(always)]
    pub const fn pdcrp(self) -> crate::common::Reg<regs::Pdcrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "PWR debug register 1."]
    #[inline(always)]
    pub const fn pdr1(self) -> crate::common::Reg<regs::Pdr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWR apply pull configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apcr(pub u32);
    impl Apcr {
        #[doc = "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx, PDCRx registers are applied in Standby mode even after wakeup until APC bit is reset to 0. When this bit is cleared, the I/O pull-up or pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx and PDCRx registers are not applied in Standby mode and IO becomes Hi-Z."]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx, PDCRx registers are applied in Standby mode even after wakeup until APC bit is reset to 0. When this bit is cleared, the I/O pull-up or pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx and PDCRx registers are not applied in Standby mode and IO becomes Hi-Z."]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port N bit 7 pull-up/down configuration When this bit is set, a weak pull-up or pull-down resistor is applied on PN7 following inverse logic applied on PN6. If the PUN6 bit in PWR_PUCRN register is set and APC bit is set the week pull-down is applied on PN7. If the PDN6 bit in PWR_PDCRN register is set and APC bit is set the week pull-up is applied on PN7."]
        #[inline(always)]
        pub const fn pn7_pupd(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Port N bit 7 pull-up/down configuration When this bit is set, a weak pull-up or pull-down resistor is applied on PN7 following inverse logic applied on PN6. If the PUN6 bit in PWR_PUCRN register is set and APC bit is set the week pull-down is applied on PN7. If the PDN6 bit in PWR_PDCRN register is set and APC bit is set the week pull-up is applied on PN7."]
        #[inline(always)]
        pub fn set_pn7_pupd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Port O bit 5 pull-up/down configuration When this bit is set, a weak pull-up or pull down resistor is applied on PO5 following inverse logic applied on PO4. If the PUO4 bit in PWR_PUCRO register is set and APC bit is set the week pull-down is applied on PO5. If the PDO4 bit in PWR_PDCRO register is set and APC bit is set the week pull-up is applied on PO5.."]
        #[inline(always)]
        pub const fn po5_pupd(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Port O bit 5 pull-up/down configuration When this bit is set, a weak pull-up or pull down resistor is applied on PO5 following inverse logic applied on PO4. If the PUO4 bit in PWR_PUCRO register is set and APC bit is set the week pull-down is applied on PO5. If the PDO4 bit in PWR_PDCRO register is set and APC bit is set the week pull-up is applied on PO5.."]
        #[inline(always)]
        pub fn set_po5_pupd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Port PB6 I3C pull-up bit When I3C is used on PB6, when set, this bit activates the pull-up on I3C1_SCL (PB6) in standby mode."]
        #[inline(always)]
        pub const fn i3cpb6_pu(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB6 I3C pull-up bit When I3C is used on PB6, when set, this bit activates the pull-up on I3C1_SCL (PB6) in standby mode."]
        #[inline(always)]
        pub fn set_i3cpb6_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Port PB7 I3C pull-up bit When I3C is used on PB7, when set, this bit activates the pull-up on I3C1_SDA (PB7) in standby mode."]
        #[inline(always)]
        pub const fn i3cpb7_pu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB7 I3C pull-up bit When I3C is used on PB7, when set, this bit activates the pull-up on I3C1_SDA (PB7) in standby mode."]
        #[inline(always)]
        pub fn set_i3cpb7_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Port PB8 I3C pull-up bit When I3C is used on PB8, when set, this bit activates the pull-up on I3C1_SCL (PB8) in standby mode."]
        #[inline(always)]
        pub const fn i3cpb8_pu(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB8 I3C pull-up bit When I3C is used on PB8, when set, this bit activates the pull-up on I3C1_SCL (PB8) in standby mode."]
        #[inline(always)]
        pub fn set_i3cpb8_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Port PB9 I3C pull-up bit When I3C is used on PB9, when set, this bit activates the pull-up on I3C1_SDA (PB9) in standby mode."]
        #[inline(always)]
        pub const fn i3cpb9_pu(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB9 I3C pull-up bit When I3C is used on PB9, when set, this bit activates the pull-up on I3C1_SDA (PB9) in standby mode."]
        #[inline(always)]
        pub fn set_i3cpb9_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            f.debug_struct("Apcr")
                .field("apc", &self.apc())
                .field("pn7_pupd", &self.pn7_pupd())
                .field("po5_pupd", &self.po5_pupd())
                .field("i3cpb6_pu", &self.i3cpb6_pu())
                .field("i3cpb7_pu", &self.i3cpb7_pu())
                .field("i3cpb8_pu", &self.i3cpb8_pu())
                .field("i3cpb9_pu", &self.i3cpb9_pu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apcr {{ apc: {=bool:?}, pn7_pupd: {=bool:?}, po5_pupd: {=bool:?}, i3cpb6_pu: {=bool:?}, i3cpb7_pu: {=bool:?}, i3cpb8_pu: {=bool:?}, i3cpb9_pu: {=bool:?} }}" , self . apc () , self . pn7_pupd () , self . po5_pupd () , self . i3cpb6_pu () , self . i3cpb7_pu () , self . i3cpb8_pu () , self . i3cpb9_pu ())
        }
    }
    #[doc = "PWR control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "System Stop mode voltage scaling selection."]
        #[inline(always)]
        pub const fn svos(&self) -> super::vals::Svos {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Svos::from_bits(val as u8)
        }
        #[doc = "System Stop mode voltage scaling selection."]
        #[inline(always)]
        pub fn set_svos(&mut self, val: super::vals::Svos) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Programmable voltage detector enable."]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detector enable."]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
        #[inline(always)]
        pub fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in the PWR_CSR1 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in the PWR_CSR1 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Flash low-power mode in Stop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when device is in Stop mode. consumption)."]
        #[inline(always)]
        pub const fn flps(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Flash low-power mode in Stop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when device is in Stop mode. consumption)."]
        #[inline(always)]
        pub fn set_flps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RAM low power mode disable in STOP. When set the RAMs will not enter to low power mode when the system enters to STOP."]
        #[inline(always)]
        pub const fn rlpsn(&self) -> super::vals::Rlpsn {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Rlpsn::from_bits(val as u8)
        }
        #[doc = "RAM low power mode disable in STOP. When set the RAMs will not enter to low power mode when the system enters to STOP."]
        #[inline(always)]
        pub fn set_rlpsn(&mut self, val: super::vals::Rlpsn) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "analog switch VBoost control This bit enables the booster to guarantee the analog switch AC performance when the VDD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The VDD supply voltage can be monitored through the PVD and the PLS bits."]
        #[inline(always)]
        pub const fn booste(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "analog switch VBoost control This bit enables the booster to guarantee the analog switch AC performance when the VDD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The VDD supply voltage can be monitored through the PVD and the PLS bits."]
        #[inline(always)]
        pub fn set_booste(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected VDDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_CSR1 register) after setting the AVDEN bit and selecting the supply level to be monitored (ALS bits)."]
        #[inline(always)]
        pub const fn avdready(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected VDDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_CSR1 register) after setting the AVDEN bit and selecting the supply level to be monitored (ALS bits)."]
        #[inline(always)]
        pub fn set_avdready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Peripheral voltage monitor on VDDA enable."]
        #[inline(always)]
        pub const fn avden(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitor on VDDA enable."]
        #[inline(always)]
        pub fn set_avden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Analog voltage detector level selection These bits select the voltage threshold detected by the AVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
        #[inline(always)]
        pub const fn als(&self) -> super::vals::Als {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Als::from_bits(val as u8)
        }
        #[doc = "Analog voltage detector level selection These bits select the voltage threshold detected by the AVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
        #[inline(always)]
        pub fn set_als(&mut self, val: super::vals::Als) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
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
                .field("svos", &self.svos())
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("dbp", &self.dbp())
                .field("flps", &self.flps())
                .field("rlpsn", &self.rlpsn())
                .field("booste", &self.booste())
                .field("avdready", &self.avdready())
                .field("avden", &self.avden())
                .field("als", &self.als())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ svos: {:?}, pvde: {=bool:?}, pls: {:?}, dbp: {=bool:?}, flps: {=bool:?}, rlpsn: {:?}, booste: {=bool:?}, avdready: {=bool:?}, avden: {=bool:?}, als: {:?} }}" , self . svos () , self . pvde () , self . pls () , self . dbp () , self . flps () , self . rlpsn () , self . booste () , self . avdready () , self . avden () , self . als ())
        }
    }
    #[doc = "PWR control status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr1(pub u32);
    impl Csr1 {
        #[doc = "Backup regulator enable When set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and V<sub>BAT</sub> modes."]
        #[inline(always)]
        pub const fn bren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Backup regulator enable When set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and V<sub>BAT</sub> modes."]
        #[inline(always)]
        pub fn set_bren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "V<sub>BAT</sub> and temperature monitoring enable When set, the V<sub>BAT</sub> supply and temperature monitoring is enabled. Note: V<sub>BAT</sub> and temperature monitoring are only available when the backup regulator is enabled (BREN bit set to 1)."]
        #[inline(always)]
        pub const fn monen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>BAT</sub> and temperature monitoring enable When set, the V<sub>BAT</sub> supply and temperature monitoring is enabled. Note: V<sub>BAT</sub> and temperature monitoring are only available when the backup regulator is enabled (BREN bit set to 1)."]
        #[inline(always)]
        pub fn set_monen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
        #[inline(always)]
        pub const fn brrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
        #[inline(always)]
        pub fn set_brrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "V<sub>BAT</sub> level monitoring versus low threshold."]
        #[inline(always)]
        pub const fn vbatl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>BAT</sub> level monitoring versus low threshold."]
        #[inline(always)]
        pub fn set_vbatl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "V<sub>BAT</sub> level monitoring versus high threshold."]
        #[inline(always)]
        pub const fn vbath(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>BAT</sub> level monitoring versus high threshold."]
        #[inline(always)]
        pub fn set_vbath(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Temperature level monitoring versus low threshold."]
        #[inline(always)]
        pub const fn templ(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature level monitoring versus low threshold."]
        #[inline(always)]
        pub fn set_templ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature level monitoring versus high threshold."]
        #[inline(always)]
        pub const fn temph(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature level monitoring versus high threshold."]
        #[inline(always)]
        pub fn set_temph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
    impl defmt::Format for Csr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr1 {{ bren: {=bool:?}, monen: {=bool:?}, brrdy: {=bool:?}, vbatl: {=bool:?}, vbath: {=bool:?}, templ: {=bool:?}, temph: {=bool:?} }}" , self . bren () , self . monen () , self . brrdy () , self . vbatl () , self . vbath () , self . templ () , self . temph ())
        }
    }
    #[doc = "PWR control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr2(pub u32);
    impl Csr2 {
        #[doc = "Power management unit bypass Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41."]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power management unit bypass Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41."]
        #[inline(always)]
        pub fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Low drop-out regulator enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41."]
        #[inline(always)]
        pub const fn ldoen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Low drop-out regulator enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41."]
        #[inline(always)]
        pub fn set_ldoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SMPS step-down converter enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41."]
        #[inline(always)]
        pub const fn sden(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS step-down converter enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41."]
        #[inline(always)]
        pub fn set_sden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SMPS external power delivery selection Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41."]
        #[inline(always)]
        pub const fn sdexthp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS external power delivery selection Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41."]
        #[inline(always)]
        pub fn set_sdexthp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SMPS step-down converter voltage output for LDO or external supply This bit is used when both the LDO and SMPS step-down converter are enabled with SDEN and LDOEN enabled or when SMPSEXTHP is enabled. In this case SDHILEVEL has to be set to 1 to confirm the regulator settings."]
        #[inline(always)]
        pub const fn sdlevel(&self) -> super::vals::Sdlevel {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Sdlevel::from_bits(val as u8)
        }
        #[doc = "SMPS step-down converter voltage output for LDO or external supply This bit is used when both the LDO and SMPS step-down converter are enabled with SDEN and LDOEN enabled or when SMPSEXTHP is enabled. In this case SDHILEVEL has to be set to 1 to confirm the regulator settings."]
        #[inline(always)]
        pub fn set_sdlevel(&mut self, val: super::vals::Sdlevel) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "VBAT charging enable."]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT charging enable."]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VBAT charging resistor selection."]
        #[inline(always)]
        pub const fn vbrs(&self) -> super::vals::Vbrs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vbrs::from_bits(val as u8)
        }
        #[doc = "VBAT charging resistor selection."]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: super::vals::Vbrs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "XSPI port 1 capacitor control bits see the product datasheet for more details."]
        #[inline(always)]
        pub const fn xspicap1(&self) -> super::vals::Xspicap {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Xspicap::from_bits(val as u8)
        }
        #[doc = "XSPI port 1 capacitor control bits see the product datasheet for more details."]
        #[inline(always)]
        pub fn set_xspicap1(&mut self, val: super::vals::Xspicap) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "XSPI port 2 capacitor control bits see the product datasheet for more details."]
        #[inline(always)]
        pub const fn xspicap2(&self) -> super::vals::Xspicap {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Xspicap::from_bits(val as u8)
        }
        #[doc = "XSPI port 2 capacitor control bits see the product datasheet for more details."]
        #[inline(always)]
        pub fn set_xspicap2(&mut self, val: super::vals::Xspicap) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "EN_XSPIM1: this bit allow the SW to enable the XSPI interface. The XSPIM_P1 supply must be stable prior to setting this bit."]
        #[inline(always)]
        pub const fn en_xspim1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "EN_XSPIM1: this bit allow the SW to enable the XSPI interface. The XSPIM_P1 supply must be stable prior to setting this bit."]
        #[inline(always)]
        pub fn set_en_xspim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "EN_XSPIM2: this bit allows the SW to enable the XSPI interface, when available. The XSPIM_P2 supply must be stable prior to setting this bit. It should also be set when FMC is used."]
        #[inline(always)]
        pub const fn en_xspim2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "EN_XSPIM2: this bit allows the SW to enable the XSPI interface, when available. The XSPIM_P2 supply must be stable prior to setting this bit. It should also be set when FMC is used."]
        #[inline(always)]
        pub fn set_en_xspim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SMPS step-down converter external supply ready This bit is set by hardware to indicate that the external supply from the SMPS step-down converter is ready."]
        #[inline(always)]
        pub const fn sdextrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS step-down converter external supply ready This bit is set by hardware to indicate that the external supply from the SMPS step-down converter is ready."]
        #[inline(always)]
        pub fn set_sdextrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VDD33_USB voltage level detector enable."]
        #[inline(always)]
        pub const fn usb33den(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VDD33_USB voltage level detector enable."]
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
        #[doc = "USB HS regulator enable."]
        #[inline(always)]
        pub const fn usbhsregen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB HS regulator enable."]
        #[inline(always)]
        pub fn set_usbhsregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Csr2 {
        #[inline(always)]
        fn default() -> Csr2 {
            Csr2(0)
        }
    }
    impl core::fmt::Debug for Csr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr2")
                .field("bypass", &self.bypass())
                .field("ldoen", &self.ldoen())
                .field("sden", &self.sden())
                .field("sdexthp", &self.sdexthp())
                .field("sdlevel", &self.sdlevel())
                .field("vbe", &self.vbe())
                .field("vbrs", &self.vbrs())
                .field("xspicap1", &self.xspicap1())
                .field("xspicap2", &self.xspicap2())
                .field("en_xspim1", &self.en_xspim1())
                .field("en_xspim2", &self.en_xspim2())
                .field("sdextrdy", &self.sdextrdy())
                .field("usb33den", &self.usb33den())
                .field("usbregen", &self.usbregen())
                .field("usb33rdy", &self.usb33rdy())
                .field("usbhsregen", &self.usbhsregen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr2 {{ bypass: {=bool:?}, ldoen: {=bool:?}, sden: {=bool:?}, sdexthp: {=bool:?}, sdlevel: {:?}, vbe: {=bool:?}, vbrs: {:?}, xspicap1: {:?}, xspicap2: {:?}, en_xspim1: {=bool:?}, en_xspim2: {=bool:?}, sdextrdy: {=bool:?}, usb33den: {=bool:?}, usbregen: {=bool:?}, usb33rdy: {=bool:?}, usbhsregen: {=bool:?} }}" , self . bypass () , self . ldoen () , self . sden () , self . sdexthp () , self . sdlevel () , self . vbe () , self . vbrs () , self . xspicap1 () , self . xspicap2 () , self . en_xspim1 () , self . en_xspim2 () , self . sdextrdy () , self . usb33den () , self . usbregen () , self . usb33rdy () , self . usbhsregen ())
        }
    }
    #[doc = "PWR CPU control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr3(pub u32);
    impl Csr3 {
        #[doc = "Power Down Deepsleep. This bit allows CPU to define the Deepsleep mode."]
        #[inline(always)]
        pub const fn pdds(&self) -> super::vals::Pdds {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Pdds::from_bits(val as u8)
        }
        #[doc = "Power Down Deepsleep. This bit allows CPU to define the Deepsleep mode."]
        #[inline(always)]
        pub fn set_pdds(&mut self, val: super::vals::Pdds) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU CSSF bit."]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU CSSF bit."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU CSSF bit."]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU CSSF bit."]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Csr3 {
        #[inline(always)]
        fn default() -> Csr3 {
            Csr3(0)
        }
    }
    impl core::fmt::Debug for Csr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr3")
                .field("pdds", &self.pdds())
                .field("cssf", &self.cssf())
                .field("stopf", &self.stopf())
                .field("sbf", &self.sbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr3 {{ pdds: {:?}, cssf: {=bool:?}, stopf: {=bool:?}, sbf: {=bool:?} }}",
                self.pdds(),
                self.cssf(),
                self.stopf(),
                self.sbf()
            )
        }
    }
    #[doc = "PWR control status register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr4(pub u32);
    impl Csr4 {
        #[doc = "Voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling must be changed before increasing the system frequency. When decreasing performance, the system frequency must first be decreased before changing the voltage scaling. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling must be changed before increasing the system frequency. When decreasing performance, the system frequency must first be decreased before changing the voltage scaling. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "VOS Ready bit."]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VOS Ready bit."]
        #[inline(always)]
        pub fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Csr4 {
        #[inline(always)]
        fn default() -> Csr4 {
            Csr4(0)
        }
    }
    impl core::fmt::Debug for Csr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr4")
                .field("vos", &self.vos())
                .field("vosrdy", &self.vosrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csr4 {{ vos: {:?}, vosrdy: {=bool:?} }}", self.vos(), self.vosrdy())
        }
    }
    #[doc = "PWR port N pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrn(pub u32);
    impl Pdcrn {
        #[doc = "Port N pull-down bit 0 When set activates the pull-down on PN0 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdn0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port N pull-down bit 0 When set activates the pull-down on PN0 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdn0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port N pull-down bit 1 When set activates the pull-down on PN1 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdn1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port N pull-down bit 1 When set activates the pull-down on PN1 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdn1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port N PN2 to PN5 pull-down activation When set, four pull-down resistors are activated on PN2 to PN5 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdn2n5(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port N PN2 to PN5 pull-down activation When set, four pull-down resistors are activated on PN2 to PN5 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdn2n5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port N pull-down bit 6 When set activates the pull-down on PN6 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdn6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port N pull-down bit 6 When set activates the pull-down on PN6 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdn6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port N - PN8 to PN11 pull-down activation When set, four pull-down resistors are activated on PN8 to PN11 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdn8n11(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port N - PN8 to PN11 pull-down activation When set, four pull-down resistors are activated on PN8 to PN11 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdn8n11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port N pull-down bit 12 When set activates the pull-down on PN12 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdn12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port N pull-down bit 12 When set activates the pull-down on PN12 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdn12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Pdcrn {
        #[inline(always)]
        fn default() -> Pdcrn {
            Pdcrn(0)
        }
    }
    impl core::fmt::Debug for Pdcrn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcrn")
                .field("pdn0", &self.pdn0())
                .field("pdn1", &self.pdn1())
                .field("pdn2n5", &self.pdn2n5())
                .field("pdn6", &self.pdn6())
                .field("pdn8n11", &self.pdn8n11())
                .field("pdn12", &self.pdn12())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcrn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pdcrn {{ pdn0: {=bool:?}, pdn1: {=bool:?}, pdn2n5: {=bool:?}, pdn6: {=bool:?}, pdn8n11: {=bool:?}, pdn12: {=bool:?} }}" , self . pdn0 () , self . pdn1 () , self . pdn2n5 () , self . pdn6 () , self . pdn8n11 () , self . pdn12 ())
        }
    }
    #[doc = "PWR port O pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcro(pub u32);
    impl Pdcro {
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdo0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdo0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdo1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdo1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdo2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdo2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdo3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdo3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdo4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdo4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Pdcro {
        #[inline(always)]
        fn default() -> Pdcro {
            Pdcro(0)
        }
    }
    impl core::fmt::Debug for Pdcro {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcro")
                .field("pdo0", &self.pdo0())
                .field("pdo1", &self.pdo1())
                .field("pdo2", &self.pdo2())
                .field("pdo3", &self.pdo3())
                .field("pdo4", &self.pdo4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcro {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdcro {{ pdo0: {=bool:?}, pdo1: {=bool:?}, pdo2: {=bool:?}, pdo3: {=bool:?}, pdo4: {=bool:?} }}",
                self.pdo0(),
                self.pdo1(),
                self.pdo2(),
                self.pdo3(),
                self.pdo4()
            )
        }
    }
    #[doc = "PWR port P pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrp(pub u32);
    impl Pdcrp {
        #[doc = "Port P0-P3 pull-down activation When set, four pull-down resistors are activated on P0 to P3 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdp0p3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port P0-P3 pull-down activation When set, four pull-down resistors are activated on P0 to P3 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdp0p3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port P4-P7 pull-down activation When set, four pull-down resitors are activated on P4 to P7 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdp4p7(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port P4-P7 pull-down activation When set, four pull-down resitors are activated on P4 to P7 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdp4p7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port P8-P11 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdp8p11(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port P8-P11 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdp8p11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port P12-P15 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub const fn pdp12p15(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port P12-P15 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR."]
        #[inline(always)]
        pub fn set_pdp12p15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Pdcrp {
        #[inline(always)]
        fn default() -> Pdcrp {
            Pdcrp(0)
        }
    }
    impl core::fmt::Debug for Pdcrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcrp")
                .field("pdp0p3", &self.pdp0p3())
                .field("pdp4p7", &self.pdp4p7())
                .field("pdp8p11", &self.pdp8p11())
                .field("pdp12p15", &self.pdp12p15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdcrp {{ pdp0p3: {=bool:?}, pdp4p7: {=bool:?}, pdp8p11: {=bool:?}, pdp12p15: {=bool:?} }}",
                self.pdp0p3(),
                self.pdp4p7(),
                self.pdp8p11(),
                self.pdp12p15()
            )
        }
    }
    #[doc = "PWR debug register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdr1(pub u32);
    impl Pdr1 {
        #[doc = "Debug Register Unlocked."]
        #[inline(always)]
        pub const fn unlocked(&self) -> super::vals::Unlocked {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Unlocked::from_bits(val as u8)
        }
        #[doc = "Debug Register Unlocked."]
        #[inline(always)]
        pub fn set_unlocked(&mut self, val: super::vals::Unlocked) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Step down converter force PWM mode."]
        #[inline(always)]
        pub const fn sdfpwmen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Step down converter force PWM mode."]
        #[inline(always)]
        pub fn set_sdfpwmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "(Non-User bit)."]
        #[inline(always)]
        pub const fn sync_adc(&self) -> super::vals::SyncAdc {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::SyncAdc::from_bits(val as u8)
        }
        #[doc = "(Non-User bit)."]
        #[inline(always)]
        pub fn set_sync_adc(&mut self, val: super::vals::SyncAdc) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Pdr1 {
        #[inline(always)]
        fn default() -> Pdr1 {
            Pdr1(0)
        }
    }
    impl core::fmt::Debug for Pdr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdr1")
                .field("unlocked", &self.unlocked())
                .field("sdfpwmen", &self.sdfpwmen())
                .field("sync_adc", &self.sync_adc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdr1 {{ unlocked: {:?}, sdfpwmen: {=bool:?}, sync_adc: {:?} }}",
                self.unlocked(),
                self.sdfpwmen(),
                self.sync_adc()
            )
        }
    }
    #[doc = "PWR port N pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrn(pub u32);
    impl Pucrn {
        #[doc = "Port N pull-up bit 1 When set, each bit activates the pull-up on PN1 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD1 bit is also set."]
        #[inline(always)]
        pub const fn pun1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port N pull-up bit 1 When set, each bit activates the pull-up on PN1 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD1 bit is also set."]
        #[inline(always)]
        pub fn set_pun1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port N pull-up bit 6 When set activates the pull-up on PN6 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PDN6 bit is also set."]
        #[inline(always)]
        pub const fn pun6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port N pull-up bit 6 When set activates the pull-up on PN6 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PDN6 bit is also set."]
        #[inline(always)]
        pub fn set_pun6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port N pull-up bit 12 When set, each bit activates the pull-up on PN12 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD12 bit is also set."]
        #[inline(always)]
        pub const fn pun12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port N pull-up bit 12 When set, each bit activates the pull-up on PN12 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD12 bit is also set."]
        #[inline(always)]
        pub fn set_pun12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Pucrn {
        #[inline(always)]
        fn default() -> Pucrn {
            Pucrn(0)
        }
    }
    impl core::fmt::Debug for Pucrn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucrn")
                .field("pun1", &self.pun1())
                .field("pun6", &self.pun6())
                .field("pun12", &self.pun12())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucrn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pucrn {{ pun1: {=bool:?}, pun6: {=bool:?}, pun12: {=bool:?} }}",
                self.pun1(),
                self.pun6(),
                self.pun12()
            )
        }
    }
    #[doc = "PWR port O pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucro(pub u32);
    impl Pucro {
        #[doc = "(n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set."]
        #[inline(always)]
        pub const fn puo0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "(n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set."]
        #[inline(always)]
        pub fn set_puo0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "(n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set."]
        #[inline(always)]
        pub const fn puo1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "(n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set."]
        #[inline(always)]
        pub fn set_puo1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port O pull-up bit 4 When set activates the pull-up on PO4 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits PDO4 in PWR_PDCRO is also set."]
        #[inline(always)]
        pub const fn puo4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port O pull-up bit 4 When set activates the pull-up on PO4 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits PDO4 in PWR_PDCRO is also set."]
        #[inline(always)]
        pub fn set_puo4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Pucro {
        #[inline(always)]
        fn default() -> Pucro {
            Pucro(0)
        }
    }
    impl core::fmt::Debug for Pucro {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucro")
                .field("puo0", &self.puo0())
                .field("puo1", &self.puo1())
                .field("puo4", &self.puo4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucro {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pucro {{ puo0: {=bool:?}, puo1: {=bool:?}, puo4: {=bool:?} }}",
                self.puo0(),
                self.puo1(),
                self.puo4()
            )
        }
    }
    #[doc = "PWR control status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "VOS currently applied for V<sub>CORE</sub> voltage scaling selection. These bit reflect the last VOS value applied to the PMU."]
        #[inline(always)]
        pub const fn actvos(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VOS currently applied for V<sub>CORE</sub> voltage scaling selection. These bit reflect the last VOS value applied to the PMU."]
        #[inline(always)]
        pub fn set_actvos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Voltage levels ready bit for currently used ACTVOS and SDHILEVEL This bit is set to 1 by hardware when the voltage regulator and the SMPS step-down converter are both disabled and Bypass mode is selected in PWR control register 2 (PWR_CSR2)."]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage levels ready bit for currently used ACTVOS and SDHILEVEL This bit is set to 1 by hardware when the voltage regulator and the SMPS step-down converter are both disabled and Bypass mode is selected in PWR control register 2 (PWR_CSR2)."]
        #[inline(always)]
        pub fn set_actvosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. PLS\\[2:0\\]
bits. bits. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
        #[inline(always)]
        pub const fn pvdo(&self) -> super::vals::Pvdo {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Pvdo::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. PLS\\[2:0\\]
bits. bits. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: super::vals::Pvdo) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
        #[inline(always)]
        pub const fn avdo(&self) -> super::vals::Avdo {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Avdo::from_bits(val as u8)
        }
        #[doc = "Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
        #[inline(always)]
        pub fn set_avdo(&mut self, val: super::vals::Avdo) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Sr1 {
        #[inline(always)]
        fn default() -> Sr1 {
            Sr1(0)
        }
    }
    impl core::fmt::Debug for Sr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1")
                .field("actvos", &self.actvos())
                .field("actvosrdy", &self.actvosrdy())
                .field("pvdo", &self.pvdo())
                .field("avdo", &self.avdo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr1 {{ actvos: {=bool:?}, actvosrdy: {=bool:?}, pvdo: {:?}, avdo: {:?} }}",
                self.actvos(),
                self.actvosrdy(),
                self.pvdo(),
                self.avdo()
            )
        }
    }
    #[doc = "PWR USB Type-C and Power Delivery register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucpdr(pub u32);
    impl Ucpdr {
        #[doc = "UCPD dead battery disable."]
        #[inline(always)]
        pub const fn ucpd_dbdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD dead battery disable."]
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
    #[doc = "PWR wakeup clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupcr(pub u32);
    impl Wkupcr {
        #[doc = "Clear Wakeup pin flag for WKUP1 These bits are always read as 0."]
        #[inline(always)]
        pub const fn wkupc(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear Wakeup pin flag for WKUP1 These bits are always read as 0."]
        #[inline(always)]
        pub fn set_wkupc(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            f.debug_struct("Wkupcr")
                .field("wkupc[0]", &self.wkupc(0usize))
                .field("wkupc[1]", &self.wkupc(1usize))
                .field("wkupc[2]", &self.wkupc(2usize))
                .field("wkupc[3]", &self.wkupc(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wkupcr {{ wkupc[0]: {=bool:?}, wkupc[1]: {=bool:?}, wkupc[2]: {=bool:?}, wkupc[3]: {=bool:?} }}",
                self.wkupc(0usize),
                self.wkupc(1usize),
                self.wkupc(2usize),
                self.wkupc(3usize)
            )
        }
    }
    #[doc = "PWR wakeup enable and polarity register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupepr(pub u32);
    impl Wkupepr {
        #[doc = "Enable Wakeup Pin WKUPn, (n = 4, 3, 2, 1) Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn bit) when WKUPn pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn selects falling edge."]
        #[inline(always)]
        pub const fn wkupen(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup Pin WKUPn, (n = 4, 3, 2, 1) Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn bit) when WKUPn pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn selects falling edge."]
        #[inline(always)]
        pub fn set_wkupen(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn, (n = 4, 3, 2, 1) These bits define the polarity used for event detection on WKUPn external wakeup pin."]
        #[inline(always)]
        pub const fn wkupp(&self, n: usize) -> super::vals::Wkupp {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wkupp::from_bits(val as u8)
        }
        #[doc = "Wakeup pin polarity bit for WKUPn, (n = 4, 3, 2, 1) These bits define the polarity used for event detection on WKUPn external wakeup pin."]
        #[inline(always)]
        pub fn set_wkupp(&mut self, n: usize, val: super::vals::Wkupp) {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup pin pull configuration"]
        #[inline(always)]
        pub const fn wkuppupd(&self, n: usize) -> super::vals::Wkuppupd {
            assert!(n < 4usize);
            let offs = 16usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Wkuppupd::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration"]
        #[inline(always)]
        pub fn set_wkuppupd(&mut self, n: usize, val: super::vals::Wkuppupd) {
            assert!(n < 4usize);
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
                .field("wkupp[0]", &self.wkupp(0usize))
                .field("wkupp[1]", &self.wkupp(1usize))
                .field("wkupp[2]", &self.wkupp(2usize))
                .field("wkupp[3]", &self.wkupp(3usize))
                .field("wkuppupd[0]", &self.wkuppupd(0usize))
                .field("wkuppupd[1]", &self.wkuppupd(1usize))
                .field("wkuppupd[2]", &self.wkuppupd(2usize))
                .field("wkuppupd[3]", &self.wkuppupd(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupepr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wkupepr {{ wkupen[0]: {=bool:?}, wkupen[1]: {=bool:?}, wkupen[2]: {=bool:?}, wkupen[3]: {=bool:?}, wkupp[0]: {:?}, wkupp[1]: {:?}, wkupp[2]: {:?}, wkupp[3]: {:?}, wkuppupd[0]: {:?}, wkuppupd[1]: {:?}, wkuppupd[2]: {:?}, wkuppupd[3]: {:?} }}" , self . wkupen (0usize) , self . wkupen (1usize) , self . wkupen (2usize) , self . wkupen (3usize) , self . wkupp (0usize) , self . wkupp (1usize) , self . wkupp (2usize) , self . wkupp (3usize) , self . wkuppupd (0usize) , self . wkuppupd (1usize) , self . wkuppupd (2usize) , self . wkuppupd (3usize))
        }
    }
    #[doc = "PWR wakeup flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupfr(pub u32);
    impl Wkupfr {
        #[doc = "Wakeup pin WKUP flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
        #[inline(always)]
        pub const fn wkupf(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
        #[inline(always)]
        pub fn set_wkupf(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wkupfr {{ wkupf[0]: {=bool:?}, wkupf[1]: {=bool:?}, wkupf[2]: {=bool:?}, wkupf[3]: {=bool:?} }}",
                self.wkupf(0usize),
                self.wkupf(1usize),
                self.wkupf(2usize),
                self.wkupf(3usize)
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Als {
        #[doc = "AVD level 1."]
        LEVEL1 = 0x0,
        #[doc = "AVD level 2."]
        LEVEL2 = 0x01,
        #[doc = "AVD level 3."]
        LEVEL3 = 0x02,
        #[doc = "AVD level 4."]
        LEVEL4 = 0x03,
    }
    impl Als {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Als {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Als {
        #[inline(always)]
        fn from(val: u8) -> Als {
            Als::from_bits(val)
        }
    }
    impl From<Als> for u8 {
        #[inline(always)]
        fn from(val: Als) -> u8 {
            Als::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Avdo {
        #[doc = "VDDA is equal or higher than the AVD threshold selected with the ALS\\[1:0\\]
bits."]
        ABOVE_OR_EQUAL = 0x0,
        #[doc = "VDDA is lower than the AVD threshold selected with the ALS\\[1:0\\]
bits."]
        BELOW = 0x01,
    }
    impl Avdo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Avdo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Avdo {
        #[inline(always)]
        fn from(val: u8) -> Avdo {
            Avdo::from_bits(val)
        }
    }
    impl From<Avdo> for u8 {
        #[inline(always)]
        fn from(val: Avdo) -> u8 {
            Avdo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pdds {
        #[doc = "Stop mode when device enters Deepsleep."]
        STOP = 0x0,
        #[doc = "Standby mode when device enters Deepsleep."]
        STANDBY = 0x01,
    }
    impl Pdds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pdds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pdds {
        #[inline(always)]
        fn from(val: u8) -> Pdds {
            Pdds::from_bits(val)
        }
    }
    impl From<Pdds> for u8 {
        #[inline(always)]
        fn from(val: Pdds) -> u8 {
            Pdds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pls {
        #[doc = "PVD level 1."]
        LEVEL1 = 0x0,
        #[doc = "PVD level 2."]
        LEVEL2 = 0x01,
        #[doc = "PVD level 3."]
        LEVEL3 = 0x02,
        #[doc = "PVD level 4."]
        LEVEL4 = 0x03,
        #[doc = "PVD level 5."]
        LEVEL5 = 0x04,
        #[doc = "PVD level 6."]
        LEVEL6 = 0x05,
        #[doc = "PVD level 7."]
        LEVEL7 = 0x06,
        #[doc = "External voltage level on PVD_IN pin, compared to internal VREFINT level."]
        EXTERNAL = 0x07,
    }
    impl Pls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pls {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pls {
        #[inline(always)]
        fn from(val: u8) -> Pls {
            Pls::from_bits(val)
        }
    }
    impl From<Pls> for u8 {
        #[inline(always)]
        fn from(val: Pls) -> u8 {
            Pls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pvdo {
        #[doc = "VDD or PVD_IN voltage is equal or higher than the PVD threshold selected through the."]
        ABOVE_OR_EQUAL = 0x0,
        #[doc = "VDD or PVD_IN voltage is lower than the PVD threshold selected through the PLS\\[2:0\\]."]
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
    pub enum Rlpsn {
        #[doc = "RAM enters to low power mode when system enters to STOP."]
        LOW_POWER = 0x0,
        #[doc = "RAM remains in normal mode when system enters to STOP."]
        NORMAL = 0x01,
    }
    impl Rlpsn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rlpsn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rlpsn {
        #[inline(always)]
        fn from(val: u8) -> Rlpsn {
            Rlpsn::from_bits(val)
        }
    }
    impl From<Rlpsn> for u8 {
        #[inline(always)]
        fn from(val: Rlpsn) -> u8 {
            Rlpsn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdlevel {
        RESET = 0x0,
        V1_8 = 0x01,
    }
    impl Sdlevel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdlevel {
            unsafe { core::mem::transmute(val & 0x01) }
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
    pub enum Svos {
        #[doc = "SVOS Low."]
        LOW = 0x0,
        #[doc = "SVOS High (default)."]
        HIGH = 0x01,
    }
    impl Svos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Svos {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Svos {
        #[inline(always)]
        fn from(val: u8) -> Svos {
            Svos::from_bits(val)
        }
    }
    impl From<Svos> for u8 {
        #[inline(always)]
        fn from(val: Svos) -> u8 {
            Svos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SyncAdc {
        #[doc = "SD_Converter clock free running."]
        FREE_RUNNING = 0x0,
        #[doc = "SD_Converter clock synchronised to ADC."]
        SYNCHRONIZED = 0x01,
    }
    impl SyncAdc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SyncAdc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SyncAdc {
        #[inline(always)]
        fn from(val: u8) -> SyncAdc {
            SyncAdc::from_bits(val)
        }
    }
    impl From<SyncAdc> for u8 {
        #[inline(always)]
        fn from(val: SyncAdc) -> u8 {
            SyncAdc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Unlocked {
        #[doc = "accessed locked: key was not written and after each register write access."]
        LOCKED = 0x0,
        #[doc = "after key 0xCAFECAFE was written in this register."]
        UNLOCKED = 0x01,
    }
    impl Unlocked {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Unlocked {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Unlocked {
        #[inline(always)]
        fn from(val: u8) -> Unlocked {
            Unlocked::from_bits(val)
        }
    }
    impl From<Unlocked> for u8 {
        #[inline(always)]
        fn from(val: Unlocked) -> u8 {
            Unlocked::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbrs {
        #[doc = "Charge VBAT through a 5 k resistor."]
        OHM5K = 0x0,
        #[doc = "Charge VBAT through a 1.5 k resistor."]
        OHM1_5K = 0x01,
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
        #[doc = "VOS Low level (default)."]
        LOW = 0x0,
        #[doc = "VOS High level."]
        HIGH = 0x01,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x01) }
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
    pub enum Wkupp {
        #[doc = "Detection on high level (rising edge)."]
        HIGH = 0x0,
        #[doc = "Detection on low level (falling edge)."]
        LOW = 0x01,
    }
    impl Wkupp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupp {
        #[inline(always)]
        fn from(val: u8) -> Wkupp {
            Wkupp::from_bits(val)
        }
    }
    impl From<Wkupp> for u8 {
        #[inline(always)]
        fn from(val: Wkupp) -> u8 {
            Wkupp::to_bits(val)
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Xspicap {
        #[doc = "XSPI Capacitor OFF (default) note: to confirm with analog design."]
        DISABLED = 0x0,
        #[doc = "XSPI Capacitor set to 1/3."]
        ONE_THIRD = 0x01,
        #[doc = "XSPI Capacitor set to 2/3."]
        TWO_THIRDS = 0x02,
        #[doc = "XSPI Capacitor set to full capacitance."]
        FULL = 0x03,
    }
    impl Xspicap {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Xspicap {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Xspicap {
        #[inline(always)]
        fn from(val: u8) -> Xspicap {
            Xspicap::from_bits(val)
        }
    }
    impl From<Xspicap> for u8 {
        #[inline(always)]
        fn from(val: Xspicap) -> u8 {
            Xspicap::to_bits(val)
        }
    }
}
