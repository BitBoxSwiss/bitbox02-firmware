///Register `SRDAMR` reader
pub type R = crate::R<SRDAMRrs>;
///Register `SRDAMR` writer
pub type W = crate::W<SRDAMRrs>;
/**SPI3 autonomous mode enable in Stop 0,1, 2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI3AMEN {
    ///0: Peripheral autonomous mode disabled during Stop 0/1/2 mode
    Disabled = 0,
    ///1: Peripheral autonomous mode enabled during Stop 0/1/2 mode
    Enabled = 1,
}
impl From<SPI3AMEN> for bool {
    #[inline(always)]
    fn from(variant: SPI3AMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI3AMEN` reader - SPI3 autonomous mode enable in Stop 0,1, 2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI3AMEN_R = crate::BitReader<SPI3AMEN>;
impl SPI3AMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI3AMEN {
        match self.bits {
            false => SPI3AMEN::Disabled,
            true => SPI3AMEN::Enabled,
        }
    }
    ///Peripheral autonomous mode disabled during Stop 0/1/2 mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI3AMEN::Disabled
    }
    ///Peripheral autonomous mode enabled during Stop 0/1/2 mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI3AMEN::Enabled
    }
}
///Field `SPI3AMEN` writer - SPI3 autonomous mode enable in Stop 0,1, 2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI3AMEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI3AMEN>;
impl<'a, REG> SPI3AMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral autonomous mode disabled during Stop 0/1/2 mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3AMEN::Disabled)
    }
    ///Peripheral autonomous mode enabled during Stop 0/1/2 mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3AMEN::Enabled)
    }
}
///Field `LPUART1AMEN` reader - LPUART1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as LPUART1AMEN_R;
///Field `I2C3AMEN` reader - I2C3 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as I2C3AMEN_R;
///Field `LPTIM1AMEN` reader - LPTIM1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as LPTIM1AMEN_R;
///Field `LPTIM3AMEN` reader - LPTIM3 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as LPTIM3AMEN_R;
///Field `LPTIM4AMEN` reader - LPTIM4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as LPTIM4AMEN_R;
///Field `OPAMPAMEN` reader - OPAMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_R as OPAMPAMEN_R;
///Field `COMPAMEN` reader - COMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_R as COMPAMEN_R;
///Field `VREFAMEN` reader - VREFBUF autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_R as VREFAMEN_R;
///Field `RTCAPBAMEN` reader - RTC and TAMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as RTCAPBAMEN_R;
///Field `ADC4AMEN` reader - ADC4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as ADC4AMEN_R;
///Field `LPGPIO1AMEN` reader - LPGPIO1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_R as LPGPIO1AMEN_R;
///Field `DAC1AMEN` reader - DAC1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as DAC1AMEN_R;
///Field `LPDMA1AMEN` reader - LPDMA1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as LPDMA1AMEN_R;
///Field `ADF1AMEN` reader - ADF1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_R as ADF1AMEN_R;
///Field `SRAM4AMEN` reader - SRAM4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_R as SRAM4AMEN_R;
///Field `LPUART1AMEN` writer - LPUART1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as LPUART1AMEN_W;
///Field `I2C3AMEN` writer - I2C3 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as I2C3AMEN_W;
///Field `LPTIM1AMEN` writer - LPTIM1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as LPTIM1AMEN_W;
///Field `LPTIM3AMEN` writer - LPTIM3 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as LPTIM3AMEN_W;
///Field `LPTIM4AMEN` writer - LPTIM4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as LPTIM4AMEN_W;
///Field `OPAMPAMEN` writer - OPAMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_W as OPAMPAMEN_W;
///Field `COMPAMEN` writer - COMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_W as COMPAMEN_W;
///Field `VREFAMEN` writer - VREFBUF autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_W as VREFAMEN_W;
///Field `RTCAPBAMEN` writer - RTC and TAMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as RTCAPBAMEN_W;
///Field `ADC4AMEN` writer - ADC4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as ADC4AMEN_W;
///Field `LPGPIO1AMEN` writer - LPGPIO1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_W as LPGPIO1AMEN_W;
///Field `DAC1AMEN` writer - DAC1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as DAC1AMEN_W;
///Field `LPDMA1AMEN` writer - LPDMA1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as LPDMA1AMEN_W;
///Field `ADF1AMEN` writer - ADF1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SPI3AMEN_W as ADF1AMEN_W;
///Field `SRAM4AMEN` writer - SRAM4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
pub use SPI3AMEN_W as SRAM4AMEN_W;
impl R {
    ///Bit 5 - SPI3 autonomous mode enable in Stop 0,1, 2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi3amen(&self) -> SPI3AMEN_R {
        SPI3AMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c3amen(&self) -> I2C3AMEN_R {
        I2C3AMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim1amen(&self) -> LPTIM1AMEN_R {
        LPTIM1AMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim4amen(&self) -> LPTIM4AMEN_R {
        LPTIM4AMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn opampamen(&self) -> OPAMPAMEN_R {
        OPAMPAMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn compamen(&self) -> COMPAMEN_R {
        COMPAMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn rtcapbamen(&self) -> RTCAPBAMEN_R {
        RTCAPBAMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 25 - ADC4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4amen(&self) -> ADC4AMEN_R {
        ADC4AMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LPGPIO1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1amen(&self) -> LPGPIO1AMEN_R {
        LPGPIO1AMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DAC1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn dac1amen(&self) -> DAC1AMEN_R {
        DAC1AMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - LPDMA1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpdma1amen(&self) -> LPDMA1AMEN_R {
        LPDMA1AMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ADF1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adf1amen(&self) -> ADF1AMEN_R {
        ADF1AMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - SRAM4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn sram4amen(&self) -> SRAM4AMEN_R {
        SRAM4AMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRDAMR")
            .field("spi3amen", &self.spi3amen())
            .field("lpuart1amen", &self.lpuart1amen())
            .field("i2c3amen", &self.i2c3amen())
            .field("lptim1amen", &self.lptim1amen())
            .field("lptim3amen", &self.lptim3amen())
            .field("lptim4amen", &self.lptim4amen())
            .field("opampamen", &self.opampamen())
            .field("compamen", &self.compamen())
            .field("vrefamen", &self.vrefamen())
            .field("rtcapbamen", &self.rtcapbamen())
            .field("adc4amen", &self.adc4amen())
            .field("lpgpio1amen", &self.lpgpio1amen())
            .field("dac1amen", &self.dac1amen())
            .field("lpdma1amen", &self.lpdma1amen())
            .field("adf1amen", &self.adf1amen())
            .field("sram4amen", &self.sram4amen())
            .finish()
    }
}
impl W {
    ///Bit 5 - SPI3 autonomous mode enable in Stop 0,1, 2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi3amen(&mut self) -> SPI3AMEN_W<SRDAMRrs> {
        SPI3AMEN_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W<SRDAMRrs> {
        LPUART1AMEN_W::new(self, 6)
    }
    ///Bit 7 - I2C3 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c3amen(&mut self) -> I2C3AMEN_W<SRDAMRrs> {
        I2C3AMEN_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim1amen(&mut self) -> LPTIM1AMEN_W<SRDAMRrs> {
        LPTIM1AMEN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W<SRDAMRrs> {
        LPTIM3AMEN_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim4amen(&mut self) -> LPTIM4AMEN_W<SRDAMRrs> {
        LPTIM4AMEN_W::new(self, 13)
    }
    ///Bit 14 - OPAMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn opampamen(&mut self) -> OPAMPAMEN_W<SRDAMRrs> {
        OPAMPAMEN_W::new(self, 14)
    }
    ///Bit 15 - COMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn compamen(&mut self) -> COMPAMEN_W<SRDAMRrs> {
        COMPAMEN_W::new(self, 15)
    }
    ///Bit 20 - VREFBUF autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefamen(&mut self) -> VREFAMEN_W<SRDAMRrs> {
        VREFAMEN_W::new(self, 20)
    }
    ///Bit 21 - RTC and TAMP autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn rtcapbamen(&mut self) -> RTCAPBAMEN_W<SRDAMRrs> {
        RTCAPBAMEN_W::new(self, 21)
    }
    ///Bit 25 - ADC4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4amen(&mut self) -> ADC4AMEN_W<SRDAMRrs> {
        ADC4AMEN_W::new(self, 25)
    }
    ///Bit 26 - LPGPIO1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1amen(&mut self) -> LPGPIO1AMEN_W<SRDAMRrs> {
        LPGPIO1AMEN_W::new(self, 26)
    }
    ///Bit 27 - DAC1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn dac1amen(&mut self) -> DAC1AMEN_W<SRDAMRrs> {
        DAC1AMEN_W::new(self, 27)
    }
    ///Bit 28 - LPDMA1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpdma1amen(&mut self) -> LPDMA1AMEN_W<SRDAMRrs> {
        LPDMA1AMEN_W::new(self, 28)
    }
    ///Bit 29 - ADF1 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adf1amen(&mut self) -> ADF1AMEN_W<SRDAMRrs> {
        ADF1AMEN_W::new(self, 29)
    }
    ///Bit 31 - SRAM4 autonomous mode enable in Stop 0/1/2 mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn sram4amen(&mut self) -> SRAM4AMEN_W<SRDAMRrs> {
        SRAM4AMEN_W::new(self, 31)
    }
}
/**RCC SmartRun domain peripheral autonomous mode register

You can [`read`](crate::Reg::read) this register and get [`srdamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srdamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:SRDAMR)*/
pub struct SRDAMRrs;
impl crate::RegisterSpec for SRDAMRrs {
    type Ux = u32;
}
///`read()` method returns [`srdamr::R`](R) reader structure
impl crate::Readable for SRDAMRrs {}
///`write(|w| ..)` method takes [`srdamr::W`](W) writer structure
impl crate::Writable for SRDAMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRDAMR to value 0
impl crate::Resettable for SRDAMRrs {}
