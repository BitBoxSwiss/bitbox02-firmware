///Register `RDATA` reader
pub type R = crate::R<RDATArs>;
///Field `RDATA` reader - Read data
pub type RDATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Read data
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATA")
            .field("rdata", &self.rdata())
            .finish()
    }
}
/**FMAC Read Data register

You can [`read`](crate::Reg::read) this register and get [`rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FMAC:RDATA)*/
pub struct RDATArs;
impl crate::RegisterSpec for RDATArs {
    type Ux = u32;
}
///`read()` method returns [`rdata::R`](R) reader structure
impl crate::Readable for RDATArs {}
///`reset()` method sets RDATA to value 0
impl crate::Resettable for RDATArs {}
