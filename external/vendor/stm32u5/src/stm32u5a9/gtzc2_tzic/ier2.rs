///Register `IER2` reader
pub type R = crate::R<IER2rs>;
///Register `IER2` writer
pub type W = crate::W<IER2rs>;
///Field `SYSCFGIE` reader - illegal access interrupt enable for SYSCFG
pub type SYSCFGIE_R = crate::BitReader;
///Field `SYSCFGIE` writer - illegal access interrupt enable for SYSCFG
pub type SYSCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCIE` reader - illegal access interrupt enable for RTC
pub type RTCIE_R = crate::BitReader;
///Field `RTCIE` writer - illegal access interrupt enable for RTC
pub type RTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPIE` reader - illegal access interrupt enable for TAMP
pub type TAMPIE_R = crate::BitReader;
///Field `TAMPIE` writer - illegal access interrupt enable for TAMP
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRIE` reader - illegal access interrupt enable for PWR
pub type PWRIE_R = crate::BitReader;
///Field `PWRIE` writer - illegal access interrupt enable for PWR
pub type PWRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCCIE` reader - illegal access interrupt enable for RCC
pub type RCCIE_R = crate::BitReader;
///Field `RCCIE` writer - illegal access interrupt enable for RCC
pub type RCCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPDMA1IE` reader - illegal access interrupt enable for LPDMA
pub type LPDMA1IE_R = crate::BitReader;
///Field `LPDMA1IE` writer - illegal access interrupt enable for LPDMA
pub type LPDMA1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTIIE` reader - illegal access interrupt enable for EXTI
pub type EXTIIE_R = crate::BitReader;
///Field `EXTIIE` writer - illegal access interrupt enable for EXTI
pub type EXTIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZSC2IE` reader - illegal access interrupt enable for GTZC2 TZSC registers
pub type TZSC2IE_R = crate::BitReader;
///Field `TZSC2IE` writer - illegal access interrupt enable for GTZC2 TZSC registers
pub type TZSC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZIC2IE` reader - illegal access interrupt enable for GTZC2 TZIC registers
pub type TZIC2IE_R = crate::BitReader;
///Field `TZIC2IE` writer - illegal access interrupt enable for GTZC2 TZIC registers
pub type TZIC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM4IE` reader - illegal access interrupt enable for SRAM4
pub type SRAM4IE_R = crate::BitReader;
///Field `SRAM4IE` writer - illegal access interrupt enable for SRAM4
pub type SRAM4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB4_REGIE` reader - illegal access interrupt enable for MPCBB4 registers
pub type MPCBB4_REGIE_R = crate::BitReader;
///Field `MPCBB4_REGIE` writer - illegal access interrupt enable for MPCBB4 registers
pub type MPCBB4_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for SYSCFG
    #[inline(always)]
    pub fn syscfgie(&self) -> SYSCFGIE_R {
        SYSCFGIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for RTC
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for TAMP
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for PWR
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for RCC
    #[inline(always)]
    pub fn rccie(&self) -> RCCIE_R {
        RCCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for LPDMA
    #[inline(always)]
    pub fn lpdma1ie(&self) -> LPDMA1IE_R {
        LPDMA1IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for EXTI
    #[inline(always)]
    pub fn extiie(&self) -> EXTIIE_R {
        EXTIIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC2 TZSC registers
    #[inline(always)]
    pub fn tzsc2ie(&self) -> TZSC2IE_R {
        TZSC2IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC2 TZIC registers
    #[inline(always)]
    pub fn tzic2ie(&self) -> TZIC2IE_R {
        TZIC2IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM4
    #[inline(always)]
    pub fn sram4ie(&self) -> SRAM4IE_R {
        SRAM4IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB4 registers
    #[inline(always)]
    pub fn mpcbb4_regie(&self) -> MPCBB4_REGIE_R {
        MPCBB4_REGIE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER2")
            .field("syscfgie", &self.syscfgie())
            .field("rtcie", &self.rtcie())
            .field("tampie", &self.tampie())
            .field("pwrie", &self.pwrie())
            .field("rccie", &self.rccie())
            .field("lpdma1ie", &self.lpdma1ie())
            .field("extiie", &self.extiie())
            .field("tzsc2ie", &self.tzsc2ie())
            .field("tzic2ie", &self.tzic2ie())
            .field("sram4ie", &self.sram4ie())
            .field("mpcbb4_regie", &self.mpcbb4_regie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for SYSCFG
    #[inline(always)]
    pub fn syscfgie(&mut self) -> SYSCFGIE_W<IER2rs> {
        SYSCFGIE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for RTC
    #[inline(always)]
    pub fn rtcie(&mut self) -> RTCIE_W<IER2rs> {
        RTCIE_W::new(self, 1)
    }
    ///Bit 2 - illegal access interrupt enable for TAMP
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<IER2rs> {
        TAMPIE_W::new(self, 2)
    }
    ///Bit 3 - illegal access interrupt enable for PWR
    #[inline(always)]
    pub fn pwrie(&mut self) -> PWRIE_W<IER2rs> {
        PWRIE_W::new(self, 3)
    }
    ///Bit 4 - illegal access interrupt enable for RCC
    #[inline(always)]
    pub fn rccie(&mut self) -> RCCIE_W<IER2rs> {
        RCCIE_W::new(self, 4)
    }
    ///Bit 5 - illegal access interrupt enable for LPDMA
    #[inline(always)]
    pub fn lpdma1ie(&mut self) -> LPDMA1IE_W<IER2rs> {
        LPDMA1IE_W::new(self, 5)
    }
    ///Bit 6 - illegal access interrupt enable for EXTI
    #[inline(always)]
    pub fn extiie(&mut self) -> EXTIIE_W<IER2rs> {
        EXTIIE_W::new(self, 6)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC2 TZSC registers
    #[inline(always)]
    pub fn tzsc2ie(&mut self) -> TZSC2IE_W<IER2rs> {
        TZSC2IE_W::new(self, 14)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC2 TZIC registers
    #[inline(always)]
    pub fn tzic2ie(&mut self) -> TZIC2IE_W<IER2rs> {
        TZIC2IE_W::new(self, 15)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM4
    #[inline(always)]
    pub fn sram4ie(&mut self) -> SRAM4IE_W<IER2rs> {
        SRAM4IE_W::new(self, 24)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB4 registers
    #[inline(always)]
    pub fn mpcbb4_regie(&mut self) -> MPCBB4_REGIE_W<IER2rs> {
        MPCBB4_REGIE_W::new(self, 25)
    }
}
/**TZIC interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`ier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC2_TZIC:IER2)*/
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
