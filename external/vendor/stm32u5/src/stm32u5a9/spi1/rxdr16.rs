///Register `RXDR16` reader
pub type R = crate::R<RXDR16rs>;
///Field `RXDR` reader - Receive data register
pub type RXDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Receive data register
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDR16")
            .field("rxdr", &self.rxdr())
            .finish()
    }
}
/**Direct 16-bit access to receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#SPI1:RXDR16)*/
pub struct RXDR16rs;
impl crate::RegisterSpec for RXDR16rs {
    type Ux = u16;
}
///`read()` method returns [`rxdr16::R`](R) reader structure
impl crate::Readable for RXDR16rs {}
///`reset()` method sets RXDR16 to value 0
impl crate::Resettable for RXDR16rs {}
