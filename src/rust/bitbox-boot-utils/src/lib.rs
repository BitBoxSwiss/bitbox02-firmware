// SPDX-License-Identifier: Apache-2.0

#![no_std]

mod image_header;

use core::fmt;
use cortex_m::peripheral::SCB;
pub use image_header::{
    IMAGE_HEADER_LEN, IMAGE_HEADER_MAGIC_BOOT1, IMAGE_HEADER_MAGIC_FIRMWARE, ImageHeader,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum BootCommand {
    // Random 32-bit command tags make false positives on cold boot RAM
    // overwhelmingly unlikely.
    FactoryReset = 0x5c3a_f91e,
    BootloaderWait = 0xb2f1_5d4c,
    UpgradeBoot1 = 0x93d2_4a70,
}

const BOOT1_HASH_LEN: usize = 32;
const BOOT1_MANIFEST_MAGIC: [u8; 4] = *b"BBB1";

pub struct HexBytes<'a>(pub &'a [u8]);

impl fmt::Display for HexBytes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Boot1Manifest {
    pub magic: [u8; 4],
    pub source_address: u32,
    pub image_len: u32,
    pub bootloader_hash: [u8; BOOT1_HASH_LEN],
}

impl Boot1Manifest {
    pub const LEN: usize = 4 + 4 + 4 + BOOT1_HASH_LEN;

    pub const fn new(
        source_address: u32,
        image_len: u32,
        bootloader_hash: [u8; BOOT1_HASH_LEN],
    ) -> Self {
        Self {
            magic: BOOT1_MANIFEST_MAGIC,
            source_address,
            image_len,
            bootloader_hash,
        }
    }

    pub fn to_bytes(&self) -> [u8; Self::LEN] {
        let mut out = [0u8; Self::LEN];
        out[..4].copy_from_slice(&self.magic);
        out[4..8].copy_from_slice(&self.source_address.to_le_bytes());
        out[8..12].copy_from_slice(&self.image_len.to_le_bytes());
        out[12..].copy_from_slice(&self.bootloader_hash);
        out
    }

    pub fn from_bytes(bytes: &[u8; Self::LEN]) -> Result<Self, ()> {
        let magic: [u8; 4] = bytes[..4].try_into().unwrap();
        if magic != BOOT1_MANIFEST_MAGIC {
            return Err(());
        }
        let source_address = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let image_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let bootloader_hash = bytes[12..].try_into().unwrap();
        Ok(Self {
            magic,
            source_address,
            image_len,
            bootloader_hash,
        })
    }
}

#[repr(C)]
struct BootArgsData {
    command: u32,
    upgrade_boot1_hash: [u8; BOOT1_HASH_LEN],
}

#[repr(C)]
struct BootArgsStorage {
    data: BootArgsData,
    // Pad the object to the full BOOT_ARGS region so nothing else can be
    // placed in the boot_arg handoff area.
    _padding: [u8; BOOT_ARGS_LEN - core::mem::size_of::<BootArgsData>()],
}

// Reserve a fixed 512-byte handoff area so bootloader and firmware can extend
// the boot arguments format later without changing the RAM layout again.
const BOOT_ARGS_LEN: usize = 512;
const _: [(); BOOT_ARGS_LEN] = [(); core::mem::size_of::<BootArgsStorage>()];

#[used]
#[cfg_attr(
    all(target_arch = "arm", target_os = "none"),
    unsafe(link_section = ".boot_arg")
)]
static mut RAW_BOOT_ARGS: BootArgsStorage = BootArgsStorage {
    data: BootArgsData {
        command: 0,
        upgrade_boot1_hash: [0; BOOT1_HASH_LEN],
    },
    _padding: [0; BOOT_ARGS_LEN - core::mem::size_of::<BootArgsData>()],
};

pub struct BootArgs;

impl BootArgs {
    pub const LEN: usize = BOOT_ARGS_LEN;

    fn raw(&self) -> *mut BootArgsData {
        unsafe { &raw mut RAW_BOOT_ARGS.data }
    }

