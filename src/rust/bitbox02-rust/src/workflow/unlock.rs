// Copyright 2020 Shift Cryptosecurity AG
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

use crate::general::abort;
use crate::hal::Ui;
use crate::workflow::{confirm, password};

pub use password::CanCancel;

use alloc::vec::Vec;

/// Confirm the entered mnemonic passphrase with the user. Returns true if the user confirmed it,
/// false if the user rejected it.
async fn confirm_mnemonic_passphrase(
    hal: &mut impl crate::hal::Hal,
    passphrase: &str,
) -> Result<(), confirm::UserAbort> {
    // Accept empty passphrase without confirmation.
    if passphrase.is_empty() {
        return Ok(());
    }

    let params = confirm::Params {
        title: "",
        body: "You will be asked to\nvisually confirm your\npassphrase now.",
        accept_only: true,
        accept_is_nextarrow: true,
        ..Default::default()
    };

    hal.ui().confirm(&params).await?;

    let params = confirm::Params {
        title: "Confirm",
        body: passphrase,
        font: bitbox02::ui::Font::Password11X12,
        scrollable: true,
        longtouch: true,
        ..Default::default()
    };

    hal.ui().confirm(&params).await
}

pub enum UnlockError {
    UserAbort,
    IncorrectPassword,
    Memory,
    Generic,
}

impl core::convert::From<password::EnterError> for UnlockError {
    fn from(error: password::EnterError) -> Self {
        match error {
            password::EnterError::Cancelled => UnlockError::UserAbort,
            password::EnterError::Memory => UnlockError::Memory,
        }
    }
}

/// Prompts the user for the device password, and returns `Ok` if the
/// keystore was successfully unlocked, or `Err` if the password was
/// incorrect. In that case, a status is displayed with how many
/// attempts are remaining until the device resets.
///
/// If they keystore is already unlocked, this function does not
/// change the state and just checks the password.
pub async fn unlock_keystore(
    hal: &mut impl crate::hal::Hal,
    title: &str,
    can_cancel: password::CanCancel,
) -> Result<zeroize::Zeroizing<Vec<u8>>, UnlockError> {
    let password = password::enter(
        hal,
        title,
        password::PasswordType::DevicePassword,
        can_cancel,
    )
    .await?;

    match crate::keystore::unlock(&password) {
        Ok(seed) => Ok(seed),
        Err(crate::keystore::Error::IncorrectPassword) => {
            let msg = match crate::keystore::get_remaining_unlock_attempts() {
                1 => "Wrong password\n1 try remains".into(),
                n => format!("Wrong password\n{} tries remain", n),
            };
            hal.ui().status(&msg, false).await;
            Err(UnlockError::IncorrectPassword)
        }
        Err(err) => {
            let msg = format!("keystore unlock failed\n{:?}", err);
            hal.ui().status(&msg, false).await;
            Err(UnlockError::Generic)
        }
    }
}

/// Performs the BIP39 keystore unlock, including unlock animation. If the optional passphrase
/// feature is enabled, the user will be asked for the passphrase.
pub async fn unlock_bip39(hal: &mut impl crate::hal::Hal, seed: &[u8]) {
    // Empty passphrase by default.
    let mut mnemonic_passphrase = zeroize::Zeroizing::new("".into());

    // If setting activated, get the passphrase from the user.
    if bitbox02::memory::is_mnemonic_passphrase_enabled() {
        // Loop until the user confirms.
        loop {
            mnemonic_passphrase = password::enter(
                hal,
                "Optional passphrase",
                password::PasswordType::Bip39Passphrase,
                password::CanCancel::No,
            )
            .await
            .expect("not cancelable and does not call memory functions");

            if let Ok(()) = confirm_mnemonic_passphrase(hal, mnemonic_passphrase.as_str()).await {
                break;
            }

            hal.ui().status("Please try again", false).await;
        }
    }

    let ((), result) = futures_lite::future::zip(
        super::unlock_animation::animate(),
        crate::keystore::unlock_bip39(
            hal.random(),
            seed,
            &mnemonic_passphrase,
            // for the simulator, we don't yield at all, otherwise unlock becomes very slow in the
            // simulator.
            #[cfg(feature = "c-unit-testing")]
            async || {},
            // we yield every time to keep the processing time per iteration to a minimum.
            #[cfg(not(feature = "c-unit-testing"))]
            futures_lite::future::yield_now,
        ),
    )
    .await;

    if result.is_err() {
        abort("bip39 unlock failed");
    }
}

