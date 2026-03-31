///Register `LTR3` reader
pub type R = crate::R<LTR3rs>;
///Register `LTR3` writer
pub type W = crate::W<LTR3rs>;
///Field `LTR3` reader - LTR3
pub type LTR3_R = crate::FieldReader<u32>;
///Field `LTR3` writer - LTR3
pub type LTR3_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32, crate::Safe>;
impl R {
    ///Bits 0:24 - LTR3
    #[inline(always)]
    pub fn ltr3(&self) -> LTR3_R {
        LTR3_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTR3").field("ltr3", &self.ltr3()).finish()
    }
}
impl W {
    ///Bits 0:24 - LTR3
    #[inline(always)]
    pub fn ltr3(&mut self) -> LTR3_W<LTR3rs> {
        LTR3_W::new(self, 0)
    }
}
/**ADC watchdog lower threshold register 3

You can [`read`](crate::Reg::read) this register and get [`ltr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC1:LTR3)*/
pub struct LTR3rs;
impl crate::RegisterSpec for LTR3rs {
    type Ux = u32;
}
///`read()` method returns [`ltr3::R`](R) reader structure
impl crate::Readable for LTR3rs {}
///`write(|w| ..)` method takes [`ltr3::W`](W) writer structure
impl crate::Writable for LTR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LTR3 to value 0
impl crate::Resettable for LTR3rs {}
