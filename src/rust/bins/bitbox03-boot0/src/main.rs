// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

use bitbox_board_stm32u5_dk::ffi;
use bitbox_boot_utils::{
    BOOT1_ADDR, BOOT1_MAX_LEN, Boot1Manifest, DFU_METADATA_ADDR, FLASH_BASE_NS, FLASH_PAGE_SIZE,
    FLASH_TOTAL_SIZE, IMAGE_HEADER_MAGIC_BOOT1, IMMUTABLE_PAGE_ADDR, bootload, halt,
    vector_table_from_signed_image_with_hasher,
};
use bitbox_mcu_stm32u5 as _;
use bitbox_platform_stm32u5::{flash, hash};
use bitbox03_boot_utils::ImmutablePage;
use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{info}");
    cortex_m::asm::bkpt();
    loop {
        cortex_m::asm::wfe();
    }
}

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
    let source = unsafe { core::slice::from_raw_parts(source_address as *const u8, image_len) };
    hash::sha256(source)
        .map(|image_hash| image_hash == manifest.bootloader_hash)
        .unwrap_or(false)
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

    let Ok(vector_table) = vector_table_from_signed_image_with_hasher(
        BOOT1_ADDR,
        BOOT1_MAX_LEN,
        IMAGE_HEADER_MAGIC_BOOT1,
        &root_pubkeys,
        |header, payload| hash::double_sha256_two_parts(header, payload).map_err(|_| ()),
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
