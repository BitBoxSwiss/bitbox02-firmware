//! Peripheral access API for STM32U5 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.36.1)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.36.1/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32U5 devices; for the complete list please
//! see:
//! [stm32u5](https://crates.io/crates/stm32u5)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32-rs.github.io/stm32-rs/)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32u535")]
pub mod stm32u535;

#[cfg(feature = "stm32u545")]
pub mod stm32u545;

#[cfg(feature = "stm32u575")]
pub mod stm32u575;

#[cfg(feature = "stm32u585")]
pub mod stm32u585;

#[cfg(feature = "stm32u595")]
pub mod stm32u595;

#[cfg(feature = "stm32u599")]
pub mod stm32u599;

#[cfg(feature = "stm32u5a5")]
pub mod stm32u5a5;

#[cfg(feature = "stm32u5a9")]
pub mod stm32u5a9;

