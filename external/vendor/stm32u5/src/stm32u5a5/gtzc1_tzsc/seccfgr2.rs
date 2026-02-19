///Register `SECCFGR2` reader
pub type R = crate::R<SECCFGR2rs>;
///Register `SECCFGR2` writer
pub type W = crate::W<SECCFGR2rs>;
///Field `TIM1SEC` reader - secure access mode for TIM1
pub type TIM1SEC_R = crate::BitReader;
///Field `TIM1SEC` writer - secure access mode for TIM1
pub type TIM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1SEC` reader - secure access mode for SPI1
pub type SPI1SEC_R = crate::BitReader;
///Field `SPI1SEC` writer - secure access mode for SPI1
pub type SPI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8SEC` reader - secure access mode for TIM8
pub type TIM8SEC_R = crate::BitReader;
///Field `TIM8SEC` writer - secure access mode for TIM8
pub type TIM8SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1SEC` reader - secure access mode for USART1
pub type USART1SEC_R = crate::BitReader;
///Field `USART1SEC` writer - secure access mode for USART1
pub type USART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15SEC` reader - secure access mode for TIM5
pub type TIM15SEC_R = crate::BitReader;
///Field `TIM15SEC` writer - secure access mode for TIM5
pub type TIM15SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16SEC` reader - secure access mode for TIM6
pub type TIM16SEC_R = crate::BitReader;
///Field `TIM16SEC` writer - secure access mode for TIM6
pub type TIM16SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17SEC` reader - secure access mode for TIM7
pub type TIM17SEC_R = crate::BitReader;
///Field `TIM17SEC` writer - secure access mode for TIM7
pub type TIM17SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1SEC` reader - secure access mode for SAI1
pub type SAI1SEC_R = crate::BitReader;
///Field `SAI1SEC` writer - secure access mode for SAI1
pub type SAI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2SEC` reader - secure access mode for SAI2
pub type SAI2SEC_R = crate::BitReader;
///Field `SAI2SEC` writer - secure access mode for SAI2
pub type SAI2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - secure access mode for TIM1
    #[inline(always)]
    pub fn tim1sec(&self) -> TIM1SEC_R {
        TIM1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for SPI1
    #[inline(always)]
    pub fn spi1sec(&self) -> SPI1SEC_R {
        SPI1SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure access mode for TIM8
    #[inline(always)]
    pub fn tim8sec(&self) -> TIM8SEC_R {
        TIM8SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure access mode for USART1
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for TIM5
    #[inline(always)]
    pub fn tim15sec(&self) -> TIM15SEC_R {
        TIM15SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure access mode for TIM6
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure access mode for TIM7
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for SAI1
    #[inline(always)]
    pub fn sai1sec(&self) -> SAI1SEC_R {
        SAI1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - secure access mode for SAI2
    #[inline(always)]
    pub fn sai2sec(&self) -> SAI2SEC_R {
        SAI2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR2")
            .field("tim1sec", &self.tim1sec())
            .field("spi1sec", &self.spi1sec())
            .field("tim8sec", &self.tim8sec())
            .field("usart1sec", &self.usart1sec())
            .field("tim15sec", &self.tim15sec())
            .field("tim16sec", &self.tim16sec())
            .field("tim17sec", &self.tim17sec())
            .field("sai1sec", &self.sai1sec())
            .field("sai2sec", &self.sai2sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - secure access mode for TIM1
    #[inline(always)]
    pub fn tim1sec(&mut self) -> TIM1SEC_W<SECCFGR2rs> {
        TIM1SEC_W::new(self, 0)
    }
    ///Bit 1 - secure access mode for SPI1
    #[inline(always)]
    pub fn spi1sec(&mut self) -> SPI1SEC_W<SECCFGR2rs> {
        SPI1SEC_W::new(self, 1)
    }
    ///Bit 2 - secure access mode for TIM8
    #[inline(always)]
    pub fn tim8sec(&mut self) -> TIM8SEC_W<SECCFGR2rs> {
        TIM8SEC_W::new(self, 2)
    }
    ///Bit 3 - secure access mode for USART1
    #[inline(always)]
    pub fn usart1sec(&mut self) -> USART1SEC_W<SECCFGR2rs> {
        USART1SEC_W::new(self, 3)
    }
    ///Bit 4 - secure access mode for TIM5
    #[inline(always)]
    pub fn tim15sec(&mut self) -> TIM15SEC_W<SECCFGR2rs> {
        TIM15SEC_W::new(self, 4)
    }
    ///Bit 5 - secure access mode for TIM6
    #[inline(always)]
    pub fn tim16sec(&mut self) -> TIM16SEC_W<SECCFGR2rs> {
        TIM16SEC_W::new(self, 5)
    }
    ///Bit 6 - secure access mode for TIM7
    #[inline(always)]
    pub fn tim17sec(&mut self) -> TIM17SEC_W<SECCFGR2rs> {
        TIM17SEC_W::new(self, 6)
    }
    ///Bit 7 - secure access mode for SAI1
    #[inline(always)]
    pub fn sai1sec(&mut self) -> SAI1SEC_W<SECCFGR2rs> {
        SAI1SEC_W::new(self, 7)
    }
    ///Bit 8 - secure access mode for SAI2
    #[inline(always)]
    pub fn sai2sec(&mut self) -> SAI2SEC_W<SECCFGR2rs> {
        SAI2SEC_W::new(self, 8)
    }
}
/**TZSC secure configuration register 2

You can [`read`](crate::Reg::read) this register and get [`seccfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#GTZC1_TZSC:SECCFGR2)*/
pub struct SECCFGR2rs;
impl crate::RegisterSpec for SECCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr2::R`](R) reader structure
impl crate::Readable for SECCFGR2rs {}
///`write(|w| ..)` method takes [`seccfgr2::W`](W) writer structure
impl crate::Writable for SECCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR2 to value 0
impl crate::Resettable for SECCFGR2rs {}
