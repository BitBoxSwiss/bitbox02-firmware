///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**MSIS clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. This bit is set by hardware to force the�MSIS oscillator on when exiting Standby or Shutdown mode. It is set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes, or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSISON {
    ///0: MSIS (MSI system) oscillator off
    Disabled = 0,
    ///1: MSIS (MSI system) oscillator on
    Enabled = 1,
}
impl From<MSISON> for bool {
    #[inline(always)]
    fn from(variant: MSISON) -> Self {
        variant as u8 != 0
    }
}
///Field `MSISON` reader - MSIS clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. This bit is set by hardware to force the�MSIS oscillator on when exiting Standby or Shutdown mode. It is set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes, or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.
pub type MSISON_R = crate::BitReader<MSISON>;
impl MSISON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSISON {
        match self.bits {
            false => MSISON::Disabled,
            true => MSISON::Enabled,
        }
    }
    ///MSIS (MSI system) oscillator off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSISON::Disabled
    }
    ///MSIS (MSI system) oscillator on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSISON::Enabled
    }
}
///Field `MSISON` writer - MSIS clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. This bit is set by hardware to force the�MSIS oscillator on when exiting Standby or Shutdown mode. It is set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes, or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.
pub type MSISON_W<'a, REG> = crate::BitWriter<'a, REG, MSISON>;
impl<'a, REG> MSISON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSIS (MSI system) oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSISON::Disabled)
    }
    ///MSIS (MSI system) oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSISON::Enabled)
    }
}
/**MSI enable for some peripheral kernels This bit is set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI on in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see Section�11.4.24 for more details). This bit must be configured at 0 before entering Stop 3 mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIKERON {
    ///0: No effect on MSI oscillator
    NotForced = 0,
    ///1: MSI oscillator forced ON even in Stop mode
    Forced = 1,
}
impl From<MSIKERON> for bool {
    #[inline(always)]
    fn from(variant: MSIKERON) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIKERON` reader - MSI enable for some peripheral kernels This bit is set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI on in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see Section�11.4.24 for more details). This bit must be configured at 0 before entering Stop 3 mode.
pub type MSIKERON_R = crate::BitReader<MSIKERON>;
impl MSIKERON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIKERON {
        match self.bits {
            false => MSIKERON::NotForced,
            true => MSIKERON::Forced,
        }
    }
    ///No effect on MSI oscillator
    #[inline(always)]
    pub fn is_not_forced(&self) -> bool {
        *self == MSIKERON::NotForced
    }
    ///MSI oscillator forced ON even in Stop mode
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == MSIKERON::Forced
    }
}
///Field `MSIKERON` writer - MSI enable for some peripheral kernels This bit is set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI on in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see Section�11.4.24 for more details). This bit must be configured at 0 before entering Stop 3 mode.
pub type MSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, MSIKERON>;
impl<'a, REG> MSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on MSI oscillator
    #[inline(always)]
    pub fn not_forced(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKERON::NotForced)
    }
    ///MSI oscillator forced ON even in Stop mode
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKERON::Forced)
    }
}
/**MSIS clock ready flag This bit is set by hardware to indicate that the MSIS oscillator is stable. It is set only when MSIS is enabled by software (by setting MSISON). Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSISRDYR {
    ///0: MSIS (MSI system) oscillator not ready
    NotReady = 0,
    ///1: MSIS (MSI system) oscillator ready
    Ready = 1,
}
impl From<MSISRDYR> for bool {
    #[inline(always)]
    fn from(variant: MSISRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `MSISRDY` reader - MSIS clock ready flag This bit is set by hardware to indicate that the MSIS oscillator is stable. It is set only when MSIS is enabled by software (by setting MSISON). Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles.
pub type MSISRDY_R = crate::BitReader<MSISRDYR>;
impl MSISRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSISRDYR {
        match self.bits {
            false => MSISRDYR::NotReady,
            true => MSISRDYR::Ready,
        }
    }
    ///MSIS (MSI system) oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSISRDYR::NotReady
    }
    ///MSIS (MSI system) oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSISRDYR::Ready
    }
}
/**MSI clock PLL-mode enable This bit is set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIPLLEN {
    ///0: MSI PLL-mode OFF
    Disabled = 0,
    ///1: MSI PLL-mode ON
    Enabled = 1,
}
impl From<MSIPLLEN> for bool {
    #[inline(always)]
    fn from(variant: MSIPLLEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIPLLEN` reader - MSI clock PLL-mode enable This bit is set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).
pub type MSIPLLEN_R = crate::BitReader<MSIPLLEN>;
impl MSIPLLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIPLLEN {
        match self.bits {
            false => MSIPLLEN::Disabled,
            true => MSIPLLEN::Enabled,
        }
    }
    ///MSI PLL-mode OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSIPLLEN::Disabled
    }
    ///MSI PLL-mode ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSIPLLEN::Enabled
    }
}
///Field `MSIPLLEN` writer - MSI clock PLL-mode enable This bit is set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).
pub type MSIPLLEN_W<'a, REG> = crate::BitWriter<'a, REG, MSIPLLEN>;
impl<'a, REG> MSIPLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI PLL-mode OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLEN::Disabled)
    }
    ///MSI PLL-mode ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLEN::Enabled)
    }
}
/**MSIK clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIK when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. It is set by hardware to force the MSIK oscillator on when STOPWUCK = 0 or STOPKERWUCK�=�0 when exiting Stop modes, or in case of a failure of the HSE oscillator.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIKON {
    ///0: MSIK (MSI kernel) oscillator disabled
    Disabled = 0,
    ///1: MSIK (MSI kernel) oscillator enabled
    Enabled = 1,
}
impl From<MSIKON> for bool {
    #[inline(always)]
    fn from(variant: MSIKON) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIKON` reader - MSIK clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIK when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. It is set by hardware to force the MSIK oscillator on when STOPWUCK = 0 or STOPKERWUCK�=�0 when exiting Stop modes, or in case of a failure of the HSE oscillator.
pub type MSIKON_R = crate::BitReader<MSIKON>;
impl MSIKON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIKON {
        match self.bits {
            false => MSIKON::Disabled,
            true => MSIKON::Enabled,
        }
    }
    ///MSIK (MSI kernel) oscillator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSIKON::Disabled
    }
    ///MSIK (MSI kernel) oscillator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSIKON::Enabled
    }
}
///Field `MSIKON` writer - MSIK clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIK when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. It is set by hardware to force the MSIK oscillator on when STOPWUCK = 0 or STOPKERWUCK�=�0 when exiting Stop modes, or in case of a failure of the HSE oscillator.
pub type MSIKON_W<'a, REG> = crate::BitWriter<'a, REG, MSIKON>;
impl<'a, REG> MSIKON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSIK (MSI kernel) oscillator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKON::Disabled)
    }
    ///MSIK (MSI kernel) oscillator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIKON::Enabled)
    }
}
/**MSIK clock ready flag This bit is set by hardware to indicate that the MSIK is stable. It is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIKRDYR {
    ///0: MSIK (MSI kernel) oscillator not ready
    NotReady = 0,
    ///1: MSIK (MSI kernel) oscillator ready
    Ready = 1,
}
impl From<MSIKRDYR> for bool {
    #[inline(always)]
    fn from(variant: MSIKRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIKRDY` reader - MSIK clock ready flag This bit is set by hardware to indicate that the MSIK is stable. It is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles.
pub type MSIKRDY_R = crate::BitReader<MSIKRDYR>;
impl MSIKRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIKRDYR {
        match self.bits {
            false => MSIKRDYR::NotReady,
            true => MSIKRDYR::Ready,
        }
    }
    ///MSIK (MSI kernel) oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIKRDYR::NotReady
    }
    ///MSIK (MSI kernel) oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIKRDYR::Ready
    }
}
/**MSI clock with PLL mode selection This bit is set and cleared by software to select which MSI output clock uses the PLL mode. It�can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to both clock outputs.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIPLLSEL {
    ///0: PLL mode applied to MSIK (MSI kernel) clock output
    Msik = 0,
    ///1: PLL mode applied to MSIS (MSI system) clock output
    Msis = 1,
}
impl From<MSIPLLSEL> for bool {
    #[inline(always)]
    fn from(variant: MSIPLLSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIPLLSEL` reader - MSI clock with PLL mode selection This bit is set and cleared by software to select which MSI output clock uses the PLL mode. It�can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to both clock outputs.
pub type MSIPLLSEL_R = crate::BitReader<MSIPLLSEL>;
impl MSIPLLSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIPLLSEL {
        match self.bits {
            false => MSIPLLSEL::Msik,
            true => MSIPLLSEL::Msis,
        }
    }
    ///PLL mode applied to MSIK (MSI kernel) clock output
    #[inline(always)]
    pub fn is_msik(&self) -> bool {
        *self == MSIPLLSEL::Msik
    }
    ///PLL mode applied to MSIS (MSI system) clock output
    #[inline(always)]
    pub fn is_msis(&self) -> bool {
        *self == MSIPLLSEL::Msis
    }
}
///Field `MSIPLLSEL` writer - MSI clock with PLL mode selection This bit is set and cleared by software to select which MSI output clock uses the PLL mode. It�can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to both clock outputs.
pub type MSIPLLSEL_W<'a, REG> = crate::BitWriter<'a, REG, MSIPLLSEL>;
impl<'a, REG> MSIPLLSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL mode applied to MSIK (MSI kernel) clock output
    #[inline(always)]
    pub fn msik(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLSEL::Msik)
    }
    ///PLL mode applied to MSIS (MSI system) clock output
    #[inline(always)]
    pub fn msis(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLSEL::Msis)
    }
}
/**MSI PLL mode fast startup This bit is set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The�fast start-up is active when the MSI in PLL mode returns from switch off.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIPLLFAST {
    ///0: MSI PLL normal start-up
    Normal = 0,
    ///1: MSI PLL fast start-up
    Fast = 1,
}
impl From<MSIPLLFAST> for bool {
    #[inline(always)]
    fn from(variant: MSIPLLFAST) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIPLLFAST` reader - MSI PLL mode fast startup This bit is set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The�fast start-up is active when the MSI in PLL mode returns from switch off.
pub type MSIPLLFAST_R = crate::BitReader<MSIPLLFAST>;
impl MSIPLLFAST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIPLLFAST {
        match self.bits {
            false => MSIPLLFAST::Normal,
            true => MSIPLLFAST::Fast,
        }
    }
    ///MSI PLL normal start-up
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MSIPLLFAST::Normal
    }
    ///MSI PLL fast start-up
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == MSIPLLFAST::Fast
    }
}
///Field `MSIPLLFAST` writer - MSI PLL mode fast startup This bit is set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The�fast start-up is active when the MSI in PLL mode returns from switch off.
pub type MSIPLLFAST_W<'a, REG> = crate::BitWriter<'a, REG, MSIPLLFAST>;
impl<'a, REG> MSIPLLFAST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI PLL normal start-up
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLFAST::Normal)
    }
    ///MSI PLL fast start-up
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLFAST::Fast)
    }
}
/**HSI16 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the�HSI16 oscillator on when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    ///0: HSI16 oscillator off
    Disabled = 0,
    ///1: HSI16 oscillator on
    Enabled = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
///Field `HSION` reader - HSI16 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the�HSI16 oscillator on when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Disabled,
            true => HSION::Enabled,
        }
    }
    ///HSI16 oscillator off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION::Disabled
    }
    ///HSI16 oscillator on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION::Enabled
    }
}
///Field `HSION` writer - HSI16 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the�HSI16 oscillator on when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI16 oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Disabled)
    }
    ///HSI16 oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Enabled)
    }
}
/**HSI16 enable for some peripheral kernels This bit is set and cleared by software to force HSI16 ON even in Stop modes. Keeping HSI16 on in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to Section�11.4.24 for more details. This bit must be configured at 0 before entering Stop 3 mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON {
    ///0: No effect on HSI16 oscillator
    NotForced = 0,
    ///1: HSI16 oscillator forced on even in Stop mode
    Forced = 1,
}
impl From<HSIKERON> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIKERON` reader - HSI16 enable for some peripheral kernels This bit is set and cleared by software to force HSI16 ON even in Stop modes. Keeping HSI16 on in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to Section�11.4.24 for more details. This bit must be configured at 0 before entering Stop 3 mode.
pub type HSIKERON_R = crate::BitReader<HSIKERON>;
impl HSIKERON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERON {
        match self.bits {
            false => HSIKERON::NotForced,
            true => HSIKERON::Forced,
        }
    }
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn is_not_forced(&self) -> bool {
        *self == HSIKERON::NotForced
    }
    ///HSI16 oscillator forced on even in Stop mode
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == HSIKERON::Forced
    }
}
///Field `HSIKERON` writer - HSI16 enable for some peripheral kernels This bit is set and cleared by software to force HSI16 ON even in Stop modes. Keeping HSI16 on in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to Section�11.4.24 for more details. This bit must be configured at 0 before entering Stop 3 mode.
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, HSIKERON>;
impl<'a, REG> HSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn not_forced(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::NotForced)
    }
    ///HSI16 oscillator forced on even in Stop mode
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::Forced)
    }
}
/**HSI16 clock ready flag This bit is set by hardware to indicate that HSI16 oscillator is stable. It is set only when HSI16 is enabled by software (by setting HSION). Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    ///0: HSI16 oscillator not ready
    NotReady = 0,
    ///1: HSI16 oscillator ready
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDY` reader - HSI16 clock ready flag This bit is set by hardware to indicate that HSI16 oscillator is stable. It is set only when HSI16 is enabled by software (by setting HSION). Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.
pub type HSIRDY_R = crate::BitReader<HSIRDYR>;
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYR {
        match self.bits {
            false => HSIRDYR::NotReady,
            true => HSIRDYR::Ready,
        }
    }
    ///HSI16 oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    ///HSI16 oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
/**HSI48 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI48 when entering in Stop, Standby, or Shutdown modes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48ON {
    ///0: HSI48 oscillator off
    Disabled = 0,
    ///1: HSI48 oscillator on
    Enabled = 1,
}
impl From<HSI48ON> for bool {
    #[inline(always)]
    fn from(variant: HSI48ON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI48ON` reader - HSI48 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI48 when entering in Stop, Standby, or Shutdown modes.
pub type HSI48ON_R = crate::BitReader<HSI48ON>;
impl HSI48ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI48ON {
        match self.bits {
            false => HSI48ON::Disabled,
            true => HSI48ON::Enabled,
        }
    }
    ///HSI48 oscillator off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48ON::Disabled
    }
    ///HSI48 oscillator on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48ON::Enabled
    }
}
///Field `HSI48ON` writer - HSI48 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI48 when entering in Stop, Standby, or Shutdown modes.
pub type HSI48ON_W<'a, REG> = crate::BitWriter<'a, REG, HSI48ON>;
impl<'a, REG> HSI48ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI48 oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON::Disabled)
    }
    ///HSI48 oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON::Enabled)
    }
}
/**HSI48 clock ready flag This bit is set by hardware to indicate that HSI48 oscillator is stable. Itis set only when HSI48 is enabled by software (by setting HSI48ON).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYR {
    ///0: HSI48 oscillator not ready
    NotReady = 0,
    ///1: HSI48 oscillator ready
    Ready = 1,
}
impl From<HSI48RDYR> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI48RDY` reader - HSI48 clock ready flag This bit is set by hardware to indicate that HSI48 oscillator is stable. Itis set only when HSI48 is enabled by software (by setting HSI48ON).
pub type HSI48RDY_R = crate::BitReader<HSI48RDYR>;
impl HSI48RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYR {
        match self.bits {
            false => HSI48RDYR::NotReady,
            true => HSI48RDYR::Ready,
        }
    }
    ///HSI48 oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI48RDYR::NotReady
    }
    ///HSI48 oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI48RDYR::Ready
    }
}
/**SHSI clock enable This bit is set and cleared by software. It is cleared by hardware to stop the SHSI when entering in Stop, Standby, or Shutdown modes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHSION {
    ///0: SHSI oscillator off
    Disabled = 0,
    ///1: SHSI oscillator on
    Enabled = 1,
}
impl From<SHSION> for bool {
    #[inline(always)]
    fn from(variant: SHSION) -> Self {
        variant as u8 != 0
    }
}
///Field `SHSION` reader - SHSI clock enable This bit is set and cleared by software. It is cleared by hardware to stop the SHSI when entering in Stop, Standby, or Shutdown modes.
pub type SHSION_R = crate::BitReader<SHSION>;
impl SHSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHSION {
        match self.bits {
            false => SHSION::Disabled,
            true => SHSION::Enabled,
        }
    }
    ///SHSI oscillator off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SHSION::Disabled
    }
    ///SHSI oscillator on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SHSION::Enabled
    }
}
///Field `SHSION` writer - SHSI clock enable This bit is set and cleared by software. It is cleared by hardware to stop the SHSI when entering in Stop, Standby, or Shutdown modes.
pub type SHSION_W<'a, REG> = crate::BitWriter<'a, REG, SHSION>;
impl<'a, REG> SHSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SHSI oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SHSION::Disabled)
    }
    ///SHSI oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SHSION::Enabled)
    }
}
/**SHSI clock ready flag This bit is set by hardware to indicate that the SHSI oscillator is stable. It is set only when SHSI is enabled by software (by setting SHSION). Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHSIRDYR {
    ///0: SHSI oscillator not ready
    NotReady = 0,
    ///1: SHSI oscillator ready
    Ready = 1,
}
impl From<SHSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: SHSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `SHSIRDY` reader - SHSI clock ready flag This bit is set by hardware to indicate that the SHSI oscillator is stable. It is set only when SHSI is enabled by software (by setting SHSION). Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles.
pub type SHSIRDY_R = crate::BitReader<SHSIRDYR>;
impl SHSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHSIRDYR {
        match self.bits {
            false => SHSIRDYR::NotReady,
            true => SHSIRDYR::Ready,
        }
    }
    ///SHSI oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == SHSIRDYR::NotReady
    }
    ///SHSI oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SHSIRDYR::Ready
    }
}
/**HSE clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON {
    ///0: HSE oscillator off
    Disabled = 0,
    ///1: HSE oscillator on
    Enabled = 1,
}
impl From<HSEON> for bool {
    #[inline(always)]
    fn from(variant: HSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEON` reader - HSE clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
pub type HSEON_R = crate::BitReader<HSEON>;
impl HSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEON {
        match self.bits {
            false => HSEON::Disabled,
            true => HSEON::Enabled,
        }
    }
    ///HSE oscillator off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSEON::Disabled
    }
    ///HSE oscillator on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSEON::Enabled
    }
}
///Field `HSEON` writer - HSE clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG, HSEON>;
impl<'a, REG> HSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Disabled)
    }
    ///HSE oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Enabled)
    }
}
/**HSE clock ready flag This bit is set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYR {
    ///0: HSE oscillator not ready
    NotReady = 0,
    ///1: HSE oscillator ready
    Ready = 1,
}
impl From<HSERDYR> for bool {
    #[inline(always)]
    fn from(variant: HSERDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDY` reader - HSE clock ready flag This bit is set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles.
pub type HSERDY_R = crate::BitReader<HSERDYR>;
impl HSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYR {
        match self.bits {
            false => HSERDYR::NotReady,
            true => HSERDYR::Ready,
        }
    }
    ///HSE oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSERDYR::NotReady
    }
    ///HSE oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSERDYR::Ready
    }
}
/**HSE crystal oscillator bypass This bit is set and cleared by software to bypass the oscillator with an external clock. The�external clock must be enabled with the HSEON bit set, to be used by the device. This�bit can be written only if the HSE oscillator is disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    ///0: HSE crystal oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE crystal oscillator bypassed with external clock
    Bypassed = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEBYP` reader - HSE crystal oscillator bypass This bit is set and cleared by software to bypass the oscillator with an external clock. The�external clock must be enabled with the HSEON bit set, to be used by the device. This�bit can be written only if the HSE oscillator is disabled.
pub type HSEBYP_R = crate::BitReader<HSEBYP>;
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP {
        match self.bits {
            false => HSEBYP::NotBypassed,
            true => HSEBYP::Bypassed,
        }
    }
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP::Bypassed
    }
}
///Field `HSEBYP` writer - HSE crystal oscillator bypass This bit is set and cleared by software to bypass the oscillator with an external clock. The�external clock must be enabled with the HSEON bit set, to be used by the device. This�bit can be written only if the HSE oscillator is disabled.
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
/**Clock security system enable This bit is set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    ///0: Clock security system OFF (clock detector OFF)
    Disabled = 0,
    ///1: Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)
    Enabled = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSON` reader - Clock security system enable This bit is set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.
pub type CSSON_R = crate::BitReader<CSSON>;
impl CSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSON {
        match self.bits {
            false => CSSON::Disabled,
            true => CSSON::Enabled,
        }
    }
    ///Clock security system OFF (clock detector OFF)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSON::Disabled
    }
    ///Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSON::Enabled
    }
}
///Field `CSSON` writer - Clock security system enable This bit is set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock security system OFF (clock detector OFF)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Disabled)
    }
    ///Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Enabled)
    }
}
/**HSE external clock bypass mode This bit is set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEEXT {
    ///0: external HSE clock analog mode
    Analog = 0,
    ///1: external HSE clock digital mode (through I/O Schmitt trigger)
    Digital = 1,
}
impl From<HSEEXT> for bool {
    #[inline(always)]
    fn from(variant: HSEEXT) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEEXT` reader - HSE external clock bypass mode This bit is set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.
pub type HSEEXT_R = crate::BitReader<HSEEXT>;
impl HSEEXT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEEXT {
        match self.bits {
            false => HSEEXT::Analog,
            true => HSEEXT::Digital,
        }
    }
    ///external HSE clock analog mode
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == HSEEXT::Analog
    }
    ///external HSE clock digital mode (through I/O Schmitt trigger)
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == HSEEXT::Digital
    }
}
///Field `HSEEXT` writer - HSE external clock bypass mode This bit is set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.
pub type HSEEXT_W<'a, REG> = crate::BitWriter<'a, REG, HSEEXT>;
impl<'a, REG> HSEEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///external HSE clock analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(HSEEXT::Analog)
    }
    ///external HSE clock digital mode (through I/O Schmitt trigger)
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(HSEEXT::Digital)
    }
}
/**PLL1 enable This bit is set and cleared by software to enable the main PLL. It is cleared by hardware when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1ON {
    ///0: PLL1 OFF
    Disabled = 0,
    ///1: PLL1 ON
    Enabled = 1,
}
impl From<PLL1ON> for bool {
    #[inline(always)]
    fn from(variant: PLL1ON) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1ON` reader - PLL1 enable This bit is set and cleared by software to enable the main PLL. It is cleared by hardware when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.
pub type PLL1ON_R = crate::BitReader<PLL1ON>;
impl PLL1ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1ON {
        match self.bits {
            false => PLL1ON::Disabled,
            true => PLL1ON::Enabled,
        }
    }
    ///PLL1 OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL1ON::Disabled
    }
    ///PLL1 ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL1ON::Enabled
    }
}
///Field `PLL1ON` writer - PLL1 enable This bit is set and cleared by software to enable the main PLL. It is cleared by hardware when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.
pub type PLL1ON_W<'a, REG> = crate::BitWriter<'a, REG, PLL1ON>;
impl<'a, REG> PLL1ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL1 OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1ON::Disabled)
    }
    ///PLL1 ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1ON::Enabled)
    }
}
/**PLL1 clock ready flag This bit is set by hardware to indicate that the PLL1 is locked.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1RDYR {
    ///0: PLL1 unlocked
    Unlocked = 0,
    ///1: PLL1 locked
    Locked = 1,
}
impl From<PLL1RDYR> for bool {
    #[inline(always)]
    fn from(variant: PLL1RDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1RDY` reader - PLL1 clock ready flag This bit is set by hardware to indicate that the PLL1 is locked.
pub type PLL1RDY_R = crate::BitReader<PLL1RDYR>;
impl PLL1RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1RDYR {
        match self.bits {
            false => PLL1RDYR::Unlocked,
            true => PLL1RDYR::Locked,
        }
    }
    ///PLL1 unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLL1RDYR::Unlocked
    }
    ///PLL1 locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLL1RDYR::Locked
    }
}
/**PLL2 enable This bit is set and cleared by software to enable PLL2. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2ON {
    ///0: PLL2 OFF
    Disabled = 0,
    ///1: PLL2 ON
    Enabled = 1,
}
impl From<PLL2ON> for bool {
    #[inline(always)]
    fn from(variant: PLL2ON) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2ON` reader - PLL2 enable This bit is set and cleared by software to enable PLL2. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.
pub type PLL2ON_R = crate::BitReader<PLL2ON>;
impl PLL2ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2ON {
        match self.bits {
            false => PLL2ON::Disabled,
            true => PLL2ON::Enabled,
        }
    }
    ///PLL2 OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL2ON::Disabled
    }
    ///PLL2 ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL2ON::Enabled
    }
}
///Field `PLL2ON` writer - PLL2 enable This bit is set and cleared by software to enable PLL2. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.
pub type PLL2ON_W<'a, REG> = crate::BitWriter<'a, REG, PLL2ON>;
impl<'a, REG> PLL2ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL2 OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2ON::Disabled)
    }
    ///PLL2 ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2ON::Enabled)
    }
}
/**PLL2 clock ready flag This bit is set by hardware to indicate that the PLL2 is locked.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2RDYR {
    ///0: PLL2 unlocked
    Unlocked = 0,
    ///1: PLL2 locked
    Locked = 1,
}
impl From<PLL2RDYR> for bool {
    #[inline(always)]
    fn from(variant: PLL2RDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2RDY` reader - PLL2 clock ready flag This bit is set by hardware to indicate that the PLL2 is locked.
pub type PLL2RDY_R = crate::BitReader<PLL2RDYR>;
impl PLL2RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2RDYR {
        match self.bits {
            false => PLL2RDYR::Unlocked,
            true => PLL2RDYR::Locked,
        }
    }
    ///PLL2 unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLL2RDYR::Unlocked
    }
    ///PLL2 locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLL2RDYR::Locked
    }
}
/**PLL3 enable This bit is set and cleared by software to enable PLL3. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3ON {
    ///0: PLL3 OFF
    Disabled = 0,
    ///1: PLL3 ON
    Enabled = 1,
}
impl From<PLL3ON> for bool {
    #[inline(always)]
    fn from(variant: PLL3ON) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3ON` reader - PLL3 enable This bit is set and cleared by software to enable PLL3. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.
pub type PLL3ON_R = crate::BitReader<PLL3ON>;
impl PLL3ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3ON {
        match self.bits {
            false => PLL3ON::Disabled,
            true => PLL3ON::Enabled,
        }
    }
    ///PLL3 OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL3ON::Disabled
    }
    ///PLL3 ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL3ON::Enabled
    }
}
///Field `PLL3ON` writer - PLL3 enable This bit is set and cleared by software to enable PLL3. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.
pub type PLL3ON_W<'a, REG> = crate::BitWriter<'a, REG, PLL3ON>;
impl<'a, REG> PLL3ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL3 OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3ON::Disabled)
    }
    ///PLL3 ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3ON::Enabled)
    }
}
/**PLL3 clock ready flag This bit is set by hardware to indicate that the PLL3 is locked.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3RDYR {
    ///0: PLL3 unlocked
    Unlocked = 0,
    ///1: PLL3 locked
    Locked = 1,
}
impl From<PLL3RDYR> for bool {
    #[inline(always)]
    fn from(variant: PLL3RDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3RDY` reader - PLL3 clock ready flag This bit is set by hardware to indicate that the PLL3 is locked.
pub type PLL3RDY_R = crate::BitReader<PLL3RDYR>;
impl PLL3RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3RDYR {
        match self.bits {
            false => PLL3RDYR::Unlocked,
            true => PLL3RDYR::Locked,
        }
    }
    ///PLL3 unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLL3RDYR::Unlocked
    }
    ///PLL3 locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLL3RDYR::Locked
    }
}
impl R {
    ///Bit 0 - MSIS clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. This bit is set by hardware to force the�MSIS oscillator on when exiting Standby or Shutdown mode. It is set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes, or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.
    #[inline(always)]
    pub fn msison(&self) -> MSISON_R {
        MSISON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSI enable for some peripheral kernels This bit is set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI on in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see Section�11.4.24 for more details). This bit must be configured at 0 before entering Stop 3 mode.
    #[inline(always)]
    pub fn msikeron(&self) -> MSIKERON_R {
        MSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSIS clock ready flag This bit is set by hardware to indicate that the MSIS oscillator is stable. It is set only when MSIS is enabled by software (by setting MSISON). Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles.
    #[inline(always)]
    pub fn msisrdy(&self) -> MSISRDY_R {
        MSISRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MSI clock PLL-mode enable This bit is set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MSIK clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIK when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. It is set by hardware to force the MSIK oscillator on when STOPWUCK = 0 or STOPKERWUCK�=�0 when exiting Stop modes, or in case of a failure of the HSE oscillator.
    #[inline(always)]
    pub fn msikon(&self) -> MSIKON_R {
        MSIKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSIK clock ready flag This bit is set by hardware to indicate that the MSIK is stable. It is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles.
    #[inline(always)]
    pub fn msikrdy(&self) -> MSIKRDY_R {
        MSIKRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MSI clock with PLL mode selection This bit is set and cleared by software to select which MSI output clock uses the PLL mode. It�can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to both clock outputs.
    #[inline(always)]
    pub fn msipllsel(&self) -> MSIPLLSEL_R {
        MSIPLLSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MSI PLL mode fast startup This bit is set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The�fast start-up is active when the MSI in PLL mode returns from switch off.
    #[inline(always)]
    pub fn msipllfast(&self) -> MSIPLLFAST_R {
        MSIPLLFAST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HSI16 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the�HSI16 oscillator on when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI16 enable for some peripheral kernels This bit is set and cleared by software to force HSI16 ON even in Stop modes. Keeping HSI16 on in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to Section�11.4.24 for more details. This bit must be configured at 0 before entering Stop 3 mode.
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI16 clock ready flag This bit is set by hardware to indicate that HSI16 oscillator is stable. It is set only when HSI16 is enabled by software (by setting HSION). Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - HSI48 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI48 when entering in Stop, Standby, or Shutdown modes.
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HSI48 clock ready flag This bit is set by hardware to indicate that HSI48 oscillator is stable. Itis set only when HSI48 is enabled by software (by setting HSI48ON).
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SHSI clock enable This bit is set and cleared by software. It is cleared by hardware to stop the SHSI when entering in Stop, Standby, or Shutdown modes.
    #[inline(always)]
    pub fn shsion(&self) -> SHSION_R {
        SHSION_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SHSI clock ready flag This bit is set by hardware to indicate that the SHSI oscillator is stable. It is set only when SHSI is enabled by software (by setting SHSION). Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles.
    #[inline(always)]
    pub fn shsirdy(&self) -> SHSIRDY_R {
        SHSIRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - HSE clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag This bit is set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles.
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE crystal oscillator bypass This bit is set and cleared by software to bypass the oscillator with an external clock. The�external clock must be enabled with the HSEON bit set, to be used by the device. This�bit can be written only if the HSE oscillator is disabled.
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clock security system enable This bit is set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HSE external clock bypass mode This bit is set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.
    #[inline(always)]
    pub fn hseext(&self) -> HSEEXT_R {
        HSEEXT_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - PLL1 enable This bit is set and cleared by software to enable the main PLL. It is cleared by hardware when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL1 clock ready flag This bit is set by hardware to indicate that the PLL1 is locked.
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PLL2 enable This bit is set and cleared by software to enable PLL2. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PLL2 clock ready flag This bit is set by hardware to indicate that the PLL2 is locked.
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PLL3 enable This bit is set and cleared by software to enable PLL3. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.
    #[inline(always)]
    pub fn pll3on(&self) -> PLL3ON_R {
        PLL3ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - PLL3 clock ready flag This bit is set by hardware to indicate that the PLL3 is locked.
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("msison", &self.msison())
            .field("msikeron", &self.msikeron())
            .field("msisrdy", &self.msisrdy())
            .field("msipllen", &self.msipllen())
            .field("msikon", &self.msikon())
            .field("msikrdy", &self.msikrdy())
            .field("msipllsel", &self.msipllsel())
            .field("msipllfast", &self.msipllfast())
            .field("hsion", &self.hsion())
            .field("hsikeron", &self.hsikeron())
            .field("hsirdy", &self.hsirdy())
            .field("hsi48on", &self.hsi48on())
            .field("hsi48rdy", &self.hsi48rdy())
            .field("shsion", &self.shsion())
            .field("shsirdy", &self.shsirdy())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .field("hsebyp", &self.hsebyp())
            .field("csson", &self.csson())
            .field("hseext", &self.hseext())
            .field("pll1on", &self.pll1on())
            .field("pll1rdy", &self.pll1rdy())
            .field("pll2on", &self.pll2on())
            .field("pll2rdy", &self.pll2rdy())
            .field("pll3on", &self.pll3on())
            .field("pll3rdy", &self.pll3rdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - MSIS clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. This bit is set by hardware to force the�MSIS oscillator on when exiting Standby or Shutdown mode. It is set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes, or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.
    #[inline(always)]
    pub fn msison(&mut self) -> MSISON_W<CRrs> {
        MSISON_W::new(self, 0)
    }
    ///Bit 1 - MSI enable for some peripheral kernels This bit is set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI on in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see Section�11.4.24 for more details). This bit must be configured at 0 before entering Stop 3 mode.
    #[inline(always)]
    pub fn msikeron(&mut self) -> MSIKERON_W<CRrs> {
        MSIKERON_W::new(self, 1)
    }
    ///Bit 3 - MSI clock PLL-mode enable This bit is set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).
    #[inline(always)]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<CRrs> {
        MSIPLLEN_W::new(self, 3)
    }
    ///Bit 4 - MSIK clock enable This bit is set and cleared by software. It is cleared by hardware to stop the MSIK when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. It is set by hardware to force the MSIK oscillator on when STOPWUCK = 0 or STOPKERWUCK�=�0 when exiting Stop modes, or in case of a failure of the HSE oscillator.
    #[inline(always)]
    pub fn msikon(&mut self) -> MSIKON_W<CRrs> {
        MSIKON_W::new(self, 4)
    }
    ///Bit 6 - MSI clock with PLL mode selection This bit is set and cleared by software to select which MSI output clock uses the PLL mode. It�can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to both clock outputs.
    #[inline(always)]
    pub fn msipllsel(&mut self) -> MSIPLLSEL_W<CRrs> {
        MSIPLLSEL_W::new(self, 6)
    }
    ///Bit 7 - MSI PLL mode fast startup This bit is set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The�fast start-up is active when the MSI in PLL mode returns from switch off.
    #[inline(always)]
    pub fn msipllfast(&mut self) -> MSIPLLFAST_W<CRrs> {
        MSIPLLFAST_W::new(self, 7)
    }
    ///Bit 8 - HSI16 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the�HSI16 oscillator on when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI16 enable for some peripheral kernels This bit is set and cleared by software to force HSI16 ON even in Stop modes. Keeping HSI16 on in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to Section�11.4.24 for more details. This bit must be configured at 0 before entering Stop 3 mode.
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bit 12 - HSI48 clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSI48 when entering in Stop, Standby, or Shutdown modes.
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W<CRrs> {
        HSI48ON_W::new(self, 12)
    }
    ///Bit 14 - SHSI clock enable This bit is set and cleared by software. It is cleared by hardware to stop the SHSI when entering in Stop, Standby, or Shutdown modes.
    #[inline(always)]
    pub fn shsion(&mut self) -> SHSION_W<CRrs> {
        SHSION_W::new(self, 14)
    }
    ///Bit 16 - HSE clock enable This bit is set and cleared by software. It is cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE crystal oscillator bypass This bit is set and cleared by software to bypass the oscillator with an external clock. The�external clock must be enabled with the HSEON bit set, to be used by the device. This�bit can be written only if the HSE oscillator is disabled.
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - Clock security system enable This bit is set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 20 - HSE external clock bypass mode This bit is set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.
    #[inline(always)]
    pub fn hseext(&mut self) -> HSEEXT_W<CRrs> {
        HSEEXT_W::new(self, 20)
    }
    ///Bit 24 - PLL1 enable This bit is set and cleared by software to enable the main PLL. It is cleared by hardware when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.
    #[inline(always)]
    pub fn pll1on(&mut self) -> PLL1ON_W<CRrs> {
        PLL1ON_W::new(self, 24)
    }
    ///Bit 26 - PLL2 enable This bit is set and cleared by software to enable PLL2. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.
    #[inline(always)]
    pub fn pll2on(&mut self) -> PLL2ON_W<CRrs> {
        PLL2ON_W::new(self, 26)
    }
    ///Bit 28 - PLL3 enable This bit is set and cleared by software to enable PLL3. It is cleared by hardware when entering Stop, Standby, or Shutdown mode.
    #[inline(always)]
    pub fn pll3on(&mut self) -> PLL3ON_W<CRrs> {
        PLL3ON_W::new(self, 28)
    }
}
/**RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x35
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x35;
}
