///Register `HRA2` reader
pub type R = crate::R<HRA2rs>;
///Field `H2` reader - H2
pub type H2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H2
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRA2").field("h2", &self.h2()).finish()
    }
}
/**HASH aliased digest register 2

You can [`read`](crate::Reg::read) this register and get [`hra2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HASH:HRA2)*/
pub struct HRA2rs;
impl crate::RegisterSpec for HRA2rs {
    type Ux = u32;
}
///`read()` method returns [`hra2::R`](R) reader structure
impl crate::Readable for HRA2rs {}
///`reset()` method sets HRA2 to value 0
impl crate::Resettable for HRA2rs {}
