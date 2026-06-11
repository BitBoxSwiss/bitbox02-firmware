///Register `LWR` reader
pub type R = crate::R<LWRrs>;
///Register `LWR` writer
pub type W = crate::W<LWRrs>;
///Field `LW` reader - Line watermark
pub type LW_R = crate::FieldReader<u16>;
///Field `LW` writer - Line watermark
pub type LW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Line watermark
    #[inline(always)]
    pub fn lw(&self) -> LW_R {
        LW_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LWR").field("lw", &self.lw()).finish()
    }
}
impl W {
    ///Bits 0:15 - Line watermark
    #[inline(always)]
    pub fn lw(&mut self) -> LW_W<LWRrs> {
        LW_W::new(self, 0)
    }
}
/**line watermark register

You can [`read`](crate::Reg::read) this register and get [`lwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DMA2D:LWR)*/
pub struct LWRrs;
impl crate::RegisterSpec for LWRrs {
    type Ux = u32;
}
///`read()` method returns [`lwr::R`](R) reader structure
impl crate::Readable for LWRrs {}
///`write(|w| ..)` method takes [`lwr::W`](W) writer structure
impl crate::Writable for LWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LWR to value 0
impl crate::Resettable for LWRrs {}
