//! eMMC-specific extensions to the core SDMMC protocol.

pub use crate::common::*;

use core::{fmt, str};

/// Type marker for eMMC-specific extensions.
#[derive(Clone, Copy, Default, Debug)]
pub struct EMMC;

impl OCR<EMMC> {
    /// OCR \[7\]. False for High Voltage, true for Dual voltage
    pub fn is_dual_voltage_card(&self) -> bool {
        self.0 & 0x0000_0080 != 0
    }
    /// OCR \[30:29\]. Access mode. Defines the addressing mode used between host and card
    ///
    /// 0b00: byte mode
    /// 0b10: sector mode
    pub fn access_mode(&self) -> u8 {
        (self.0 & 0x6000_0000 >> 29) as u8
    }
}
impl fmt::Debug for OCR<EMMC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OCR: Operation Conditions Register")
            .field(
                "Dual Voltage",
                &if self.is_dual_voltage_card() {
                    "yes"
                } else {
                    "no"
                },
            )
            .field(
                "Access mode",
                &match self.access_mode() {
                    0b00 => "byte",
                    0b10 => "sector",
                    _ => "unknown",
                },
            )
            .field("Busy", &self.is_busy())
            .finish()
    }
}

/// All possible values of the CBX field of the CID register on eMMC devices.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeviceType {
    RemovableDevice = 0b00,
    BGA = 0b01,
    POP = 0b10,
    Unknown = 0b11,
}

impl CID<EMMC> {
    /// CBX field, indicating device type.
    pub fn device_type(&self) -> DeviceType {
        match self.bytes[1] & 0x3 {
            0b00 => DeviceType::RemovableDevice,
            0b01 => DeviceType::BGA,
            0b10 => DeviceType::POP,
            _ => DeviceType::POP,
        }
    }

    /// OID field, indicating OEM/Application ID.
    ///
    /// The OID number is controlled, defined and allocated to an eMMC manufacturer by JEDEC.
    pub fn oem_application_id(&self) -> u8 {
        self.bytes[2]
    }

    /// PNM field, indicating product name.
    pub fn product_name(&self) -> &str {
        str::from_utf8(&self.bytes[3..9]).unwrap_or(&"<ERR>")
    }

    /// PRV field, indicating product revision.
    ///
    /// The return value is a (major, minor) version tuple.
    pub fn product_revision(&self) -> (u8, u8) {
        let major = (self.bytes[9] & 0xF0) >> 4;
        let minor = self.bytes[9] & 0x0F;
        (major, minor)
    }

    /// PSN field, indicating product serial number.
    pub fn serial(&self) -> u32 {
        (self.inner >> 16) as u32
    }

    /// MDT field, indicating manufacturing date.
    ///
    /// The return value is a (month, year) tuple where the month code has 1 = January and the year
    /// is an offset from either 1997 or 2013 depending on the value of `EXT_CSD_REV`.
    pub fn manufacturing_date(&self) -> (u8, u8) {
        let month = (self.inner >> 12) as u8 & 0xF;
        let year = (self.inner >> 8) as u8 & 0xF;
        (month, year)
    }
}
impl fmt::Debug for CID<EMMC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CID: Card Identification")
            .field("Manufacturer ID", &self.manufacturer_id())
            .field("Device Type", &self.device_type())
            .field("OEM ID", &self.oem_application_id())
            .field("Product Name", &self.product_name())
            .field("Product Revision", &self.product_revision())
            .field("Product Serial Number", &self.serial())
            .field("Manufacturing Date", &self.manufacturing_date())
            .finish()
    }
}

impl CSD<EMMC> {
    /// Erase size (in blocks)
    ///
    /// Minimum number of write blocks that must be erased in a single erase
    /// command
    pub fn erase_size_blocks(&self) -> u32 {
        let erase_grp_size = (self.0 >> 42) & 0x1F;
        let erase_grp_mult = (self.0 >> 37) & 0x1F;

        (erase_grp_size as u32 + 1) + (erase_grp_mult as u32 + 1)
    }
}
impl fmt::Debug for CSD<EMMC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CSD: Card Specific Data")
            .field("Transfer Rate", &self.transfer_rate())
            .field("Read I (@min VDD)", &self.read_current_minimum_vdd())
            .field("Write I (@min VDD)", &self.write_current_minimum_vdd())
            .field("Read I (@max VDD)", &self.read_current_maximum_vdd())
            .field("Write I (@max VDD)", &self.write_current_maximum_vdd())
            .field("Erase Size (Blocks)", &self.erase_size_blocks())
            .finish()
    }
}

