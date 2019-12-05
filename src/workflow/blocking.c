#include "blocking.h"

#include <hardfault.h>
#include <stddef.h>
#include <ui/screen_process.h>
#include <ui/workflow_stack.h>
#include <workflow/workflow.h>

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
        workflow_t* workflow = workflow_stack_top();
        if (!workflow) {
            Abort("NULL workflow in _run_blocking_ui");
        }
        screen_process();
        workflow->spin(workflow);
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

void workflow_blocking_unblock(void)
{
    _done = UNBLOCKED_NORMAL;
}

void workflow_blocking_unblock_force(void)
{
    _done = UNBLOCKED_FORCED;
}
