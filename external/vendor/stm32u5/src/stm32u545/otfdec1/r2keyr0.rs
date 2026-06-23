///Register `R2KEYR0` writer
pub type W = crate::W<R2KEYR0rs>;
///Field `REG2_KEY` writer - REG2_KEY
pub type REG2_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R2KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - REG2_KEY
    #[inline(always)]
    pub fn reg2_key(&mut self) -> REG2_KEY_W<R2KEYR0rs> {
        REG2_KEY_W::new(self, 0)
    }
}
/**OTFDEC region x key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OTFDEC1:R2KEYR0)*/
pub struct R2KEYR0rs;
impl crate::RegisterSpec for R2KEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r2keyr0::W`](W) writer structure
impl crate::Writable for R2KEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R2KEYR0 to value 0
impl crate::Resettable for R2KEYR0rs {}
