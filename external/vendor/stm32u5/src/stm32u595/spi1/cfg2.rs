///Register `CFG2` reader
pub type R = crate::R<CFG2rs>;
///Register `CFG2` writer
pub type W = crate::W<CFG2rs>;
///Field `MSSI` reader - Master SS Idleness
pub type MSSI_R = crate::FieldReader;
///Field `MSSI` writer - Master SS Idleness
pub type MSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `MIDI` reader - Master Inter-Data Idleness
pub type MIDI_R = crate::FieldReader;
///Field `MIDI` writer - Master Inter-Data Idleness
pub type MIDI_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**RDIMM

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIOM {
    ///0: RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)
    Active = 0,
    ///1: RDY signal is overtaken from alternate function input (at master case) or output (at slave case) of the dedicated pin (RDIOP setting takes effect)
    Pin = 1,
}
impl From<RDIOM> for bool {
    #[inline(always)]
    fn from(variant: RDIOM) -> Self {
        variant as u8 != 0
    }
}
///Field `RDIOM` reader - RDIMM
pub type RDIOM_R = crate::BitReader<RDIOM>;
impl RDIOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDIOM {
        match self.bits {
            false => RDIOM::Active,
            true => RDIOM::Pin,
        }
    }
    ///RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RDIOM::Active
    }
    ///RDY signal is overtaken from alternate function input (at master case) or output (at slave case) of the dedicated pin (RDIOP setting takes effect)
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == RDIOM::Pin
    }
}
///Field `RDIOM` writer - RDIMM
pub type RDIOM_W<'a, REG> = crate::BitWriter<'a, REG, RDIOM>;
impl<'a, REG> RDIOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(RDIOM::Active)
    }
    ///RDY signal is overtaken from alternate function input (at master case) or output (at slave case) of the dedicated pin (RDIOP setting takes effect)
    #[inline(always)]
    pub fn pin(self) -> &'a mut crate::W<REG> {
        self.variant(RDIOM::Pin)
    }
}
/**RDIOP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIOP {
    ///0: high level of the signal means the slave is ready for communication
    High = 0,
    ///1: low level of the signal means the slave is ready for communication
    Low = 1,
}
impl From<RDIOP> for bool {
    #[inline(always)]
    fn from(variant: RDIOP) -> Self {
        variant as u8 != 0
    }
}
///Field `RDIOP` reader - RDIOP
pub type RDIOP_R = crate::BitReader<RDIOP>;
impl RDIOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDIOP {
        match self.bits {
            false => RDIOP::High,
            true => RDIOP::Low,
        }
    }
    ///high level of the signal means the slave is ready for communication
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RDIOP::High
    }
    ///low level of the signal means the slave is ready for communication
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RDIOP::Low
    }
}
///Field `RDIOP` writer - RDIOP
pub type RDIOP_W<'a, REG> = crate::BitWriter<'a, REG, RDIOP>;
impl<'a, REG> RDIOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///high level of the signal means the slave is ready for communication
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(RDIOP::High)
    }
    ///low level of the signal means the slave is ready for communication
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(RDIOP::Low)
    }
}
/**Swap functionality of MISO and MOSI pins

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSWP {
    ///0: MISO and MOSI not swapped
    Disabled = 0,
    ///1: MISO and MOSI swapped
    Enabled = 1,
}
impl From<IOSWP> for bool {
    #[inline(always)]
    fn from(variant: IOSWP) -> Self {
        variant as u8 != 0
    }
}
///Field `IOSWP` reader - Swap functionality of MISO and MOSI pins
pub type IOSWP_R = crate::BitReader<IOSWP>;
impl IOSWP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOSWP {
        match self.bits {
            false => IOSWP::Disabled,
            true => IOSWP::Enabled,
        }
    }
    ///MISO and MOSI not swapped
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOSWP::Disabled
    }
    ///MISO and MOSI swapped
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOSWP::Enabled
    }
}
///Field `IOSWP` writer - Swap functionality of MISO and MOSI pins
pub type IOSWP_W<'a, REG> = crate::BitWriter<'a, REG, IOSWP>;
impl<'a, REG> IOSWP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MISO and MOSI not swapped
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOSWP::Disabled)
    }
    ///MISO and MOSI swapped
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOSWP::Enabled)
    }
}
/**SPI Communication Mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMM {
    ///0: Full duplex
    FullDuplex = 0,
    ///1: Simplex transmitter only
    Transmitter = 1,
    ///2: Simplex receiver only
    Receiver = 2,
    ///3: Half duplex
    HalfDuplex = 3,
}
impl From<COMM> for u8 {
    #[inline(always)]
    fn from(variant: COMM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMM {
    type Ux = u8;
}
impl crate::IsEnum for COMM {}
///Field `COMM` reader - SPI Communication Mode
pub type COMM_R = crate::FieldReader<COMM>;
impl COMM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMM {
        match self.bits {
            0 => COMM::FullDuplex,
            1 => COMM::Transmitter,
            2 => COMM::Receiver,
            3 => COMM::HalfDuplex,
            _ => unreachable!(),
        }
    }
    ///Full duplex
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == COMM::FullDuplex
    }
    ///Simplex transmitter only
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == COMM::Transmitter
    }
    ///Simplex receiver only
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == COMM::Receiver
    }
    ///Half duplex
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == COMM::HalfDuplex
    }
}
///Field `COMM` writer - SPI Communication Mode
pub type COMM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COMM, crate::Safe>;
impl<'a, REG> COMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Full duplex
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(COMM::FullDuplex)
    }
    ///Simplex transmitter only
    #[inline(always)]
    pub fn transmitter(self) -> &'a mut crate::W<REG> {
        self.variant(COMM::Transmitter)
    }
    ///Simplex receiver only
    #[inline(always)]
    pub fn receiver(self) -> &'a mut crate::W<REG> {
        self.variant(COMM::Receiver)
    }
    ///Half duplex
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(COMM::HalfDuplex)
    }
}
/**Serial Protocol

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SP {
    ///0: Motorola SPI protocol
    Motorola = 0,
    ///1: TI SPI protocol
    Ti = 1,
}
impl From<SP> for u8 {
    #[inline(always)]
    fn from(variant: SP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SP {
    type Ux = u8;
}
impl crate::IsEnum for SP {}
///Field `SP` reader - Serial Protocol
pub type SP_R = crate::FieldReader<SP>;
impl SP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SP> {
        match self.bits {
            0 => Some(SP::Motorola),
            1 => Some(SP::Ti),
            _ => None,
        }
    }
    ///Motorola SPI protocol
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == SP::Motorola
    }
    ///TI SPI protocol
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == SP::Ti
    }
}
///Field `SP` writer - Serial Protocol
pub type SP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SP>;
impl<'a, REG> SP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Motorola SPI protocol
    #[inline(always)]
    pub fn motorola(self) -> &'a mut crate::W<REG> {
        self.variant(SP::Motorola)
    }
    ///TI SPI protocol
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(SP::Ti)
    }
}
/**SPI Master

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER {
    ///0: Slave configuration
    Slave = 0,
    ///1: Master configuration
    Master = 1,
}
impl From<MASTER> for bool {
    #[inline(always)]
    fn from(variant: MASTER) -> Self {
        variant as u8 != 0
    }
}
///Field `MASTER` reader - SPI Master
pub type MASTER_R = crate::BitReader<MASTER>;
impl MASTER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MASTER {
        match self.bits {
            false => MASTER::Slave,
            true => MASTER::Master,
        }
    }
    ///Slave configuration
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MASTER::Slave
    }
    ///Master configuration
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MASTER::Master
    }
}
///Field `MASTER` writer - SPI Master
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG, MASTER>;
impl<'a, REG> MASTER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave configuration
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MASTER::Slave)
    }
    ///Master configuration
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MASTER::Master)
    }
}
/**Data frame format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFRST {
    ///0: Data is transmitted/received with the MSB first
    Msbfirst = 0,
    ///1: Data is transmitted/received with the LSB first
    Lsbfirst = 1,
}
impl From<LSBFRST> for bool {
    #[inline(always)]
    fn from(variant: LSBFRST) -> Self {
        variant as u8 != 0
    }
}
///Field `LSBFRST` reader - Data frame format
pub type LSBFRST_R = crate::BitReader<LSBFRST>;
impl LSBFRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSBFRST {
        match self.bits {
            false => LSBFRST::Msbfirst,
            true => LSBFRST::Lsbfirst,
        }
    }
    ///Data is transmitted/received with the MSB first
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFRST::Msbfirst
    }
    ///Data is transmitted/received with the LSB first
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFRST::Lsbfirst
    }
}
///Field `LSBFRST` writer - Data frame format
pub type LSBFRST_W<'a, REG> = crate::BitWriter<'a, REG, LSBFRST>;
impl<'a, REG> LSBFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data is transmitted/received with the MSB first
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFRST::Msbfirst)
    }
    ///Data is transmitted/received with the LSB first
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFRST::Lsbfirst)
    }
}
/**Clock phase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA {
    ///0: The first clock transition is the first data capture edge
    FirstEdge = 0,
    ///1: The second clock transition is the first data capture edge
    SecondEdge = 1,
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
            false => CPHA::FirstEdge,
            true => CPHA::SecondEdge,
        }
    }
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA::FirstEdge
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA::SecondEdge
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
    pub fn first_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::FirstEdge)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::SecondEdge)
    }
}
/**Clock polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL {
    ///0: CK to 0 when idle
    IdleLow = 0,
    ///1: CK to 1 when idle
    IdleHigh = 1,
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
            false => CPOL::IdleLow,
            true => CPOL::IdleHigh,
        }
    }
    ///CK to 0 when idle
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL::IdleLow
    }
    ///CK to 1 when idle
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL::IdleHigh
    }
}
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CK to 0 when idle
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::IdleLow)
    }
    ///CK to 1 when idle
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::IdleHigh)
    }
}
/**Software management of SS signal input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM {
    ///0: Software slave management disabled
    Disabled = 0,
    ///1: Software slave management enabled
    Enabled = 1,
}
impl From<SSM> for bool {
    #[inline(always)]
    fn from(variant: SSM) -> Self {
        variant as u8 != 0
    }
}
///Field `SSM` reader - Software management of SS signal input
pub type SSM_R = crate::BitReader<SSM>;
impl SSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSM {
        match self.bits {
            false => SSM::Disabled,
            true => SSM::Enabled,
        }
    }
    ///Software slave management disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM::Disabled
    }
    ///Software slave management enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM::Enabled
    }
}
///Field `SSM` writer - Software management of SS signal input
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG, SSM>;
impl<'a, REG> SSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software slave management disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSM::Disabled)
    }
    ///Software slave management enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSM::Enabled)
    }
}
/**SS input/output polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIOP {
    ///0: Low level is active for SS signal
    ActiveLow = 0,
    ///1: High level is active for SS signal
    ActiveHigh = 1,
}
impl From<SSIOP> for bool {
    #[inline(always)]
    fn from(variant: SSIOP) -> Self {
        variant as u8 != 0
    }
}
///Field `SSIOP` reader - SS input/output polarity
pub type SSIOP_R = crate::BitReader<SSIOP>;
impl SSIOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSIOP {
        match self.bits {
            false => SSIOP::ActiveLow,
            true => SSIOP::ActiveHigh,
        }
    }
    ///Low level is active for SS signal
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == SSIOP::ActiveLow
    }
    ///High level is active for SS signal
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == SSIOP::ActiveHigh
    }
}
///Field `SSIOP` writer - SS input/output polarity
pub type SSIOP_W<'a, REG> = crate::BitWriter<'a, REG, SSIOP>;
impl<'a, REG> SSIOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low level is active for SS signal
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(SSIOP::ActiveLow)
    }
    ///High level is active for SS signal
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(SSIOP::ActiveHigh)
    }
}
/**SS output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE {
    ///0: SS output is disabled in master mode
    Disabled = 0,
    ///1: SS output is enabled in master mode
    Enabled = 1,
}
impl From<SSOE> for bool {
    #[inline(always)]
    fn from(variant: SSOE) -> Self {
        variant as u8 != 0
    }
}
///Field `SSOE` reader - SS output enable
pub type SSOE_R = crate::BitReader<SSOE>;
impl SSOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSOE {
        match self.bits {
            false => SSOE::Disabled,
            true => SSOE::Enabled,
        }
    }
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE::Disabled
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE::Enabled
    }
}
///Field `SSOE` writer - SS output enable
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG, SSOE>;
impl<'a, REG> SSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Disabled)
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Enabled)
    }
}
/**SS output management in master mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOM {
    ///0: SS is asserted until data transfer complete
    Asserted = 0,
    ///1: Data frames interleaved with SS not asserted during MIDI
    NotAsserted = 1,
}
impl From<SSOM> for bool {
    #[inline(always)]
    fn from(variant: SSOM) -> Self {
        variant as u8 != 0
    }
}
///Field `SSOM` reader - SS output management in master mode
pub type SSOM_R = crate::BitReader<SSOM>;
impl SSOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSOM {
        match self.bits {
            false => SSOM::Asserted,
            true => SSOM::NotAsserted,
        }
    }
    ///SS is asserted until data transfer complete
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SSOM::Asserted
    }
    ///Data frames interleaved with SS not asserted during MIDI
    #[inline(always)]
    pub fn is_not_asserted(&self) -> bool {
        *self == SSOM::NotAsserted
    }
}
///Field `SSOM` writer - SS output management in master mode
pub type SSOM_W<'a, REG> = crate::BitWriter<'a, REG, SSOM>;
impl<'a, REG> SSOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SS is asserted until data transfer complete
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SSOM::Asserted)
    }
    ///Data frames interleaved with SS not asserted during MIDI
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SSOM::NotAsserted)
    }
}
/**Alternate function GPIOs control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFCNTR {
    ///0: Peripheral takes no control of GPIOs while disabled
    NotControlled = 0,
    ///1: Peripheral controls GPIOs while disabled
    Controlled = 1,
}
impl From<AFCNTR> for bool {
    #[inline(always)]
    fn from(variant: AFCNTR) -> Self {
        variant as u8 != 0
    }
}
///Field `AFCNTR` reader - Alternate function GPIOs control
pub type AFCNTR_R = crate::BitReader<AFCNTR>;
impl AFCNTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFCNTR {
        match self.bits {
            false => AFCNTR::NotControlled,
            true => AFCNTR::Controlled,
        }
    }
    ///Peripheral takes no control of GPIOs while disabled
    #[inline(always)]
    pub fn is_not_controlled(&self) -> bool {
        *self == AFCNTR::NotControlled
    }
    ///Peripheral controls GPIOs while disabled
    #[inline(always)]
    pub fn is_controlled(&self) -> bool {
        *self == AFCNTR::Controlled
    }
}
///Field `AFCNTR` writer - Alternate function GPIOs control
pub type AFCNTR_W<'a, REG> = crate::BitWriter<'a, REG, AFCNTR>;
impl<'a, REG> AFCNTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral takes no control of GPIOs while disabled
    #[inline(always)]
    pub fn not_controlled(self) -> &'a mut crate::W<REG> {
        self.variant(AFCNTR::NotControlled)
    }
    ///Peripheral controls GPIOs while disabled
    #[inline(always)]
    pub fn controlled(self) -> &'a mut crate::W<REG> {
        self.variant(AFCNTR::Controlled)
    }
}
impl R {
    ///Bits 0:3 - Master SS Idleness
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Master Inter-Data Idleness
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 13 - RDIMM
    #[inline(always)]
    pub fn rdiom(&self) -> RDIOM_R {
        RDIOM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RDIOP
    #[inline(always)]
    pub fn rdiop(&self) -> RDIOP_R {
        RDIOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Swap functionality of MISO and MOSI pins
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:18 - SPI Communication Mode
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:21 - Serial Protocol
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bit 22 - SPI Master
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Data frame format
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software management of SS signal input
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - SS input/output polarity
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SS output enable
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SS output management in master mode
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Alternate function GPIOs control
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("afcntr", &self.afcntr())
            .field("ssom", &self.ssom())
            .field("ssoe", &self.ssoe())
            .field("ssiop", &self.ssiop())
            .field("ssm", &self.ssm())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .field("lsbfrst", &self.lsbfrst())
            .field("master", &self.master())
            .field("sp", &self.sp())
            .field("comm", &self.comm())
            .field("ioswp", &self.ioswp())
            .field("rdiop", &self.rdiop())
            .field("rdiom", &self.rdiom())
            .field("midi", &self.midi())
            .field("mssi", &self.mssi())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Master SS Idleness
    #[inline(always)]
    pub fn mssi(&mut self) -> MSSI_W<CFG2rs> {
        MSSI_W::new(self, 0)
    }
    ///Bits 4:7 - Master Inter-Data Idleness
    #[inline(always)]
    pub fn midi(&mut self) -> MIDI_W<CFG2rs> {
        MIDI_W::new(self, 4)
    }
    ///Bit 13 - RDIMM
    #[inline(always)]
    pub fn rdiom(&mut self) -> RDIOM_W<CFG2rs> {
        RDIOM_W::new(self, 13)
    }
    ///Bit 14 - RDIOP
    #[inline(always)]
    pub fn rdiop(&mut self) -> RDIOP_W<CFG2rs> {
        RDIOP_W::new(self, 14)
    }
    ///Bit 15 - Swap functionality of MISO and MOSI pins
    #[inline(always)]
    pub fn ioswp(&mut self) -> IOSWP_W<CFG2rs> {
        IOSWP_W::new(self, 15)
    }
    ///Bits 17:18 - SPI Communication Mode
    #[inline(always)]
    pub fn comm(&mut self) -> COMM_W<CFG2rs> {
        COMM_W::new(self, 17)
    }
    ///Bits 19:21 - Serial Protocol
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W<CFG2rs> {
        SP_W::new(self, 19)
    }
    ///Bit 22 - SPI Master
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W<CFG2rs> {
        MASTER_W::new(self, 22)
    }
    ///Bit 23 - Data frame format
    #[inline(always)]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<CFG2rs> {
        LSBFRST_W::new(self, 23)
    }
    ///Bit 24 - Clock phase
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<CFG2rs> {
        CPHA_W::new(self, 24)
    }
    ///Bit 25 - Clock polarity
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<CFG2rs> {
        CPOL_W::new(self, 25)
    }
    ///Bit 26 - Software management of SS signal input
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W<CFG2rs> {
        SSM_W::new(self, 26)
    }
    ///Bit 28 - SS input/output polarity
    #[inline(always)]
    pub fn ssiop(&mut self) -> SSIOP_W<CFG2rs> {
        SSIOP_W::new(self, 28)
    }
    ///Bit 29 - SS output enable
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<CFG2rs> {
        SSOE_W::new(self, 29)
    }
    ///Bit 30 - SS output management in master mode
    #[inline(always)]
    pub fn ssom(&mut self) -> SSOM_W<CFG2rs> {
        SSOM_W::new(self, 30)
    }
    ///Bit 31 - Alternate function GPIOs control
    #[inline(always)]
    pub fn afcntr(&mut self) -> AFCNTR_W<CFG2rs> {
        AFCNTR_W::new(self, 31)
    }
}
/**configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#SPI1:CFG2)*/
pub struct CFG2rs;
impl crate::RegisterSpec for CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`cfg2::R`](R) reader structure
impl crate::Readable for CFG2rs {}
///`write(|w| ..)` method takes [`cfg2::W`](W) writer structure
impl crate::Writable for CFG2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFG2 to value 0
impl crate::Resettable for CFG2rs {}
