///Register `AHB3RSTR` reader
pub type R = crate::R<AHB3RSTRrs>;
///Register `AHB3RSTR` writer
pub type W = crate::W<AHB3RSTRrs>;
/**LPGPIO1 reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPGPIO1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<LPGPIO1RST> for bool {
    #[inline(always)]
    fn from(variant: LPGPIO1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `LPGPIO1RST` reader - LPGPIO1 reset This bit is set and cleared by software.
pub type LPGPIO1RST_R = crate::BitReader<LPGPIO1RST>;
impl LPGPIO1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPGPIO1RST {
        match self.bits {
            false => LPGPIO1RST::NoEffect,
            true => LPGPIO1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LPGPIO1RST::NoEffect
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPGPIO1RST::Reset
    }
}
///Field `LPGPIO1RST` writer - LPGPIO1 reset This bit is set and cleared by software.
pub type LPGPIO1RST_W<'a, REG> = crate::BitWriter<'a, REG, LPGPIO1RST>;
impl<'a, REG> LPGPIO1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LPGPIO1RST::NoEffect)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPGPIO1RST::Reset)
    }
}
///Field `ADC4RST` reader - ADC4 reset This bit is set and cleared by software.
pub use LPGPIO1RST_R as ADC4RST_R;
///Field `DAC1RST` reader - DAC1 reset This bit is set and cleared by software.
pub use LPGPIO1RST_R as DAC1RST_R;
///Field `LPDMA1RST` reader - LPDMA1 reset This bit is set and cleared by software.
pub use LPGPIO1RST_R as LPDMA1RST_R;
///Field `ADF1RST` reader - ADF1 reset This bit is set and cleared by software.
pub use LPGPIO1RST_R as ADF1RST_R;
///Field `ADC4RST` writer - ADC4 reset This bit is set and cleared by software.
pub use LPGPIO1RST_W as ADC4RST_W;
///Field `DAC1RST` writer - DAC1 reset This bit is set and cleared by software.
pub use LPGPIO1RST_W as DAC1RST_W;
///Field `LPDMA1RST` writer - LPDMA1 reset This bit is set and cleared by software.
pub use LPGPIO1RST_W as LPDMA1RST_W;
///Field `ADF1RST` writer - ADF1 reset This bit is set and cleared by software.
pub use LPGPIO1RST_W as ADF1RST_W;
impl R {
    ///Bit 0 - LPGPIO1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1rst(&self) -> LPGPIO1RST_R {
        LPGPIO1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - ADC4 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn adc4rst(&self) -> ADC4RST_R {
        ADC4RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAC1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LPDMA1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpdma1rst(&self) -> LPDMA1RST_R {
        LPDMA1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADF1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn adf1rst(&self) -> ADF1RST_R {
        ADF1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTR")
            .field("lpgpio1rst", &self.lpgpio1rst())
            .field("adc4rst", &self.adc4rst())
            .field("dac1rst", &self.dac1rst())
            .field("lpdma1rst", &self.lpdma1rst())
            .field("adf1rst", &self.adf1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPGPIO1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1rst(&mut self) -> LPGPIO1RST_W<AHB3RSTRrs> {
        LPGPIO1RST_W::new(self, 0)
    }
    ///Bit 5 - ADC4 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn adc4rst(&mut self) -> ADC4RST_W<AHB3RSTRrs> {
        ADC4RST_W::new(self, 5)
    }
    ///Bit 6 - DAC1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn dac1rst(&mut self) -> DAC1RST_W<AHB3RSTRrs> {
        DAC1RST_W::new(self, 6)
    }
    ///Bit 9 - LPDMA1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpdma1rst(&mut self) -> LPDMA1RST_W<AHB3RSTRrs> {
        LPDMA1RST_W::new(self, 9)
    }
    ///Bit 10 - ADF1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn adf1rst(&mut self) -> ADF1RST_W<AHB3RSTRrs> {
        ADF1RST_W::new(self, 10)
    }
}
/**RCC AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RCC:AHB3RSTR)*/
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3rstr::R`](R) reader structure
impl crate::Readable for AHB3RSTRrs {}
///`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTRrs {}
