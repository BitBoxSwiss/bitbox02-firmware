//! SD Card Registers
//!
//! Register representations can be created from an array of little endian
//! words. Note that the SDMMC protocol transfers the registers in big endian
//! byte order.
//!
//! ```
//! # use sdio_host::sd::SCR;
//! let scr: SCR = [0, 1].into();
//! ```
//!
//! ## Reference documents:
//!
//! PLSS_v7_10: Physical Layer Specification Simplified Specification Version
//! 7.10. March 25, 2020. (C) SD Card Association

#![no_std]

pub mod common_cmd;
#[doc(inline)]
pub use common_cmd::Cmd;
pub mod sd_cmd;
pub mod emmc_cmd;

mod common;

pub mod sd;
pub mod emmc;
