///Register `LTR2` reader
pub type R = crate::R<LTR2rs>;
///Register `LTR2` writer
pub type W = crate::W<LTR2rs>;
///Field `LTR2` reader - LTR2
pub type LTR2_R = crate::FieldReader<u32>;
///Field `LTR2` writer - LTR2
pub type LTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32, crate::Safe>;
impl R {
    ///Bits 0:24 - LTR2
    #[inline(always)]
    pub fn ltr2(&self) -> LTR2_R {
        LTR2_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTR2").field("ltr2", &self.ltr2()).finish()
    }
}
impl W {
    ///Bits 0:24 - LTR2
    #[inline(always)]
    pub fn ltr2(&mut self) -> LTR2_W<LTR2rs> {
        LTR2_W::new(self, 0)
    }
}
/**ADC watchdog lower threshold register 2

You can [`read`](crate::Reg::read) this register and get [`ltr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADC1:LTR2)*/
pub struct LTR2rs;
impl crate::RegisterSpec for LTR2rs {
    type Ux = u32;
}
///`read()` method returns [`ltr2::R`](R) reader structure
impl crate::Readable for LTR2rs {}
///`write(|w| ..)` method takes [`ltr2::W`](W) writer structure
impl crate::Writable for LTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LTR2 to value 0
impl crate::Resettable for LTR2rs {}
