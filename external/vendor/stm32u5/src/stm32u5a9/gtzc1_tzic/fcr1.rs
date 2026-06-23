///Register `FCR1` writer
pub type W = crate::W<FCR1rs>;
///Field `CTIM2F` writer - clear the illegal access flag for TIM2
pub type CTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM3F` writer - clear the illegal access flag for TIM3
pub type CTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM4F` writer - clear the illegal access flag for TIM4
pub type CTIM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM5F` writer - clear the illegal access flag for TIM5
pub type CTIM5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM6F` writer - clear the illegal access flag for TIM6
pub type CTIM6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM7F` writer - clear the illegal access flag for TIM7
pub type CTIM7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWWDGF` writer - clear the illegal access flag for WWDG
pub type CWWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIWDGF` writer - clear the illegal access flag for IWDG
pub type CIWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI2F` writer - clear the illegal access flag for SPI2
pub type CSPI2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART2F` writer - clear the illegal access flag for USART2
pub type CUSART2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART3F` writer - clear the illegal access flag for USART3
pub type CUSART3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUART4F` writer - clear the illegal access flag for UART4
pub type CUART4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUART5F` writer - clear the illegal access flag for UART5
pub type CUART5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C1F` writer - clear the illegal access flag for I2C1
pub type CI2C1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C2F` writer - clear the illegal access flag for I2C2
pub type CI2C2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRSF` writer - clear the illegal access flag for CRS
pub type CCRSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C4F` writer - clear the illegal access flag for I2C4
pub type CI2C4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM2F` writer - clear the illegal access flag for LPTIM2
pub type CLPTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFDCAN1F` writer - clear the illegal access flag for FDCAN1
pub type CFDCAN1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUCPD1F` writer - clear the illegal access flag for UCPD1
pub type CUCPD1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART6F` writer - clear the illegal access flag for USART6
pub type CUSART6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C5F` writer - clear the illegal access flag for I2C5
pub type CI2C5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C6F` writer - clear the illegal access flag for I2C6
pub type CI2C6F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for TIM2
    #[inline(always)]
    pub fn ctim2f(&mut self) -> CTIM2F_W<FCR1rs> {
        CTIM2F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for TIM3
    #[inline(always)]
    pub fn ctim3f(&mut self) -> CTIM3F_W<FCR1rs> {
        CTIM3F_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for TIM4
    #[inline(always)]
    pub fn ctim4f(&mut self) -> CTIM4F_W<FCR1rs> {
        CTIM4F_W::new(self, 2)
    }
    ///Bit 3 - clear the illegal access flag for TIM5
    #[inline(always)]
    pub fn ctim5f(&mut self) -> CTIM5F_W<FCR1rs> {
        CTIM5F_W::new(self, 3)
    }
    ///Bit 4 - clear the illegal access flag for TIM6
    #[inline(always)]
    pub fn ctim6f(&mut self) -> CTIM6F_W<FCR1rs> {
        CTIM6F_W::new(self, 4)
    }
    ///Bit 5 - clear the illegal access flag for TIM7
    #[inline(always)]
    pub fn ctim7f(&mut self) -> CTIM7F_W<FCR1rs> {
        CTIM7F_W::new(self, 5)
    }
    ///Bit 6 - clear the illegal access flag for WWDG
    #[inline(always)]
    pub fn cwwdgf(&mut self) -> CWWDGF_W<FCR1rs> {
        CWWDGF_W::new(self, 6)
    }
    ///Bit 7 - clear the illegal access flag for IWDG
    #[inline(always)]
    pub fn ciwdgf(&mut self) -> CIWDGF_W<FCR1rs> {
        CIWDGF_W::new(self, 7)
    }
    ///Bit 8 - clear the illegal access flag for SPI2
    #[inline(always)]
    pub fn cspi2f(&mut self) -> CSPI2F_W<FCR1rs> {
        CSPI2F_W::new(self, 8)
    }
    ///Bit 9 - clear the illegal access flag for USART2
    #[inline(always)]
    pub fn cusart2f(&mut self) -> CUSART2F_W<FCR1rs> {
        CUSART2F_W::new(self, 9)
    }
    ///Bit 10 - clear the illegal access flag for USART3
    #[inline(always)]
    pub fn cusart3f(&mut self) -> CUSART3F_W<FCR1rs> {
        CUSART3F_W::new(self, 10)
    }
    ///Bit 11 - clear the illegal access flag for UART4
    #[inline(always)]
    pub fn cuart4f(&mut self) -> CUART4F_W<FCR1rs> {
        CUART4F_W::new(self, 11)
    }
    ///Bit 12 - clear the illegal access flag for UART5
    #[inline(always)]
    pub fn cuart5f(&mut self) -> CUART5F_W<FCR1rs> {
        CUART5F_W::new(self, 12)
    }
    ///Bit 13 - clear the illegal access flag for I2C1
    #[inline(always)]
    pub fn ci2c1f(&mut self) -> CI2C1F_W<FCR1rs> {
        CI2C1F_W::new(self, 13)
    }
    ///Bit 14 - clear the illegal access flag for I2C2
    #[inline(always)]
    pub fn ci2c2f(&mut self) -> CI2C2F_W<FCR1rs> {
        CI2C2F_W::new(self, 14)
    }
    ///Bit 15 - clear the illegal access flag for CRS
    #[inline(always)]
    pub fn ccrsf(&mut self) -> CCRSF_W<FCR1rs> {
        CCRSF_W::new(self, 15)
    }
    ///Bit 16 - clear the illegal access flag for I2C4
    #[inline(always)]
    pub fn ci2c4f(&mut self) -> CI2C4F_W<FCR1rs> {
        CI2C4F_W::new(self, 16)
    }
    ///Bit 17 - clear the illegal access flag for LPTIM2
    #[inline(always)]
    pub fn clptim2f(&mut self) -> CLPTIM2F_W<FCR1rs> {
        CLPTIM2F_W::new(self, 17)
    }
    ///Bit 18 - clear the illegal access flag for FDCAN1
    #[inline(always)]
    pub fn cfdcan1f(&mut self) -> CFDCAN1F_W<FCR1rs> {
        CFDCAN1F_W::new(self, 18)
    }
    ///Bit 19 - clear the illegal access flag for UCPD1
    #[inline(always)]
    pub fn cucpd1f(&mut self) -> CUCPD1F_W<FCR1rs> {
        CUCPD1F_W::new(self, 19)
    }
    ///Bit 21 - clear the illegal access flag for USART6
    #[inline(always)]
    pub fn cusart6f(&mut self) -> CUSART6F_W<FCR1rs> {
        CUSART6F_W::new(self, 21)
    }
    ///Bit 22 - clear the illegal access flag for I2C5
    #[inline(always)]
    pub fn ci2c5f(&mut self) -> CI2C5F_W<FCR1rs> {
        CI2C5F_W::new(self, 22)
    }
    ///Bit 23 - clear the illegal access flag for I2C6
    #[inline(always)]
    pub fn ci2c6f(&mut self) -> CI2C6F_W<FCR1rs> {
        CI2C6F_W::new(self, 23)
    }
}
/**TZIC flag clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_TZIC:FCR1)*/
pub struct FCR1rs;
impl crate::RegisterSpec for FCR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr1::W`](W) writer structure
impl crate::Writable for FCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR1 to value 0
impl crate::Resettable for FCR1rs {}
