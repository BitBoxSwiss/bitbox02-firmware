///Register `ATSEEDR` writer
pub type W = crate::W<ATSEEDRrs>;
///Field `SEED` writer - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG.
pub type SEED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<ATSEEDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG.
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W<ATSEEDRrs> {
        SEED_W::new(self, 0)
    }
}
/**TAMP active tamper seed register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atseedr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:ATSEEDR)*/
pub struct ATSEEDRrs;
impl crate::RegisterSpec for ATSEEDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`atseedr::W`](W) writer structure
impl crate::Writable for ATSEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ATSEEDR to value 0
impl crate::Resettable for ATSEEDRrs {}
