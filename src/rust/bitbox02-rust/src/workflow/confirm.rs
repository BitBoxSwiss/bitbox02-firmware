// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use crate::hal::ui::{ConfirmParams, UserAbort};

pub(crate) use crate::hal::ui::MAX_CONFIRM_BODY_SIZE;

pub(crate) const TRUNCATION_WARNING_BODY: &str = "The next value is\ntoo large to display\nin full";

/// Confirm a potentially long value.
///
/// If the value exceeds the target UI's label limit, warn the user before showing it.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;

    #[async_test::test]
    async fn test_confirm_value() {
        let body = "a".repeat(MAX_CONFIRM_BODY_SIZE + 1);
        let params = ConfirmParams {
            title: "Value",
            body: &body,
            ..Default::default()
        };

        let mut hal = TestingHal::new();
        assert!(confirm_value(&mut hal, &params).await.is_ok());
        assert_eq!(hal.ui.screens.len(), 2);
        assert!(hal.ui.contains_confirm("Warning", TRUNCATION_WARNING_BODY));
    }
}
