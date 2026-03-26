///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
/**LPGPIO1 enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPGPIO1EN {
    ///0: Peripheral clock disabled
    Disabled = 0,
    ///1: Peripheral clock enabled
    Enabled = 1,
}
impl From<LPGPIO1EN> for bool {
    #[inline(always)]
    fn from(variant: LPGPIO1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPGPIO1EN` reader - LPGPIO1 enable This bit is set and cleared by software.
pub type LPGPIO1EN_R = crate::BitReader<LPGPIO1EN>;
impl LPGPIO1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPGPIO1EN {
        match self.bits {
            false => LPGPIO1EN::Disabled,
            true => LPGPIO1EN::Enabled,
        }
    }
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPGPIO1EN::Disabled
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPGPIO1EN::Enabled
    }
}
///Field `LPGPIO1EN` writer - LPGPIO1 enable This bit is set and cleared by software.
pub type LPGPIO1EN_W<'a, REG> = crate::BitWriter<'a, REG, LPGPIO1EN>;
impl<'a, REG> LPGPIO1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPGPIO1EN::Disabled)
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPGPIO1EN::Enabled)
    }
}
///Field `PWREN` reader - PWR clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_R as PWREN_R;
///Field `ADC4EN` reader - ADC4 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_R as ADC4EN_R;
///Field `DAC1EN` reader - DAC1 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_R as DAC1EN_R;
///Field `LPDMA1EN` reader - LPDMA1 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_R as LPDMA1EN_R;
///Field `ADF1EN` reader - ADF1 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_R as ADF1EN_R;
///Field `GTZC2EN` reader - GTZC2 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_R as GTZC2EN_R;
///Field `SRAM4EN` reader - SRAM4 clock enable This bit is set and reset by software.
pub use LPGPIO1EN_R as SRAM4EN_R;
///Field `PWREN` writer - PWR clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_W as PWREN_W;
///Field `ADC4EN` writer - ADC4 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_W as ADC4EN_W;
///Field `DAC1EN` writer - DAC1 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_W as DAC1EN_W;
///Field `LPDMA1EN` writer - LPDMA1 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_W as LPDMA1EN_W;
///Field `ADF1EN` writer - ADF1 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_W as ADF1EN_W;
///Field `GTZC2EN` writer - GTZC2 clock enable This bit is set and cleared by software.
pub use LPGPIO1EN_W as GTZC2EN_W;
///Field `SRAM4EN` writer - SRAM4 clock enable This bit is set and reset by software.
pub use LPGPIO1EN_W as SRAM4EN_W;
impl R {
    ///Bit 0 - LPGPIO1 enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1en(&self) -> LPGPIO1EN_R {
        LPGPIO1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PWR clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn adc4en(&self) -> ADC4EN_R {
        ADC4EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAC1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LPDMA1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpdma1en(&self) -> LPDMA1EN_R {
        LPDMA1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADF1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn adf1en(&self) -> ADF1EN_R {
        ADF1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - GTZC2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gtzc2en(&self) -> GTZC2EN_R {
        GTZC2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 31 - SRAM4 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn sram4en(&self) -> SRAM4EN_R {
        SRAM4EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("lpgpio1en", &self.lpgpio1en())
            .field("pwren", &self.pwren())
            .field("adc4en", &self.adc4en())
            .field("dac1en", &self.dac1en())
            .field("lpdma1en", &self.lpdma1en())
            .field("adf1en", &self.adf1en())
            .field("gtzc2en", &self.gtzc2en())
            .field("sram4en", &self.sram4en())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPGPIO1 enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1en(&mut self) -> LPGPIO1EN_W<AHB3ENRrs> {
        LPGPIO1EN_W::new(self, 0)
    }
    ///Bit 2 - PWR clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<AHB3ENRrs> {
        PWREN_W::new(self, 2)
    }
    ///Bit 5 - ADC4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn adc4en(&mut self) -> ADC4EN_W<AHB3ENRrs> {
        ADC4EN_W::new(self, 5)
    }
    ///Bit 6 - DAC1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W<AHB3ENRrs> {
        DAC1EN_W::new(self, 6)
    }
    ///Bit 9 - LPDMA1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpdma1en(&mut self) -> LPDMA1EN_W<AHB3ENRrs> {
        LPDMA1EN_W::new(self, 9)
    }
    ///Bit 10 - ADF1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn adf1en(&mut self) -> ADF1EN_W<AHB3ENRrs> {
        ADF1EN_W::new(self, 10)
    }
    ///Bit 12 - GTZC2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gtzc2en(&mut self) -> GTZC2EN_W<AHB3ENRrs> {
        GTZC2EN_W::new(self, 12)
    }
    ///Bit 31 - SRAM4 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn sram4en(&mut self) -> SRAM4EN_W<AHB3ENRrs> {
        SRAM4EN_W::new(self, 31)
    }
}
/**RCC AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:AHB3ENR)*/
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3enr::R`](R) reader structure
impl crate::Readable for AHB3ENRrs {}
///`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENR to value 0x8000_0000
impl crate::Resettable for AHB3ENRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
