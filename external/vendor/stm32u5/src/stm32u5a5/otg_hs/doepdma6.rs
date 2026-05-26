///Register `DOEPDMA6` reader
pub type R = crate::R<DOEPDMA6rs>;
///Register `DOEPDMA6` writer
pub type W = crate::W<DOEPDMA6rs>;
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
        f.debug_struct("DOEPDMA6")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DOEPDMA6rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG device OUT endpoint 6 DMA address register

You can [`read`](crate::Reg::read) this register and get [`doepdma6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTG_HS:DOEPDMA6)*/
pub struct DOEPDMA6rs;
impl crate::RegisterSpec for DOEPDMA6rs {
    type Ux = u32;
}
///`read()` method returns [`doepdma6::R`](R) reader structure
impl crate::Readable for DOEPDMA6rs {}
///`write(|w| ..)` method takes [`doepdma6::W`](W) writer structure
impl crate::Writable for DOEPDMA6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPDMA6 to value 0
impl crate::Resettable for DOEPDMA6rs {}
