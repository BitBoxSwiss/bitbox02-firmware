///Register `HCDMA9` reader
pub type R = crate::R<HCDMA9rs>;
///Register `HCDMA9` writer
pub type W = crate::W<HCDMA9rs>;
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
        f.debug_struct("HCDMA9")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA9rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 9 DMA address register

You can [`read`](crate::Reg::read) this register and get [`hcdma9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTG_HS:HCDMA9)*/
pub struct HCDMA9rs;
impl crate::RegisterSpec for HCDMA9rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma9::R`](R) reader structure
impl crate::Readable for HCDMA9rs {}
///`write(|w| ..)` method takes [`hcdma9::W`](W) writer structure
impl crate::Writable for HCDMA9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA9 to value 0
impl crate::Resettable for HCDMA9rs {}
