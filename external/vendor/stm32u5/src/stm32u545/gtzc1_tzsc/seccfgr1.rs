///Register `SECCFGR1` reader
pub type R = crate::R<SECCFGR1rs>;
///Register `SECCFGR1` writer
pub type W = crate::W<SECCFGR1rs>;
///Field `TIM2SEC` reader - secure access mode for TIM2
pub type TIM2SEC_R = crate::BitReader;
///Field `TIM2SEC` writer - secure access mode for TIM2
pub type TIM2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3SEC` reader - secure access mode for TIM3
pub type TIM3SEC_R = crate::BitReader;
///Field `TIM3SEC` writer - secure access mode for TIM3
pub type TIM3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4SEC` reader - secure access mode for TIM4
pub type TIM4SEC_R = crate::BitReader;
///Field `TIM4SEC` writer - secure access mode for TIM4
pub type TIM4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5SEC` reader - secure access mode for TIM5
pub type TIM5SEC_R = crate::BitReader;
///Field `TIM5SEC` writer - secure access mode for TIM5
pub type TIM5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6SEC` reader - secure access mode for TIM6
pub type TIM6SEC_R = crate::BitReader;
///Field `TIM6SEC` writer - secure access mode for TIM6
pub type TIM6SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7SEC` reader - secure access mode for TIM7
pub type TIM7SEC_R = crate::BitReader;
///Field `TIM7SEC` writer - secure access mode for TIM7
pub type TIM7SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGSEC` reader - secure access mode for WWDG
pub type WWDGSEC_R = crate::BitReader;
///Field `WWDGSEC` writer - secure access mode for WWDG
pub type WWDGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGSEC` reader - secure access mode for IWDG
pub type IWDGSEC_R = crate::BitReader;
///Field `IWDGSEC` writer - secure access mode for IWDG
pub type IWDGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2SEC` reader - secure access mode for SPI2
pub type SPI2SEC_R = crate::BitReader;
///Field `SPI2SEC` writer - secure access mode for SPI2
pub type SPI2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3SEC` reader - secure access mode for USART3
pub type USART3SEC_R = crate::BitReader;
///Field `USART3SEC` writer - secure access mode for USART3
pub type USART3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4SEC` reader - secure access mode for UART4
pub type UART4SEC_R = crate::BitReader;
///Field `UART4SEC` writer - secure access mode for UART4
pub type UART4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5SEC` reader - secure access mode for UART5
pub type UART5SEC_R = crate::BitReader;
///Field `UART5SEC` writer - secure access mode for UART5
pub type UART5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1SEC` reader - secure access mode for I2C1
pub type I2C1SEC_R = crate::BitReader;
///Field `I2C1SEC` writer - secure access mode for I2C1
pub type I2C1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2SEC` reader - secure access mode for I2C2
pub type I2C2SEC_R = crate::BitReader;
///Field `I2C2SEC` writer - secure access mode for I2C2
pub type I2C2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSSEC` reader - secure access mode for CRS
pub type CRSSEC_R = crate::BitReader;
///Field `CRSSEC` writer - secure access mode for CRS
pub type CRSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4SEC` reader - secure access mode for I2C4
pub type I2C4SEC_R = crate::BitReader;
///Field `I2C4SEC` writer - secure access mode for I2C4
pub type I2C4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2SEC` reader - secure access mode for LPTIM2
pub type LPTIM2SEC_R = crate::BitReader;
///Field `LPTIM2SEC` writer - secure access mode for LPTIM2
pub type LPTIM2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCAN1SEC` reader - secure access mode for FDCAN1
pub type FDCAN1SEC_R = crate::BitReader;
///Field `FDCAN1SEC` writer - secure access mode for FDCAN1
pub type FDCAN1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - secure access mode for TIM2
    #[inline(always)]
    pub fn tim2sec(&self) -> TIM2SEC_R {
        TIM2SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for TIM3
    #[inline(always)]
    pub fn tim3sec(&self) -> TIM3SEC_R {
        TIM3SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure access mode for TIM4
    #[inline(always)]
    pub fn tim4sec(&self) -> TIM4SEC_R {
        TIM4SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure access mode for TIM5
    #[inline(always)]
    pub fn tim5sec(&self) -> TIM5SEC_R {
        TIM5SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for TIM6
    #[inline(always)]
    pub fn tim6sec(&self) -> TIM6SEC_R {
        TIM6SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure access mode for TIM7
    #[inline(always)]
    pub fn tim7sec(&self) -> TIM7SEC_R {
        TIM7SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure access mode for WWDG
    #[inline(always)]
    pub fn wwdgsec(&self) -> WWDGSEC_R {
        WWDGSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for IWDG
    #[inline(always)]
    pub fn iwdgsec(&self) -> IWDGSEC_R {
        IWDGSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - secure access mode for SPI2
    #[inline(always)]
    pub fn spi2sec(&self) -> SPI2SEC_R {
        SPI2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - secure access mode for USART3
    #[inline(always)]
    pub fn usart3sec(&self) -> USART3SEC_R {
        USART3SEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - secure access mode for UART4
    #[inline(always)]
    pub fn uart4sec(&self) -> UART4SEC_R {
        UART4SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - secure access mode for UART5
    #[inline(always)]
    pub fn uart5sec(&self) -> UART5SEC_R {
        UART5SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - secure access mode for I2C1
    #[inline(always)]
    pub fn i2c1sec(&self) -> I2C1SEC_R {
        I2C1SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - secure access mode for I2C2
    #[inline(always)]
    pub fn i2c2sec(&self) -> I2C2SEC_R {
        I2C2SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - secure access mode for CRS
    #[inline(always)]
    pub fn crssec(&self) -> CRSSEC_R {
        CRSSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - secure access mode for I2C4
    #[inline(always)]
    pub fn i2c4sec(&self) -> I2C4SEC_R {
        I2C4SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - secure access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2sec(&self) -> LPTIM2SEC_R {
        LPTIM2SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - secure access mode for FDCAN1
    #[inline(always)]
    pub fn fdcan1sec(&self) -> FDCAN1SEC_R {
        FDCAN1SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR1")
            .field("tim2sec", &self.tim2sec())
            .field("tim3sec", &self.tim3sec())
            .field("tim4sec", &self.tim4sec())
            .field("tim5sec", &self.tim5sec())
            .field("tim6sec", &self.tim6sec())
            .field("tim7sec", &self.tim7sec())
            .field("wwdgsec", &self.wwdgsec())
            .field("iwdgsec", &self.iwdgsec())
            .field("spi2sec", &self.spi2sec())
            .field("usart3sec", &self.usart3sec())
            .field("uart4sec", &self.uart4sec())
            .field("uart5sec", &self.uart5sec())
            .field("i2c1sec", &self.i2c1sec())
            .field("i2c2sec", &self.i2c2sec())
            .field("crssec", &self.crssec())
            .field("i2c4sec", &self.i2c4sec())
            .field("lptim2sec", &self.lptim2sec())
            .field("fdcan1sec", &self.fdcan1sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - secure access mode for TIM2
    #[inline(always)]
    pub fn tim2sec(&mut self) -> TIM2SEC_W<SECCFGR1rs> {
        TIM2SEC_W::new(self, 0)
    }
    ///Bit 1 - secure access mode for TIM3
    #[inline(always)]
    pub fn tim3sec(&mut self) -> TIM3SEC_W<SECCFGR1rs> {
        TIM3SEC_W::new(self, 1)
    }
    ///Bit 2 - secure access mode for TIM4
    #[inline(always)]
    pub fn tim4sec(&mut self) -> TIM4SEC_W<SECCFGR1rs> {
        TIM4SEC_W::new(self, 2)
    }
    ///Bit 3 - secure access mode for TIM5
    #[inline(always)]
    pub fn tim5sec(&mut self) -> TIM5SEC_W<SECCFGR1rs> {
        TIM5SEC_W::new(self, 3)
    }
    ///Bit 4 - secure access mode for TIM6
    #[inline(always)]
    pub fn tim6sec(&mut self) -> TIM6SEC_W<SECCFGR1rs> {
        TIM6SEC_W::new(self, 4)
    }
    ///Bit 5 - secure access mode for TIM7
    #[inline(always)]
    pub fn tim7sec(&mut self) -> TIM7SEC_W<SECCFGR1rs> {
        TIM7SEC_W::new(self, 5)
    }
    ///Bit 6 - secure access mode for WWDG
    #[inline(always)]
    pub fn wwdgsec(&mut self) -> WWDGSEC_W<SECCFGR1rs> {
        WWDGSEC_W::new(self, 6)
    }
    ///Bit 7 - secure access mode for IWDG
    #[inline(always)]
    pub fn iwdgsec(&mut self) -> IWDGSEC_W<SECCFGR1rs> {
        IWDGSEC_W::new(self, 7)
    }
    ///Bit 8 - secure access mode for SPI2
    #[inline(always)]
    pub fn spi2sec(&mut self) -> SPI2SEC_W<SECCFGR1rs> {
        SPI2SEC_W::new(self, 8)
    }
    ///Bit 10 - secure access mode for USART3
    #[inline(always)]
    pub fn usart3sec(&mut self) -> USART3SEC_W<SECCFGR1rs> {
        USART3SEC_W::new(self, 10)
    }
    ///Bit 11 - secure access mode for UART4
    #[inline(always)]
    pub fn uart4sec(&mut self) -> UART4SEC_W<SECCFGR1rs> {
        UART4SEC_W::new(self, 11)
    }
    ///Bit 12 - secure access mode for UART5
    #[inline(always)]
    pub fn uart5sec(&mut self) -> UART5SEC_W<SECCFGR1rs> {
        UART5SEC_W::new(self, 12)
    }
    ///Bit 13 - secure access mode for I2C1
    #[inline(always)]
    pub fn i2c1sec(&mut self) -> I2C1SEC_W<SECCFGR1rs> {
        I2C1SEC_W::new(self, 13)
    }
    ///Bit 14 - secure access mode for I2C2
    #[inline(always)]
    pub fn i2c2sec(&mut self) -> I2C2SEC_W<SECCFGR1rs> {
        I2C2SEC_W::new(self, 14)
    }
    ///Bit 15 - secure access mode for CRS
    #[inline(always)]
    pub fn crssec(&mut self) -> CRSSEC_W<SECCFGR1rs> {
        CRSSEC_W::new(self, 15)
    }
    ///Bit 16 - secure access mode for I2C4
    #[inline(always)]
    pub fn i2c4sec(&mut self) -> I2C4SEC_W<SECCFGR1rs> {
        I2C4SEC_W::new(self, 16)
    }
    ///Bit 17 - secure access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2sec(&mut self) -> LPTIM2SEC_W<SECCFGR1rs> {
        LPTIM2SEC_W::new(self, 17)
    }
    ///Bit 18 - secure access mode for FDCAN1
    #[inline(always)]
    pub fn fdcan1sec(&mut self) -> FDCAN1SEC_W<SECCFGR1rs> {
        FDCAN1SEC_W::new(self, 18)
    }
}
/**TZSC secure configuration register 1

You can [`read`](crate::Reg::read) this register and get [`seccfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#GTZC1_TZSC:SECCFGR1)*/
pub struct SECCFGR1rs;
impl crate::RegisterSpec for SECCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr1::R`](R) reader structure
impl crate::Readable for SECCFGR1rs {}
///`write(|w| ..)` method takes [`seccfgr1::W`](W) writer structure
impl crate::Writable for SECCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR1 to value 0
impl crate::Resettable for SECCFGR1rs {}