    pub fn command(&self) -> Option<BootCommand> {
        let command = unsafe { core::ptr::read_volatile(&(*self.raw()).command) };
        match command {
            x if x == BootCommand::FactoryReset as u32 => Some(BootCommand::FactoryReset),
            x if x == BootCommand::BootloaderWait as u32 => Some(BootCommand::BootloaderWait),
            x if x == BootCommand::UpgradeBoot1 as u32 => Some(BootCommand::UpgradeBoot1),
            _ => None,
        }
    }

    pub fn upgrade_boot1_hash(&self) -> Option<[u8; BOOT1_HASH_LEN]> {
        match self.command() {
            Some(BootCommand::UpgradeBoot1) => {
                Some(unsafe { core::ptr::read_volatile(&(*self.raw()).upgrade_boot1_hash) })
            }
            _ => None,
        }
    }

    pub fn set_factory_reset(&self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.raw()).upgrade_boot1_hash, [0; BOOT1_HASH_LEN]);
            core::ptr::write_volatile(&mut (*self.raw()).command, BootCommand::FactoryReset as u32);
        }
    }

    pub fn set_bootloader_wait(&self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.raw()).upgrade_boot1_hash, [0; BOOT1_HASH_LEN]);
            core::ptr::write_volatile(
                &mut (*self.raw()).command,
                BootCommand::BootloaderWait as u32,
            );
        }
    }

    pub fn set_upgrade_boot1(&self, hash: &[u8; BOOT1_HASH_LEN]) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.raw()).upgrade_boot1_hash, *hash);
            core::ptr::write_volatile(&mut (*self.raw()).command, BootCommand::UpgradeBoot1 as u32);
        }
    }

    pub fn clear(&self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.raw()).upgrade_boot1_hash, [0; BOOT1_HASH_LEN]);
            core::ptr::write_volatile(&mut (*self.raw()).command, 0);
        }
    }
}

pub static BOOT_ARGS: BootArgs = BootArgs;

const NVIC_ICER_BASE: *mut u32 = 0xE000_E180 as *mut u32;
const NVIC_ICPR_BASE: *mut u32 = 0xE000_E280 as *mut u32;
const NVIC_REGISTERS: usize = 8;

pub fn halt() -> ! {
    cortex_m::asm::bkpt();
    loop {
        cortex_m::asm::wfe();
    }
}

pub fn vector_table_from_image_header(
    slot_address: usize,
    slot_len: usize,
    expected_magic: [u8; 4],
) -> Result<*const u32, ()> {
    let bytes = unsafe { &*(slot_address as *const [u8; ImageHeader::LEN]) };
    let header = ImageHeader::from_bytes(bytes);

    if header.magic != expected_magic {
        return Err(());
    }
    let header_len = header.header_len as usize;
    if header_len != IMAGE_HEADER_LEN || !header_len.is_multiple_of(4) {
        return Err(());
    }
    if header_len >= slot_len {
        return Err(());
    }

    Ok((slot_address + header_len) as *const u32)
}

fn clear_nvic_state() {
    for index in 0..NVIC_REGISTERS {
        unsafe {
            core::ptr::write_volatile(NVIC_ICER_BASE.add(index), u32::MAX);
            core::ptr::write_volatile(NVIC_ICPR_BASE.add(index), u32::MAX);
        }
    }
}

