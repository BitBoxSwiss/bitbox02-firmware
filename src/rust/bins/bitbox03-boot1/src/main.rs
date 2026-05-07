// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

#[cfg(feature = "board-stm32u5a9j-dk")]
use bitbox_board_stm32u5a9j_dk as board;
use bitbox_boot_utils::{
    IMAGE_HEADER_MAGIC_FIRMWARE, bootload, halt, vector_table_from_image_header,
};
use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{info}");
    halt();
}

fn firmware_vector_table() -> Result<*const u32, ()> {
    vector_table_from_image_header(
        board::memory::FIRMWARE_ADDR,
        board::memory::FIRMWARE_MAX_LEN,
        IMAGE_HEADER_MAGIC_FIRMWARE,
    )
}

#[entry]
fn main() -> ! {
    bitbox_debug::rtt_logger_init!();
    log::debug!("init");

    let vector_table = match firmware_vector_table() {
        Ok(vector_table) => vector_table,
        Err(()) => {
            log::error!("halt: firmware image is invalid");
            halt();
        }
    };
    // SAFETY: Boot1 deliberately trusts the firmware at the fixed slot after
    // checking its image header and vector table, including the reset vector range.
    unsafe { bootload(vector_table) }
}
