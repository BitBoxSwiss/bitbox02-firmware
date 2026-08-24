///Register `DFLT0DR` reader
pub type R = crate::R<DFLT0DRrs>;
///Field `DR` reader - DR
pub type DR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 8:31 - DR
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT0DR").field("dr", &self.dr()).finish()
    }
}
/**ADF digital filter data register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADF1:DFLT0DR)*/
pub struct DFLT0DRrs;
impl crate::RegisterSpec for DFLT0DRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt0dr::R`](R) reader structure
impl crate::Readable for DFLT0DRrs {}
///`reset()` method sets DFLT0DR to value 0
impl crate::Resettable for DFLT0DRrs {}
