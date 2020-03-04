#![no_std]
#![feature(alloc_error_handler)] // used in alloc.rs

mod alloc;
pub mod commander;
pub mod platform;
pub mod util;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen. If we compile with `panic=abort`
// this code will never get executed.
#[cfg(not(test))]
#[cfg_attr(not(debug_assertions), allow(unused_variables))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(debug_assertions)]
    bitbox02_rust::print_debug!(0, "Error: {}", info);
    #[cfg(not(debug_assertions))]
    bitbox02_rust::print_debug!(0, "Error");
    loop {}
}

// A trick to convince cbindgen that an u8 is char.
// cbindgen will convert `u8` to `uint8_t` and `i8` to `int8_t` which are `unsigned char` and
// `signed char` respectively. `c_char` is converted to `char` without `signed` or `unsigned`.
#[allow(non_camel_case_types)]
type c_char = u8;
