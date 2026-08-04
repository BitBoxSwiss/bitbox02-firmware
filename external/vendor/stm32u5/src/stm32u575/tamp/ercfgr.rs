///Register `ERCFGR` reader
pub type R = crate::R<ERCFGRrs>;
///Register `ERCFGR` writer
pub type W = crate::W<ERCFGRrs>;
///Field `ERCFG0` reader - Configurable device secrets configuration
pub type ERCFG0_R = crate::BitReader;
///Field `ERCFG0` writer - Configurable device secrets configuration
pub type ERCFG0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg0(&self) -> ERCFG0_R {
        ERCFG0_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERCFGR")
            .field("ercfg0", &self.ercfg0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg0(&mut self) -> ERCFG0_W<ERCFGRrs> {
        ERCFG0_W::new(self, 0)
    }
}
/**TAMP erase configuration register

You can [`read`](crate::Reg::read) this register and get [`ercfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ercfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:ERCFGR)*/
pub struct ERCFGRrs;
impl crate::RegisterSpec for ERCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ercfgr::R`](R) reader structure
impl crate::Readable for ERCFGRrs {}
///`write(|w| ..)` method takes [`ercfgr::W`](W) writer structure
impl crate::Writable for ERCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ERCFGR to value 0
impl crate::Resettable for ERCFGRrs {}
