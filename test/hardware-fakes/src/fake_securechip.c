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

#include <rust/rust.h>
#include <salt.h>
#include <securechip/securechip.h>
#include <stdio.h>
#include <string.h>

static uint32_t _u2f_counter;

// Mocked contents of the securechip kdf slot.
static const uint8_t _kdfkey[32] =
    "\xd2\xe1\xe6\xb1\x8b\x6c\x6b\x08\x43\x3e\xdb\xc1\xd1\x68\xc1\xa0\x04\x37\x74\xa4\x22\x18\x77"
    "\xe7\x9e\xd5\x66\x84\xbe\x5a\xc0\x1b";

// Count how man seceurity events happen. The numbers were obtained by reading the security event
// counter slot (0xE0C5) on a real device. We can use this to assert how many events were used in
// unit tests. The number is relevant due to Optiga's throttling mechanism.
static uint32_t _event_counter = 0;

int securechip_kdf(const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    _event_counter += 1;
    rust_hmac_sha256(_kdfkey, 32, msg, len, kdf_out);
    return 0;
}

int securechip_init_new_password(const char* password)
{
    _event_counter += 1;
    (void)password;
    return 0;
}
int securechip_stretch_password(const char* password, uint8_t* stretched_out)
{
    _event_counter += 5;
    uint8_t key[9] = "unit-test";
    rust_hmac_sha256(key, sizeof(key), (const uint8_t*)password, strlen(password), stretched_out);
    return 0;
}
bool securechip_u2f_counter_set(uint32_t counter)
{
    _event_counter += 0;
    _u2f_counter = counter;
    return true;
}

bool securechip_u2f_counter_inc(uint32_t* counter)
{
    _event_counter += 0;
    *counter = _u2f_counter++;
    return true;
}

bool securechip_attestation_sign(const uint8_t* msg, uint8_t* signature_out)
{
    (void)msg;
    (void)signature_out;
    _event_counter += 1;
    return false;
}

bool securechip_monotonic_increments_remaining(uint32_t* remaining_out)
{
    *remaining_out = 1;
    return true;
}

bool securechip_model(securechip_model_t* model_out)
{
    *model_out = ATECC_ATECC608B;
    return true;
}

void fake_securechip_event_counter_reset(void)
{
    _event_counter = 0;
}

uint32_t fake_securechip_event_counter(void)
{
    return _event_counter;
}
