///Register `HCDMA14` reader
pub type R = crate::R<HCDMA14rs>;
///Register `HCDMA14` writer
pub type W = crate::W<HCDMA14rs>;
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
        f.debug_struct("HCDMA14")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA14rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 14 DMA address register

You can [`read`](crate::Reg::read) this register and get [`hcdma14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:HCDMA14)*/
pub struct HCDMA14rs;
impl crate::RegisterSpec for HCDMA14rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma14::R`](R) reader structure
impl crate::Readable for HCDMA14rs {}
///`write(|w| ..)` method takes [`hcdma14::W`](W) writer structure
impl crate::Writable for HCDMA14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA14 to value 0
impl crate::Resettable for HCDMA14rs {}
