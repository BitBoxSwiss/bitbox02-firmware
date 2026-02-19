///Register `AHB2RSTR1` reader
pub type R = crate::R<AHB2RSTR1rs>;
///Register `AHB2RSTR1` writer
pub type W = crate::W<AHB2RSTR1rs>;
/**I/O port A reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<GPIOARST> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOARST` reader - I/O port A reset This bit is set and cleared by software.
pub type GPIOARST_R = crate::BitReader<GPIOARST>;
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOARST {
        match self.bits {
            false => GPIOARST::NoEffect,
            true => GPIOARST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GPIOARST::NoEffect
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST::Reset
    }
}
///Field `GPIOARST` writer - I/O port A reset This bit is set and cleared by software.
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::NoEffect)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::Reset)
    }
}
///Field `GPIOBRST` reader - I/O port B reset This bit is set and cleared by software.
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - I/O port C reset This bit is set and cleared by software.
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - I/O port D reset This bit is set and cleared by software.
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOERST` reader - I/O port E reset This bit is set and cleared by software.
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - I/O port F reset This bit is set and cleared by software. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - I/O port G reset This bit is set and cleared by software.
pub use GPIOARST_R as GPIOGRST_R;
///Field `GPIOHRST` reader - I/O port H reset This bit is set and cleared by software.
pub use GPIOARST_R as GPIOHRST_R;
///Field `GPIOIRST` reader - I/O port I reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as GPIOIRST_R;
///Field `GPIOJRST` reader - I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as GPIOJRST_R;
///Field `ADC12RST` reader - ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
pub use GPIOARST_R as ADC12RST_R;
///Field `DCMI_PSSIRST` reader - DCMI and PSSI reset This bit is set and cleared by software.
pub use GPIOARST_R as DCMI_PSSIRST_R;
///Field `OTGRST` reader - OTG_FS or OTG_HS reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as OTGRST_R;
///Field `AESRST` reader - AES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as AESRST_R;
///Field `HASHRST` reader - HASH reset This bit is set and cleared by software.
pub use GPIOARST_R as HASHRST_R;
///Field `RNGRST` reader - RNG reset This bit is set and cleared by software.
pub use GPIOARST_R as RNGRST_R;
///Field `PKARST` reader - PKA reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as PKARST_R;
///Field `SAESRST` reader - SAES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as SAESRST_R;
///Field `OCTOSPIMRST` reader - OCTOSPIM reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as OCTOSPIMRST_R;
///Field `OTFDEC1RST` reader - OTFDEC1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as OTFDEC1RST_R;
///Field `OTFDEC2RST` reader - OTFDEC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as OTFDEC2RST_R;
///Field `SDMMC1RST` reader - SDMMC1 reset This bit is set and cleared by software.
pub use GPIOARST_R as SDMMC1RST_R;
///Field `SDMMC2RST` reader - SDMMC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_R as SDMMC2RST_R;
///Field `GPIOBRST` writer - I/O port B reset This bit is set and cleared by software.
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - I/O port C reset This bit is set and cleared by software.
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - I/O port D reset This bit is set and cleared by software.
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - I/O port E reset This bit is set and cleared by software.
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - I/O port F reset This bit is set and cleared by software. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - I/O port G reset This bit is set and cleared by software.
pub use GPIOARST_W as GPIOGRST_W;
///Field `GPIOHRST` writer - I/O port H reset This bit is set and cleared by software.
pub use GPIOARST_W as GPIOHRST_W;
///Field `GPIOIRST` writer - I/O port I reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as GPIOIRST_W;
///Field `GPIOJRST` writer - I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as GPIOJRST_W;
///Field `ADC12RST` writer - ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
pub use GPIOARST_W as ADC12RST_W;
///Field `DCMI_PSSIRST` writer - DCMI and PSSI reset This bit is set and cleared by software.
pub use GPIOARST_W as DCMI_PSSIRST_W;
///Field `OTGRST` writer - OTG_FS or OTG_HS reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as OTGRST_W;
///Field `AESRST` writer - AES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as AESRST_W;
///Field `HASHRST` writer - HASH reset This bit is set and cleared by software.
pub use GPIOARST_W as HASHRST_W;
///Field `RNGRST` writer - RNG reset This bit is set and cleared by software.
pub use GPIOARST_W as RNGRST_W;
///Field `PKARST` writer - PKA reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as PKARST_W;
///Field `SAESRST` writer - SAES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as SAESRST_W;
///Field `OCTOSPIMRST` writer - OCTOSPIM reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as OCTOSPIMRST_W;
///Field `OTFDEC1RST` writer - OTFDEC1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as OTFDEC1RST_W;
///Field `OTFDEC2RST` writer - OTFDEC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as OTFDEC2RST_W;
///Field `SDMMC1RST` writer - SDMMC1 reset This bit is set and cleared by software.
pub use GPIOARST_W as SDMMC1RST_W;
///Field `SDMMC2RST` writer - SDMMC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub use GPIOARST_W as SDMMC2RST_W;
impl R {
    ///Bit 0 - I/O port A reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port F reset This bit is set and cleared by software. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I/O port G reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I/O port H reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I/O port I reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - DCMI and PSSI reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OTG_FS or OTG_HS reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otgrst(&self) -> OTGRST_R {
        OTGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PKA reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saesrst(&self) -> SAESRST_R {
        SAESRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospimrst(&self) -> OCTOSPIMRST_R {
        OCTOSPIMRST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - OTFDEC1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec1rst(&self) -> OTFDEC1RST_R {
        OTFDEC1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTFDEC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec2rst(&self) -> OTFDEC2RST_R {
        OTFDEC2RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 27 - SDMMC1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDMMC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR1")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpioirst", &self.gpioirst())
            .field("gpiojrst", &self.gpiojrst())
            .field("adc12rst", &self.adc12rst())
            .field("dcmi_pssirst", &self.dcmi_pssirst())
            .field("otgrst", &self.otgrst())
            .field("aesrst", &self.aesrst())
            .field("hashrst", &self.hashrst())
            .field("rngrst", &self.rngrst())
            .field("pkarst", &self.pkarst())
            .field("saesrst", &self.saesrst())
            .field("octospimrst", &self.octospimrst())
            .field("otfdec1rst", &self.otfdec1rst())
            .field("otfdec2rst", &self.otfdec2rst())
            .field("sdmmc1rst", &self.sdmmc1rst())
            .field("sdmmc2rst", &self.sdmmc2rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<AHB2RSTR1rs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - I/O port B reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<AHB2RSTR1rs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - I/O port C reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<AHB2RSTR1rs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - I/O port D reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<AHB2RSTR1rs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - I/O port E reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<AHB2RSTR1rs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - I/O port F reset This bit is set and cleared by software. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<AHB2RSTR1rs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - I/O port G reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<AHB2RSTR1rs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 7 - I/O port H reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<AHB2RSTR1rs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 8 - I/O port I reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<AHB2RSTR1rs> {
        GPIOIRST_W::new(self, 8)
    }
    ///Bit 9 - I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<AHB2RSTR1rs> {
        GPIOJRST_W::new(self, 9)
    }
    ///Bit 10 - ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W<AHB2RSTR1rs> {
        ADC12RST_W::new(self, 10)
    }
    ///Bit 12 - DCMI and PSSI reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W<AHB2RSTR1rs> {
        DCMI_PSSIRST_W::new(self, 12)
    }
    ///Bit 14 - OTG_FS or OTG_HS reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otgrst(&mut self) -> OTGRST_W<AHB2RSTR1rs> {
        OTGRST_W::new(self, 14)
    }
    ///Bit 16 - AES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W<AHB2RSTR1rs> {
        AESRST_W::new(self, 16)
    }
    ///Bit 17 - HASH reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn hashrst(&mut self) -> HASHRST_W<AHB2RSTR1rs> {
        HASHRST_W::new(self, 17)
    }
    ///Bit 18 - RNG reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB2RSTR1rs> {
        RNGRST_W::new(self, 18)
    }
    ///Bit 19 - PKA reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn pkarst(&mut self) -> PKARST_W<AHB2RSTR1rs> {
        PKARST_W::new(self, 19)
    }
    ///Bit 20 - SAES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saesrst(&mut self) -> SAESRST_W<AHB2RSTR1rs> {
        SAESRST_W::new(self, 20)
    }
    ///Bit 21 - OCTOSPIM reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospimrst(&mut self) -> OCTOSPIMRST_W<AHB2RSTR1rs> {
        OCTOSPIMRST_W::new(self, 21)
    }
    ///Bit 23 - OTFDEC1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec1rst(&mut self) -> OTFDEC1RST_W<AHB2RSTR1rs> {
        OTFDEC1RST_W::new(self, 23)
    }
    ///Bit 24 - OTFDEC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otfdec2rst(&mut self) -> OTFDEC2RST_W<AHB2RSTR1rs> {
        OTFDEC2RST_W::new(self, 24)
    }
    ///Bit 27 - SDMMC1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<AHB2RSTR1rs> {
        SDMMC1RST_W::new(self, 27)
    }
    ///Bit 28 - SDMMC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<AHB2RSTR1rs> {
        SDMMC2RST_W::new(self, 28)
    }
}
/**RCC AHB2 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:AHB2RSTR1)*/
pub struct AHB2RSTR1rs;
impl crate::RegisterSpec for AHB2RSTR1rs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr1::R`](R) reader structure
impl crate::Readable for AHB2RSTR1rs {}
///`write(|w| ..)` method takes [`ahb2rstr1::W`](W) writer structure
impl crate::Writable for AHB2RSTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR1 to value 0
impl crate::Resettable for AHB2RSTR1rs {}
