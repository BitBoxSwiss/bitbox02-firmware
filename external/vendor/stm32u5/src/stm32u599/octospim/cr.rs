///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `MUXEN` reader - Multiplexed mode enable
pub type MUXEN_R = crate::BitReader;
///Field `MUXEN` writer - Multiplexed mode enable
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REQ2ACK_TIME` reader - REQ to ACK time
pub type REQ2ACK_TIME_R = crate::FieldReader;
///Field `REQ2ACK_TIME` writer - REQ to ACK time
pub type REQ2ACK_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Multiplexed mode enable
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:23 - REQ to ACK time
    #[inline(always)]
    pub fn req2ack_time(&self) -> REQ2ACK_TIME_R {
        REQ2ACK_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("req2ack_time", &self.req2ack_time())
            .field("muxen", &self.muxen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Multiplexed mode enable
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<CRrs> {
        MUXEN_W::new(self, 0)
    }
    ///Bits 16:23 - REQ to ACK time
    #[inline(always)]
    pub fn req2ack_time(&mut self) -> REQ2ACK_TIME_W<CRrs> {
        REQ2ACK_TIME_W::new(self, 16)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OCTOSPIM:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
