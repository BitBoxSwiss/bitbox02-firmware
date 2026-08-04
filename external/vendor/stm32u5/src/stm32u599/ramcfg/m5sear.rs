///Register `M5SEAR` reader
pub type R = crate::R<M5SEARrs>;
///Field `ESEA` reader - ESEA
pub type ESEA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ESEA
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5SEAR")
            .field("esea", &self.esea())
            .finish()
    }
}
/**RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`m5sear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RAMCFG:M5SEAR)*/
pub struct M5SEARrs;
impl crate::RegisterSpec for M5SEARrs {
    type Ux = u32;
}
///`read()` method returns [`m5sear::R`](R) reader structure
impl crate::Readable for M5SEARrs {}
///`reset()` method sets M5SEAR to value 0
impl crate::Resettable for M5SEARrs {}
