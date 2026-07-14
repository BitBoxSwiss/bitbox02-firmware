// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use crate::hal::ui::{ConfirmParams, UserAbort};

// Keep this in sync with src/ui/components/label.h:MAX_LABEL_SIZE. `MAX_CONFIRM_BODY_SIZE` is the
// effective confirmation body limit and intentionally matches that UI label size limit.
pub(crate) const MAX_CONFIRM_BODY_SIZE: usize = 640;

pub(crate) const TRUNCATION_WARNING_BODY: &str = "The next value is\ntoo large to display\nin full";

/// Confirm a potentially long value.
///
/// If the value exceeds the UI label limit, the UI will truncate it with `...`, so warn the user
/// before showing it.
pub(crate) async fn confirm_value(
    hal: &mut impl crate::hal::Hal,
    params: &ConfirmParams<'_>,
) -> Result<(), UserAbort> {
    if params.body.len() > MAX_CONFIRM_BODY_SIZE {
        hal.ui()
            .confirm(&ConfirmParams {
                title: "Warning",
                body: TRUNCATION_WARNING_BODY,
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;
    }
    hal.ui().confirm(params).await
}
