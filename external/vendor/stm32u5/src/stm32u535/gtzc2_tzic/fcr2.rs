///Register `FCR2` writer
pub type W = crate::W<FCR2rs>;
///Field `CSYSCFGF` writer - clear the illegal access flag for SYSCFG
pub type CSYSCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRTCF` writer - clear the illegal access flag for RTC
pub type CRTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMPF` writer - clear the illegal access flag for TAMP
pub type CTAMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPWRF` writer - clear the illegal access flag for PWR
pub type CPWRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCCF` writer - clear the illegal access flag for RCC
pub type CRCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPDMA1F` writer - clear the illegal access flag for LPDMA
pub type CLPDMA1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEXTIF` writer - clear the illegal access flag for EXTI
pub type CEXTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTZSC2F` writer - clear the illegal access flag for GTZC2 TZSC registers
pub type CTZSC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTZIC2F` writer - clear the illegal access flag for GTZC2 TZIC registers
pub type CTZIC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRAM4F` writer - clear the illegal access flag for SRAM4
pub type CSRAM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPCBB4_REGF` writer - clear the illegal access flag for MPCBB4 registers
pub type CMPCBB4_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for SYSCFG
    #[inline(always)]
    pub fn csyscfgf(&mut self) -> CSYSCFGF_W<FCR2rs> {
        CSYSCFGF_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for RTC
    #[inline(always)]
    pub fn crtcf(&mut self) -> CRTCF_W<FCR2rs> {
        CRTCF_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for TAMP
    #[inline(always)]
    pub fn ctampf(&mut self) -> CTAMPF_W<FCR2rs> {
        CTAMPF_W::new(self, 2)
    }
    ///Bit 3 - clear the illegal access flag for PWR
    #[inline(always)]
    pub fn cpwrf(&mut self) -> CPWRF_W<FCR2rs> {
        CPWRF_W::new(self, 3)
    }
    ///Bit 4 - clear the illegal access flag for RCC
    #[inline(always)]
    pub fn crccf(&mut self) -> CRCCF_W<FCR2rs> {
        CRCCF_W::new(self, 4)
    }
    ///Bit 5 - clear the illegal access flag for LPDMA
    #[inline(always)]
    pub fn clpdma1f(&mut self) -> CLPDMA1F_W<FCR2rs> {
        CLPDMA1F_W::new(self, 5)
    }
    ///Bit 6 - clear the illegal access flag for EXTI
    #[inline(always)]
    pub fn cextif(&mut self) -> CEXTIF_W<FCR2rs> {
        CEXTIF_W::new(self, 6)
    }
    ///Bit 14 - clear the illegal access flag for GTZC2 TZSC registers
    #[inline(always)]
    pub fn ctzsc2f(&mut self) -> CTZSC2F_W<FCR2rs> {
        CTZSC2F_W::new(self, 14)
    }
    ///Bit 15 - clear the illegal access flag for GTZC2 TZIC registers
    #[inline(always)]
    pub fn ctzic2f(&mut self) -> CTZIC2F_W<FCR2rs> {
        CTZIC2F_W::new(self, 15)
    }
    ///Bit 24 - clear the illegal access flag for SRAM4
    #[inline(always)]
    pub fn csram4f(&mut self) -> CSRAM4F_W<FCR2rs> {
        CSRAM4F_W::new(self, 24)
    }
    ///Bit 25 - clear the illegal access flag for MPCBB4 registers
    #[inline(always)]
    pub fn cmpcbb4_regf(&mut self) -> CMPCBB4_REGF_W<FCR2rs> {
        CMPCBB4_REGF_W::new(self, 25)
    }
}
/**TZIC flag clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#GTZC2_TZIC:FCR2)*/
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
