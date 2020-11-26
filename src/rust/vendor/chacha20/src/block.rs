//! The ChaCha20 block function. Defined in RFC 8439 Section 2.3.
//!
//! <https://tools.ietf.org/html/rfc8439#section-2.3>

// TODO(tarcieri): figure out what circumstances these occur in
#![allow(unused_imports)]

pub(crate) mod soft;

use crate::rounds::Rounds;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2",
    not(target_feature = "avx2")
))]
mod sse2;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
mod avx2;

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    any(target_feature = "sse2", target_feature = "avx2")
)))]
pub(crate) use self::soft::{Block, BUFFER_SIZE};

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2",
    not(target_feature = "avx2")
))]
pub(crate) use self::sse2::{Block, BUFFER_SIZE};

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
pub(crate) use self::avx2::{Block, BUFFER_SIZE};

use core::fmt::{self, Debug};

/// Common debug impl for all blocks
impl<R: Rounds> Debug for Block<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Block {{  .. }}")
    }
}
