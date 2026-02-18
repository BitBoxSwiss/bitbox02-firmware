// SPDX-License-Identifier: Apache-2.0

#[allow(unused)]
type HalImpl = bitbox02::hal::BitBox02Hal;

#[cfg(not(any(feature = "c-unit-testing", feature = "simulator-graphical")))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_main_loop() -> ! {
    bitbox02_rust::main_loop::main_loop(&mut HalImpl::new())
}
