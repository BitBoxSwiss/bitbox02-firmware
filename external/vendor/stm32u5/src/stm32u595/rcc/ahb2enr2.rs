///Register `AHB2ENR2` reader
pub type R = crate::R<AHB2ENR2rs>;
///Register `AHB2ENR2` writer
pub type W = crate::W<AHB2ENR2rs>;
/**FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCEN {
    ///0: Peripheral clock disabled
    Disabled = 0,
    ///1: Peripheral clock enabled
    Enabled = 1,
}
impl From<FSMCEN> for bool {
    #[inline(always)]
    fn from(variant: FSMCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FSMCEN` reader - FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type FSMCEN_R = crate::BitReader<FSMCEN>;
impl FSMCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSMCEN {
        match self.bits {
            false => FSMCEN::Disabled,
            true => FSMCEN::Enabled,
        }
    }
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSMCEN::Disabled
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSMCEN::Enabled
    }
}
///Field `FSMCEN` writer - FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type FSMCEN_W<'a, REG> = crate::BitWriter<'a, REG, FSMCEN>;
impl<'a, REG> FSMCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCEN::Disabled)
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCEN::Enabled)
    }
}
///Field `OCTOSPI1EN` reader - OCTOSPI1 clock enable This bit is set and cleared by software.
pub use FSMCEN_R as OCTOSPI1EN_R;
///Field `OCTOSPI2EN` reader - OCTOSPI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCEN_R as OCTOSPI2EN_R;
///Field `HSPI1EN` reader - HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCEN_R as HSPI1EN_R;
///Field `SRAM6EN` reader - SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCEN_R as SRAM6EN_R;
///Field `SRAM5EN` reader - SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCEN_R as SRAM5EN_R;
///Field `OCTOSPI1EN` writer - OCTOSPI1 clock enable This bit is set and cleared by software.
pub use FSMCEN_W as OCTOSPI1EN_W;
///Field `OCTOSPI2EN` writer - OCTOSPI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCEN_W as OCTOSPI2EN_W;
///Field `HSPI1EN` writer - HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCEN_W as HSPI1EN_W;
///Field `SRAM6EN` writer - SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCEN_W as SRAM6EN_W;
///Field `SRAM5EN` writer - SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use FSMCEN_W as SRAM5EN_W;
impl R {
    ///Bit 0 - FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - OCTOSPI1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - OCTOSPI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospi2en(&self) -> OCTOSPI2EN_R {
        OCTOSPI2EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn hspi1en(&self) -> HSPI1EN_R {
        HSPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 30 - SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram6en(&self) -> SRAM6EN_R {
        SRAM6EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram5en(&self) -> SRAM5EN_R {
        SRAM5EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR2")
            .field("fsmcen", &self.fsmcen())
            .field("octospi1en", &self.octospi1en())
            .field("octospi2en", &self.octospi2en())
            .field("hspi1en", &self.hspi1en())
            .field("sram6en", &self.sram6en())
            .field("sram5en", &self.sram5en())
            .finish()
    }
}
impl W {
    ///Bit 0 - FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn fsmcen(&mut self) -> FSMCEN_W<AHB2ENR2rs> {
        FSMCEN_W::new(self, 0)
    }
    ///Bit 4 - OCTOSPI1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<AHB2ENR2rs> {
        OCTOSPI1EN_W::new(self, 4)
    }
    ///Bit 8 - OCTOSPI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospi2en(&mut self) -> OCTOSPI2EN_W<AHB2ENR2rs> {
        OCTOSPI2EN_W::new(self, 8)
    }
    ///Bit 12 - HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn hspi1en(&mut self) -> HSPI1EN_W<AHB2ENR2rs> {
        HSPI1EN_W::new(self, 12)
    }
    ///Bit 30 - SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram6en(&mut self) -> SRAM6EN_W<AHB2ENR2rs> {
        SRAM6EN_W::new(self, 30)
    }
    ///Bit 31 - SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram5en(&mut self) -> SRAM5EN_W<AHB2ENR2rs> {
        SRAM5EN_W::new(self, 31)
    }
}
/**RCC AHB2 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`ahb2enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:AHB2ENR2)*/
pub struct AHB2ENR2rs;
impl crate::RegisterSpec for AHB2ENR2rs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr2::R`](R) reader structure
impl crate::Readable for AHB2ENR2rs {}
///`write(|w| ..)` method takes [`ahb2enr2::W`](W) writer structure
impl crate::Writable for AHB2ENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR2 to value 0x8000_0000
impl crate::Resettable for AHB2ENR2rs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
