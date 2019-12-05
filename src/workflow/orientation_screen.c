// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include "orientation_screen.h"

#include <string.h>

#include "idle_workflow.h"
#include "workflow.h"

#include <hardfault.h>
#include <screen.h>
#include <ui/components/orientation_arrows.h>
#include <ui/screen_stack.h>
#include <ui/workflow_stack.h>
#include <util.h>

typedef struct {
    bool finished;
    bool upside_down;
} orientation_screen_data_t;

#if PLATFORM_BITBOX02 == 1
/**
 * Called when the "select orientation" screen is over.
 * Switch to the main view.
 */
static void _select_orientation_done(bool upside_down, void* cb_param)
{
    orientation_screen_data_t* data = ((workflow_t*)cb_param)->data;
    data->finished = true;
    data->upside_down = upside_down;
}

static void _orientation_screen_init(workflow_t* self)
{
    orientation_screen_data_t* data = malloc(sizeof(orientation_screen_data_t));
    if (!data) {
        Abort("malloc failed in _orientation_screen_init()");
    }
    data->finished = false;
    self->data = data;
    component_t* select_orientation = orientation_arrows_create(_select_orientation_done, self);
    ui_screen_stack_push(select_orientation);
}

static void _orientation_screen_cleanup(workflow_t* self)
{
    ui_screen_stack_pop();
    ui_screen_stack_cleanup();
    free(self);
}

static void _orientation_screen_spin(workflow_t* self)
{
    orientation_screen_data_t* data = self->data;
    if (data->finished) {
        if (data->upside_down) {
            screen_rotate();
        }
        workflow_stack_stop_workflow();
        workflow_stack_start_workflow(idle_workflow());
    }
}
#endif

workflow_t* orientation_screen(void)
{
#if PLATFORM_BITBOXBASE == 1
    return idle_workflow();
#elif PLATFORM_BITBOX02 == 1
    return workflow_allocate(
        _orientation_screen_init, _orientation_screen_cleanup, _orientation_screen_spin, sizeof(orientation_screen_data_t));
#endif
}
