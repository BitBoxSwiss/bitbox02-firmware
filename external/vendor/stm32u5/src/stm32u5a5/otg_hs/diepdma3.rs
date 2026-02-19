///Register `DIEPDMA3` reader
pub type R = crate::R<DIEPDMA3rs>;
///Register `DIEPDMA3` writer
pub type W = crate::W<DIEPDMA3rs>;
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
        f.debug_struct("DIEPDMA3")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DIEPDMA3rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG device IN endpoint 3 DMA address register

You can [`read`](crate::Reg::read) this register and get [`diepdma3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTG_HS:DIEPDMA3)*/
pub struct DIEPDMA3rs;
impl crate::RegisterSpec for DIEPDMA3rs {
    type Ux = u32;
}
///`read()` method returns [`diepdma3::R`](R) reader structure
impl crate::Readable for DIEPDMA3rs {}
///`write(|w| ..)` method takes [`diepdma3::W`](W) writer structure
impl crate::Writable for DIEPDMA3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPDMA3 to value 0
impl crate::Resettable for DIEPDMA3rs {}
