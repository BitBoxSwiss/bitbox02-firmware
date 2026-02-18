// SPDX-License-Identifier: Apache-2.0

#[cfg(not(any(feature = "c-unit-testing", feature = "simulator-graphical")))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_main_loop() -> ! {
    bitbox02_rust::main_loop::main_loop(&mut crate::HalImpl::new())
}
