///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL {
    ///0: Falling edge active for inputs or rising edge active for outputs
    FallingEdge = 0,
    ///1: Rising edge active for inputs or falling edge active for outputs
    RisingEdge = 1,
}
impl From<CKPOL> for bool {
    #[inline(always)]
    fn from(variant: CKPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPOL` reader - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.
pub type CKPOL_R = crate::BitReader<CKPOL>;
impl CKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL {
        match self.bits {
            false => CKPOL::FallingEdge,
            true => CKPOL::RisingEdge,
        }
    }
    ///Falling edge active for inputs or rising edge active for outputs
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKPOL::FallingEdge
    }
    ///Rising edge active for inputs or falling edge active for outputs
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKPOL::RisingEdge
    }
}
///Field `CKPOL` writer - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edge active for inputs or rising edge active for outputs
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::FallingEdge)
    }
    ///Rising edge active for inputs or falling edge active for outputs
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::RisingEdge)
    }
}
/**Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEPOL {
    ///0: PSSI_DE active low (0 indicates that data is valid)
    ActiveLow = 0,
    ///1: PSSI_DE active high (1 indicates that data is valid)
    ActiveHigh = 1,
}
impl From<DEPOL> for bool {
    #[inline(always)]
    fn from(variant: DEPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `DEPOL` reader - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.
pub type DEPOL_R = crate::BitReader<DEPOL>;
impl DEPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEPOL {
        match self.bits {
            false => DEPOL::ActiveLow,
            true => DEPOL::ActiveHigh,
        }
    }
    ///PSSI_DE active low (0 indicates that data is valid)
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == DEPOL::ActiveLow
    }
    ///PSSI_DE active high (1 indicates that data is valid)
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == DEPOL::ActiveHigh
    }
}
///Field `DEPOL` writer - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.
pub type DEPOL_W<'a, REG> = crate::BitWriter<'a, REG, DEPOL>;
impl<'a, REG> DEPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PSSI_DE active low (0 indicates that data is valid)
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(DEPOL::ActiveLow)
    }
    ///PSSI_DE active high (1 indicates that data is valid)
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(DEPOL::ActiveHigh)
    }
}
/**Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDYPOL {
    ///0: PSSI_RDY active low (0 indicates that the receiver is ready to receive)
    ActiveLow = 0,
    ///1: PSSI_RDY active high (1 indicates that the receiver is ready to receive)
    ActiveHigh = 1,
}
impl From<RDYPOL> for bool {
    #[inline(always)]
    fn from(variant: RDYPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `RDYPOL` reader - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.
pub type RDYPOL_R = crate::BitReader<RDYPOL>;
impl RDYPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDYPOL {
        match self.bits {
            false => RDYPOL::ActiveLow,
            true => RDYPOL::ActiveHigh,
        }
    }
    ///PSSI_RDY active low (0 indicates that the receiver is ready to receive)
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == RDYPOL::ActiveLow
    }
    ///PSSI_RDY active high (1 indicates that the receiver is ready to receive)
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == RDYPOL::ActiveHigh
    }
}
///Field `RDYPOL` writer - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.
pub type RDYPOL_W<'a, REG> = crate::BitWriter<'a, REG, RDYPOL>;
impl<'a, REG> RDYPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PSSI_RDY active low (0 indicates that the receiver is ready to receive)
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(RDYPOL::ActiveLow)
    }
    ///PSSI_RDY active high (1 indicates that the receiver is ready to receive)
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(RDYPOL::ActiveHigh)
    }
}
/**Extended data mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDM {
    ///0: Interface captures 8-bit data on every parallel data clock
    BitWidth8 = 0,
    ///3: The interface captures 16-bit data on every parallel data clock
    BitWidth16 = 3,
}
impl From<EDM> for u8 {
    #[inline(always)]
    fn from(variant: EDM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDM {
    type Ux = u8;
}
impl crate::IsEnum for EDM {}
///Field `EDM` reader - Extended data mode
pub type EDM_R = crate::FieldReader<EDM>;
impl EDM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EDM> {
        match self.bits {
            0 => Some(EDM::BitWidth8),
            3 => Some(EDM::BitWidth16),
            _ => None,
        }
    }
    ///Interface captures 8-bit data on every parallel data clock
    #[inline(always)]
    pub fn is_bit_width8(&self) -> bool {
        *self == EDM::BitWidth8
    }
    ///The interface captures 16-bit data on every parallel data clock
    #[inline(always)]
    pub fn is_bit_width16(&self) -> bool {
        *self == EDM::BitWidth16
    }
}
///Field `EDM` writer - Extended data mode
pub type EDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EDM>;
impl<'a, REG> EDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Interface captures 8-bit data on every parallel data clock
    #[inline(always)]
    pub fn bit_width8(self) -> &'a mut crate::W<REG> {
        self.variant(EDM::BitWidth8)
    }
    ///The interface captures 16-bit data on every parallel data clock
    #[inline(always)]
    pub fn bit_width16(self) -> &'a mut crate::W<REG> {
        self.variant(EDM::BitWidth16)
    }
}
/**PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE {
    ///0: PSSI disabled
    Disabled = 0,
    ///1: PSSI enabled
    Enabled = 1,
}
impl From<ENABLE> for bool {
    #[inline(always)]
    fn from(variant: ENABLE) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.
pub type ENABLE_R = crate::BitReader<ENABLE>;
impl ENABLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE {
        match self.bits {
            false => ENABLE::Disabled,
            true => ENABLE::Enabled,
        }
    }
    ///PSSI disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE::Disabled
    }
    ///PSSI enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE::Enabled
    }
}
///Field `ENABLE` writer - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PSSI disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Disabled)
    }
    ///PSSI enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Enabled)
    }
}
/**Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DERDYCFG {
    ///0: PSSI_DE and PSSI_RDY both disabled
    Disabled = 0,
    ///1: Only PSSI_RDY enabled
    Rdy = 1,
    ///2: Only PSSI_DE enabled
    De = 2,
    ///3: Both PSSI_RDY and PSSI_DE alternate functions enabled
    RdyDeAlt = 3,
    ///4: Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin
    RdyDe = 4,
    ///5: Only PSSI_RDY function enabled, but mapped to PSSI_DE pin
    RdyRemapped = 5,
    ///6: Only PSSI_DE function enabled, but mapped to PSSI_RDY pin
    DeRemapped = 6,
    ///7: Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin
    RdyDeBidi = 7,
}
impl From<DERDYCFG> for u8 {
    #[inline(always)]
    fn from(variant: DERDYCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DERDYCFG {
    type Ux = u8;
}
impl crate::IsEnum for DERDYCFG {}
///Field `DERDYCFG` reader - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.
pub type DERDYCFG_R = crate::FieldReader<DERDYCFG>;
impl DERDYCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DERDYCFG {
        match self.bits {
            0 => DERDYCFG::Disabled,
            1 => DERDYCFG::Rdy,
            2 => DERDYCFG::De,
            3 => DERDYCFG::RdyDeAlt,
            4 => DERDYCFG::RdyDe,
            5 => DERDYCFG::RdyRemapped,
            6 => DERDYCFG::DeRemapped,
            7 => DERDYCFG::RdyDeBidi,
            _ => unreachable!(),
        }
    }
    ///PSSI_DE and PSSI_RDY both disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DERDYCFG::Disabled
    }
    ///Only PSSI_RDY enabled
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == DERDYCFG::Rdy
    }
    ///Only PSSI_DE enabled
    #[inline(always)]
    pub fn is_de(&self) -> bool {
        *self == DERDYCFG::De
    }
    ///Both PSSI_RDY and PSSI_DE alternate functions enabled
    #[inline(always)]
    pub fn is_rdy_de_alt(&self) -> bool {
        *self == DERDYCFG::RdyDeAlt
    }
    ///Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin
    #[inline(always)]
    pub fn is_rdy_de(&self) -> bool {
        *self == DERDYCFG::RdyDe
    }
    ///Only PSSI_RDY function enabled, but mapped to PSSI_DE pin
    #[inline(always)]
    pub fn is_rdy_remapped(&self) -> bool {
        *self == DERDYCFG::RdyRemapped
    }
    ///Only PSSI_DE function enabled, but mapped to PSSI_RDY pin
    #[inline(always)]
    pub fn is_de_remapped(&self) -> bool {
        *self == DERDYCFG::DeRemapped
    }
    ///Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin
    #[inline(always)]
    pub fn is_rdy_de_bidi(&self) -> bool {
        *self == DERDYCFG::RdyDeBidi
    }
}
///Field `DERDYCFG` writer - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.
pub type DERDYCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DERDYCFG, crate::Safe>;
impl<'a, REG> DERDYCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PSSI_DE and PSSI_RDY both disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::Disabled)
    }
    ///Only PSSI_RDY enabled
    #[inline(always)]
    pub fn rdy(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::Rdy)
    }
    ///Only PSSI_DE enabled
    #[inline(always)]
    pub fn de(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::De)
    }
    ///Both PSSI_RDY and PSSI_DE alternate functions enabled
    #[inline(always)]
    pub fn rdy_de_alt(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::RdyDeAlt)
    }
    ///Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin
    #[inline(always)]
    pub fn rdy_de(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::RdyDe)
    }
    ///Only PSSI_RDY function enabled, but mapped to PSSI_DE pin
    #[inline(always)]
    pub fn rdy_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::RdyRemapped)
    }
    ///Only PSSI_DE function enabled, but mapped to PSSI_RDY pin
    #[inline(always)]
    pub fn de_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::DeRemapped)
    }
    ///Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin
    #[inline(always)]
    pub fn rdy_de_bidi(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::RdyDeBidi)
    }
}
/**DMA enable bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled.
    Disabled = 0,
    ///1: DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA enable bit
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    ///DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    ///DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
///Field `DMAEN` writer - DMA enable bit
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    ///DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
/**Data direction selection bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTEN {
    ///0: Data is input synchronously with PSSI_PDCK
    ReceiveMode = 0,
    ///1: Data is output synchronously with PSSI_PDCK
    TransmitMode = 1,
}
impl From<OUTEN> for bool {
    #[inline(always)]
    fn from(variant: OUTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `OUTEN` reader - Data direction selection bit
pub type OUTEN_R = crate::BitReader<OUTEN>;
impl OUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTEN {
        match self.bits {
            false => OUTEN::ReceiveMode,
            true => OUTEN::TransmitMode,
        }
    }
    ///Data is input synchronously with PSSI_PDCK
    #[inline(always)]
    pub fn is_receive_mode(&self) -> bool {
        *self == OUTEN::ReceiveMode
    }
    ///Data is output synchronously with PSSI_PDCK
    #[inline(always)]
    pub fn is_transmit_mode(&self) -> bool {
        *self == OUTEN::TransmitMode
    }
}
///Field `OUTEN` writer - Data direction selection bit
pub type OUTEN_W<'a, REG> = crate::BitWriter<'a, REG, OUTEN>;
impl<'a, REG> OUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data is input synchronously with PSSI_PDCK
    #[inline(always)]
    pub fn receive_mode(self) -> &'a mut crate::W<REG> {
        self.variant(OUTEN::ReceiveMode)
    }
    ///Data is output synchronously with PSSI_PDCK
    #[inline(always)]
    pub fn transmit_mode(self) -> &'a mut crate::W<REG> {
        self.variant(OUTEN::TransmitMode)
    }
}
impl R {
    ///Bit 5 - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn rdypol(&self) -> RDYPOL_R {
        RDYPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 14 - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 18:20 - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.
    #[inline(always)]
    pub fn derdycfg(&self) -> DERDYCFG_R {
        DERDYCFG_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - DMA enable bit
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Data direction selection bit
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("ckpol", &self.ckpol())
            .field("depol", &self.depol())
            .field("rdypol", &self.rdypol())
            .field("edm", &self.edm())
            .field("enable", &self.enable())
            .field("derdycfg", &self.derdycfg())
            .field("dmaen", &self.dmaen())
            .field("outen", &self.outen())
            .finish()
    }
}
impl W {
    ///Bit 5 - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<CRrs> {
        CKPOL_W::new(self, 5)
    }
    ///Bit 6 - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn depol(&mut self) -> DEPOL_W<CRrs> {
        DEPOL_W::new(self, 6)
    }
    ///Bit 8 - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn rdypol(&mut self) -> RDYPOL_W<CRrs> {
        RDYPOL_W::new(self, 8)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&mut self) -> EDM_W<CRrs> {
        EDM_W::new(self, 10)
    }
    ///Bit 14 - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 14)
    }
    ///Bits 18:20 - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.
    #[inline(always)]
    pub fn derdycfg(&mut self) -> DERDYCFG_W<CRrs> {
        DERDYCFG_W::new(self, 18)
    }
    ///Bit 30 - DMA enable bit
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 30)
    }
    ///Bit 31 - Data direction selection bit
    #[inline(always)]
    pub fn outen(&mut self) -> OUTEN_W<CRrs> {
        OUTEN_W::new(self, 31)
    }
}
/**PSSI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#PSSI:CR)*/
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
///`reset()` method sets CR to value 0x4000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
