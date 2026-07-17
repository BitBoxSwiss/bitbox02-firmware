///Register `HRA3` reader
pub type R = crate::R<HRA3rs>;
///Field `H3` reader - H3
pub type H3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H3
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRA3").field("h3", &self.h3()).finish()
    }
}
/**HASH aliased digest register 3

You can [`read`](crate::Reg::read) this register and get [`hra3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#HASH:HRA3)*/
pub struct HRA3rs;
impl crate::RegisterSpec for HRA3rs {
    type Ux = u32;
}
///`read()` method returns [`hra3::R`](R) reader structure
impl crate::Readable for HRA3rs {}
///`reset()` method sets HRA3 to value 0
impl crate::Resettable for HRA3rs {}
