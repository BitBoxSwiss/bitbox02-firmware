///Register `PDKEY1R` writer
pub type W = crate::W<PDKEY1Rrs>;
///Field `PDKEY1` writer - Bank 1 power-down key The following values must be written consecutively to unlock the PDREQ1 bit in FLASH_ACR: PDKEY1_1: 0x0415 2637 PDKEY1_2: 0xFAFB FCFD
pub type PDKEY1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<PDKEY1Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Bank 1 power-down key The following values must be written consecutively to unlock the PDREQ1 bit in FLASH_ACR: PDKEY1_1: 0x0415 2637 PDKEY1_2: 0xFAFB FCFD
    #[inline(always)]
    pub fn pdkey1(&mut self) -> PDKEY1_W<PDKEY1Rrs> {
        PDKEY1_W::new(self, 0)
    }
}
/**FLASH bank 1 power-down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkey1r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#FLASH:PDKEY1R)*/
pub struct PDKEY1Rrs;
impl crate::RegisterSpec for PDKEY1Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pdkey1r::W`](W) writer structure
impl crate::Writable for PDKEY1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDKEY1R to value 0
impl crate::Resettable for PDKEY1Rrs {}
