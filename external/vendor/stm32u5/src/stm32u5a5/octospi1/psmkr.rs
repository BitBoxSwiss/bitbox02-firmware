///Register `PSMKR` reader
pub type R = crate::R<PSMKRrs>;
///Register `PSMKR` writer
pub type W = crate::W<PSMKRrs>;
///Field `MASK` reader - Status MASK
pub type MASK_R = crate::FieldReader<u32>;
///Field `MASK` writer - Status MASK
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Status MASK
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSMKR").field("mask", &self.mask()).finish()
    }
}
impl W {
    ///Bits 0:31 - Status MASK
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W<PSMKRrs> {
        MASK_W::new(self, 0)
    }
}
/**polling status mask register

You can [`read`](crate::Reg::read) this register and get [`psmkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OCTOSPI1:PSMKR)*/
pub struct PSMKRrs;
impl crate::RegisterSpec for PSMKRrs {
    type Ux = u32;
}
///`read()` method returns [`psmkr::R`](R) reader structure
impl crate::Readable for PSMKRrs {}
///`write(|w| ..)` method takes [`psmkr::W`](W) writer structure
impl crate::Writable for PSMKRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets PSMKR to value 0
impl crate::Resettable for PSMKRrs {}
