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
use crate::workflow::confirm;
use crate::workflow::password;
use crate::workflow::status::status;
use bitbox02::keystore;

pub use password::CanCancel;

use alloc::string::String;

/// Confirm the entered mnemonic passphrase with the user. Returns true if the user confirmed it,
/// false if the user rejected it.
pub async fn confirm_mnemonic_passphrase(passphrase: &str) -> Result<(), confirm::UserAbort> {
    // Accept empty passphrase without confirmation.
    if passphrase.is_empty() {
        confirm::confirm(&confirm::Params {
            title: "",
            body: "Proceed with\nempty passphrase?",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
        return Ok(());
    }

    confirm::confirm(&confirm::Params {
        title: "",
        body: "You will be asked to\nvisually confirm your\npassphrase now.",
        accept_only: true,
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    confirm::confirm(&confirm::Params {
        title: "Confirm",
        body: passphrase,
        font: bitbox02::ui::Font::Password11X12,
        scrollable: true,
        longtouch: true,
        ..Default::default()
    })
    .await
}

pub enum UnlockError {
    UserAbort,
    IncorrectPassword,
    Generic,
}

impl core::convert::From<super::cancel::Error> for UnlockError {
    fn from(_error: super::cancel::Error) -> Self {
        UnlockError::UserAbort
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
    title: &str,
    can_cancel: password::CanCancel,
) -> Result<(), UnlockError> {
    let password = password::enter(title, false, can_cancel).await?;

    match keystore::unlock(&password) {
        Ok(()) => Ok(()),
        Err(keystore::Error::IncorrectPassword { remaining_attempts }) => {
            let msg = match remaining_attempts {
                1 => "Wrong password\n1 try remains".into(),
                n => format!("Wrong password\n{} tries remain", n),
            };
            status(&msg, false).await;
            Err(UnlockError::IncorrectPassword)
        }
        Err(err) => {
            let msg = format!("keystore unlock failed\n{:?}", err);
            status(&msg, false).await;
            Err(UnlockError::Generic)
        }
    }
}

/// Prompts the user to enter the optional passphrase on the device and returns the entered
/// passphrase.
pub async fn enter_mnemonic_passphrase_on_device() -> Result<Option<zeroize::Zeroizing<String>>, ()>
{
    Ok(Some(
        password::enter("Optional passphrase", true, password::CanCancel::No)
            .await
            .expect("not cancelable"),
    ))
}

/// Performs the BIP39 keystore unlock, including unlock animation. If the optional passphrase
/// feature is enabled, the user will be asked for the passphrase.
pub async fn unlock_bip39<E>(
    get_mnemonic_passphrase: impl AsyncFn() -> Result<Option<zeroize::Zeroizing<String>>, E>,
) -> Result<(), E> {
    // Empty passphrase by default.
    let mut mnemonic_passphrase = zeroize::Zeroizing::new("".into());

    // If setting activated, get the passphrase from the user.
    if bitbox02::memory::is_mnemonic_passphrase_enabled() {
        // Loop until the user confirms.
        loop {
            if let Some(passphrase) = get_mnemonic_passphrase().await? {
                mnemonic_passphrase = passphrase;

                if let Ok(()) = confirm_mnemonic_passphrase(mnemonic_passphrase.as_str()).await {
                    break;
                }
            }

            status("Please try again", false).await;
        }
    }

    let result = bitbox02::ui::with_lock_animation(|| keystore::unlock_bip39(&mnemonic_passphrase));
    if result.is_err() {
        abort("bip39 unlock failed");
    }
    Ok(())
}

/// Invokes the unlock workflow. This function does not finish until the keystore is unlocked, or
/// the device is reset due to too many failed unlock attempts.
///
/// If the optional passphrase feature is enabled, the passphrase will be fetched using the
/// callback. Otherwise, the empty "" passphrase is used by default.
///
/// Returns Ok on success, Err if the device cannot be unlocked because it was not initialized.
pub async fn unlock<E>(
    get_mnemonic_passphrase: impl AsyncFn() -> Result<Option<zeroize::Zeroizing<String>>, E>,
) -> Result<(), E> {
    if !bitbox02::keystore::is_locked() {
        return Ok(());
    }

    // Loop unlock until the password is correct or the device resets.
    while unlock_keystore("Enter password", password::CanCancel::No)
        .await
        .is_err()
    {}

    unlock_bip39(get_mnemonic_passphrase).await
}
