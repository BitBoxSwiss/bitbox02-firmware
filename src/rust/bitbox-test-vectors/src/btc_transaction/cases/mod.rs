// SPDX-License-Identifier: Apache-2.0

use crate::btc_transaction::TestVector;

mod additional_psbt;
mod common;
mod descriptor_psbt;
mod metadata_psbt;
mod screens;
mod standard_psbt;

pub(super) use common::firmware_request_from_psbt;

pub fn all() -> Vec<TestVector> {
    let mut vectors = standard_psbt::all();
    vectors.extend(metadata_psbt::all());
    vectors.extend(additional_psbt::all());
    vectors.extend(descriptor_psbt::all());
    vectors
}
