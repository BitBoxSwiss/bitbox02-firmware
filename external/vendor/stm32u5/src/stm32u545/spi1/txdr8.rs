///Register `TXDR8` writer
pub type W = crate::W<TXDR8rs>;
///Field `TXDR` writer - Transmit data register
pub type TXDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<TXDR8rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Transmit data register
    #[inline(always)]
    pub fn txdr(&mut self) -> TXDR_W<TXDR8rs> {
        TXDR_W::new(self, 0)
    }
}
/**Direct 8-bit access to transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr8::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SPI1:TXDR8)*/
pub struct TXDR8rs;
impl crate::RegisterSpec for TXDR8rs {
    type Ux = u8;
}
///`write(|w| ..)` method takes [`txdr8::W`](W) writer structure
impl crate::Writable for TXDR8rs {
    type Safety = crate::Safe;
}
///`reset()` method sets TXDR8 to value 0
impl crate::Resettable for TXDR8rs {}
