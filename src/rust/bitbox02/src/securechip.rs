// SPDX-License-Identifier: Apache-2.0

#[cfg_attr(
    any(
        test,
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    ),
    path = "securechip/imp_fake.rs"
)]
mod imp;

pub(crate) use imp::*;
