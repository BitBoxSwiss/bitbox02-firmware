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

#include "create_seed.h"
#include "password.h"
#include "unlock_bip39.h"

#include <hardfault.h>
#include <keystore.h>
#include <util.h>

bool workflow_create_seed(const uint8_t* host_entropy)
{
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(password);
    if (!password_set(password)) {
        return false;
    }
    bool result = keystore_create_and_store_seed(password, host_entropy);
    if (!result) {
        return false;
    }
    uint8_t remaining_attempts;
    if (keystore_unlock(password, &remaining_attempts) != KEYSTORE_OK) {
        // This should/can never happen, but let's check anyway.
        Abort("Unexpected error during restore: unlock failed.");
    }
    workflow_unlock_bip39_blocking();
    return true;
}
