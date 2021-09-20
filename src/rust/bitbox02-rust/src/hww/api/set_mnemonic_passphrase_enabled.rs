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

use crate::workflow::confirm;

pub async fn process(
    &pb::SetMnemonicPassphraseEnabledRequest { enabled }: &pb::SetMnemonicPassphraseEnabledRequest,
) -> Result<Response, Error> {
    let params = confirm::Params {
        title: if enabled { "Enable" } else { "Disable" },
        body: "Optional\npassphrase",
        longtouch: true,
        ..Default::default()
    };

    confirm::confirm(&params).await?;

    if bitbox02::memory::set_mnemonic_passphrase_enabled(enabled).is_err() {
        return Err(Error::Memory);
    }

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::{mock, mock_memory, Data, MUTEX};
    use std::boxed::Box;

    #[test]
    pub fn test_mnemonic_passphrase_enabled() {
        let _guard = MUTEX.lock().unwrap();

        // All good.
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                assert_eq!(params.body, "Optional\npassphrase");
                true
            })),
            ..Default::default()
        });
        mock_memory();
        // Enable:
        assert_eq!(
            block_on(process(&pb::SetMnemonicPassphraseEnabledRequest {
                enabled: true
            })),
            Ok(Response::Success(pb::Success {}))
        );
        assert!(bitbox02::memory::is_mnemonic_passphrase_enabled());
        // Disable:
        assert_eq!(
            block_on(process(&pb::SetMnemonicPassphraseEnabledRequest {
                enabled: false
            })),
            Ok(Response::Success(pb::Success {}))
        );
        assert!(!bitbox02::memory::is_mnemonic_passphrase_enabled());

        // User aborted confirmation.
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                assert_eq!(params.body, "Optional\npassphrase");
                false
            })),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::SetMnemonicPassphraseEnabledRequest {
                enabled: true
            })),
            Err(Error::UserAbort)
        );
    }
}
