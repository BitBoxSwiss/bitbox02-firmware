///Register `RCR` reader
pub type R = crate::R<RCRrs>;
///Register `RCR` writer
pub type W = crate::W<RCRrs>;
///Field `REP` reader - Repetition register value
pub type REP_R = crate::FieldReader;
///Field `REP` writer - Repetition register value
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Repetition register value
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR").field("rep", &self.rep()).finish()
    }
}
impl W {
    ///Bits 0:7 - Repetition register value
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<RCRrs> {
        REP_W::new(self, 0)
    }
}
/**LPTIM repetition register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPTIM1:RCR)*/
pub struct RCRrs;
impl crate::RegisterSpec for RCRrs {
    type Ux = u32;
}
///`read()` method returns [`rcr::R`](R) reader structure
impl crate::Readable for RCRrs {}
///`write(|w| ..)` method takes [`rcr::W`](W) writer structure
impl crate::Writable for RCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCR to value 0
impl crate::Resettable for RCRrs {}
