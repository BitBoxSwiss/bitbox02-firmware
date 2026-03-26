///Register `RXDR` reader
pub type R = crate::R<RXDRrs>;
///Field `RXDR` reader - receive data register The register serves as an interface with RxFIFO. When it is read, RxFIFO is accessed. Note: data is always right-aligned. Unused bits are read as zero when the register is read. Writing to the register is ignored. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is read by single access halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be read by single access word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be read by single access. Read access of this register less than the configured data size is forbidden.
pub type RXDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - receive data register The register serves as an interface with RxFIFO. When it is read, RxFIFO is accessed. Note: data is always right-aligned. Unused bits are read as zero when the register is read. Writing to the register is ignored. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is read by single access halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be read by single access word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be read by single access. Read access of this register less than the configured data size is forbidden.
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDR").field("rxdr", &self.rxdr()).finish()
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:RXDR)*/
pub struct RXDRrs;
impl crate::RegisterSpec for RXDRrs {
    type Ux = u32;
}
///`read()` method returns [`rxdr::R`](R) reader structure
impl crate::Readable for RXDRrs {}
///`reset()` method sets RXDR to value 0
impl crate::Resettable for RXDRrs {}
