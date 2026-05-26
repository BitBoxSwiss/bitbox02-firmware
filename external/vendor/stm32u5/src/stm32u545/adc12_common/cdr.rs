///Register `CDR` reader
pub type R = crate::R<CDRrs>;
///Field `RDATA_MST` reader - Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)) In DAMDF\[1:0\] = 11 mode, bits 15:8 contains SLV_ADC_DR\[7:0\], bits 7:0 contains MST_ADC_DR\[7:0\].
pub type RDATA_MST_R = crate::FieldReader<u16>;
///Field `RDATA_SLV` reader - Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT))
pub type RDATA_SLV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)) In DAMDF\[1:0\] = 11 mode, bits 15:8 contains SLV_ADC_DR\[7:0\], bits 7:0 contains MST_ADC_DR\[7:0\].
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT))
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR")
            .field("rdata_mst", &self.rdata_mst())
            .field("rdata_slv", &self.rdata_slv())
            .finish()
    }
}
/**ADC common regular data register for dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC12_Common:CDR)*/
pub struct CDRrs;
impl crate::RegisterSpec for CDRrs {
    type Ux = u32;
}
///`read()` method returns [`cdr::R`](R) reader structure
impl crate::Readable for CDRrs {}
///`reset()` method sets CDR to value 0
impl crate::Resettable for CDRrs {}
