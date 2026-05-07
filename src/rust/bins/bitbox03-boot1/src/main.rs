// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

#[cfg(feature = "board-stm32u5a9j-dk")]
use bitbox_board_stm32u5a9j_dk as board;
use bitbox_boot_utils::{IMAGE_HEADER_LEN, bootload, halt};
use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{info}");
    halt();
}

fn firmware_vector_table() -> *const u32 {
    (board::memory::FIRMWARE_ADDR + IMAGE_HEADER_LEN) as *const u32
}

fn firmware_is_erased(vector_table: *const u32) -> bool {
    let initial_stack_pointer = unsafe { core::ptr::read_volatile(vector_table) };
    let reset_vector = unsafe { core::ptr::read_volatile(vector_table.add(1)) };
    initial_stack_pointer == u32::MAX || reset_vector == u32::MAX
}

#[entry]
fn main() -> ! {
    bitbox_debug::rtt_logger_init!();
    log::debug!("init");

    let vector_table = firmware_vector_table();
    if firmware_is_erased(vector_table) {
        log::error!("halt: firmware is erased");
        halt();
    }

    // SAFETY: Boot1 deliberately trusts the firmware at the fixed slot after
    // checking only that the vector table is not erased.
    unsafe { bootload(vector_table) }
}
