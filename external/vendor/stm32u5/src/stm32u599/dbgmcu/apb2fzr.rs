///Register `APB2FZR` reader
pub type R = crate::R<APB2FZRrs>;
///Register `APB2FZR` writer
pub type W = crate::W<APB2FZRrs>;
///Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM8_STOP` reader - TIM8 stop in debug
pub type DBG_TIM8_STOP_R = crate::BitReader;
///Field `DBG_TIM8_STOP` writer - TIM8 stop in debug
pub type DBG_TIM8_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM15_STOP` reader - TIM15 counter stopped when core is halted
pub type DBG_TIM15_STOP_R = crate::BitReader;
///Field `DBG_TIM15_STOP` writer - TIM15 counter stopped when core is halted
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - TIM16 counter stopped when core is halted
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - TIM16 counter stopped when core is halted
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM17_STOP` reader - DBG_TIM17_STOP
pub type DBG_TIM17_STOP_R = crate::BitReader;
///Field `DBG_TIM17_STOP` writer - DBG_TIM17_STOP
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - TIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - TIM8 stop in debug
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - TIM15 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DBG_TIM17_STOP
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2FZR")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim8_stop", &self.dbg_tim8_stop())
            .field("dbg_tim15_stop", &self.dbg_tim15_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<APB2FZRrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    ///Bit 13 - TIM8 stop in debug
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W<APB2FZRrs> {
        DBG_TIM8_STOP_W::new(self, 13)
    }
    ///Bit 16 - TIM15 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<APB2FZRrs> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    ///Bit 17 - TIM16 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB2FZRrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    ///Bit 18 - DBG_TIM17_STOP
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<APB2FZRrs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
/**Debug MCU APB2 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DBGMCU:APB2FZR)*/
pub struct APB2FZRrs;
impl crate::RegisterSpec for APB2FZRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2fzr::R`](R) reader structure
impl crate::Readable for APB2FZRrs {}
///`write(|w| ..)` method takes [`apb2fzr::W`](W) writer structure
impl crate::Writable for APB2FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2FZR to value 0
impl crate::Resettable for APB2FZRrs {}
