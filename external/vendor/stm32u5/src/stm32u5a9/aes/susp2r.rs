///Register `SUSP2R` writer
pub type W = crate::W<SUSP2Rrs>;
///Field `SUSP` writer - AES suspend
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<SUSP2Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<SUSP2Rrs> {
        SUSP_W::new(self, 0)
    }
}
/**suspend registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp2r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#AES:SUSP2R)*/
pub struct SUSP2Rrs;
impl crate::RegisterSpec for SUSP2Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`susp2r::W`](W) writer structure
impl crate::Writable for SUSP2Rrs {
    type Safety = crate::Safe;
}
///`reset()` method sets SUSP2R to value 0
impl crate::Resettable for SUSP2Rrs {}
