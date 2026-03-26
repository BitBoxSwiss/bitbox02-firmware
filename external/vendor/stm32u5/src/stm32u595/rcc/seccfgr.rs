///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
/**HSI clock configuration and status bit security This bit is set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSISEC {
    ///0: Nonsecure
    NonSecure = 0,
    ///1: Secure
    Secure = 1,
}
impl From<HSISEC> for bool {
    #[inline(always)]
    fn from(variant: HSISEC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSISEC` reader - HSI clock configuration and status bit security This bit is set and reset by software.
pub type HSISEC_R = crate::BitReader<HSISEC>;
impl HSISEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSISEC {
        match self.bits {
            false => HSISEC::NonSecure,
            true => HSISEC::Secure,
        }
    }
    ///Nonsecure
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == HSISEC::NonSecure
    }
    ///Secure
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == HSISEC::Secure
    }
}
///Field `HSISEC` writer - HSI clock configuration and status bit security This bit is set and reset by software.
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG, HSISEC>;
impl<'a, REG> HSISEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Nonsecure
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(HSISEC::NonSecure)
    }
    ///Secure
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(HSISEC::Secure)
    }
}
///Field `HSESEC` reader - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software.
pub use HSISEC_R as HSESEC_R;
///Field `MSISEC` reader - MSI clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_R as MSISEC_R;
///Field `LSISEC` reader - LSI clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_R as LSISEC_R;
///Field `LSESEC` reader - LSE clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_R as LSESEC_R;
///Field `SYSCLKSEC` reader - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software.
pub use HSISEC_R as SYSCLKSEC_R;
///Field `PRESCSEC` reader - AHBx/APBx prescaler configuration bits security This bit is set and reset by software.
pub use HSISEC_R as PRESCSEC_R;
///Field `PLL1SEC` reader - PLL1 clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_R as PLL1SEC_R;
///Field `PLL2SEC` reader - PLL2 clock configuration and status bit security Set and reset by software.
pub use HSISEC_R as PLL2SEC_R;
///Field `PLL3SEC` reader - PLL3 clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_R as PLL3SEC_R;
///Field `ICLKSEC` reader - Intermediate clock source selection security This bit is set and reset by software.
pub use HSISEC_R as ICLKSEC_R;
///Field `HSI48SEC` reader - HSI48 clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_R as HSI48SEC_R;
///Field `RMVFSEC` reader - Remove reset flag security This bit is set and reset by software.
pub use HSISEC_R as RMVFSEC_R;
///Field `HSESEC` writer - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software.
pub use HSISEC_W as HSESEC_W;
///Field `MSISEC` writer - MSI clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_W as MSISEC_W;
///Field `LSISEC` writer - LSI clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_W as LSISEC_W;
///Field `LSESEC` writer - LSE clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_W as LSESEC_W;
///Field `SYSCLKSEC` writer - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software.
pub use HSISEC_W as SYSCLKSEC_W;
///Field `PRESCSEC` writer - AHBx/APBx prescaler configuration bits security This bit is set and reset by software.
pub use HSISEC_W as PRESCSEC_W;
///Field `PLL1SEC` writer - PLL1 clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_W as PLL1SEC_W;
///Field `PLL2SEC` writer - PLL2 clock configuration and status bit security Set and reset by software.
pub use HSISEC_W as PLL2SEC_W;
///Field `PLL3SEC` writer - PLL3 clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_W as PLL3SEC_W;
///Field `ICLKSEC` writer - Intermediate clock source selection security This bit is set and reset by software.
pub use HSISEC_W as ICLKSEC_W;
///Field `HSI48SEC` writer - HSI48 clock configuration and status bit security This bit is set and reset by software.
pub use HSISEC_W as HSI48SEC_W;
///Field `RMVFSEC` writer - Remove reset flag security This bit is set and reset by software.
pub use HSISEC_W as RMVFSEC_W;
impl R {
    ///Bit 0 - HSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LSE clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software.
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AHBx/APBx prescaler configuration bits security This bit is set and reset by software.
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL1 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn pll1sec(&self) -> PLL1SEC_R {
        PLL1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL2 clock configuration and status bit security Set and reset by software.
    #[inline(always)]
    pub fn pll2sec(&self) -> PLL2SEC_R {
        PLL2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL3 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn pll3sec(&self) -> PLL3SEC_R {
        PLL3SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Intermediate clock source selection security This bit is set and reset by software.
    #[inline(always)]
    pub fn iclksec(&self) -> ICLKSEC_R {
        ICLKSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI48 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Remove reset flag security This bit is set and reset by software.
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("hsisec", &self.hsisec())
            .field("hsesec", &self.hsesec())
            .field("msisec", &self.msisec())
            .field("lsisec", &self.lsisec())
            .field("lsesec", &self.lsesec())
            .field("sysclksec", &self.sysclksec())
            .field("prescsec", &self.prescsec())
            .field("pll1sec", &self.pll1sec())
            .field("pll2sec", &self.pll2sec())
            .field("pll3sec", &self.pll3sec())
            .field("iclksec", &self.iclksec())
            .field("hsi48sec", &self.hsi48sec())
            .field("rmvfsec", &self.rmvfsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsisec(&mut self) -> HSISEC_W<SECCFGRrs> {
        HSISEC_W::new(self, 0)
    }
    ///Bit 1 - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsesec(&mut self) -> HSESEC_W<SECCFGRrs> {
        HSESEC_W::new(self, 1)
    }
    ///Bit 2 - MSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn msisec(&mut self) -> MSISEC_W<SECCFGRrs> {
        MSISEC_W::new(self, 2)
    }
    ///Bit 3 - LSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn lsisec(&mut self) -> LSISEC_W<SECCFGRrs> {
        LSISEC_W::new(self, 3)
    }
    ///Bit 4 - LSE clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn lsesec(&mut self) -> LSESEC_W<SECCFGRrs> {
        LSESEC_W::new(self, 4)
    }
    ///Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software.
    #[inline(always)]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<SECCFGRrs> {
        SYSCLKSEC_W::new(self, 5)
    }
    ///Bit 6 - AHBx/APBx prescaler configuration bits security This bit is set and reset by software.
    #[inline(always)]
    pub fn prescsec(&mut self) -> PRESCSEC_W<SECCFGRrs> {
        PRESCSEC_W::new(self, 6)
    }
    ///Bit 7 - PLL1 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn pll1sec(&mut self) -> PLL1SEC_W<SECCFGRrs> {
        PLL1SEC_W::new(self, 7)
    }
    ///Bit 8 - PLL2 clock configuration and status bit security Set and reset by software.
    #[inline(always)]
    pub fn pll2sec(&mut self) -> PLL2SEC_W<SECCFGRrs> {
        PLL2SEC_W::new(self, 8)
    }
    ///Bit 9 - PLL3 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn pll3sec(&mut self) -> PLL3SEC_W<SECCFGRrs> {
        PLL3SEC_W::new(self, 9)
    }
    ///Bit 10 - Intermediate clock source selection security This bit is set and reset by software.
    #[inline(always)]
    pub fn iclksec(&mut self) -> ICLKSEC_W<SECCFGRrs> {
        ICLKSEC_W::new(self, 10)
    }
    ///Bit 11 - HSI48 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<SECCFGRrs> {
        HSI48SEC_W::new(self, 11)
    }
    ///Bit 12 - Remove reset flag security This bit is set and reset by software.
    #[inline(always)]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<SECCFGRrs> {
        RMVFSEC_W::new(self, 12)
    }
}
/**RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
