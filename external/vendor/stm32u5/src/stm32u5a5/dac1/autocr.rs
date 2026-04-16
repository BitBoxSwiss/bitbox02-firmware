///Register `AUTOCR` reader
pub type R = crate::R<AUTOCRrs>;
///Register `AUTOCR` writer
pub type W = crate::W<AUTOCRrs>;
/**DAC Autonomous mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOMODE {
    ///0: DAC Autonomous mode disabled
    Disabled = 0,
    ///1: DAC Autonomous mode enabled
    Enabled = 1,
}
impl From<AUTOMODE> for bool {
    #[inline(always)]
    fn from(variant: AUTOMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOMODE` reader - DAC Autonomous mode
pub type AUTOMODE_R = crate::BitReader<AUTOMODE>;
impl AUTOMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTOMODE {
        match self.bits {
            false => AUTOMODE::Disabled,
            true => AUTOMODE::Enabled,
        }
    }
    ///DAC Autonomous mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOMODE::Disabled
    }
    ///DAC Autonomous mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOMODE::Enabled
    }
}
///Field `AUTOMODE` writer - DAC Autonomous mode
pub type AUTOMODE_W<'a, REG> = crate::BitWriter<'a, REG, AUTOMODE>;
impl<'a, REG> AUTOMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC Autonomous mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOMODE::Disabled)
    }
    ///DAC Autonomous mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOMODE::Enabled)
    }
}
impl R {
    ///Bit 22 - DAC Autonomous mode
    #[inline(always)]
    pub fn automode(&self) -> AUTOMODE_R {
        AUTOMODE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCR")
            .field("automode", &self.automode())
            .finish()
    }
}
impl W {
    ///Bit 22 - DAC Autonomous mode
    #[inline(always)]
    pub fn automode(&mut self) -> AUTOMODE_W<AUTOCRrs> {
        AUTOMODE_W::new(self, 22)
    }
}
/**Autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`autocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DAC1:AUTOCR)*/
pub struct AUTOCRrs;
impl crate::RegisterSpec for AUTOCRrs {
    type Ux = u32;
}
///`read()` method returns [`autocr::R`](R) reader structure
impl crate::Readable for AUTOCRrs {}
///`write(|w| ..)` method takes [`autocr::W`](W) writer structure
impl crate::Writable for AUTOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AUTOCR to value 0
impl crate::Resettable for AUTOCRrs {}
