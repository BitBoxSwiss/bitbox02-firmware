///Register `BGMAR` reader
pub type R = crate::R<BGMARrs>;
///Register `BGMAR` writer
pub type W = crate::W<BGMARrs>;
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
        f.debug_struct("BGMAR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<BGMARrs> {
        MA_W::new(self, 0)
    }
}
/**background memory address register

You can [`read`](crate::Reg::read) this register and get [`bgmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:BGMAR)*/
pub struct BGMARrs;
impl crate::RegisterSpec for BGMARrs {
    type Ux = u32;
}
///`read()` method returns [`bgmar::R`](R) reader structure
impl crate::Readable for BGMARrs {}
///`write(|w| ..)` method takes [`bgmar::W`](W) writer structure
impl crate::Writable for BGMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BGMAR to value 0
impl crate::Resettable for BGMARrs {}
