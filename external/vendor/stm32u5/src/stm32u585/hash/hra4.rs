///Register `HRA4` reader
pub type R = crate::R<HRA4rs>;
///Field `H4` reader - H4
pub type H4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H4
    #[inline(always)]
    pub fn h4(&self) -> H4_R {
        H4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRA4").field("h4", &self.h4()).finish()
    }
}
/**HASH aliased digest register 4

You can [`read`](crate::Reg::read) this register and get [`hra4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:HRA4)*/
pub struct HRA4rs;
impl crate::RegisterSpec for HRA4rs {
    type Ux = u32;
}
///`read()` method returns [`hra4::R`](R) reader structure
impl crate::Readable for HRA4rs {}
///`reset()` method sets HRA4 to value 0
impl crate::Resettable for HRA4rs {}
