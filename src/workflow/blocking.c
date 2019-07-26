#include "blocking.h"

#include <hardfault.h>
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

bool workflow_blocking_block(void)
{
    if (!_is_done()) {
        Abort("workflow_blocking_block");
    }
    _done = BLOCKED;
    ui_screen_process(_is_done);
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
