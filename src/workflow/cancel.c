#include "cancel.h"

#include "blocking.h"
#include "confirm.h"
#include "status.h"

#include <ui/screen_stack.h>

static bool _cancel_pressed = false;
static bool _force = false;

void workflow_cancel(void)
{
    if (!_cancel_pressed) {
        _cancel_pressed = true;
        workflow_blocking_unblock();
    }
}

void workflow_cancel_force(void)
{
    if (!_cancel_pressed) {
        _cancel_pressed = true;
        _force = true;
        workflow_blocking_unblock();
    }
}

bool workflow_cancel_run(const char* title, component_t* component)
{
    ui_screen_stack_push(component);
    while (true) {
        _cancel_pressed = false;
        _force = false;
        workflow_blocking_block();
        if (_cancel_pressed) {
            if (!_force) {
                const confirm_params_t params = {
                    .title = title,
                    .body = "Do you really\nwant to cancel?",
                };
                if (!workflow_confirm_blocking(&params)) {
                    continue;
                }
            }
            ui_screen_stack_pop();
            if (!_force) {
                workflow_status_blocking("Cancelled", false);
            }
            return false;
        }
        ui_screen_stack_pop();
        return true;
    }
}
