#include "status.h"

#include <ui/components/status.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>

static bool _done = true;
static bool _is_done(void)
{
    return _done;
}
static void _set_done(void)
{
    _done = true;
}

void workflow_status_create(const char* msg)
{
    _done = false;
    ui_screen_stack_push(status_create(msg, false, STATUS_DEFAULT_DELAY, _set_done));
    ui_screen_process(_is_done);
    ui_screen_stack_pop();
}
