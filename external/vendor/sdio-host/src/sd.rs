//! SD-specific extensions to the core SDMMC protocol.

pub use crate::common::*;

/// Type marker for SD-specific extensions.
#[derive(Clone, Copy, Default)]
pub struct SD;

use core::{fmt, str};

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SDSpecVersion {
    /// Version 1.0 and and 1.0.1
    V1_0,
    /// Version 1.10
    V1_10,
    /// Version 2.0
    V2,
    /// Version 3.0
    V3,
    /// Version 4.0
    V4,
    /// Version 5.0
    V5,
    /// Version 6.0
    V6,
    /// Version 7.0
    V7,
    /// Version not known by this crate
    Unknown,
}

/// SD CARD Configuration Register (SCR)
#[derive(Clone, Copy, Default)]
pub struct SCR(pub u64);
/// From little endian words
impl From<[u32; 2]> for SCR {
    fn from(words: [u32; 2]) -> Self {
        Self(((words[1] as u64) << 32) | words[0] as u64)
    }
}
impl SCR {
    /// Physical Layer Specification Version Number
    pub fn version(&self) -> SDSpecVersion {
        let spec = (self.0 >> 56) & 0xF;
        let spec3 = (self.0 >> 47) & 1;
        let spec4 = (self.0 >> 42) & 1;
        let specx = (self.0 >> 38) & 0xF;

        // Ref PLSS_v7_10 Table 5-17
        match (spec, spec3, spec4, specx) {
            (0, 0, 0, 0) => SDSpecVersion::V1_0,
            (1, 0, 0, 0) => SDSpecVersion::V1_10,
            (2, 0, 0, 0) => SDSpecVersion::V2,
            (2, 1, 0, 0) => SDSpecVersion::V3,
            (2, 1, 1, 0) => SDSpecVersion::V4,
            (2, 1, _, 1) => SDSpecVersion::V5,
            (2, 1, _, 2) => SDSpecVersion::V6,
            (2, 1, _, 3) => SDSpecVersion::V7,
            _ => SDSpecVersion::Unknown,
        }
    }
    /// Bus widths supported
    pub fn bus_widths(&self) -> u8 {
        // Ref PLSS_v7_10 Table 5-21
        ((self.0 >> 48) as u8) & 0xF
    }
    /// Supports 1-bit bus width
    pub fn bus_width_one(&self) -> bool {
        (self.0 >> 48) & 1 != 0
    }
    /// Supports 4-bit bus width
    pub fn bus_width_four(&self) -> bool {
        (self.0 >> 50) & 1 != 0
    }
}
impl core::fmt::Debug for SCR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SCR: SD CARD Configuration Register")
            .field("Version", &self.version())
            .field("1-bit width", &self.bus_width_one())
            .field("4-bit width", &self.bus_width_four())
            .finish()
    }
}

impl OCR<SD> {
    /// VDD voltage window.
    // 00000000 00000000 00000000 00000000
    // 00000000 00000000 0
    //          11111111 1
    // OCR [23:15].
    pub fn voltage_window_mv(&self) -> Option<(u16, u16)> {
        let mut window = (self.0 >> 15) & 0x1FF;
        let mut min = 2_700;

        while window & 1 == 0 && window != 0 {
            min += 100;
            window >>= 1;
        }
        let mut max = min;
        while window != 0 {
            max += 100;
            window >>= 1;
        }

        if max == min {
            None
        } else {
            Some((min, max))
        }
    }
    /// Switching to 1.8V Accepted (S18A). Only UHS-I cards support this bit
    // 00000000 00000000 00000000 00000000
    //        1
    // OCR [24].
    pub fn v18_allowed(&self) -> bool {
        self.0 & 0x0100_0000 != 0
    }
    /// Over 2TB support Status. Only SDUC card support this bit
    // 00000000 00000000 00000000 00000000
    //     1
    // OCR [27].
    pub fn over_2tb(&self) -> bool {
        self.0 & 0x0800_0000 != 0
    }
    /// Indicates whether the card supports UHS-II Interface
    // 00000000 00000000 00000000 00000000
    //   1
    // OCR [29].
    pub fn uhs2_card_status(&self) -> bool {
        self.0 & 0x2000_0000 != 0
    }
    /// Card Capacity Status (CCS)
    ///
    /// For SD cards, this is true for SDHC/SDXC/SDUC, false for SDSC
    pub fn high_capacity(&self) -> bool {
        self.0 & 0x4000_0000 != 0
    }
}
impl fmt::Debug for OCR<SD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OCR: Operation Conditions Register")
            .field(
                "Voltage Window (mV)",
                &self.voltage_window_mv().unwrap_or((0, 0)),
            )
            .field("S18A (UHS-I only)", &self.v18_allowed())
            .field("Over 2TB flag (SDUC only)", &self.over_2tb())
            .field("UHS-II Card", &self.uhs2_card_status())
            .field(
                "Card Capacity Status (CSS)",
                &if self.high_capacity() {
                    "SDHC/SDXC/SDUC"
                } else {
                    "SDSC"
                },
            )
            .field("Busy", &self.is_busy())
            .finish()
    }
}

