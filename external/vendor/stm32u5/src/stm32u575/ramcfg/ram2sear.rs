///Register `RAM2SEAR` reader
pub type R = crate::R<RAM2SEARrs>;
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
        f.debug_struct("RAM2SEAR")
            .field("esea", &self.esea())
            .finish()
    }
}
/**RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`ram2sear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM2SEAR)*/
pub struct RAM2SEARrs;
impl crate::RegisterSpec for RAM2SEARrs {
    type Ux = u32;
}
///`read()` method returns [`ram2sear::R`](R) reader structure
impl crate::Readable for RAM2SEARrs {}
///`reset()` method sets RAM2SEAR to value 0
impl crate::Resettable for RAM2SEARrs {}
