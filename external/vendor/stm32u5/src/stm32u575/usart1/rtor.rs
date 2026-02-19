///Register `RTOR` reader
pub type R = crate::R<RTORrs>;
///Register `RTOR` writer
pub type W = crate::W<RTORrs>;
///Field `RTO` reader - Receiver timeout value
pub type RTO_R = crate::FieldReader<u32>;
///Field `RTO` writer - Receiver timeout value
pub type RTO_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
///Field `BLEN` reader - Block Length
pub type BLEN_R = crate::FieldReader;
///Field `BLEN` writer - Block Length
pub type BLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:23 - Receiver timeout value
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31 - Block Length
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTOR")
            .field("blen", &self.blen())
            .field("rto", &self.rto())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Receiver timeout value
    #[inline(always)]
    pub fn rto(&mut self) -> RTO_W<RTORrs> {
        RTO_W::new(self, 0)
    }
    ///Bits 24:31 - Block Length
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W<RTORrs> {
        BLEN_W::new(self, 24)
    }
}
/**Receiver timeout register

You can [`read`](crate::Reg::read) this register and get [`rtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#USART1:RTOR)*/
pub struct RTORrs;
impl crate::RegisterSpec for RTORrs {
    type Ux = u32;
}
///`read()` method returns [`rtor::R`](R) reader structure
impl crate::Readable for RTORrs {}
///`write(|w| ..)` method takes [`rtor::W`](W) writer structure
impl crate::Writable for RTORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTOR to value 0
impl crate::Resettable for RTORrs {}
