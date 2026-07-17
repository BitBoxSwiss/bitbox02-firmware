///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
/**LSE oscillator enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON {
    ///0: LSE oscillator off
    Disabled = 0,
    ///1: LSE oscillator on
    Enabled = 1,
}
impl From<LSEON> for bool {
    #[inline(always)]
    fn from(variant: LSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEON` reader - LSE oscillator enable This bit is set and cleared by software.
pub type LSEON_R = crate::BitReader<LSEON>;
impl LSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEON {
        match self.bits {
            false => LSEON::Disabled,
            true => LSEON::Enabled,
        }
    }
    ///LSE oscillator off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEON::Disabled
    }
    ///LSE oscillator on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEON::Enabled
    }
}
///Field `LSEON` writer - LSE oscillator enable This bit is set and cleared by software.
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG, LSEON>;
impl<'a, REG> LSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::Disabled)
    }
    ///LSE oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::Enabled)
    }
}
/**LSE oscillator ready This bit is set and cleared by hardware to indicate when the external 32�kHz oscillator is stable. After LSEON is cleared, this LSERDY bit goes low after six external low-speed oscillator clock cycles.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYR {
    ///0: LSE oscillator not ready
    NotReady = 0,
    ///1: LSE oscillator ready
    Ready = 1,
}
impl From<LSERDYR> for bool {
    #[inline(always)]
    fn from(variant: LSERDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDY` reader - LSE oscillator ready This bit is set and cleared by hardware to indicate when the external 32�kHz oscillator is stable. After LSEON is cleared, this LSERDY bit goes low after six external low-speed oscillator clock cycles.
pub type LSERDY_R = crate::BitReader<LSERDYR>;
impl LSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYR {
        match self.bits {
            false => LSERDYR::NotReady,
            true => LSERDYR::Ready,
        }
    }
    ///LSE oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDYR::NotReady
    }
    ///LSE oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDYR::Ready
    }
}
/**LSE oscillator bypass This bit is set and cleared by software to bypass oscillator in debug mode. It can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP {
    ///0: LSE oscillator not bypassed
    NotBypassed = 0,
    ///1: LSE oscillator bypassed
    Bypassed = 1,
}
impl From<LSEBYP> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEBYP` reader - LSE oscillator bypass This bit is set and cleared by software to bypass oscillator in debug mode. It can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
pub type LSEBYP_R = crate::BitReader<LSEBYP>;
impl LSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEBYP {
        match self.bits {
            false => LSEBYP::NotBypassed,
            true => LSEBYP::Bypassed,
        }
    }
    ///LSE oscillator not bypassed
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP::NotBypassed
    }
    ///LSE oscillator bypassed
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP::Bypassed
    }
}
///Field `LSEBYP` writer - LSE oscillator bypass This bit is set and cleared by software to bypass oscillator in debug mode. It can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, LSEBYP>;
impl<'a, REG> LSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::NotBypassed)
    }
    ///LSE oscillator bypassed
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::Bypassed)
    }
}
/**LSE oscillator drive capability This bitfield is set by software to modulate the drive capability of the LSE oscillator. It can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV {
    ///0: 'Xtal mode' lower driving capability
    Lower = 0,
    ///1: 'Xtal mode' medium-low driving capability
    MediumLow = 1,
    ///2: 'Xtal mode' medium-high driving capability
    MediumHigh = 2,
    ///3: 'Xtal mode' higher driving capability
    Higher = 3,
}
impl From<LSEDRV> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSEDRV {
    type Ux = u8;
}
impl crate::IsEnum for LSEDRV {}
///Field `LSEDRV` reader - LSE oscillator drive capability This bitfield is set by software to modulate the drive capability of the LSE oscillator. It can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode.
pub type LSEDRV_R = crate::FieldReader<LSEDRV>;
impl LSEDRV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEDRV {
        match self.bits {
            0 => LSEDRV::Lower,
            1 => LSEDRV::MediumLow,
            2 => LSEDRV::MediumHigh,
            3 => LSEDRV::Higher,
            _ => unreachable!(),
        }
    }
    ///'Xtal mode' lower driving capability
    #[inline(always)]
    pub fn is_lower(&self) -> bool {
        *self == LSEDRV::Lower
    }
    ///'Xtal mode' medium-low driving capability
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRV::MediumLow
    }
    ///'Xtal mode' medium-high driving capability
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRV::MediumHigh
    }
    ///'Xtal mode' higher driving capability
    #[inline(always)]
    pub fn is_higher(&self) -> bool {
        *self == LSEDRV::Higher
    }
}
///Field `LSEDRV` writer - LSE oscillator drive capability This bitfield is set by software to modulate the drive capability of the LSE oscillator. It can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode.
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LSEDRV, crate::Safe>;
impl<'a, REG> LSEDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///'Xtal mode' lower driving capability
    #[inline(always)]
    pub fn lower(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Lower)
    }
    ///'Xtal mode' medium-low driving capability
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MediumLow)
    }
    ///'Xtal mode' medium-high driving capability
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MediumHigh)
    }
    ///'Xtal mode' higher driving capability
    #[inline(always)]
    pub fn higher(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Higher)
    }
}
/**CSS on LSE enable This bit is set by software to enable the CSS on LSE. It must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable this LSECSSON bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON {
    ///0: CSS on LSE OFF
    Disabled = 0,
    ///1: CSS on LSE ON
    Enabled = 1,
}
impl From<LSECSSON> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSON` reader - CSS on LSE enable This bit is set by software to enable the CSS on LSE. It must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable this LSECSSON bit.
pub type LSECSSON_R = crate::BitReader<LSECSSON>;
impl LSECSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSON {
        match self.bits {
            false => LSECSSON::Disabled,
            true => LSECSSON::Enabled,
        }
    }
    ///CSS on LSE OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSON::Disabled
    }
    ///CSS on LSE ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSECSSON::Enabled
    }
}
///Field `LSECSSON` writer - CSS on LSE enable This bit is set by software to enable the CSS on LSE. It must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable this LSECSSON bit.
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSON>;
impl<'a, REG> LSECSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CSS on LSE OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::Disabled)
    }
    ///CSS on LSE ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::Enabled)
    }
}
/**CSS on LSE failure detection This bit is set by hardware to indicate when a failure is detected by the CCS on the external 32�kHz oscillator (LSE).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSDR {
    ///0: No failure detected on LSE
    NoFailure = 0,
    ///1: Failure detected on LSE
    Failure = 1,
}
impl From<LSECSSDR> for bool {
    #[inline(always)]
    fn from(variant: LSECSSDR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSD` reader - CSS on LSE failure detection This bit is set by hardware to indicate when a failure is detected by the CCS on the external 32�kHz oscillator (LSE).
pub type LSECSSD_R = crate::BitReader<LSECSSDR>;
impl LSECSSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSDR {
        match self.bits {
            false => LSECSSDR::NoFailure,
            true => LSECSSDR::Failure,
        }
    }
    ///No failure detected on LSE
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSDR::NoFailure
    }
    ///Failure detected on LSE
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSDR::Failure
    }
}
/**LSE system clock (LSESYS) enable This bit is set by software to enable always the LSE system clock generated by RCC, which can be used by any peripheral when its source clock is the LSE, or at system level if one of LSCOSEL, MCO, or MSI PLL mode is needed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSEN {
    ///0: LSE can be used only for RTC, TAMP, and CSS on LSE
    Disabled = 0,
    ///1: LSE can be used by any other peripheral or function
    Enabled = 1,
}
impl From<LSESYSEN> for bool {
    #[inline(always)]
    fn from(variant: LSESYSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LSESYSEN` reader - LSE system clock (LSESYS) enable This bit is set by software to enable always the LSE system clock generated by RCC, which can be used by any peripheral when its source clock is the LSE, or at system level if one of LSCOSEL, MCO, or MSI PLL mode is needed.
pub type LSESYSEN_R = crate::BitReader<LSESYSEN>;
impl LSESYSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSESYSEN {
        match self.bits {
            false => LSESYSEN::Disabled,
            true => LSESYSEN::Enabled,
        }
    }
    ///LSE can be used only for RTC, TAMP, and CSS on LSE
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSESYSEN::Disabled
    }
    ///LSE can be used by any other peripheral or function
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSESYSEN::Enabled
    }
}
///Field `LSESYSEN` writer - LSE system clock (LSESYS) enable This bit is set by software to enable always the LSE system clock generated by RCC, which can be used by any peripheral when its source clock is the LSE, or at system level if one of LSCOSEL, MCO, or MSI PLL mode is needed.
pub type LSESYSEN_W<'a, REG> = crate::BitWriter<'a, REG, LSESYSEN>;
impl<'a, REG> LSESYSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE can be used only for RTC, TAMP, and CSS on LSE
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSESYSEN::Disabled)
    }
    ///LSE can be used by any other peripheral or function
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSESYSEN::Enabled)
    }
}
/**RTC and TAMP clock source selection This bit is set by software to select the clock source for the RTC and TAMP. Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the�backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). BDRST bit can be used to reset them.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL {
    ///0: No clock selected
    NoClock = 0,
    ///1: LSE oscillator clock selected
    Lse = 1,
    ///2: LSI oscillator clock selected
    Lsi = 2,
    ///3: HSE oscillator clock divided by 32 selected
    HseDiv32 = 3,
}
impl From<RTCSEL> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSEL {
    type Ux = u8;
}
impl crate::IsEnum for RTCSEL {}
///Field `RTCSEL` reader - RTC and TAMP clock source selection This bit is set by software to select the clock source for the RTC and TAMP. Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the�backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). BDRST bit can be used to reset them.
pub type RTCSEL_R = crate::FieldReader<RTCSEL>;
impl RTCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCSEL {
        match self.bits {
            0 => RTCSEL::NoClock,
            1 => RTCSEL::Lse,
            2 => RTCSEL::Lsi,
            3 => RTCSEL::HseDiv32,
            _ => unreachable!(),
        }
    }
    ///No clock selected
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL::NoClock
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL::Lse
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL::Lsi
    }
    ///HSE oscillator clock divided by 32 selected
    #[inline(always)]
    pub fn is_hse_div32(&self) -> bool {
        *self == RTCSEL::HseDiv32
    }
}
///Field `RTCSEL` writer - RTC and TAMP clock source selection This bit is set by software to select the clock source for the RTC and TAMP. Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the�backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). BDRST bit can be used to reset them.
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTCSEL, crate::Safe>;
impl<'a, REG> RTCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock selected
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::NoClock)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lse)
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lsi)
    }
    ///HSE oscillator clock divided by 32 selected
    #[inline(always)]
    pub fn hse_div32(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::HseDiv32)
    }
}
/**LSE system clock (LSESYS) ready This bit is set and cleared by hardware to indicate when the LSE system clock is stable.When LSESYSEN is set, this LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSRDYR {
    ///0: LSESYS clock not ready
    NotReady = 0,
    ///1: LSESYS clock ready
    Ready = 1,
}
impl From<LSESYSRDYR> for bool {
    #[inline(always)]
    fn from(variant: LSESYSRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSESYSRDY` reader - LSE system clock (LSESYS) ready This bit is set and cleared by hardware to indicate when the LSE system clock is stable.When LSESYSEN is set, this LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.
pub type LSESYSRDY_R = crate::BitReader<LSESYSRDYR>;
impl LSESYSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSESYSRDYR {
        match self.bits {
            false => LSESYSRDYR::NotReady,
            true => LSESYSRDYR::Ready,
        }
    }
    ///LSESYS clock not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSESYSRDYR::NotReady
    }
    ///LSESYS clock ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSESYSRDYR::Ready
    }
}
/**LSE clock glitch filter enable This bit is set and cleared by hardware to enable the LSE glitch filter. It can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEGFON {
    ///0: LSE glitch filter disabled
    Disabled = 0,
    ///1: LSE glitch filter enabled
    Enabled = 1,
}
impl From<LSEGFON> for bool {
    #[inline(always)]
    fn from(variant: LSEGFON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEGFON` reader - LSE clock glitch filter enable This bit is set and cleared by hardware to enable the LSE glitch filter. It can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0).
pub type LSEGFON_R = crate::BitReader<LSEGFON>;
impl LSEGFON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEGFON {
        match self.bits {
            false => LSEGFON::Disabled,
            true => LSEGFON::Enabled,
        }
    }
    ///LSE glitch filter disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEGFON::Disabled
    }
    ///LSE glitch filter enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEGFON::Enabled
    }
}
///Field `LSEGFON` writer - LSE clock glitch filter enable This bit is set and cleared by hardware to enable the LSE glitch filter. It can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0).
pub type LSEGFON_W<'a, REG> = crate::BitWriter<'a, REG, LSEGFON>;
impl<'a, REG> LSEGFON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE glitch filter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSEGFON::Disabled)
    }
    ///LSE glitch filter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSEGFON::Enabled)
    }
}
/**RTC and TAMP clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN {
    ///0: RTC and TAMP clock disabled
    Disabled = 0,
    ///1: RTC and TAMP clock enabled
    Enabled = 1,
}
impl From<RTCEN> for bool {
    #[inline(always)]
    fn from(variant: RTCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCEN` reader - RTC and TAMP clock enable This bit is set and cleared by software.
pub type RTCEN_R = crate::BitReader<RTCEN>;
impl RTCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCEN {
        match self.bits {
            false => RTCEN::Disabled,
            true => RTCEN::Enabled,
        }
    }
    ///RTC and TAMP clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN::Disabled
    }
    ///RTC and TAMP clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN::Enabled
    }
}
///Field `RTCEN` writer - RTC and TAMP clock enable This bit is set and cleared by software.
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCEN>;
impl<'a, REG> RTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC and TAMP clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Disabled)
    }
    ///RTC and TAMP clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Enabled)
    }
}
/**Backup domain software reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST {
    ///0: Reset not activated
    NoReset = 0,
    ///1: Reset the entire backup domain
    Reset = 1,
}
impl From<BDRST> for bool {
    #[inline(always)]
    fn from(variant: BDRST) -> Self {
        variant as u8 != 0
    }
}
///Field `BDRST` reader - Backup domain software reset This bit is set and cleared by software.
pub type BDRST_R = crate::BitReader<BDRST>;
impl BDRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BDRST {
        match self.bits {
            false => BDRST::NoReset,
            true => BDRST::Reset,
        }
    }
    ///Reset not activated
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BDRST::NoReset
    }
    ///Reset the entire backup domain
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BDRST::Reset
    }
}
///Field `BDRST` writer - Backup domain software reset This bit is set and cleared by software.
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG, BDRST>;
impl<'a, REG> BDRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset not activated
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(BDRST::NoReset)
    }
    ///Reset the entire backup domain
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BDRST::Reset)
    }
}
/**Low-speed clock output (LSCO) enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOEN {
    ///0: LSCO disabled
    Disabled = 0,
    ///1: LSCO enabled
    Enabled = 1,
}
impl From<LSCOEN> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LSCOEN` reader - Low-speed clock output (LSCO) enable This bit is set and cleared by software.
pub type LSCOEN_R = crate::BitReader<LSCOEN>;
impl LSCOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSCOEN {
        match self.bits {
            false => LSCOEN::Disabled,
            true => LSCOEN::Enabled,
        }
    }
    ///LSCO disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSCOEN::Disabled
    }
    ///LSCO enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSCOEN::Enabled
    }
}
///Field `LSCOEN` writer - Low-speed clock output (LSCO) enable This bit is set and cleared by software.
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG, LSCOEN>;
impl<'a, REG> LSCOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSCO disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN::Disabled)
    }
    ///LSCO enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN::Enabled)
    }
}
/**Low-speed clock output selection This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOSEL {
    ///0: LSI clock selected
    Lsi = 0,
    ///1: LSE clock selected
    Lse = 1,
}
impl From<LSCOSEL> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `LSCOSEL` reader - Low-speed clock output selection This bit is set and cleared by software.
pub type LSCOSEL_R = crate::BitReader<LSCOSEL>;
impl LSCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSCOSEL {
        match self.bits {
            false => LSCOSEL::Lsi,
            true => LSCOSEL::Lse,
        }
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LSCOSEL::Lsi
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LSCOSEL::Lse
    }
}
///Field `LSCOSEL` writer - Low-speed clock output selection This bit is set and cleared by software.
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, LSCOSEL>;
impl<'a, REG> LSCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL::Lsi)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL::Lse)
    }
}
/**LSI oscillator enable This bit is set and cleared by software. The LSI oscillator is disabled 60��s maximum after the LSION bit is cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    ///0: LSI oscillator OFF
    Disabled = 0,
    ///1: LSI oscillator ON
    Enabled = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
///Field `LSION` reader - LSI oscillator enable This bit is set and cleared by software. The LSI oscillator is disabled 60��s maximum after the LSION bit is cleared.
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::Disabled,
            true => LSION::Enabled,
        }
    }
    ///LSI oscillator OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSION::Disabled
    }
    ///LSI oscillator ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSION::Enabled
    }
}
///Field `LSION` writer - LSI oscillator enable This bit is set and cleared by software. The LSI oscillator is disabled 60��s maximum after the LSION bit is cleared.
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI oscillator OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Disabled)
    }
    ///LSI oscillator ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Enabled)
    }
}
/**LSI oscillator ready This bit is set and cleared by hardware to indicate when the LSI oscillator is stable. After�LSION is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYR {
    ///0: LSI oscillator not ready
    NotReady = 0,
    ///1: LSI oscillator ready
    Ready = 1,
}
impl From<LSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDY` reader - LSI oscillator ready This bit is set and cleared by hardware to indicate when the LSI oscillator is stable. After�LSION is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
pub type LSIRDY_R = crate::BitReader<LSIRDYR>;
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYR {
        match self.bits {
            false => LSIRDYR::NotReady,
            true => LSIRDYR::Ready,
        }
    }
    ///LSI oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDYR::NotReady
    }
    ///LSI oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDYR::Ready
    }
}
///Field `LSIRDY` writer - LSI oscillator ready This bit is set and cleared by hardware to indicate when the LSI oscillator is stable. After�LSION is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
pub type LSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYR>;
impl<'a, REG> LSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI oscillator not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYR::NotReady)
    }
    ///LSI oscillator ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYR::Ready)
    }
}
/**Low-speed clock divider configuration This bit is set and cleared by software to enable the LSI division. It can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 μs after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIPREDIV {
    ///0: LSI not divided
    Div1 = 0,
    ///1: LSI divided by 128
    Div128 = 1,
}
impl From<LSIPREDIV> for bool {
    #[inline(always)]
    fn from(variant: LSIPREDIV) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIPREDIV` reader - Low-speed clock divider configuration This bit is set and cleared by software to enable the LSI division. It can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 μs after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.
pub type LSIPREDIV_R = crate::BitReader<LSIPREDIV>;
impl LSIPREDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIPREDIV {
        match self.bits {
            false => LSIPREDIV::Div1,
            true => LSIPREDIV::Div128,
        }
    }
    ///LSI not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LSIPREDIV::Div1
    }
    ///LSI divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LSIPREDIV::Div128
    }
}
///Field `LSIPREDIV` writer - Low-speed clock divider configuration This bit is set and cleared by software to enable the LSI division. It can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 μs after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.
pub type LSIPREDIV_W<'a, REG> = crate::BitWriter<'a, REG, LSIPREDIV>;
impl<'a, REG> LSIPREDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPREDIV::Div1)
    }
    ///LSI divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPREDIV::Div128)
    }
}
impl R {
    ///Bit 0 - LSE oscillator enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready This bit is set and cleared by hardware to indicate when the external 32�kHz oscillator is stable. After LSEON is cleared, this LSERDY bit goes low after six external low-speed oscillator clock cycles.
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass This bit is set and cleared by software to bypass oscillator in debug mode. It can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - LSE oscillator drive capability This bitfield is set by software to modulate the drive capability of the LSE oscillator. It can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode.
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - CSS on LSE enable This bit is set by software to enable the CSS on LSE. It must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable this LSECSSON bit.
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSS on LSE failure detection This bit is set by hardware to indicate when a failure is detected by the CCS on the external 32�kHz oscillator (LSE).
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LSE system clock (LSESYS) enable This bit is set by software to enable always the LSE system clock generated by RCC, which can be used by any peripheral when its source clock is the LSE, or at system level if one of LSCOSEL, MCO, or MSI PLL mode is needed.
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - RTC and TAMP clock source selection This bit is set by software to select the clock source for the RTC and TAMP. Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the�backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). BDRST bit can be used to reset them.
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - LSE system clock (LSESYS) ready This bit is set and cleared by hardware to indicate when the LSE system clock is stable.When LSESYSEN is set, this LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LSE clock glitch filter enable This bit is set and cleared by hardware to enable the LSE glitch filter. It can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0).
    #[inline(always)]
    pub fn lsegfon(&self) -> LSEGFON_R {
        LSEGFON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - RTC and TAMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Backup domain software reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low-speed clock output selection This bit is set and cleared by software.
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LSI oscillator enable This bit is set and cleared by software. The LSI oscillator is disabled 60��s maximum after the LSION bit is cleared.
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LSI oscillator ready This bit is set and cleared by hardware to indicate when the LSI oscillator is stable. After�LSION is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Low-speed clock divider configuration This bit is set and cleared by software to enable the LSI division. It can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 μs after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.
    #[inline(always)]
    pub fn lsiprediv(&self) -> LSIPREDIV_R {
        LSIPREDIV_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("lseon", &self.lseon())
            .field("lserdy", &self.lserdy())
            .field("lsebyp", &self.lsebyp())
            .field("lsedrv", &self.lsedrv())
            .field("lsecsson", &self.lsecsson())
            .field("lsecssd", &self.lsecssd())
            .field("lsesysen", &self.lsesysen())
            .field("rtcsel", &self.rtcsel())
            .field("lsesysrdy", &self.lsesysrdy())
            .field("lsegfon", &self.lsegfon())
            .field("rtcen", &self.rtcen())
            .field("bdrst", &self.bdrst())
            .field("lscoen", &self.lscoen())
            .field("lscosel", &self.lscosel())
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("lsiprediv", &self.lsiprediv())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<BDCRrs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 2 - LSE oscillator bypass This bit is set and cleared by software to bypass oscillator in debug mode. It can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bits 3:4 - LSE oscillator drive capability This bitfield is set by software to modulate the drive capability of the LSE oscillator. It can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode.
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    ///Bit 5 - CSS on LSE enable This bit is set by software to enable the CSS on LSE. It must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable this LSECSSON bit.
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    ///Bit 7 - LSE system clock (LSESYS) enable This bit is set by software to enable always the LSE system clock generated by RCC, which can be used by any peripheral when its source clock is the LSE, or at system level if one of LSCOSEL, MCO, or MSI PLL mode is needed.
    #[inline(always)]
    pub fn lsesysen(&mut self) -> LSESYSEN_W<BDCRrs> {
        LSESYSEN_W::new(self, 7)
    }
    ///Bits 8:9 - RTC and TAMP clock source selection This bit is set by software to select the clock source for the RTC and TAMP. Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the�backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). BDRST bit can be used to reset them.
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<BDCRrs> {
        RTCSEL_W::new(self, 8)
    }
    ///Bit 12 - LSE clock glitch filter enable This bit is set and cleared by hardware to enable the LSE glitch filter. It can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0).
    #[inline(always)]
    pub fn lsegfon(&mut self) -> LSEGFON_W<BDCRrs> {
        LSEGFON_W::new(self, 12)
    }
    ///Bit 15 - RTC and TAMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    ///Bit 16 - Backup domain software reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<BDCRrs> {
        BDRST_W::new(self, 16)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W<BDCRrs> {
        LSCOEN_W::new(self, 24)
    }
    ///Bit 25 - Low-speed clock output selection This bit is set and cleared by software.
    #[inline(always)]
    pub fn lscosel(&mut self) -> LSCOSEL_W<BDCRrs> {
        LSCOSEL_W::new(self, 25)
    }
    ///Bit 26 - LSI oscillator enable This bit is set and cleared by software. The LSI oscillator is disabled 60��s maximum after the LSION bit is cleared.
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<BDCRrs> {
        LSION_W::new(self, 26)
    }
    ///Bit 27 - LSI oscillator ready This bit is set and cleared by hardware to indicate when the LSI oscillator is stable. After�LSION is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
    #[inline(always)]
    pub fn lsirdy(&mut self) -> LSIRDY_W<BDCRrs> {
        LSIRDY_W::new(self, 27)
    }
    ///Bit 28 - Low-speed clock divider configuration This bit is set and cleared by software to enable the LSI division. It can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 μs after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.
    #[inline(always)]
    pub fn lsiprediv(&mut self) -> LSIPREDIV_W<BDCRrs> {
        LSIPREDIV_W::new(self, 28)
    }
}
/**RCC backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:BDCR)*/
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
///`read()` method returns [`bdcr::R`](R) reader structure
impl crate::Readable for BDCRrs {}
///`write(|w| ..)` method takes [`bdcr::W`](W) writer structure
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCRrs {}
