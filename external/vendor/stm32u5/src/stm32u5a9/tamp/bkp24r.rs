///Register `BKP24R` reader
pub type R = crate::R<BKP24Rrs>;
///Register `BKP24R` writer
pub type W = crate::W<BKP24Rrs>;
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
        f.debug_struct("BKP24R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKP24Rrs> {
        BKP_W::new(self, 0)
    }
}
/**TAMP backup register

You can [`read`](crate::Reg::read) this register and get [`bkp24r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp24r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TAMP:BKP24R)*/
pub struct BKP24Rrs;
impl crate::RegisterSpec for BKP24Rrs {
    type Ux = u32;
}
///`read()` method returns [`bkp24r::R`](R) reader structure
impl crate::Readable for BKP24Rrs {}
///`write(|w| ..)` method takes [`bkp24r::W`](W) writer structure
impl crate::Writable for BKP24Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKP24R to value 0
impl crate::Resettable for BKP24Rrs {}
