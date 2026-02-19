///Register `HR0` reader
pub type R = crate::R<HR0rs>;
///Field `H0` reader - H0
pub type H0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H0
    #[inline(always)]
    pub fn h0(&self) -> H0_R {
        H0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR0").field("h0", &self.h0()).finish()
    }
}
/**digest register 0

You can [`read`](crate::Reg::read) this register and get [`hr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#HASH:HR0)*/
pub struct HR0rs;
impl crate::RegisterSpec for HR0rs {
    type Ux = u32;
}
///`read()` method returns [`hr0::R`](R) reader structure
impl crate::Readable for HR0rs {}
///`reset()` method sets HR0 to value 0
impl crate::Resettable for HR0rs {}
