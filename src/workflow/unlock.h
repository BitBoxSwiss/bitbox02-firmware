// Copyright 2019 Shift Cryptosecurity AG
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

#ifndef _WORKFLOW_UNLOCK_H_
#define _WORKFLOW_UNLOCK_H_

#include <keystore.h>

/**
 * Unlocks the keystore and stretches the seed with
 * a BIP39 passphrase.
 */
void workflow_unlock_enter_done(const char* password);

/**
 * Prompts the user for the password and unlocks the keystore. It is blocking until either the user
 * enters the correct password, or the device is reset after too many failed attempts.
 */
void workflow_unlock(void);

/**
 * Tries to unlock the key store, showing appropriate error messages.
 * If the error is KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED (device reset), we return to workflow_start.
 */
keystore_error_t workflow_unlock_and_handle_error(const char* password);

/**
 * Unlocks BIP39 with the default empty passphrase, or with a user provided passphrase if
 * mnemonic passphrase support is enabled.
 * Displays a simple unlock animation.
 */
void workflow_unlock_bip39(void);

#endif
