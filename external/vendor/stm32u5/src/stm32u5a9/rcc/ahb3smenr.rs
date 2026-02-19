///Register `AHB3SMENR` reader
pub type R = crate::R<AHB3SMENRrs>;
///Register `AHB3SMENR` writer
pub type W = crate::W<AHB3SMENRrs>;
/**LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPGPIO1SMEN {
    ///0: Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    Enabled = 1,
}
impl From<LPGPIO1SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPGPIO1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPGPIO1SMEN` reader - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.
pub type LPGPIO1SMEN_R = crate::BitReader<LPGPIO1SMEN>;
impl LPGPIO1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPGPIO1SMEN {
        match self.bits {
            false => LPGPIO1SMEN::Disabled,
            true => LPGPIO1SMEN::Enabled,
        }
    }
    ///Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPGPIO1SMEN::Disabled
    }
    ///Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPGPIO1SMEN::Enabled
    }
}
///Field `LPGPIO1SMEN` writer - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.
pub type LPGPIO1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPGPIO1SMEN>;
impl<'a, REG> LPGPIO1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPGPIO1SMEN::Disabled)
    }
    ///Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPGPIO1SMEN::Enabled)
    }
}
///Field `PWRSMEN` reader - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use LPGPIO1SMEN_R as PWRSMEN_R;
///Field `ADC4SMEN` reader - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use LPGPIO1SMEN_R as ADC4SMEN_R;
///Field `DAC1SMEN` reader - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use LPGPIO1SMEN_R as DAC1SMEN_R;
///Field `LPDMA1SMEN` reader - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use LPGPIO1SMEN_R as LPDMA1SMEN_R;
///Field `ADF1SMEN` reader - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use LPGPIO1SMEN_R as ADF1SMEN_R;
///Field `GTZC2SMEN` reader - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use LPGPIO1SMEN_R as GTZC2SMEN_R;
///Field `SRAM4SMEN` reader - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use LPGPIO1SMEN_R as SRAM4SMEN_R;
///Field `PWRSMEN` writer - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use LPGPIO1SMEN_W as PWRSMEN_W;
///Field `ADC4SMEN` writer - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use LPGPIO1SMEN_W as ADC4SMEN_W;
///Field `DAC1SMEN` writer - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use LPGPIO1SMEN_W as DAC1SMEN_W;
///Field `LPDMA1SMEN` writer - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use LPGPIO1SMEN_W as LPDMA1SMEN_W;
///Field `ADF1SMEN` writer - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use LPGPIO1SMEN_W as ADF1SMEN_W;
///Field `GTZC2SMEN` writer - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use LPGPIO1SMEN_W as GTZC2SMEN_W;
///Field `SRAM4SMEN` writer - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use LPGPIO1SMEN_W as SRAM4SMEN_W;
impl R {
    ///Bit 0 - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1smen(&self) -> LPGPIO1SMEN_R {
        LPGPIO1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4smen(&self) -> ADC4SMEN_R {
        ADC4SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpdma1smen(&self) -> LPDMA1SMEN_R {
        LPDMA1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adf1smen(&self) -> ADF1SMEN_R {
        ADF1SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gtzc2smen(&self) -> GTZC2SMEN_R {
        GTZC2SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 31 - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sram4smen(&self) -> SRAM4SMEN_R {
        SRAM4SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3SMENR")
            .field("lpgpio1smen", &self.lpgpio1smen())
            .field("pwrsmen", &self.pwrsmen())
            .field("adc4smen", &self.adc4smen())
            .field("dac1smen", &self.dac1smen())
            .field("lpdma1smen", &self.lpdma1smen())
            .field("adf1smen", &self.adf1smen())
            .field("gtzc2smen", &self.gtzc2smen())
            .field("sram4smen", &self.sram4smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1smen(&mut self) -> LPGPIO1SMEN_W<AHB3SMENRrs> {
        LPGPIO1SMEN_W::new(self, 0)
    }
    ///Bit 2 - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<AHB3SMENRrs> {
        PWRSMEN_W::new(self, 2)
    }
    ///Bit 5 - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4smen(&mut self) -> ADC4SMEN_W<AHB3SMENRrs> {
        ADC4SMEN_W::new(self, 5)
    }
    ///Bit 6 - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<AHB3SMENRrs> {
        DAC1SMEN_W::new(self, 6)
    }
    ///Bit 9 - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpdma1smen(&mut self) -> LPDMA1SMEN_W<AHB3SMENRrs> {
        LPDMA1SMEN_W::new(self, 9)
    }
    ///Bit 10 - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adf1smen(&mut self) -> ADF1SMEN_W<AHB3SMENRrs> {
        ADF1SMEN_W::new(self, 10)
    }
    ///Bit 12 - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gtzc2smen(&mut self) -> GTZC2SMEN_W<AHB3SMENRrs> {
        GTZC2SMEN_W::new(self, 12)
    }
    ///Bit 31 - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sram4smen(&mut self) -> SRAM4SMEN_W<AHB3SMENRrs> {
        SRAM4SMEN_W::new(self, 31)
    }
}
/**RCC AHB3 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RCC:AHB3SMENR)*/
pub struct AHB3SMENRrs;
impl crate::RegisterSpec for AHB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3smenr::R`](R) reader structure
impl crate::Readable for AHB3SMENRrs {}
///`write(|w| ..)` method takes [`ahb3smenr::W`](W) writer structure
impl crate::Writable for AHB3SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3SMENR to value 0xffff_ffff
impl crate::Resettable for AHB3SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
