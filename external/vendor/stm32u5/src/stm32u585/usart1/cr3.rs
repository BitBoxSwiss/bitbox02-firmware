///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
/**Error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    Enabled = 1,
}
impl From<EIE> for bool {
    #[inline(always)]
    fn from(variant: EIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EIE` reader - Error interrupt enable
pub type EIE_R = crate::BitReader<EIE>;
impl EIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EIE {
        match self.bits {
            false => EIE::Disabled,
            true => EIE::Enabled,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE::Disabled
    }
    ///An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE::Enabled
    }
}
///Field `EIE` writer - Error interrupt enable
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG, EIE>;
impl<'a, REG> EIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Disabled)
    }
    ///An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Enabled)
    }
}
/**Ir mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN {
    ///0: IrDA disabled
    Disabled = 0,
    ///1: IrDA enabled
    Enabled = 1,
}
impl From<IREN> for bool {
    #[inline(always)]
    fn from(variant: IREN) -> Self {
        variant as u8 != 0
    }
}
///Field `IREN` reader - Ir mode enable
pub type IREN_R = crate::BitReader<IREN>;
impl IREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IREN {
        match self.bits {
            false => IREN::Disabled,
            true => IREN::Enabled,
        }
    }
    ///IrDA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN::Disabled
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN::Enabled
    }
}
///Field `IREN` writer - Ir mode enable
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG, IREN>;
impl<'a, REG> IREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IrDA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Disabled)
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Enabled)
    }
}
/**Ir low-power

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRLP {
    ///0: Normal mode
    Normal = 0,
    ///1: Low-power mode
    LowPower = 1,
}
impl From<IRLP> for bool {
    #[inline(always)]
    fn from(variant: IRLP) -> Self {
        variant as u8 != 0
    }
}
///Field `IRLP` reader - Ir low-power
pub type IRLP_R = crate::BitReader<IRLP>;
impl IRLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRLP {
        match self.bits {
            false => IRLP::Normal,
            true => IRLP::LowPower,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLP::Normal
    }
    ///Low-power mode
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == IRLP::LowPower
    }
}
///Field `IRLP` writer - Ir low-power
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG, IRLP>;
impl<'a, REG> IRLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP::Normal)
    }
    ///Low-power mode
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP::LowPower)
    }
}
/**Half-duplex selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL {
    ///0: Half duplex mode is not selected
    NotSelected = 0,
    ///1: Half duplex mode is selected
    Selected = 1,
}
impl From<HDSEL> for bool {
    #[inline(always)]
    fn from(variant: HDSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `HDSEL` reader - Half-duplex selection
pub type HDSEL_R = crate::BitReader<HDSEL>;
impl HDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HDSEL {
        match self.bits {
            false => HDSEL::NotSelected,
            true => HDSEL::Selected,
        }
    }
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == HDSEL::NotSelected
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == HDSEL::Selected
    }
}
///Field `HDSEL` writer - Half-duplex selection
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG, HDSEL>;
impl<'a, REG> HDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::NotSelected)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::Selected)
    }
}
/**Smartcard NACK enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK {
    ///0: NACK transmission in case of parity error is disabled
    Disabled = 0,
    ///1: NACK transmission during parity error is enabled
    Enabled = 1,
}
impl From<NACK> for bool {
    #[inline(always)]
    fn from(variant: NACK) -> Self {
        variant as u8 != 0
    }
}
///Field `NACK` reader - Smartcard NACK enable
pub type NACK_R = crate::BitReader<NACK>;
impl NACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NACK {
        match self.bits {
            false => NACK::Disabled,
            true => NACK::Enabled,
        }
    }
    ///NACK transmission in case of parity error is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACK::Disabled
    }
    ///NACK transmission during parity error is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACK::Enabled
    }
}
///Field `NACK` writer - Smartcard NACK enable
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG, NACK>;
impl<'a, REG> NACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NACK transmission in case of parity error is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NACK::Disabled)
    }
    ///NACK transmission during parity error is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NACK::Enabled)
    }
}
/**Smartcard mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCEN {
    ///0: Smartcard Mode disabled
    Disabled = 0,
    ///1: Smartcard Mode enabled
    Enabled = 1,
}
impl From<SCEN> for bool {
    #[inline(always)]
    fn from(variant: SCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SCEN` reader - Smartcard mode enable
pub type SCEN_R = crate::BitReader<SCEN>;
impl SCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCEN {
        match self.bits {
            false => SCEN::Disabled,
            true => SCEN::Enabled,
        }
    }
    ///Smartcard Mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCEN::Disabled
    }
    ///Smartcard Mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCEN::Enabled
    }
}
///Field `SCEN` writer - Smartcard mode enable
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG, SCEN>;
impl<'a, REG> SCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Smartcard Mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCEN::Disabled)
    }
    ///Smartcard Mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCEN::Enabled)
    }
}
/**DMA enable receiver

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR {
    ///0: DMA mode is disabled for reception
    Disabled = 0,
    ///1: DMA mode is enabled for reception
    Enabled = 1,
}
impl From<DMAR> for bool {
    #[inline(always)]
    fn from(variant: DMAR) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAR` reader - DMA enable receiver
pub type DMAR_R = crate::BitReader<DMAR>;
impl DMAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAR {
        match self.bits {
            false => DMAR::Disabled,
            true => DMAR::Enabled,
        }
    }
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR::Disabled
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR::Enabled
    }
}
///Field `DMAR` writer - DMA enable receiver
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG, DMAR>;
impl<'a, REG> DMAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Disabled)
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Enabled)
    }
}
/**DMA enable transmitter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT {
    ///0: DMA mode is disabled for transmission
    Disabled = 0,
    ///1: DMA mode is enabled for transmission
    Enabled = 1,
}
impl From<DMAT> for bool {
    #[inline(always)]
    fn from(variant: DMAT) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAT` reader - DMA enable transmitter
pub type DMAT_R = crate::BitReader<DMAT>;
impl DMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAT {
        match self.bits {
            false => DMAT::Disabled,
            true => DMAT::Enabled,
        }
    }
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT::Disabled
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT::Enabled
    }
}
///Field `DMAT` writer - DMA enable transmitter
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG, DMAT>;
impl<'a, REG> DMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Disabled)
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Enabled)
    }
}
/**RTS enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSE {
    ///0: RTS hardware flow control disabled
    Disabled = 0,
    ///1: RTS output enabled, data is only requested when there is space in the receive buffer
    Enabled = 1,
}
impl From<RTSE> for bool {
    #[inline(always)]
    fn from(variant: RTSE) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSE` reader - RTS enable
pub type RTSE_R = crate::BitReader<RTSE>;
impl RTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTSE {
        match self.bits {
            false => RTSE::Disabled,
            true => RTSE::Enabled,
        }
    }
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSE::Disabled
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSE::Enabled
    }
}
///Field `RTSE` writer - RTS enable
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG, RTSE>;
impl<'a, REG> RTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::Disabled)
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::Enabled)
    }
}
/**CTS enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE {
    ///0: CTS hardware flow control disabled
    Disabled = 0,
    ///1: CTS mode enabled, data is only transmitted when the CTS input is asserted
    Enabled = 1,
}
impl From<CTSE> for bool {
    #[inline(always)]
    fn from(variant: CTSE) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSE` reader - CTS enable
pub type CTSE_R = crate::BitReader<CTSE>;
impl CTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSE {
        match self.bits {
            false => CTSE::Disabled,
            true => CTSE::Enabled,
        }
    }
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSE::Disabled
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is asserted
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSE::Enabled
    }
}
///Field `CTSE` writer - CTS enable
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG, CTSE>;
impl<'a, REG> CTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::Disabled)
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is asserted
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::Enabled)
    }
}
/**CTS interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An interrupt is generated whenever CTSIF=1 in the ISR register
    Enabled = 1,
}
impl From<CTSIE> for bool {
    #[inline(always)]
    fn from(variant: CTSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIE` reader - CTS interrupt enable
pub type CTSIE_R = crate::BitReader<CTSIE>;
impl CTSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSIE {
        match self.bits {
            false => CTSIE::Disabled,
            true => CTSIE::Enabled,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE::Disabled
    }
    ///An interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE::Enabled
    }
}
///Field `CTSIE` writer - CTS interrupt enable
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG, CTSIE>;
impl<'a, REG> CTSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::Disabled)
    }
    ///An interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::Enabled)
    }
}
/**One sample bit method enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONEBIT {
    ///0: Three sample bit method
    Sample3 = 0,
    ///1: One sample bit method
    Sample1 = 1,
}
impl From<ONEBIT> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT) -> Self {
        variant as u8 != 0
    }
}
///Field `ONEBIT` reader - One sample bit method enable
pub type ONEBIT_R = crate::BitReader<ONEBIT>;
impl ONEBIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ONEBIT {
        match self.bits {
            false => ONEBIT::Sample3,
            true => ONEBIT::Sample1,
        }
    }
    ///Three sample bit method
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        *self == ONEBIT::Sample3
    }
    ///One sample bit method
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        *self == ONEBIT::Sample1
    }
}
///Field `ONEBIT` writer - One sample bit method enable
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG, ONEBIT>;
impl<'a, REG> ONEBIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Three sample bit method
    #[inline(always)]
    pub fn sample3(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT::Sample3)
    }
    ///One sample bit method
    #[inline(always)]
    pub fn sample1(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT::Sample1)
    }
}
/**Overrun Disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRDIS {
    ///0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    Enabled = 0,
    ///1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    Disabled = 1,
}
impl From<OVRDIS> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRDIS` reader - Overrun Disable
pub type OVRDIS_R = crate::BitReader<OVRDIS>;
impl OVRDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRDIS {
        match self.bits {
            false => OVRDIS::Enabled,
            true => OVRDIS::Disabled,
        }
    }
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRDIS::Enabled
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRDIS::Disabled
    }
}
///Field `OVRDIS` writer - Overrun Disable
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG, OVRDIS>;
impl<'a, REG> OVRDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS::Enabled)
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS::Disabled)
    }
}
/**DMA Disable on Reception Error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRE {
    ///0: DMA is not disabled in case of reception error
    NotDisabled = 0,
    ///1: DMA is disabled following a reception error
    Disabled = 1,
}
impl From<DDRE> for bool {
    #[inline(always)]
    fn from(variant: DDRE) -> Self {
        variant as u8 != 0
    }
}
///Field `DDRE` reader - DMA Disable on Reception Error
pub type DDRE_R = crate::BitReader<DDRE>;
impl DDRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DDRE {
        match self.bits {
            false => DDRE::NotDisabled,
            true => DDRE::Disabled,
        }
    }
    ///DMA is not disabled in case of reception error
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == DDRE::NotDisabled
    }
    ///DMA is disabled following a reception error
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRE::Disabled
    }
}
///Field `DDRE` writer - DMA Disable on Reception Error
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG, DDRE>;
impl<'a, REG> DDRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA is not disabled in case of reception error
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE::NotDisabled)
    }
    ///DMA is disabled following a reception error
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE::Disabled)
    }
}
/**Driver enable mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEM {
    ///0: DE function is disabled
    Disabled = 0,
    ///1: The DE signal is output on the RTS pin
    Enabled = 1,
}
impl From<DEM> for bool {
    #[inline(always)]
    fn from(variant: DEM) -> Self {
        variant as u8 != 0
    }
}
///Field `DEM` reader - Driver enable mode
pub type DEM_R = crate::BitReader<DEM>;
impl DEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEM {
        match self.bits {
            false => DEM::Disabled,
            true => DEM::Enabled,
        }
    }
    ///DE function is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEM::Disabled
    }
    ///The DE signal is output on the RTS pin
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEM::Enabled
    }
}
///Field `DEM` writer - Driver enable mode
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG, DEM>;
impl<'a, REG> DEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DE function is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEM::Disabled)
    }
    ///The DE signal is output on the RTS pin
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEM::Enabled)
    }
}
/**Driver enable polarity selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEP {
    ///0: DE signal is active high
    High = 0,
    ///1: DE signal is active low
    Low = 1,
}
impl From<DEP> for bool {
    #[inline(always)]
    fn from(variant: DEP) -> Self {
        variant as u8 != 0
    }
}
///Field `DEP` reader - Driver enable polarity selection
pub type DEP_R = crate::BitReader<DEP>;
impl DEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEP {
        match self.bits {
            false => DEP::High,
            true => DEP::Low,
        }
    }
    ///DE signal is active high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEP::High
    }
    ///DE signal is active low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEP::Low
    }
}
///Field `DEP` writer - Driver enable polarity selection
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG, DEP>;
impl<'a, REG> DEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DE signal is active high
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::High)
    }
    ///DE signal is active low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::Low)
    }
}
///Field `SCARCNT` reader - Smartcard auto-retry count
pub type SCARCNT_R = crate::FieldReader;
///Field `SCARCNT` writer - Smartcard auto-retry count
pub type SCARCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**TXFTIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFTIE {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when Transmit FIFO reaches the threshold programmed in TXFTCFG
    Enabled = 1,
}
impl From<TXFTIE> for bool {
    #[inline(always)]
    fn from(variant: TXFTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFTIE` reader - TXFTIE
pub type TXFTIE_R = crate::BitReader<TXFTIE>;
impl TXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFTIE {
        match self.bits {
            false => TXFTIE::Disabled,
            true => TXFTIE::Enabled,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFTIE::Disabled
    }
    ///USART interrupt generated when Transmit FIFO reaches the threshold programmed in TXFTCFG
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFTIE::Enabled
    }
}
///Field `TXFTIE` writer - TXFTIE
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG, TXFTIE>;
impl<'a, REG> TXFTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTIE::Disabled)
    }
    ///USART interrupt generated when Transmit FIFO reaches the threshold programmed in TXFTCFG
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTIE::Enabled)
    }
}
/**TCBGTIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGTIE {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    Enabled = 1,
}
impl From<TCBGTIE> for bool {
    #[inline(always)]
    fn from(variant: TCBGTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCBGTIE` reader - TCBGTIE
pub type TCBGTIE_R = crate::BitReader<TCBGTIE>;
impl TCBGTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCBGTIE {
        match self.bits {
            false => TCBGTIE::Disabled,
            true => TCBGTIE::Enabled,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCBGTIE::Disabled
    }
    ///USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCBGTIE::Enabled
    }
}
///Field `TCBGTIE` writer - TCBGTIE
pub type TCBGTIE_W<'a, REG> = crate::BitWriter<'a, REG, TCBGTIE>;
impl<'a, REG> TCBGTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCBGTIE::Disabled)
    }
    ///USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCBGTIE::Enabled)
    }
}
/**RXFTCFG

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFTCFG {
    ///0: RXFIFO reaches 1/8 of its depth
    Depth1_8 = 0,
    ///1: RXFIFO reaches 1/4 of its depth
    Depth1_4 = 1,
    ///2: RXFIFO reaches 1/2 of its depth
    Depth1_2 = 2,
    ///3: RXFIFO reaches 3/4 of its depth
    Depth3_4 = 3,
    ///4: RXFIFO reaches 7/8 of its depth
    Depth7_8 = 4,
    ///5: RXFIFO becomes full
    Full = 5,
}
impl From<RXFTCFG> for u8 {
    #[inline(always)]
    fn from(variant: RXFTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXFTCFG {
    type Ux = u8;
}
impl crate::IsEnum for RXFTCFG {}
///Field `RXFTCFG` reader - RXFTCFG
pub type RXFTCFG_R = crate::FieldReader<RXFTCFG>;
impl RXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXFTCFG> {
        match self.bits {
            0 => Some(RXFTCFG::Depth1_8),
            1 => Some(RXFTCFG::Depth1_4),
            2 => Some(RXFTCFG::Depth1_2),
            3 => Some(RXFTCFG::Depth3_4),
            4 => Some(RXFTCFG::Depth7_8),
            5 => Some(RXFTCFG::Full),
            _ => None,
        }
    }
    ///RXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn is_depth_1_8(&self) -> bool {
        *self == RXFTCFG::Depth1_8
    }
    ///RXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn is_depth_1_4(&self) -> bool {
        *self == RXFTCFG::Depth1_4
    }
    ///RXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn is_depth_1_2(&self) -> bool {
        *self == RXFTCFG::Depth1_2
    }
    ///RXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn is_depth_3_4(&self) -> bool {
        *self == RXFTCFG::Depth3_4
    }
    ///RXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn is_depth_7_8(&self) -> bool {
        *self == RXFTCFG::Depth7_8
    }
    ///RXFIFO becomes full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFTCFG::Full
    }
}
///Field `RXFTCFG` writer - RXFTCFG
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RXFTCFG>;
impl<'a, REG> RXFTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn depth_1_8(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth1_8)
    }
    ///RXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn depth_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth1_4)
    }
    ///RXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn depth_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth1_2)
    }
    ///RXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn depth_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth3_4)
    }
    ///RXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn depth_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth7_8)
    }
    ///RXFIFO becomes full
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Full)
    }
}
/**RXFTIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFTIE {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    Enabled = 1,
}
impl From<RXFTIE> for bool {
    #[inline(always)]
    fn from(variant: RXFTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFTIE` reader - RXFTIE
pub type RXFTIE_R = crate::BitReader<RXFTIE>;
impl RXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFTIE {
        match self.bits {
            false => RXFTIE::Disabled,
            true => RXFTIE::Enabled,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFTIE::Disabled
    }
    ///USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFTIE::Enabled
    }
}
///Field `RXFTIE` writer - RXFTIE
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG, RXFTIE>;
impl<'a, REG> RXFTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTIE::Disabled)
    }
    ///USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTIE::Enabled)
    }
}
/**TXFTCFG

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFTCFG {
    ///0: TXFIFO reaches 1/8 of its depth
    Depth1_8 = 0,
    ///1: TXFIFO reaches 1/4 of its depth
    Depth1_4 = 1,
    ///2: TXFIFO reaches 1/2 of its depth
    Depth1_2 = 2,
    ///3: TXFIFO reaches 3/4 of its depth
    Depth3_4 = 3,
    ///4: TXFIFO reaches 7/8 of its depth
    Depth7_8 = 4,
    ///5: TXFIFO becomes empty
    Empty = 5,
}
impl From<TXFTCFG> for u8 {
    #[inline(always)]
    fn from(variant: TXFTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXFTCFG {
    type Ux = u8;
}
impl crate::IsEnum for TXFTCFG {}
///Field `TXFTCFG` reader - TXFTCFG
pub type TXFTCFG_R = crate::FieldReader<TXFTCFG>;
impl TXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXFTCFG> {
        match self.bits {
            0 => Some(TXFTCFG::Depth1_8),
            1 => Some(TXFTCFG::Depth1_4),
            2 => Some(TXFTCFG::Depth1_2),
            3 => Some(TXFTCFG::Depth3_4),
            4 => Some(TXFTCFG::Depth7_8),
            5 => Some(TXFTCFG::Empty),
            _ => None,
        }
    }
    ///TXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn is_depth_1_8(&self) -> bool {
        *self == TXFTCFG::Depth1_8
    }
    ///TXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn is_depth_1_4(&self) -> bool {
        *self == TXFTCFG::Depth1_4
    }
    ///TXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn is_depth_1_2(&self) -> bool {
        *self == TXFTCFG::Depth1_2
    }
    ///TXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn is_depth_3_4(&self) -> bool {
        *self == TXFTCFG::Depth3_4
    }
    ///TXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn is_depth_7_8(&self) -> bool {
        *self == TXFTCFG::Depth7_8
    }
    ///TXFIFO becomes empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFTCFG::Empty
    }
}
///Field `TXFTCFG` writer - TXFTCFG
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TXFTCFG>;
impl<'a, REG> TXFTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn depth_1_8(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth1_8)
    }
    ///TXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn depth_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth1_4)
    }
    ///TXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn depth_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth1_2)
    }
    ///TXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn depth_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth3_4)
    }
    ///TXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn depth_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth7_8)
    }
    ///TXFIFO becomes empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Empty)
    }
}
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 23 - TXFTIE
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - TCBGTIE
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - RXFTCFG
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFTIE
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFTCFG
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("scarcnt", &self.scarcnt())
            .field("dep", &self.dep())
            .field("dem", &self.dem())
            .field("ddre", &self.ddre())
            .field("ovrdis", &self.ovrdis())
            .field("onebit", &self.onebit())
            .field("ctsie", &self.ctsie())
            .field("ctse", &self.ctse())
            .field("rtse", &self.rtse())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("scen", &self.scen())
            .field("nack", &self.nack())
            .field("hdsel", &self.hdsel())
            .field("irlp", &self.irlp())
            .field("iren", &self.iren())
            .field("eie", &self.eie())
            .field("txftie", &self.txftie())
            .field("tcbgtie", &self.tcbgtie())
            .field("rxftcfg", &self.rxftcfg())
            .field("rxftie", &self.rxftie())
            .field("txftcfg", &self.txftcfg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<CR3rs> {
        IREN_W::new(self, 1)
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W<CR3rs> {
        IRLP_W::new(self, 2)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<CR3rs> {
        NACK_W::new(self, 4)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W<CR3rs> {
        SCEN_W::new(self, 5)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<CR3rs> {
        RTSE_W::new(self, 8)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<CR3rs> {
        CTSE_W::new(self, 9)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<CR3rs> {
        CTSIE_W::new(self, 10)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W<CR3rs> {
        ONEBIT_W::new(self, 11)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<CR3rs> {
        DDRE_W::new(self, 13)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<CR3rs> {
        DEM_W::new(self, 14)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<CR3rs> {
        DEP_W::new(self, 15)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&mut self) -> SCARCNT_W<CR3rs> {
        SCARCNT_W::new(self, 17)
    }
    ///Bit 23 - TXFTIE
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<CR3rs> {
        TXFTIE_W::new(self, 23)
    }
    ///Bit 24 - TCBGTIE
    #[inline(always)]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W<CR3rs> {
        TCBGTIE_W::new(self, 24)
    }
    ///Bits 25:27 - RXFTCFG
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<CR3rs> {
        RXFTCFG_W::new(self, 25)
    }
    ///Bit 28 - RXFTIE
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<CR3rs> {
        RXFTIE_W::new(self, 28)
    }
    ///Bits 29:31 - TXFTCFG
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<CR3rs> {
        TXFTCFG_W::new(self, 29)
    }
}
/**Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#USART1:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
