// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::pb;

use pb::response::Response;

use crate::hal::{Memory, Ui};
use crate::workflow::confirm;

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    &pb::SetMnemonicPassphraseEnabledRequest { enabled }: &pb::SetMnemonicPassphraseEnabledRequest,
) -> Result<Response, Error> {
    let params = confirm::Params {
        title: if enabled { "Enable" } else { "Disable" },
        body: "Optional\npassphrase",
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
    use bitbox02::testing::mock_memory;
    use util::bb02_async::block_on;

    #[test]
    pub fn test_mnemonic_passphrase_enabled() {
        // All good.
        mock_memory();
        // Enable:
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::SetMnemonicPassphraseEnabledRequest { enabled: true }
            )),
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
            block_on(process(
                &mut mock_hal,
                &pb::SetMnemonicPassphraseEnabledRequest { enabled: false }
            )),
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
            block_on(process(
                &mut mock_hal,
                &pb::SetMnemonicPassphraseEnabledRequest { enabled: true }
            )),
            Err(Error::UserAbort)
        );
    }
}
