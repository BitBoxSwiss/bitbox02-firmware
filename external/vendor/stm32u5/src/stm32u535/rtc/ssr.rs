///Register `SSR` reader
pub type R = crate::R<SSRrs>;
///Field `SS` reader - SS
pub type SS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SS
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSR").field("ss", &self.ss()).finish()
    }
}
/**RTC sub second register

You can [`read`](crate::Reg::read) this register and get [`ssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RTC:SSR)*/
pub struct SSRrs;
impl crate::RegisterSpec for SSRrs {
    type Ux = u32;
}
///`read()` method returns [`ssr::R`](R) reader structure
impl crate::Readable for SSRrs {}
///`reset()` method sets SSR to value 0
impl crate::Resettable for SSRrs {}
