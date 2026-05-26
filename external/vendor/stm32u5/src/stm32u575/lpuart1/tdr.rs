///Register `TDR` reader
pub type R = crate::R<TDRrs>;
///Register `TDR` writer
pub type W = crate::W<TDRrs>;
///Field `TDR` reader - Transmit data value
pub type TDR_R = crate::FieldReader<u16>;
///Field `TDR` writer - Transmit data value
pub type TDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Transmit data value
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDR").field("tdr", &self.tdr()).finish()
    }
}
impl W {
    ///Bits 0:8 - Transmit data value
    #[inline(always)]
    pub fn tdr(&mut self) -> TDR_W<TDRrs> {
        TDR_W::new(self, 0)
    }
}
/**Transmit data register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#LPUART1:TDR)*/
pub struct TDRrs;
impl crate::RegisterSpec for TDRrs {
    type Ux = u32;
}
///`read()` method returns [`tdr::R`](R) reader structure
impl crate::Readable for TDRrs {}
///`write(|w| ..)` method takes [`tdr::W`](W) writer structure
impl crate::Writable for TDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDR to value 0
impl crate::Resettable for TDRrs {}
