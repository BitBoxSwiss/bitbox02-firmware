///Register `_PSMKR` reader
pub type R = crate::R<_PSMKRrs>;
///Register `_PSMKR` writer
pub type W = crate::W<_PSMKRrs>;
///Field `MASK` reader - Status mask Mask to be applied to the status bytes received in Polling mode For bit n:
pub type MASK_R = crate::FieldReader<u32>;
///Field `MASK` writer - Status mask Mask to be applied to the status bytes received in Polling mode For bit n:
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Status mask Mask to be applied to the status bytes received in Polling mode For bit n:
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_PSMKR")
            .field("mask", &self.mask())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Status mask Mask to be applied to the status bytes received in Polling mode For bit n:
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W<_PSMKRrs> {
        MASK_W::new(self, 0)
    }
}
/**HSPI polling status mask register

You can [`read`](crate::Reg::read) this register and get [`_psmkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_psmkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_PSMKR)*/
pub struct _PSMKRrs;
impl crate::RegisterSpec for _PSMKRrs {
    type Ux = u32;
}
///`read()` method returns [`_psmkr::R`](R) reader structure
impl crate::Readable for _PSMKRrs {}
///`write(|w| ..)` method takes [`_psmkr::W`](W) writer structure
impl crate::Writable for _PSMKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _PSMKR to value 0
impl crate::Resettable for _PSMKRrs {}
