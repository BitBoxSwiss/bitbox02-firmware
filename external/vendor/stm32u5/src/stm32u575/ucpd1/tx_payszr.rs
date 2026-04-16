///Register `TX_PAYSZR` reader
pub type R = crate::R<TX_PAYSZRrs>;
///Register `TX_PAYSZR` writer
pub type W = crate::W<TX_PAYSZRrs>;
///Field `TXPAYSZ` reader - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
pub type TXPAYSZ_R = crate::FieldReader<u16>;
///Field `TXPAYSZ` writer - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
pub type TXPAYSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    ///Bits 0:9 - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
    #[inline(always)]
    pub fn txpaysz(&self) -> TXPAYSZ_R {
        TXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PAYSZR")
            .field("txpaysz", &self.txpaysz())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
    #[inline(always)]
    pub fn txpaysz(&mut self) -> TXPAYSZ_W<TX_PAYSZRrs> {
        TXPAYSZ_W::new(self, 0)
    }
}
/**UCPD Tx payload size register

You can [`read`](crate::Reg::read) this register and get [`tx_payszr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_payszr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:TX_PAYSZR)*/
pub struct TX_PAYSZRrs;
impl crate::RegisterSpec for TX_PAYSZRrs {
    type Ux = u32;
}
///`read()` method returns [`tx_payszr::R`](R) reader structure
impl crate::Readable for TX_PAYSZRrs {}
///`write(|w| ..)` method takes [`tx_payszr::W`](W) writer structure
impl crate::Writable for TX_PAYSZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TX_PAYSZR to value 0
impl crate::Resettable for TX_PAYSZRrs {}
