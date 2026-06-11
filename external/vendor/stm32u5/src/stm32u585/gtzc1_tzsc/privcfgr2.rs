///Register `PRIVCFGR2` reader
pub type R = crate::R<PRIVCFGR2rs>;
///Register `PRIVCFGR2` writer
pub type W = crate::W<PRIVCFGR2rs>;
///Field `TIM1PRIV` reader - privileged access mode for TIM1
pub type TIM1PRIV_R = crate::BitReader;
///Field `TIM1PRIV` writer - privileged access mode for TIM1
pub type TIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1PRIV` reader - privileged access mode for SPI1PRIV
pub type SPI1PRIV_R = crate::BitReader;
///Field `SPI1PRIV` writer - privileged access mode for SPI1PRIV
pub type SPI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8PRIV` reader - privileged access mode for TIM8
pub type TIM8PRIV_R = crate::BitReader;
///Field `TIM8PRIV` writer - privileged access mode for TIM8
pub type TIM8PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1PRIV` reader - privileged access mode for USART1
pub type USART1PRIV_R = crate::BitReader;
///Field `USART1PRIV` writer - privileged access mode for USART1
pub type USART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15PRIV` reader - privileged access mode for TIM15
pub type TIM15PRIV_R = crate::BitReader;
///Field `TIM15PRIV` writer - privileged access mode for TIM15
pub type TIM15PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16PRIV` reader - privileged access mode for TIM16
pub type TIM16PRIV_R = crate::BitReader;
///Field `TIM16PRIV` writer - privileged access mode for TIM16
pub type TIM16PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17PRIV` reader - privileged access mode for TIM17
pub type TIM17PRIV_R = crate::BitReader;
///Field `TIM17PRIV` writer - privileged access mode for TIM17
pub type TIM17PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1PRIV` reader - privileged access mode for SAI1
pub type SAI1PRIV_R = crate::BitReader;
///Field `SAI1PRIV` writer - privileged access mode for SAI1
pub type SAI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2PRIV` reader - privileged access mode for SAI2
pub type SAI2PRIV_R = crate::BitReader;
///Field `SAI2PRIV` writer - privileged access mode for SAI2
pub type SAI2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - privileged access mode for TIM1
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for SPI1PRIV
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged access mode for TIM8
    #[inline(always)]
    pub fn tim8priv(&self) -> TIM8PRIV_R {
        TIM8PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged access mode for USART1
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for TIM15
    #[inline(always)]
    pub fn tim15priv(&self) -> TIM15PRIV_R {
        TIM15PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for TIM16
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for TIM17
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged access mode for SAI1
    #[inline(always)]
    pub fn sai1priv(&self) -> SAI1PRIV_R {
        SAI1PRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for SAI2
    #[inline(always)]
    pub fn sai2priv(&self) -> SAI2PRIV_R {
        SAI2PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR2")
            .field("tim1priv", &self.tim1priv())
            .field("spi1priv", &self.spi1priv())
            .field("tim8priv", &self.tim8priv())
            .field("usart1priv", &self.usart1priv())
            .field("tim15priv", &self.tim15priv())
            .field("tim16priv", &self.tim16priv())
            .field("tim17priv", &self.tim17priv())
            .field("sai1priv", &self.sai1priv())
            .field("sai2priv", &self.sai2priv())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged access mode for TIM1
    #[inline(always)]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W<PRIVCFGR2rs> {
        TIM1PRIV_W::new(self, 0)
    }
    ///Bit 1 - privileged access mode for SPI1PRIV
    #[inline(always)]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W<PRIVCFGR2rs> {
        SPI1PRIV_W::new(self, 1)
    }
    ///Bit 2 - privileged access mode for TIM8
    #[inline(always)]
    pub fn tim8priv(&mut self) -> TIM8PRIV_W<PRIVCFGR2rs> {
        TIM8PRIV_W::new(self, 2)
    }
    ///Bit 3 - privileged access mode for USART1
    #[inline(always)]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<PRIVCFGR2rs> {
        USART1PRIV_W::new(self, 3)
    }
    ///Bit 4 - privileged access mode for TIM15
    #[inline(always)]
    pub fn tim15priv(&mut self) -> TIM15PRIV_W<PRIVCFGR2rs> {
        TIM15PRIV_W::new(self, 4)
    }
    ///Bit 5 - privileged access mode for TIM16
    #[inline(always)]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W<PRIVCFGR2rs> {
        TIM16PRIV_W::new(self, 5)
    }
    ///Bit 6 - privileged access mode for TIM17
    #[inline(always)]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W<PRIVCFGR2rs> {
        TIM17PRIV_W::new(self, 6)
    }
    ///Bit 7 - privileged access mode for SAI1
    #[inline(always)]
    pub fn sai1priv(&mut self) -> SAI1PRIV_W<PRIVCFGR2rs> {
        SAI1PRIV_W::new(self, 7)
    }
    ///Bit 8 - privileged access mode for SAI2
    #[inline(always)]
    pub fn sai2priv(&mut self) -> SAI2PRIV_W<PRIVCFGR2rs> {
        SAI2PRIV_W::new(self, 8)
    }
}
/**TZSC privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC1_TZSC:PRIVCFGR2)*/
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr2::R`](R) reader structure
impl crate::Readable for PRIVCFGR2rs {}
///`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR2 to value 0
impl crate::Resettable for PRIVCFGR2rs {}
