///Register `TXBRP` reader
pub type R = crate::R<TXBRPrs>;
///Field `TRP` reader - Transmission Request Pending
pub type TRP_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Transmission Request Pending
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBRP").field("trp", &self.trp()).finish()
    }
}
/**FDCAN Tx Buffer Request Pending Register

You can [`read`](crate::Reg::read) this register and get [`txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FDCAN1_RAM:TXBRP)*/
pub struct TXBRPrs;
impl crate::RegisterSpec for TXBRPrs {
    type Ux = u32;
}
///`read()` method returns [`txbrp::R`](R) reader structure
impl crate::Readable for TXBRPrs {}
///`reset()` method sets TXBRP to value 0
impl crate::Resettable for TXBRPrs {}
