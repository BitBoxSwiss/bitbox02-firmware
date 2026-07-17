///Register `RXCRC` reader
pub type R = crate::R<RXCRCrs>;
///Field `RXCRC` reader - CRC register for receiver When CRC calculation is enabled, the RXCRC\[31:0\] bits contain the computed CRC value of the subsequently received bytes. CRC calculation is initialized when the CRCEN bit of SPI_CR1 is written to 1 or when a data block is transacted completely. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPOLY register. The number of bits considered at calculation depends on SPI_CRCPOLY register and CRCSIZE bits settings at SPI_CFG1 register. Note: a read to this register when the communication is ongoing could return an incorrect value. RXCRC\[31-16\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored. Note: The configuration of CRCSIZE bit field is not taken into account when the content of this register is read by software. No masking is applied for unused bits at this case.
pub type RXCRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CRC register for receiver When CRC calculation is enabled, the RXCRC\[31:0\] bits contain the computed CRC value of the subsequently received bytes. CRC calculation is initialized when the CRCEN bit of SPI_CR1 is written to 1 or when a data block is transacted completely. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPOLY register. The number of bits considered at calculation depends on SPI_CRCPOLY register and CRCSIZE bits settings at SPI_CFG1 register. Note: a read to this register when the communication is ongoing could return an incorrect value. RXCRC\[31-16\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored. Note: The configuration of CRCSIZE bit field is not taken into account when the content of this register is read by software. No masking is applied for unused bits at this case.
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
/**

You can [`read`](crate::Reg::read) this register and get [`rxcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SPI1:RXCRC)*/
pub struct RXCRCrs;
impl crate::RegisterSpec for RXCRCrs {
    type Ux = u32;
}
///`read()` method returns [`rxcrc::R`](R) reader structure
impl crate::Readable for RXCRCrs {}
///`reset()` method sets RXCRC to value 0
impl crate::Resettable for RXCRCrs {}
