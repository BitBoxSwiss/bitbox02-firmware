// SPDX-License-Identifier: Apache-2.0

// This crate contains safe wrappers around C functions provided by bitbox02_sys.
#![no_std]

#[cfg(any(test, feature = "c-unit-testing", feature = "simulator-graphical"))]
#[allow(unused_imports)]
#[macro_use]
extern crate std;

// allow unused as we use format!() only in some build configs (for
// the simulator), but replicating the conditions under which it is
// used here is not worth it.
#[allow(unused_imports)]
// for `format!`
#[macro_use]
extern crate alloc;

#[cfg(any(feature = "testing", feature = "simulator-graphical"))]
pub mod testing;

#[cfg(test)]
extern crate bitbox02_rust;

pub mod delay;
#[cfg(feature = "simulator-graphical")]
pub mod event;
#[cfg(feature = "simulator-graphical")]
pub mod hww;
pub mod memory;
#[cfg(feature = "simulator-graphical")]
pub mod queue;
pub mod random;
pub mod ringbuffer;
#[cfg(feature = "simulator-graphical")]
pub mod screen;
pub mod screen_saver;
pub mod sd;
pub mod securechip;
pub mod smarteeprom;
pub mod spi_mem;
pub mod ui;
#[cfg(feature = "simulator-graphical")]
pub mod usb_packet;
pub mod usb_processing;

pub use bitbox02_sys::buffer_t;
use core::time::Duration;

pub fn ug_put_string(x: i16, y: i16, input: &str, inverted: bool) {
    unsafe {
        bitbox02_sys::UG_PutString(
            x,
            y,
            util::strings::str_to_cstr_vec(input)
                .unwrap()
                .as_ptr()
                .cast(),
            inverted,
        );
    }
}

pub fn screen_clear() {
    unsafe { bitbox02_sys::screen_clear() }
}

pub fn ug_send_buffer() {
    unsafe { bitbox02_sys::UG_SendBuffer() }
}

pub fn ug_font_select_9x9() {
    unsafe { bitbox02_sys::UG_FontSelect(&bitbox02_sys::font_font_a_9X9) }
}

pub fn ug_font_select_11x10() {
    unsafe { bitbox02_sys::UG_FontSelect(&bitbox02_sys::font_font_a_11X10) }
}

pub fn screen_rotate() {
    unsafe { bitbox02_sys::screen_rotate() }
}

#[cfg_attr(not(target_arch = "arm"), allow(unused_variables))]
pub fn delay(duration: Duration) {
    #[cfg(target_arch = "arm")]
    {
        if duration < Duration::from_micros(1) {
            unsafe {
                // Sleep the smallest unit of sleep we support
                bitbox02_sys::delay_us(1)
            }
        } else if duration < Duration::from_millis(1) {
            unsafe {
                bitbox02_sys::delay_us(duration.as_micros() as u16);
            }
        } else {
            unsafe {
                bitbox02_sys::delay_ms(duration.as_millis() as u16);
            }
        }
    }
}

pub fn screen_print_debug(msg: &str, duration: i32) {
    unsafe {
        bitbox02_sys::screen_print_debug(
            util::strings::str_to_cstr_vec(msg).unwrap().as_ptr().cast(),
            duration,
        )
    }
}

pub fn reset_ble() {
    unsafe { bitbox02_sys::reset_ble() }
}

#[cfg(not(feature = "testing"))]
pub fn reboot() {
    unsafe { bitbox02_sys::reboot() }
}

#[allow(clippy::empty_loop)]
pub fn reboot_to_bootloader() -> ! {
    unsafe { bitbox02_sys::reboot_to_bootloader() }
    loop {}
}

#[cfg(any(feature = "testing", feature = "c-unit-testing"))]
pub fn print_stdout(msg: &str) {
    unsafe {
        bitbox02_sys::printf(util::strings::str_to_cstr_vec(msg).unwrap().as_ptr().cast());
    }
}

#[cfg(any(feature = "testing", feature = "c-unit-testing"))]
pub fn println_stdout(msg: &str) {
    unsafe {
        bitbox02_sys::printf(util::strings::str_to_cstr_vec(msg).unwrap().as_ptr().cast());
        bitbox02_sys::printf(
            util::strings::str_to_cstr_vec("\n")
                .unwrap()
                .as_ptr()
                .cast(),
        );
    }
}
