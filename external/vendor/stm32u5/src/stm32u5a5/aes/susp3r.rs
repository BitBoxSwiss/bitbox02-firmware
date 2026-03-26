///Register `SUSP3R` writer
pub type W = crate::W<SUSP3Rrs>;
///Field `SUSP` writer - AES suspend
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<SUSP3Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<SUSP3Rrs> {
        SUSP_W::new(self, 0)
    }
}
/**suspend registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp3r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#AES:SUSP3R)*/
pub struct SUSP3Rrs;
impl crate::RegisterSpec for SUSP3Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`susp3r::W`](W) writer structure
impl crate::Writable for SUSP3Rrs {
    type Safety = crate::Safe;
}
///`reset()` method sets SUSP3R to value 0
impl crate::Resettable for SUSP3Rrs {}
