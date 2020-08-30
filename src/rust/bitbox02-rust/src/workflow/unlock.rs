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
use crate::workflow::password;
use crate::workflow::status::status;
use bitbox02::keystore;
use bitbox02::keystore::Keystore;
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
        accept_is_nextarrow: true,
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

/// Prompts the user for the device password, and returns true if the
/// keystore was successfully unlocked, or false if the password was
/// incorrect. In that case, a status is displayed with how many
/// attempts are remaining until the device resets.
///
/// If they keystore is already unlocked, this function does not
/// change the state and just checks the password.
pub async fn unlock_keystore<K: Keystore>(title: &str) -> bool {
    let mut password = Password::new();
    password::enter(title, false, &mut password).await;

    match K::unlock(&password) {
        Ok(()) => true,
        Err(keystore::Error::IncorrectPassword { remaining_attempts }) => {
            let msg = match remaining_attempts {
                1 => "Wrong password\n1 try remains".into(),
                n => format!("Wrong password\n{} tries remain", n),
            };
            status(&msg, false).await;
            false
        }
        _ => panic!("keystore unlock failed"),
    }
}

/// Performs the BIP39 keystore unlock, including unlock animation. If the optional passphrase
/// feature is enabled, the user will be asked for the passphrase.
pub async fn unlock_bip39<K: Keystore>() {
    // Empty passphrase by default.
    let mut mnemonic_passphrase = Password::new();

    // If setting activated, get the passphrase from the user.
    if bitbox02::memory::is_mnemonic_passphrase_enabled() {
        // Loop until the user confirms.
        loop {
            password::enter("Optional passphrase", true, &mut mnemonic_passphrase).await;

            if confirm_mnemonic_passphrase(mnemonic_passphrase.as_str()).await {
                break;
            }

            status("Please try again", false).await;
        }
    }

    bitbox02::ui::with_lock_animation(|| {
        K::unlock_bip39(&mnemonic_passphrase).expect("bip39 unlock failed");
    });
}

/// Invokes the unlock workflow. This function does not finish until the keystore is unlocked, or
/// the device is reset due to too many failed unlock attempts.
///
/// If the optional passphrase feature is enabled, the passphrase will also be entered by the
/// user. Otherwise, the empty "" passphrase is used by default.
///
/// Returns Ok on success, Err if the device cannot be unlocked because it was not initialized.
pub async fn unlock<K: Keystore>() -> Result<(), ()> {
    if !bitbox02::memory::is_initialized() {
        return Err(());
    }
    if !K::is_locked() {
        return Ok(());
    }

    // Loop unlock until the password is correct or the device resets.
    while !unlock_keystore::<K>("Enter password").await {}

    unlock_bip39::<K>().await;
    Ok(())
}
