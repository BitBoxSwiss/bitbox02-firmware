///Register `SADSDLVR` reader
pub type R = crate::R<SADSDLVRrs>;
///Field `SDLVL` reader - SDLVL
pub type SDLVL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - SDLVL
    #[inline(always)]
    pub fn sdlvl(&self) -> SDLVL_R {
        SDLVL_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SADSDLVR")
            .field("sdlvl", &self.sdlvl())
            .finish()
    }
}
/**ADF SAD sound level register

You can [`read`](crate::Reg::read) this register and get [`sadsdlvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADF1:SADSDLVR)*/
pub struct SADSDLVRrs;
impl crate::RegisterSpec for SADSDLVRrs {
    type Ux = u32;
}
///`read()` method returns [`sadsdlvr::R`](R) reader structure
impl crate::Readable for SADSDLVRrs {}
///`reset()` method sets SADSDLVR to value 0
impl crate::Resettable for SADSDLVRrs {}
