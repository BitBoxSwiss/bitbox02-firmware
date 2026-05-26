///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `FF` reader - Force flush When set, the cache entries are flushed. This bit is reset by hardware when the flushing is complete. Write 0 has no effect.
pub type FF_R = crate::BitReader;
///Field `FF` writer - Force flush When set, the cache entries are flushed. This bit is reset by hardware when the flushing is complete. Write 0 has no effect.
pub type FF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FI` reader - Force invalidate When set, the cache entries are invalidated. This bit is reset by hardware when the invalidation is complete. Write 0 has no effect.
pub type FI_R = crate::BitReader;
///Field `FI` writer - Force invalidate When set, the cache entries are invalidated. This bit is reset by hardware when the invalidation is complete. Write 0 has no effect.
pub type FI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Force flush When set, the cache entries are flushed. This bit is reset by hardware when the flushing is complete. Write 0 has no effect.
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Force invalidate When set, the cache entries are invalidated. This bit is reset by hardware when the invalidation is complete. Write 0 has no effect.
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("ff", &self.ff())
            .field("fi", &self.fi())
            .finish()
    }
}
impl W {
    ///Bit 0 - Force flush When set, the cache entries are flushed. This bit is reset by hardware when the flushing is complete. Write 0 has no effect.
    #[inline(always)]
    pub fn ff(&mut self) -> FF_W<CCRrs> {
        FF_W::new(self, 0)
    }
    ///Bit 1 - Force invalidate When set, the cache entries are invalidated. This bit is reset by hardware when the invalidation is complete. Write 0 has no effect.
    #[inline(always)]
    pub fn fi(&mut self) -> FI_W<CCRrs> {
        FI_W::new(self, 1)
    }
}
/**GFXMMU cache control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GFXMMU:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
