///Register `TXDR16` writer
pub type W = crate::W<TXDR16rs>;
///Field `TXDR` writer - Transmit data register
pub type TXDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<TXDR16rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Transmit data register
    #[inline(always)]
    pub fn txdr(&mut self) -> TXDR_W<TXDR16rs> {
        TXDR_W::new(self, 0)
    }
}
/**Direct 16-bit access to transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr16::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#SPI1:TXDR16)*/
pub struct TXDR16rs;
impl crate::RegisterSpec for TXDR16rs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`txdr16::W`](W) writer structure
impl crate::Writable for TXDR16rs {
    type Safety = crate::Safe;
}
///`reset()` method sets TXDR16 to value 0
impl crate::Resettable for TXDR16rs {}
