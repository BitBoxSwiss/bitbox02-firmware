// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

use bitbox_board_stm32u5_dk::ffi;
use bitbox_boot_utils::{
    BOOT1_ADDR, BOOT1_MAX_LEN, Boot1Manifest, DFU_METADATA_ADDR, FLASH_BASE_NS, FLASH_PAGE_SIZE,
    FLASH_TOTAL_SIZE, IMAGE_HEADER_MAGIC_BOOT1, IMAGE_SIGNATURE_COUNT, IMMUTABLE_PAGE_ADDR,
    P256_PUBLIC_KEY_LEN, bootload, halt, vector_table_from_signed_image,
};
use bitbox_mcu_stm32u5 as _;
use bitbox_platform_stm32u5::flash;
use bitbox03_boot_utils::{ImmutablePage, build_immutable_page_bytes};
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use sha2::{Digest, Sha256};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{info}");
    cortex_m::asm::bkpt();
    loop {
        cortex_m::asm::wfe();
    }
}

const ROOT_PUBKEYS: [[u8; P256_PUBLIC_KEY_LEN]; IMAGE_SIGNATURE_COUNT] = [
    [
        0x08, 0xa6, 0xdc, 0x5f, 0x9b, 0x9e, 0x0c, 0x74, 0x25, 0x06, 0x3d, 0x00, 0x77, 0x66, 0xe1,
        0x69, 0x0a, 0x57, 0xe7, 0x2d, 0xdb, 0xab, 0xa6, 0x4e, 0x3d, 0x88, 0x75, 0x41, 0x6d, 0xd1,
        0x86, 0x37, 0x9e, 0x01, 0x8c, 0x2a, 0xd1, 0xcf, 0x01, 0xf7, 0x0f, 0x92, 0x5c, 0x18, 0x4f,
        0x64, 0x36, 0xa9, 0xc3, 0xf8, 0x9a, 0x9c, 0x75, 0x9c, 0x92, 0xdb, 0x6a, 0x1a, 0x75, 0xcb,
        0x00, 0xb0, 0x26, 0x88,
    ],
    [
        0xf5, 0xb9, 0xd3, 0xa8, 0x43, 0x99, 0x2c, 0xb2, 0x5a, 0xcc, 0xd4, 0x20, 0xb8, 0x24, 0x65,
        0x46, 0x77, 0xa2, 0x03, 0xb0, 0x11, 0x68, 0xdb, 0x97, 0x26, 0x8d, 0xe4, 0xd5, 0xd1, 0x94,
        0x28, 0x95, 0x09, 0x3d, 0x22, 0x7e, 0x57, 0x8f, 0x19, 0x4f, 0x2c, 0xd8, 0x45, 0x05, 0x83,
        0xdf, 0xe8, 0xfe, 0xfd, 0x41, 0xdd, 0xb6, 0x7b, 0x05, 0xfe, 0xc1, 0x32, 0xfa, 0xc1, 0x51,
        0xe1, 0xbb, 0x44, 0xc7,
    ],
    [
        0xa9, 0x1a, 0x8e, 0xc6, 0x46, 0xfc, 0x37, 0x41, 0x64, 0xb5, 0xdc, 0xbf, 0x29, 0x80, 0xfd,
        0xbf, 0xbc, 0xd1, 0x2b, 0x57, 0xaf, 0xa0, 0x29, 0xa4, 0x05, 0x5d, 0x7f, 0x9a, 0x81, 0x75,
        0x0f, 0x18, 0xfc, 0x13, 0x48, 0xdc, 0xda, 0xbd, 0x6e, 0x33, 0x25, 0x5b, 0x29, 0xa5, 0xb7,
        0x51, 0x16, 0xbf, 0xf0, 0xca, 0xde, 0x45, 0xd6, 0x1c, 0x51, 0x4d, 0x86, 0x09, 0xfc, 0xa7,
        0x64, 0x1c, 0x9e, 0xe2,
    ],
];

#[used]
#[unsafe(link_section = ".immutable_data")]
static IMMUTABLE_PAGE: [u8; FLASH_PAGE_SIZE] = build_immutable_page_bytes(ROOT_PUBKEYS);

fn read_manifest() -> Option<Boot1Manifest> {
    let mut bytes = [0u8; Boot1Manifest::LEN];
    flash::read(DFU_METADATA_ADDR, &mut bytes);
    Boot1Manifest::from_bytes(&bytes).ok()
}

fn manifest_is_plausible(manifest: &Boot1Manifest) -> bool {
    let source_address = manifest.source_address as usize;
    let image_len = manifest.image_len as usize;
    if image_len == 0 || image_len > BOOT1_MAX_LEN {
        return false;
    }
    let Some(source_end) = source_address.checked_add(image_len) else {
        return false;
    };
    let dest_start = BOOT1_ADDR;
    let dest_end = BOOT1_ADDR + image_len;
    source_address >= FLASH_BASE_NS
        && source_end <= FLASH_BASE_NS + FLASH_TOTAL_SIZE
        && (source_end <= dest_start || source_address >= dest_end)
}

fn verify_manifest_hash(manifest: &Boot1Manifest) -> bool {
    let source_address = manifest.source_address as usize;
    let image_len = manifest.image_len as usize;
    let mut hasher = Sha256::new();
    let mut offset = 0usize;
    let mut chunk = [0u8; FLASH_PAGE_SIZE];

    while offset < image_len {
        let chunk_len = (image_len - offset).min(chunk.len());
        flash::read(source_address + offset, &mut chunk[..chunk_len]);
        hasher.update(&chunk[..chunk_len]);
        offset += chunk_len;
    }

    hasher.finalize().as_slice() == manifest.bootloader_hash
}

fn flash_boot1_from_manifest(manifest: &Boot1Manifest) -> Result<(), ()> {
    let source_address = manifest.source_address as usize;
    let image_len = manifest.image_len as usize;
    let page_count = image_len.div_ceil(FLASH_PAGE_SIZE);

    for page_index in 0..page_count {
        let page_addr = BOOT1_ADDR + page_index * FLASH_PAGE_SIZE;
        let page_start = source_address + page_index * FLASH_PAGE_SIZE;
        let remaining = image_len.saturating_sub(page_index * FLASH_PAGE_SIZE);
        let chunk_len = remaining.min(FLASH_PAGE_SIZE);
        let mut page = [0xff; FLASH_PAGE_SIZE];
        flash::read(page_start, &mut page[..chunk_len]);
        flash::write_page(page_addr, &page).map_err(|_| ())?;
    }

    Ok(())
}

fn main() -> ! {
    log::debug!("[b0] init");

    if let Some(manifest) = read_manifest() {
        if !manifest_is_plausible(&manifest) {
            halt();
        }

        if !verify_manifest_hash(&manifest) {
            halt();
        }

        if flash_boot1_from_manifest(&manifest).is_err() {
            halt();
        }
    }

    let Ok(root_pubkeys) =
        ImmutablePage::from_address(IMMUTABLE_PAGE_ADDR).map(|page| page.root_pubkeys)
    else {
        halt();
    };

    let Ok(vector_table) = vector_table_from_signed_image(
        BOOT1_ADDR,
        BOOT1_MAX_LEN,
        IMAGE_HEADER_MAGIC_BOOT1,
        &root_pubkeys,
    ) else {
        halt();
    };

    bootload(vector_table)
}

#[entry]
fn entry() -> ! {
    unsafe {
        ffi::board_init_essentials();
    }
    bitbox03_boot_utils::rtt_logger_init!();

    main()
}
