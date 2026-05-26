///Register `RAM3ERKEYR` writer
pub type W = crate::W<RAM3ERKEYRrs>;
///Field `ERASEKEY` writer - ERASEKEY
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<RAM3ERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - ERASEKEY
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<RAM3ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG SRAM x erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram3erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM3ERKEYR)*/
pub struct RAM3ERKEYRrs;
impl crate::RegisterSpec for RAM3ERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ram3erkeyr::W`](W) writer structure
impl crate::Writable for RAM3ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RAM3ERKEYR to value 0
impl crate::Resettable for RAM3ERKEYRrs {}
