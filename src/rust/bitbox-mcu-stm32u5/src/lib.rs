// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![cfg(all(target_arch = "arm", target_os = "none"))]

#[cfg(any(
    all(feature = "stm32u5a9", feature = "stm32u5g9"),
    not(any(feature = "stm32u5a9", feature = "stm32u5g9")),
))]
compile_error!("select exactly one STM32U5 chip");

pub use stm32_metapac as pac;
