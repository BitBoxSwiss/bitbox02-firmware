///Register `CALFACT2` reader
pub type R = crate::R<CALFACT2rs>;
///Register `CALFACT2` writer
pub type W = crate::W<CALFACT2rs>;
///Field `CALFACT` reader - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\[31:0\], indexed by CALINDEX\[3:0\] bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\[31:0\], indexed by CALINDEX\[3:0\] bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_R = crate::FieldReader<u32>;
///Field `CALFACT` writer - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\[31:0\], indexed by CALINDEX\[3:0\] bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\[31:0\], indexed by CALINDEX\[3:0\] bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\[31:0\], indexed by CALINDEX\[3:0\] bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\[31:0\], indexed by CALINDEX\[3:0\] bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFACT2")
            .field("calfact", &self.calfact())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\[31:0\], indexed by CALINDEX\[3:0\] bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\[31:0\], indexed by CALINDEX\[3:0\] bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact(&mut self) -> CALFACT_W<CALFACT2rs> {
        CALFACT_W::new(self, 0)
    }
}
/**ADC calibration factor register

You can [`read`](crate::Reg::read) this register and get [`calfact2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC1:CALFACT2)*/
pub struct CALFACT2rs;
impl crate::RegisterSpec for CALFACT2rs {
    type Ux = u32;
}
///`read()` method returns [`calfact2::R`](R) reader structure
impl crate::Readable for CALFACT2rs {}
///`write(|w| ..)` method takes [`calfact2::W`](W) writer structure
impl crate::Writable for CALFACT2rs {
    type Safety = crate::Safe;
}
///`reset()` method sets CALFACT2 to value 0
impl crate::Resettable for CALFACT2rs {}
