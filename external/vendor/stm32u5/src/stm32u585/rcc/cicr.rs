///Register `CICR` writer
pub type W = crate::W<CICRrs>;
/**LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW {
    ///1: Clear flag
    Clear = 1,
}
impl From<LSIRDYCW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYCW>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYCW::Clear)
    }
}
///Field `LSERDYC` writer - LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as LSERDYC_W;
///Field `MSISRDYC` writer - MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as MSISRDYC_W;
///Field `HSIRDYC` writer - HSI16 ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as HSIRDYC_W;
///Field `HSERDYC` writer - HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as HSERDYC_W;
///Field `HSI48RDYC` writer - HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as HSI48RDYC_W;
///Field `PLL1RDYC` writer - PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as PLL1RDYC_W;
///Field `PLL2RDYC` writer - PLL2 ready interrupt clear Writing this bit to 1 clears the PLL2RDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as PLL2RDYC_W;
///Field `PLL3RDYC` writer - PLL3 ready interrupt clear Writing this bit to 1 clears the PLL3RDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as PLL3RDYC_W;
///Field `CSSC` writer - Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect.
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIKRDYC` writer - MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as MSIKRDYC_W;
///Field `SHSIRDYC` writer - SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect.
pub use LSIRDYC_W as SHSIRDYC_W;
impl core::fmt::Debug for crate::generic::Reg<CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn msisrdyc(&mut self) -> MSISRDYC_W<CICRrs> {
        MSISRDYC_W::new(self, 2)
    }
    ///Bit 3 - HSI16 ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 5 - HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<CICRrs> {
        HSI48RDYC_W::new(self, 5)
    }
    ///Bit 6 - PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<CICRrs> {
        PLL1RDYC_W::new(self, 6)
    }
    ///Bit 7 - PLL2 ready interrupt clear Writing this bit to 1 clears the PLL2RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W<CICRrs> {
        PLL2RDYC_W::new(self, 7)
    }
    ///Bit 8 - PLL3 ready interrupt clear Writing this bit to 1 clears the PLL3RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn pll3rdyc(&mut self) -> PLL3RDYC_W<CICRrs> {
        PLL3RDYC_W::new(self, 8)
    }
    ///Bit 10 - Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<CICRrs> {
        CSSC_W::new(self, 10)
    }
    ///Bit 11 - MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn msikrdyc(&mut self) -> MSIKRDYC_W<CICRrs> {
        MSIKRDYC_W::new(self, 11)
    }
    ///Bit 12 - SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    pub fn shsirdyc(&mut self) -> SHSIRDYC_W<CICRrs> {
        SHSIRDYC_W::new(self, 12)
    }
}
/**RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RCC:CICR)*/
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cicr::W`](W) writer structure
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICRrs {}
