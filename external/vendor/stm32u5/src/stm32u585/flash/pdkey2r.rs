///Register `PDKEY2R` writer
pub type W = crate::W<PDKEY2Rrs>;
///Field `PDKEY2` writer - Bank 2 power-down key
pub type PDKEY2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<PDKEY2Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Bank 2 power-down key
    #[inline(always)]
    pub fn pdkey2(&mut self) -> PDKEY2_W<PDKEY2Rrs> {
        PDKEY2_W::new(self, 0)
    }
}
/**FLASH bank 2 power-down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkey2r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FLASH:PDKEY2R)*/
pub struct PDKEY2Rrs;
impl crate::RegisterSpec for PDKEY2Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pdkey2r::W`](W) writer structure
impl crate::Writable for PDKEY2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDKEY2R to value 0
impl crate::Resettable for PDKEY2Rrs {}
