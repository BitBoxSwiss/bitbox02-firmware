///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    ///0: MSIS selected as system clock
    Msis = 0,
    ///1: HSI16 selected as system clock
    Hsi16 = 1,
    ///2: HSE selected as system clock
    Hse = 2,
    ///3: PLL pll1_r_ck selected as system clock
    Pll = 3,
}
impl From<SW> for u8 {
    #[inline(always)]
    fn from(variant: SW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SW {
    type Ux = u8;
}
impl crate::IsEnum for SW {}
///Field `SW` reader - system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SW {
        match self.bits {
            0 => SW::Msis,
            1 => SW::Hsi16,
            2 => SW::Hse,
            3 => SW::Pll,
            _ => unreachable!(),
        }
    }
    ///MSIS selected as system clock
    #[inline(always)]
    pub fn is_msis(&self) -> bool {
        *self == SW::Msis
    }
    ///HSI16 selected as system clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SW::Hsi16
    }
    ///HSE selected as system clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW::Hse
    }
    ///PLL pll1_r_ck selected as system clock
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW::Pll
    }
}
///Field `SW` writer - system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SW, crate::Safe>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MSIS selected as system clock
    #[inline(always)]
    pub fn msis(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Msis)
    }
    ///HSI16 selected as system clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsi16)
    }
    ///HSE selected as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse)
    }
    ///PLL pll1_r_ck selected as system clock
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pll)
    }
}
/**system clock switch status This bitfield is set and cleared by hardware to indicate which clock source is used as system clock.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR {
    ///0: MSIS oscillator used as system clock
    Msis = 0,
    ///1: HSI16 oscillator used as system clock
    Hsi16 = 1,
    ///2: HSE used as system clock
    Hse = 2,
    ///3: PLL pll1_r_ck used as system clock
    Pll = 3,
}
impl From<SWSR> for u8 {
    #[inline(always)]
    fn from(variant: SWSR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWSR {
    type Ux = u8;
}
impl crate::IsEnum for SWSR {}
///Field `SWS` reader - system clock switch status This bitfield is set and cleared by hardware to indicate which clock source is used as system clock.
pub type SWS_R = crate::FieldReader<SWSR>;
impl SWS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWSR {
        match self.bits {
            0 => SWSR::Msis,
            1 => SWSR::Hsi16,
            2 => SWSR::Hse,
            3 => SWSR::Pll,
            _ => unreachable!(),
        }
    }
    ///MSIS oscillator used as system clock
    #[inline(always)]
    pub fn is_msis(&self) -> bool {
        *self == SWSR::Msis
    }
    ///HSI16 oscillator used as system clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SWSR::Hsi16
    }
    ///HSE used as system clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR::Hse
    }
    ///PLL pll1_r_ck used as system clock
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWSR::Pll
    }
}
/**wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK {
    ///0: MSIS oscillator selected as wake-up from stop clock and CSS backup clock
    Msis = 0,
    ///1: HSI16 oscillator selected as wake-up from stop clock and CSS backup clock
    Hsi16 = 1,
}
impl From<STOPWUCK> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPWUCK` reader - wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).
pub type STOPWUCK_R = crate::BitReader<STOPWUCK>;
impl STOPWUCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOPWUCK {
        match self.bits {
            false => STOPWUCK::Msis,
            true => STOPWUCK::Hsi16,
        }
    }
    ///MSIS oscillator selected as wake-up from stop clock and CSS backup clock
    #[inline(always)]
    pub fn is_msis(&self) -> bool {
        *self == STOPWUCK::Msis
    }
    ///HSI16 oscillator selected as wake-up from stop clock and CSS backup clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == STOPWUCK::Hsi16
    }
}
///Field `STOPWUCK` writer - wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPWUCK>;
impl<'a, REG> STOPWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSIS oscillator selected as wake-up from stop clock and CSS backup clock
    #[inline(always)]
    pub fn msis(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Msis)
    }
    ///HSI16 oscillator selected as wake-up from stop clock and CSS backup clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK::Hsi16)
    }
}
/**wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPKERWUCK {
    ///0: MSIK oscillator automatically enabled when exiting Stop mode or when a CSS on HSE event occurs.
    Msik = 0,
    ///1: HSI16 oscillator automatically enabled when exiting Stop mode or when a CSS on HSE event occurs.
    Hsi16 = 1,
}
impl From<STOPKERWUCK> for bool {
    #[inline(always)]
    fn from(variant: STOPKERWUCK) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPKERWUCK` reader - wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
pub type STOPKERWUCK_R = crate::BitReader<STOPKERWUCK>;
impl STOPKERWUCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOPKERWUCK {
        match self.bits {
            false => STOPKERWUCK::Msik,
            true => STOPKERWUCK::Hsi16,
        }
    }
    ///MSIK oscillator automatically enabled when exiting Stop mode or when a CSS on HSE event occurs.
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == STOPKERWUCK::Msik
    }
    ///HSI16 oscillator automatically enabled when exiting Stop mode or when a CSS on HSE event occurs.
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == STOPKERWUCK::Hsi16
    }
}
///Field `STOPKERWUCK` writer - wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
pub type STOPKERWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPKERWUCK>;
impl<'a, REG> STOPKERWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSIK oscillator automatically enabled when exiting Stop mode or when a CSS on HSE event occurs.
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(STOPKERWUCK::Msik)
    }
    ///HSI16 oscillator automatically enabled when exiting Stop mode or when a CSS on HSE event occurs.
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(STOPKERWUCK::Hsi16)
    }
}
/**microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL {
    ///0: MCO output disabled, no clock on MCO
    None = 0,
    ///1: SYSCLK system clock selected
    Sysclk = 1,
    ///2: MSIS clock selected
    Msis = 2,
    ///3: HSI16 clock selected
    Hsi16 = 3,
    ///4: HSE clock selected
    Hse = 4,
    ///5: Main PLL clock pll1_r_ck selected
    Pll = 5,
    ///6: LSI clock selected
    Lsi = 6,
    ///7: LSE clock selected
    Lse = 7,
    ///8: Internal HSI48 clock selected
    Hsi48 = 8,
    ///9: MSIK clock selected
    Msik = 9,
}
impl From<MCOSEL> for u8 {
    #[inline(always)]
    fn from(variant: MCOSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOSEL {
    type Ux = u8;
}
impl crate::IsEnum for MCOSEL {}
///Field `MCOSEL` reader - microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_R = crate::FieldReader<MCOSEL>;
impl MCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCOSEL> {
        match self.bits {
            0 => Some(MCOSEL::None),
            1 => Some(MCOSEL::Sysclk),
            2 => Some(MCOSEL::Msis),
            3 => Some(MCOSEL::Hsi16),
            4 => Some(MCOSEL::Hse),
            5 => Some(MCOSEL::Pll),
            6 => Some(MCOSEL::Lsi),
            7 => Some(MCOSEL::Lse),
            8 => Some(MCOSEL::Hsi48),
            9 => Some(MCOSEL::Msik),
            _ => None,
        }
    }
    ///MCO output disabled, no clock on MCO
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MCOSEL::None
    }
    ///SYSCLK system clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCOSEL::Sysclk
    }
    ///MSIS clock selected
    #[inline(always)]
    pub fn is_msis(&self) -> bool {
        *self == MCOSEL::Msis
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == MCOSEL::Hsi16
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCOSEL::Hse
    }
    ///Main PLL clock pll1_r_ck selected
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCOSEL::Pll
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCOSEL::Lsi
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCOSEL::Lse
    }
    ///Internal HSI48 clock selected
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCOSEL::Hsi48
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == MCOSEL::Msik
    }
}
///Field `MCOSEL` writer - microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCOSEL>;
impl<'a, REG> MCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MCO output disabled, no clock on MCO
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::None)
    }
    ///SYSCLK system clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Sysclk)
    }
    ///MSIS clock selected
    #[inline(always)]
    pub fn msis(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Msis)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi16)
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hse)
    }
    ///Main PLL clock pll1_r_ck selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Pll)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lsi)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Lse)
    }
    ///Internal HSI48 clock selected
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Hsi48)
    }
    ///MSIK clock selected
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::Msik)
    }
}
/**microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE {
    ///0: MCO divided by 1
    Div1 = 0,
    ///1: MCO divided by 2
    Div2 = 1,
    ///2: MCO divided by 4
    Div4 = 2,
    ///3: MCO divided by 8
    Div8 = 3,
    ///4: MCO divided by 16
    Div16 = 4,
}
impl From<MCOPRE> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOPRE {
    type Ux = u8;
}
impl crate::IsEnum for MCOPRE {}
///Field `MCOPRE` reader - microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
pub type MCOPRE_R = crate::FieldReader<MCOPRE>;
impl MCOPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCOPRE> {
        match self.bits {
            0 => Some(MCOPRE::Div1),
            1 => Some(MCOPRE::Div2),
            2 => Some(MCOPRE::Div4),
            3 => Some(MCOPRE::Div8),
            4 => Some(MCOPRE::Div16),
            _ => None,
        }
    }
    ///MCO divided by 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE::Div1
    }
    ///MCO divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE::Div2
    }
    ///MCO divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE::Div4
    }
    ///MCO divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE::Div8
    }
    ///MCO divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRE::Div16
    }
}
///Field `MCOPRE` writer - microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCOPRE>;
impl<'a, REG> MCOPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MCO divided by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div1)
    }
    ///MCO divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div2)
    }
    ///MCO divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div4)
    }
    ///MCO divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div8)
    }
    ///MCO divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::Div16)
    }
}
impl R {
    ///Bits 0:1 - system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - system clock switch status This bitfield is set and cleared by hardware to indicate which clock source is used as system clock.
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 24:27 - microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("stopwuck", &self.stopwuck())
            .field("stopkerwuck", &self.stopkerwuck())
            .field("mcosel", &self.mcosel())
            .field("mcopre", &self.mcopre())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<CFGR1rs> {
        SW_W::new(self, 0)
    }
    ///Bit 4 - wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<CFGR1rs> {
        STOPWUCK_W::new(self, 4)
    }
    ///Bit 5 - wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
    #[inline(always)]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W<CFGR1rs> {
        STOPKERWUCK_W::new(self, 5)
    }
    ///Bits 24:27 - microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<CFGR1rs> {
        MCOSEL_W::new(self, 24)
    }
    ///Bits 28:30 - microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<CFGR1rs> {
        MCOPRE_W::new(self, 28)
    }
}
/**RCC clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RCC:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
