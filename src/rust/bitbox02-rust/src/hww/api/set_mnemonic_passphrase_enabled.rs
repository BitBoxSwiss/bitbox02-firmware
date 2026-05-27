// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::hal::ui::ConfirmParams;
use crate::i18n::I18n as _;
use crate::pb;

use pb::response::Response;

use crate::hal::{Memory, Ui};

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    &pb::SetMnemonicPassphraseEnabledRequest { enabled }: &pb::SetMnemonicPassphraseEnabledRequest,
) -> Result<Response, Error> {
    let title = if enabled {
        crate::tr!(hal, "Enable")
    } else {
        crate::tr!(hal, "Disable")
    };
    let body = crate::tr!(hal, "Optional\npassphrase");
    let params = ConfirmParams {
        title: &title,
        body: &body,
        longtouch: true,
        ..Default::default()
    };

    hal.ui().confirm(&params).await?;

    if hal
        .memory()
        .set_mnemonic_passphrase_enabled(enabled)
        .is_err()
    {
        return Err(Error::Memory);
    }

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use alloc::boxed::Box;

    #[async_test::test]
    pub async fn test_mnemonic_passphrase_enabled() {
        // All good.

        // Enable:
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetMnemonicPassphraseEnabledRequest { enabled: true }
            )
            .await,
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Enable".into(),
                body: "Optional\npassphrase".into(),
                longtouch: true,
            }],
        );

        assert!(mock_hal.memory.is_mnemonic_passphrase_enabled());
        // Disable:
        mock_hal.ui.screens.clear();
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetMnemonicPassphraseEnabledRequest { enabled: false }
            )
            .await,
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Disable".into(),
                body: "Optional\npassphrase".into(),
                longtouch: true,
            }],
        );
        assert!(!mock_hal.memory.is_mnemonic_passphrase_enabled());

        // User aborted confirmation.
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.abort_nth(0);
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetMnemonicPassphraseEnabledRequest { enabled: true }
            )
            .await,
            Err(Error::UserAbort)
        );
    }
}
