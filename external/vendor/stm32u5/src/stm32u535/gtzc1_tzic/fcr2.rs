///Register `FCR2` writer
pub type W = crate::W<FCR2rs>;
///Field `CTIM1F` writer - clear the illegal access flag for TIM1
pub type CTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI1F` writer - clear the illegal access flag for SPI1
pub type CSPI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM8F` writer - clear the illegal access flag for TIM8
pub type CTIM8F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART1F` writer - clear the illegal access flag for USART1
pub type CUSART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM15F` writer - clear the illegal access flag for TIM5
pub type CTIM15F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM16F` writer - clear the illegal access flag for TIM6
pub type CTIM16F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM17F` writer - clear the illegal access flag for TIM7
pub type CTIM17F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSAI1F` writer - clear the illegal access flag for SAI1
pub type CSAI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for TIM1
    #[inline(always)]
    pub fn ctim1f(&mut self) -> CTIM1F_W<FCR2rs> {
        CTIM1F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for SPI1
    #[inline(always)]
    pub fn cspi1f(&mut self) -> CSPI1F_W<FCR2rs> {
        CSPI1F_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for TIM8
    #[inline(always)]
    pub fn ctim8f(&mut self) -> CTIM8F_W<FCR2rs> {
        CTIM8F_W::new(self, 2)
    }
    ///Bit 3 - clear the illegal access flag for USART1
    #[inline(always)]
    pub fn cusart1f(&mut self) -> CUSART1F_W<FCR2rs> {
        CUSART1F_W::new(self, 3)
    }
    ///Bit 4 - clear the illegal access flag for TIM5
    #[inline(always)]
    pub fn ctim15f(&mut self) -> CTIM15F_W<FCR2rs> {
        CTIM15F_W::new(self, 4)
    }
    ///Bit 5 - clear the illegal access flag for TIM6
    #[inline(always)]
    pub fn ctim16f(&mut self) -> CTIM16F_W<FCR2rs> {
        CTIM16F_W::new(self, 5)
    }
    ///Bit 6 - clear the illegal access flag for TIM7
    #[inline(always)]
    pub fn ctim17f(&mut self) -> CTIM17F_W<FCR2rs> {
        CTIM17F_W::new(self, 6)
    }
    ///Bit 7 - clear the illegal access flag for SAI1
    #[inline(always)]
    pub fn csai1f(&mut self) -> CSAI1F_W<FCR2rs> {
        CSAI1F_W::new(self, 7)
    }
}
/**TZIC flag clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#GTZC1_TZIC:FCR2)*/
pub struct FCR2rs;
impl crate::RegisterSpec for FCR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr2::W`](W) writer structure
impl crate::Writable for FCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR2 to value 0
impl crate::Resettable for FCR2rs {}