impl CID<SD> {
    /// OEM/Application ID
    pub fn oem_id(&self) -> &str {
        str::from_utf8(&self.bytes[1..3]).unwrap_or(&"<ERR>")
    }
    /// Product name
    pub fn product_name(&self) -> &str {
        str::from_utf8(&self.bytes[3..8]).unwrap_or(&"<ERR>")
    }
    /// Product revision
    pub fn product_revision(&self) -> u8 {
        self.bytes[8]
    }
    /// Product serial number
    pub fn serial(&self) -> u32 {
        (self.inner >> 24) as u32
    }
    /// Manufacturing date
    pub fn manufacturing_date(&self) -> (u8, u16) {
        (
            (self.inner >> 8) as u8 & 0xF,             // Month
            ((self.inner >> 12) as u16 & 0xFF) + 2000, // Year
        )
    }
}

impl fmt::Debug for CID<SD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CID: Card Identification")
            .field("Manufacturer ID", &self.manufacturer_id())
            .field("OEM ID", &self.oem_id())
            .field("Product Name", &self.product_name())
            .field("Product Revision", &self.product_revision())
            .field("Product Serial Number", &self.serial())
            .field("Manufacturing Date", &self.manufacturing_date())
            .finish()
    }
}

impl CSD<SD> {
    /// Number of blocks in the card
    pub fn block_count(&self) -> u64 {
        match self.version() {
            0 => {
                // SDSC
                let c_size: u16 = ((self.0 >> 62) as u16) & 0xFFF;
                let c_size_mult: u8 = ((self.0 >> 47) as u8) & 7;

                ((c_size + 1) as u64) * ((1 << (c_size_mult + 2)) as u64)
            }
            1 => {
                // SDHC/SDXC
                (((self.0 >> 48) as u64 & 0x3F_FFFF) + 1) * 1024
            }
            2 => {
                // SDUC
                (((self.0 >> 48) as u64 & 0xFFF_FFFF) + 1) * 1024
            }
            _ => 0,
        }
    }
    /// Card size in bytes
    pub fn card_size(&self) -> u64 {
        let block_size_bytes = 1 << self.block_length() as u64;

        self.block_count() * block_size_bytes
    }
    /// Erase size (in blocks)
    pub fn erase_size_blocks(&self) -> u32 {
        if (self.0 >> 46) & 1 == 1 {
            // ERASE_BLK_EN
            1
        } else {
            let sector_size_tens = (self.0 >> 43) & 0x7;
            let sector_size_units = (self.0 >> 39) & 0xF;

            (sector_size_tens as u32 * 10) + (sector_size_units as u32)
        }
    }
}

impl fmt::Debug for CSD<SD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CSD: Card Specific Data")
            .field("Transfer Rate", &self.transfer_rate())
            .field("Block Count", &self.block_count())
            .field("Card Size (bytes)", &self.card_size())
            .field("Read I (@min VDD)", &self.read_current_minimum_vdd())
            .field("Write I (@min VDD)", &self.write_current_minimum_vdd())
            .field("Read I (@max VDD)", &self.read_current_maximum_vdd())
            .field("Write I (@max VDD)", &self.write_current_maximum_vdd())
            .field("Erase Size (Blocks)", &self.erase_size_blocks())
            .finish()
    }
}

impl CardStatus<SD> {
    /// Command was executed without internal ECC
    pub fn ecc_disabled(&self) -> bool {
        self.0 & 0x4000 != 0
    }
    /// Extension function specific status
    pub fn fx_event(&self) -> bool {
        self.0 & 0x40 != 0
    }
    /// Authentication sequence error
    pub fn ake_seq_error(&self) -> bool {
        self.0 & 0x8 != 0
    }
}

