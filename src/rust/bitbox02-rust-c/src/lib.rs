// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod alloc;

#[cfg(feature = "firmware")]
pub mod async_usb;
#[cfg(feature = "firmware")]
mod der;
#[cfg(feature = "firmware")]
pub mod workflow;

// Expose C interface defined in bitbox_aes
#[cfg(feature = "firmware")]
extern crate bitbox_aes;

// Expose C interface defined in util
extern crate util;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen and over RTT. If we compile with
// `panic=abort` this code will never get executed.
#[cfg_attr(feature = "bootloader", allow(unused_variables))]
#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "firmware")]
    ::util::log::log!("{}", info);
    #[cfg(feature = "firmware")]
    bitbox02_rust::print_screen!(0, "Error: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}
