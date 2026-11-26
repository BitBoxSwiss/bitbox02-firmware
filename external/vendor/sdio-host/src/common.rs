use core::fmt;
use core::marker::PhantomData;

/// Types of SD Card
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CardCapacity {
    /// SDSC / Standard Capacity (<= 2GB)
    StandardCapacity,
    /// SDHC / High capacity (<= 32GB for SD cards, <= 256GB for eMMC)
    HighCapacity,
}

impl Default for CardCapacity {
    fn default() -> Self {
        CardCapacity::StandardCapacity
    }
}

/// The number of data lines in use on the SDMMC bus
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum BusWidth {
    #[non_exhaustive]
    Unknown,
    One = 1,
    Four = 4,
    Eight = 8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BlockSize {
    #[non_exhaustive]
    B1 = 0,
    B2 = 1,
    B4 = 2,
    B8 = 3,
    B16 = 4,
    B32 = 5,
    B64 = 6,
    B128 = 7,
    B256 = 8,
    B512 = 9,
    B1024 = 10,
    B2048 = 11,
    B4096 = 12,
    B8192 = 13,
    B16kB = 14,
    Unknown = 15,
}

/// CURRENT_STATE enum. Used for R1 response in command queue mode in SD spec, or all R1 responses
/// in eMMC spec.
///
/// Ref PLSS_v7_10 Table 4-75
/// Ref JESD84-B51 Table 68
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum CurrentState {
    /// Card state is ready
    Ready = 1,
    /// Card is in identification state
    Identification = 2,
    /// Card is in standby state
    Standby = 3,
    /// Card is in transfer state
    Transfer = 4,
    /// Card is sending an operation
    Sending = 5,
    /// Card is receiving operation information
    Receiving = 6,
    /// Card is in programming state
    Programming = 7,
    /// Card is disconnected
    Disconnected = 8,
    /// Card is in bus testing mode. Only valid for eMMC (reserved by SD spec).
    BusTest = 9,
    /// Card is in sleep mode. Only valid for eMMC (reserved by SD spec).
    Sleep = 10,
    // 11 - 15: Reserved
    /// Error
    Error = 128,
}

impl From<u8> for CurrentState {
    fn from(n: u8) -> Self {
        match n {
            1 => Self::Ready,
            2 => Self::Identification,
            3 => Self::Standby,
            4 => Self::Transfer,
            5 => Self::Sending,
            6 => Self::Receiving,
            7 => Self::Programming,
            8 => Self::Disconnected,
            9 => Self::BusTest,
            10 => Self::Sleep,
            _ => Self::Error,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CurrentConsumption {
    I_0mA,
    I_1mA,
    I_5mA,
    I_10mA,
    I_25mA,
    I_35mA,
    I_45mA,
    I_60mA,
    I_80mA,
    I_100mA,
    I_200mA,
}
impl From<&CurrentConsumption> for u32 {
    fn from(i: &CurrentConsumption) -> u32 {
        match i {
            CurrentConsumption::I_0mA => 0,
            CurrentConsumption::I_1mA => 1,
            CurrentConsumption::I_5mA => 5,
            CurrentConsumption::I_10mA => 10,
            CurrentConsumption::I_25mA => 25,
            CurrentConsumption::I_35mA => 35,
            CurrentConsumption::I_45mA => 45,
            CurrentConsumption::I_60mA => 60,
            CurrentConsumption::I_80mA => 80,
            CurrentConsumption::I_100mA => 100,
            CurrentConsumption::I_200mA => 200,
        }
    }
}
impl CurrentConsumption {
    fn from_minimum_reg(reg: u128) -> CurrentConsumption {
        match reg & 0x7 {
            0 => CurrentConsumption::I_0mA,
            1 => CurrentConsumption::I_1mA,
            2 => CurrentConsumption::I_5mA,
            3 => CurrentConsumption::I_10mA,
            4 => CurrentConsumption::I_25mA,
            5 => CurrentConsumption::I_35mA,
            6 => CurrentConsumption::I_60mA,
            _ => CurrentConsumption::I_100mA,
        }
    }
    fn from_maximum_reg(reg: u128) -> CurrentConsumption {
        match reg & 0x7 {
            0 => CurrentConsumption::I_1mA,
            1 => CurrentConsumption::I_5mA,
            2 => CurrentConsumption::I_10mA,
            3 => CurrentConsumption::I_25mA,
            4 => CurrentConsumption::I_35mA,
            5 => CurrentConsumption::I_45mA,
            6 => CurrentConsumption::I_80mA,
            _ => CurrentConsumption::I_200mA,
        }
    }
}
impl fmt::Debug for CurrentConsumption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ma: u32 = self.into();
        write!(f, "{} mA", ma)
    }
}

/// Operation Conditions Register (OCR)
///
/// R3
#[derive(Clone, Copy, Default)]
pub struct OCR<Ext>(pub(crate) u32, PhantomData<Ext>);
impl<Ext> From<u32> for OCR<Ext> {
    fn from(word: u32) -> Self {
        Self(word, PhantomData)
    }
}
impl<Ext> OCR<Ext> {
    /// Card power up status bit (busy)
    pub fn is_busy(&self) -> bool {
        self.0 & 0x8000_0000 == 0 // Set active LOW
    }
}

/// Card Identification Register (CID)
///
/// R2
#[derive(Clone, Copy, Default)]
pub struct CID<Ext> {
    pub(crate) inner: u128,
    pub(crate) bytes: [u8; 16],
    ext: PhantomData<Ext>,
}
impl<Ext> From<u128> for CID<Ext> {
    fn from(inner: u128) -> Self {
        Self {
            inner,
            bytes: inner.to_be_bytes(),
            ext: PhantomData,
        }
    }
}
/// From little endian words
impl<Ext> From<[u32; 4]> for CID<Ext> {
    fn from(words: [u32; 4]) -> Self {
        let inner = ((words[3] as u128) << 96)
            | ((words[2] as u128) << 64)
            | ((words[1] as u128) << 32)
            | words[0] as u128;
        inner.into()
    }
}
impl<Ext> CID<Ext> {
    /// Manufacturer ID
    pub fn manufacturer_id(&self) -> u8 {
        self.bytes[0]
    }
    #[allow(unused)]
    fn crc7(&self) -> u8 {
        (self.bytes[15] >> 1) & 0x7F
    }
}

/// Card Specific Data (CSD)
#[derive(Clone, Copy, Default)]
pub struct CSD<Ext>(pub(crate) u128, PhantomData<Ext>);
impl<Ext> From<u128> for CSD<Ext> {
    fn from(inner: u128) -> Self {
        Self(inner, PhantomData)
    }
}
/// From little endian words
impl<Ext> From<[u32; 4]> for CSD<Ext> {
    fn from(words: [u32; 4]) -> Self {
        let inner = ((words[3] as u128) << 96)
            | ((words[2] as u128) << 64)
            | ((words[1] as u128) << 32)
            | words[0] as u128;
        inner.into()
    }
}

impl<Ext> CSD<Ext> {
    /// CSD structure version
    pub fn version(&self) -> u8 {
        (self.0 >> 126) as u8 & 3
    }
    /// Maximum data transfer rate per one data line
    pub fn transfer_rate(&self) -> u8 {
        (self.0 >> 96) as u8
    }
    /// Maximum block length. In an SD Memory Card the WRITE_BL_LEN is
    /// always equal to READ_BL_LEN
    pub fn block_length(&self) -> BlockSize {
        // Read block length
        match (self.0 >> 80) & 0xF {
            0 => BlockSize::B1,
            1 => BlockSize::B2,
            2 => BlockSize::B4,
            3 => BlockSize::B8,
            4 => BlockSize::B16,
            5 => BlockSize::B32,
            6 => BlockSize::B64,
            7 => BlockSize::B128,
            8 => BlockSize::B256,
            9 => BlockSize::B512,
            10 => BlockSize::B1024,
            11 => BlockSize::B2048,
            12 => BlockSize::B4096,
            13 => BlockSize::B8192,
            14 => BlockSize::B16kB,
            _ => BlockSize::Unknown,
        }
    }
    /// Maximum read current at the minimum VDD
    pub fn read_current_minimum_vdd(&self) -> CurrentConsumption {
        CurrentConsumption::from_minimum_reg((self.0 >> 59) & 0x7)
    }
    /// Maximum write current at the minimum VDD
    pub fn write_current_minimum_vdd(&self) -> CurrentConsumption {
        CurrentConsumption::from_minimum_reg((self.0 >> 56) & 0x7)
    }
    /// Maximum read current at the maximum VDD
    pub fn read_current_maximum_vdd(&self) -> CurrentConsumption {
        CurrentConsumption::from_maximum_reg((self.0 >> 53) & 0x7)
    }
    /// Maximum write current at the maximum VDD
    pub fn write_current_maximum_vdd(&self) -> CurrentConsumption {
        CurrentConsumption::from_maximum_reg((self.0 >> 50) & 0x7)
    }
}

/// Card Status (R1)
///
/// Error and state information of an executed command
///
/// Ref PLSS_v7_10 Section 4.10.1
#[derive(Clone, Copy)]
pub struct CardStatus<Ext>(pub(crate) u32, PhantomData<Ext>);

impl<Ext> From<u32> for CardStatus<Ext> {
    fn from(word: u32) -> Self {
        Self(word, PhantomData)
    }
}

impl<Ext> CardStatus<Ext> {
    /// Command's argument was out of range
    pub fn out_of_range(&self) -> bool {
        self.0 & 0x8000_0000 != 0
    }
    /// Misaligned address
    pub fn address_error(&self) -> bool {
        self.0 & 0x4000_0000 != 0
    }
    /// Block len error
    pub fn block_len_error(&self) -> bool {
        self.0 & 0x2000_0000 != 0
    }
    /// Error in the erase commands sequence
    pub fn erase_seq_error(&self) -> bool {
        self.0 & 0x1000_0000 != 0
    }
    /// Invalid selection of blocks for erase
    pub fn erase_param(&self) -> bool {
        self.0 & 0x800_0000 != 0
    }
    /// Host attempted to write to protected area
    pub fn wp_violation(&self) -> bool {
        self.0 & 0x400_0000 != 0
    }
    /// Card is locked by the host
    pub fn card_is_locked(&self) -> bool {
        self.0 & 0x200_0000 != 0
    }
    /// Password error
    pub fn lock_unlock_failed(&self) -> bool {
        self.0 & 0x100_0000 != 0
    }
    /// Crc check of previous command failed
    pub fn com_crc_error(&self) -> bool {
        self.0 & 0x80_0000 != 0
    }
    /// Command is not legal for the card state
    pub fn illegal_command(&self) -> bool {
        self.0 & 0x40_0000 != 0
    }
    /// Card internal ECC failed
    pub fn card_ecc_failed(&self) -> bool {
        self.0 & 0x20_0000 != 0
    }
    /// Internal controller error
    pub fn cc_error(&self) -> bool {
        self.0 & 0x10_0000 != 0
    }
    /// A General error occurred
    pub fn error(&self) -> bool {
        self.0 & 0x8_0000 != 0
    }
    /// CSD error
    pub fn csd_overwrite(&self) -> bool {
        self.0 & 0x1_0000 != 0
    }
    /// Some blocks where skipped while erasing
    pub fn wp_erase_skip(&self) -> bool {
        self.0 & 0x8000 != 0
    }
    /// Erase sequence was aborted
    pub fn erase_reset(&self) -> bool {
        self.0 & 0x2000 != 0
    }
    /// Current card state
    pub fn state(&self) -> CurrentState {
        CurrentState::from(((self.0 >> 9) & 0xF) as u8)
    }
    /// Corresponds to buffer empty signaling on the bus
    pub fn ready_for_data(&self) -> bool {
        self.0 & 0x100 != 0
    }
    /// The card will accept a ACMD
    pub fn app_cmd(&self) -> bool {
        self.0 & 0x20 != 0
    }
}

/// Relative Card Address (RCA)
///
/// R6
#[derive(Debug, Copy, Clone, Default)]
pub struct RCA<Ext>(pub(crate) u32, PhantomData<Ext>);
impl<Ext> From<u32> for RCA<Ext> {
    fn from(word: u32) -> Self {
        Self(word, PhantomData)
    }
}
impl<Ext> RCA<Ext> {
    /// Address of card
    pub fn address(&self) -> u16 {
        (self.0 >> 16) as u16
    }
}
