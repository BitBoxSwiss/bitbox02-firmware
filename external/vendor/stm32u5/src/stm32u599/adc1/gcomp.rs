///Register `GCOMP` reader
pub type R = crate::R<GCOMPrs>;
///Register `GCOMP` writer
pub type W = crate::W<GCOMPrs>;
///Field `GCOMPCOEFF` reader - GCOMPCOEFF
pub type GCOMPCOEFF_R = crate::FieldReader<u16>;
///Field `GCOMPCOEFF` writer - GCOMPCOEFF
pub type GCOMPCOEFF_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
/**GCOMP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCOMP {
    ///0: Regular ADC operating mode
    Disabled = 0,
    ///1: Gain compensation enabled and applied to all channels
    Enabled = 1,
}
impl From<GCOMP> for bool {
    #[inline(always)]
    fn from(variant: GCOMP) -> Self {
        variant as u8 != 0
    }
}
///Field `GCOMP` reader - GCOMP
pub type GCOMP_R = crate::BitReader<GCOMP>;
impl GCOMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GCOMP {
        match self.bits {
            false => GCOMP::Disabled,
            true => GCOMP::Enabled,
        }
    }
    ///Regular ADC operating mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCOMP::Disabled
    }
    ///Gain compensation enabled and applied to all channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCOMP::Enabled
    }
}
///Field `GCOMP` writer - GCOMP
pub type GCOMP_W<'a, REG> = crate::BitWriter<'a, REG, GCOMP>;
impl<'a, REG> GCOMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular ADC operating mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCOMP::Disabled)
    }
    ///Gain compensation enabled and applied to all channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCOMP::Enabled)
    }
}
impl R {
    ///Bits 0:13 - GCOMPCOEFF
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GCOMPCOEFF_R {
        GCOMPCOEFF_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 31 - GCOMP
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCOMP")
            .field("gcomp", &self.gcomp())
            .field("gcompcoeff", &self.gcompcoeff())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - GCOMPCOEFF
    #[inline(always)]
    pub fn gcompcoeff(&mut self) -> GCOMPCOEFF_W<GCOMPrs> {
        GCOMPCOEFF_W::new(self, 0)
    }
    ///Bit 31 - GCOMP
    #[inline(always)]
    pub fn gcomp(&mut self) -> GCOMP_W<GCOMPrs> {
        GCOMP_W::new(self, 31)
    }
}
/**ADC gain compensation register

You can [`read`](crate::Reg::read) this register and get [`gcomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:GCOMP)*/
pub struct GCOMPrs;
impl crate::RegisterSpec for GCOMPrs {
    type Ux = u32;
}
///`read()` method returns [`gcomp::R`](R) reader structure
impl crate::Readable for GCOMPrs {}
///`write(|w| ..)` method takes [`gcomp::W`](W) writer structure
impl crate::Writable for GCOMPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCOMP to value 0
impl crate::Resettable for GCOMPrs {}
