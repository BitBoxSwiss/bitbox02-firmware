///Register `KEYR4` writer
pub type W = crate::W<KEYR4rs>;
///Field `KEY` writer - Cryptographic key, bits \[159:128\]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<KEYR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[159:128\]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<KEYR4rs> {
        KEY_W::new(self, 0)
    }
}
/**key register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#AES:KEYR4)*/
pub struct KEYR4rs;
impl crate::RegisterSpec for KEYR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr4::W`](W) writer structure
impl crate::Writable for KEYR4rs {
    type Safety = crate::Safe;
}
///`reset()` method sets KEYR4 to value 0
impl crate::Resettable for KEYR4rs {}
