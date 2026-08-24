///Register `VVBPCCR` reader
pub type R = crate::R<VVBPCCRrs>;
///Field `VBP` reader - Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines.
pub type VBP_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVBPCCR").field("vbp", &self.vbp()).finish()
    }
}
/**DSI Host video VBP current configuration register

You can [`read`](crate::Reg::read) this register and get [`vvbpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VVBPCCR)*/
pub struct VVBPCCRrs;
impl crate::RegisterSpec for VVBPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvbpccr::R`](R) reader structure
impl crate::Readable for VVBPCCRrs {}
///`reset()` method sets VVBPCCR to value 0
impl crate::Resettable for VVBPCCRrs {}
