///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Field `TIM2F` reader - illegal access flag for TIM2
pub type TIM2F_R = crate::BitReader;
///Field `TIM3F` reader - illegal access flag for TIM3
pub type TIM3F_R = crate::BitReader;
///Field `TIM4F` reader - illegal access flag for TIM4
pub type TIM4F_R = crate::BitReader;
///Field `TIM5F` reader - illegal access flag for TIM5
pub type TIM5F_R = crate::BitReader;
///Field `TIM6F` reader - illegal access flag for TIM6
pub type TIM6F_R = crate::BitReader;
///Field `TIM7F` reader - illegal access flag for TIM7
pub type TIM7F_R = crate::BitReader;
///Field `WWDGF` reader - illegal access flag for WWDG
pub type WWDGF_R = crate::BitReader;
///Field `IWDGF` reader - illegal access flag for IWDG
pub type IWDGF_R = crate::BitReader;
///Field `SPI2F` reader - illegal access flag for SPI2
pub type SPI2F_R = crate::BitReader;
///Field `USART2F` reader - illegal access flag for USART2
pub type USART2F_R = crate::BitReader;
///Field `USART3F` reader - illegal access flag for USART3
pub type USART3F_R = crate::BitReader;
///Field `UART4F` reader - illegal access flag for UART4
pub type UART4F_R = crate::BitReader;
///Field `UART5F` reader - illegal access flag for UART5
pub type UART5F_R = crate::BitReader;
///Field `I2C1F` reader - illegal access flag for I2C1
pub type I2C1F_R = crate::BitReader;
///Field `I2C2F` reader - illegal access flag for I2C2
pub type I2C2F_R = crate::BitReader;
///Field `CRSF` reader - illegal access flag for CRS
pub type CRSF_R = crate::BitReader;
///Field `I2C4F` reader - illegal access flag for I2C4
pub type I2C4F_R = crate::BitReader;
///Field `LPTIM2F` reader - illegal access flag for LPTIM2
pub type LPTIM2F_R = crate::BitReader;
///Field `FDCAN1F` reader - illegal access flag for FDCAN1
pub type FDCAN1F_R = crate::BitReader;
///Field `UCPD1F` reader - illegal access flag for UCPD1
pub type UCPD1F_R = crate::BitReader;
///Field `USART6F` reader - illegal access flag for USART6
pub type USART6F_R = crate::BitReader;
///Field `I2C5F` reader - illegal access flag for I2C5
pub type I2C5F_R = crate::BitReader;
///Field `I2C6F` reader - illegal access flag for I2C6
pub type I2C6F_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for TIM2
    #[inline(always)]
    pub fn tim2f(&self) -> TIM2F_R {
        TIM2F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for TIM3
    #[inline(always)]
    pub fn tim3f(&self) -> TIM3F_R {
        TIM3F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for TIM4
    #[inline(always)]
    pub fn tim4f(&self) -> TIM4F_R {
        TIM4F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for TIM5
    #[inline(always)]
    pub fn tim5f(&self) -> TIM5F_R {
        TIM5F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for TIM6
    #[inline(always)]
    pub fn tim6f(&self) -> TIM6F_R {
        TIM6F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for TIM7
    #[inline(always)]
    pub fn tim7f(&self) -> TIM7F_R {
        TIM7F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for WWDG
    #[inline(always)]
    pub fn wwdgf(&self) -> WWDGF_R {
        WWDGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for IWDG
    #[inline(always)]
    pub fn iwdgf(&self) -> IWDGF_R {
        IWDGF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for SPI2
    #[inline(always)]
    pub fn spi2f(&self) -> SPI2F_R {
        SPI2F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for USART2
    #[inline(always)]
    pub fn usart2f(&self) -> USART2F_R {
        USART2F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for USART3
    #[inline(always)]
    pub fn usart3f(&self) -> USART3F_R {
        USART3F_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access flag for UART4
    #[inline(always)]
    pub fn uart4f(&self) -> UART4F_R {
        UART4F_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for UART5
    #[inline(always)]
    pub fn uart5f(&self) -> UART5F_R {
        UART5F_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for I2C1
    #[inline(always)]
    pub fn i2c1f(&self) -> I2C1F_R {
        I2C1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for I2C2
    #[inline(always)]
    pub fn i2c2f(&self) -> I2C2F_R {
        I2C2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for CRS
    #[inline(always)]
    pub fn crsf(&self) -> CRSF_R {
        CRSF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access flag for I2C4
    #[inline(always)]
    pub fn i2c4f(&self) -> I2C4F_R {
        I2C4F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for LPTIM2
    #[inline(always)]
    pub fn lptim2f(&self) -> LPTIM2F_R {
        LPTIM2F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for FDCAN1
    #[inline(always)]
    pub fn fdcan1f(&self) -> FDCAN1F_R {
        FDCAN1F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access flag for UCPD1
    #[inline(always)]
    pub fn ucpd1f(&self) -> UCPD1F_R {
        UCPD1F_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - illegal access flag for USART6
    #[inline(always)]
    pub fn usart6f(&self) -> USART6F_R {
        USART6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access flag for I2C5
    #[inline(always)]
    pub fn i2c5f(&self) -> I2C5F_R {
        I2C5F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access flag for I2C6
    #[inline(always)]
    pub fn i2c6f(&self) -> I2C6F_R {
        I2C6F_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("tim2f", &self.tim2f())
            .field("tim3f", &self.tim3f())
            .field("tim4f", &self.tim4f())
            .field("tim5f", &self.tim5f())
            .field("tim6f", &self.tim6f())
            .field("tim7f", &self.tim7f())
            .field("wwdgf", &self.wwdgf())
            .field("iwdgf", &self.iwdgf())
            .field("spi2f", &self.spi2f())
            .field("usart2f", &self.usart2f())
            .field("usart3f", &self.usart3f())
            .field("uart4f", &self.uart4f())
            .field("uart5f", &self.uart5f())
            .field("i2c1f", &self.i2c1f())
            .field("i2c2f", &self.i2c2f())
            .field("crsf", &self.crsf())
            .field("i2c4f", &self.i2c4f())
            .field("lptim2f", &self.lptim2f())
            .field("fdcan1f", &self.fdcan1f())
            .field("ucpd1f", &self.ucpd1f())
            .field("usart6f", &self.usart6f())
            .field("i2c5f", &self.i2c5f())
            .field("i2c6f", &self.i2c6f())
            .finish()
    }
}
/**TZIC status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
