#![cfg_attr(feature = "nightly", feature(maybe_uninit_ref))]
#![cfg_attr(feature = "nightly", feature(never_type))]
#![cfg_attr(all(feature = "std", feature = "nightly"), feature(read_initializer))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(dead_code))]

pub mod error;
pub mod io;

#[cfg(feature = "alloc")]
extern crate alloc;
