//! eMMC-specific command definitions.

use crate::common_cmd::{cmd, Cmd, R1, R3};

/// CMD1: Ask all cards to send their supported OCR, or become inactive if they cannot be
/// supported.
pub fn send_op_cond(ocr: u32) -> Cmd<R3> {
    cmd(1, ocr)
}

/// CMD3: Assigns relative address (RCA) to the Device
pub fn assign_relative_address(address: u16) -> Cmd<R1> {
    cmd(3, (address as u32) << 16)
}

/// Specifies a method of modifying a field of EXT_CSD. Used for CMD6.
pub enum AccessMode {
    // The 0b00 pattern corresponds to Command Set, which has different semantics.
    SetBits = 0b01,
    ClearBits = 0b10,
    WriteByte = 0b11,
}

/// Uses CMD6 to modify a field of the EXT_CSD.
pub fn modify_ext_csd(access_mode: AccessMode, index: u8, value: u8) -> Cmd<R1> {
    let arg = 0u32 | ((access_mode as u32) << 24) | ((index as u32) << 16) | ((value as u32) << 8);
    cmd(6, arg)
}

/// CMD8: Device sends its EXT_CSD register as a block of data.
pub fn send_ext_csd() -> Cmd<R1> {
    cmd(8, 0)
}

/// CMD14: Host reads the reversed bus testing data pattern from a card
pub fn bustest_read() -> Cmd<R1> {
    cmd(14, 0)
}

/// CMD19: Host sends bus test pattern to a card
pub fn bustest_write() -> Cmd<R1> {
    cmd(19, 0)
}

/// CMD23: Defines the number of blocks (read/write) for a block read or write
/// operation
pub fn set_block_count(blockcount: u16) -> Cmd<R1> {
    cmd(23, blockcount as u32)
}

/// CMD35: Sets the address of the first erase group within a range to be
/// selected for erase
///
/// Address is either byte address or sector address (set in OCR)
pub fn erase_group_start(address: u32) -> Cmd<R1> {
    cmd(35, address)
}

/// CMD36: Sets the address of the last erase group within a continuous range to
/// be selected for erase
///
/// Address is either byte address or sector address (set in OCR)
pub fn erase_group_end(address: u32) -> Cmd<R1> {
    cmd(36, address)
}
