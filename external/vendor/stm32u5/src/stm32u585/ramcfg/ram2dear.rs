///Register `RAM2DEAR` reader
pub type R = crate::R<RAM2DEARrs>;
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
        f.debug_struct("RAM2DEAR")
            .field("edea", &self.edea())
            .finish()
    }
}
/**RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`ram2dear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM2DEAR)*/
pub struct RAM2DEARrs;
impl crate::RegisterSpec for RAM2DEARrs {
    type Ux = u32;
}
///`read()` method returns [`ram2dear::R`](R) reader structure
impl crate::Readable for RAM2DEARrs {}
///`reset()` method sets RAM2DEAR to value 0
impl crate::Resettable for RAM2DEARrs {}
