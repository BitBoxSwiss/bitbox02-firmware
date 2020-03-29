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

use crate::workflow::confirm;
use crate::workflow::password_enter::password_enter;
use crate::workflow::status::status;
use bitbox02::keystore;
use bitbox02::password::Password;

/// Confirm the entered mnemonic passphrase with the user. Returns true if the user confirmed it,
/// false if the user rejected it.
async fn confirm_mnemonic_passphrase(passphrase: &str) -> bool {
    // Accept empty passphrase without confirmation.
    if passphrase.is_empty() {
        return true;
    }

    let params = confirm::Params {
        title: "",
        body: "You will be asked to\nvisually confirm your\npassphrase now.",
        accept_only: true,
        ..Default::default()
    };

    if !confirm::confirm(&params).await {
        // Can't happen because accept_only = true.
        return false;
    }

    let params = confirm::Params {
        title: "Confirm",
        body: passphrase,
        font: bitbox02::ui::Font::Password11X12,
        scrollable: true,
        longtouch: true,
        ..Default::default()
    };

    confirm::confirm(&params).await
}

/// Performs the BIP39 keystore unlock, including unlock animation. If the optional passphrase
/// feature is enabled, the user will be asked for the passphrase.
async fn unlock_bip39() {
    // Empty passphrase by default.
    let mut mnemonic_passphrase = Password::new();

    // If setting activated, get the passphrase from the user.
    if bitbox02::memory::is_mnemonic_passphrase_enabled() {
        // Loop until the user confirms.
        loop {
            password_enter("Enter\noptional passphrase", true, &mut mnemonic_passphrase).await;

            if confirm_mnemonic_passphrase(mnemonic_passphrase.as_str()).await {
                break;
            }

            status("Please try again", false).await;
        }
    }

    bitbox02::ui::with_lock_animation(|| {
        keystore::unlock_bip39(&mnemonic_passphrase).expect("bip39 unlock failed");
    });
}

/// Invokes the unlock workflow. This function does not finish until the keystore is unlocked, or
/// the device is reset due to too many failed unlock attempts.
///
/// If the optional passphrase feature is enabled, the passphrase will also be entered by the
/// user. Otherwise, the empty "" passphrase is used by default.
///
/// Returns Ok on success, Err if the device cannot be unlocked because it was not initialized.
pub async fn unlock() -> Result<(), ()> {
    if !bitbox02::memory::is_initialized() {
        return Err(());
    }
    if !bitbox02::keystore::is_locked() {
        return Ok(());
    }

    loop {
        let mut password = Password::new();
        password_enter("Enter password", false, &mut password).await;

        match keystore::unlock(&password) {
            Ok(()) => break,
            Err(keystore::Error::IncorrectPassword { remaining_attempts }) => {
                if remaining_attempts == 1 {
                    status("Wrong password\n1 try remains", false).await;
                } else {
                    status(
                        &format!("Wrong password\n{} tries remain", remaining_attempts),
                        false,
                    )
                    .await;
                };
            }
            _ => panic!("keystore unlock failed"),
        }
    }
    unlock_bip39().await;
    Ok(())
}
