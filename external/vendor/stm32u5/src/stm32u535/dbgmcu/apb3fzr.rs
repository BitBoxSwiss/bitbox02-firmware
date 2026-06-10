///Register `APB3FZR` reader
pub type R = crate::R<APB3FZRrs>;
///Register `APB3FZR` writer
pub type W = crate::W<APB3FZRrs>;
///Field `DBG_I2C3_STOP` reader - I2C3 stop in debug
pub type DBG_I2C3_STOP_R = crate::BitReader;
///Field `DBG_I2C3_STOP` writer - I2C3 stop in debug
pub type DBG_I2C3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in debug
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
///Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in debug
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM3_STOP` reader - LPTIM3 stop in debug
pub type DBG_LPTIM3_STOP_R = crate::BitReader;
///Field `DBG_LPTIM3_STOP` writer - LPTIM3 stop in debug
pub type DBG_LPTIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM4_STOP` reader - LPTIM4 stop in debug
pub type DBG_LPTIM4_STOP_R = crate::BitReader;
///Field `DBG_LPTIM4_STOP` writer - LPTIM4 stop in debug
pub type DBG_LPTIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_RTC_STOP` reader - RTC stop in debug
pub type DBG_RTC_STOP_R = crate::BitReader;
///Field `DBG_RTC_STOP` writer - RTC stop in debug
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 10 - I2C3 stop in debug
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 17 - LPTIM1 stop in debug
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - LPTIM3 stop in debug
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn dbg_lptim4_stop(&self) -> DBG_LPTIM4_STOP_R {
        DBG_LPTIM4_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 30 - RTC stop in debug
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3FZR")
            .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
            .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
            .field("dbg_lptim3_stop", &self.dbg_lptim3_stop())
            .field("dbg_lptim4_stop", &self.dbg_lptim4_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .finish()
    }
}
impl W {
    ///Bit 10 - I2C3 stop in debug
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<APB3FZRrs> {
        DBG_I2C3_STOP_W::new(self, 10)
    }
    ///Bit 17 - LPTIM1 stop in debug
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<APB3FZRrs> {
        DBG_LPTIM1_STOP_W::new(self, 17)
    }
    ///Bit 18 - LPTIM3 stop in debug
    #[inline(always)]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<APB3FZRrs> {
        DBG_LPTIM3_STOP_W::new(self, 18)
    }
    ///Bit 19 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn dbg_lptim4_stop(&mut self) -> DBG_LPTIM4_STOP_W<APB3FZRrs> {
        DBG_LPTIM4_STOP_W::new(self, 19)
    }
    ///Bit 30 - RTC stop in debug
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB3FZRrs> {
        DBG_RTC_STOP_W::new(self, 30)
    }
}
/**Debug MCU APB3 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb3fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DBGMCU:APB3FZR)*/
pub struct APB3FZRrs;
impl crate::RegisterSpec for APB3FZRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3fzr::R`](R) reader structure
impl crate::Readable for APB3FZRrs {}
///`write(|w| ..)` method takes [`apb3fzr::W`](W) writer structure
impl crate::Writable for APB3FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3FZR to value 0
impl crate::Resettable for APB3FZRrs {}
