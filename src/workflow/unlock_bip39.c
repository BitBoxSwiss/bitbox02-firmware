#include "unlock_bip39.h"

#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <screen.h>
#include <ui/components/trinary_input_string.h>
#include <ui/components/ui_images.h>
#include <ui/graphics/lock_animation.h>
#include <ui/ugui/ugui.h>
#include <ui/workflow_stack.h>
#include <util.h>

#ifndef TESTING
#include <hal_timer.h>
#include <platform/driver_init.h>
#endif

#include "blocking.h"
#include "get_mnemonic_passphrase.h"
#include "workflow.h"

#define TIMEOUT_TICK_PERIOD_MS 100

#if !defined(TESTING)
static struct timer_task _animation_timer_task = {0};
static int _animation_current_frame = 0;

/**
 * Displays frames of the lock animation
 * at a regular rate until it's finished.
 * Leaves the last frame on the screen.
 */
static void _animation_timer_cb(const struct timer_task* const timer_task)
{
    (void)timer_task;
    if (_animation_current_frame == LOCK_ANIMATION_N_FRAMES) {
        /* End of the animation */
        return;
    }

    /* Draw the frame. */
    UG_ClearBuffer();
    position_t pos = {.left = (SCREEN_WIDTH - LOCK_ANIMATION_FRAME_WIDTH) / 2,
                      .top = (SCREEN_HEIGHT - LOCK_ANIMATION_FRAME_HEIGHT) / 2};
    dimension_t dim = {.width = LOCK_ANIMATION_FRAME_WIDTH, .height = LOCK_ANIMATION_FRAME_HEIGHT};
    in_buffer_t image = {.data = lock_animation_get_frame(_animation_current_frame),
                         .len = LOCK_ANIMATION_FRAME_SIZE};
    graphics_draw_image(&pos, &dim, &image);
    UG_SendBuffer();
    _animation_current_frame++;
}

/**
 * Sets up a timer animating a lock icon every 100ms
 */
static void _start_animation_timer(void)
{
    _animation_timer_task.interval = TIMEOUT_TICK_PERIOD_MS;
    _animation_timer_task.cb = _animation_timer_cb;
    _animation_timer_task.mode = TIMER_TASK_REPEAT;
    timer_stop(&TIMER_0);
    timer_add_task(&TIMER_0, &_animation_timer_task);
    _animation_current_frame = 0;
    timer_start(&TIMER_0);
}

static void _stop_animation_timer(void)
{
    timer_stop(&TIMER_0);
    timer_remove_task(&TIMER_0, &_animation_timer_task);
    timer_start(&TIMER_0);
}
#endif

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

#ifndef TESTING
    _start_animation_timer();
#endif
    bool unlock_result = keystore_unlock_bip39(data->mnemonic_passphrase);
#ifndef TESTING
    _stop_animation_timer();
#endif

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
