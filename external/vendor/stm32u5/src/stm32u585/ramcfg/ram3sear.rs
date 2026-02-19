///Register `RAM3SEAR` reader
pub type R = crate::R<RAM3SEARrs>;
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
        f.debug_struct("RAM3SEAR")
            .field("esea", &self.esea())
            .finish()
    }
}
/**RAMCFG RAM x ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`ram3sear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3SEAR)*/
pub struct RAM3SEARrs;
impl crate::RegisterSpec for RAM3SEARrs {
    type Ux = u32;
}
///`read()` method returns [`ram3sear::R`](R) reader structure
impl crate::Readable for RAM3SEARrs {}
///`reset()` method sets RAM3SEAR to value 0
impl crate::Resettable for RAM3SEARrs {}
