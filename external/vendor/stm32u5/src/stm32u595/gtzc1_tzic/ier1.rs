///Register `IER1` reader
pub type R = crate::R<IER1rs>;
///Register `IER1` writer
pub type W = crate::W<IER1rs>;
///Field `TIM2IE` reader - TIM2IE
pub type TIM2IE_R = crate::BitReader;
///Field `TIM2IE` writer - TIM2IE
pub type TIM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3IE` reader - TIM3IE
pub type TIM3IE_R = crate::BitReader;
///Field `TIM3IE` writer - TIM3IE
pub type TIM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4IE` reader - TIM4IE
pub type TIM4IE_R = crate::BitReader;
///Field `TIM4IE` writer - TIM4IE
pub type TIM4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5IE` reader - TIM5IE
pub type TIM5IE_R = crate::BitReader;
///Field `TIM5IE` writer - TIM5IE
pub type TIM5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6IE` reader - TIM6IE
pub type TIM6IE_R = crate::BitReader;
///Field `TIM6IE` writer - TIM6IE
pub type TIM6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7IE` reader - TIM7IE
pub type TIM7IE_R = crate::BitReader;
///Field `TIM7IE` writer - TIM7IE
pub type TIM7IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGIE` reader - WWDGIE
pub type WWDGIE_R = crate::BitReader;
///Field `WWDGIE` writer - WWDGIE
pub type WWDGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGIE` reader - IWDGIE
pub type IWDGIE_R = crate::BitReader;
///Field `IWDGIE` writer - IWDGIE
pub type IWDGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2IE` reader - SPI2IE
pub type SPI2IE_R = crate::BitReader;
///Field `SPI2IE` writer - SPI2IE
pub type SPI2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2IE` reader - illegal access interrupt enable for USART2
pub type USART2IE_R = crate::BitReader;
///Field `USART2IE` writer - illegal access interrupt enable for USART2
pub type USART2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3IE` reader - illegal access interrupt enable for USART3
pub type USART3IE_R = crate::BitReader;
///Field `USART3IE` writer - illegal access interrupt enable for USART3
pub type USART3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART4IE` reader - illegal access interrupt enable for UART4
pub type USART4IE_R = crate::BitReader;
///Field `USART4IE` writer - illegal access interrupt enable for UART4
pub type USART4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5IE` reader - illegal access interrupt enable for UART5
pub type UART5IE_R = crate::BitReader;
///Field `UART5IE` writer - illegal access interrupt enable for UART5
pub type UART5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1IE` reader - illegal access interrupt enable for I2C1
pub type I2C1IE_R = crate::BitReader;
///Field `I2C1IE` writer - illegal access interrupt enable for I2C1
pub type I2C1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2IE` reader - illegal access interrupt enable for I2C2
pub type I2C2IE_R = crate::BitReader;
///Field `I2C2IE` writer - illegal access interrupt enable for I2C2
pub type I2C2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSIE` reader - illegal access interrupt enable for CRS
pub type CRSIE_R = crate::BitReader;
///Field `CRSIE` writer - illegal access interrupt enable for CRS
pub type CRSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4IE` reader - illegal access interrupt enable for I2C4
pub type I2C4IE_R = crate::BitReader;
///Field `I2C4IE` writer - illegal access interrupt enable for I2C4
pub type I2C4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2IE` reader - illegal access interrupt enable for LPTIM2
pub type LPTIM2IE_R = crate::BitReader;
///Field `LPTIM2IE` writer - illegal access interrupt enable for LPTIM2
pub type LPTIM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCAN1IE` reader - illegal access interrupt enable for FDCAN1
pub type FDCAN1IE_R = crate::BitReader;
///Field `FDCAN1IE` writer - illegal access interrupt enable for FDCAN1
pub type FDCAN1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1IE` reader - illegal access interrupt enable for UCPD1
pub type UCPD1IE_R = crate::BitReader;
///Field `UCPD1IE` writer - illegal access interrupt enable for UCPD1
pub type UCPD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6IE` reader - illegal access interrupt enable for USART6
pub type USART6IE_R = crate::BitReader;
///Field `USART6IE` writer - illegal access interrupt enable for USART6
pub type USART6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C5IE` reader - illegal access interrupt enable for I2C5
pub type I2C5IE_R = crate::BitReader;
///Field `I2C5IE` writer - illegal access interrupt enable for I2C5
pub type I2C5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C6IE` reader - illegal access interrupt enable for I2C6
pub type I2C6IE_R = crate::BitReader;
///Field `I2C6IE` writer - illegal access interrupt enable for I2C6
pub type I2C6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2IE
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3IE
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4IE
    #[inline(always)]
    pub fn tim4ie(&self) -> TIM4IE_R {
        TIM4IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5IE
    #[inline(always)]
    pub fn tim5ie(&self) -> TIM5IE_R {
        TIM5IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6IE
    #[inline(always)]
    pub fn tim6ie(&self) -> TIM6IE_R {
        TIM6IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7IE
    #[inline(always)]
    pub fn tim7ie(&self) -> TIM7IE_R {
        TIM7IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WWDGIE
    #[inline(always)]
    pub fn wwdgie(&self) -> WWDGIE_R {
        WWDGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IWDGIE
    #[inline(always)]
    pub fn iwdgie(&self) -> IWDGIE_R {
        IWDGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPI2IE
    #[inline(always)]
    pub fn spi2ie(&self) -> SPI2IE_R {
        SPI2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for USART2
    #[inline(always)]
    pub fn usart2ie(&self) -> USART2IE_R {
        USART2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for USART3
    #[inline(always)]
    pub fn usart3ie(&self) -> USART3IE_R {
        USART3IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for UART4
    #[inline(always)]
    pub fn usart4ie(&self) -> USART4IE_R {
        USART4IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for UART5
    #[inline(always)]
    pub fn uart5ie(&self) -> UART5IE_R {
        UART5IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for I2C1
    #[inline(always)]
    pub fn i2c1ie(&self) -> I2C1IE_R {
        I2C1IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for I2C2
    #[inline(always)]
    pub fn i2c2ie(&self) -> I2C2IE_R {
        I2C2IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for CRS
    #[inline(always)]
    pub fn crsie(&self) -> CRSIE_R {
        CRSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for I2C4
    #[inline(always)]
    pub fn i2c4ie(&self) -> I2C4IE_R {
        I2C4IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for LPTIM2
    #[inline(always)]
    pub fn lptim2ie(&self) -> LPTIM2IE_R {
        LPTIM2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for FDCAN1
    #[inline(always)]
    pub fn fdcan1ie(&self) -> FDCAN1IE_R {
        FDCAN1IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for UCPD1
    #[inline(always)]
    pub fn ucpd1ie(&self) -> UCPD1IE_R {
        UCPD1IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for USART6
    #[inline(always)]
    pub fn usart6ie(&self) -> USART6IE_R {
        USART6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for I2C5
    #[inline(always)]
    pub fn i2c5ie(&self) -> I2C5IE_R {
        I2C5IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for I2C6
    #[inline(always)]
    pub fn i2c6ie(&self) -> I2C6IE_R {
        I2C6IE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER1")
            .field("tim2ie", &self.tim2ie())
            .field("tim3ie", &self.tim3ie())
            .field("tim4ie", &self.tim4ie())
            .field("tim5ie", &self.tim5ie())
            .field("tim6ie", &self.tim6ie())
            .field("tim7ie", &self.tim7ie())
            .field("wwdgie", &self.wwdgie())
            .field("iwdgie", &self.iwdgie())
            .field("spi2ie", &self.spi2ie())
            .field("usart2ie", &self.usart2ie())
            .field("usart3ie", &self.usart3ie())
            .field("usart4ie", &self.usart4ie())
            .field("uart5ie", &self.uart5ie())
            .field("i2c1ie", &self.i2c1ie())
            .field("i2c2ie", &self.i2c2ie())
            .field("crsie", &self.crsie())
            .field("i2c4ie", &self.i2c4ie())
            .field("lptim2ie", &self.lptim2ie())
            .field("fdcan1ie", &self.fdcan1ie())
            .field("ucpd1ie", &self.ucpd1ie())
            .field("usart6ie", &self.usart6ie())
            .field("i2c5ie", &self.i2c5ie())
            .field("i2c6ie", &self.i2c6ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2IE
    #[inline(always)]
    pub fn tim2ie(&mut self) -> TIM2IE_W<IER1rs> {
        TIM2IE_W::new(self, 0)
    }
    ///Bit 1 - TIM3IE
    #[inline(always)]
    pub fn tim3ie(&mut self) -> TIM3IE_W<IER1rs> {
        TIM3IE_W::new(self, 1)
    }
    ///Bit 2 - TIM4IE
    #[inline(always)]
    pub fn tim4ie(&mut self) -> TIM4IE_W<IER1rs> {
        TIM4IE_W::new(self, 2)
    }
    ///Bit 3 - TIM5IE
    #[inline(always)]
    pub fn tim5ie(&mut self) -> TIM5IE_W<IER1rs> {
        TIM5IE_W::new(self, 3)
    }
    ///Bit 4 - TIM6IE
    #[inline(always)]
    pub fn tim6ie(&mut self) -> TIM6IE_W<IER1rs> {
        TIM6IE_W::new(self, 4)
    }
    ///Bit 5 - TIM7IE
    #[inline(always)]
    pub fn tim7ie(&mut self) -> TIM7IE_W<IER1rs> {
        TIM7IE_W::new(self, 5)
    }
    ///Bit 6 - WWDGIE
    #[inline(always)]
    pub fn wwdgie(&mut self) -> WWDGIE_W<IER1rs> {
        WWDGIE_W::new(self, 6)
    }
    ///Bit 7 - IWDGIE
    #[inline(always)]
    pub fn iwdgie(&mut self) -> IWDGIE_W<IER1rs> {
        IWDGIE_W::new(self, 7)
    }
    ///Bit 8 - SPI2IE
    #[inline(always)]
    pub fn spi2ie(&mut self) -> SPI2IE_W<IER1rs> {
        SPI2IE_W::new(self, 8)
    }
    ///Bit 9 - illegal access interrupt enable for USART2
    #[inline(always)]
    pub fn usart2ie(&mut self) -> USART2IE_W<IER1rs> {
        USART2IE_W::new(self, 9)
    }
    ///Bit 10 - illegal access interrupt enable for USART3
    #[inline(always)]
    pub fn usart3ie(&mut self) -> USART3IE_W<IER1rs> {
        USART3IE_W::new(self, 10)
    }
    ///Bit 11 - illegal access interrupt enable for UART4
    #[inline(always)]
    pub fn usart4ie(&mut self) -> USART4IE_W<IER1rs> {
        USART4IE_W::new(self, 11)
    }
    ///Bit 12 - illegal access interrupt enable for UART5
    #[inline(always)]
    pub fn uart5ie(&mut self) -> UART5IE_W<IER1rs> {
        UART5IE_W::new(self, 12)
    }
    ///Bit 13 - illegal access interrupt enable for I2C1
    #[inline(always)]
    pub fn i2c1ie(&mut self) -> I2C1IE_W<IER1rs> {
        I2C1IE_W::new(self, 13)
    }
    ///Bit 14 - illegal access interrupt enable for I2C2
    #[inline(always)]
    pub fn i2c2ie(&mut self) -> I2C2IE_W<IER1rs> {
        I2C2IE_W::new(self, 14)
    }
    ///Bit 15 - illegal access interrupt enable for CRS
    #[inline(always)]
    pub fn crsie(&mut self) -> CRSIE_W<IER1rs> {
        CRSIE_W::new(self, 15)
    }
    ///Bit 16 - illegal access interrupt enable for I2C4
    #[inline(always)]
    pub fn i2c4ie(&mut self) -> I2C4IE_W<IER1rs> {
        I2C4IE_W::new(self, 16)
    }
    ///Bit 17 - illegal access interrupt enable for LPTIM2
    #[inline(always)]
    pub fn lptim2ie(&mut self) -> LPTIM2IE_W<IER1rs> {
        LPTIM2IE_W::new(self, 17)
    }
    ///Bit 18 - illegal access interrupt enable for FDCAN1
    #[inline(always)]
    pub fn fdcan1ie(&mut self) -> FDCAN1IE_W<IER1rs> {
        FDCAN1IE_W::new(self, 18)
    }
    ///Bit 19 - illegal access interrupt enable for UCPD1
    #[inline(always)]
    pub fn ucpd1ie(&mut self) -> UCPD1IE_W<IER1rs> {
        UCPD1IE_W::new(self, 19)
    }
    ///Bit 21 - illegal access interrupt enable for USART6
    #[inline(always)]
    pub fn usart6ie(&mut self) -> USART6IE_W<IER1rs> {
        USART6IE_W::new(self, 21)
    }
    ///Bit 22 - illegal access interrupt enable for I2C5
    #[inline(always)]
    pub fn i2c5ie(&mut self) -> I2C5IE_W<IER1rs> {
        I2C5IE_W::new(self, 22)
    }
    ///Bit 23 - illegal access interrupt enable for I2C6
    #[inline(always)]
    pub fn i2c6ie(&mut self) -> I2C6IE_W<IER1rs> {
        I2C6IE_W::new(self, 23)
    }
}
/**TZIC interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GTZC1_TZIC:IER1)*/
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
///`read()` method returns [`ier1::R`](R) reader structure
impl crate::Readable for IER1rs {}
///`write(|w| ..)` method takes [`ier1::W`](W) writer structure
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1rs {}
