///Register `HR7` reader
pub type R = crate::R<HR7rs>;
///Field `H7` reader - H7
pub type H7_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H7
    #[inline(always)]
    pub fn h7(&self) -> H7_R {
        H7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR7").field("h7", &self.h7()).finish()
    }
}
/**supplementary digest register 7

You can [`read`](crate::Reg::read) this register and get [`hr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#HASH:HR7)*/
pub struct HR7rs;
impl crate::RegisterSpec for HR7rs {
    type Ux = u32;
}
///`read()` method returns [`hr7::R`](R) reader structure
impl crate::Readable for HR7rs {}
///`reset()` method sets HR7 to value 0
impl crate::Resettable for HR7rs {}
