///Register `AHB2ENR1` reader
pub type R = crate::R<AHB2ENR1rs>;
///Register `AHB2ENR1` writer
pub type W = crate::W<AHB2ENR1rs>;
/**I/O port A clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: Peripheral clock disabled
    Disabled = 0,
    ///1: Peripheral clock enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - I/O port A clock enable This bit is set and cleared by software.
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - I/O port A clock enable This bit is set and cleared by software.
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///Peripheral clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - I/O port B clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - I/O port C clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - I/O port D clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - I/O port E clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - I/O port F clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - I/O port G clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - I/O port H clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOIEN` reader - I/O port I clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as GPIOIEN_R;
///Field `GPIOJEN` reader - I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as GPIOJEN_R;
///Field `ADC12EN` reader - ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
pub use GPIOAEN_R as ADC12EN_R;
///Field `DCMI_PSSIEN` reader - DCMI and PSSI clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as DCMI_PSSIEN_R;
///Field `OTGEN` reader - OTG_FS or OTG_HS clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as OTGEN_R;
///Field `OTGHSPHYEN` reader - OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as OTGHSPHYEN_R;
///Field `AESEN` reader - AES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as AESEN_R;
///Field `HASHEN` reader - HASH clock enable This bit is set and cleared by software
pub use GPIOAEN_R as HASHEN_R;
///Field `RNGEN` reader - RNG clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as RNGEN_R;
///Field `PKAEN` reader - PKA clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as PKAEN_R;
///Field `SAESEN` reader - SAES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as SAESEN_R;
///Field `OCTOSPIMEN` reader - OCTOSPIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as OCTOSPIMEN_R;
///Field `OTFDEC1EN` reader - OTFDEC1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as OTFDEC1EN_R;
///Field `OTFDEC2EN` reader - OTFDEC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as OTFDEC2EN_R;
///Field `SDMMC1EN` reader - SDMMC1 clock enable This bit is set and cleared by software.
pub use GPIOAEN_R as SDMMC1EN_R;
///Field `SDMMC2EN` reader - SDMMC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as SDMMC2EN_R;
///Field `SRAM2EN` reader - SRAM2 clock enable This bit is set and reset by software.
pub use GPIOAEN_R as SRAM2EN_R;
///Field `SRAM3EN` reader - SRAM3 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_R as SRAM3EN_R;
///Field `GPIOBEN` writer - I/O port B clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - I/O port C clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - I/O port D clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - I/O port E clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - I/O port F clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - I/O port G clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - I/O port H clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as GPIOHEN_W;
///Field `GPIOIEN` writer - I/O port I clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as GPIOIEN_W;
///Field `GPIOJEN` writer - I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as GPIOJEN_W;
///Field `ADC12EN` writer - ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
pub use GPIOAEN_W as ADC12EN_W;
///Field `DCMI_PSSIEN` writer - DCMI and PSSI clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as DCMI_PSSIEN_W;
///Field `OTGEN` writer - OTG_FS or OTG_HS clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as OTGEN_W;
///Field `OTGHSPHYEN` writer - OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as OTGHSPHYEN_W;
///Field `AESEN` writer - AES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as AESEN_W;
///Field `HASHEN` writer - HASH clock enable This bit is set and cleared by software
pub use GPIOAEN_W as HASHEN_W;
///Field `RNGEN` writer - RNG clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as RNGEN_W;
///Field `PKAEN` writer - PKA clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as PKAEN_W;
///Field `SAESEN` writer - SAES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as SAESEN_W;
///Field `OCTOSPIMEN` writer - OCTOSPIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as OCTOSPIMEN_W;
///Field `OTFDEC1EN` writer - OTFDEC1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as OTFDEC1EN_W;
///Field `OTFDEC2EN` writer - OTFDEC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as OTFDEC2EN_W;
///Field `SDMMC1EN` writer - SDMMC1 clock enable This bit is set and cleared by software.
pub use GPIOAEN_W as SDMMC1EN_W;
///Field `SDMMC2EN` writer - SDMMC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as SDMMC2EN_W;
///Field `SRAM2EN` writer - SRAM2 clock enable This bit is set and reset by software.
pub use GPIOAEN_W as SRAM2EN_W;
///Field `SRAM3EN` writer - SRAM3 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOAEN_W as SRAM3EN_W;
impl R {
    ///Bit 0 - I/O port A clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I/O port G clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I/O port H clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I/O port I clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - DCMI and PSSI clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssien(&self) -> DCMI_PSSIEN_R {
        DCMI_PSSIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OTG_FS or OTG_HS clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otghsphyen(&self) -> OTGHSPHYEN_R {
        OTGHSPHYEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable This bit is set and cleared by software
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PKA clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saesen(&self) -> SAESEN_R {
        SAESEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospimen(&self) -> OCTOSPIMEN_R {
        OCTOSPIMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - OTFDEC1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec1en(&self) -> OTFDEC1EN_R {
        OTFDEC1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTFDEC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec2en(&self) -> OTFDEC2EN_R {
        OTFDEC2EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 27 - SDMMC1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDMMC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - SRAM2 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM3 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR1")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("gpioien", &self.gpioien())
            .field("gpiojen", &self.gpiojen())
            .field("adc12en", &self.adc12en())
            .field("dcmi_pssien", &self.dcmi_pssien())
            .field("otgen", &self.otgen())
            .field("otghsphyen", &self.otghsphyen())
            .field("aesen", &self.aesen())
            .field("hashen", &self.hashen())
            .field("rngen", &self.rngen())
            .field("pkaen", &self.pkaen())
            .field("saesen", &self.saesen())
            .field("octospimen", &self.octospimen())
            .field("otfdec1en", &self.otfdec1en())
            .field("otfdec2en", &self.otfdec2en())
            .field("sdmmc1en", &self.sdmmc1en())
            .field("sdmmc2en", &self.sdmmc2en())
            .field("sram2en", &self.sram2en())
            .field("sram3en", &self.sram3en())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB2ENR1rs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB2ENR1rs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB2ENR1rs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB2ENR1rs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - I/O port E clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB2ENR1rs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - I/O port F clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHB2ENR1rs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - I/O port G clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHB2ENR1rs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - I/O port H clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB2ENR1rs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 8 - I/O port I clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<AHB2ENR1rs> {
        GPIOIEN_W::new(self, 8)
    }
    ///Bit 9 - I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<AHB2ENR1rs> {
        GPIOJEN_W::new(self, 9)
    }
    ///Bit 10 - ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<AHB2ENR1rs> {
        ADC12EN_W::new(self, 10)
    }
    ///Bit 12 - DCMI and PSSI clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssien(&mut self) -> DCMI_PSSIEN_W<AHB2ENR1rs> {
        DCMI_PSSIEN_W::new(self, 12)
    }
    ///Bit 14 - OTG_FS or OTG_HS clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otgen(&mut self) -> OTGEN_W<AHB2ENR1rs> {
        OTGEN_W::new(self, 14)
    }
    ///Bit 15 - OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otghsphyen(&mut self) -> OTGHSPHYEN_W<AHB2ENR1rs> {
        OTGHSPHYEN_W::new(self, 15)
    }
    ///Bit 16 - AES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<AHB2ENR1rs> {
        AESEN_W::new(self, 16)
    }
    ///Bit 17 - HASH clock enable This bit is set and cleared by software
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<AHB2ENR1rs> {
        HASHEN_W::new(self, 17)
    }
    ///Bit 18 - RNG clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<AHB2ENR1rs> {
        RNGEN_W::new(self, 18)
    }
    ///Bit 19 - PKA clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W<AHB2ENR1rs> {
        PKAEN_W::new(self, 19)
    }
    ///Bit 20 - SAES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saesen(&mut self) -> SAESEN_W<AHB2ENR1rs> {
        SAESEN_W::new(self, 20)
    }
    ///Bit 21 - OCTOSPIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospimen(&mut self) -> OCTOSPIMEN_W<AHB2ENR1rs> {
        OCTOSPIMEN_W::new(self, 21)
    }
    ///Bit 23 - OTFDEC1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec1en(&mut self) -> OTFDEC1EN_W<AHB2ENR1rs> {
        OTFDEC1EN_W::new(self, 23)
    }
    ///Bit 24 - OTFDEC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec2en(&mut self) -> OTFDEC2EN_W<AHB2ENR1rs> {
        OTFDEC2EN_W::new(self, 24)
    }
    ///Bit 27 - SDMMC1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<AHB2ENR1rs> {
        SDMMC1EN_W::new(self, 27)
    }
    ///Bit 28 - SDMMC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<AHB2ENR1rs> {
        SDMMC2EN_W::new(self, 28)
    }
    ///Bit 30 - SRAM2 clock enable This bit is set and reset by software.
    #[inline(always)]
    pub fn sram2en(&mut self) -> SRAM2EN_W<AHB2ENR1rs> {
        SRAM2EN_W::new(self, 30)
    }
    ///Bit 31 - SRAM3 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram3en(&mut self) -> SRAM3EN_W<AHB2ENR1rs> {
        SRAM3EN_W::new(self, 31)
    }
}
/**RCC AHB2 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`ahb2enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:AHB2ENR1)*/
pub struct AHB2ENR1rs;
impl crate::RegisterSpec for AHB2ENR1rs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr1::R`](R) reader structure
impl crate::Readable for AHB2ENR1rs {}
///`write(|w| ..)` method takes [`ahb2enr1::W`](W) writer structure
impl crate::Writable for AHB2ENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR1 to value 0xc000_0000
impl crate::Resettable for AHB2ENR1rs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
