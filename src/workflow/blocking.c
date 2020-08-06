#include "blocking.h"

#include <hardfault.h>
#include <stddef.h>
#include <ui/screen_process.h>

static bool _done = true;

static bool _is_done(void)
{
    return _done;
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

void workflow_blocking_block(void)
{
    if (!_done) {
        Abort("workflow_blocking_block");
    }
    _done = false;
    _run_blocking_ui(_is_done);
}

void workflow_blocking_unblock(void)
{
    _done = true;
}
