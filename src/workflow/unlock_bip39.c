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

#include "unlock_bip39.h"

#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <rust/rust.h>
#include <ui/components/trinary_input_string.h>
#include <ui/graphics/lock_animation.h>
#include <ui/workflow_stack.h>
#include <util.h>

#include "get_mnemonic_passphrase.h"
#include "workflow.h"

typedef struct {
    /**
     * Buffer containing the generated mnemonic passphrase.
     * Empty passphrase by default.
     */
    char mnemonic_passphrase[SET_PASSWORD_MAX_PASSWORD_LENGTH];
    void (*callback)(void*);
    void* callback_param;
} unlock_bip39_data_t;

static void _passphrase_ready(char* passphrase, void* param)
{
    unlock_bip39_data_t* data = (unlock_bip39_data_t*)param;
    int n_written =
        snprintf(data->mnemonic_passphrase, sizeof(data->mnemonic_passphrase), "%s", passphrase);
    if (n_written < 0 || (unsigned int)n_written >= sizeof(data->mnemonic_passphrase)) {
        Abort("unlock bip39 bad passphrase length");
    }
}

static void _unlock_bip39_cleanup(workflow_t* self)
{
    unlock_bip39_data_t* data = (unlock_bip39_data_t*)self->data;
    util_zero(data->mnemonic_passphrase, sizeof(data->mnemonic_passphrase));
}

static void _unlock_bip39_init(workflow_t* self)
{
    unlock_bip39_data_t* data = (unlock_bip39_data_t*)self->data;
    if (memory_is_mnemonic_passphrase_enabled()) {
        workflow_stack_start_workflow(workflow_get_mnemonic_passphrase(_passphrase_ready, data));
    }
}

/**
 * When this workflow becomes active, any needed passphrase
 * has already been asked for. We just need to try unlocking
 * the seed and exit.
 */
static void _unlock_bip39_spin(workflow_t* self)
{
    unlock_bip39_data_t* data = (unlock_bip39_data_t*)self->data;

    lock_animation_start();
    bool unlock_result = keystore_unlock_bip39(data->mnemonic_passphrase);
    lock_animation_stop();

    if (!unlock_result) {
        Abort("bip39 unlock failed");
    }
    if (data->callback) {
        data->callback(data->callback_param);
    }
    workflow_stack_stop_workflow();
}

workflow_t* workflow_unlock_bip39(void (*callback)(void* param), void* callback_param)
{
    workflow_t* result = workflow_allocate(
        _unlock_bip39_init, _unlock_bip39_cleanup, _unlock_bip39_spin, sizeof(unlock_bip39_data_t));
    unlock_bip39_data_t* data = (unlock_bip39_data_t*)result->data;
    data->callback = callback;
    data->callback_param = callback_param;
    return result;
}

void workflow_unlock_bip39_blocking(void)
{
    rust_workflow_status_unlock_bip39_blocking();
}
