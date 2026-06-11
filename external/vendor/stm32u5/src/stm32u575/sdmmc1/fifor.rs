///Register `FIFOR%s` reader
pub type R = crate::R<FIFORrs>;
///Register `FIFOR%s` writer
pub type W = crate::W<FIFORrs>;
///Field `FIFODATA` reader - Receive and transmit FIFO data
pub type FIFODATA_R = crate::FieldReader<u32>;
///Field `FIFODATA` writer - Receive and transmit FIFO data
pub type FIFODATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOR")
            .field("fifodata", &self.fifodata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W<FIFORrs> {
        FIFODATA_W::new(self, 0)
    }
}
/**data FIFO register %s

You can [`read`](crate::Reg::read) this register and get [`fifor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SDMMC1:FIFOR[0])*/
pub struct FIFORrs;
impl crate::RegisterSpec for FIFORrs {
    type Ux = u32;
}
///`read()` method returns [`fifor::R`](R) reader structure
impl crate::Readable for FIFORrs {}
///`write(|w| ..)` method takes [`fifor::W`](W) writer structure
impl crate::Writable for FIFORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIFOR%s to value 0
impl crate::Resettable for FIFORrs {}
