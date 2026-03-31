///Register `RWD` reader
pub type R = crate::R<RWDrs>;
///Register `RWD` writer
pub type W = crate::W<RWDrs>;
///Field `WDC` reader - Watchdog configuration
pub type WDC_R = crate::FieldReader;
///Field `WDC` writer - Watchdog configuration
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WDV` reader - Watchdog value
pub type WDV_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Watchdog configuration
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Watchdog value
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWD")
            .field("wdv", &self.wdv())
            .field("wdc", &self.wdc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Watchdog configuration
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W<RWDrs> {
        WDC_W::new(self, 0)
    }
}
/**FDCAN RAM Watchdog Register

You can [`read`](crate::Reg::read) this register and get [`rwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#FDCAN1_RAM:RWD)*/
pub struct RWDrs;
impl crate::RegisterSpec for RWDrs {
    type Ux = u32;
}
///`read()` method returns [`rwd::R`](R) reader structure
impl crate::Readable for RWDrs {}
///`write(|w| ..)` method takes [`rwd::W`](W) writer structure
impl crate::Writable for RWDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RWD to value 0
impl crate::Resettable for RWDrs {}
