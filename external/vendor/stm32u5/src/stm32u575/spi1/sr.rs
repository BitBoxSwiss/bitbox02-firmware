///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Rx-Packet available RXP flag is changed by hardware. It monitors number of overall data currently available at RxFIFO if SPI is enabled. It has to be checked once a data packet is completely read out from RxFIFO.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP {
    ///0: Rx buffer empty
    Empty = 0,
    ///1: Rx buffer not empty
    NotEmpty = 1,
}
impl From<RXP> for bool {
    #[inline(always)]
    fn from(variant: RXP) -> Self {
        variant as u8 != 0
    }
}
///Field `RXP` reader - Rx-Packet available RXP flag is changed by hardware. It monitors number of overall data currently available at RxFIFO if SPI is enabled. It has to be checked once a data packet is completely read out from RxFIFO.
pub type RXP_R = crate::BitReader<RXP>;
impl RXP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXP {
        match self.bits {
            false => RXP::Empty,
            true => RXP::NotEmpty,
        }
    }
    ///Rx buffer empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXP::Empty
    }
    ///Rx buffer not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXP::NotEmpty
    }
}
/**Tx-Packet space available TXP flag is changed by hardware. It monitors overall space currently available at TxFIFO no matter if SPI is enabled or not. It has to be checked once a complete data packet is stored at TxFIFO.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP {
    ///0: Tx buffer full
    Full = 0,
    ///1: Tx buffer not full
    NotFull = 1,
}
impl From<TXP> for bool {
    #[inline(always)]
    fn from(variant: TXP) -> Self {
        variant as u8 != 0
    }
}
///Field `TXP` reader - Tx-Packet space available TXP flag is changed by hardware. It monitors overall space currently available at TxFIFO no matter if SPI is enabled or not. It has to be checked once a complete data packet is stored at TxFIFO.
pub type TXP_R = crate::BitReader<TXP>;
impl TXP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXP {
        match self.bits {
            false => TXP::Full,
            true => TXP::NotFull,
        }
    }
    ///Tx buffer full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXP::Full
    }
    ///Tx buffer not full
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXP::NotFull
    }
}
/**duplex packet DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXP {
    ///0: Duplex packet unavailable: no space for transmission and/or no data received
    Unavailable = 0,
    ///1: Duplex packet available: space for transmission and data received
    Available = 1,
}
impl From<DXP> for bool {
    #[inline(always)]
    fn from(variant: DXP) -> Self {
        variant as u8 != 0
    }
}
///Field `DXP` reader - duplex packet DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode.
pub type DXP_R = crate::BitReader<DXP>;
impl DXP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DXP {
        match self.bits {
            false => DXP::Unavailable,
            true => DXP::Available,
        }
    }
    ///Duplex packet unavailable: no space for transmission and/or no data received
    #[inline(always)]
    pub fn is_unavailable(&self) -> bool {
        *self == DXP::Unavailable
    }
    ///Duplex packet available: space for transmission and data received
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == DXP::Available
    }
}
/**end of transfer EOT is set by hardware as soon as a full transfer is complete, that is when TSIZE number of data have been transmitted and/or received on the SPI. EOT is cleared by software write 1 to EOTC bit at SPI_IFCR. EOT flag triggers an interrupt if EOTIE bit is set. If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to download the last packets contained into RxFIFO in one-shot. In master, EOT event terminates the data transaction and handles SS output optionally. When CRC is applied, the EOT event is extended over the CRC frame transaction. To restart the internal state machine properly, SPI is strongly suggested to be disabled and re-enabled before next transaction starts despite its setting is not changed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOT {
    ///0: Transfer ongoing or not started
    NotCompleted = 0,
    ///1: Transfer complete
    Completed = 1,
}
impl From<EOT> for bool {
    #[inline(always)]
    fn from(variant: EOT) -> Self {
        variant as u8 != 0
    }
}
///Field `EOT` reader - end of transfer EOT is set by hardware as soon as a full transfer is complete, that is when TSIZE number of data have been transmitted and/or received on the SPI. EOT is cleared by software write 1 to EOTC bit at SPI_IFCR. EOT flag triggers an interrupt if EOTIE bit is set. If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to download the last packets contained into RxFIFO in one-shot. In master, EOT event terminates the data transaction and handles SS output optionally. When CRC is applied, the EOT event is extended over the CRC frame transaction. To restart the internal state machine properly, SPI is strongly suggested to be disabled and re-enabled before next transaction starts despite its setting is not changed.
pub type EOT_R = crate::BitReader<EOT>;
impl EOT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOT {
        match self.bits {
            false => EOT::NotCompleted,
            true => EOT::Completed,
        }
    }
    ///Transfer ongoing or not started
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == EOT::NotCompleted
    }
    ///Transfer complete
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == EOT::Completed
    }
}
/**transmission transfer filled TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted for transmission by application software or DMA, that is when TSIZE number of data have been pushed into the TxFIFO. This bit is cleared by software write 1 to TXTFC bit at SPI_IFCR TXTF flag triggers an interrupt if TXTFIE bit is set. TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from calculating when to disable TXP and DXP interrupts.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTF {
    ///0: Transmission buffer incomplete
    NotCompleted = 0,
    ///1: Transmission buffer filled with at least one transfer
    Completed = 1,
}
impl From<TXTF> for bool {
    #[inline(always)]
    fn from(variant: TXTF) -> Self {
        variant as u8 != 0
    }
}
///Field `TXTF` reader - transmission transfer filled TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted for transmission by application software or DMA, that is when TSIZE number of data have been pushed into the TxFIFO. This bit is cleared by software write 1 to TXTFC bit at SPI_IFCR TXTF flag triggers an interrupt if TXTFIE bit is set. TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from calculating when to disable TXP and DXP interrupts.
pub type TXTF_R = crate::BitReader<TXTF>;
impl TXTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXTF {
        match self.bits {
            false => TXTF::NotCompleted,
            true => TXTF::Completed,
        }
    }
    ///Transmission buffer incomplete
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TXTF::NotCompleted
    }
    ///Transmission buffer filled with at least one transfer
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXTF::Completed
    }
}
/**underrun at slave transmission mode This bit is cleared by writing 1 to UDRC bit at SPI_IFCR Note: UDR flag applies to Slave mode only

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR {
    ///0: No underrun occurred
    NoUnderrun = 0,
    ///1: Underrun occurred
    Underrun = 1,
}
impl From<UDR> for bool {
    #[inline(always)]
    fn from(variant: UDR) -> Self {
        variant as u8 != 0
    }
}
///Field `UDR` reader - underrun at slave transmission mode This bit is cleared by writing 1 to UDRC bit at SPI_IFCR Note: UDR flag applies to Slave mode only
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
    ///No underrun occurred
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDR::NoUnderrun
    }
    ///Underrun occurred
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDR::Underrun
    }
}
/**overrun This bit is cleared by writing 1 to OVRC bit at SPI_IFCR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVR> for bool {
    #[inline(always)]
    fn from(variant: OVR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - overrun This bit is cleared by writing 1 to OVRC bit at SPI_IFCR
pub type OVR_R = crate::BitReader<OVR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR {
        match self.bits {
            false => OVR::NoOverrun,
            true => OVR::Overrun,
        }
    }
    ///No overrun occurred
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR::NoOverrun
    }
    ///Overrun occurred
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR::Overrun
    }
}
/**CRC error This bit is cleared by writing 1 to CRCEC bit at SPI_IFCR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCE {
    ///0: No CRC error detected
    NoError = 0,
    ///1: CRC error detected
    Error = 1,
}
impl From<CRCE> for bool {
    #[inline(always)]
    fn from(variant: CRCE) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCE` reader - CRC error This bit is cleared by writing 1 to CRCEC bit at SPI_IFCR
pub type CRCE_R = crate::BitReader<CRCE>;
impl CRCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCE {
        match self.bits {
            false => CRCE::NoError,
            true => CRCE::Error,
        }
    }
    ///No CRC error detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == CRCE::NoError
    }
    ///CRC error detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == CRCE::Error
    }
}
/**TI frame format error This bit is cleared by writing 1 to TIFREC bit at SPI_IFCR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFRE {
    ///0: TI frame format error detected
    NoError = 0,
    ///1: TI frame format error detected
    Error = 1,
}
impl From<TIFRE> for bool {
    #[inline(always)]
    fn from(variant: TIFRE) -> Self {
        variant as u8 != 0
    }
}
///Field `TIFRE` reader - TI frame format error This bit is cleared by writing 1 to TIFREC bit at SPI_IFCR
pub type TIFRE_R = crate::BitReader<TIFRE>;
impl TIFRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIFRE {
        match self.bits {
            false => TIFRE::NoError,
            true => TIFRE::Error,
        }
    }
    ///TI frame format error detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TIFRE::NoError
    }
    ///TI frame format error detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TIFRE::Error
    }
}
/**mode fault This bit is cleared by writing 1 to MODFC bit at SPI_IFCR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF {
    ///0: No mode fault detected
    NoFault = 0,
    ///1: Mode fault detected
    Fault = 1,
}
impl From<MODF> for bool {
    #[inline(always)]
    fn from(variant: MODF) -> Self {
        variant as u8 != 0
    }
}
///Field `MODF` reader - mode fault This bit is cleared by writing 1 to MODFC bit at SPI_IFCR
pub type MODF_R = crate::BitReader<MODF>;
impl MODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODF {
        match self.bits {
            false => MODF::NoFault,
            true => MODF::Fault,
        }
    }
    ///No mode fault detected
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODF::NoFault
    }
    ///Mode fault detected
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODF::Fault
    }
}
/**suspension status In Master mode, SUSP is set by hardware either as soon as the current frame is completed after CSUSP request is done or at master automatic suspend receive mode (MASRX bit is set at SPI_CR1 register) on RxFIFO full condition. SUSP generates an interrupt when EOTIE is set. This bit has to be cleared prior SPI is disabled by writing 1 to SUSPC bit at SPI_IFCR.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP {
    ///0: Master not suspended
    NotSuspended = 0,
    ///1: Master suspended
    Suspended = 1,
}
impl From<SUSP> for bool {
    #[inline(always)]
    fn from(variant: SUSP) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSP` reader - suspension status In Master mode, SUSP is set by hardware either as soon as the current frame is completed after CSUSP request is done or at master automatic suspend receive mode (MASRX bit is set at SPI_CR1 register) on RxFIFO full condition. SUSP generates an interrupt when EOTIE is set. This bit has to be cleared prior SPI is disabled by writing 1 to SUSPC bit at SPI_IFCR.
pub type SUSP_R = crate::BitReader<SUSP>;
impl SUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUSP {
        match self.bits {
            false => SUSP::NotSuspended,
            true => SUSP::Suspended,
        }
    }
    ///Master not suspended
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP::NotSuspended
    }
    ///Master suspended
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP::Suspended
    }
}
/**TxFIFO transmission complete The flag behavior depends on TSIZE setting. When TSIZE=0 the TXC is changed by hardware exclusively and it raises each time the TxFIFO becomes empty and there is no activity on the bus. If TSIZE <>0 there is no specific reason to monitor TXC as it just copies the EOT flag value including its software clearing. The TXC generates an interrupt when EOTIE is set.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXC {
    ///0: Transmission ongoing
    Ongoing = 0,
    ///1: Transmission completed
    Completed = 1,
}
impl From<TXC> for bool {
    #[inline(always)]
    fn from(variant: TXC) -> Self {
        variant as u8 != 0
    }
}
///Field `TXC` reader - TxFIFO transmission complete The flag behavior depends on TSIZE setting. When TSIZE=0 the TXC is changed by hardware exclusively and it raises each time the TxFIFO becomes empty and there is no activity on the bus. If TSIZE <>0 there is no specific reason to monitor TXC as it just copies the EOT flag value including its software clearing. The TXC generates an interrupt when EOTIE is set.
pub type TXC_R = crate::BitReader<TXC>;
impl TXC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXC {
        match self.bits {
            false => TXC::Ongoing,
            true => TXC::Completed,
        }
    }
    ///Transmission ongoing
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == TXC::Ongoing
    }
    ///Transmission completed
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXC::Completed
    }
}
/**RxFIFO packing level When RXWNE=0 and data size is set up to 16-bit, the value gives number of remaining data frames persisting at RxFIFO. Note: (*): Optional value when data size is set up to 8-bit only. When data size is greater than 16-bit, these bits are always read as 00. In that consequence, the single data frame received at the FIFO cannot be detected neither by RWNE nor by RXPLVL bits if data size is set from 17 to 24 bits. The user then must apply other methods like TSIZE>0 or FTHLV=0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPLVL {
    ///0: Zero frames beyond packing ratio available
    ZeroFrames = 0,
    ///1: One frame beyond packing ratio available
    OneFrame = 1,
    ///2: Two frame beyond packing ratio available
    TwoFrames = 2,
    ///3: Three frame beyond packing ratio available
    ThreeFrames = 3,
}
impl From<RXPLVL> for u8 {
    #[inline(always)]
    fn from(variant: RXPLVL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXPLVL {
    type Ux = u8;
}
impl crate::IsEnum for RXPLVL {}
///Field `RXPLVL` reader - RxFIFO packing level When RXWNE=0 and data size is set up to 16-bit, the value gives number of remaining data frames persisting at RxFIFO. Note: (*): Optional value when data size is set up to 8-bit only. When data size is greater than 16-bit, these bits are always read as 00. In that consequence, the single data frame received at the FIFO cannot be detected neither by RWNE nor by RXPLVL bits if data size is set from 17 to 24 bits. The user then must apply other methods like TSIZE>0 or FTHLV=0.
pub type RXPLVL_R = crate::FieldReader<RXPLVL>;
impl RXPLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXPLVL {
        match self.bits {
            0 => RXPLVL::ZeroFrames,
            1 => RXPLVL::OneFrame,
            2 => RXPLVL::TwoFrames,
            3 => RXPLVL::ThreeFrames,
            _ => unreachable!(),
        }
    }
    ///Zero frames beyond packing ratio available
    #[inline(always)]
    pub fn is_zero_frames(&self) -> bool {
        *self == RXPLVL::ZeroFrames
    }
    ///One frame beyond packing ratio available
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == RXPLVL::OneFrame
    }
    ///Two frame beyond packing ratio available
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == RXPLVL::TwoFrames
    }
    ///Three frame beyond packing ratio available
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == RXPLVL::ThreeFrames
    }
}
/**RxFIFO word not empty Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL\[1:0\] information about RxFIFO occupancy by residual data.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXWNE {
    ///0: Less than 32-bit data frame received
    LessThan32 = 0,
    ///1: At least 32-bit data frame received
    AtLeast32 = 1,
}
impl From<RXWNE> for bool {
    #[inline(always)]
    fn from(variant: RXWNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXWNE` reader - RxFIFO word not empty Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL\[1:0\] information about RxFIFO occupancy by residual data.
pub type RXWNE_R = crate::BitReader<RXWNE>;
impl RXWNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXWNE {
        match self.bits {
            false => RXWNE::LessThan32,
            true => RXWNE::AtLeast32,
        }
    }
    ///Less than 32-bit data frame received
    #[inline(always)]
    pub fn is_less_than32(&self) -> bool {
        *self == RXWNE::LessThan32
    }
    ///At least 32-bit data frame received
    #[inline(always)]
    pub fn is_at_least32(&self) -> bool {
        *self == RXWNE::AtLeast32
    }
}
///Field `CTSIZE` reader - number of data frames remaining in current TSIZE session The value is not quite reliable when traffic is ongoing on bus or during autonomous operation at low-power mode. Note: CTSIZE\[15:0\] bits are not available at instances with limited set of features
pub type CTSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - Rx-Packet available RXP flag is changed by hardware. It monitors number of overall data currently available at RxFIFO if SPI is enabled. It has to be checked once a data packet is completely read out from RxFIFO.
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx-Packet space available TXP flag is changed by hardware. It monitors overall space currently available at TxFIFO no matter if SPI is enabled or not. It has to be checked once a complete data packet is stored at TxFIFO.
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - duplex packet DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode.
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - end of transfer EOT is set by hardware as soon as a full transfer is complete, that is when TSIZE number of data have been transmitted and/or received on the SPI. EOT is cleared by software write 1 to EOTC bit at SPI_IFCR. EOT flag triggers an interrupt if EOTIE bit is set. If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to download the last packets contained into RxFIFO in one-shot. In master, EOT event terminates the data transaction and handles SS output optionally. When CRC is applied, the EOT event is extended over the CRC frame transaction. To restart the internal state machine properly, SPI is strongly suggested to be disabled and re-enabled before next transaction starts despite its setting is not changed.
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - transmission transfer filled TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted for transmission by application software or DMA, that is when TSIZE number of data have been pushed into the TxFIFO. This bit is cleared by software write 1 to TXTFC bit at SPI_IFCR TXTF flag triggers an interrupt if TXTFIE bit is set. TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from calculating when to disable TXP and DXP interrupts.
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - underrun at slave transmission mode This bit is cleared by writing 1 to UDRC bit at SPI_IFCR Note: UDR flag applies to Slave mode only
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - overrun This bit is cleared by writing 1 to OVRC bit at SPI_IFCR
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC error This bit is cleared by writing 1 to CRCEC bit at SPI_IFCR
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TI frame format error This bit is cleared by writing 1 to TIFREC bit at SPI_IFCR
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - mode fault This bit is cleared by writing 1 to MODFC bit at SPI_IFCR
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - suspension status In Master mode, SUSP is set by hardware either as soon as the current frame is completed after CSUSP request is done or at master automatic suspend receive mode (MASRX bit is set at SPI_CR1 register) on RxFIFO full condition. SUSP generates an interrupt when EOTIE is set. This bit has to be cleared prior SPI is disabled by writing 1 to SUSPC bit at SPI_IFCR.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TxFIFO transmission complete The flag behavior depends on TSIZE setting. When TSIZE=0 the TXC is changed by hardware exclusively and it raises each time the TxFIFO becomes empty and there is no activity on the bus. If TSIZE <>0 there is no specific reason to monitor TXC as it just copies the EOT flag value including its software clearing. The TXC generates an interrupt when EOTIE is set.
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - RxFIFO packing level When RXWNE=0 and data size is set up to 16-bit, the value gives number of remaining data frames persisting at RxFIFO. Note: (*): Optional value when data size is set up to 8-bit only. When data size is greater than 16-bit, these bits are always read as 00. In that consequence, the single data frame received at the FIFO cannot be detected neither by RWNE nor by RXPLVL bits if data size is set from 17 to 24 bits. The user then must apply other methods like TSIZE>0 or FTHLV=0.
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - RxFIFO word not empty Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL\[1:0\] information about RxFIFO occupancy by residual data.
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - number of data frames remaining in current TSIZE session The value is not quite reliable when traffic is ongoing on bus or during autonomous operation at low-power mode. Note: CTSIZE\[15:0\] bits are not available at instances with limited set of features
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rxp", &self.rxp())
            .field("txp", &self.txp())
            .field("dxp", &self.dxp())
            .field("eot", &self.eot())
            .field("txtf", &self.txtf())
            .field("udr", &self.udr())
            .field("ovr", &self.ovr())
            .field("crce", &self.crce())
            .field("tifre", &self.tifre())
            .field("modf", &self.modf())
            .field("susp", &self.susp())
            .field("txc", &self.txc())
            .field("rxplvl", &self.rxplvl())
            .field("rxwne", &self.rxwne())
            .field("ctsize", &self.ctsize())
            .finish()
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x1002
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x1002;
}
