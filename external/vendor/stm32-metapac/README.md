# stm32-metapac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for STMicroelectronics STM32 microcontrollers.

This crate has been automatically generated based on data in the [`stm32-data` project](https://github.com/embassy-rs/stm32-data), and is used for the [`embassy-stm32`](github.com/embassy-rs/embassy/) Rust Hardware Abstraction Layer (HAL) for the STM32 microcontrollers.

## Metadata

This PAC additionally exports "metadata" about the chips. To use it, enable the `metadata` feature and access it at `stm32_metapac::METADATA`. It is intended to be consumed from `build.rs` scripts or code-generation tools running on PCs, not from the firmware itself.

The metadata includes the following info:

- Memory maps for RAM, flash.
- Interrupts
- GPIO Alternate Function mappings
- Interrupt -> peripheral mappings
- DMA channel -> peripehral mappings
- RCC clock tree information for each peripheral (what clocks does it receive, which RCC registers to poke to enable, reset, or choose the clock)

## Supported chips

This PAC aims to support all STM32 chip families:

- STM32F0
- STM32F1
- STM32F2
- STM32F3
- STM32F4
- STM32F7
- STM32C0
- STM32G0
- STM32G4
- STM32H5
- STM32H7
- STM32H7RS
- STM32L0
- STM32L1
- STM32L4
- STM32L5
- STM32U0
- STM32U5
- STM32WB
- STM32WBA
- STM32WL
