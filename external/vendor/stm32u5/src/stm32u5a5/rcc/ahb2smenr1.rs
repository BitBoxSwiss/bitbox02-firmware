///Register `AHB2SMENR1` reader
pub type R = crate::R<AHB2SMENR1rs>;
///Register `AHB2SMENR1` writer
pub type W = crate::W<AHB2SMENR1rs>;
/**I/O port A clocks enable during Sleep and Stop modes This bit is set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOASMEN {
    ///0: Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    Enabled = 1,
}
impl From<GPIOASMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOASMEN` reader - I/O port A clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub type GPIOASMEN_R = crate::BitReader<GPIOASMEN>;
impl GPIOASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOASMEN {
        match self.bits {
            false => GPIOASMEN::Disabled,
            true => GPIOASMEN::Enabled,
        }
    }
    ///Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOASMEN::Disabled
    }
    ///Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOASMEN::Enabled
    }
}
///Field `GPIOASMEN` writer - I/O port A clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOASMEN>;
impl<'a, REG> GPIOASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Disabled)
    }
    ///Peripheral clocks enabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Enabled)
    }
}
///Field `GPIOBSMEN` reader - I/O port B clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as GPIOBSMEN_R;
///Field `GPIOCSMEN` reader - I/O port C clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as GPIOCSMEN_R;
///Field `GPIODSMEN` reader - I/O port D clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as GPIODSMEN_R;
///Field `GPIOESMEN` reader - I/O port E clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as GPIOESMEN_R;
///Field `GPIOFSMEN` reader - I/O port F clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as GPIOFSMEN_R;
///Field `GPIOGSMEN` reader - I/O port G clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as GPIOGSMEN_R;
///Field `GPIOHSMEN` reader - I/O port H clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as GPIOHSMEN_R;
///Field `GPIOISMEN` reader - I/O port I clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as GPIOISMEN_R;
///Field `GPIOJSMEN` reader - I/O port J clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as GPIOJSMEN_R;
///Field `ADC12SMEN` reader - ADC1 and ADC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
pub use GPIOASMEN_R as ADC12SMEN_R;
///Field `DCMI_PSSISMEN` reader - DCMI and PSSI clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as DCMI_PSSISMEN_R;
///Field `OTGSMEN` reader - OTG_FS and OTG_HS clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as OTGSMEN_R;
///Field `OTGHSPHYSMEN` reader - OTG_HS PHY clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as OTGHSPHYSMEN_R;
///Field `AESSMEN` reader - AES clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as AESSMEN_R;
///Field `HASHSMEN` reader - HASH clock enable during Sleep and Stop modes This bit is set and cleared by software
pub use GPIOASMEN_R as HASHSMEN_R;
///Field `RNGSMEN` reader - RNG clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as RNGSMEN_R;
///Field `PKASMEN` reader - PKA clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as PKASMEN_R;
///Field `SAESSMEN` reader - SAES accelerator clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as SAESSMEN_R;
///Field `OCTOSPIMSMEN` reader - OCTOSPIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as OCTOSPIMSMEN_R;
///Field `OTFDEC1SMEN` reader - OTFDEC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as OTFDEC1SMEN_R;
///Field `OTFDEC2SMEN` reader - OTFDEC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as OTFDEC2SMEN_R;
///Field `SDMMC1SMEN` reader - SDMMC1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as SDMMC1SMEN_R;
///Field `SDMMC2SMEN` reader - SDMMC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as SDMMC2SMEN_R;
///Field `SRAM2SMEN` reader - SRAM2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_R as SRAM2SMEN_R;
///Field `SRAM3SMEN` reader - SRAM3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_R as SRAM3SMEN_R;
///Field `GPIOBSMEN` writer - I/O port B clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as GPIOBSMEN_W;
///Field `GPIOCSMEN` writer - I/O port C clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as GPIOCSMEN_W;
///Field `GPIODSMEN` writer - I/O port D clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as GPIODSMEN_W;
///Field `GPIOESMEN` writer - I/O port E clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as GPIOESMEN_W;
///Field `GPIOFSMEN` writer - I/O port F clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as GPIOFSMEN_W;
///Field `GPIOGSMEN` writer - I/O port G clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as GPIOGSMEN_W;
///Field `GPIOHSMEN` writer - I/O port H clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as GPIOHSMEN_W;
///Field `GPIOISMEN` writer - I/O port I clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as GPIOISMEN_W;
///Field `GPIOJSMEN` writer - I/O port J clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as GPIOJSMEN_W;
///Field `ADC12SMEN` writer - ADC1 and ADC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
pub use GPIOASMEN_W as ADC12SMEN_W;
///Field `DCMI_PSSISMEN` writer - DCMI and PSSI clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as DCMI_PSSISMEN_W;
///Field `OTGSMEN` writer - OTG_FS and OTG_HS clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as OTGSMEN_W;
///Field `OTGHSPHYSMEN` writer - OTG_HS PHY clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as OTGHSPHYSMEN_W;
///Field `AESSMEN` writer - AES clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as AESSMEN_W;
///Field `HASHSMEN` writer - HASH clock enable during Sleep and Stop modes This bit is set and cleared by software
pub use GPIOASMEN_W as HASHSMEN_W;
///Field `RNGSMEN` writer - RNG clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as RNGSMEN_W;
///Field `PKASMEN` writer - PKA clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as PKASMEN_W;
///Field `SAESSMEN` writer - SAES accelerator clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as SAESSMEN_W;
///Field `OCTOSPIMSMEN` writer - OCTOSPIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as OCTOSPIMSMEN_W;
///Field `OTFDEC1SMEN` writer - OTFDEC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as OTFDEC1SMEN_W;
///Field `OTFDEC2SMEN` writer - OTFDEC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as OTFDEC2SMEN_W;
///Field `SDMMC1SMEN` writer - SDMMC1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as SDMMC1SMEN_W;
///Field `SDMMC2SMEN` writer - SDMMC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as SDMMC2SMEN_W;
///Field `SRAM2SMEN` writer - SRAM2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub use GPIOASMEN_W as SRAM2SMEN_W;
///Field `SRAM3SMEN` writer - SRAM3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOASMEN_W as SRAM3SMEN_W;
impl R {
    ///Bit 0 - I/O port A clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port F clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I/O port G clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I/O port H clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I/O port I clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpioismen(&self) -> GPIOISMEN_R {
        GPIOISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I/O port J clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiojsmen(&self) -> GPIOJSMEN_R {
        GPIOJSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC1 and ADC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
    #[inline(always)]
    pub fn adc12smen(&self) -> ADC12SMEN_R {
        ADC12SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - DCMI and PSSI clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssismen(&self) -> DCMI_PSSISMEN_R {
        DCMI_PSSISMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OTG_FS and OTG_HS clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otgsmen(&self) -> OTGSMEN_R {
        OTGSMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - OTG_HS PHY clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otghsphysmen(&self) -> OTGHSPHYSMEN_R {
        OTGHSPHYSMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AES clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes This bit is set and cleared by software
    #[inline(always)]
    pub fn hashsmen(&self) -> HASHSMEN_R {
        HASHSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PKA clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES accelerator clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saessmen(&self) -> SAESSMEN_R {
        SAESSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospimsmen(&self) -> OCTOSPIMSMEN_R {
        OCTOSPIMSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - OTFDEC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec1smen(&self) -> OTFDEC1SMEN_R {
        OTFDEC1SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTFDEC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec2smen(&self) -> OTFDEC2SMEN_R {
        OTFDEC2SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 27 - SDMMC1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1smen(&self) -> SDMMC1SMEN_R {
        SDMMC1SMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDMMC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sdmmc2smen(&self) -> SDMMC2SMEN_R {
        SDMMC2SMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - SRAM2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram3smen(&self) -> SRAM3SMEN_R {
        SRAM3SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2SMENR1")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpioesmen", &self.gpioesmen())
            .field("gpiofsmen", &self.gpiofsmen())
            .field("gpiogsmen", &self.gpiogsmen())
            .field("gpiohsmen", &self.gpiohsmen())
            .field("gpioismen", &self.gpioismen())
            .field("gpiojsmen", &self.gpiojsmen())
            .field("adc12smen", &self.adc12smen())
            .field("dcmi_pssismen", &self.dcmi_pssismen())
            .field("otgsmen", &self.otgsmen())
            .field("otghsphysmen", &self.otghsphysmen())
            .field("aessmen", &self.aessmen())
            .field("hashsmen", &self.hashsmen())
            .field("rngsmen", &self.rngsmen())
            .field("pkasmen", &self.pkasmen())
            .field("saessmen", &self.saessmen())
            .field("octospimsmen", &self.octospimsmen())
            .field("otfdec1smen", &self.otfdec1smen())
            .field("otfdec2smen", &self.otfdec2smen())
            .field("sdmmc1smen", &self.sdmmc1smen())
            .field("sdmmc2smen", &self.sdmmc2smen())
            .field("sram2smen", &self.sram2smen())
            .field("sram3smen", &self.sram3smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<AHB2SMENR1rs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<AHB2SMENR1rs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<AHB2SMENR1rs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<AHB2SMENR1rs> {
        GPIODSMEN_W::new(self, 3)
    }
    ///Bit 4 - I/O port E clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<AHB2SMENR1rs> {
        GPIOESMEN_W::new(self, 4)
    }
    ///Bit 5 - I/O port F clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<AHB2SMENR1rs> {
        GPIOFSMEN_W::new(self, 5)
    }
    ///Bit 6 - I/O port G clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<AHB2SMENR1rs> {
        GPIOGSMEN_W::new(self, 6)
    }
    ///Bit 7 - I/O port H clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<AHB2SMENR1rs> {
        GPIOHSMEN_W::new(self, 7)
    }
    ///Bit 8 - I/O port I clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpioismen(&mut self) -> GPIOISMEN_W<AHB2SMENR1rs> {
        GPIOISMEN_W::new(self, 8)
    }
    ///Bit 9 - I/O port J clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiojsmen(&mut self) -> GPIOJSMEN_W<AHB2SMENR1rs> {
        GPIOJSMEN_W::new(self, 9)
    }
    ///Bit 10 - ADC1 and ADC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
    #[inline(always)]
    pub fn adc12smen(&mut self) -> ADC12SMEN_W<AHB2SMENR1rs> {
        ADC12SMEN_W::new(self, 10)
    }
    ///Bit 12 - DCMI and PSSI clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssismen(&mut self) -> DCMI_PSSISMEN_W<AHB2SMENR1rs> {
        DCMI_PSSISMEN_W::new(self, 12)
    }
    ///Bit 14 - OTG_FS and OTG_HS clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otgsmen(&mut self) -> OTGSMEN_W<AHB2SMENR1rs> {
        OTGSMEN_W::new(self, 14)
    }
    ///Bit 15 - OTG_HS PHY clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otghsphysmen(&mut self) -> OTGHSPHYSMEN_W<AHB2SMENR1rs> {
        OTGHSPHYSMEN_W::new(self, 15)
    }
    ///Bit 16 - AES clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W<AHB2SMENR1rs> {
        AESSMEN_W::new(self, 16)
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes This bit is set and cleared by software
    #[inline(always)]
    pub fn hashsmen(&mut self) -> HASHSMEN_W<AHB2SMENR1rs> {
        HASHSMEN_W::new(self, 17)
    }
    ///Bit 18 - RNG clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<AHB2SMENR1rs> {
        RNGSMEN_W::new(self, 18)
    }
    ///Bit 19 - PKA clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn pkasmen(&mut self) -> PKASMEN_W<AHB2SMENR1rs> {
        PKASMEN_W::new(self, 19)
    }
    ///Bit 20 - SAES accelerator clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saessmen(&mut self) -> SAESSMEN_W<AHB2SMENR1rs> {
        SAESSMEN_W::new(self, 20)
    }
    ///Bit 21 - OCTOSPIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospimsmen(&mut self) -> OCTOSPIMSMEN_W<AHB2SMENR1rs> {
        OCTOSPIMSMEN_W::new(self, 21)
    }
    ///Bit 23 - OTFDEC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec1smen(&mut self) -> OTFDEC1SMEN_W<AHB2SMENR1rs> {
        OTFDEC1SMEN_W::new(self, 23)
    }
    ///Bit 24 - OTFDEC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec2smen(&mut self) -> OTFDEC2SMEN_W<AHB2SMENR1rs> {
        OTFDEC2SMEN_W::new(self, 24)
    }
    ///Bit 27 - SDMMC1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1smen(&mut self) -> SDMMC1SMEN_W<AHB2SMENR1rs> {
        SDMMC1SMEN_W::new(self, 27)
    }
    ///Bit 28 - SDMMC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sdmmc2smen(&mut self) -> SDMMC2SMEN_W<AHB2SMENR1rs> {
        SDMMC2SMEN_W::new(self, 28)
    }
    ///Bit 30 - SRAM2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<AHB2SMENR1rs> {
        SRAM2SMEN_W::new(self, 30)
    }
    ///Bit 31 - SRAM3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram3smen(&mut self) -> SRAM3SMEN_W<AHB2SMENR1rs> {
        SRAM3SMEN_W::new(self, 31)
    }
}
/**RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RCC:AHB2SMENR1)*/
pub struct AHB2SMENR1rs;
impl crate::RegisterSpec for AHB2SMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`ahb2smenr1::R`](R) reader structure
impl crate::Readable for AHB2SMENR1rs {}
///`write(|w| ..)` method takes [`ahb2smenr1::W`](W) writer structure
impl crate::Writable for AHB2SMENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2SMENR1 to value 0xffff_ffff
impl crate::Resettable for AHB2SMENR1rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
