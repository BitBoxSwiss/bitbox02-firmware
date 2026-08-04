///Register `VVSACCR` reader
pub type R = crate::R<VVSACCRrs>;
///Field `VSA` reader - Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines.
pub type VSA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVSACCR").field("vsa", &self.vsa()).finish()
    }
}
/**DSI Host video VSA current configuration register

You can [`read`](crate::Reg::read) this register and get [`vvsaccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VVSACCR)*/
pub struct VVSACCRrs;
impl crate::RegisterSpec for VVSACCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvsaccr::R`](R) reader structure
impl crate::Readable for VVSACCRrs {}
///`reset()` method sets VVSACCR to value 0
impl crate::Resettable for VVSACCRrs {}
