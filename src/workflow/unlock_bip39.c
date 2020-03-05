#include "unlock_bip39.h"

#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <screen.h>
#include <ui/components/trinary_input_string.h>
#include <ui/components/ui_images.h>
#include <ui/ugui/ugui.h>
#include <ui/workflow_stack.h>
#include <util.h>

#ifndef TESTING
#include <hal_delay.h>
#endif

#include "blocking.h"
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
        workflow_stack_start_workflow(get_mnemonic_passphrase(_passphrase_ready, data));
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
    { // animation
        // Cannot render screens during unlocking (unlocking blocks)
        // Therefore hardcode a status screen
        UG_ClearBuffer();
        image_lock(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 1, IMAGE_DEFAULT_LOCK_RADIUS);
        UG_SendBuffer();
#ifndef TESTING
        delay_ms(1200);
#endif
        UG_ClearBuffer();
        image_unlock(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 1, IMAGE_DEFAULT_LOCK_RADIUS);
        UG_SendBuffer();
    }

    if (!keystore_unlock_bip39(data->mnemonic_passphrase)) {
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

static void _unlock_cb(void* param)
{
    (void)param;
    workflow_blocking_unblock();
}

void workflow_unlock_bip39_blocking(void)
{
    workflow_stack_start_workflow(workflow_unlock_bip39(_unlock_cb, NULL));
    workflow_blocking_block();
}
