///Register `COUNT1R` reader
pub type R = crate::R<COUNT1Rrs>;
///Field `COUNT` reader - COUNT
pub type COUNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - COUNT
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT1R")
            .field("count", &self.count())
            .finish()
    }
}
/**TAMP monotonic counter 1register

You can [`read`](crate::Reg::read) this register and get [`count1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#TAMP:COUNT1R)*/
pub struct COUNT1Rrs;
impl crate::RegisterSpec for COUNT1Rrs {
    type Ux = u32;
}
///`read()` method returns [`count1r::R`](R) reader structure
impl crate::Readable for COUNT1Rrs {}
///`reset()` method sets COUNT1R to value 0
impl crate::Resettable for COUNT1Rrs {}
