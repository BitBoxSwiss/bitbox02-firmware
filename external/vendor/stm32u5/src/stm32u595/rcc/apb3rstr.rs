///Register `APB3RSTR` reader
pub type R = crate::R<APB3RSTRrs>;
///Register `APB3RSTR` writer
pub type W = crate::W<APB3RSTRrs>;
/**SYSCFG reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<SYSCFGRST> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGRST` reader - SYSCFG reset This bit is set and cleared by software.
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST>;
impl SYSCFGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGRST {
        match self.bits {
            false => SYSCFGRST::NoEffect,
            true => SYSCFGRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SYSCFGRST::NoEffect
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST::Reset
    }
}
///Field `SYSCFGRST` writer - SYSCFG reset This bit is set and cleared by software.
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRST>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::NoEffect)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::Reset)
    }
}
///Field `SPI3RST` reader - SPI3 reset This bit is set and cleared by software.
pub use SYSCFGRST_R as SPI3RST_R;
///Field `LPUART1RST` reader - LPUART1 reset This bit is set and cleared by software.
pub use SYSCFGRST_R as LPUART1RST_R;
///Field `I2C3RST` reader - I2C3 reset This bit is set and cleared by software.
pub use SYSCFGRST_R as I2C3RST_R;
///Field `LPTIM1RST` reader - LPTIM1 reset This bit is set and cleared by software.
pub use SYSCFGRST_R as LPTIM1RST_R;
///Field `LPTIM3RST` reader - LPTIM3 reset This bit is set and cleared by software.
pub use SYSCFGRST_R as LPTIM3RST_R;
///Field `LPTIM4RST` reader - LPTIM4 reset This bit is set and cleared by software.
pub use SYSCFGRST_R as LPTIM4RST_R;
///Field `OPAMPRST` reader - OPAMP reset This bit is set and cleared by software.
pub use SYSCFGRST_R as OPAMPRST_R;
///Field `COMPRST` reader - COMP reset This bit is set and cleared by software.
pub use SYSCFGRST_R as COMPRST_R;
///Field `VREFRST` reader - VREFBUF reset This bit is set and cleared by software.
pub use SYSCFGRST_R as VREFRST_R;
///Field `SPI3RST` writer - SPI3 reset This bit is set and cleared by software.
pub use SYSCFGRST_W as SPI3RST_W;
///Field `LPUART1RST` writer - LPUART1 reset This bit is set and cleared by software.
pub use SYSCFGRST_W as LPUART1RST_W;
///Field `I2C3RST` writer - I2C3 reset This bit is set and cleared by software.
pub use SYSCFGRST_W as I2C3RST_W;
///Field `LPTIM1RST` writer - LPTIM1 reset This bit is set and cleared by software.
pub use SYSCFGRST_W as LPTIM1RST_W;
///Field `LPTIM3RST` writer - LPTIM3 reset This bit is set and cleared by software.
pub use SYSCFGRST_W as LPTIM3RST_W;
///Field `LPTIM4RST` writer - LPTIM4 reset This bit is set and cleared by software.
pub use SYSCFGRST_W as LPTIM4RST_W;
///Field `OPAMPRST` writer - OPAMP reset This bit is set and cleared by software.
pub use SYSCFGRST_W as OPAMPRST_W;
///Field `COMPRST` writer - COMP reset This bit is set and cleared by software.
pub use SYSCFGRST_W as COMPRST_W;
///Field `VREFRST` writer - VREFBUF reset This bit is set and cleared by software.
pub use SYSCFGRST_W as VREFRST_W;
impl R {
    ///Bit 1 - SYSCFG reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn comprst(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("spi3rst", &self.spi3rst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i2c3rst", &self.i2c3rst())
            .field("lptim1rst", &self.lptim1rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("lptim4rst", &self.lptim4rst())
            .field("opamprst", &self.opamprst())
            .field("comprst", &self.comprst())
            .field("vrefrst", &self.vrefrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB3RSTRrs> {
        SYSCFGRST_W::new(self, 1)
    }
    ///Bit 5 - SPI3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<APB3RSTRrs> {
        SPI3RST_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APB3RSTRrs> {
        LPUART1RST_W::new(self, 6)
    }
    ///Bit 7 - I2C3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<APB3RSTRrs> {
        I2C3RST_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APB3RSTRrs> {
        LPTIM1RST_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<APB3RSTRrs> {
        LPTIM3RST_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<APB3RSTRrs> {
        LPTIM4RST_W::new(self, 13)
    }
    ///Bit 14 - OPAMP reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn opamprst(&mut self) -> OPAMPRST_W<APB3RSTRrs> {
        OPAMPRST_W::new(self, 14)
    }
    ///Bit 15 - COMP reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn comprst(&mut self) -> COMPRST_W<APB3RSTRrs> {
        COMPRST_W::new(self, 15)
    }
    ///Bit 20 - VREFBUF reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W<APB3RSTRrs> {
        VREFRST_W::new(self, 20)
    }
}
/**RCC APB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:APB3RSTR)*/
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3rstr::R`](R) reader structure
impl crate::Readable for APB3RSTRrs {}
///`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3RSTR to value 0
impl crate::Resettable for APB3RSTRrs {}
