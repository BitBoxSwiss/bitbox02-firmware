// Copyright 2024 Shift Crypto AG
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

#include "securechip.h"

#include <atecc/atecc.h>
#include <hardfault.h>
#include <memory/memory_shared.h>
#include <optiga/optiga.h>

typedef struct {
    int (*setup)(const securechip_interface_functions_t* fns);
    int (*kdf)(const uint8_t* msg, size_t msg_len, uint8_t* kdf_out);
    int (*init_new_password)(const char* password);
    int (*stretch_password)(const char* password, uint8_t* stretched_out);
    bool (*reset_keys)(void);
    bool (*gen_attestation_key)(uint8_t* pubkey_out);
    bool (*attestation_sign)(const uint8_t* challenge, uint8_t* signature_out);
    bool (*monotonic_increments_remaining)(uint32_t* remaining_out);
    bool (*random)(uint8_t* rand_out);
#if APP_U2F == 1 || FACTORYSETUP == 1
    bool (*u2f_counter_set)(uint32_t counter);
#endif
#if APP_U2F == 1
    bool (*u2f_counter_inc)(uint32_t* counter);
#endif
    bool (*model)(securechip_model_t* model_out);
} securechip_crypt_interface_t;

static securechip_crypt_interface_t _fns = {0};

// Detect if we have atecc or optiga chip and set interface functions
bool securechip_init(void)
{
    switch (memory_get_securechip_type()) {
    case MEMORY_SECURECHIP_TYPE_OPTIGA:
        _fns.setup = optiga_setup;
        _fns.kdf = optiga_kdf_external;
        _fns.init_new_password = optiga_init_new_password;
        _fns.stretch_password = optiga_stretch_password;
        _fns.reset_keys = optiga_reset_keys;
        _fns.gen_attestation_key = optiga_gen_attestation_key;
        _fns.attestation_sign = optiga_attestation_sign;
        _fns.monotonic_increments_remaining = optiga_monotonic_increments_remaining;
        _fns.random = optiga_random;
#if APP_U2F == 1 || FACTORYSETUP == 1
        _fns.u2f_counter_set = optiga_u2f_counter_set;
#endif
#if APP_U2F == 1
        _fns.u2f_counter_inc = optiga_u2f_counter_inc;
#endif
        _fns.model = optiga_model;
        break;
    case MEMORY_SECURECHIP_TYPE_ATECC:
    default:
        _fns.setup = atecc_setup;
        _fns.kdf = atecc_kdf;
        _fns.init_new_password = atecc_init_new_password;
        _fns.stretch_password = atecc_stretch_password;
        _fns.reset_keys = atecc_reset_keys;
        _fns.gen_attestation_key = atecc_gen_attestation_key;
        _fns.attestation_sign = atecc_attestation_sign;
        _fns.monotonic_increments_remaining = atecc_monotonic_increments_remaining;
        _fns.random = atecc_random;
#if APP_U2F == 1 || FACTORYSETUP == 1
        _fns.u2f_counter_set = atecc_u2f_counter_set;
#endif
#if APP_U2F == 1
        _fns.u2f_counter_inc = atecc_u2f_counter_inc;
#endif
        _fns.model = atecc_model;
        break;
    }
    return true;
}

#define ABORT_IF_NULL(fn)                 \
    do {                                  \
        if (_fns.fn == 0) {               \
            Abort("No " #fn " function"); \
        }                                 \
    } while (0)

int securechip_setup(const securechip_interface_functions_t* ifs)
{
    ABORT_IF_NULL(setup);
    return _fns.setup(ifs);
}

int securechip_kdf(const uint8_t* msg, size_t msg_len, uint8_t* mac_out)
{
    ABORT_IF_NULL(kdf);
    return _fns.kdf(msg, msg_len, mac_out);
}

int securechip_init_new_password(const char* password)
{
    ABORT_IF_NULL(init_new_password);
    return _fns.init_new_password(password);
}

int securechip_stretch_password(const char* password, uint8_t* stretched_out)
{
    ABORT_IF_NULL(stretch_password);
    return _fns.stretch_password(password, stretched_out);
}

bool securechip_reset_keys(void)
{
    ABORT_IF_NULL(reset_keys);
    return _fns.reset_keys();
}

bool securechip_gen_attestation_key(uint8_t* pubkey_out)
{
    ABORT_IF_NULL(gen_attestation_key);
    return _fns.gen_attestation_key(pubkey_out);
}

bool securechip_attestation_sign(const uint8_t* challenge, uint8_t* signature_out)
{
    ABORT_IF_NULL(attestation_sign);
    return _fns.attestation_sign(challenge, signature_out);
}

bool securechip_monotonic_increments_remaining(uint32_t* remaining_out)
{
    ABORT_IF_NULL(monotonic_increments_remaining);
    return _fns.monotonic_increments_remaining(remaining_out);
}

bool securechip_random(uint8_t* rand_out)
{
    ABORT_IF_NULL(random);
    return _fns.random(rand_out);
}

#if APP_U2F == 1 || FACTORYSETUP == 1
bool securechip_u2f_counter_set(uint32_t counter)
{
    ABORT_IF_NULL(u2f_counter_set);
    return _fns.u2f_counter_set(counter);
}
#endif
#if APP_U2F == 1
bool securechip_u2f_counter_inc(uint32_t* counter)
{
    ABORT_IF_NULL(u2f_counter_inc);
    return _fns.u2f_counter_inc(counter);
}
#endif
bool securechip_model(securechip_model_t* model_out)
{
    ABORT_IF_NULL(model);
    return _fns.model(model_out);
}
