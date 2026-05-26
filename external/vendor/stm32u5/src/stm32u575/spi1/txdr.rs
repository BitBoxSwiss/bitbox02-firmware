///Register `TXDR` writer
pub type W = crate::W<TXDRrs>;
///Field `TXDR` writer - transmit data register The register serves as an interface with TxFIFO. A write to it accesses TxFIFO. Note: data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is written by single access. halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be written by single access. word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be written by single access. Write access of this register less than the configured data size is forbidden.
pub type TXDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<TXDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - transmit data register The register serves as an interface with TxFIFO. A write to it accesses TxFIFO. Note: data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is written by single access. halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be written by single access. word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be written by single access. Write access of this register less than the configured data size is forbidden.
    #[inline(always)]
    pub fn txdr(&mut self) -> TXDR_W<TXDRrs> {
        TXDR_W::new(self, 0)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:TXDR)*/
pub struct TXDRrs;
impl crate::RegisterSpec for TXDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`txdr::W`](W) writer structure
impl crate::Writable for TXDRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets TXDR to value 0
impl crate::Resettable for TXDRrs {}
