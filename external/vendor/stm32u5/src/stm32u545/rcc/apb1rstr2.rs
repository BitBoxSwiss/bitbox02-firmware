///Register `APB1RSTR2` reader
pub type R = crate::R<APB1RSTR2rs>;
///Register `APB1RSTR2` writer
pub type W = crate::W<APB1RSTR2rs>;
/**I2C4 reset This bit is set and cleared by software

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<I2C4RST> for bool {
    #[inline(always)]
    fn from(variant: I2C4RST) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C4RST` reader - I2C4 reset This bit is set and cleared by software
pub type I2C4RST_R = crate::BitReader<I2C4RST>;
impl I2C4RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C4RST {
        match self.bits {
            false => I2C4RST::NoEffect,
            true => I2C4RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == I2C4RST::NoEffect
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C4RST::Reset
    }
}
///Field `I2C4RST` writer - I2C4 reset This bit is set and cleared by software
pub type I2C4RST_W<'a, REG> = crate::BitWriter<'a, REG, I2C4RST>;
impl<'a, REG> I2C4RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4RST::NoEffect)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4RST::Reset)
    }
}
///Field `LPTIM2RST` reader - LPTIM2 reset This bit is set and cleared by software.
pub use I2C4RST_R as LPTIM2RST_R;
///Field `I2C5RST` reader - I2C5 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4RST_R as I2C5RST_R;
///Field `I2C6RST` reader - I2C6 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4RST_R as I2C6RST_R;
///Field `FDCAN1RST` reader - FDCAN1 reset This bit is set and cleared by software.
pub use I2C4RST_R as FDCAN1RST_R;
///Field `UCPD1RST` reader - UCPD1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4RST_R as UCPD1RST_R;
///Field `LPTIM2RST` writer - LPTIM2 reset This bit is set and cleared by software.
pub use I2C4RST_W as LPTIM2RST_W;
///Field `I2C5RST` writer - I2C5 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4RST_W as I2C5RST_W;
///Field `I2C6RST` writer - I2C6 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4RST_W as I2C6RST_W;
///Field `FDCAN1RST` writer - FDCAN1 reset This bit is set and cleared by software.
pub use I2C4RST_W as FDCAN1RST_W;
///Field `UCPD1RST` writer - UCPD1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use I2C4RST_W as UCPD1RST_W;
impl R {
    ///Bit 1 - I2C4 reset This bit is set and cleared by software
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I2C5 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5rst(&self) -> I2C5RST_R {
        I2C5RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C6 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6rst(&self) -> I2C6RST_R {
        I2C6RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn fdcan1rst(&self) -> FDCAN1RST_R {
        FDCAN1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR2")
            .field("i2c4rst", &self.i2c4rst())
            .field("lptim2rst", &self.lptim2rst())
            .field("i2c5rst", &self.i2c5rst())
            .field("i2c6rst", &self.i2c6rst())
            .field("fdcan1rst", &self.fdcan1rst())
            .field("ucpd1rst", &self.ucpd1rst())
            .finish()
    }
}
impl W {
    ///Bit 1 - I2C4 reset This bit is set and cleared by software
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<APB1RSTR2rs> {
        I2C4RST_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<APB1RSTR2rs> {
        LPTIM2RST_W::new(self, 5)
    }
    ///Bit 6 - I2C5 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5rst(&mut self) -> I2C5RST_W<APB1RSTR2rs> {
        I2C5RST_W::new(self, 6)
    }
    ///Bit 7 - I2C6 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6rst(&mut self) -> I2C6RST_W<APB1RSTR2rs> {
        I2C6RST_W::new(self, 7)
    }
    ///Bit 9 - FDCAN1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn fdcan1rst(&mut self) -> FDCAN1RST_W<APB1RSTR2rs> {
        FDCAN1RST_W::new(self, 9)
    }
    ///Bit 23 - UCPD1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<APB1RSTR2rs> {
        UCPD1RST_W::new(self, 23)
    }
}
/**RCC APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:APB1RSTR2)*/
pub struct APB1RSTR2rs;
impl crate::RegisterSpec for APB1RSTR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr2::R`](R) reader structure
impl crate::Readable for APB1RSTR2rs {}
///`write(|w| ..)` method takes [`apb1rstr2::W`](W) writer structure
impl crate::Writable for APB1RSTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR2 to value 0
impl crate::Resettable for APB1RSTR2rs {}
