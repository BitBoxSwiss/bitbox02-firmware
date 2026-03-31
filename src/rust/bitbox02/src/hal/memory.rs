// SPDX-License-Identifier: Apache-2.0

#[cfg_attr(feature = "simulator-graphical", path = "memory_fake.rs")]
#[cfg_attr(not(feature = "simulator-graphical"), path = "memory_real.rs")]
#[allow(clippy::module_inception)]
mod memory;

pub use memory::*;
