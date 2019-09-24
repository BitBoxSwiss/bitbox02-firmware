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

#include <securechip/securechip.h>
#include <stdio.h>
#include <string.h>
#include <wally_crypto.h>

static uint32_t _u2f_counter;

bool securechip_update_keys(void)
{
    return true;
}

bool securechip_kdf(securechip_slot_t slot, const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    check_expected(slot);
    check_expected(msg);
    // wally_sha256(msg, len, kdf_out, 32);
    memcpy(kdf_out, (const uint8_t*)mock(), 32);
    return true;
}

bool securechip_u2f_counter_set(uint32_t counter)
{
    _u2f_counter = counter;
    return true;
}

bool securechip_u2f_counter_inc(uint32_t* counter)
{
    *counter = _u2f_counter++;
    return true;
}

bool securechip_ecc_unsafe_sign(const uint8_t* priv_key, const uint8_t* msg, uint8_t* sig)
{
    return false;
}

bool securechip_ecc_generate_public_key(uint8_t* priv_key, uint8_t* pub_key)
{
    return false;
}
