///Register `RX_ORDEXTR2` reader
pub type R = crate::R<RX_ORDEXTR2rs>;
///Register `RX_ORDEXTR2` writer
pub type W = crate::W<RX_ORDEXTR2rs>;
///Field `RXSOPX2` reader - RXSOPX2
pub type RXSOPX2_R = crate::FieldReader<u32>;
///Field `RXSOPX2` writer - RXSOPX2
pub type RXSOPX2_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32, crate::Safe>;
impl R {
    ///Bits 0:19 - RXSOPX2
    #[inline(always)]
    pub fn rxsopx2(&self) -> RXSOPX2_R {
        RXSOPX2_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDEXTR2")
            .field("rxsopx2", &self.rxsopx2())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - RXSOPX2
    #[inline(always)]
    pub fn rxsopx2(&mut self) -> RXSOPX2_W<RX_ORDEXTR2rs> {
        RXSOPX2_W::new(self, 0)
    }
}
/**UCPD Rx Ordered Set Extension Register 2

You can [`read`](crate::Reg::read) this register and get [`rx_ordextr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordextr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#UCPD1:RX_ORDEXTR2)*/
pub struct RX_ORDEXTR2rs;
impl crate::RegisterSpec for RX_ORDEXTR2rs {
    type Ux = u32;
}
///`read()` method returns [`rx_ordextr2::R`](R) reader structure
impl crate::Readable for RX_ORDEXTR2rs {}
///`write(|w| ..)` method takes [`rx_ordextr2::W`](W) writer structure
impl crate::Writable for RX_ORDEXTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RX_ORDEXTR2 to value 0
impl crate::Resettable for RX_ORDEXTR2rs {}
