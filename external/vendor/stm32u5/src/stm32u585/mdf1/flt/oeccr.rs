///Register `OECCR` reader
pub type R = crate::R<OECCRrs>;
///Register `OECCR` writer
pub type W = crate::W<OECCRrs>;
///Field `OFFSET` reader - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\] field will inform the application on the current offset value. OFFSET\[25:0\] represents the value to be subtracted to the signal before going to the SCALE.
pub type OFFSET_R = crate::FieldReader<u32>;
///Field `OFFSET` writer - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\] field will inform the application on the current offset value. OFFSET\[25:0\] represents the value to be subtracted to the signal before going to the SCALE.
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\] field will inform the application on the current offset value. OFFSET\[25:0\] represents the value to be subtracted to the signal before going to the SCALE.
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OECCR")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\] field will inform the application on the current offset value. OFFSET\[25:0\] represents the value to be subtracted to the signal before going to the SCALE.
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<OECCRrs> {
        OFFSET_W::new(self, 0)
    }
}
/**This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`oeccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OECCRrs;
impl crate::RegisterSpec for OECCRrs {
    type Ux = u32;
}
///`read()` method returns [`oeccr::R`](R) reader structure
impl crate::Readable for OECCRrs {}
///`write(|w| ..)` method takes [`oeccr::W`](W) writer structure
impl crate::Writable for OECCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OECCR to value 0
impl crate::Resettable for OECCRrs {}
