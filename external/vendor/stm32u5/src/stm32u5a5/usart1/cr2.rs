///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**SLVEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVEN {
    ///0: Slave mode disabled
    Disabled = 0,
    ///1: Slave mode enabled
    Enabled = 1,
}
impl From<SLVEN> for bool {
    #[inline(always)]
    fn from(variant: SLVEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SLVEN` reader - SLVEN
pub type SLVEN_R = crate::BitReader<SLVEN>;
impl SLVEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLVEN {
        match self.bits {
            false => SLVEN::Disabled,
            true => SLVEN::Enabled,
        }
    }
    ///Slave mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVEN::Disabled
    }
    ///Slave mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVEN::Enabled
    }
}
///Field `SLVEN` writer - SLVEN
pub type SLVEN_W<'a, REG> = crate::BitWriter<'a, REG, SLVEN>;
impl<'a, REG> SLVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLVEN::Disabled)
    }
    ///Slave mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLVEN::Enabled)
    }
}
/**DIS_NSS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_NSS {
    ///0: SPI slave selection depends on NSS input pin
    Disabled = 0,
    ///1: SPI slave is always selected and NSS input pin is ignored
    Enabled = 1,
}
impl From<DIS_NSS> for bool {
    #[inline(always)]
    fn from(variant: DIS_NSS) -> Self {
        variant as u8 != 0
    }
}
///Field `DIS_NSS` reader - DIS_NSS
pub type DIS_NSS_R = crate::BitReader<DIS_NSS>;
impl DIS_NSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIS_NSS {
        match self.bits {
            false => DIS_NSS::Disabled,
            true => DIS_NSS::Enabled,
        }
    }
    ///SPI slave selection depends on NSS input pin
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIS_NSS::Disabled
    }
    ///SPI slave is always selected and NSS input pin is ignored
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIS_NSS::Enabled
    }
}
///Field `DIS_NSS` writer - DIS_NSS
pub type DIS_NSS_W<'a, REG> = crate::BitWriter<'a, REG, DIS_NSS>;
impl<'a, REG> DIS_NSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI slave selection depends on NSS input pin
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIS_NSS::Disabled)
    }
    ///SPI slave is always selected and NSS input pin is ignored
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIS_NSS::Enabled)
    }
}
/**7-bit Address Detection/4-bit Address Detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDM7 {
    ///0: 4-bit address detection
    Bit4 = 0,
    ///1: 7-bit address detection
    Bit7 = 1,
}
impl From<ADDM7> for bool {
    #[inline(always)]
    fn from(variant: ADDM7) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_R = crate::BitReader<ADDM7>;
impl ADDM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDM7 {
        match self.bits {
            false => ADDM7::Bit4,
            true => ADDM7::Bit7,
        }
    }
    ///4-bit address detection
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM7::Bit4
    }
    ///7-bit address detection
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADDM7::Bit7
    }
}
///Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG, ADDM7>;
impl<'a, REG> ADDM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///4-bit address detection
    #[inline(always)]
    pub fn bit4(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7::Bit4)
    }
    ///7-bit address detection
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7::Bit7)
    }
}
/**LIN break detection length

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDL {
    ///0: 10-bit break detection
    Bit10 = 0,
    ///1: 11-bit break detection
    Bit11 = 1,
}
impl From<LBDL> for bool {
    #[inline(always)]
    fn from(variant: LBDL) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDL` reader - LIN break detection length
pub type LBDL_R = crate::BitReader<LBDL>;
impl LBDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDL {
        match self.bits {
            false => LBDL::Bit10,
            true => LBDL::Bit11,
        }
    }
    ///10-bit break detection
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == LBDL::Bit10
    }
    ///11-bit break detection
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == LBDL::Bit11
    }
}
///Field `LBDL` writer - LIN break detection length
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG, LBDL>;
impl<'a, REG> LBDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///10-bit break detection
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL::Bit10)
    }
    ///11-bit break detection
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL::Bit11)
    }
}
/**LIN break detection interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDIE {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An interrupt is generated whenever LBDF=1 in the ISR register
    Enabled = 1,
}
impl From<LBDIE> for bool {
    #[inline(always)]
    fn from(variant: LBDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDIE` reader - LIN break detection interrupt enable
pub type LBDIE_R = crate::BitReader<LBDIE>;
impl LBDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDIE {
        match self.bits {
            false => LBDIE::Disabled,
            true => LBDIE::Enabled,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE::Disabled
    }
    ///An interrupt is generated whenever LBDF=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE::Enabled
    }
}
///Field `LBDIE` writer - LIN break detection interrupt enable
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG, LBDIE>;
impl<'a, REG> LBDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE::Disabled)
    }
    ///An interrupt is generated whenever LBDF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE::Enabled)
    }
}
/**Last bit clock pulse

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCL {
    ///0: The clock pulse of the last data bit is not output to the CK pin
    NotOutput = 0,
    ///1: The clock pulse of the last data bit is output to the CK pin
    Output = 1,
}
impl From<LBCL> for bool {
    #[inline(always)]
    fn from(variant: LBCL) -> Self {
        variant as u8 != 0
    }
}
///Field `LBCL` reader - Last bit clock pulse
pub type LBCL_R = crate::BitReader<LBCL>;
impl LBCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBCL {
        match self.bits {
            false => LBCL::NotOutput,
            true => LBCL::Output,
        }
    }
    ///The clock pulse of the last data bit is not output to the CK pin
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == LBCL::NotOutput
    }
    ///The clock pulse of the last data bit is output to the CK pin
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == LBCL::Output
    }
}
///Field `LBCL` writer - Last bit clock pulse
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG, LBCL>;
impl<'a, REG> LBCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The clock pulse of the last data bit is not output to the CK pin
    #[inline(always)]
    pub fn not_output(self) -> &'a mut crate::W<REG> {
        self.variant(LBCL::NotOutput)
    }
    ///The clock pulse of the last data bit is output to the CK pin
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(LBCL::Output)
    }
}
/**Clock phase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA {
    ///0: The first clock transition is the first data capture edge
    First = 0,
    ///1: The second clock transition is the first data capture edge
    Second = 1,
}
impl From<CPHA> for bool {
    #[inline(always)]
    fn from(variant: CPHA) -> Self {
        variant as u8 != 0
    }
}
///Field `CPHA` reader - Clock phase
pub type CPHA_R = crate::BitReader<CPHA>;
impl CPHA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPHA {
        match self.bits {
            false => CPHA::First,
            true => CPHA::Second,
        }
    }
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPHA::First
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPHA::Second
    }
}
///Field `CPHA` writer - Clock phase
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::First)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::Second)
    }
}
/**Clock polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL {
    ///0: Steady low value on CK pin outside transmission window
    Low = 0,
    ///1: Steady high value on CK pin outside transmission window
    High = 1,
}
impl From<CPOL> for bool {
    #[inline(always)]
    fn from(variant: CPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `CPOL` reader - Clock polarity
pub type CPOL_R = crate::BitReader<CPOL>;
impl CPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPOL {
        match self.bits {
            false => CPOL::Low,
            true => CPOL::High,
        }
    }
    ///Steady low value on CK pin outside transmission window
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL::Low
    }
    ///Steady high value on CK pin outside transmission window
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL::High
    }
}
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Steady low value on CK pin outside transmission window
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::Low)
    }
    ///Steady high value on CK pin outside transmission window
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::High)
    }
}
/**Clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN {
    ///0: CK pin disabled
    Disabled = 0,
    ///1: CK pin enabled
    Enabled = 1,
}
impl From<CLKEN> for bool {
    #[inline(always)]
    fn from(variant: CLKEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKEN` reader - Clock enable
pub type CLKEN_R = crate::BitReader<CLKEN>;
impl CLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN {
        match self.bits {
            false => CLKEN::Disabled,
            true => CLKEN::Enabled,
        }
    }
    ///CK pin disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN::Disabled
    }
    ///CK pin enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN::Enabled
    }
}
///Field `CLKEN` writer - Clock enable
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CK pin disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Disabled)
    }
    ///CK pin enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Enabled)
    }
}
/**STOP bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP {
    ///0: 1 stop bit
    Stop1 = 0,
    ///1: 0.5 stop bit
    Stop0p5 = 1,
    ///2: 2 stop bit
    Stop2 = 2,
    ///3: 1.5 stop bit
    Stop1p5 = 3,
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP {
    type Ux = u8;
}
impl crate::IsEnum for STOP {}
///Field `STOP` reader - STOP bits
pub type STOP_R = crate::FieldReader<STOP>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOP {
        match self.bits {
            0 => STOP::Stop1,
            1 => STOP::Stop0p5,
            2 => STOP::Stop2,
            3 => STOP::Stop1p5,
            _ => unreachable!(),
        }
    }
    ///1 stop bit
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP::Stop1
    }
    ///0.5 stop bit
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOP::Stop0p5
    }
    ///2 stop bit
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP::Stop2
    }
    ///1.5 stop bit
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOP::Stop1p5
    }
}
///Field `STOP` writer - STOP bits
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOP, crate::Safe>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 stop bit
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1)
    }
    ///0.5 stop bit
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop0p5)
    }
    ///2 stop bit
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop2)
    }
    ///1.5 stop bit
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1p5)
    }
}
/**LIN mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEN {
    ///0: LIN mode disabled
    Disabled = 0,
    ///1: LIN mode enabled
    Enabled = 1,
}
impl From<LINEN> for bool {
    #[inline(always)]
    fn from(variant: LINEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LINEN` reader - LIN mode enable
pub type LINEN_R = crate::BitReader<LINEN>;
impl LINEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LINEN {
        match self.bits {
            false => LINEN::Disabled,
            true => LINEN::Enabled,
        }
    }
    ///LIN mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINEN::Disabled
    }
    ///LIN mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINEN::Enabled
    }
}
///Field `LINEN` writer - LIN mode enable
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG, LINEN>;
impl<'a, REG> LINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LIN mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN::Disabled)
    }
    ///LIN mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN::Enabled)
    }
}
/**Swap TX/RX pins

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP {
    ///0: TX/RX pins are used as defined in standard pinout
    Standard = 0,
    ///1: The TX and RX pins functions are swapped
    Swapped = 1,
}
impl From<SWAP> for bool {
    #[inline(always)]
    fn from(variant: SWAP) -> Self {
        variant as u8 != 0
    }
}
///Field `SWAP` reader - Swap TX/RX pins
pub type SWAP_R = crate::BitReader<SWAP>;
impl SWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWAP {
        match self.bits {
            false => SWAP::Standard,
            true => SWAP::Swapped,
        }
    }
    ///TX/RX pins are used as defined in standard pinout
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SWAP::Standard
    }
    ///The TX and RX pins functions are swapped
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWAP::Swapped
    }
}
///Field `SWAP` writer - Swap TX/RX pins
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG, SWAP>;
impl<'a, REG> SWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX/RX pins are used as defined in standard pinout
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP::Standard)
    }
    ///The TX and RX pins functions are swapped
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP::Swapped)
    }
}
/**RX pin active level inversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV {
    ///0: RX pin signal works using the standard logic levels
    Standard = 0,
    ///1: RX pin signal values are inverted
    Inverted = 1,
}
impl From<RXINV> for bool {
    #[inline(always)]
    fn from(variant: RXINV) -> Self {
        variant as u8 != 0
    }
}
///Field `RXINV` reader - RX pin active level inversion
pub type RXINV_R = crate::BitReader<RXINV>;
impl RXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXINV {
        match self.bits {
            false => RXINV::Standard,
            true => RXINV::Inverted,
        }
    }
    ///RX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXINV::Standard
    }
    ///RX pin signal values are inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXINV::Inverted
    }
}
///Field `RXINV` writer - RX pin active level inversion
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG, RXINV>;
impl<'a, REG> RXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV::Standard)
    }
    ///RX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV::Inverted)
    }
}
/**TX pin active level inversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV {
    ///0: TX pin signal works using the standard logic levels
    Standard = 0,
    ///1: TX pin signal values are inverted
    Inverted = 1,
}
impl From<TXINV> for bool {
    #[inline(always)]
    fn from(variant: TXINV) -> Self {
        variant as u8 != 0
    }
}
///Field `TXINV` reader - TX pin active level inversion
pub type TXINV_R = crate::BitReader<TXINV>;
impl TXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXINV {
        match self.bits {
            false => TXINV::Standard,
            true => TXINV::Inverted,
        }
    }
    ///TX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXINV::Standard
    }
    ///TX pin signal values are inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXINV::Inverted
    }
}
///Field `TXINV` writer - TX pin active level inversion
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG, TXINV>;
impl<'a, REG> TXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV::Standard)
    }
    ///TX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV::Inverted)
    }
}
/**Binary data inversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAINV {
    ///0: Logical data from the data register are send/received in positive/direct logic
    Positive = 0,
    ///1: Logical data from the data register are send/received in negative/inverse logic
    Negative = 1,
}
impl From<DATAINV> for bool {
    #[inline(always)]
    fn from(variant: DATAINV) -> Self {
        variant as u8 != 0
    }
}
///Field `DATAINV` reader - Binary data inversion
pub type DATAINV_R = crate::BitReader<DATAINV>;
impl DATAINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DATAINV {
        match self.bits {
            false => DATAINV::Positive,
            true => DATAINV::Negative,
        }
    }
    ///Logical data from the data register are send/received in positive/direct logic
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DATAINV::Positive
    }
    ///Logical data from the data register are send/received in negative/inverse logic
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DATAINV::Negative
    }
}
///Field `DATAINV` writer - Binary data inversion
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG, DATAINV>;
impl<'a, REG> DATAINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Logical data from the data register are send/received in positive/direct logic
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV::Positive)
    }
    ///Logical data from the data register are send/received in negative/inverse logic
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV::Negative)
    }
}
/**Most significant bit first

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBFIRST {
    ///0: data is transmitted/received with data bit 0 first, following the start bit
    Lsb = 0,
    ///1: data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    Msb = 1,
}
impl From<MSBFIRST> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `MSBFIRST` reader - Most significant bit first
pub type MSBFIRST_R = crate::BitReader<MSBFIRST>;
impl MSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSBFIRST {
        match self.bits {
            false => MSBFIRST::Lsb,
            true => MSBFIRST::Msb,
        }
    }
    ///data is transmitted/received with data bit 0 first, following the start bit
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRST::Lsb
    }
    ///data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRST::Msb
    }
}
///Field `MSBFIRST` writer - Most significant bit first
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, MSBFIRST>;
impl<'a, REG> MSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///data is transmitted/received with data bit 0 first, following the start bit
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST::Lsb)
    }
    ///data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST::Msb)
    }
}
/**Auto baud rate enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABREN {
    ///0: Auto baud rate detection is disabled
    Disabled = 0,
    ///1: Auto baud rate detection is enabled
    Enabled = 1,
}
impl From<ABREN> for bool {
    #[inline(always)]
    fn from(variant: ABREN) -> Self {
        variant as u8 != 0
    }
}
///Field `ABREN` reader - Auto baud rate enable
pub type ABREN_R = crate::BitReader<ABREN>;
impl ABREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABREN {
        match self.bits {
            false => ABREN::Disabled,
            true => ABREN::Enabled,
        }
    }
    ///Auto baud rate detection is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABREN::Disabled
    }
    ///Auto baud rate detection is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABREN::Enabled
    }
}
///Field `ABREN` writer - Auto baud rate enable
pub type ABREN_W<'a, REG> = crate::BitWriter<'a, REG, ABREN>;
impl<'a, REG> ABREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto baud rate detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABREN::Disabled)
    }
    ///Auto baud rate detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABREN::Enabled)
    }
}
/**Auto baud rate mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABRMOD {
    ///0: Measurement of the start bit is used to detect the baud rate
    Start = 0,
    ///1: Falling edge to falling edge measurement
    Edge = 1,
    ///2: 0x7F frame detection
    Frame7f = 2,
    ///3: 0x55 frame detection
    Frame55 = 3,
}
impl From<ABRMOD> for u8 {
    #[inline(always)]
    fn from(variant: ABRMOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ABRMOD {
    type Ux = u8;
}
impl crate::IsEnum for ABRMOD {}
///Field `ABRMOD` reader - Auto baud rate mode
pub type ABRMOD_R = crate::FieldReader<ABRMOD>;
impl ABRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABRMOD {
        match self.bits {
            0 => ABRMOD::Start,
            1 => ABRMOD::Edge,
            2 => ABRMOD::Frame7f,
            3 => ABRMOD::Frame55,
            _ => unreachable!(),
        }
    }
    ///Measurement of the start bit is used to detect the baud rate
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ABRMOD::Start
    }
    ///Falling edge to falling edge measurement
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ABRMOD::Edge
    }
    ///0x7F frame detection
    #[inline(always)]
    pub fn is_frame7f(&self) -> bool {
        *self == ABRMOD::Frame7f
    }
    ///0x55 frame detection
    #[inline(always)]
    pub fn is_frame55(&self) -> bool {
        *self == ABRMOD::Frame55
    }
}
///Field `ABRMOD` writer - Auto baud rate mode
pub type ABRMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ABRMOD, crate::Safe>;
impl<'a, REG> ABRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Measurement of the start bit is used to detect the baud rate
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD::Start)
    }
    ///Falling edge to falling edge measurement
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD::Edge)
    }
    ///0x7F frame detection
    #[inline(always)]
    pub fn frame7f(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD::Frame7f)
    }
    ///0x55 frame detection
    #[inline(always)]
    pub fn frame55(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD::Frame55)
    }
}
/**Receiver timeout enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOEN {
    ///0: Receiver timeout feature disabled
    Disabled = 0,
    ///1: Receiver timeout feature enabled
    Enabled = 1,
}
impl From<RTOEN> for bool {
    #[inline(always)]
    fn from(variant: RTOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOEN` reader - Receiver timeout enable
pub type RTOEN_R = crate::BitReader<RTOEN>;
impl RTOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTOEN {
        match self.bits {
            false => RTOEN::Disabled,
            true => RTOEN::Enabled,
        }
    }
    ///Receiver timeout feature disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOEN::Disabled
    }
    ///Receiver timeout feature enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOEN::Enabled
    }
}
///Field `RTOEN` writer - Receiver timeout enable
pub type RTOEN_W<'a, REG> = crate::BitWriter<'a, REG, RTOEN>;
impl<'a, REG> RTOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiver timeout feature disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTOEN::Disabled)
    }
    ///Receiver timeout feature enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTOEN::Enabled)
    }
}
///Field `ADD` reader - Address of the USART node
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - Address of the USART node
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bit 0 - SLVEN
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - DIS_NSS
    #[inline(always)]
    pub fn dis_nss(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("add", &self.add())
            .field("rtoen", &self.rtoen())
            .field("abrmod", &self.abrmod())
            .field("abren", &self.abren())
            .field("msbfirst", &self.msbfirst())
            .field("datainv", &self.datainv())
            .field("txinv", &self.txinv())
            .field("rxinv", &self.rxinv())
            .field("swap", &self.swap())
            .field("linen", &self.linen())
            .field("stop", &self.stop())
            .field("clken", &self.clken())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .field("lbcl", &self.lbcl())
            .field("lbdie", &self.lbdie())
            .field("lbdl", &self.lbdl())
            .field("addm7", &self.addm7())
            .field("slven", &self.slven())
            .field("dis_nss", &self.dis_nss())
            .finish()
    }
}
impl W {
    ///Bit 0 - SLVEN
    #[inline(always)]
    pub fn slven(&mut self) -> SLVEN_W<CR2rs> {
        SLVEN_W::new(self, 0)
    }
    ///Bit 3 - DIS_NSS
    #[inline(always)]
    pub fn dis_nss(&mut self) -> DIS_NSS_W<CR2rs> {
        DIS_NSS_W::new(self, 3)
    }
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W<CR2rs> {
        ADDM7_W::new(self, 4)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W<CR2rs> {
        LBDL_W::new(self, 5)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W<CR2rs> {
        LBDIE_W::new(self, 6)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&mut self) -> LBCL_W<CR2rs> {
        LBCL_W::new(self, 8)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<CR2rs> {
        CPHA_W::new(self, 9)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<CR2rs> {
        CPOL_W::new(self, 10)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<CR2rs> {
        CLKEN_W::new(self, 11)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 12)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W<CR2rs> {
        LINEN_W::new(self, 14)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W<CR2rs> {
        SWAP_W::new(self, 15)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W<CR2rs> {
        RXINV_W::new(self, 16)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W<CR2rs> {
        TXINV_W::new(self, 17)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&mut self) -> DATAINV_W<CR2rs> {
        DATAINV_W::new(self, 18)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W<CR2rs> {
        ABREN_W::new(self, 20)
    }
    ///Bits 21:22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod(&mut self) -> ABRMOD_W<CR2rs> {
        ABRMOD_W::new(self, 21)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&mut self) -> RTOEN_W<CR2rs> {
        RTOEN_W::new(self, 23)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<CR2rs> {
        ADD_W::new(self, 24)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#USART1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
