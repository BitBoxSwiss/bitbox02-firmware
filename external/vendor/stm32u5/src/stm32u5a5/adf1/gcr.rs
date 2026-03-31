///Register `GCR` reader
pub type R = crate::R<GCRrs>;
///Register `GCR` writer
pub type W = crate::W<GCRrs>;
///Field `TRGO` reader - Trigger output control Set by software and reset by
pub type TRGO_R = crate::BitReader;
///Field `TRGO` writer - Trigger output control Set by software and reset by
pub type TRGO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Trigger output control Set by software and reset by
    #[inline(always)]
    pub fn trgo(&self) -> TRGO_R {
        TRGO_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR").field("trgo", &self.trgo()).finish()
    }
}
impl W {
    ///Bit 0 - Trigger output control Set by software and reset by
    #[inline(always)]
    pub fn trgo(&mut self) -> TRGO_W<GCRrs> {
        TRGO_W::new(self, 0)
    }
}
/**ADF Global Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:GCR)*/
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
///`read()` method returns [`gcr::R`](R) reader structure
impl crate::Readable for GCRrs {}
///`write(|w| ..)` method takes [`gcr::W`](W) writer structure
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCR to value 0
impl crate::Resettable for GCRrs {}
