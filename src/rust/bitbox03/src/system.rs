use crate::BOOT_ARGS;
use bitbox_hal as hal;
#[cfg(target_arch = "arm")]
use core::arch::asm;
#[cfg(target_arch = "arm")]
use cortex_m::peripheral::SCB;

pub struct BitBox03System;

// Use a small emergency stack at the top of RAM so the actual wipe loop can
// run in Rust without clobbering its own active stack frames.
#[cfg(target_arch = "arm")]
const RESET_STACK_LEN: usize = 128;

#[cfg(target_arch = "arm")]
#[inline(never)]
unsafe extern "C" fn wipe_ram_and_reset_from_reset_stack() -> ! {
    unsafe extern "C" {
        static _ram_start: u32;
        static _ram_end: u32;
    }

    let mut current = (&raw const _ram_start) as usize;
    let wipe_end = ((&raw const _ram_end) as usize) - RESET_STACK_LEN;

    while current < wipe_end {
        unsafe {
            core::ptr::write_volatile(current as *mut u32, 0);
        }
        current += core::mem::size_of::<u32>();
    }

    SCB::sys_reset()
}

#[cfg(target_arch = "arm")]
#[inline(never)]
fn wipe_ram_and_reset() -> ! {
    unsafe extern "C" {
        static _ram_end: u32;
    }

    unsafe {
        asm!(
            "msr MSP, {stack_top}",
            "b {wipe_and_reset}",
            stack_top = in(reg) (&raw const _ram_end),
            wipe_and_reset = sym wipe_ram_and_reset_from_reset_stack,
            options(noreturn),
        );
    }
}

#[cfg(not(target_arch = "arm"))]
fn wipe_ram_and_reset() -> ! {
    panic!("RAM wipe reset is not available on this target")
}

impl hal::system::System for BitBox03System {
    async fn startup() {}

    fn is_btconly(&mut self) -> bool {
        false
    }

    fn reboot(&mut self) -> ! {
        BOOT_ARGS.clear();
        #[cfg(target_arch = "arm")]
        cortex_m::interrupt::disable();
        wipe_ram_and_reset()
    }

    fn reboot_to_bootloader(&mut self) -> ! {
        BOOT_ARGS.set_bootloader_wait();
        #[cfg(target_arch = "arm")]
        cortex_m::interrupt::disable();
        wipe_ram_and_reset()
    }

    fn reset_ble(&mut self) {
        todo!()
    }
    fn communication_timeout_reset(&mut self, _value: i16) {
        todo!()
    }
}
