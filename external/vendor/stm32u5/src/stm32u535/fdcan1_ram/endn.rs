///Register `ENDN` reader
pub type R = crate::R<ENDNrs>;
///Field `ETV` reader - Endiannes Test Value
pub type ETV_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Endiannes Test Value
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDN").field("etv", &self.etv()).finish()
    }
}
/**FDCAN endian register

You can [`read`](crate::Reg::read) this register and get [`endn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#FDCAN1_RAM:ENDN)*/
pub struct ENDNrs;
impl crate::RegisterSpec for ENDNrs {
    type Ux = u32;
}
///`read()` method returns [`endn::R`](R) reader structure
impl crate::Readable for ENDNrs {}
///`reset()` method sets ENDN to value 0x8765_4321
impl crate::Resettable for ENDNrs {
    const RESET_VALUE: u32 = 0x8765_4321;
}
