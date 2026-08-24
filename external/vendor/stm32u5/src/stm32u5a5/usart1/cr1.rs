///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**USART enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE {
    ///0: UART is disabled
    Disabled = 0,
    ///1: UART is enabled
    Enabled = 1,
}
impl From<UE> for bool {
    #[inline(always)]
    fn from(variant: UE) -> Self {
        variant as u8 != 0
    }
}
///Field `UE` reader - USART enable
pub type UE_R = crate::BitReader<UE>;
impl UE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UE {
        match self.bits {
            false => UE::Disabled,
            true => UE::Enabled,
        }
    }
    ///UART is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UE::Disabled
    }
    ///UART is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UE::Enabled
    }
}
///Field `UE` writer - USART enable
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG, UE>;
impl<'a, REG> UE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///UART is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UE::Disabled)
    }
    ///UART is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UE::Enabled)
    }
}
///Field `UESM` reader - USART enable in Stop mode
pub type UESM_R = crate::BitReader;
///Field `UESM` writer - USART enable in Stop mode
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Receiver enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE {
    ///0: Receiver is disabled
    Disabled = 0,
    ///1: Receiver is enabled
    Enabled = 1,
}
impl From<RE> for bool {
    #[inline(always)]
    fn from(variant: RE) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Receiver enable
pub type RE_R = crate::BitReader<RE>;
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RE {
        match self.bits {
            false => RE::Disabled,
            true => RE::Enabled,
        }
    }
    ///Receiver is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE::Disabled
    }
    ///Receiver is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE::Enabled
    }
}
///Field `RE` writer - Receiver enable
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiver is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Disabled)
    }
    ///Receiver is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Enabled)
    }
}
/**Transmitter enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE {
    ///0: Transmitter is disabled
    Disabled = 0,
    ///1: Transmitter is enabled
    Enabled = 1,
}
impl From<TE> for bool {
    #[inline(always)]
    fn from(variant: TE) -> Self {
        variant as u8 != 0
    }
}
///Field `TE` reader - Transmitter enable
pub type TE_R = crate::BitReader<TE>;
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TE {
        match self.bits {
            false => TE::Disabled,
            true => TE::Enabled,
        }
    }
    ///Transmitter is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE::Disabled
    }
    ///Transmitter is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE::Enabled
    }
}
///Field `TE` writer - Transmitter enable
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmitter is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Disabled)
    }
    ///Transmitter is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Enabled)
    }
}
/**IDLE interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever IDLE=1 in the ISR register
    Enabled = 1,
}
impl From<IDLEIE> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLEIE` reader - IDLE interrupt enable
pub type IDLEIE_R = crate::BitReader<IDLEIE>;
impl IDLEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLEIE {
        match self.bits {
            false => IDLEIE::Disabled,
            true => IDLEIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE::Disabled
    }
    ///Interrupt is generated whenever IDLE=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE::Enabled
    }
}
///Field `IDLEIE` writer - IDLE interrupt enable
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG, IDLEIE>;
impl<'a, REG> IDLEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::Disabled)
    }
    ///Interrupt is generated whenever IDLE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::Enabled)
    }
}
/**RXFIFO not empty interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    Enabled = 1,
}
impl From<RXNEIE> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNEIE` reader - RXFIFO not empty interrupt enable
pub type RXNEIE_R = crate::BitReader<RXNEIE>;
impl RXNEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNEIE {
        match self.bits {
            false => RXNEIE::Disabled,
            true => RXNEIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIE::Disabled
    }
    ///Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIE::Enabled
    }
}
///Field `RXNEIE` writer - RXFIFO not empty interrupt enable
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Disabled)
    }
    ///Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Enabled)
    }
}
/**Transmission complete interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever TC=1 in the ISR register
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transmission complete interrupt enable
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    ///Interrupt is generated whenever TC=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
///Field `TCIE` writer - Transmission complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    ///Interrupt is generated whenever TC=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
/**TXFIFO not full interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever TXE=1 in the ISR register
    Enabled = 1,
}
impl From<TXEIE> for bool {
    #[inline(always)]
    fn from(variant: TXEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXEIE` reader - TXFIFO not full interrupt enable
pub type TXEIE_R = crate::BitReader<TXEIE>;
impl TXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXEIE {
        match self.bits {
            false => TXEIE::Disabled,
            true => TXEIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIE::Disabled
    }
    ///Interrupt is generated whenever TXE=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIE::Enabled
    }
}
///Field `TXEIE` writer - TXFIFO not full interrupt enable
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Disabled)
    }
    ///Interrupt is generated whenever TXE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Enabled)
    }
}
/**PE interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever PE=1 in the ISR register
    Enabled = 1,
}
impl From<PEIE> for bool {
    #[inline(always)]
    fn from(variant: PEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PEIE` reader - PE interrupt enable
pub type PEIE_R = crate::BitReader<PEIE>;
impl PEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PEIE {
        match self.bits {
            false => PEIE::Disabled,
            true => PEIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE::Disabled
    }
    ///Interrupt is generated whenever PE=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEIE::Enabled
    }
}
///Field `PEIE` writer - PE interrupt enable
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG, PEIE>;
impl<'a, REG> PEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::Disabled)
    }
    ///Interrupt is generated whenever PE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::Enabled)
    }
}
/**Parity selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS {
    ///0: Even parity
    Even = 0,
    ///1: Odd parity
    Odd = 1,
}
impl From<PS> for bool {
    #[inline(always)]
    fn from(variant: PS) -> Self {
        variant as u8 != 0
    }
}
///Field `PS` reader - Parity selection
pub type PS_R = crate::BitReader<PS>;
impl PS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PS {
        match self.bits {
            false => PS::Even,
            true => PS::Odd,
        }
    }
    ///Even parity
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS::Even
    }
    ///Odd parity
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS::Odd
    }
}
///Field `PS` writer - Parity selection
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG, PS>;
impl<'a, REG> PS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Even parity
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PS::Even)
    }
    ///Odd parity
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PS::Odd)
    }
}
/**Parity control enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE {
    ///0: Parity control disabled
    Disabled = 0,
    ///1: Parity control enabled
    Enabled = 1,
}
impl From<PCE> for bool {
    #[inline(always)]
    fn from(variant: PCE) -> Self {
        variant as u8 != 0
    }
}
///Field `PCE` reader - Parity control enable
pub type PCE_R = crate::BitReader<PCE>;
impl PCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCE {
        match self.bits {
            false => PCE::Disabled,
            true => PCE::Enabled,
        }
    }
    ///Parity control disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCE::Disabled
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCE::Enabled
    }
}
///Field `PCE` writer - Parity control enable
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG, PCE>;
impl<'a, REG> PCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Parity control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCE::Disabled)
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCE::Enabled)
    }
}
/**Receiver wakeup method

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE {
    ///0: Idle line
    Idle = 0,
    ///1: Address mask
    Address = 1,
}
impl From<WAKE> for bool {
    #[inline(always)]
    fn from(variant: WAKE) -> Self {
        variant as u8 != 0
    }
}
///Field `WAKE` reader - Receiver wakeup method
pub type WAKE_R = crate::BitReader<WAKE>;
impl WAKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAKE {
        match self.bits {
            false => WAKE::Idle,
            true => WAKE::Address,
        }
    }
    ///Idle line
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WAKE::Idle
    }
    ///Address mask
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WAKE::Address
    }
}
///Field `WAKE` writer - Receiver wakeup method
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG, WAKE>;
impl<'a, REG> WAKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Idle line
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::Idle)
    }
    ///Address mask
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::Address)
    }
}
/**Word length

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M0 {
    ///0: 1 start bit, 8 data bits, n stop bits
    Bit8 = 0,
    ///1: 1 start bit, 9 data bits, n stop bits
    Bit9 = 1,
}
impl From<M0> for bool {
    #[inline(always)]
    fn from(variant: M0) -> Self {
        variant as u8 != 0
    }
}
///Field `M0` reader - Word length
pub type M0_R = crate::BitReader<M0>;
impl M0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> M0 {
        match self.bits {
            false => M0::Bit8,
            true => M0::Bit9,
        }
    }
    ///1 start bit, 8 data bits, n stop bits
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == M0::Bit8
    }
    ///1 start bit, 9 data bits, n stop bits
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == M0::Bit9
    }
}
///Field `M0` writer - Word length
pub type M0_W<'a, REG> = crate::BitWriter<'a, REG, M0>;
impl<'a, REG> M0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///1 start bit, 8 data bits, n stop bits
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(M0::Bit8)
    }
    ///1 start bit, 9 data bits, n stop bits
    #[inline(always)]
    pub fn bit9(self) -> &'a mut crate::W<REG> {
        self.variant(M0::Bit9)
    }
}
/**Mute mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MME {
    ///0: Receiver in active mode permanently
    Disabled = 0,
    ///1: Receiver can switch between mute mode and active mode
    Enabled = 1,
}
impl From<MME> for bool {
    #[inline(always)]
    fn from(variant: MME) -> Self {
        variant as u8 != 0
    }
}
///Field `MME` reader - Mute mode enable
pub type MME_R = crate::BitReader<MME>;
impl MME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MME {
        match self.bits {
            false => MME::Disabled,
            true => MME::Enabled,
        }
    }
    ///Receiver in active mode permanently
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MME::Disabled
    }
    ///Receiver can switch between mute mode and active mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MME::Enabled
    }
}
///Field `MME` writer - Mute mode enable
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG, MME>;
impl<'a, REG> MME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiver in active mode permanently
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MME::Disabled)
    }
    ///Receiver can switch between mute mode and active mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MME::Enabled)
    }
}
/**Character match interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated when the CMF bit is set in the ISR register
    Enabled = 1,
}
impl From<CMIE> for bool {
    #[inline(always)]
    fn from(variant: CMIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMIE` reader - Character match interrupt enable
pub type CMIE_R = crate::BitReader<CMIE>;
impl CMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMIE {
        match self.bits {
            false => CMIE::Disabled,
            true => CMIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMIE::Disabled
    }
    ///Interrupt is generated when the CMF bit is set in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMIE::Enabled
    }
}
///Field `CMIE` writer - Character match interrupt enable
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG, CMIE>;
impl<'a, REG> CMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMIE::Disabled)
    }
    ///Interrupt is generated when the CMF bit is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMIE::Enabled)
    }
}
/**Oversampling mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER8 {
    ///0: Oversampling by 16
    Oversampling16 = 0,
    ///1: Oversampling by 8
    Oversampling8 = 1,
}
impl From<OVER8> for bool {
    #[inline(always)]
    fn from(variant: OVER8) -> Self {
        variant as u8 != 0
    }
}
///Field `OVER8` reader - Oversampling mode
pub type OVER8_R = crate::BitReader<OVER8>;
impl OVER8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVER8 {
        match self.bits {
            false => OVER8::Oversampling16,
            true => OVER8::Oversampling8,
        }
    }
    ///Oversampling by 16
    #[inline(always)]
    pub fn is_oversampling16(&self) -> bool {
        *self == OVER8::Oversampling16
    }
    ///Oversampling by 8
    #[inline(always)]
    pub fn is_oversampling8(&self) -> bool {
        *self == OVER8::Oversampling8
    }
}
///Field `OVER8` writer - Oversampling mode
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG, OVER8>;
impl<'a, REG> OVER8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oversampling by 16
    #[inline(always)]
    pub fn oversampling16(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8::Oversampling16)
    }
    ///Oversampling by 8
    #[inline(always)]
    pub fn oversampling8(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8::Oversampling8)
    }
}
///Field `DEDT` reader - DEDT
pub type DEDT_R = crate::FieldReader;
///Field `DEDT` writer - DEDT
pub type DEDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `DEAT` reader - DEAT
pub type DEAT_R = crate::FieldReader;
///Field `DEAT` writer - DEAT
pub type DEAT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Receiver timeout interrupt

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOIE {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An USART interrupt is generated when the RTOF bit is set in the ISR register
    Enabled = 1,
}
impl From<RTOIE> for bool {
    #[inline(always)]
    fn from(variant: RTOIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOIE` reader - Receiver timeout interrupt
pub type RTOIE_R = crate::BitReader<RTOIE>;
impl RTOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTOIE {
        match self.bits {
            false => RTOIE::Disabled,
            true => RTOIE::Enabled,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOIE::Disabled
    }
    ///An USART interrupt is generated when the RTOF bit is set in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOIE::Enabled
    }
}
///Field `RTOIE` writer - Receiver timeout interrupt
pub type RTOIE_W<'a, REG> = crate::BitWriter<'a, REG, RTOIE>;
impl<'a, REG> RTOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTOIE::Disabled)
    }
    ///An USART interrupt is generated when the RTOF bit is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTOIE::Enabled)
    }
}
/**End of Block interruptenable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBIE {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: A USART interrupt is generated when the EOBF flag is set in the ISR register
    Enabled = 1,
}
impl From<EOBIE> for bool {
    #[inline(always)]
    fn from(variant: EOBIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBIE` reader - End of Block interruptenable
pub type EOBIE_R = crate::BitReader<EOBIE>;
impl EOBIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOBIE {
        match self.bits {
            false => EOBIE::Disabled,
            true => EOBIE::Enabled,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOBIE::Disabled
    }
    ///A USART interrupt is generated when the EOBF flag is set in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOBIE::Enabled
    }
}
///Field `EOBIE` writer - End of Block interruptenable
pub type EOBIE_W<'a, REG> = crate::BitWriter<'a, REG, EOBIE>;
impl<'a, REG> EOBIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOBIE::Disabled)
    }
    ///A USART interrupt is generated when the EOBF flag is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOBIE::Enabled)
    }
}
/**Word length

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M1 {
    ///0: Use M0 to set the data bits
    M0 = 0,
    ///1: 1 start bit, 7 data bits, n stop bits
    Bit7 = 1,
}
impl From<M1> for bool {
    #[inline(always)]
    fn from(variant: M1) -> Self {
        variant as u8 != 0
    }
}
///Field `M1` reader - Word length
pub type M1_R = crate::BitReader<M1>;
impl M1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> M1 {
        match self.bits {
            false => M1::M0,
            true => M1::Bit7,
        }
    }
    ///Use M0 to set the data bits
    #[inline(always)]
    pub fn is_m0(&self) -> bool {
        *self == M1::M0
    }
    ///1 start bit, 7 data bits, n stop bits
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == M1::Bit7
    }
}
///Field `M1` writer - Word length
pub type M1_W<'a, REG> = crate::BitWriter<'a, REG, M1>;
impl<'a, REG> M1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Use M0 to set the data bits
    #[inline(always)]
    pub fn m0(self) -> &'a mut crate::W<REG> {
        self.variant(M1::M0)
    }
    ///1 start bit, 7 data bits, n stop bits
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(M1::Bit7)
    }
}
/**FIFOEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN {
    ///0: FIFO mode is disabled
    Disabled = 0,
    ///1: FIFO mode is enabled
    Enabled = 1,
}
impl From<FIFOEN> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FIFOEN` reader - FIFOEN
pub type FIFOEN_R = crate::BitReader<FIFOEN>;
impl FIFOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FIFOEN {
        match self.bits {
            false => FIFOEN::Disabled,
            true => FIFOEN::Enabled,
        }
    }
    ///FIFO mode is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIFOEN::Disabled
    }
    ///FIFO mode is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIFOEN::Enabled
    }
}
///Field `FIFOEN` writer - FIFOEN
pub type FIFOEN_W<'a, REG> = crate::BitWriter<'a, REG, FIFOEN>;
impl<'a, REG> FIFOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO mode is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOEN::Disabled)
    }
    ///FIFO mode is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOEN::Enabled)
    }
}
/**TXFEIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFEIE {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when TXFE = 1 in the USART_ISR register
    Enabled = 1,
}
impl From<TXFEIE> for bool {
    #[inline(always)]
    fn from(variant: TXFEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFEIE` reader - TXFEIE
pub type TXFEIE_R = crate::BitReader<TXFEIE>;
impl TXFEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFEIE {
        match self.bits {
            false => TXFEIE::Disabled,
            true => TXFEIE::Enabled,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFEIE::Disabled
    }
    ///USART interrupt generated when TXFE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFEIE::Enabled
    }
}
///Field `TXFEIE` writer - TXFEIE
pub type TXFEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXFEIE>;
impl<'a, REG> TXFEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXFEIE::Disabled)
    }
    ///USART interrupt generated when TXFE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXFEIE::Enabled)
    }
}
/**RXFFIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFFIE {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when RXFF = 1 in the USART_ISR register
    Enabled = 1,
}
impl From<RXFFIE> for bool {
    #[inline(always)]
    fn from(variant: RXFFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFFIE` reader - RXFFIE
pub type RXFFIE_R = crate::BitReader<RXFFIE>;
impl RXFFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFFIE {
        match self.bits {
            false => RXFFIE::Disabled,
            true => RXFFIE::Enabled,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFFIE::Disabled
    }
    ///USART interrupt generated when RXFF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFFIE::Enabled
    }
}
///Field `RXFFIE` writer - RXFFIE
pub type RXFFIE_W<'a, REG> = crate::BitWriter<'a, REG, RXFFIE>;
impl<'a, REG> RXFFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXFFIE::Disabled)
    }
    ///USART interrupt generated when RXFF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXFFIE::Enabled)
    }
}
impl R {
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXFIFO not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFIFO not full interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - DEDT
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - DEAT
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Receiver timeout interrupt
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - End of Block interruptenable
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFOEN
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TXFEIE
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RXFFIE
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("m1", &self.m1())
            .field("eobie", &self.eobie())
            .field("rtoie", &self.rtoie())
            .field("deat", &self.deat())
            .field("dedt", &self.dedt())
            .field("over8", &self.over8())
            .field("cmie", &self.cmie())
            .field("mme", &self.mme())
            .field("m0", &self.m0())
            .field("wake", &self.wake())
            .field("pce", &self.pce())
            .field("ps", &self.ps())
            .field("peie", &self.peie())
            .field("txeie", &self.txeie())
            .field("tcie", &self.tcie())
            .field("rxneie", &self.rxneie())
            .field("idleie", &self.idleie())
            .field("te", &self.te())
            .field("re", &self.re())
            .field("uesm", &self.uesm())
            .field("ue", &self.ue())
            .field("fifoen", &self.fifoen())
            .field("txfeie", &self.txfeie())
            .field("rxffie", &self.rxffie())
            .finish()
    }
}
impl W {
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<CR1rs> {
        UE_W::new(self, 0)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W<CR1rs> {
        UESM_W::new(self, 1)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<CR1rs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<CR1rs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<CR1rs> {
        IDLEIE_W::new(self, 4)
    }
    ///Bit 5 - RXFIFO not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR1rs> {
        RXNEIE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - TXFIFO not full interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<CR1rs> {
        TXEIE_W::new(self, 7)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<CR1rs> {
        PEIE_W::new(self, 8)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<CR1rs> {
        PS_W::new(self, 9)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<CR1rs> {
        PCE_W::new(self, 10)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<CR1rs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W<CR1rs> {
        M0_W::new(self, 12)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W<CR1rs> {
        MME_W::new(self, 13)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W<CR1rs> {
        CMIE_W::new(self, 14)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&mut self) -> OVER8_W<CR1rs> {
        OVER8_W::new(self, 15)
    }
    ///Bits 16:20 - DEDT
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W<CR1rs> {
        DEDT_W::new(self, 16)
    }
    ///Bits 21:25 - DEAT
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W<CR1rs> {
        DEAT_W::new(self, 21)
    }
    ///Bit 26 - Receiver timeout interrupt
    #[inline(always)]
    pub fn rtoie(&mut self) -> RTOIE_W<CR1rs> {
        RTOIE_W::new(self, 26)
    }
    ///Bit 27 - End of Block interruptenable
    #[inline(always)]
    pub fn eobie(&mut self) -> EOBIE_W<CR1rs> {
        EOBIE_W::new(self, 27)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W<CR1rs> {
        M1_W::new(self, 28)
    }
    ///Bit 29 - FIFOEN
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<CR1rs> {
        FIFOEN_W::new(self, 29)
    }
    ///Bit 30 - TXFEIE
    #[inline(always)]
    pub fn txfeie(&mut self) -> TXFEIE_W<CR1rs> {
        TXFEIE_W::new(self, 30)
    }
    ///Bit 31 - RXFFIE
    #[inline(always)]
    pub fn rxffie(&mut self) -> RXFFIE_W<CR1rs> {
        RXFFIE_W::new(self, 31)
    }
}
/**Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#USART1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