impl CardStatus<EMMC> {
    /// If set, the Device did not switch to the expected mode as requested by the SWITCH command
    pub fn switch_error(&self) -> bool {
        self.0 & 0x80 != 0
    }
    /// If set, one of the exception bits in field EXCEPTION_EVENTS_STATUS was set to indicate some
    /// exception has occurred. Host should check that field to discover the exception that has
    /// occurred to understand what further actions are needed in order to clear this bit.
    pub fn exception_event(&self) -> bool {
        self.0 & 0x40 != 0
    }
}
impl fmt::Debug for CardStatus<EMMC> {
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
            .field("Erase sequence cleared", &self.erase_reset())
            .field("Card state", &self.state())
            .field("Buffer empty", &self.ready_for_data())
            .field("Switch error", &self.switch_error())
            .field("Exception event", &self.exception_event())
            .field("Card expects app cmd", &self.app_cmd())
            .finish()
    }
}

/// Extended Card Specific Data
///
/// Ref JEDEC 84-A43 Section 8.4
#[derive(Clone, Copy)]
pub struct ExtCSD {
    pub inner: [u32; 128],
}
impl Default for ExtCSD {
    fn default() -> ExtCSD {
        ExtCSD { inner: [0; 128] }
    }
}
/// From little endian words
impl From<[u32; 128]> for ExtCSD {
    fn from(inner: [u32; 128]) -> Self {
        Self { inner }
    }
}
impl ExtCSD {
    pub fn boot_info(&self) -> u8 {
        // byte 228
        (self.inner[57] >> 24) as u8
    }
    pub fn sleep_awake_timeout(&self) -> u8 {
        // byte 217
        (self.inner[54] >> 16) as u8
    }
    pub fn sleep_notification_time(&self) -> u8 {
        // byte 216
        (self.inner[54] >> 24) as u8
    }
    pub fn sector_count(&self) -> u32 {
        // bytes [215:212]
        self.inner[53]
    }
    pub fn driver_strength(&self) -> u8 {
        // byte 197
        (self.inner[49] >> 16) as u8
    }
    pub fn card_type(&self) -> u8 {
        // byte 196
        (self.inner[49] >> 24) as u8
    }
    pub fn csd_structure_version(&self) -> u8 {
        // byte 194
        (self.inner[48] >> 8) as u8
    }
    pub fn extended_csd_revision(&self) -> u8 {
        // byte 192
        (self.inner[48] >> 24) as u8
    }
    pub fn data_sector_size(&self) -> u8 {
        // byte 61
        (self.inner[15] >> 16) as u8
    }
    pub fn secure_removal_type(&self) -> u8 {
        // byte 16
        (self.inner[4] >> 24) as u8
    }
}
impl fmt::Debug for ExtCSD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extended CSD")
            .field("Boot Info", &self.boot_info())
            .field("Sleep/Awake Timeout", &self.sleep_awake_timeout())
            .field("Sleep Notification Time", &self.sleep_notification_time())
            .field("Sector Count", &self.sector_count())
            .field("Driver Strength", &self.driver_strength())
            .field("Card Type", &self.card_type())
            .field("CSD Structure Version", &self.csd_structure_version())
            .field("Extended CSD Revision", &self.extended_csd_revision())
            .field("Sector Size", &self.data_sector_size())
            .field("Secure removal type", &self.secure_removal_type())
            .finish()
    }
}

/// eMMC hosts need to be able to create relative card addresses so that they can be assigned to
/// devices. SD hosts only ever retrieve RCAs from 32-bit card responses.
impl From<u16> for RCA<EMMC> {
    fn from(address: u16) -> Self {
        Self::from((address as u32) << 16)
    }
}
