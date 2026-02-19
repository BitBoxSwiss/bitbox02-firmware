///Register `HRA1` reader
pub type R = crate::R<HRA1rs>;
///Field `H1` reader - H1
pub type H1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H1
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRA1").field("h1", &self.h1()).finish()
    }
}
/**HASH aliased digest register 1

You can [`read`](crate::Reg::read) this register and get [`hra1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#HASH:HRA1)*/
pub struct HRA1rs;
impl crate::RegisterSpec for HRA1rs {
    type Ux = u32;
}
///`read()` method returns [`hra1::R`](R) reader structure
impl crate::Readable for HRA1rs {}
///`reset()` method sets HRA1 to value 0
impl crate::Resettable for HRA1rs {}
