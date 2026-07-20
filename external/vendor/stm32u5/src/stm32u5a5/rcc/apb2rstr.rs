///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**TIM1 reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<TIM1RST> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1RST` reader - TIM1 reset This bit is set and cleared by software.
pub type TIM1RST_R = crate::BitReader<TIM1RST>;
impl TIM1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1RST {
        match self.bits {
            false => TIM1RST::NoEffect,
            true => TIM1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIM1RST::NoEffect
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST::Reset
    }
}
///Field `TIM1RST` writer - TIM1 reset This bit is set and cleared by software.
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM1RST>;
impl<'a, REG> TIM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::NoEffect)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::Reset)
    }
}
///Field `SPI1RST` reader - SPI1 reset This bit is set and cleared by software.
pub use TIM1RST_R as SPI1RST_R;
///Field `TIM8RST` reader - TIM8 reset This bit is set and cleared by software.
pub use TIM1RST_R as TIM8RST_R;
///Field `USART1RST` reader - USART1 reset This bit is set and cleared by software.
pub use TIM1RST_R as USART1RST_R;
///Field `TIM15RST` reader - TIM15 reset This bit is set and cleared by software.
pub use TIM1RST_R as TIM15RST_R;
///Field `TIM16RST` reader - TIM16 reset This bit is set and cleared by software.
pub use TIM1RST_R as TIM16RST_R;
///Field `TIM17RST` reader - TIM17 reset This bit is set and cleared by software.
pub use TIM1RST_R as TIM17RST_R;
///Field `SAI1RST` reader - SAI1 reset This bit is set and cleared by software.
pub use TIM1RST_R as SAI1RST_R;
///Field `SAI2RST` reader - SAI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_R as SAI2RST_R;
///Field `USBRST` reader - USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_R as USBRST_R;
///Field `GFXTIMRST` reader - GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_R as GFXTIMRST_R;
///Field `LTDCRST` reader - LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_R as LTDCRST_R;
///Field `DSIRST` reader - DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_R as DSIRST_R;
///Field `SPI1RST` writer - SPI1 reset This bit is set and cleared by software.
pub use TIM1RST_W as SPI1RST_W;
///Field `TIM8RST` writer - TIM8 reset This bit is set and cleared by software.
pub use TIM1RST_W as TIM8RST_W;
///Field `USART1RST` writer - USART1 reset This bit is set and cleared by software.
pub use TIM1RST_W as USART1RST_W;
///Field `TIM15RST` writer - TIM15 reset This bit is set and cleared by software.
pub use TIM1RST_W as TIM15RST_W;
///Field `TIM16RST` writer - TIM16 reset This bit is set and cleared by software.
pub use TIM1RST_W as TIM16RST_W;
///Field `TIM17RST` writer - TIM17 reset This bit is set and cleared by software.
pub use TIM1RST_W as TIM17RST_W;
///Field `SAI1RST` writer - SAI1 reset This bit is set and cleared by software.
pub use TIM1RST_W as SAI1RST_W;
///Field `SAI2RST` writer - SAI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_W as SAI2RST_W;
///Field `USBRST` writer - USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_W as USBRST_W;
///Field `GFXTIMRST` writer - GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_W as GFXTIMRST_W;
///Field `LTDCRST` writer - LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_W as LTDCRST_W;
///Field `DSIRST` writer - DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use TIM1RST_W as DSIRST_W;
impl R {
    ///Bit 11 - TIM1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - SAI1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxtimrst(&self) -> GFXTIMRST_R {
        GFXTIMRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("tim1rst", &self.tim1rst())
            .field("spi1rst", &self.spi1rst())
            .field("tim8rst", &self.tim8rst())
            .field("usart1rst", &self.usart1rst())
            .field("tim15rst", &self.tim15rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("sai1rst", &self.sai1rst())
            .field("sai2rst", &self.sai2rst())
            .field("usbrst", &self.usbrst())
            .field("gfxtimrst", &self.gfxtimrst())
            .field("ltdcrst", &self.ltdcrst())
            .field("dsirst", &self.dsirst())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    ///Bit 12 - SPI1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 13 - TIM8 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<APB2RSTRrs> {
        TIM8RST_W::new(self, 13)
    }
    ///Bit 14 - USART1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 16 - TIM15 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    ///Bit 17 - TIM16 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM17 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    ///Bit 21 - SAI1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W<APB2RSTRrs> {
        SAI1RST_W::new(self, 21)
    }
    ///Bit 22 - SAI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W<APB2RSTRrs> {
        SAI2RST_W::new(self, 22)
    }
    ///Bit 24 - USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<APB2RSTRrs> {
        USBRST_W::new(self, 24)
    }
    ///Bit 25 - GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxtimrst(&mut self) -> GFXTIMRST_W<APB2RSTRrs> {
        GFXTIMRST_W::new(self, 25)
    }
    ///Bit 26 - LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<APB2RSTRrs> {
        LTDCRST_W::new(self, 26)
    }
    ///Bit 27 - DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dsirst(&mut self) -> DSIRST_W<APB2RSTRrs> {
        DSIRST_W::new(self, 27)
    }
}
/**RCC APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RCC:APB2RSTR)*/
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2rstr::R`](R) reader structure
impl crate::Readable for APB2RSTRrs {}
///`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTR to value 0
impl crate::Resettable for APB2RSTRrs {}
