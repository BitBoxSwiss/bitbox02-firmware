#include "get_mnemonic_passphrase.h"

#include <string.h>

#include <hardfault.h>
#include <ui/components/trinary_input_string.h>
#include <ui/fonts/password_11X12.h>
#include <ui/workflow_stack.h>

#include "blocking.h"
#include "confirm.h"
#include "password_enter.h"
#include "status.h"

typedef enum {
    MNEMONIC_PASSPHRASE_PW_ENTER,
    MNEMONIC_PASSPHRASE_CONFIRM_MSG,
    MNEMONIC_PASSPHRASE_CONFIRM_PASSPHRASE,
    MNEMONIC_PASSPHRASE_TRY_AGAIN,
    MNEMONIC_PASSPHRASE_FINISHED
} mnemonic_passphrase_state_t;

typedef struct {
    mnemonic_passphrase_state_t state;
    char* passphrase;
    void (*callback)(char*, void*);
    void* callback_param;
} mnemonic_passphrase_data_t;

/**
 * Frees and zeros out the passphrase memory.
 */
static void _clean_passphrase(mnemonic_passphrase_data_t* data)
{
    if (data->passphrase) {
        util_zero(data->passphrase, strlen(data->passphrase));
        free(data->passphrase);
        data->passphrase = NULL;
    }
}

/**
 * Invoked when the user has finished entering the passphrase.
 */
static void _passphrase_ready_cb(const char* passphrase, void* param)
{
    mnemonic_passphrase_data_t* data = (mnemonic_passphrase_data_t*)param;
    data->passphrase = util_strdup(passphrase);
    if (strlen(passphrase) == 0) {
        // No need to confirm the empty passphrase.
        data->state = MNEMONIC_PASSPHRASE_FINISHED;
    } else {
        data->state = MNEMONIC_PASSPHRASE_CONFIRM_MSG;
    }
}

/**
 * Invoked when the user has finished with the visual
 * passphrase confirmation.
 */
static void _confirm_passphrase_cb(bool result, void* param)
{
    mnemonic_passphrase_data_t* data = (mnemonic_passphrase_data_t*)param;
    if (!result) {
        /* Confirmation failed. Start over. */
        _clean_passphrase(data);
        data->state = MNEMONIC_PASSPHRASE_TRY_AGAIN;
    } else {
        data->state = MNEMONIC_PASSPHRASE_FINISHED;
    }
}

static void _get_mnemonic_passphrase_init(workflow_t* self)
{
    mnemonic_passphrase_data_t* data = (mnemonic_passphrase_data_t*)self->data;
    data->state = MNEMONIC_PASSPHRASE_PW_ENTER;
    data->passphrase = NULL;
}

static void _get_mnemonic_passphrase_cleanup(workflow_t* self)
{
    mnemonic_passphrase_data_t* data = (mnemonic_passphrase_data_t*)self->data;
    _clean_passphrase(data);
}

static void _get_mnemonic_passphrase_spin(workflow_t* self)
{
    mnemonic_passphrase_data_t* data = (mnemonic_passphrase_data_t*)self->data;
    switch (data->state) {
    case MNEMONIC_PASSPHRASE_PW_ENTER:
        workflow_stack_start_workflow(
            password_enter("Optional passphrase", true, _passphrase_ready_cb, data));
        break;
    case MNEMONIC_PASSPHRASE_CONFIRM_MSG: {
        const confirm_params_t params = {
            .title = "",
            .body = "You will be asked to\nvisually confirm your\npassphrase now.",
            .accept_only = true,
        };
        workflow_stack_start_workflow(workflow_confirm(&params, NULL, NULL));
        data->state = MNEMONIC_PASSPHRASE_CONFIRM_PASSPHRASE;
        break;
    }
    case MNEMONIC_PASSPHRASE_CONFIRM_PASSPHRASE: {
        const confirm_params_t passphrase_params = {.title = "Confirm",
                                                    .body = data->passphrase,
                                                    .font = &font_password_11X12,
                                                    .scrollable = true,
                                                    .longtouch = true};
        workflow_stack_start_workflow(
            workflow_confirm(&passphrase_params, _confirm_passphrase_cb, data));
        break;
    }
    case MNEMONIC_PASSPHRASE_TRY_AGAIN:
        workflow_stack_start_workflow(workflow_status("Please try again", false, NULL, NULL));
        data->state = MNEMONIC_PASSPHRASE_PW_ENTER;
        break;
    case MNEMONIC_PASSPHRASE_FINISHED:
        if (data->callback) {
            data->callback(data->passphrase, data->callback_param);
        }
        _clean_passphrase(data);
        workflow_stack_stop_workflow();
        break;
    default:
        Abort("Invalid get_mnemonic_passphrase state.");
    }
}

workflow_t* workflow_get_mnemonic_passphrase(void (*callback)(char*, void*), void* callback_param)
{
    workflow_t* result = workflow_allocate(
        _get_mnemonic_passphrase_init,
        _get_mnemonic_passphrase_cleanup,
        _get_mnemonic_passphrase_spin,
        sizeof(mnemonic_passphrase_data_t));
    mnemonic_passphrase_data_t* data = (mnemonic_passphrase_data_t*)result->data;
    data->callback = callback;
    data->callback_param = callback_param;
    return result;
}
