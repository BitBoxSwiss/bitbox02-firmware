///Register `RXCRC` reader
pub type R = crate::R<RXCRCrs>;
///Field `RXCRC` reader - CRC register for receiver
pub type RXCRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CRC register for receiver
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXCRC")
            .field("rxcrc", &self.rxcrc())
            .finish()
    }
}
/**Receiver CRC Register

You can [`read`](crate::Reg::read) this register and get [`rxcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#SPI1:RXCRC)*/
pub struct RXCRCrs;
impl crate::RegisterSpec for RXCRCrs {
    type Ux = u32;
}
///`read()` method returns [`rxcrc::R`](R) reader structure
impl crate::Readable for RXCRCrs {}
///`reset()` method sets RXCRC to value 0
impl crate::Resettable for RXCRCrs {}
