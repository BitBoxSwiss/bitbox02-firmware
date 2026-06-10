///Register `BKP18R` reader
pub type R = crate::R<BKP18Rrs>;
///Register `BKP18R` writer
pub type W = crate::W<BKP18Rrs>;
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
        f.debug_struct("BKP18R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKP18Rrs> {
        BKP_W::new(self, 0)
    }
}
/**TAMP backup register

You can [`read`](crate::Reg::read) this register and get [`bkp18r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp18r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TAMP:BKP18R)*/
pub struct BKP18Rrs;
impl crate::RegisterSpec for BKP18Rrs {
    type Ux = u32;
}
///`read()` method returns [`bkp18r::R`](R) reader structure
impl crate::Readable for BKP18Rrs {}
///`write(|w| ..)` method takes [`bkp18r::W`](W) writer structure
impl crate::Writable for BKP18Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKP18R to value 0
impl crate::Resettable for BKP18Rrs {}
