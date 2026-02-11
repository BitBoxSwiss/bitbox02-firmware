// SPDX-License-Identifier: Apache-2.0

pub use super::cancel::Error;
pub use bitbox02::ui::{TrinaryInputStringParams as Params, trinary_input_string};

use alloc::string::String;

#[derive(Copy, Clone)]
pub enum CanCancel {
    No,
    Yes,
}

/// If `can_cancel` is `Yes`, the workflow can be cancelled.
/// If it is no, the result is always `Ok(())`.
/// If `preset` is not empty, it must be part of `params.wordlist` and will be pre-entered.
/// ```
pub async fn enter(
    params: &Params<'_>,
    can_cancel: CanCancel,
    preset: &str,
) -> Result<zeroize::Zeroizing<String>, Error> {
    let can_cancel = match can_cancel {
        CanCancel::Yes => true,
        CanCancel::No => false,
    };
    trinary_input_string(params, can_cancel, preset)
        .await
        .or(Err(Error::Cancelled))
}
