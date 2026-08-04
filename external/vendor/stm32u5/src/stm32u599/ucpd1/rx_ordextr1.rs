///Register `RX_ORDEXTR1` reader
pub type R = crate::R<RX_ORDEXTR1rs>;
///Register `RX_ORDEXTR1` writer
pub type W = crate::W<RX_ORDEXTR1rs>;
///Field `RXSOPX1` reader - RXSOPX1
pub type RXSOPX1_R = crate::FieldReader<u32>;
///Field `RXSOPX1` writer - RXSOPX1
pub type RXSOPX1_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32, crate::Safe>;
impl R {
    ///Bits 0:19 - RXSOPX1
    #[inline(always)]
    pub fn rxsopx1(&self) -> RXSOPX1_R {
        RXSOPX1_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDEXTR1")
            .field("rxsopx1", &self.rxsopx1())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - RXSOPX1
    #[inline(always)]
    pub fn rxsopx1(&mut self) -> RXSOPX1_W<RX_ORDEXTR1rs> {
        RXSOPX1_W::new(self, 0)
    }
}
/**UCPD Rx Ordered Set Extension Register 1

You can [`read`](crate::Reg::read) this register and get [`rx_ordextr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordextr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:RX_ORDEXTR1)*/
pub struct RX_ORDEXTR1rs;
impl crate::RegisterSpec for RX_ORDEXTR1rs {
    type Ux = u32;
}
///`read()` method returns [`rx_ordextr1::R`](R) reader structure
impl crate::Readable for RX_ORDEXTR1rs {}
///`write(|w| ..)` method takes [`rx_ordextr1::W`](W) writer structure
impl crate::Writable for RX_ORDEXTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RX_ORDEXTR1 to value 0
impl crate::Resettable for RX_ORDEXTR1rs {}
