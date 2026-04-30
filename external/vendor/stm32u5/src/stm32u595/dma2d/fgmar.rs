///Register `FGMAR` reader
pub type R = crate::R<FGMARrs>;
///Register `FGMAR` writer
pub type W = crate::W<FGMARrs>;
///Field `MA` reader - Memory address
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - Memory address
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FGMAR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<FGMARrs> {
        MA_W::new(self, 0)
    }
}
/**foreground memory address register

You can [`read`](crate::Reg::read) this register and get [`fgmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DMA2D:FGMAR)*/
pub struct FGMARrs;
impl crate::RegisterSpec for FGMARrs {
    type Ux = u32;
}
///`read()` method returns [`fgmar::R`](R) reader structure
impl crate::Readable for FGMARrs {}
///`write(|w| ..)` method takes [`fgmar::W`](W) writer structure
impl crate::Writable for FGMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FGMAR to value 0
impl crate::Resettable for FGMARrs {}