/// Boot from the provided vector table.
///
/// # Safety
///
/// `vector_table` must point to a valid Cortex-M vector table for executable
/// code. The stack pointer and reset vector entries must be valid for the
/// target image.
pub unsafe fn bootload(vector_table: *const u32) -> ! {
    cortex_m::interrupt::disable();
    clear_nvic_state();
    cortex_m::asm::dsb();
    cortex_m::asm::isb();

    unsafe {
        (*SCB::PTR).vtor.write(vector_table as u32);
    }
    cortex_m::asm::dsb();
    cortex_m::asm::isb();

    let reset_vector = unsafe { core::ptr::read_volatile(vector_table.add(1)) };
    if reset_vector == u32::MAX {
        halt();
    }

    unsafe {
        cortex_m::interrupt::enable();
        cortex_m::asm::bootload(vector_table);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_image_header(magic: [u8; 4], code_size: u32) -> [u8; IMAGE_HEADER_LEN] {
        let mut header = [0u8; IMAGE_HEADER_LEN];
        header[..4].copy_from_slice(&magic);
        header[4..8].copy_from_slice(&(IMAGE_HEADER_LEN as u32).to_le_bytes());
        header[8..12].copy_from_slice(&code_size.to_le_bytes());
        header
    }

    #[test]
    fn test_boot1_manifest_to_bytes() {
        let manifest = Boot1Manifest::new(0x0810_0000, 0x4000, [0x5a; BOOT1_HASH_LEN]);
        let bytes = manifest.to_bytes();

        assert_eq!(&bytes[..4], b"BBB1");
        assert_eq!(&bytes[4..8], &0x0810_0000u32.to_le_bytes());
        assert_eq!(&bytes[8..12], &0x4000u32.to_le_bytes());
        assert_eq!(&bytes[12..], &[0x5a; BOOT1_HASH_LEN]);
    }

    #[test]
    fn test_boot1_manifest_from_bytes() {
        let bytes = [
            b'B', b'B', b'B', b'1', 0x78, 0x56, 0x34, 0x12, 0xef, 0xcd, 0xab, 0x00, 0xaa, 0xaa,
            0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
            0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
            0xaa, 0xaa,
        ];

        let manifest = Boot1Manifest::from_bytes(&bytes).unwrap();

        assert_eq!(manifest.magic, *b"BBB1");
        assert_eq!(manifest.source_address, 0x1234_5678);
        assert_eq!(manifest.image_len, 0x00ab_cdef);
        assert_eq!(manifest.bootloader_hash, [0xaa; BOOT1_HASH_LEN]);
    }

    #[test]
    fn test_boot1_manifest_from_bytes_invalid_magic() {
        let mut bytes = Boot1Manifest::new(0x0810_0000, 0x4000, [0; BOOT1_HASH_LEN]).to_bytes();
        bytes[0] = b'X';

        assert_eq!(Boot1Manifest::from_bytes(&bytes), Err(()));
    }

    #[test]
    fn test_boot_args_commands() {
        BOOT_ARGS.clear();
        assert_eq!(BOOT_ARGS.command(), None);
        assert_eq!(BOOT_ARGS.upgrade_boot1_hash(), None);

        BOOT_ARGS.set_factory_reset();
        assert_eq!(BOOT_ARGS.command(), Some(BootCommand::FactoryReset));
        assert_eq!(BOOT_ARGS.upgrade_boot1_hash(), None);

        BOOT_ARGS.set_bootloader_wait();
        assert_eq!(BOOT_ARGS.command(), Some(BootCommand::BootloaderWait));
        assert_eq!(BOOT_ARGS.upgrade_boot1_hash(), None);

        let expected_hash = [0x42; BOOT1_HASH_LEN];
        BOOT_ARGS.set_upgrade_boot1(&expected_hash);
        assert_eq!(BOOT_ARGS.command(), Some(BootCommand::UpgradeBoot1));
        assert_eq!(BOOT_ARGS.upgrade_boot1_hash(), Some(expected_hash));

        BOOT_ARGS.clear();
        assert_eq!(BOOT_ARGS.command(), None);
        assert_eq!(BOOT_ARGS.upgrade_boot1_hash(), None);
    }

    #[test]
    fn test_image_header_from_bytes() {
        let bytes = [
            b'B', b'B', b'F', b'W', 0x00, 0x04, 0x00, 0x00, 0x34, 0x12, 0x00, 0x00,
        ];
        let header = ImageHeader::from_bytes(&bytes);

        assert_eq!(ImageHeader::LEN, 12);
        assert_eq!(header.magic, IMAGE_HEADER_MAGIC_FIRMWARE);
        assert_eq!(header.header_len, IMAGE_HEADER_LEN as u32);
        assert_eq!(header.code_size, 0x1234);
    }

    #[test]
    fn test_vector_table_from_image_header() {
        let bytes = build_test_image_header(IMAGE_HEADER_MAGIC_BOOT1, 0x1234);
        let slot = bytes.as_ptr() as usize;

        let vector_table =
            vector_table_from_image_header(slot, 2048, IMAGE_HEADER_MAGIC_BOOT1).unwrap();

        assert_eq!(vector_table as usize, slot + IMAGE_HEADER_LEN);
    }
}
