// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::Error;
use crate::pb;

use pb::response::Response;

use crate::hal::Ui;
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

    if bitbox02::memory::set_mnemonic_passphrase_enabled(enabled).is_err() {
        return Err(Error::Memory);
    }

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use crate::hal::testing::TestingHal;
    use crate::workflow::testing::Screen;
    use alloc::boxed::Box;
    use bitbox02::testing::mock_memory;

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

        assert!(bitbox02::memory::is_mnemonic_passphrase_enabled());
        // Disable:
        let mut mock_hal = TestingHal::new();
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
        assert!(!bitbox02::memory::is_mnemonic_passphrase_enabled());

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
