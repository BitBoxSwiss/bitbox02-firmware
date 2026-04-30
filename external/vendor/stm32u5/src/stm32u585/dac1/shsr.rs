///Register `SHSR%s` reader
pub type R = crate::R<SHSRrs>;
///Register `SHSR%s` writer
pub type W = crate::W<SHSRrs>;
///Field `TSAMPLE` reader - DAC Channel 1 sample Time (only valid in sample &amp; hold mode)
pub type TSAMPLE_R = crate::FieldReader<u16>;
///Field `TSAMPLE` writer - DAC Channel 1 sample Time (only valid in sample &amp; hold mode)
pub type TSAMPLE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    ///Bits 0:9 - DAC Channel 1 sample Time (only valid in sample &amp; hold mode)
    #[inline(always)]
    pub fn tsample(&self) -> TSAMPLE_R {
        TSAMPLE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHSR")
            .field("tsample", &self.tsample())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - DAC Channel 1 sample Time (only valid in sample &amp; hold mode)
    #[inline(always)]
    pub fn tsample(&mut self) -> TSAMPLE_W<SHSRrs> {
        TSAMPLE_W::new(self, 0)
    }
}
/**DAC channel%s sample and hold sample time register

You can [`read`](crate::Reg::read) this register and get [`shsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DAC1:SHSR[1])*/
pub struct SHSRrs;
impl crate::RegisterSpec for SHSRrs {
    type Ux = u32;
}
///`read()` method returns [`shsr::R`](R) reader structure
impl crate::Readable for SHSRrs {}
///`write(|w| ..)` method takes [`shsr::W`](W) writer structure
impl crate::Writable for SHSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHSR%s to value 0
impl crate::Resettable for SHSRrs {}
