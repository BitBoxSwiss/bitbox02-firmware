#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)] // used in alloc.rs

#[cfg(test)]
#[macro_use]
extern crate std;

// Since util_c defines an "alloc_error_handler" we get conflicts with std when testing
#[cfg(not(test))]
mod alloc;

mod util;

#[cfg(feature = "platform-bitboxbase")]
pub mod bitboxbase;

#[cfg(feature = "platform-bitbox02")]
pub mod bitbox02;

#[cfg(feature = "app-ethereum")]
pub mod app_ethereum;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen. If we compile with `panic=abort`
// this code will never get executed.
#[cfg(not(test))]
#[cfg_attr(feature = "bootloader", allow(unused_variables))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(not(feature = "bootloader"))]
    bitbox02_rust::print_debug!(0, "Error: {}", info);
    loop {}
}