impl fmt::Debug for CardStatus<SD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Card Status")
            .field("Out of range error", &self.out_of_range())
            .field("Address error", &self.address_error())
            .field("Block len error", &self.block_len_error())
            .field("Erase seq error", &self.erase_seq_error())
            .field("Erase param error", &self.erase_param())
            .field("Write protect error", &self.wp_violation())
            .field("Card locked", &self.card_is_locked())
            .field("Password lock unlock error", &self.lock_unlock_failed())
            .field(
                "Crc check for the previous command failed",
                &self.com_crc_error(),
            )
            .field("Illegal command", &self.illegal_command())
            .field("Card internal ecc failed", &self.card_ecc_failed())
            .field("Internal card controller error", &self.cc_error())
            .field("General Error", &self.error())
            .field("Csd error", &self.csd_overwrite())
            .field("Write protect error", &self.wp_erase_skip())
            .field("Command ecc disabled", &self.ecc_disabled())
            .field("Erase sequence cleared", &self.erase_reset())
            .field("Card state", &self.state())
            .field("Buffer empty", &self.ready_for_data())
            .field("Extension event", &self.fx_event())
            .field("Card expects app cmd", &self.app_cmd())
            .field("Auth process error", &self.ake_seq_error())
            .finish()
    }
}

/// SD Status
///
/// Status bits related to SD Memory Card proprietary features
///
/// Ref PLSS_v7_10 Section 4.10.2 SD Status
#[derive(Clone, Copy, Default)]
pub struct SDStatus {
    inner: [u32; 16],
}
/// From little endian words
impl From<[u32; 16]> for SDStatus {
    fn from(inner: [u32; 16]) -> Self {
        Self { inner }
    }
}
impl SDStatus {
    /// Current data bus width
    pub fn bus_width(&self) -> BusWidth {
        match (self.inner[15] >> 30) & 3 {
            0 => BusWidth::One,
            2 => BusWidth::Four,
            _ => BusWidth::Unknown,
        }
    }
    /// Is the card currently in the secured mode
    pub fn secure_mode(&self) -> bool {
        self.inner[15] & 0x2000_0000 != 0
    }
    /// SD Memory Card type (ROM, OTP, etc)
    pub fn sd_memory_card_type(&self) -> u16 {
        self.inner[15] as u16
    }
    /// SDHC / SDXC: Capacity of Protected Area in bytes
    pub fn protected_area_size(&self) -> u32 {
        self.inner[14]
    }
    /// Speed Class
    pub fn speed_class(&self) -> u8 {
        (self.inner[13] >> 24) as u8
    }
    /// "Performance Move" indicator in 1 MB/s units
    pub fn move_performance(&self) -> u8 {
        (self.inner[13] >> 16) as u8
    }
    /// Allocation Unit (AU) size. Lookup in PLSS v7_10 Table 4-47
    pub fn allocation_unit_size(&self) -> u8 {
        (self.inner[13] >> 12) as u8 & 0xF
    }
    /// Indicates N_Erase, in units of AU
    pub fn erase_size(&self) -> u16 {
        (self.inner[13] & 0xFF) as u16 | ((self.inner[12] >> 24) & 0xFF) as u16
    }
    /// Indicates T_Erase / Erase Timeout (s)
    pub fn erase_timeout(&self) -> u8 {
        (self.inner[12] >> 18) as u8 & 0x3F
    }
    /// Video speed class
    pub fn video_speed_class(&self) -> u8 {
        (self.inner[11] & 0xFF) as u8
    }
    /// Application Performance Class
    pub fn app_perf_class(&self) -> u8 {
        (self.inner[9] >> 16) as u8 & 0xF
    }
    /// Discard Support
    pub fn discard_support(&self) -> bool {
        self.inner[8] & 0x0200_0000 != 0
    }
}
impl fmt::Debug for SDStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SD Status")
            .field("Bus Width", &self.bus_width())
            .field("Secured Mode", &self.secure_mode())
            .field("SD Memory Card Type", &self.sd_memory_card_type())
            .field("Protected Area Size (B)", &self.protected_area_size())
            .field("Speed Class", &self.speed_class())
            .field("Video Speed Class", &self.video_speed_class())
            .field("Application Performance Class", &self.app_perf_class())
            .field("Move Performance (MB/s)", &self.move_performance())
            .field("AU Size", &self.allocation_unit_size())
            .field("Erase Size (units of AU)", &self.erase_size())
            .field("Erase Timeout (s)", &self.erase_timeout())
            .field("Discard Support", &self.discard_support())
            .finish()
    }
}

/// Card interface condition (R7)
#[derive(Copy, Clone, Default)]
pub struct CIC(u32);
impl From<u32> for CIC {
    fn from(word: u32) -> Self {
        Self(word)
    }
}
impl CIC {
    /// The voltage range the card accepts
    pub fn voltage_accepted(&self) -> u8 {
        (self.0 >> 8) as u8
    }
    /// Echo-back check pattern
    pub fn pattern(&self) -> u8 {
        self.0 as u8
    }
}

impl RCA<SD> {
    /// Status
    pub fn status(&self) -> u16 {
        self.0 as u16
    }
}
