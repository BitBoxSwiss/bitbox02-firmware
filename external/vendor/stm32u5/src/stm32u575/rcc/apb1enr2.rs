///Register `APB1ENR2` reader
pub type R = crate::R<APB1ENR2rs>;
///Register `APB1ENR2` writer
pub type W = crate::W<APB1ENR2rs>;
/**I2C4 clock enable This bit is set and cleared by software

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4EN {
    ///0: Peripheral clock disabled
    Disabled = 0,
    ///1: Peripheral clock enabled
    Enabled = 1,
}
impl From<I2C4EN> for bool {
    #[inline(always)]
    fn from(variant: I2C4EN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C4EN` reader - I2C4 clock enable This bit is set and cleared by software
pub type I2C4EN_R = crate::BitReader<I2C4EN>;
impl I2C4EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C4EN {
        match self.bits {
            false => I2C4EN::Disabled,
            true => I2C4EN::Enabled,
        }
    }
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C4EN::Disabled
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C4EN::Enabled
    }
}
///Field `I2C4EN` writer - I2C4 clock enable This bit is set and cleared by software
pub type I2C4EN_W<'a, REG> = crate::BitWriter<'a, REG, I2C4EN>;
impl<'a, REG> I2C4EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4EN::Disabled)
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4EN::Enabled)
    }
}
///Field `LPTIM2EN` reader - LPTIM2 clock enable This bit is set and cleared by software.
pub use I2C4EN_R as LPTIM2EN_R;
///Field `I2C5EN` reader - I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4EN_R as I2C5EN_R;
///Field `I2C6EN` reader - I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4EN_R as I2C6EN_R;
///Field `FDCAN1EN` reader - FDCAN1 clock enable This bit is set and cleared by software.
pub use I2C4EN_R as FDCAN1EN_R;
///Field `UCPD1EN` reader - UCPD1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4EN_R as UCPD1EN_R;
///Field `LPTIM2EN` writer - LPTIM2 clock enable This bit is set and cleared by software.
pub use I2C4EN_W as LPTIM2EN_W;
///Field `I2C5EN` writer - I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4EN_W as I2C5EN_W;
///Field `I2C6EN` writer - I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4EN_W as I2C6EN_W;
///Field `FDCAN1EN` writer - FDCAN1 clock enable This bit is set and cleared by software.
pub use I2C4EN_W as FDCAN1EN_W;
///Field `UCPD1EN` writer - UCPD1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4EN_W as UCPD1EN_W;
impl R {
    ///Bit 1 - I2C4 clock enable This bit is set and cleared by software
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5en(&self) -> I2C5EN_R {
        I2C5EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6en(&self) -> I2C6EN_R {
        I2C6EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fdcan1en(&self) -> FDCAN1EN_R {
        FDCAN1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR2")
            .field("i2c4en", &self.i2c4en())
            .field("lptim2en", &self.lptim2en())
            .field("i2c5en", &self.i2c5en())
            .field("i2c6en", &self.i2c6en())
            .field("fdcan1en", &self.fdcan1en())
            .field("ucpd1en", &self.ucpd1en())
            .finish()
    }
}
impl W {
    ///Bit 1 - I2C4 clock enable This bit is set and cleared by software
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W<APB1ENR2rs> {
        I2C4EN_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
    ///Bit 6 - I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5en(&mut self) -> I2C5EN_W<APB1ENR2rs> {
        I2C5EN_W::new(self, 6)
    }
    ///Bit 7 - I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6en(&mut self) -> I2C6EN_W<APB1ENR2rs> {
        I2C6EN_W::new(self, 7)
    }
    ///Bit 9 - FDCAN1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fdcan1en(&mut self) -> FDCAN1EN_W<APB1ENR2rs> {
        FDCAN1EN_W::new(self, 9)
    }
    ///Bit 23 - UCPD1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<APB1ENR2rs> {
        UCPD1EN_W::new(self, 23)
    }
}
/**RCC APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:APB1ENR2)*/
pub struct APB1ENR2rs;
impl crate::RegisterSpec for APB1ENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr2::R`](R) reader structure
impl crate::Readable for APB1ENR2rs {}
///`write(|w| ..)` method takes [`apb1enr2::W`](W) writer structure
impl crate::Writable for APB1ENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR2 to value 0
impl crate::Resettable for APB1ENR2rs {}
