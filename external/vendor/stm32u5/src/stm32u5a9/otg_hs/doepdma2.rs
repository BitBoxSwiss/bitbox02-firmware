///Register `DOEPDMA2` reader
pub type R = crate::R<DOEPDMA2rs>;
///Register `DOEPDMA2` writer
pub type W = crate::W<DOEPDMA2rs>;
///Field `DMAADDR` reader - DMAADDR
pub type DMAADDR_R = crate::FieldReader<u32>;
///Field `DMAADDR` writer - DMAADDR
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA2")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DOEPDMA2rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG device OUT endpoint 2 DMA address register

You can [`read`](crate::Reg::read) this register and get [`doepdma2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:DOEPDMA2)*/
pub struct DOEPDMA2rs;
impl crate::RegisterSpec for DOEPDMA2rs {
    type Ux = u32;
}
///`read()` method returns [`doepdma2::R`](R) reader structure
impl crate::Readable for DOEPDMA2rs {}
///`write(|w| ..)` method takes [`doepdma2::W`](W) writer structure
impl crate::Writable for DOEPDMA2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPDMA2 to value 0
impl crate::Resettable for DOEPDMA2rs {}
