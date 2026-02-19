///Register `LTR1` reader
pub type R = crate::R<LTR1rs>;
///Register `LTR1` writer
pub type W = crate::W<LTR1rs>;
///Field `LTR1` reader - LTR1
pub type LTR1_R = crate::FieldReader<u32>;
///Field `LTR1` writer - LTR1
pub type LTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32, crate::Safe>;
impl R {
    ///Bits 0:24 - LTR1
    #[inline(always)]
    pub fn ltr1(&self) -> LTR1_R {
        LTR1_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTR1").field("ltr1", &self.ltr1()).finish()
    }
}
impl W {
    ///Bits 0:24 - LTR1
    #[inline(always)]
    pub fn ltr1(&mut self) -> LTR1_W<LTR1rs> {
        LTR1_W::new(self, 0)
    }
}
/**ADC watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`ltr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC1:LTR1)*/
pub struct LTR1rs;
impl crate::RegisterSpec for LTR1rs {
    type Ux = u32;
}
///`read()` method returns [`ltr1::R`](R) reader structure
impl crate::Readable for LTR1rs {}
///`write(|w| ..)` method takes [`ltr1::W`](W) writer structure
impl crate::Writable for LTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LTR1 to value 0
impl crate::Resettable for LTR1rs {}
