///Register `RX_PAYSZR` reader
pub type R = crate::R<RX_PAYSZRrs>;
///Field `RXPAYSZ` reader - RXPAYSZ
pub type RXPAYSZ_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PAYSZR")
            .field("rxpaysz", &self.rxpaysz())
            .finish()
    }
}
/**UCPD Rx payload size Register

You can [`read`](crate::Reg::read) this register and get [`rx_payszr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#UCPD1:RX_PAYSZR)*/
pub struct RX_PAYSZRrs;
impl crate::RegisterSpec for RX_PAYSZRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_payszr::R`](R) reader structure
impl crate::Readable for RX_PAYSZRrs {}
///`reset()` method sets RX_PAYSZR to value 0
impl crate::Resettable for RX_PAYSZRrs {}
