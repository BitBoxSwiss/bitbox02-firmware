///Register `SAR` reader
pub type R = crate::R<SARrs>;
///Register `SAR` writer
pub type W = crate::W<SARrs>;
///Field `SA` reader - source address
pub type SA_R = crate::FieldReader<u32>;
///Field `SA` writer - source address
pub type SA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - source address
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR").field("sa", &self.sa()).finish()
    }
}
impl W {
    ///Bits 0:31 - source address
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<SARrs> {
        SA_W::new(self, 0)
    }
}
/**GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SARrs;
impl crate::RegisterSpec for SARrs {
    type Ux = u32;
}
///`read()` method returns [`sar::R`](R) reader structure
impl crate::Readable for SARrs {}
///`write(|w| ..)` method takes [`sar::W`](W) writer structure
impl crate::Writable for SARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SAR to value 0
impl crate::Resettable for SARrs {}
