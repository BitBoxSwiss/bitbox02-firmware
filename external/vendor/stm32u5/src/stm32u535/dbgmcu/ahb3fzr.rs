///Register `AHB3FZR` reader
pub type R = crate::R<AHB3FZRrs>;
///Register `AHB3FZR` writer
pub type W = crate::W<AHB3FZRrs>;
///Field `DBG_LPDMA0_STOP` reader - LPDMA channel 0 stop in debug
pub type DBG_LPDMA0_STOP_R = crate::BitReader;
///Field `DBG_LPDMA0_STOP` writer - LPDMA channel 0 stop in debug
pub type DBG_LPDMA0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPDMA1_STOP` reader - LPDMA channel 1 stop in debug
pub type DBG_LPDMA1_STOP_R = crate::BitReader;
///Field `DBG_LPDMA1_STOP` writer - LPDMA channel 1 stop in debug
pub type DBG_LPDMA1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPDMA2_STOP` reader - LPDMA channel 2 stop in debug
pub type DBG_LPDMA2_STOP_R = crate::BitReader;
///Field `DBG_LPDMA2_STOP` writer - LPDMA channel 2 stop in debug
pub type DBG_LPDMA2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPDMA3_STOP` reader - LPDMA channel 3 stop in debug
pub type DBG_LPDMA3_STOP_R = crate::BitReader;
///Field `DBG_LPDMA3_STOP` writer - LPDMA channel 3 stop in debug
pub type DBG_LPDMA3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPDMA channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma0_stop(&self) -> DBG_LPDMA0_STOP_R {
        DBG_LPDMA0_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPDMA channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma1_stop(&self) -> DBG_LPDMA1_STOP_R {
        DBG_LPDMA1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LPDMA channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma2_stop(&self) -> DBG_LPDMA2_STOP_R {
        DBG_LPDMA2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPDMA channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma3_stop(&self) -> DBG_LPDMA3_STOP_R {
        DBG_LPDMA3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3FZR")
            .field("dbg_lpdma0_stop", &self.dbg_lpdma0_stop())
            .field("dbg_lpdma1_stop", &self.dbg_lpdma1_stop())
            .field("dbg_lpdma2_stop", &self.dbg_lpdma2_stop())
            .field("dbg_lpdma3_stop", &self.dbg_lpdma3_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPDMA channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma0_stop(&mut self) -> DBG_LPDMA0_STOP_W<AHB3FZRrs> {
        DBG_LPDMA0_STOP_W::new(self, 0)
    }
    ///Bit 1 - LPDMA channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma1_stop(&mut self) -> DBG_LPDMA1_STOP_W<AHB3FZRrs> {
        DBG_LPDMA1_STOP_W::new(self, 1)
    }
    ///Bit 2 - LPDMA channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma2_stop(&mut self) -> DBG_LPDMA2_STOP_W<AHB3FZRrs> {
        DBG_LPDMA2_STOP_W::new(self, 2)
    }
    ///Bit 3 - LPDMA channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma3_stop(&mut self) -> DBG_LPDMA3_STOP_W<AHB3FZRrs> {
        DBG_LPDMA3_STOP_W::new(self, 3)
    }
}
/**Debug MCU AHB3 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`ahb3fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DBGMCU:AHB3FZR)*/
pub struct AHB3FZRrs;
impl crate::RegisterSpec for AHB3FZRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3fzr::R`](R) reader structure
impl crate::Readable for AHB3FZRrs {}
///`write(|w| ..)` method takes [`ahb3fzr::W`](W) writer structure
impl crate::Writable for AHB3FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3FZR to value 0
impl crate::Resettable for AHB3FZRrs {}
