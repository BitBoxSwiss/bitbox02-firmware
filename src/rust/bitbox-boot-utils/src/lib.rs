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

/// Return the next stage vector table after validating its image header.
///
/// The image header must have the expected magic, a sane header length, and a
/// payload that fits into the slot. The vector table is also checked before it
/// is returned.
pub fn vector_table_from_image_header(
    slot_address: usize,
    slot_len: usize,
    ram_base: usize,
    ram_len: usize,
    expected_magic: [u8; 4],
) -> Result<*const u32, ()> {
    let bytes = unsafe { &*(slot_address as *const [u8; ImageHeader::LEN]) };
    let header = ImageHeader::try_from_bytes(bytes)?;

    if header.magic != expected_magic {
        return Err(());
    }
    let header_len = header.header_len as usize;
    if header_len < ImageHeader::LEN || !header_len.is_multiple_of(4) {
        return Err(());
    }
    let code_len = header.code_size as usize;
    if code_len < 8 {
        return Err(());
    }

    let image_len = header_len.checked_add(code_len).ok_or(())?;
    if image_len > slot_len {
        return Err(());
    }

    let vector_table = slot_address.checked_add(header_len).ok_or(())? as *const u32;
    if !unsafe { vector_table_is_valid(vector_table, slot_address, slot_len, ram_base, ram_len) } {
        return Err(());
    }

    Ok(vector_table)
}

