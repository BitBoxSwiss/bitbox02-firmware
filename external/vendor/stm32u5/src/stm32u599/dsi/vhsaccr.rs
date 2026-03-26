///Register `VHSACCR` reader
pub type R = crate::R<VHSACCRrs>;
///Field `HSA` reader - Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles.
pub type HSA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles.
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VHSACCR").field("hsa", &self.hsa()).finish()
    }
}
/**DSI Host video HSA current configuration register

You can [`read`](crate::Reg::read) this register and get [`vhsaccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VHSACCR)*/
pub struct VHSACCRrs;
impl crate::RegisterSpec for VHSACCRrs {
    type Ux = u32;
}
///`read()` method returns [`vhsaccr::R`](R) reader structure
impl crate::Readable for VHSACCRrs {}
///`reset()` method sets VHSACCR to value 0
impl crate::Resettable for VHSACCRrs {}
