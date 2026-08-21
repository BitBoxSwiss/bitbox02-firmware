///Register `APB3SMENR` reader
pub type R = crate::R<APB3SMENRrs>;
///Register `APB3SMENR` writer
pub type W = crate::W<APB3SMENRrs>;
/**SYSCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN {
    ///0: Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    Enabled = 1,
}
impl From<SYSCFGSMEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGSMEN` reader - SYSCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type SYSCFGSMEN_R = crate::BitReader<SYSCFGSMEN>;
impl SYSCFGSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGSMEN {
        match self.bits {
            false => SYSCFGSMEN::Disabled,
            true => SYSCFGSMEN::Enabled,
        }
    }
    ///Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGSMEN::Disabled
    }
    ///Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGSMEN::Enabled
    }
}
///Field `SYSCFGSMEN` writer - SYSCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGSMEN>;
impl<'a, REG> SYSCFGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Disabled)
    }
    ///Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Enabled)
    }
}
///Field `SPI3SMEN` reader - SPI3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_R as SPI3SMEN_R;
///Field `LPUART1SMEN` reader - LPUART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_R as LPUART1SMEN_R;
///Field `I2C3SMEN` reader - I2C3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_R as I2C3SMEN_R;
///Field `LPTIM1SMEN` reader - LPTIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_R as LPTIM1SMEN_R;
///Field `LPTIM3SMEN` reader - LPTIM3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_R as LPTIM3SMEN_R;
///Field `LPTIM4SMEN` reader - LPTIM4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_R as LPTIM4SMEN_R;
///Field `OPAMPSMEN` reader - OPAMP clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use SYSCFGSMEN_R as OPAMPSMEN_R;
///Field `COMPSMEN` reader - COMP clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use SYSCFGSMEN_R as COMPSMEN_R;
///Field `VREFSMEN` reader - VREFBUF clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use SYSCFGSMEN_R as VREFSMEN_R;
///Field `RTCAPBSMEN` reader - RTC and TAMP APB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_R as RTCAPBSMEN_R;
///Field `SPI3SMEN` writer - SPI3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_W as SPI3SMEN_W;
///Field `LPUART1SMEN` writer - LPUART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_W as LPUART1SMEN_W;
///Field `I2C3SMEN` writer - I2C3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_W as I2C3SMEN_W;
///Field `LPTIM1SMEN` writer - LPTIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_W as LPTIM1SMEN_W;
///Field `LPTIM3SMEN` writer - LPTIM3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_W as LPTIM3SMEN_W;
///Field `LPTIM4SMEN` writer - LPTIM4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_W as LPTIM4SMEN_W;
///Field `OPAMPSMEN` writer - OPAMP clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use SYSCFGSMEN_W as OPAMPSMEN_W;
///Field `COMPSMEN` writer - COMP clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use SYSCFGSMEN_W as COMPSMEN_W;
///Field `VREFSMEN` writer - VREFBUF clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use SYSCFGSMEN_W as VREFSMEN_W;
///Field `RTCAPBSMEN` writer - RTC and TAMP APB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use SYSCFGSMEN_W as RTCAPBSMEN_W;
impl R {
    ///Bit 1 - SYSCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim4smen(&self) -> LPTIM4SMEN_R {
        LPTIM4SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn compsmen(&self) -> COMPSMEN_R {
        COMPSMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefsmen(&self) -> VREFSMEN_R {
        VREFSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP APB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3SMENR")
            .field("syscfgsmen", &self.syscfgsmen())
            .field("spi3smen", &self.spi3smen())
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c3smen", &self.i2c3smen())
            .field("lptim1smen", &self.lptim1smen())
            .field("lptim3smen", &self.lptim3smen())
            .field("lptim4smen", &self.lptim4smen())
            .field("opampsmen", &self.opampsmen())
            .field("compsmen", &self.compsmen())
            .field("vrefsmen", &self.vrefsmen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<APB3SMENRrs> {
        SYSCFGSMEN_W::new(self, 1)
    }
    ///Bit 5 - SPI3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<APB3SMENRrs> {
        SPI3SMEN_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<APB3SMENRrs> {
        LPUART1SMEN_W::new(self, 6)
    }
    ///Bit 7 - I2C3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<APB3SMENRrs> {
        I2C3SMEN_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<APB3SMENRrs> {
        LPTIM1SMEN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W<APB3SMENRrs> {
        LPTIM3SMEN_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim4smen(&mut self) -> LPTIM4SMEN_W<APB3SMENRrs> {
        LPTIM4SMEN_W::new(self, 13)
    }
    ///Bit 14 - OPAMP clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<APB3SMENRrs> {
        OPAMPSMEN_W::new(self, 14)
    }
    ///Bit 15 - COMP clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn compsmen(&mut self) -> COMPSMEN_W<APB3SMENRrs> {
        COMPSMEN_W::new(self, 15)
    }
    ///Bit 20 - VREFBUF clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefsmen(&mut self) -> VREFSMEN_W<APB3SMENRrs> {
        VREFSMEN_W::new(self, 20)
    }
    ///Bit 21 - RTC and TAMP APB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<APB3SMENRrs> {
        RTCAPBSMEN_W::new(self, 21)
    }
}
/**RCC APB3 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`apb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:APB3SMENR)*/
pub struct APB3SMENRrs;
impl crate::RegisterSpec for APB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3smenr::R`](R) reader structure
impl crate::Readable for APB3SMENRrs {}
///`write(|w| ..)` method takes [`apb3smenr::W`](W) writer structure
impl crate::Writable for APB3SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3SMENR to value 0xffff_ffff
impl crate::Resettable for APB3SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
