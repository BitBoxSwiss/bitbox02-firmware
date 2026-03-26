///Register `UDRDR` reader
pub type R = crate::R<UDRDRrs>;
///Register `UDRDR` writer
pub type W = crate::W<UDRDRrs>;
///Field `UDRDR` reader - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\[31-16\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored.
pub type UDRDR_R = crate::FieldReader<u32>;
///Field `UDRDR` writer - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\[31-16\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored.
pub type UDRDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\[31-16\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored.
    #[inline(always)]
    pub fn udrdr(&self) -> UDRDR_R {
        UDRDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDRDR")
            .field("udrdr", &self.udrdr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\[31-16\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored.
    #[inline(always)]
    pub fn udrdr(&mut self) -> UDRDR_W<UDRDRrs> {
        UDRDR_W::new(self, 0)
    }
}
/**SPI underrun data register

You can [`read`](crate::Reg::read) this register and get [`udrdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udrdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SPI1:UDRDR)*/
pub struct UDRDRrs;
impl crate::RegisterSpec for UDRDRrs {
    type Ux = u32;
}
///`read()` method returns [`udrdr::R`](R) reader structure
impl crate::Readable for UDRDRrs {}
///`write(|w| ..)` method takes [`udrdr::W`](W) writer structure
impl crate::Writable for UDRDRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets UDRDR to value 0
impl crate::Resettable for UDRDRrs {}
