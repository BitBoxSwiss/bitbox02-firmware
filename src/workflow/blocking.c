#include "blocking.h"

#include <hardfault.h>
#include <stddef.h>
#include <ui/screen_process.h>

typedef enum {
    BLOCKED,
    UNBLOCKED_NORMAL,
    UNBLOCKED_FORCED,
} _done_t;

static _done_t _done = UNBLOCKED_NORMAL;

static bool _is_done(void)
{
    return _done != BLOCKED;
}

/**
 * Process screen, gestures, in a loop.
 * @param[in] is_done Runs until is_done().
 *            This should return true if and only if this GUI should
 *            terminate.
 */
static void _run_blocking_ui(bool (*is_done)(void))
{
    if (is_done == NULL) {
        Abort("is_done function\nis NULL.");
    }
    while (!is_done()) {
        screen_process();
    }
}

/**
 * Process screen, gestures, in a loop with timeout.
 * @param[in] is_done
 * @param[in] on_timeout called when timeout occurs
 * @param[in] timeout number of event loop cycles until timeout
 */
static void _run_blocking_ui_with_timeout(
    bool (*is_done)(void),
    void (*on_timeout)(void),
    uint32_t timeout)
{
    if (!is_done) {
        Abort("is_done function\nis NULL.");
    }
    if (!on_timeout) {
        Abort("on_timeout function\nis NULL.");
    }
    uint32_t timeout_cnt = 0;
    while (!is_done()) {
        if (on_timeout != NULL && timeout_cnt > timeout) {
            on_timeout();
        }
        timeout_cnt += 1;
        screen_process();
    }
}

bool workflow_blocking_block(void)
{
    if (!_is_done()) {
        Abort("workflow_blocking_block");
    }
    _done = BLOCKED;
    _run_blocking_ui(_is_done);
    return _done == UNBLOCKED_NORMAL;
}

bool workflow_blocking_block_with_timeout(uint32_t timeout)
{
    if (!_is_done()) {
        Abort("workflow_blocking_block");
    }
    _done = BLOCKED;
    _run_blocking_ui_with_timeout(_is_done, workflow_blocking_unblock_force, timeout);
    return _done == UNBLOCKED_NORMAL;
}

void workflow_blocking_unblock(void)
{
    _done = UNBLOCKED_NORMAL;
}

void workflow_blocking_unblock_force(void)
{
    _done = UNBLOCKED_FORCED;
}
