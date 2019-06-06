#include "status.h"

#include <ui/components/status.h>
#include <ui/screen_stack.h>

void workflow_status_create(const char* msg)
{
    ui_screen_stack_push(status_create(msg, false, STATUS_DEFAULT_DELAY, ui_screen_stack_pop));
}
