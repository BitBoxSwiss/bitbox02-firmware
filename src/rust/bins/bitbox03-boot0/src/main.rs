// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

#[cfg(feature = "board-stm32u5a9j-dk")]
use bitbox_board_stm32u5a9j_dk as board;
use bitbox_boot_utils::{IMAGE_HEADER_MAGIC_BOOT1, bootload, halt, vector_table_from_image_header};
use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{info}");
    halt();
}

fn boot1_vector_table() -> Result<*const u32, ()> {
    // SAFETY: The board memory layout defines BOOT1_ADDR as an aligned,
    // readable flash slot of BOOT1_MAX_LEN bytes. Boot0 does not mutate it
    // while validating the image.
    unsafe {
        vector_table_from_image_header(
            board::memory::BOOT1_ADDR,
            board::memory::BOOT1_MAX_LEN,
            board::memory::RAM_ADDR,
            board::memory::RAM_LEN,
            IMAGE_HEADER_MAGIC_BOOT1,
        )
    }
}

#[entry]
fn main() -> ! {
    bitbox_debug::rtt_logger_init!();
    log::debug!("init");

    let vector_table = match boot1_vector_table() {
        Ok(vector_table) => vector_table,
        Err(()) => {
            log::error!("halt: boot1 image is invalid");
            halt();
        }
    };
    // SAFETY: Boot0 deliberately trusts boot1 at the fixed slot after checking
    // its image header and vector table, including the stack pointer and reset
    // vector ranges.
    unsafe { bootload(vector_table) }
}
