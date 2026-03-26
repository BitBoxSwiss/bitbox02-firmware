///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
///Field `HSE32EN` reader - HSE Divided by 32 enable
pub type HSE32EN_R = crate::BitReader;
///Field `HSE32EN` writer - HSE Divided by 32 enable
pub type HSE32EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HSE Divided by 32 enable
    #[inline(always)]
    pub fn hse32en(&self) -> HSE32EN_R {
        HSE32EN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("hse32en", &self.hse32en())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSE Divided by 32 enable
    #[inline(always)]
    pub fn hse32en(&mut self) -> HSE32EN_W<OR1rs> {
        HSE32EN_W::new(self, 0)
    }
}
/**option register 1

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TIM16:OR1)*/
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
///`read()` method returns [`or1::R`](R) reader structure
impl crate::Readable for OR1rs {}
///`write(|w| ..)` method takes [`or1::W`](W) writer structure
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1rs {}
