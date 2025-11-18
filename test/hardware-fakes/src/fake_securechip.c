// SPDX-License-Identifier: Apache-2.0

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

int securechip_kdf(const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    rust_hmac_sha256(_kdfkey, 32, msg, len, kdf_out);
    return 0;
}

int securechip_init_new_password(
    const char* password,
    memory_password_stretch_algo_t password_stretch_algo)
{
    (void)password;
    (void)password_stretch_algo;
    return 0;
}
int securechip_stretch_password(
    const char* password,
    memory_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out)
{
    (void)password_stretch_algo;
    uint8_t key[9] = "unit-test";
    rust_hmac_sha256(key, sizeof(key), (const uint8_t*)password, strlen(password), stretched_out);
    return 0;
}

bool securechip_reset_keys(void)
{
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

bool securechip_attestation_sign(const uint8_t* msg, uint8_t* signature_out)
{
    (void)msg;
    (void)signature_out;
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
