#![no_std]
#![no_main]

#[cfg(all(feature = "multi", feature = "btc-only"))]
compile_error!("Only one firmware variant can be enabled at a time.");

#[cfg(not(any(feature = "multi", feature = "btc-only")))]
compile_error!("One firmware variant must be enabled.");

extern crate bitbox02_rust_c as _;
