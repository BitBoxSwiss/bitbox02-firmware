///Register `RAM5SEAR` reader
pub type R = crate::R<RAM5SEARrs>;
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
        f.debug_struct("RAM5SEAR")
            .field("esea", &self.esea())
            .finish()
    }
}
/**RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`ram5sear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM5SEAR)*/
pub struct RAM5SEARrs;
impl crate::RegisterSpec for RAM5SEARrs {
    type Ux = u32;
}
///`read()` method returns [`ram5sear::R`](R) reader structure
impl crate::Readable for RAM5SEARrs {}
///`reset()` method sets RAM5SEAR to value 0
impl crate::Resettable for RAM5SEARrs {}
