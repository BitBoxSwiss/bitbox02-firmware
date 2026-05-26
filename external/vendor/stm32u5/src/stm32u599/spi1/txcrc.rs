///Register `TXCRC` reader
pub type R = crate::R<TXCRCrs>;
///Field `TXCRC` reader - CRC register for transmitter
pub type TXCRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CRC register for transmitter
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXCRC")
            .field("txcrc", &self.txcrc())
            .finish()
    }
}
/**Transmitter CRC Register

You can [`read`](crate::Reg::read) this register and get [`txcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#SPI1:TXCRC)*/
pub struct TXCRCrs;
impl crate::RegisterSpec for TXCRCrs {
    type Ux = u32;
}
///`read()` method returns [`txcrc::R`](R) reader structure
impl crate::Readable for TXCRCrs {}
///`reset()` method sets TXCRC to value 0
impl crate::Resettable for TXCRCrs {}
