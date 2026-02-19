///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS {
    ///0: Stop 0 mode
    Stop0 = 0,
    ///1: Stop 1 mode
    Stop1 = 1,
    ///2: Stop 2 mode
    Stop2 = 2,
    ///3: Stop 3 mode
    Stop3 = 3,
    ///4: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1)
    Standby = 4,
    ///6: Shutdown mode if BREN = 0 in PWR_BDCR1
    Shutdown = 6,
}
impl From<LPMS> for u8 {
    #[inline(always)]
    fn from(variant: LPMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPMS {
    type Ux = u8;
}
impl crate::IsEnum for LPMS {}
///Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1
pub type LPMS_R = crate::FieldReader<LPMS>;
impl LPMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPMS> {
        match self.bits {
            0 => Some(LPMS::Stop0),
            1 => Some(LPMS::Stop1),
            2 => Some(LPMS::Stop2),
            3 => Some(LPMS::Stop3),
            4 => Some(LPMS::Standby),
            6 => Some(LPMS::Shutdown),
            _ => None,
        }
    }
    ///Stop 0 mode
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        *self == LPMS::Stop0
    }
    ///Stop 1 mode
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == LPMS::Stop1
    }
    ///Stop 2 mode
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == LPMS::Stop2
    }
    ///Stop 3 mode
    #[inline(always)]
    pub fn is_stop3(&self) -> bool {
        *self == LPMS::Stop3
    }
    ///Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1)
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMS::Standby
    }
    ///Shutdown mode if BREN = 0 in PWR_BDCR1
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == LPMS::Shutdown
    }
}
///Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Stop 0 mode
    #[inline(always)]
    pub fn stop0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop0)
    }
    ///Stop 1 mode
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop1)
    }
    ///Stop 2 mode
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop2)
    }
    ///Stop 3 mode
    #[inline(always)]
    pub fn stop3(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop3)
    }
    ///Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1)
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Standby)
    }
    ///Shutdown mode if BREN = 0 in PWR_BDCR1
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Shutdown)
    }
}
/**SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRSB1 {
    ///0: SRAM2 page1 content not retained in Stop3 and Standby modes
    Disabled = 0,
    ///1: SRAM2 page1 content retained in Stop 3 and Standby modes
    Enabled = 1,
}
impl From<RRSB1> for bool {
    #[inline(always)]
    fn from(variant: RRSB1) -> Self {
        variant as u8 != 0
    }
}
///Field `RRSB1` reader - SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.
pub type RRSB1_R = crate::BitReader<RRSB1>;
impl RRSB1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRSB1 {
        match self.bits {
            false => RRSB1::Disabled,
            true => RRSB1::Enabled,
        }
    }
    ///SRAM2 page1 content not retained in Stop3 and Standby modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRSB1::Disabled
    }
    ///SRAM2 page1 content retained in Stop 3 and Standby modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRSB1::Enabled
    }
}
///Field `RRSB1` writer - SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.
pub type RRSB1_W<'a, REG> = crate::BitWriter<'a, REG, RRSB1>;
impl<'a, REG> RRSB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 page1 content not retained in Stop3 and Standby modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRSB1::Disabled)
    }
    ///SRAM2 page1 content retained in Stop 3 and Standby modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRSB1::Enabled)
    }
}
/**SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRSB2 {
    ///0: SRAM2 page2 content not retained in Stop3 and Standby modes
    Disabled = 0,
    ///1: SRAM2 page2 content retained in Stop 3 and Standby modes
    Enabled = 1,
}
impl From<RRSB2> for bool {
    #[inline(always)]
    fn from(variant: RRSB2) -> Self {
        variant as u8 != 0
    }
}
///Field `RRSB2` reader - SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.
pub type RRSB2_R = crate::BitReader<RRSB2>;
impl RRSB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRSB2 {
        match self.bits {
            false => RRSB2::Disabled,
            true => RRSB2::Enabled,
        }
    }
    ///SRAM2 page2 content not retained in Stop3 and Standby modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRSB2::Disabled
    }
    ///SRAM2 page2 content retained in Stop 3 and Standby modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRSB2::Enabled
    }
}
///Field `RRSB2` writer - SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.
pub type RRSB2_W<'a, REG> = crate::BitWriter<'a, REG, RRSB2>;
impl<'a, REG> RRSB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 page2 content not retained in Stop3 and Standby modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRSB2::Disabled)
    }
    ///SRAM2 page2 content retained in Stop 3 and Standby modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRSB2::Enabled)
    }
}
/**BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPMEN {
    ///0: BOR level 0 operating in continuous (normal) mode in Standby mode
    Disabled = 0,
    ///1: BOR level 0 operating in discontinuous (ultra-low power) mode in Standby mode
    Enabled = 1,
}
impl From<ULPMEN> for bool {
    #[inline(always)]
    fn from(variant: ULPMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ULPMEN` reader - BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.
pub type ULPMEN_R = crate::BitReader<ULPMEN>;
impl ULPMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ULPMEN {
        match self.bits {
            false => ULPMEN::Disabled,
            true => ULPMEN::Enabled,
        }
    }
    ///BOR level 0 operating in continuous (normal) mode in Standby mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ULPMEN::Disabled
    }
    ///BOR level 0 operating in discontinuous (ultra-low power) mode in Standby mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ULPMEN::Enabled
    }
}
///Field `ULPMEN` writer - BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.
pub type ULPMEN_W<'a, REG> = crate::BitWriter<'a, REG, ULPMEN>;
impl<'a, REG> ULPMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BOR level 0 operating in continuous (normal) mode in Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ULPMEN::Disabled)
    }
    ///BOR level 0 operating in discontinuous (ultra-low power) mode in Standby mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ULPMEN::Enabled)
    }
}
/**SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1PD {
    ///0: SRAMx powered on
    On = 0,
    ///1: SRAMx powered off
    Off = 1,
}
impl From<SRAM1PD> for bool {
    #[inline(always)]
    fn from(variant: SRAM1PD) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM1PD` reader - SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.
pub type SRAM1PD_R = crate::BitReader<SRAM1PD>;
impl SRAM1PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1PD {
        match self.bits {
            false => SRAM1PD::On,
            true => SRAM1PD::Off,
        }
    }
    ///SRAMx powered on
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == SRAM1PD::On
    }
    ///SRAMx powered off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SRAM1PD::Off
    }
}
///Field `SRAM1PD` writer - SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.
pub type SRAM1PD_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1PD>;
impl<'a, REG> SRAM1PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAMx powered on
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1PD::On)
    }
    ///SRAMx powered off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1PD::Off)
    }
}
///Field `SRAM2PD` reader - SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2.
pub use SRAM1PD_R as SRAM2PD_R;
///Field `SRAM3PD` reader - SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3.
pub use SRAM1PD_R as SRAM3PD_R;
///Field `SRAM4PD` reader - SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4.
pub use SRAM1PD_R as SRAM4PD_R;
///Field `SRAM5PD` reader - SRAM5 power down This bit is used to reduce the consumption by powering off the SRAM5. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
pub use SRAM1PD_R as SRAM5PD_R;
///Field `SRAM2PD` writer - SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2.
pub use SRAM1PD_W as SRAM2PD_W;
///Field `SRAM3PD` writer - SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3.
pub use SRAM1PD_W as SRAM3PD_W;
///Field `SRAM4PD` writer - SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4.
pub use SRAM1PD_W as SRAM4PD_W;
///Field `SRAM5PD` writer - SRAM5 power down This bit is used to reduce the consumption by powering off the SRAM5. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
pub use SRAM1PD_W as SRAM5PD_W;
/**

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_USBPWR {
    ///0: OTG_HS PHY power is not maintained during low-power modes
    Disabled = 0,
    ///1: OTG_HS PHY power is maintained during low-power modes
    Enabled = 1,
}
impl From<FORCE_USBPWR> for bool {
    #[inline(always)]
    fn from(variant: FORCE_USBPWR) -> Self {
        variant as u8 != 0
    }
}
///Field `FORCE_USBPWR` reader -
pub type FORCE_USBPWR_R = crate::BitReader<FORCE_USBPWR>;
impl FORCE_USBPWR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_USBPWR {
        match self.bits {
            false => FORCE_USBPWR::Disabled,
            true => FORCE_USBPWR::Enabled,
        }
    }
    ///OTG_HS PHY power is not maintained during low-power modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FORCE_USBPWR::Disabled
    }
    ///OTG_HS PHY power is maintained during low-power modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FORCE_USBPWR::Enabled
    }
}
///Field `FORCE_USBPWR` writer -
pub type FORCE_USBPWR_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_USBPWR>;
impl<'a, REG> FORCE_USBPWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OTG_HS PHY power is not maintained during low-power modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_USBPWR::Disabled)
    }
    ///OTG_HS PHY power is maintained during low-power modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_USBPWR::Enabled)
    }
}
impl R {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 5 - SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.
    #[inline(always)]
    pub fn rrsb1(&self) -> RRSB1_R {
        RRSB1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.
    #[inline(always)]
    pub fn rrsb2(&self) -> RRSB2_R {
        RRSB2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.
    #[inline(always)]
    pub fn ulpmen(&self) -> ULPMEN_R {
        ULPMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.
    #[inline(always)]
    pub fn sram1pd(&self) -> SRAM1PD_R {
        SRAM1PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2.
    #[inline(always)]
    pub fn sram2pd(&self) -> SRAM2PD_R {
        SRAM2PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3.
    #[inline(always)]
    pub fn sram3pd(&self) -> SRAM3PD_R {
        SRAM3PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4.
    #[inline(always)]
    pub fn sram4pd(&self) -> SRAM4PD_R {
        SRAM4PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM5 power down This bit is used to reduce the consumption by powering off the SRAM5. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
    #[inline(always)]
    pub fn sram5pd(&self) -> SRAM5PD_R {
        SRAM5PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn force_usbpwr(&self) -> FORCE_USBPWR_R {
        FORCE_USBPWR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpms", &self.lpms())
            .field("rrsb1", &self.rrsb1())
            .field("rrsb2", &self.rrsb2())
            .field("ulpmen", &self.ulpmen())
            .field("sram1pd", &self.sram1pd())
            .field("sram2pd", &self.sram2pd())
            .field("sram3pd", &self.sram3pd())
            .field("sram4pd", &self.sram4pd())
            .field("sram5pd", &self.sram5pd())
            .field("force_usbpwr", &self.force_usbpwr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 5 - SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.
    #[inline(always)]
    pub fn rrsb1(&mut self) -> RRSB1_W<CR1rs> {
        RRSB1_W::new(self, 5)
    }
    ///Bit 6 - SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.
    #[inline(always)]
    pub fn rrsb2(&mut self) -> RRSB2_W<CR1rs> {
        RRSB2_W::new(self, 6)
    }
    ///Bit 7 - BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.
    #[inline(always)]
    pub fn ulpmen(&mut self) -> ULPMEN_W<CR1rs> {
        ULPMEN_W::new(self, 7)
    }
    ///Bit 8 - SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.
    #[inline(always)]
    pub fn sram1pd(&mut self) -> SRAM1PD_W<CR1rs> {
        SRAM1PD_W::new(self, 8)
    }
    ///Bit 9 - SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2.
    #[inline(always)]
    pub fn sram2pd(&mut self) -> SRAM2PD_W<CR1rs> {
        SRAM2PD_W::new(self, 9)
    }
    ///Bit 10 - SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3.
    #[inline(always)]
    pub fn sram3pd(&mut self) -> SRAM3PD_W<CR1rs> {
        SRAM3PD_W::new(self, 10)
    }
    ///Bit 11 - SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4.
    #[inline(always)]
    pub fn sram4pd(&mut self) -> SRAM4PD_W<CR1rs> {
        SRAM4PD_W::new(self, 11)
    }
    ///Bit 12 - SRAM5 power down This bit is used to reduce the consumption by powering off the SRAM5. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
    #[inline(always)]
    pub fn sram5pd(&mut self) -> SRAM5PD_W<CR1rs> {
        SRAM5PD_W::new(self, 12)
    }
    ///Bit 15
    #[inline(always)]
    pub fn force_usbpwr(&mut self) -> FORCE_USBPWR_W<CR1rs> {
        FORCE_USBPWR_W::new(self, 15)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
