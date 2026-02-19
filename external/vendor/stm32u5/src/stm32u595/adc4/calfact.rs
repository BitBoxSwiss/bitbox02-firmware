///Register `CALFACT` reader
pub type R = crate::R<CALFACTrs>;
///Register `CALFACT` writer
pub type W = crate::W<CALFACTrs>;
///Field `CALFACT` reader - CALFACT
pub type CALFACT_R = crate::FieldReader;
///Field `CALFACT` writer - CALFACT
pub type CALFACT_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    ///Bits 0:6 - CALFACT
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFACT")
            .field("calfact", &self.calfact())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - CALFACT
    #[inline(always)]
    pub fn calfact(&mut self) -> CALFACT_W<CALFACTrs> {
        CALFACT_W::new(self, 0)
    }
}
/**ADC Calibration factor

You can [`read`](crate::Reg::read) this register and get [`calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADC4:CALFACT)*/
pub struct CALFACTrs;
impl crate::RegisterSpec for CALFACTrs {
    type Ux = u32;
}
///`read()` method returns [`calfact::R`](R) reader structure
impl crate::Readable for CALFACTrs {}
///`write(|w| ..)` method takes [`calfact::W`](W) writer structure
impl crate::Writable for CALFACTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALFACT to value 0
impl crate::Resettable for CALFACTrs {}
