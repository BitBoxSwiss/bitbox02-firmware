///Register `BDCR1` reader
pub type R = crate::R<BDCR1rs>;
///Register `BDCR1` writer
pub type W = crate::W<BDCR1rs>;
/**Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREN {
    ///0: Backup RAM content lost in Standby and VBAT modes
    Disabled = 0,
    ///1: Backup RAM content preserved in Standby and VBAT modes
    Enabled = 1,
}
impl From<BREN> for bool {
    #[inline(always)]
    fn from(variant: BREN) -> Self {
        variant as u8 != 0
    }
}
///Field `BREN` reader - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
pub type BREN_R = crate::BitReader<BREN>;
impl BREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BREN {
        match self.bits {
            false => BREN::Disabled,
            true => BREN::Enabled,
        }
    }
    ///Backup RAM content lost in Standby and VBAT modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BREN::Disabled
    }
    ///Backup RAM content preserved in Standby and VBAT modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BREN::Enabled
    }
}
///Field `BREN` writer - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG, BREN>;
impl<'a, REG> BREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Backup RAM content lost in Standby and VBAT modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BREN::Disabled)
    }
    ///Backup RAM content preserved in Standby and VBAT modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BREN::Enabled)
    }
}
/**Backup domain voltage and temperature monitoring enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONEN {
    ///0: Backup domain voltage and temperature monitoring disabled
    Disabled = 0,
    ///1: Backup domain voltage and temperature monitoring enabled
    Enabled = 1,
}
impl From<MONEN> for bool {
    #[inline(always)]
    fn from(variant: MONEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MONEN` reader - Backup domain voltage and temperature monitoring enable
pub type MONEN_R = crate::BitReader<MONEN>;
impl MONEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MONEN {
        match self.bits {
            false => MONEN::Disabled,
            true => MONEN::Enabled,
        }
    }
    ///Backup domain voltage and temperature monitoring disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONEN::Disabled
    }
    ///Backup domain voltage and temperature monitoring enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONEN::Enabled
    }
}
///Field `MONEN` writer - Backup domain voltage and temperature monitoring enable
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG, MONEN>;
impl<'a, REG> MONEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Backup domain voltage and temperature monitoring disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MONEN::Disabled)
    }
    ///Backup domain voltage and temperature monitoring enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MONEN::Enabled)
    }
}
impl R {
    ///Bit 0 - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Backup domain voltage and temperature monitoring enable
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR1")
            .field("bren", &self.bren())
            .field("monen", &self.monen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W<BDCR1rs> {
        BREN_W::new(self, 0)
    }
    ///Bit 4 - Backup domain voltage and temperature monitoring enable
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W<BDCR1rs> {
        MONEN_W::new(self, 4)
    }
}
/**PWR Backup domain control register 1

You can [`read`](crate::Reg::read) this register and get [`bdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#PWR:BDCR1)*/
pub struct BDCR1rs;
impl crate::RegisterSpec for BDCR1rs {
    type Ux = u32;
}
///`read()` method returns [`bdcr1::R`](R) reader structure
impl crate::Readable for BDCR1rs {}
///`write(|w| ..)` method takes [`bdcr1::W`](W) writer structure
impl crate::Writable for BDCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR1 to value 0
impl crate::Resettable for BDCR1rs {}
