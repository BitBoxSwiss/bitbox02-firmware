///Register `KEYR3` writer
pub type W = crate::W<KEYR3rs>;
///Field `SAES_KEYR3` writer - Cryptographic key, bits \[127:96\]
pub type SAES_KEYR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[127:96\]
    #[inline(always)]
    pub fn saes_keyr3(&mut self) -> SAES_KEYR3_W<KEYR3rs> {
        SAES_KEYR3_W::new(self, 0)
    }
}
/**key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR3)*/
pub struct KEYR3rs;
impl crate::RegisterSpec for KEYR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr3::W`](W) writer structure
impl crate::Writable for KEYR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR3 to value 0
impl crate::Resettable for KEYR3rs {}
