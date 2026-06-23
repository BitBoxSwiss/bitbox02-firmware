///Register `M5DEAR` reader
pub type R = crate::R<M5DEARrs>;
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
        f.debug_struct("M5DEAR")
            .field("edea", &self.edea())
            .finish()
    }
}
/**RAMCFG RAM x ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`m5dear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RAMCFG:M5DEAR)*/
pub struct M5DEARrs;
impl crate::RegisterSpec for M5DEARrs {
    type Ux = u32;
}
///`read()` method returns [`m5dear::R`](R) reader structure
impl crate::Readable for M5DEARrs {}
///`reset()` method sets M5DEAR to value 0
impl crate::Resettable for M5DEARrs {}
