///Register `UCPDR` reader
pub type R = crate::R<UCPDRrs>;
///Register `UCPDR` writer
pub type W = crate::W<UCPDRrs>;
/**UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPD_DBDIS {
    ///0: UCPD dead battery pull-down behavior enabled on UCPDx_CC1 and UCPDx_CC2 pins
    Enabled = 0,
    ///1: UCPD dead battery pull-down behavior disabled on UCPDx_CC1 and UCPDx_CC2 pins
    Disabled = 1,
}
impl From<UCPD_DBDIS> for bool {
    #[inline(always)]
    fn from(variant: UCPD_DBDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `UCPD_DBDIS` reader - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).
pub type UCPD_DBDIS_R = crate::BitReader<UCPD_DBDIS>;
impl UCPD_DBDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UCPD_DBDIS {
        match self.bits {
            false => UCPD_DBDIS::Enabled,
            true => UCPD_DBDIS::Disabled,
        }
    }
    ///UCPD dead battery pull-down behavior enabled on UCPDx_CC1 and UCPDx_CC2 pins
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UCPD_DBDIS::Enabled
    }
    ///UCPD dead battery pull-down behavior disabled on UCPDx_CC1 and UCPDx_CC2 pins
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UCPD_DBDIS::Disabled
    }
}
///Field `UCPD_DBDIS` writer - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).
pub type UCPD_DBDIS_W<'a, REG> = crate::BitWriter<'a, REG, UCPD_DBDIS>;
impl<'a, REG> UCPD_DBDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///UCPD dead battery pull-down behavior enabled on UCPDx_CC1 and UCPDx_CC2 pins
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UCPD_DBDIS::Enabled)
    }
    ///UCPD dead battery pull-down behavior disabled on UCPDx_CC1 and UCPDx_CC2 pins
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UCPD_DBDIS::Disabled)
    }
}
/**UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPD_STBY {
    ///0: UCPD configuration is not memorized in Standby mode (Must be in this state after exiting Stop 3 or Standby mode, and before writing any UCPD registers)
    Disabled = 0,
    ///1: UCPD configuration is memorized in Stop 3 and Standby modes
    Enabled = 1,
}
impl From<UCPD_STBY> for bool {
    #[inline(always)]
    fn from(variant: UCPD_STBY) -> Self {
        variant as u8 != 0
    }
}
///Field `UCPD_STBY` reader - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
pub type UCPD_STBY_R = crate::BitReader<UCPD_STBY>;
impl UCPD_STBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UCPD_STBY {
        match self.bits {
            false => UCPD_STBY::Disabled,
            true => UCPD_STBY::Enabled,
        }
    }
    ///UCPD configuration is not memorized in Standby mode (Must be in this state after exiting Stop 3 or Standby mode, and before writing any UCPD registers)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UCPD_STBY::Disabled
    }
    ///UCPD configuration is memorized in Stop 3 and Standby modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UCPD_STBY::Enabled
    }
}
///Field `UCPD_STBY` writer - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
pub type UCPD_STBY_W<'a, REG> = crate::BitWriter<'a, REG, UCPD_STBY>;
impl<'a, REG> UCPD_STBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///UCPD configuration is not memorized in Standby mode (Must be in this state after exiting Stop 3 or Standby mode, and before writing any UCPD registers)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UCPD_STBY::Disabled)
    }
    ///UCPD configuration is memorized in Stop 3 and Standby modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UCPD_STBY::Enabled)
    }
}
impl R {
    ///Bit 0 - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).
    #[inline(always)]
    pub fn ucpd_dbdis(&self) -> UCPD_DBDIS_R {
        UCPD_DBDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
    #[inline(always)]
    pub fn ucpd_stby(&self) -> UCPD_STBY_R {
        UCPD_STBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UCPDR")
            .field("ucpd_dbdis", &self.ucpd_dbdis())
            .field("ucpd_stby", &self.ucpd_stby())
            .finish()
    }
}
impl W {
    ///Bit 0 - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).
    #[inline(always)]
    pub fn ucpd_dbdis(&mut self) -> UCPD_DBDIS_W<UCPDRrs> {
        UCPD_DBDIS_W::new(self, 0)
    }
    ///Bit 1 - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
    #[inline(always)]
    pub fn ucpd_stby(&mut self) -> UCPD_STBY_W<UCPDRrs> {
        UCPD_STBY_W::new(self, 1)
    }
}
/**PWR USB Type-C™ and Power Delivery register

You can [`read`](crate::Reg::read) this register and get [`ucpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#PWR:UCPDR)*/
pub struct UCPDRrs;
impl crate::RegisterSpec for UCPDRrs {
    type Ux = u32;
}
///`read()` method returns [`ucpdr::R`](R) reader structure
impl crate::Readable for UCPDRrs {}
///`write(|w| ..)` method takes [`ucpdr::W`](W) writer structure
impl crate::Writable for UCPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UCPDR to value 0
impl crate::Resettable for UCPDRrs {}
