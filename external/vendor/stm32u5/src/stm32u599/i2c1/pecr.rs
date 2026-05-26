///Register `PECR` reader
pub type R = crate::R<PECRrs>;
///Field `PEC` reader - Packet error checking register
pub type PEC_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Packet error checking register
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECR").field("pec", &self.pec()).finish()
    }
}
/**PEC register

You can [`read`](crate::Reg::read) this register and get [`pecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#I2C1:PECR)*/
pub struct PECRrs;
impl crate::RegisterSpec for PECRrs {
    type Ux = u32;
}
///`read()` method returns [`pecr::R`](R) reader structure
impl crate::Readable for PECRrs {}
///`reset()` method sets PECR to value 0
impl crate::Resettable for PECRrs {}
