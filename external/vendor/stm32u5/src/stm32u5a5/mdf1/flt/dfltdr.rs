///Register `DFLTDR` reader
pub type R = crate::R<DFLTDRrs>;
///Field `DR` reader - Data processed by digital filter.
pub type DR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 8:31 - Data processed by digital filter.
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLTDR").field("dr", &self.dr()).finish()
    }
}
/**This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`dfltdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DFLTDRrs;
impl crate::RegisterSpec for DFLTDRrs {
    type Ux = u32;
}
///`read()` method returns [`dfltdr::R`](R) reader structure
impl crate::Readable for DFLTDRrs {}
///`reset()` method sets DFLTDR to value 0
impl crate::Resettable for DFLTDRrs {}
