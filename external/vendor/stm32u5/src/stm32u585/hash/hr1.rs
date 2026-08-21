///Register `HR1` reader
pub type R = crate::R<HR1rs>;
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
        f.debug_struct("HR1").field("h1", &self.h1()).finish()
    }
}
/**digest register 1

You can [`read`](crate::Reg::read) this register and get [`hr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#HASH:HR1)*/
pub struct HR1rs;
impl crate::RegisterSpec for HR1rs {
    type Ux = u32;
}
///`read()` method returns [`hr1::R`](R) reader structure
impl crate::Readable for HR1rs {}
///`reset()` method sets HR1 to value 0
impl crate::Resettable for HR1rs {}
