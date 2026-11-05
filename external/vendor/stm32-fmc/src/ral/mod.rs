#![allow(non_snake_case)]

pub mod peripherals;
pub mod register;

pub use crate::{modify_reg, read_reg, write_reg};

pub mod fmc {
    pub use super::peripherals::fmc::*;
}
