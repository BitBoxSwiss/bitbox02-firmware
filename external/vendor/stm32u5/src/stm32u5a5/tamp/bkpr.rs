///Register `BKP%sR` reader
pub type R = crate::R<BKPRrs>;
///Register `BKP%sR` writer
pub type W = crate::W<BKPRrs>;
///Field `BKP` reader - BKP
pub type BKP_R = crate::FieldReader<u32>;
///Field `BKP` writer - BKP
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKPR").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKPRrs> {
        BKP_W::new(self, 0)
    }
}
/**TAMP backup register

You can [`read`](crate::Reg::read) this register and get [`bkpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#TAMP:BKP[0]R)*/
pub struct BKPRrs;
impl crate::RegisterSpec for BKPRrs {
    type Ux = u32;
}
///`read()` method returns [`bkpr::R`](R) reader structure
impl crate::Readable for BKPRrs {}
///`write(|w| ..)` method takes [`bkpr::W`](W) writer structure
impl crate::Writable for BKPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKP%sR to value 0
impl crate::Resettable for BKPRrs {}
