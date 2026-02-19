///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
/**LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE>;
impl LSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE {
        match self.bits {
            false => LSIRDYIE::Disabled,
            true => LSIRDYIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE::Enabled
    }
}
///Field `LSIRDYIE` writer - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYIE>;
impl<'a, REG> LSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Enabled)
    }
}
///Field `LSERDYIE` reader - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `MSISRDYIE` reader - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
pub use LSIRDYIE_R as MSISRDYIE_R;
///Field `HSIRDYIE` reader - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
pub use LSIRDYIE_R as HSIRDYIE_R;
///Field `HSERDYIE` reader - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
pub use LSIRDYIE_R as HSI48RDYIE_R;
///Field `PLL1RDYIE` reader - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
pub use LSIRDYIE_R as PLL1RDYIE_R;
///Field `PLL2RDYIE` reader - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
pub use LSIRDYIE_R as PLL2RDYIE_R;
///Field `PLL3RDYIE` reader - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
pub use LSIRDYIE_R as PLL3RDYIE_R;
///Field `MSIKRDYIE` reader - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
pub use LSIRDYIE_R as MSIKRDYIE_R;
///Field `SHSIRDYIE` reader - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
pub use LSIRDYIE_R as SHSIRDYIE_R;
///Field `LSERDYIE` writer - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
pub use LSIRDYIE_W as LSERDYIE_W;
///Field `MSISRDYIE` writer - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
pub use LSIRDYIE_W as MSISRDYIE_W;
///Field `HSIRDYIE` writer - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
pub use LSIRDYIE_W as HSIRDYIE_W;
///Field `HSERDYIE` writer - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
pub use LSIRDYIE_W as HSERDYIE_W;
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
pub use LSIRDYIE_W as HSI48RDYIE_W;
///Field `PLL1RDYIE` writer - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
pub use LSIRDYIE_W as PLL1RDYIE_W;
///Field `PLL2RDYIE` writer - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
pub use LSIRDYIE_W as PLL2RDYIE_W;
///Field `PLL3RDYIE` writer - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
pub use LSIRDYIE_W as PLL3RDYIE_W;
///Field `MSIKRDYIE` writer - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
pub use LSIRDYIE_W as MSIKRDYIE_W;
///Field `SHSIRDYIE` writer - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
pub use LSIRDYIE_W as SHSIRDYIE_W;
impl R {
    ///Bit 0 - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
    #[inline(always)]
    pub fn msisrdyie(&self) -> MSISRDYIE_R {
        MSISRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
    #[inline(always)]
    pub fn msikrdyie(&self) -> MSIKRDYIE_R {
        MSIKRDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
    #[inline(always)]
    pub fn shsirdyie(&self) -> SHSIRDYIE_R {
        SHSIRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("msisrdyie", &self.msisrdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsi48rdyie", &self.hsi48rdyie())
            .field("pll1rdyie", &self.pll1rdyie())
            .field("pll2rdyie", &self.pll2rdyie())
            .field("pll3rdyie", &self.pll3rdyie())
            .field("msikrdyie", &self.msikrdyie())
            .field("shsirdyie", &self.shsirdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - MSIS ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
    #[inline(always)]
    pub fn msisrdyie(&mut self) -> MSISRDYIE_W<CIERrs> {
        MSISRDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSI16 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    ///Bit 5 - HSI48 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<CIERrs> {
        HSI48RDYIE_W::new(self, 5)
    }
    ///Bit 6 - PLL ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
    #[inline(always)]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<CIERrs> {
        PLL1RDYIE_W::new(self, 6)
    }
    ///Bit 7 - PLL2 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
    #[inline(always)]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<CIERrs> {
        PLL2RDYIE_W::new(self, 7)
    }
    ///Bit 8 - PLL3 ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
    #[inline(always)]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<CIERrs> {
        PLL3RDYIE_W::new(self, 8)
    }
    ///Bit 11 - MSIK ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
    #[inline(always)]
    pub fn msikrdyie(&mut self) -> MSIKRDYIE_W<CIERrs> {
        MSIKRDYIE_W::new(self, 11)
    }
    ///Bit 12 - SHSI ready interrupt enable This bit is set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
    #[inline(always)]
    pub fn shsirdyie(&mut self) -> SHSIRDYIE_W<CIERrs> {
        SHSIRDYIE_W::new(self, 12)
    }
}
/**RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RCC:CIER)*/
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
///`read()` method returns [`cier::R`](R) reader structure
impl crate::Readable for CIERrs {}
///`write(|w| ..)` method takes [`cier::W`](W) writer structure
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIERrs {}
