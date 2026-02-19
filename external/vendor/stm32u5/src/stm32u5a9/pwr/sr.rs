///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC = 1 in PWR_SECCFGR. This bit is protected against unprivileged access when LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1. Writing 1 to this bit clears the STOPF and SBF flags.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSFW {
    ///1: Clear the STOPF and SBF flags
    Clear = 1,
}
impl From<CSSFW> for bool {
    #[inline(always)]
    fn from(variant: CSSFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSF` writer - Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC = 1 in PWR_SECCFGR. This bit is protected against unprivileged access when LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1. Writing 1 to this bit clears the STOPF and SBF flags.
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG, CSSFW>;
impl<'a, REG> CSSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the STOPF and SBF flags
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSSFW::Clear)
    }
}
/**Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPFR {
    ///0: The device did not enter any Stop mode
    NoStop = 0,
    ///1: The device entered a Stop mode
    Stop = 1,
}
impl From<STOPFR> for bool {
    #[inline(always)]
    fn from(variant: STOPFR) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPF` reader - Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit.
pub type STOPF_R = crate::BitReader<STOPFR>;
impl STOPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOPFR {
        match self.bits {
            false => STOPFR::NoStop,
            true => STOPFR::Stop,
        }
    }
    ///The device did not enter any Stop mode
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPFR::NoStop
    }
    ///The device entered a Stop mode
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPFR::Stop
    }
}
/**Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBFR {
    ///0: The device did not enter Standby mode
    NoStandby = 0,
    ///1: The device entered Standby mode
    Standby = 1,
}
impl From<SBFR> for bool {
    #[inline(always)]
    fn from(variant: SBFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SBF` reader - Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.
pub type SBF_R = crate::BitReader<SBFR>;
impl SBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBFR {
        match self.bits {
            false => SBFR::NoStandby,
            true => SBFR::Standby,
        }
    }
    ///The device did not enter Standby mode
    #[inline(always)]
    pub fn is_no_standby(&self) -> bool {
        *self == SBFR::NoStandby
    }
    ///The device entered Standby mode
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == SBFR::Standby
    }
}
impl R {
    ///Bit 1 - Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC = 1 in PWR_SECCFGR. This bit is protected against unprivileged access when LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1. Writing 1 to this bit clears the STOPF and SBF flags.
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<SRrs> {
        CSSF_W::new(self, 0)
    }
}
/**PWR status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#PWR:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
