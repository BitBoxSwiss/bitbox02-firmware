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

#include "blocking.h"
#include "workflow.h"

#include "hww.pb.h"

#include <sd.h>
#include <ui/components/sdcard.h>
#include <ui/screen_stack.h>

static void _unblock(void* param)
{
    (void)param;
    workflow_blocking_unblock();
}

void sdcard_handle(const InsertRemoveSDCardRequest* insert_remove_sdcard)
{
    bool inserted = sd_card_inserted();

    // No action required, already inserted (INSERT request) or not inserted (REMOVE request)
    if ((insert_remove_sdcard->action == InsertRemoveSDCardRequest_SDCardAction_INSERT_CARD &&
         inserted) ||
        (insert_remove_sdcard->action == InsertRemoveSDCardRequest_SDCardAction_REMOVE_CARD &&
         !inserted)) {
        return;
    }

    component_t* screen;
    if (insert_remove_sdcard->action == InsertRemoveSDCardRequest_SDCardAction_INSERT_CARD) {
        screen = sdcard_create(true, _unblock, NULL);
    } else if (insert_remove_sdcard->action == InsertRemoveSDCardRequest_SDCardAction_REMOVE_CARD) {
        screen = sdcard_create(false, _unblock, NULL);
    } else {
        return;
    }

    ui_screen_stack_push(screen);
    workflow_blocking_block();
    ui_screen_stack_pop();
}
