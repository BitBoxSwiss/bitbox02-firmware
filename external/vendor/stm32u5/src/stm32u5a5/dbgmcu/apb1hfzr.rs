///Register `APB1HFZR` reader
pub type R = crate::R<APB1HFZRrs>;
///Register `APB1HFZR` writer
pub type W = crate::W<APB1HFZRrs>;
///Field `DBG_I2C4_STOP` reader - I2C4 stop in debug
pub type DBG_I2C4_STOP_R = crate::BitReader;
///Field `DBG_I2C4_STOP` writer - I2C4 stop in debug
pub type DBG_I2C4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM2_STOP` reader - LPTIM2 stop in debug
pub type DBG_LPTIM2_STOP_R = crate::BitReader;
///Field `DBG_LPTIM2_STOP` writer - LPTIM2 stop in debug
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - I2C4 stop in debug
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HFZR")
            .field("dbg_i2c4_stop", &self.dbg_i2c4_stop())
            .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
            .finish()
    }
}
impl W {
    ///Bit 1 - I2C4 stop in debug
    #[inline(always)]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W<APB1HFZRrs> {
        DBG_I2C4_STOP_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<APB1HFZRrs> {
        DBG_LPTIM2_STOP_W::new(self, 5)
    }
}
/**Debug MCU APB1H peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1hfzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hfzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DBGMCU:APB1HFZR)*/
pub struct APB1HFZRrs;
impl crate::RegisterSpec for APB1HFZRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1hfzr::R`](R) reader structure
impl crate::Readable for APB1HFZRrs {}
///`write(|w| ..)` method takes [`apb1hfzr::W`](W) writer structure
impl crate::Writable for APB1HFZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HFZR to value 0
impl crate::Resettable for APB1HFZRrs {}
