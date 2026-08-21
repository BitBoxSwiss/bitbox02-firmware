///Register `PSC` reader
pub type R = crate::R<PSCrs>;
///Register `PSC` writer
pub type W = crate::W<PSCrs>;
///Field `PSC` reader - Prescaler value
pub type PSC_R = crate::FieldReader<u16>;
///Field `PSC` writer - Prescaler value
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSC").field("psc", &self.psc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Prescaler value
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**prescaler

You can [`read`](crate::Reg::read) this register and get [`psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#TIM2:PSC)*/
pub struct PSCrs;
impl crate::RegisterSpec for PSCrs {
    type Ux = u32;
}
///`read()` method returns [`psc::R`](R) reader structure
impl crate::Readable for PSCrs {}
///`write(|w| ..)` method takes [`psc::W`](W) writer structure
impl crate::Writable for PSCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSC to value 0
impl crate::Resettable for PSCrs {}
