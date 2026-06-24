///Register `R4KEYR1` writer
pub type W = crate::W<R4KEYR1rs>;
///Field `REG4_KEY` writer - REG4_KEY
pub type REG4_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R4KEYR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - REG4_KEY
    #[inline(always)]
    pub fn reg4_key(&mut self) -> REG4_KEY_W<R4KEYR1rs> {
        REG4_KEY_W::new(self, 0)
    }
}
/**OTFDEC region x key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r4keyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OTFDEC1:R4KEYR1)*/
pub struct R4KEYR1rs;
impl crate::RegisterSpec for R4KEYR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r4keyr1::W`](W) writer structure
impl crate::Writable for R4KEYR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R4KEYR1 to value 0
impl crate::Resettable for R4KEYR1rs {}
