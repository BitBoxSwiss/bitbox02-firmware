///Register `ACKTIMER` reader
pub type R = crate::R<ACKTIMERrs>;
///Register `ACKTIMER` writer
pub type W = crate::W<ACKTIMERrs>;
///Field `ACKTIME` reader - Boot acknowledgment timeout period
pub type ACKTIME_R = crate::FieldReader<u32>;
///Field `ACKTIME` writer - Boot acknowledgment timeout period
pub type ACKTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - Boot acknowledgment timeout period
    #[inline(always)]
    pub fn acktime(&self) -> ACKTIME_R {
        ACKTIME_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACKTIMER")
            .field("acktime", &self.acktime())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - Boot acknowledgment timeout period
    #[inline(always)]
    pub fn acktime(&mut self) -> ACKTIME_W<ACKTIMERrs> {
        ACKTIME_W::new(self, 0)
    }
}
/**acknowledgment timer register

You can [`read`](crate::Reg::read) this register and get [`acktimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acktimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SDMMC1:ACKTIMER)*/
pub struct ACKTIMERrs;
impl crate::RegisterSpec for ACKTIMERrs {
    type Ux = u32;
}
///`read()` method returns [`acktimer::R`](R) reader structure
impl crate::Readable for ACKTIMERrs {}
///`write(|w| ..)` method takes [`acktimer::W`](W) writer structure
impl crate::Writable for ACKTIMERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACKTIMER to value 0
impl crate::Resettable for ACKTIMERrs {}
