// SPDX-License-Identifier: Apache-2.0

pub use bitbox02::ui::{ConfirmParams as Params, Font};

pub struct UserAbort;

/// Returns `Ok(())` if the user accepts, `Err(UserAbort)` if the user rejects.
pub async fn confirm(params: &Params<'_>) -> Result<(), UserAbort> {
    if bitbox02::ui::confirm(params).await {
        Ok(())
    } else {
        Err(UserAbort)
    }
}
