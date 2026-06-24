///Register `RXDR8` reader
pub type R = crate::R<RXDR8rs>;
///Field `RXDR` reader - Receive data register
pub type RXDR_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Receive data register
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDR8").field("rxdr", &self.rxdr()).finish()
    }
}
/**Direct 8-bit access to receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SPI1:RXDR8)*/
pub struct RXDR8rs;
impl crate::RegisterSpec for RXDR8rs {
    type Ux = u8;
}
///`read()` method returns [`rxdr8::R`](R) reader structure
impl crate::Readable for RXDR8rs {}
///`reset()` method sets RXDR8 to value 0
impl crate::Resettable for RXDR8rs {}
