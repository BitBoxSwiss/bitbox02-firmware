///Register `CDR2` reader
pub type R = crate::R<CDR2rs>;
///Field `RDATA_ALT` reader - Regular data of the master/slave alternated ADCs In dual mode, these bits alternatively contains the regular 32-bit data of the master and the slave ADC. Refer to . The data alignment is applied as described in (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT).
pub type RDATA_ALT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Regular data of the master/slave alternated ADCs In dual mode, these bits alternatively contains the regular 32-bit data of the master and the slave ADC. Refer to . The data alignment is applied as described in (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT).
    #[inline(always)]
    pub fn rdata_alt(&self) -> RDATA_ALT_R {
        RDATA_ALT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR2")
            .field("rdata_alt", &self.rdata_alt())
            .finish()
    }
}
/**ADC common regular data register for 32-bit dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC12_Common:CDR2)*/
pub struct CDR2rs;
impl crate::RegisterSpec for CDR2rs {
    type Ux = u32;
}
///`read()` method returns [`cdr2::R`](R) reader structure
impl crate::Readable for CDR2rs {}
///`reset()` method sets CDR2 to value 0
impl crate::Resettable for CDR2rs {}
