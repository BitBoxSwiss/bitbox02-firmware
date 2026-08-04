///Register `HCDMA3` reader
pub type R = crate::R<HCDMA3rs>;
///Register `HCDMA3` writer
pub type W = crate::W<HCDMA3rs>;
///Field `DMAADDR` reader - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
pub type DMAADDR_R = crate::FieldReader<u32>;
///Field `DMAADDR` writer - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA3")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA3rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 3 DMA address register

You can [`read`](crate::Reg::read) this register and get [`hcdma3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#OTG_HS:HCDMA3)*/
pub struct HCDMA3rs;
impl crate::RegisterSpec for HCDMA3rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma3::R`](R) reader structure
impl crate::Readable for HCDMA3rs {}
///`write(|w| ..)` method takes [`hcdma3::W`](W) writer structure
impl crate::Writable for HCDMA3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA3 to value 0
impl crate::Resettable for HCDMA3rs {}
