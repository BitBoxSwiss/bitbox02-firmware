///Register `GCR` reader
pub type R = crate::R<GCRrs>;
///Register `GCR` writer
pub type W = crate::W<GCRrs>;
///Field `SYNCIN` reader - Synchronization inputs
pub type SYNCIN_R = crate::FieldReader;
///Field `SYNCIN` writer - Synchronization inputs
pub type SYNCIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SYNCOUT` reader - Synchronization outputs
pub type SYNCOUT_R = crate::FieldReader;
///Field `SYNCOUT` writer - Synchronization outputs
pub type SYNCOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Synchronization inputs
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Synchronization outputs
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR")
            .field("syncin", &self.syncin())
            .field("syncout", &self.syncout())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Synchronization inputs
    #[inline(always)]
    pub fn syncin(&mut self) -> SYNCIN_W<GCRrs> {
        SYNCIN_W::new(self, 0)
    }
    ///Bits 4:5 - Synchronization outputs
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W<GCRrs> {
        SYNCOUT_W::new(self, 4)
    }
}
/**Global configuration register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SAI1:GCR)*/
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
