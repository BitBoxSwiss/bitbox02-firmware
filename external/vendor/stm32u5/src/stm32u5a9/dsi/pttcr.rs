///Register `PTTCR` reader
pub type R = crate::R<PTTCRrs>;
///Register `PTTCR` writer
pub type W = crate::W<PTTCRrs>;
///Field `TX_TRIG` reader - Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time.
pub type TX_TRIG_R = crate::FieldReader;
///Field `TX_TRIG` writer - Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time.
pub type TX_TRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time.
    #[inline(always)]
    pub fn tx_trig(&self) -> TX_TRIG_R {
        TX_TRIG_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTTCR")
            .field("tx_trig", &self.tx_trig())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time.
    #[inline(always)]
    pub fn tx_trig(&mut self) -> TX_TRIG_W<PTTCRrs> {
        TX_TRIG_W::new(self, 0)
    }
}
/**DSI Host PHY TX triggers configuration register

You can [`read`](crate::Reg::read) this register and get [`pttcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pttcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:PTTCR)*/
pub struct PTTCRrs;
impl crate::RegisterSpec for PTTCRrs {
    type Ux = u32;
}
///`read()` method returns [`pttcr::R`](R) reader structure
impl crate::Readable for PTTCRrs {}
///`write(|w| ..)` method takes [`pttcr::W`](W) writer structure
impl crate::Writable for PTTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTTCR to value 0
impl crate::Resettable for PTTCRrs {}
