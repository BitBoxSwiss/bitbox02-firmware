#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

pub mod const_init;
pub mod uninit;

#[cfg(feature = "cas")]
pub mod alloc_single;
