///Register `AHB1FZR` reader
pub type R = crate::R<AHB1FZRrs>;
///Register `AHB1FZR` writer
pub type W = crate::W<AHB1FZRrs>;
///Field `DBG_GPDMA0_STOP` reader - GPDMA channel 0 stop in debug
pub type DBG_GPDMA0_STOP_R = crate::BitReader;
///Field `DBG_GPDMA0_STOP` writer - GPDMA channel 0 stop in debug
pub type DBG_GPDMA0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA1_STOP` reader - GPDMA channel 1 stop in debug
pub type DBG_GPDMA1_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_STOP` writer - GPDMA channel 1 stop in debug
pub type DBG_GPDMA1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA2_STOP` reader - GPDMA channel 2 stop in debug
pub type DBG_GPDMA2_STOP_R = crate::BitReader;
///Field `DBG_GPDMA2_STOP` writer - GPDMA channel 2 stop in debug
pub type DBG_GPDMA2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA3_STOP` reader - GPDMA channel 3 stop in debug
pub type DBG_GPDMA3_STOP_R = crate::BitReader;
///Field `DBG_GPDMA3_STOP` writer - GPDMA channel 3 stop in debug
pub type DBG_GPDMA3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA4_STOP` reader - GPDMA channel 4 stop in debug
pub type DBG_GPDMA4_STOP_R = crate::BitReader;
///Field `DBG_GPDMA4_STOP` writer - GPDMA channel 4 stop in debug
pub type DBG_GPDMA4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA5_STOP` reader - GPDMA channel 5 stop in debug
pub type DBG_GPDMA5_STOP_R = crate::BitReader;
///Field `DBG_GPDMA5_STOP` writer - GPDMA channel 5 stop in debug
pub type DBG_GPDMA5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA6_STOP` reader - GPDMA channel 6 stop in debug
pub type DBG_GPDMA6_STOP_R = crate::BitReader;
///Field `DBG_GPDMA6_STOP` writer - GPDMA channel 6 stop in debug
pub type DBG_GPDMA6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA7_STOP` reader - GPDMA channel 7 stop in debug
pub type DBG_GPDMA7_STOP_R = crate::BitReader;
///Field `DBG_GPDMA7_STOP` writer - GPDMA channel 7 stop in debug
pub type DBG_GPDMA7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA8_STOP` reader - GPDMA channel 8 stop in debug
pub type DBG_GPDMA8_STOP_R = crate::BitReader;
///Field `DBG_GPDMA8_STOP` writer - GPDMA channel 8 stop in debug
pub type DBG_GPDMA8_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA9_STOP` reader - GPDMA channel 9 stop in debug
pub type DBG_GPDMA9_STOP_R = crate::BitReader;
///Field `DBG_GPDMA9_STOP` writer - GPDMA channel 9 stop in debug
pub type DBG_GPDMA9_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA10_STOP` reader - GPDMA channel 10 stop in debug
pub type DBG_GPDMA10_STOP_R = crate::BitReader;
///Field `DBG_GPDMA10_STOP` writer - GPDMA channel 10 stop in debug
pub type DBG_GPDMA10_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA11_STOP` reader - GPDMA channel 11 stop in debug
pub type DBG_GPDMA11_STOP_R = crate::BitReader;
///Field `DBG_GPDMA11_STOP` writer - GPDMA channel 11 stop in debug
pub type DBG_GPDMA11_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA12_STOP` reader - GPDMA channel 12 stop in debug
pub type DBG_GPDMA12_STOP_R = crate::BitReader;
///Field `DBG_GPDMA12_STOP` writer - GPDMA channel 12 stop in debug
pub type DBG_GPDMA12_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA13_STOP` reader - GPDMA channel 13 stop in debug
pub type DBG_GPDMA13_STOP_R = crate::BitReader;
///Field `DBG_GPDMA13_STOP` writer - GPDMA channel 13 stop in debug
pub type DBG_GPDMA13_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA14_STOP` reader - GPDMA channel 14 stop in debug
pub type DBG_GPDMA14_STOP_R = crate::BitReader;
///Field `DBG_GPDMA14_STOP` writer - GPDMA channel 14 stop in debug
pub type DBG_GPDMA14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA15_STOP` reader - GPDMA channel 15 stop in debug
pub type DBG_GPDMA15_STOP_R = crate::BitReader;
///Field `DBG_GPDMA15_STOP` writer - GPDMA channel 15 stop in debug
pub type DBG_GPDMA15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPDMA channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma0_stop(&self) -> DBG_GPDMA0_STOP_R {
        DBG_GPDMA0_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPDMA channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_stop(&self) -> DBG_GPDMA1_STOP_R {
        DBG_GPDMA1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPDMA channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_stop(&self) -> DBG_GPDMA2_STOP_R {
        DBG_GPDMA2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPDMA channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma3_stop(&self) -> DBG_GPDMA3_STOP_R {
        DBG_GPDMA3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPDMA channel 4 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma4_stop(&self) -> DBG_GPDMA4_STOP_R {
        DBG_GPDMA4_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPDMA channel 5 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma5_stop(&self) -> DBG_GPDMA5_STOP_R {
        DBG_GPDMA5_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPDMA channel 6 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma6_stop(&self) -> DBG_GPDMA6_STOP_R {
        DBG_GPDMA6_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPDMA channel 7 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma7_stop(&self) -> DBG_GPDMA7_STOP_R {
        DBG_GPDMA7_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPDMA channel 8 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma8_stop(&self) -> DBG_GPDMA8_STOP_R {
        DBG_GPDMA8_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPDMA channel 9 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma9_stop(&self) -> DBG_GPDMA9_STOP_R {
        DBG_GPDMA9_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPDMA channel 10 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma10_stop(&self) -> DBG_GPDMA10_STOP_R {
        DBG_GPDMA10_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GPDMA channel 11 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma11_stop(&self) -> DBG_GPDMA11_STOP_R {
        DBG_GPDMA11_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GPDMA channel 12 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma12_stop(&self) -> DBG_GPDMA12_STOP_R {
        DBG_GPDMA12_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GPDMA channel 13 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma13_stop(&self) -> DBG_GPDMA13_STOP_R {
        DBG_GPDMA13_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GPDMA channel 14 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma14_stop(&self) -> DBG_GPDMA14_STOP_R {
        DBG_GPDMA14_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GPDMA channel 15 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma15_stop(&self) -> DBG_GPDMA15_STOP_R {
        DBG_GPDMA15_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1FZR")
            .field("dbg_gpdma0_stop", &self.dbg_gpdma0_stop())
            .field("dbg_gpdma1_stop", &self.dbg_gpdma1_stop())
            .field("dbg_gpdma2_stop", &self.dbg_gpdma2_stop())
            .field("dbg_gpdma3_stop", &self.dbg_gpdma3_stop())
            .field("dbg_gpdma4_stop", &self.dbg_gpdma4_stop())
            .field("dbg_gpdma5_stop", &self.dbg_gpdma5_stop())
            .field("dbg_gpdma6_stop", &self.dbg_gpdma6_stop())
            .field("dbg_gpdma7_stop", &self.dbg_gpdma7_stop())
            .field("dbg_gpdma8_stop", &self.dbg_gpdma8_stop())
            .field("dbg_gpdma9_stop", &self.dbg_gpdma9_stop())
            .field("dbg_gpdma10_stop", &self.dbg_gpdma10_stop())
            .field("dbg_gpdma11_stop", &self.dbg_gpdma11_stop())
            .field("dbg_gpdma12_stop", &self.dbg_gpdma12_stop())
            .field("dbg_gpdma13_stop", &self.dbg_gpdma13_stop())
            .field("dbg_gpdma14_stop", &self.dbg_gpdma14_stop())
            .field("dbg_gpdma15_stop", &self.dbg_gpdma15_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma0_stop(&mut self) -> DBG_GPDMA0_STOP_W<AHB1FZRrs> {
        DBG_GPDMA0_STOP_W::new(self, 0)
    }
    ///Bit 1 - GPDMA channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_stop(&mut self) -> DBG_GPDMA1_STOP_W<AHB1FZRrs> {
        DBG_GPDMA1_STOP_W::new(self, 1)
    }
    ///Bit 2 - GPDMA channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_stop(&mut self) -> DBG_GPDMA2_STOP_W<AHB1FZRrs> {
        DBG_GPDMA2_STOP_W::new(self, 2)
    }
    ///Bit 3 - GPDMA channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma3_stop(&mut self) -> DBG_GPDMA3_STOP_W<AHB1FZRrs> {
        DBG_GPDMA3_STOP_W::new(self, 3)
    }
    ///Bit 4 - GPDMA channel 4 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma4_stop(&mut self) -> DBG_GPDMA4_STOP_W<AHB1FZRrs> {
        DBG_GPDMA4_STOP_W::new(self, 4)
    }
    ///Bit 5 - GPDMA channel 5 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma5_stop(&mut self) -> DBG_GPDMA5_STOP_W<AHB1FZRrs> {
        DBG_GPDMA5_STOP_W::new(self, 5)
    }
    ///Bit 6 - GPDMA channel 6 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma6_stop(&mut self) -> DBG_GPDMA6_STOP_W<AHB1FZRrs> {
        DBG_GPDMA6_STOP_W::new(self, 6)
    }
    ///Bit 7 - GPDMA channel 7 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma7_stop(&mut self) -> DBG_GPDMA7_STOP_W<AHB1FZRrs> {
        DBG_GPDMA7_STOP_W::new(self, 7)
    }
    ///Bit 8 - GPDMA channel 8 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma8_stop(&mut self) -> DBG_GPDMA8_STOP_W<AHB1FZRrs> {
        DBG_GPDMA8_STOP_W::new(self, 8)
    }
    ///Bit 9 - GPDMA channel 9 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma9_stop(&mut self) -> DBG_GPDMA9_STOP_W<AHB1FZRrs> {
        DBG_GPDMA9_STOP_W::new(self, 9)
    }
    ///Bit 10 - GPDMA channel 10 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma10_stop(&mut self) -> DBG_GPDMA10_STOP_W<AHB1FZRrs> {
        DBG_GPDMA10_STOP_W::new(self, 10)
    }
    ///Bit 11 - GPDMA channel 11 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma11_stop(&mut self) -> DBG_GPDMA11_STOP_W<AHB1FZRrs> {
        DBG_GPDMA11_STOP_W::new(self, 11)
    }
    ///Bit 12 - GPDMA channel 12 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma12_stop(&mut self) -> DBG_GPDMA12_STOP_W<AHB1FZRrs> {
        DBG_GPDMA12_STOP_W::new(self, 12)
    }
    ///Bit 13 - GPDMA channel 13 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma13_stop(&mut self) -> DBG_GPDMA13_STOP_W<AHB1FZRrs> {
        DBG_GPDMA13_STOP_W::new(self, 13)
    }
    ///Bit 14 - GPDMA channel 14 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma14_stop(&mut self) -> DBG_GPDMA14_STOP_W<AHB1FZRrs> {
        DBG_GPDMA14_STOP_W::new(self, 14)
    }
    ///Bit 15 - GPDMA channel 15 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma15_stop(&mut self) -> DBG_GPDMA15_STOP_W<AHB1FZRrs> {
        DBG_GPDMA15_STOP_W::new(self, 15)
    }
}
/**Debug MCU AHB1 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`ahb1fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DBGMCU:AHB1FZR)*/
pub struct AHB1FZRrs;
impl crate::RegisterSpec for AHB1FZRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1fzr::R`](R) reader structure
impl crate::Readable for AHB1FZRrs {}
///`write(|w| ..)` method takes [`ahb1fzr::W`](W) writer structure
impl crate::Writable for AHB1FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1FZR to value 0
impl crate::Resettable for AHB1FZRrs {}
