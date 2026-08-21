///Register `VPCCR` reader
pub type R = crate::R<VPCCRrs>;
///Field `VPSIZE` reader - Video packet size This field returns the number of pixels in a single video packet.
pub type VPSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:13 - Video packet size This field returns the number of pixels in a single video packet.
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VPCCR")
            .field("vpsize", &self.vpsize())
            .finish()
    }
}
/**DSI Host video packet current configuration register

You can [`read`](crate::Reg::read) this register and get [`vpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VPCCR)*/
pub struct VPCCRrs;
impl crate::RegisterSpec for VPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vpccr::R`](R) reader structure
impl crate::Readable for VPCCRrs {}
///`reset()` method sets VPCCR to value 0
impl crate::Resettable for VPCCRrs {}
