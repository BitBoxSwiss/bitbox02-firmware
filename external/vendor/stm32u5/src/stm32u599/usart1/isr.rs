///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**PE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    ///0: No parity error
    NoError = 0,
    ///1: Parity error
    Error = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - PE
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::NoError,
            true => PE::Error,
        }
    }
    ///No parity error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PE::NoError
    }
    ///Parity error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PE::Error
    }
}
/**FE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE {
    ///0: No Framing error is detected
    NoError = 0,
    ///1: Framing error or break character is detected
    Error = 1,
}
impl From<FE> for bool {
    #[inline(always)]
    fn from(variant: FE) -> Self {
        variant as u8 != 0
    }
}
///Field `FE` reader - FE
pub type FE_R = crate::BitReader<FE>;
impl FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FE {
        match self.bits {
            false => FE::NoError,
            true => FE::Error,
        }
    }
    ///No Framing error is detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FE::NoError
    }
    ///Framing error or break character is detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FE::Error
    }
}
/**NE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NE {
    ///0: No noise is detected
    NoNoise = 0,
    ///1: Noise is detected
    Noise = 1,
}
impl From<NE> for bool {
    #[inline(always)]
    fn from(variant: NE) -> Self {
        variant as u8 != 0
    }
}
///Field `NE` reader - NE
pub type NE_R = crate::BitReader<NE>;
impl NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NE {
        match self.bits {
            false => NE::NoNoise,
            true => NE::Noise,
        }
    }
    ///No noise is detected
    #[inline(always)]
    pub fn is_no_noise(&self) -> bool {
        *self == NE::NoNoise
    }
    ///Noise is detected
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == NE::Noise
    }
}
/**ORE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORE {
    ///0: No Overrun error
    NoOverrun = 0,
    ///1: Overrun error is detected
    Overrun = 1,
}
impl From<ORE> for bool {
    #[inline(always)]
    fn from(variant: ORE) -> Self {
        variant as u8 != 0
    }
}
///Field `ORE` reader - ORE
pub type ORE_R = crate::BitReader<ORE>;
impl ORE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORE {
        match self.bits {
            false => ORE::NoOverrun,
            true => ORE::Overrun,
        }
    }
    ///No Overrun error
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == ORE::NoOverrun
    }
    ///Overrun error is detected
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == ORE::Overrun
    }
}
/**IDLE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE {
    ///0: No Idle Line is detected
    NoIdle = 0,
    ///1: Idle Line is detected
    Idle = 1,
}
impl From<IDLE> for bool {
    #[inline(always)]
    fn from(variant: IDLE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLE` reader - IDLE
pub type IDLE_R = crate::BitReader<IDLE>;
impl IDLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLE {
        match self.bits {
            false => IDLE::NoIdle,
            true => IDLE::Idle,
        }
    }
    ///No Idle Line is detected
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == IDLE::NoIdle
    }
    ///Idle Line is detected
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLE::Idle
    }
}
/**RXFNE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFNE {
    ///0: Data is not received
    NoData = 0,
    ///1: Received data is ready to be read
    DataReady = 1,
}
impl From<RXFNE> for bool {
    #[inline(always)]
    fn from(variant: RXFNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFNE` reader - RXFNE
pub type RXFNE_R = crate::BitReader<RXFNE>;
impl RXFNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFNE {
        match self.bits {
            false => RXFNE::NoData,
            true => RXFNE::DataReady,
        }
    }
    ///Data is not received
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == RXFNE::NoData
    }
    ///Received data is ready to be read
    #[inline(always)]
    pub fn is_data_ready(&self) -> bool {
        *self == RXFNE::DataReady
    }
}
/**TC

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC {
    ///0: Transmission is not complete
    TxNotComplete = 0,
    ///1: Transmission is complete
    TxComplete = 1,
}
impl From<TC> for bool {
    #[inline(always)]
    fn from(variant: TC) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - TC
pub type TC_R = crate::BitReader<TC>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TC {
        match self.bits {
            false => TC::TxNotComplete,
            true => TC::TxComplete,
        }
    }
    ///Transmission is not complete
    #[inline(always)]
    pub fn is_tx_not_complete(&self) -> bool {
        *self == TC::TxNotComplete
    }
    ///Transmission is complete
    #[inline(always)]
    pub fn is_tx_complete(&self) -> bool {
        *self == TC::TxComplete
    }
}
/**TXFNF

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFNF {
    ///0: Transmit FIFO is full
    Full = 0,
    ///1: Transmit FIFO is not full
    NotFull = 1,
}
impl From<TXFNF> for bool {
    #[inline(always)]
    fn from(variant: TXFNF) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFNF` reader - TXFNF
pub type TXFNF_R = crate::BitReader<TXFNF>;
impl TXFNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFNF {
        match self.bits {
            false => TXFNF::Full,
            true => TXFNF::NotFull,
        }
    }
    ///Transmit FIFO is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXFNF::Full
    }
    ///Transmit FIFO is not full
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXFNF::NotFull
    }
}
/**LBDF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDF {
    ///0: LIN break not detected
    NotDetected = 0,
    ///1: LIN break detected
    Detected = 1,
}
impl From<LBDF> for bool {
    #[inline(always)]
    fn from(variant: LBDF) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDF` reader - LBDF
pub type LBDF_R = crate::BitReader<LBDF>;
impl LBDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDF {
        match self.bits {
            false => LBDF::NotDetected,
            true => LBDF::Detected,
        }
    }
    ///LIN break not detected
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LBDF::NotDetected
    }
    ///LIN break detected
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LBDF::Detected
    }
}
/**CTSIF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIF {
    ///0: No change occurred on the CTS status line
    NotChanged = 0,
    ///1: A change occurred on the CTS status line
    Changed = 1,
}
impl From<CTSIF> for bool {
    #[inline(always)]
    fn from(variant: CTSIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIF` reader - CTSIF
pub type CTSIF_R = crate::BitReader<CTSIF>;
impl CTSIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSIF {
        match self.bits {
            false => CTSIF::NotChanged,
            true => CTSIF::Changed,
        }
    }
    ///No change occurred on the CTS status line
    #[inline(always)]
    pub fn is_not_changed(&self) -> bool {
        *self == CTSIF::NotChanged
    }
    ///A change occurred on the CTS status line
    #[inline(always)]
    pub fn is_changed(&self) -> bool {
        *self == CTSIF::Changed
    }
}
/**CTS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTS {
    ///0: CTS line set
    Set = 0,
    ///1: CTS line reset
    Reset = 1,
}
impl From<CTS> for bool {
    #[inline(always)]
    fn from(variant: CTS) -> Self {
        variant as u8 != 0
    }
}
///Field `CTS` reader - CTS
pub type CTS_R = crate::BitReader<CTS>;
impl CTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTS {
        match self.bits {
            false => CTS::Set,
            true => CTS::Reset,
        }
    }
    ///CTS line set
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CTS::Set
    }
    ///CTS line reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CTS::Reset
    }
}
/**RTOF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOF {
    ///0: Timeout value not reached
    NotReached = 0,
    ///1: Timeout value reached without any data reception
    Reached = 1,
}
impl From<RTOF> for bool {
    #[inline(always)]
    fn from(variant: RTOF) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOF` reader - RTOF
pub type RTOF_R = crate::BitReader<RTOF>;
impl RTOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTOF {
        match self.bits {
            false => RTOF::NotReached,
            true => RTOF::Reached,
        }
    }
    ///Timeout value not reached
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == RTOF::NotReached
    }
    ///Timeout value reached without any data reception
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == RTOF::Reached
    }
}
/**EOBF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBF {
    ///0: End of Block not reached
    NotReached = 0,
    ///1: End of Block (number of characters) reached
    Reached = 1,
}
impl From<EOBF> for bool {
    #[inline(always)]
    fn from(variant: EOBF) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBF` reader - EOBF
pub type EOBF_R = crate::BitReader<EOBF>;
impl EOBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOBF {
        match self.bits {
            false => EOBF::NotReached,
            true => EOBF::Reached,
        }
    }
    ///End of Block not reached
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == EOBF::NotReached
    }
    ///End of Block (number of characters) reached
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == EOBF::Reached
    }
}
/**SPI slave underrun error flag In Slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to .

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR {
    ///0: No underrun error
    NoUnderrun = 0,
    ///1: underrun error
    Underrun = 1,
}
impl From<UDR> for bool {
    #[inline(always)]
    fn from(variant: UDR) -> Self {
        variant as u8 != 0
    }
}
///Field `UDR` reader - SPI slave underrun error flag In Slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to .
pub type UDR_R = crate::BitReader<UDR>;
impl UDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDR {
        match self.bits {
            false => UDR::NoUnderrun,
            true => UDR::Underrun,
        }
    }
    ///No underrun error
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDR::NoUnderrun
    }
    ///underrun error
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDR::Underrun
    }
}
///Field `ABRE` reader - ABRE
pub type ABRE_R = crate::BitReader;
///Field `ABRF` reader - ABRF
pub type ABRF_R = crate::BitReader;
/**BUSY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: USART is idle (no reception)
    Idle = 0,
    ///1: Reception on going
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::Idle,
            true => BUSY::Busy,
        }
    }
    ///USART is idle (no reception)
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY::Idle
    }
    ///Reception on going
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
/**CMF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMF {
    ///0: No Character match detected
    NoMatch = 0,
    ///1: Character match detected
    Match = 1,
}
impl From<CMF> for bool {
    #[inline(always)]
    fn from(variant: CMF) -> Self {
        variant as u8 != 0
    }
}
///Field `CMF` reader - CMF
pub type CMF_R = crate::BitReader<CMF>;
impl CMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMF {
        match self.bits {
            false => CMF::NoMatch,
            true => CMF::Match,
        }
    }
    ///No Character match detected
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CMF::NoMatch
    }
    ///Character match detected
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CMF::Match
    }
}
/**SBKF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKF {
    ///0: No break character transmitted
    NoBreak = 0,
    ///1: Break character transmitted
    Break = 1,
}
impl From<SBKF> for bool {
    #[inline(always)]
    fn from(variant: SBKF) -> Self {
        variant as u8 != 0
    }
}
///Field `SBKF` reader - SBKF
pub type SBKF_R = crate::BitReader<SBKF>;
impl SBKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBKF {
        match self.bits {
            false => SBKF::NoBreak,
            true => SBKF::Break,
        }
    }
    ///No break character transmitted
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == SBKF::NoBreak
    }
    ///Break character transmitted
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBKF::Break
    }
}
///Field `RWU` reader - RWU
pub type RWU_R = crate::BitReader;
///Field `TEACK` reader - TEACK
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - REACK
pub type REACK_R = crate::BitReader;
/**TXFE

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFE {
    ///0: TXFIFO not empty.
    NotEmpty = 0,
    ///1: TXFIFO empty.
    Empty = 1,
}
impl From<TXFE> for bool {
    #[inline(always)]
    fn from(variant: TXFE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFE` reader - TXFE
pub type TXFE_R = crate::BitReader<TXFE>;
impl TXFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFE {
        match self.bits {
            false => TXFE::NotEmpty,
            true => TXFE::Empty,
        }
    }
    ///TXFIFO not empty.
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXFE::NotEmpty
    }
    ///TXFIFO empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFE::Empty
    }
}
/**RXFF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFF {
    ///0: RXFIFO not full.
    NotFull = 0,
    ///1: RXFIFO Full.
    Full = 1,
}
impl From<RXFF> for bool {
    #[inline(always)]
    fn from(variant: RXFF) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFF` reader - RXFF
pub type RXFF_R = crate::BitReader<RXFF>;
impl RXFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFF {
        match self.bits {
            false => RXFF::NotFull,
            true => RXFF::Full,
        }
    }
    ///RXFIFO not full.
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFF::NotFull
    }
    ///RXFIFO Full.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFF::Full
    }
}
/**TCBGT

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGT {
    ///0: Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)
    NotCompleted = 0,
    ///1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card)
    Completed = 1,
}
impl From<TCBGT> for bool {
    #[inline(always)]
    fn from(variant: TCBGT) -> Self {
        variant as u8 != 0
    }
}
///Field `TCBGT` reader - TCBGT
pub type TCBGT_R = crate::BitReader<TCBGT>;
impl TCBGT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCBGT {
        match self.bits {
            false => TCBGT::NotCompleted,
            true => TCBGT::Completed,
        }
    }
    ///Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TCBGT::NotCompleted
    }
    ///Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card)
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TCBGT::Completed
    }
}
/**RXFT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFT {
    ///0: Receive FIFO does not reach the programmed threshold.
    NotReached = 0,
    ///1: Receive FIFO reached the programmed threshold.
    Reached = 1,
}
impl From<RXFT> for bool {
    #[inline(always)]
    fn from(variant: RXFT) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFT` reader - RXFT
pub type RXFT_R = crate::BitReader<RXFT>;
impl RXFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFT {
        match self.bits {
            false => RXFT::NotReached,
            true => RXFT::Reached,
        }
    }
    ///Receive FIFO does not reach the programmed threshold.
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == RXFT::NotReached
    }
    ///Receive FIFO reached the programmed threshold.
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == RXFT::Reached
    }
}
/**TXFT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFT {
    ///0: TXFIFO does not reach the programmed threshold.
    NotReached = 0,
    ///1: TXFIFO reached the programmed threshold.
    Reached = 1,
}
impl From<TXFT> for bool {
    #[inline(always)]
    fn from(variant: TXFT) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFT` reader - TXFT
pub type TXFT_R = crate::BitReader<TXFT>;
impl TXFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFT {
        match self.bits {
            false => TXFT::NotReached,
            true => TXFT::Reached,
        }
    }
    ///TXFIFO does not reach the programmed threshold.
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == TXFT::NotReached
    }
    ///TXFIFO reached the programmed threshold.
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == TXFT::Reached
    }
}
impl R {
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FE
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NE
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ORE
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXFNE
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFNF
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LBDF
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTSIF
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RTOF
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EOBF
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI slave underrun error flag In Slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to .
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ABRE
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ABRF
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMF
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SBKF
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RWU
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - TEACK
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - REACK
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFE
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - RXFF
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TCBGT
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - RXFT
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXFT
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("reack", &self.reack())
            .field("teack", &self.teack())
            .field("rwu", &self.rwu())
            .field("sbkf", &self.sbkf())
            .field("cmf", &self.cmf())
            .field("busy", &self.busy())
            .field("abrf", &self.abrf())
            .field("abre", &self.abre())
            .field("eobf", &self.eobf())
            .field("rtof", &self.rtof())
            .field("cts", &self.cts())
            .field("ctsif", &self.ctsif())
            .field("lbdf", &self.lbdf())
            .field("txfnf", &self.txfnf())
            .field("tc", &self.tc())
            .field("rxfne", &self.rxfne())
            .field("idle", &self.idle())
            .field("ore", &self.ore())
            .field("ne", &self.ne())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .field("txfe", &self.txfe())
            .field("rxff", &self.rxff())
            .field("tcbgt", &self.tcbgt())
            .field("rxft", &self.rxft())
            .field("txft", &self.txft())
            .field("udr", &self.udr())
            .finish()
    }
}
/**Interrupt & status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#USART1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0x0280_00c0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x0280_00c0;
}
