///Register `RAM3DEAR` reader
pub type R = crate::R<RAM3DEARrs>;
///Field `EDEA` reader - EDEA
pub type EDEA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - EDEA
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM3DEAR")
            .field("edea", &self.edea())
            .finish()
    }
}
/**RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`ram3dear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM3DEAR)*/
pub struct RAM3DEARrs;
impl crate::RegisterSpec for RAM3DEARrs {
    type Ux = u32;
}
///`read()` method returns [`ram3dear::R`](R) reader structure
impl crate::Readable for RAM3DEARrs {}
///`reset()` method sets RAM3DEAR to value 0
impl crate::Resettable for RAM3DEARrs {}