/// Check whether a vector table is valid enough to try booting the next stage.
///
/// This rejects erased images and ensures that the initial stack pointer is
/// aligned and points into the assigned RAM area, and that the reset vector is
/// a Thumb address in the assigned flash area. The memory bounds must come from
/// the bootloader's memory layout rather than from the image being validated.
///
/// # Safety
///
/// `vector_table` must point to readable memory containing at least the first
/// two Cortex-M vector table entries.
pub unsafe fn vector_table_is_valid(
    vector_table: *const u32,
    flash_base: usize,
    flash_len: usize,
    ram_base: usize,
    ram_len: usize,
) -> bool {
    let initial_stack_pointer = unsafe { core::ptr::read_volatile(vector_table) };
    let reset_vector = unsafe { core::ptr::read_volatile(vector_table.add(1)) };
    if initial_stack_pointer == u32::MAX || reset_vector == u32::MAX {
        return false;
    }
    let Some(flash_end) = flash_base.checked_add(flash_len) else {
        return false;
    };
    let Some(ram_end) = ram_base.checked_add(ram_len) else {
        return false;
    };
    // The stack grows down, so its initial value may be the address immediately
    // after RAM, but it must be above the start of RAM to have usable space.
    let initial_stack_pointer = initial_stack_pointer as usize;
    if !initial_stack_pointer.is_multiple_of(8)
        || initial_stack_pointer <= ram_base
        || initial_stack_pointer > ram_end
    {
        return false;
    }
    // Cortex-M executes Thumb instructions. Bit 0 of an exception vector must
    // therefore be set; the remaining bits form a halfword-aligned address.
    if reset_vector & 1 == 0 {
        return false;
    }
    let reset_address = (reset_vector & !1) as usize;
    reset_address >= flash_base && reset_address < flash_end
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

    const TEST_FLASH_BASE: usize = 0x0800_0000;
    const TEST_FLASH_LEN: usize = 0x200;
    const TEST_RAM_BASE: usize = 0x2000_0000;
    const TEST_RAM_LEN: usize = 0x200;
    const TEST_INITIAL_STACK_POINTER: u32 = (TEST_RAM_BASE + TEST_RAM_LEN) as u32;
    const TEST_IMAGE_LEN: usize = ImageHeader::LEN + 8;

    #[repr(align(4))]
    struct TestImage<const N: usize>([u8; N]);

    unsafe fn test_vector_table(vector_table: *const u32) -> bool {
        unsafe {
            vector_table_is_valid(
                vector_table,
                TEST_FLASH_BASE,
                TEST_FLASH_LEN,
                TEST_RAM_BASE,
                TEST_RAM_LEN,
            )
        }
    }

    fn build_test_image_header(
        magic: [u8; 4],
        header_len: u32,
        code_size: u32,
    ) -> [u8; ImageHeader::LEN] {
        let mut header = [0u8; ImageHeader::LEN];
        header[..4].copy_from_slice(&magic);
        header[4..8].copy_from_slice(&header_len.to_le_bytes());
        header[8..12].copy_from_slice(&code_size.to_le_bytes());
        header
    }

    fn build_test_image<const N: usize>(
        magic: [u8; 4],
        header_len: u32,
        code_size: u32,
    ) -> TestImage<N> {
        assert!(N >= ImageHeader::LEN);
        let mut image = TestImage([0u8; N]);
        let header = build_test_image_header(magic, header_len, code_size);
        image.0[..ImageHeader::LEN].copy_from_slice(&header);
        image
    }

    fn assert_test_image_invalid(magic: [u8; 4], header_len: u32, code_size: u32, slot_len: usize) {
        let image = build_test_image::<TEST_IMAGE_LEN>(magic, header_len, code_size);
        assert_eq!(
            vector_table_from_image_header(
                image.0.as_ptr() as usize,
                slot_len,
                TEST_RAM_BASE,
                TEST_RAM_LEN,
                IMAGE_HEADER_MAGIC_BOOT1,
            ),
            Err(())
        );
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
    fn test_image_header_try_from_bytes() {
        let bytes = [
            b'B', b'B', b'F', b'W', 0x00, 0x04, 0x00, 0x00, 0x34, 0x12, 0x00, 0x00,
        ];
        let header = ImageHeader::try_from_bytes(&bytes).unwrap();

        assert_eq!(ImageHeader::LEN, 12);
        assert_eq!(header.magic, IMAGE_HEADER_MAGIC_FIRMWARE);
        assert_eq!(header.header_len, IMAGE_HEADER_LEN as u32);
        assert_eq!(header.code_size, 0x1234);
    }

    #[test]
    fn test_image_header_try_from_bytes_invalid_len() {
        assert_eq!(
            ImageHeader::try_from_bytes(&[0; ImageHeader::LEN - 1]),
            Err(())
        );
        assert_eq!(
            ImageHeader::try_from_bytes(&[0; ImageHeader::LEN + 1]),
            Err(())
        );
    }

    #[test]
    fn test_vector_table_from_image_header_invalid_magic() {
        assert_test_image_invalid(*b"NOPE", ImageHeader::LEN as u32, 8, TEST_IMAGE_LEN);
    }

    #[test]
    fn test_vector_table_from_image_header_header_len_too_small() {
        assert_test_image_invalid(
            IMAGE_HEADER_MAGIC_BOOT1,
            (ImageHeader::LEN - 4) as u32,
            8,
            TEST_IMAGE_LEN,
        );
    }

    #[test]
    fn test_vector_table_from_image_header_header_len_unaligned() {
        assert_test_image_invalid(
            IMAGE_HEADER_MAGIC_BOOT1,
            (ImageHeader::LEN + 1) as u32,
            8,
            TEST_IMAGE_LEN,
        );
    }

    #[test]
    fn test_vector_table_from_image_header_code_len_too_small() {
        assert_test_image_invalid(
            IMAGE_HEADER_MAGIC_BOOT1,
            ImageHeader::LEN as u32,
            7,
            TEST_IMAGE_LEN,
        );
    }

    #[test]
    fn test_vector_table_from_image_header_header_len_exceeds_slot() {
        assert_test_image_invalid(
            IMAGE_HEADER_MAGIC_BOOT1,
            (TEST_IMAGE_LEN + 4) as u32,
            8,
            TEST_IMAGE_LEN,
        );
    }

    #[test]
    fn test_vector_table_from_image_header_code_len_exceeds_slot() {
        assert_test_image_invalid(
            IMAGE_HEADER_MAGIC_BOOT1,
            ImageHeader::LEN as u32,
            9,
            TEST_IMAGE_LEN,
        );
    }

    #[test]
    fn test_vector_table_from_image_header_invalid_vector_table() {
        assert_test_image_invalid(
            IMAGE_HEADER_MAGIC_BOOT1,
            ImageHeader::LEN as u32,
            8,
            TEST_IMAGE_LEN,
        );
    }

    #[test]
    fn test_vector_table_from_image_header_flash_range_overflow() {
        assert_test_image_invalid(
            IMAGE_HEADER_MAGIC_BOOT1,
            ImageHeader::LEN as u32,
            8,
            usize::MAX,
        );
    }

    #[test]
    fn test_vector_table_from_image_header_ram_range_overflow() {
        let image = build_test_image::<TEST_IMAGE_LEN>(
            IMAGE_HEADER_MAGIC_BOOT1,
            ImageHeader::LEN as u32,
            8,
        );

        assert_eq!(
            vector_table_from_image_header(
                image.0.as_ptr() as usize,
                image.0.len(),
                usize::MAX - 3,
                4,
                IMAGE_HEADER_MAGIC_BOOT1,
            ),
            Err(())
        );
    }

    #[test]
    fn test_vector_table_is_valid() {
        let vector_table = [TEST_INITIAL_STACK_POINTER, 0x0800_0101];

        assert!(unsafe { test_vector_table(vector_table.as_ptr()) });
    }

    #[test]
    fn test_vector_table_is_valid_erased_stack_pointer() {
        let vector_table = [u32::MAX, 0x0800_0101];

        assert!(!unsafe { test_vector_table(vector_table.as_ptr()) });
    }

    #[test]
    fn test_vector_table_is_valid_stack_pointer_outside_ram() {
        let vector_table = [(TEST_RAM_BASE + TEST_RAM_LEN + 4) as u32, 0x0800_0101];

        assert!(!unsafe { test_vector_table(vector_table.as_ptr()) });
    }

    #[test]
    fn test_vector_table_is_valid_unaligned_stack_pointer() {
        let vector_table = [TEST_INITIAL_STACK_POINTER - 4, 0x0800_0101];

        assert!(!unsafe { test_vector_table(vector_table.as_ptr()) });
    }

    #[test]
    fn test_vector_table_is_valid_erased_reset_vector() {
        let vector_table = [TEST_INITIAL_STACK_POINTER, u32::MAX];

        assert!(!unsafe { test_vector_table(vector_table.as_ptr()) });
    }

    #[test]
    fn test_vector_table_is_valid_reset_vector_outside_flash_area() {
        let vector_table = [TEST_INITIAL_STACK_POINTER, 0x0800_0301];

        assert!(!unsafe { test_vector_table(vector_table.as_ptr()) });
    }

    #[test]
    fn test_vector_table_is_valid_reset_vector_without_thumb_bit() {
        let vector_table = [TEST_INITIAL_STACK_POINTER, 0x0800_0100];

        assert!(!unsafe { test_vector_table(vector_table.as_ptr()) });
    }

    #[test]
    fn test_vector_table_is_valid_halfword_aligned_reset_vector() {
        let vector_table = [TEST_INITIAL_STACK_POINTER, 0x0800_0103];

        assert!(unsafe { test_vector_table(vector_table.as_ptr()) });
    }
}
