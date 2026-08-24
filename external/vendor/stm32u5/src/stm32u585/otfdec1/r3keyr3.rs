///Register `R3KEYR3` writer
pub type W = crate::W<R3KEYR3rs>;
///Field `REGx_KEY` writer - REGx_KEY
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R3KEYR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - REGx_KEY
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W<R3KEYR3rs> {
        REGX_KEY_W::new(self, 0)
    }
}
/**OTFDEC region x key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3keyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTFDEC1:R3KEYR3)*/
pub struct R3KEYR3rs;
impl crate::RegisterSpec for R3KEYR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r3keyr3::W`](W) writer structure
impl crate::Writable for R3KEYR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R3KEYR3 to value 0
impl crate::Resettable for R3KEYR3rs {}
