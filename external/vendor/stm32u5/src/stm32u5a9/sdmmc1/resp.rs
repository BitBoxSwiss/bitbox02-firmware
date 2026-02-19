///Register `RESP%s` reader
pub type R = crate::R<RESPrs>;
///Field `CARDSTATUS` reader - Status of a card, which is part of the received response
pub type CARDSTATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Status of a card, which is part of the received response
    #[inline(always)]
    pub fn cardstatus(&self) -> CARDSTATUS_R {
        CARDSTATUS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP")
            .field("cardstatus", &self.cardstatus())
            .finish()
    }
}
/**SDIO response %s register

You can [`read`](crate::Reg::read) this register and get [`resp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#SDMMC1:RESP[1])*/
pub struct RESPrs;
impl crate::RegisterSpec for RESPrs {
    type Ux = u32;
}
///`read()` method returns [`resp::R`](R) reader structure
impl crate::Readable for RESPrs {}
///`reset()` method sets RESP%s to value 0
impl crate::Resettable for RESPrs {}
