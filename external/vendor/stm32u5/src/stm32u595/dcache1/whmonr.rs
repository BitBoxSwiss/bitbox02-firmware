///Register `WHMONR` reader
pub type R = crate::R<WHMONRrs>;
///Field `WHITMON` reader - WHITMON
pub type WHITMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - WHITMON
    #[inline(always)]
    pub fn whitmon(&self) -> WHITMON_R {
        WHITMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WHMONR")
            .field("whitmon", &self.whitmon())
            .finish()
    }
}
/**write-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`whmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DCACHE1:WHMONR)*/
pub struct WHMONRrs;
impl crate::RegisterSpec for WHMONRrs {
    type Ux = u32;
}
///`read()` method returns [`whmonr::R`](R) reader structure
impl crate::Readable for WHMONRrs {}
///`reset()` method sets WHMONR to value 0
impl crate::Resettable for WHMONRrs {}
