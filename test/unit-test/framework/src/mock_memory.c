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

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <cmocka.h>

#include <fake_memory.h>
#include <flags.h>
#include <memory/memory.h>
#include <stdio.h>
#include <string.h>

bool __wrap_memory_is_initialized(void)
{
    return mock();
}

bool __wrap_memory_is_seeded(void)
{
    return mock();
}

static uint8_t _encrypted_seed_and_hmac[96];
static uint8_t _encrypted_seed_and_hmac_len;

bool __wrap_memory_set_encrypted_seed_and_hmac(uint8_t* encrypted_seed_and_hmac, uint8_t len)
{
    memcpy(_encrypted_seed_and_hmac, encrypted_seed_and_hmac, len);
    _encrypted_seed_and_hmac_len = len;
    return true;
}

bool __wrap_memory_get_encrypted_seed_and_hmac(
    uint8_t* encrypted_seed_and_hmac_out,
    uint8_t* len_out)
{
    *len_out = _encrypted_seed_and_hmac_len;
    memcpy(encrypted_seed_and_hmac_out, _encrypted_seed_and_hmac, *len_out);
    return true;
}

void __wrap_memory_get_device_name(char* name_out)
{
    snprintf(name_out, MEMORY_DEVICE_NAME_MAX_LEN, "%s", (const char*)mock());
}

bool __wrap_memory_set_device_name(const char* name)
{
    return mock();
}

bool __wrap_memory_set_mnemonic_passphrase_enabled(bool enabled)
{
    check_expected(enabled);
    return mock();
}

bool __wrap_memory_get_salt_root(uint8_t* salt_root_out)
{
    memcpy(salt_root_out, fake_memory_get_salt_root(), 32);
    return true;
}
