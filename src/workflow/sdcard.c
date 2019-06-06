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

#include "sdcard.h"
#include "generated/hww.pb.h"
#include "workflow/workflow.h"

#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>

static bool _done = false;

static bool _is_done(void)
{
    return _done;
}

static void _continue(void)
{
    _done = true;
}

void sdcard_handle(const InsertRemoveSDCardRequest* insert_remove_sdcard)
{
    _done = false;
    bool pushed = false;
    if (insert_remove_sdcard->action == InsertRemoveSDCardRequest_SDCardAction_INSERT_CARD &&
        !workflow_get_interface_functions()->sd_card_inserted()) {
        component_t* screen = insert_sd_card_create(_continue);
        pushed = true;
        ui_screen_stack_push(screen);
    } else if (
        insert_remove_sdcard->action == InsertRemoveSDCardRequest_SDCardAction_INSERT_CARD &&
        workflow_get_interface_functions()->sd_card_inserted()) {
        _continue();
    } else if (
        insert_remove_sdcard->action == InsertRemoveSDCardRequest_SDCardAction_REMOVE_CARD &&
        workflow_get_interface_functions()->sd_card_inserted()) {
        component_t* screen = remove_sd_card_create(_continue);
        pushed = true;
        ui_screen_stack_push(screen);
    } else {
        _continue();
    }
    ui_screen_process(_is_done);
    if (pushed) {
        ui_screen_stack_pop();
    }
}
