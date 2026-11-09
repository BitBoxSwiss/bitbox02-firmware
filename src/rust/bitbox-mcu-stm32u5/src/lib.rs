// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(any(
    all(feature = "stm32u5a9", feature = "stm32u5g9"),
    not(any(feature = "stm32u5a9", feature = "stm32u5g9")),
))]
compile_error!("select exactly one STM32U5 chip");

#[cfg(all(feature = "stm32u5g9", not(feature = "stm32u5a9")))]
pub use stm32_metapac as pac;
#[cfg(feature = "stm32u5a9")]
pub use stm32u5::stm32u5a9 as pac;
