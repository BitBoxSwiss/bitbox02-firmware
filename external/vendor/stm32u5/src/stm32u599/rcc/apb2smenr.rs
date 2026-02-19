///Register `APB2SMENR` reader
pub type R = crate::R<APB2SMENRrs>;
///Register `APB2SMENR` writer
pub type W = crate::W<APB2SMENRrs>;
/**TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SMEN {
    ///0: Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    Enabled = 1,
}
impl From<TIM1SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1SMEN` reader - TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM1SMEN_R = crate::BitReader<TIM1SMEN>;
impl TIM1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1SMEN {
        match self.bits {
            false => TIM1SMEN::Disabled,
            true => TIM1SMEN::Enabled,
        }
    }
    ///Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1SMEN::Disabled
    }
    ///Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1SMEN::Enabled
    }
}
///Field `TIM1SMEN` writer - TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1SMEN>;
impl<'a, REG> TIM1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SMEN::Disabled)
    }
    ///Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SMEN::Enabled)
    }
}
///Field `SPI1SMEN` reader - SPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use TIM1SMEN_R as SPI1SMEN_R;
///Field `TIM8SMEN` reader - TIM8 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_R as TIM8SMEN_R;
///Field `USART1SMEN` reader - USART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use TIM1SMEN_R as USART1SMEN_R;
///Field `TIM15SMEN` reader - TIM15 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_R as TIM15SMEN_R;
///Field `TIM16SMEN` reader - TIM16 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_R as TIM16SMEN_R;
///Field `TIM17SMEN` reader - TIM17 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_R as TIM17SMEN_R;
///Field `SAI1SMEN` reader - SAI1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_R as SAI1SMEN_R;
///Field `SAI2SMEN` reader - SAI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_R as SAI2SMEN_R;
///Field `USBSMEN` reader - USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_R as USBSMEN_R;
///Field `GFXTIMSMEN` reader - GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_R as GFXTIMSMEN_R;
///Field `LTDCSMEN` reader - LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_R as LTDCSMEN_R;
///Field `DSISMEN` reader - DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_R as DSISMEN_R;
///Field `SPI1SMEN` writer - SPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use TIM1SMEN_W as SPI1SMEN_W;
///Field `TIM8SMEN` writer - TIM8 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_W as TIM8SMEN_W;
///Field `USART1SMEN` writer - USART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub use TIM1SMEN_W as USART1SMEN_W;
///Field `TIM15SMEN` writer - TIM15 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_W as TIM15SMEN_W;
///Field `TIM16SMEN` writer - TIM16 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_W as TIM16SMEN_W;
///Field `TIM17SMEN` writer - TIM17 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_W as TIM17SMEN_W;
///Field `SAI1SMEN` writer - SAI1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use TIM1SMEN_W as SAI1SMEN_W;
///Field `SAI2SMEN` writer - SAI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_W as SAI2SMEN_W;
///Field `USBSMEN` writer - USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_W as USBSMEN_W;
///Field `GFXTIMSMEN` writer - GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_W as GFXTIMSMEN_W;
///Field `LTDCSMEN` writer - LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_W as LTDCSMEN_W;
///Field `DSISMEN` writer - DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1SMEN_W as DSISMEN_W;
impl R {
    ///Bit 11 - TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - SAI1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sai2smen(&self) -> SAI2SMEN_R {
        SAI2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxtimsmen(&self) -> GFXTIMSMEN_R {
        GFXTIMSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ltdcsmen(&self) -> LTDCSMEN_R {
        LTDCSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dsismen(&self) -> DSISMEN_R {
        DSISMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2SMENR")
            .field("tim1smen", &self.tim1smen())
            .field("spi1smen", &self.spi1smen())
            .field("tim8smen", &self.tim8smen())
            .field("usart1smen", &self.usart1smen())
            .field("tim15smen", &self.tim15smen())
            .field("tim16smen", &self.tim16smen())
            .field("tim17smen", &self.tim17smen())
            .field("sai1smen", &self.sai1smen())
            .field("sai2smen", &self.sai2smen())
            .field("usbsmen", &self.usbsmen())
            .field("gfxtimsmen", &self.gfxtimsmen())
            .field("ltdcsmen", &self.ltdcsmen())
            .field("dsismen", &self.dsismen())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<APB2SMENRrs> {
        TIM1SMEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    ///Bit 13 - TIM8 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W<APB2SMENRrs> {
        TIM8SMEN_W::new(self, 13)
    }
    ///Bit 14 - USART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    ///Bit 16 - TIM15 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<APB2SMENRrs> {
        TIM15SMEN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<APB2SMENRrs> {
        TIM16SMEN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<APB2SMENRrs> {
        TIM17SMEN_W::new(self, 18)
    }
    ///Bit 21 - SAI1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W<APB2SMENRrs> {
        SAI1SMEN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sai2smen(&mut self) -> SAI2SMEN_W<APB2SMENRrs> {
        SAI2SMEN_W::new(self, 22)
    }
    ///Bit 24 - USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W<APB2SMENRrs> {
        USBSMEN_W::new(self, 24)
    }
    ///Bit 25 - GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxtimsmen(&mut self) -> GFXTIMSMEN_W<APB2SMENRrs> {
        GFXTIMSMEN_W::new(self, 25)
    }
    ///Bit 26 - LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ltdcsmen(&mut self) -> LTDCSMEN_W<APB2SMENRrs> {
        LTDCSMEN_W::new(self, 26)
    }
    ///Bit 27 - DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dsismen(&mut self) -> DSISMEN_W<APB2SMENRrs> {
        DSISMEN_W::new(self, 27)
    }
}
/**RCC APB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:APB2SMENR)*/
pub struct APB2SMENRrs;
impl crate::RegisterSpec for APB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2smenr::R`](R) reader structure
impl crate::Readable for APB2SMENRrs {}
///`write(|w| ..)` method takes [`apb2smenr::W`](W) writer structure
impl crate::Writable for APB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2SMENR to value 0xffff_ffff
impl crate::Resettable for APB2SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
