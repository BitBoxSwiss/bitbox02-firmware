///Register `APB3ENR` reader
pub type R = crate::R<APB3ENRrs>;
///Register `APB3ENR` writer
pub type W = crate::W<APB3ENRrs>;
/**SYSCFG clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    ///0: Peripheral clock disabled
    Disabled = 0,
    ///1: Peripheral clock enabled
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGEN` reader - SYSCFG clock enable This bit is set and cleared by software.
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::Disabled,
            true => SYSCFGEN::Enabled,
        }
    }
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
///Field `SYSCFGEN` writer - SYSCFG clock enable This bit is set and cleared by software.
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
///Field `SPI3EN` reader - SPI3 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as SPI3EN_R;
///Field `LPUART1EN` reader - LPUART1 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as LPUART1EN_R;
///Field `I2C3EN` reader - I2C3 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as I2C3EN_R;
///Field `LPTIM1EN` reader - LPTIM1 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as LPTIM1EN_R;
///Field `LPTIM3EN` reader - LPTIM3 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as LPTIM3EN_R;
///Field `LPTIM4EN` reader - LPTIM4 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as LPTIM4EN_R;
///Field `OPAMPEN` reader - OPAMP clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as OPAMPEN_R;
///Field `COMPEN` reader - COMP clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as COMPEN_R;
///Field `VREFEN` reader - VREFBUF clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as VREFEN_R;
///Field `RTCAPBEN` reader - RTC and TAMP APB clock enable This bit is set and cleared by software.
pub use SYSCFGEN_R as RTCAPBEN_R;
///Field `SPI3EN` writer - SPI3 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as SPI3EN_W;
///Field `LPUART1EN` writer - LPUART1 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as LPUART1EN_W;
///Field `I2C3EN` writer - I2C3 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as I2C3EN_W;
///Field `LPTIM1EN` writer - LPTIM1 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as LPTIM1EN_W;
///Field `LPTIM3EN` writer - LPTIM3 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as LPTIM3EN_W;
///Field `LPTIM4EN` writer - LPTIM4 clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as LPTIM4EN_W;
///Field `OPAMPEN` writer - OPAMP clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as OPAMPEN_W;
///Field `COMPEN` writer - COMP clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as COMPEN_W;
///Field `VREFEN` writer - VREFBUF clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as VREFEN_W;
///Field `RTCAPBEN` writer - RTC and TAMP APB clock enable This bit is set and cleared by software.
pub use SYSCFGEN_W as RTCAPBEN_W;
impl R {
    ///Bit 1 - SYSCFG clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP APB clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3ENR")
            .field("syscfgen", &self.syscfgen())
            .field("spi3en", &self.spi3en())
            .field("lpuart1en", &self.lpuart1en())
            .field("i2c3en", &self.i2c3en())
            .field("lptim1en", &self.lptim1en())
            .field("lptim3en", &self.lptim3en())
            .field("lptim4en", &self.lptim4en())
            .field("opampen", &self.opampen())
            .field("compen", &self.compen())
            .field("vrefen", &self.vrefen())
            .field("rtcapben", &self.rtcapben())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APB3ENRrs> {
        SYSCFGEN_W::new(self, 1)
    }
    ///Bit 5 - SPI3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<APB3ENRrs> {
        SPI3EN_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APB3ENRrs> {
        LPUART1EN_W::new(self, 6)
    }
    ///Bit 7 - I2C3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB3ENRrs> {
        I2C3EN_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB3ENRrs> {
        LPTIM1EN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<APB3ENRrs> {
        LPTIM3EN_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<APB3ENRrs> {
        LPTIM4EN_W::new(self, 13)
    }
    ///Bit 14 - OPAMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn opampen(&mut self) -> OPAMPEN_W<APB3ENRrs> {
        OPAMPEN_W::new(self, 14)
    }
    ///Bit 15 - COMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn compen(&mut self) -> COMPEN_W<APB3ENRrs> {
        COMPEN_W::new(self, 15)
    }
    ///Bit 20 - VREFBUF clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<APB3ENRrs> {
        VREFEN_W::new(self, 20)
    }
    ///Bit 21 - RTC and TAMP APB clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APB3ENRrs> {
        RTCAPBEN_W::new(self, 21)
    }
}
/**RCC APB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RCC:APB3ENR)*/
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3enr::R`](R) reader structure
impl crate::Readable for APB3ENRrs {}
///`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3ENR to value 0
impl crate::Resettable for APB3ENRrs {}
