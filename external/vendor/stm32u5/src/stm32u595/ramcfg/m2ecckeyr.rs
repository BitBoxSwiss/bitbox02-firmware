///Register `M2ECCKEYR` writer
pub type W = crate::W<M2ECCKEYRrs>;
///Field `ECCKEY` writer - ECCKEY
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<M2ECCKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - ECCKEY
    #[inline(always)]
    pub fn ecckey(&mut self) -> ECCKEY_W<M2ECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
/**RAMCFG SRAM x ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ecckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RAMCFG:M2ECCKEYR)*/
pub struct M2ECCKEYRrs;
impl crate::RegisterSpec for M2ECCKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`m2ecckeyr::W`](W) writer structure
impl crate::Writable for M2ECCKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M2ECCKEYR to value 0
impl crate::Resettable for M2ECCKEYRrs {}
