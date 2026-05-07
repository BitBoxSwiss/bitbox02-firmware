// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub mod flash;

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod inner;
