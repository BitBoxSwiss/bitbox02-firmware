///Register `RAM2ECCKEYR` writer
pub type W = crate::W<RAM2ECCKEYRrs>;
///Field `ECCKEY` writer - ECCKEY
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<RAM2ECCKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - ECCKEY
    #[inline(always)]
    pub fn ecckey(&mut self) -> ECCKEY_W<RAM2ECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
/**RAMCFG SRAM x ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2ecckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM2ECCKEYR)*/
pub struct RAM2ECCKEYRrs;
impl crate::RegisterSpec for RAM2ECCKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ram2ecckeyr::W`](W) writer structure
impl crate::Writable for RAM2ECCKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RAM2ECCKEYR to value 0
impl crate::Resettable for RAM2ECCKEYRrs {}
