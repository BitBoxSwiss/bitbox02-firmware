///Register `TXBCR` reader
pub type R = crate::R<TXBCRrs>;
///Register `TXBCR` writer
pub type W = crate::W<TXBCRrs>;
///Field `CR` reader - Cancellation Request
pub type CR_R = crate::FieldReader;
///Field `CR` writer - Cancellation Request
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Cancellation Request
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCR").field("cr", &self.cr()).finish()
    }
}
impl W {
    ///Bits 0:2 - Cancellation Request
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<TXBCRrs> {
        CR_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Cancellation Request Register

You can [`read`](crate::Reg::read) this register and get [`txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FDCAN1_RAM:TXBCR)*/
pub struct TXBCRrs;
impl crate::RegisterSpec for TXBCRrs {
    type Ux = u32;
}
///`read()` method returns [`txbcr::R`](R) reader structure
impl crate::Readable for TXBCRrs {}
///`write(|w| ..)` method takes [`txbcr::W`](W) writer structure
impl crate::Writable for TXBCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXBCR to value 0
impl crate::Resettable for TXBCRrs {}
