///Register `IER2` reader
pub type R = crate::R<IER2rs>;
///Register `IER2` writer
pub type W = crate::W<IER2rs>;
///Field `TIM1IE` reader - illegal access interrupt enable for TIM1
pub type TIM1IE_R = crate::BitReader;
///Field `TIM1IE` writer - illegal access interrupt enable for TIM1
pub type TIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1IE` reader - illegal access interrupt enable for SPI1
pub type SPI1IE_R = crate::BitReader;
///Field `SPI1IE` writer - illegal access interrupt enable for SPI1
pub type SPI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8IE` reader - illegal access interrupt enable for TIM8
pub type TIM8IE_R = crate::BitReader;
///Field `TIM8IE` writer - illegal access interrupt enable for TIM8
pub type TIM8IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1IE` reader - illegal access interrupt enable for USART1
pub type USART1IE_R = crate::BitReader;
///Field `USART1IE` writer - illegal access interrupt enable for USART1
pub type USART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15IE` reader - illegal access interrupt enable for TIM5
pub type TIM15IE_R = crate::BitReader;
///Field `TIM15IE` writer - illegal access interrupt enable for TIM5
pub type TIM15IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16IE` reader - illegal access interrupt enable for TIM6
pub type TIM16IE_R = crate::BitReader;
///Field `TIM16IE` writer - illegal access interrupt enable for TIM6
pub type TIM16IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17IE` reader - illegal access interrupt enable for TIM7
pub type TIM17IE_R = crate::BitReader;
///Field `TIM17IE` writer - illegal access interrupt enable for TIM7
pub type TIM17IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1IE` reader - illegal access interrupt enable for SAI1
pub type SAI1IE_R = crate::BitReader;
///Field `SAI1IE` writer - illegal access interrupt enable for SAI1
pub type SAI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for TIM1
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for SPI1
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for TIM8
    #[inline(always)]
    pub fn tim8ie(&self) -> TIM8IE_R {
        TIM8IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for USART1
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for TIM5
    #[inline(always)]
    pub fn tim15ie(&self) -> TIM15IE_R {
        TIM15IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for TIM6
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for TIM7
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for SAI1
    #[inline(always)]
    pub fn sai1ie(&self) -> SAI1IE_R {
        SAI1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER2")
            .field("tim1ie", &self.tim1ie())
            .field("spi1ie", &self.spi1ie())
            .field("tim8ie", &self.tim8ie())
            .field("usart1ie", &self.usart1ie())
            .field("tim15ie", &self.tim15ie())
            .field("tim16ie", &self.tim16ie())
            .field("tim17ie", &self.tim17ie())
            .field("sai1ie", &self.sai1ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for TIM1
    #[inline(always)]
    pub fn tim1ie(&mut self) -> TIM1IE_W<IER2rs> {
        TIM1IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for SPI1
    #[inline(always)]
    pub fn spi1ie(&mut self) -> SPI1IE_W<IER2rs> {
        SPI1IE_W::new(self, 1)
    }
    ///Bit 2 - illegal access interrupt enable for TIM8
    #[inline(always)]
    pub fn tim8ie(&mut self) -> TIM8IE_W<IER2rs> {
        TIM8IE_W::new(self, 2)
    }
    ///Bit 3 - illegal access interrupt enable for USART1
    #[inline(always)]
    pub fn usart1ie(&mut self) -> USART1IE_W<IER2rs> {
        USART1IE_W::new(self, 3)
    }
    ///Bit 4 - illegal access interrupt enable for TIM5
    #[inline(always)]
    pub fn tim15ie(&mut self) -> TIM15IE_W<IER2rs> {
        TIM15IE_W::new(self, 4)
    }
    ///Bit 5 - illegal access interrupt enable for TIM6
    #[inline(always)]
    pub fn tim16ie(&mut self) -> TIM16IE_W<IER2rs> {
        TIM16IE_W::new(self, 5)
    }
    ///Bit 6 - illegal access interrupt enable for TIM7
    #[inline(always)]
    pub fn tim17ie(&mut self) -> TIM17IE_W<IER2rs> {
        TIM17IE_W::new(self, 6)
    }
    ///Bit 7 - illegal access interrupt enable for SAI1
    #[inline(always)]
    pub fn sai1ie(&mut self) -> SAI1IE_W<IER2rs> {
        SAI1IE_W::new(self, 7)
    }
}
/**TZIC interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`ier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#GTZC1_TZIC:IER2)*/
pub struct IER2rs;
impl crate::RegisterSpec for IER2rs {
    type Ux = u32;
}
///`read()` method returns [`ier2::R`](R) reader structure
impl crate::Readable for IER2rs {}
///`write(|w| ..)` method takes [`ier2::W`](W) writer structure
impl crate::Writable for IER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER2 to value 0
impl crate::Resettable for IER2rs {}
