// SPDX-License-Identifier: Apache-2.0

pub use bitbox_securechip::{Error, Model, PasswordStretchAlgo, SecureChipError};

#[cfg_attr(
    any(
        test,
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    ),
    path = "securechip/imp_fake.rs"
)]
pub(crate) mod imp;
