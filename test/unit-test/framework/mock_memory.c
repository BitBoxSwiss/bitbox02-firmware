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

#include <memory/memory.h>
#include <mock_memory.h>
#include <stdio.h>
#include <string.h>

bool memory_write_to_address_mock(uint32_t addr, uint8_t* chunk)
{
    return true;
}

bool memory_write_chunk_mock(uint32_t chunk_num, uint8_t* chunk)
{
    return true;
}

void memory_read_chunk_mock(uint32_t chunk_num, uint8_t* chunk_out) {}

void memory_read_shared_bootdata_mock(uint8_t* chunk_out) {}

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

static uint8_t _salt_root[32];
void mock_memory_set_salt_root(const uint8_t* salt_root)
{
    memcpy(_salt_root, salt_root, 32);
}
bool __wrap_memory_get_salt_root(uint8_t* salt_root_out)
{
    memcpy(salt_root_out, _salt_root, 32);
    return true;
}
