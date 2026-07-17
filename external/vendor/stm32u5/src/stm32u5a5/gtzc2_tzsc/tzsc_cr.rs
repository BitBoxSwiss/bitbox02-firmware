///Register `TZSC_CR` reader
pub type R = crate::R<TZSC_CRrs>;
///Register `TZSC_CR` writer
pub type W = crate::W<TZSC_CRrs>;
///Field `LCK` reader - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers until next reset
pub type LCK_R = crate::BitReader;
///Field `LCK` writer - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers until next reset
pub type LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers until next reset
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_CR").field("lck", &self.lck()).finish()
    }
}
impl W {
    ///Bit 0 - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers until next reset
    #[inline(always)]
    pub fn lck(&mut self) -> LCK_W<TZSC_CRrs> {
        LCK_W::new(self, 0)
    }
}
/**TZSC control register

You can [`read`](crate::Reg::read) this register and get [`tzsc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#GTZC2_TZSC:TZSC_CR)*/
pub struct TZSC_CRrs;
impl crate::RegisterSpec for TZSC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_cr::R`](R) reader structure
impl crate::Readable for TZSC_CRrs {}
///`write(|w| ..)` method takes [`tzsc_cr::W`](W) writer structure
impl crate::Writable for TZSC_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZSC_CR to value 0
impl crate::Resettable for TZSC_CRrs {}