/// Invokes the unlock workflow. This function does not finish until the keystore is unlocked, or
/// the device is reset due to too many failed unlock attempts.
///
/// If the optional passphrase feature is enabled, the passphrase will also be entered by the
/// user. Otherwise, the empty "" passphrase is used by default.
///
/// Returns Ok on success, Err if the device cannot be unlocked because it was not initialized.
pub async fn unlock(hal: &mut impl crate::hal::Hal) -> Result<(), ()> {
    if !bitbox02::memory::is_initialized() {
        return Err(());
    }
    if !crate::keystore::is_locked() {
        return Ok(());
    }

    // Loop unlock until the password is correct or the device resets.
    loop {
        if let Ok(seed) = unlock_keystore(hal, "Enter password", password::CanCancel::No).await {
            unlock_bip39(hal, &seed).await;
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::keystore::testing::{mock_unlocked, mock_unlocked_using_mnemonic};
    use crate::workflow::testing::Screen;
    use alloc::boxed::Box;
    use bitbox02::testing::mock_memory;
    use util::bb02_async::block_on;

    use hex_lit::hex;

    #[test]
    fn test_unlock_success() {
        mock_memory();

        let mut mock_hal = TestingHal::new();

        // Set up an initialized wallet with password
        crate::keystore::encrypt_and_store_seed(
            &mut mock_hal,
            &hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c"),
            "password",
        )
        .unwrap();

        bitbox02::memory::set_initialized().unwrap();

        // Lock the keystore to simulate the normal locked state
        crate::keystore::lock();

        let mut password_entered = false;

        mock_hal.ui.set_enter_string(Box::new(|_params| {
            password_entered = true;
            Ok("password".into())
        }));
        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(block_on(unlock(&mut mock_hal)), Ok(()));
        // 6 for keystore unlock, 1 for keystore bip39 unlock.
        assert_eq!(bitbox02::securechip::fake_event_counter(), 7);

        assert!(!crate::keystore::is_locked());

        assert_eq!(
            crate::keystore::copy_bip39_seed().unwrap().as_slice(),
            &hex!(
                "cff4b263e5b0eb299e5fd35fcd09988f6b14e5b464f8d18fb84b152f889dd2a30550f4c2b346cae825ffedd4a87fc63fc12a9433de5125b6c7fdbc5eab0c590b"
            ),
        );

        drop(mock_hal); // to remove mutable borrow of `password_entered`
        assert!(password_entered);
    }

    #[test]
    fn test_unlock_keystore_wrong_password() {
        mock_memory();
        let mut mock_hal = TestingHal::new();

        // Set up an initialized wallet with password
        crate::keystore::encrypt_and_store_seed(
            &mut mock_hal,
            &hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c"),
            "password",
        )
        .unwrap();

        bitbox02::memory::set_initialized().unwrap();

        // Lock the keystore to simulate the normal locked state
        crate::keystore::lock();

        let mut password_entered = false;

        mock_hal.ui.set_enter_string(Box::new(|_params| {
            password_entered = true;
            Ok("wrong password".into())
        }));

        bitbox02::securechip::fake_event_counter_reset();
        assert!(matches!(
            block_on(unlock_keystore(
                &mut mock_hal,
                "title",
                password::CanCancel::No,
            )),
            Err(UnlockError::IncorrectPassword),
        ));
        assert_eq!(bitbox02::securechip::fake_event_counter(), 5);

        // Checks that the device is locked.
        assert!(crate::keystore::copy_seed().is_err());

        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Status {
                title: "Wrong password\n9 tries remain".into(),
                success: false,
            },],
        );

        drop(mock_hal); // to remove mutable borrow of `password_entered`
        assert!(password_entered);
    }
}
