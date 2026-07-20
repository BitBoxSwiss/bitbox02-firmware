///Register `DIEPDMA4` reader
pub type R = crate::R<DIEPDMA4rs>;
///Register `DIEPDMA4` writer
pub type W = crate::W<DIEPDMA4rs>;
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
        f.debug_struct("DIEPDMA4")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DIEPDMA4rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG device IN endpoint 4 DMA address register

You can [`read`](crate::Reg::read) this register and get [`diepdma4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:DIEPDMA4)*/
pub struct DIEPDMA4rs;
impl crate::RegisterSpec for DIEPDMA4rs {
    type Ux = u32;
}
///`read()` method returns [`diepdma4::R`](R) reader structure
impl crate::Readable for DIEPDMA4rs {}
///`write(|w| ..)` method takes [`diepdma4::W`](W) writer structure
impl crate::Writable for DIEPDMA4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPDMA4 to value 0
impl crate::Resettable for DIEPDMA4rs {}
