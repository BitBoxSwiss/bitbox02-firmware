///Register `KEYR1` writer
pub type W = crate::W<KEYR1rs>;
///Field `KEY` writer - Cryptographic key, bits \[63:32\]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<KEYR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[63:32\]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<KEYR1rs> {
        KEY_W::new(self, 0)
    }
}
/**key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#AES:KEYR1)*/
pub struct KEYR1rs;
impl crate::RegisterSpec for KEYR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr1::W`](W) writer structure
impl crate::Writable for KEYR1rs {
    type Safety = crate::Safe;
}
///`reset()` method sets KEYR1 to value 0
impl crate::Resettable for KEYR1rs {}
