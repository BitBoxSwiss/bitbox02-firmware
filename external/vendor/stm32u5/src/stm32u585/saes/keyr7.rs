///Register `KEYR7` writer
pub type W = crate::W<KEYR7rs>;
///Field `KEY` writer - Cryptographic key, bits \[255:224\]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYR7rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[255:224\]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<KEYR7rs> {
        KEY_W::new(self, 0)
    }
}
/**key register 7

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAES:KEYR7)*/
pub struct KEYR7rs;
impl crate::RegisterSpec for KEYR7rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr7::W`](W) writer structure
impl crate::Writable for KEYR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR7 to value 0
impl crate::Resettable for KEYR7rs {}
