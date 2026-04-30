///Register `DCNTR` reader
pub type R = crate::R<DCNTRrs>;
///Field `DATACOUNT` reader - Data count value
pub type DATACOUNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:24 - Data count value
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCNTR")
            .field("datacount", &self.datacount())
            .finish()
    }
}
/**data counter register

You can [`read`](crate::Reg::read) this register and get [`dcntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SDMMC1:DCNTR)*/
pub struct DCNTRrs;
impl crate::RegisterSpec for DCNTRrs {
    type Ux = u32;
}
///`read()` method returns [`dcntr::R`](R) reader structure
impl crate::Readable for DCNTRrs {}
///`reset()` method sets DCNTR to value 0
impl crate::Resettable for DCNTRrs {}
