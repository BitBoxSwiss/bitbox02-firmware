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

use alloc::vec::Vec;

use super::Error;
use crate::pb;

use pb::response::Response;

use crate::hal::Ui;
use crate::workflow::{confirm, unlock};

/// Handle the ShowMnemonic API call. This shows the seed encoded as
/// 12/18/24 BIP39 English words. Afterwards, for each word, the user
/// is asked to pick the right word among 5 words, to check if they
/// wrote it down correctly.
pub async fn process(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    let mnemonic_sentence = {
        let seed = if bitbox02::memory::is_initialized() {
            unlock::unlock_keystore(hal, "Unlock device", unlock::CanCancel::Yes).await?
        } else {
            crate::keystore::copy_seed(hal)?
        };

        crate::bip39::mnemonic_from_seed(&seed)?
    };

    hal.ui()
        .confirm(&confirm::Params {
            title: "Warning",
            body: "DO NOT share your\nrecovery words with\nanyone!",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    hal.ui()
        .confirm(&confirm::Params {
            title: "Recovery\nwords",
            body: "Please write down\nthe following words",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    let words: Vec<&str> = mnemonic_sentence.split(' ').collect();

    hal.ui().show_and_confirm_mnemonic(&words).await?;

    bitbox02::memory::set_initialized().or(Err(Error::Memory))?;

    hal.ui().status("Backup created", true).await;
    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::boxed::Box;

    use crate::hal::testing::TestingHal;
    use crate::workflow::testing::Screen;
    use bitbox02::testing::mock_memory;
    use util::bb02_async::block_on;

    /// When not yet initialized, we show the mnemonic without a password check. This happens during
    /// wallet setup.
    #[test]
    fn test_process_uninitialized() {
        mock_memory();
        crate::keystore::encrypt_and_store_seed(
            &mut TestingHal::new(),
            hex::decode("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c")
                .unwrap()
                .as_slice(),
            "password",
        )
        .unwrap();

        assert!(!bitbox02::memory::is_initialized());

        let mut mock_hal = TestingHal::new();
        mock_hal.ui.set_enter_string(Box::new(|_params| {
            panic!("unexpected call to enter password")
        }));

        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            block_on(process(&mut mock_hal)),
            Ok(Response::Success(pb::Success {}))
        );
        // 1 operation for one copy_seed() to get the seed to display it.
        assert_eq!(mock_hal.securechip.get_event_counter(), 1);

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Warning".into(),
                    body: "DO NOT share your\nrecovery words with\nanyone!".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Recovery\nwords".into(),
                    body: "Please write down\nthe following words".into(),
                    longtouch: false
                },
                Screen::ShowAndConfirmMnemonic {
                    mnemonic: "shy parrot age monkey rhythm snake mystery burden topic hello mouse script gesture tattoo demand float verify shoe recycle cool network better aspect list".into(),
                },
                Screen::Status {
                    title: "Backup created".into(),
                    success: true
                },
            ]
        );
    }
    /// When initialized, a password check is prompted before displaying the mnemonic.
    #[test]
    fn test_process_initialized() {
        mock_memory();
        crate::keystore::encrypt_and_store_seed(
            &mut TestingHal::new(),
            hex::decode("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c")
                .unwrap()
                .as_slice(),
            "password",
        )
        .unwrap();

        bitbox02::memory::set_initialized().unwrap();

        let mut password_entered: bool = false;

        let mut mock_hal = TestingHal::new();
        mock_hal.ui.set_enter_string(Box::new(|_params| {
            password_entered = true;
            Ok("password".into())
        }));

        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            block_on(process(&mut mock_hal)),
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 5);

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Warning".into(),
                    body: "DO NOT share your\nrecovery words with\nanyone!".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Recovery\nwords".into(),
                    body: "Please write down\nthe following words".into(),
                    longtouch: false
                },
                Screen::ShowAndConfirmMnemonic {
                    mnemonic: "shy parrot age monkey rhythm snake mystery burden topic hello mouse script gesture tattoo demand float verify shoe recycle cool network better aspect list".into(),
                },
                Screen::Status {
                    title: "Backup created".into(),
                    success: true
                },
            ]
        );

        drop(mock_hal); // to remove mutable borrow of `password_entered`
        assert!(password_entered);
    }

    /// When initialized, a password check is prompted before displaying the mnemonic.
    /// This tests that we fail early if the wrong password is entered.
    #[test]
    fn test_process_initialized_wrong_password() {
        mock_memory();
        crate::keystore::encrypt_and_store_seed(
            &mut TestingHal::new(),
            hex::decode("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c")
                .unwrap()
                .as_slice(),
            "password",
        )
        .unwrap();

        bitbox02::memory::set_initialized().unwrap();

        let mut mock_hal = TestingHal::new();
        mock_hal
            .ui
            .set_enter_string(Box::new(|_params| Ok("wrong password".into())));

        mock_hal.securechip.event_counter_reset();
        assert_eq!(block_on(process(&mut mock_hal)), Err(Error::Generic));
        assert_eq!(mock_hal.securechip.get_event_counter(), 5);

        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Status {
                title: "Wrong password\n9 tries remain".into(),
                success: false,
            }]
        );
    }
}
