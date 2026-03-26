///Register `WINR` reader
pub type R = crate::R<WINRrs>;
///Register `WINR` writer
pub type W = crate::W<WINRrs>;
///Field `WIN` reader - Watchdog counter window value
pub type WIN_R = crate::FieldReader<u16>;
///Field `WIN` writer - Watchdog counter window value
pub type WIN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Watchdog counter window value
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(self.bits & 0x0fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WINR").field("win", &self.win()).finish()
    }
}
impl W {
    ///Bits 0:11 - Watchdog counter window value
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W<WINRrs> {
        WIN_W::new(self, 0)
    }
}
/**Window register

You can [`read`](crate::Reg::read) this register and get [`winr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#IWDG:WINR)*/
pub struct WINRrs;
impl crate::RegisterSpec for WINRrs {
    type Ux = u16;
}
///`read()` method returns [`winr::R`](R) reader structure
impl crate::Readable for WINRrs {}
///`write(|w| ..)` method takes [`winr::W`](W) writer structure
impl crate::Writable for WINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WINR to value 0x0fff
impl crate::Resettable for WINRrs {
    const RESET_VALUE: u16 = 0x0fff;
}
