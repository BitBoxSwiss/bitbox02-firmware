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

#ifndef _UNLOCK_BIP39_H
#define _UNLOCK_BIP39_H

#include "workflow.h"

#include <util.h>

/**
 * Unlocks BIP39 with the default empty passphrase, or with a user provided passphrase if
 * mnemonic passphrase support is enabled.
 * Displays a simple unlock animation.
 */
USE_RESULT
workflow_t* workflow_unlock_bip39(void (*callback)(void* param), void* callback_param);

/**
 * Blocking wrapper around workflow_unlock_bip39.
 */
void workflow_unlock_bip39_blocking(void);

#endif //  _UNLOCK_BIP39_H
