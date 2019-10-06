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
#include <ui/components/insert_sd_card.h>
#include <ui/components/remove_sd_card.h>
#include <ui/screen_stack.h>

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
        screen = insert_sd_card_create(workflow_blocking_unblock);
    } else if (insert_remove_sdcard->action == InsertRemoveSDCardRequest_SDCardAction_REMOVE_CARD) {
        screen = remove_sd_card_create(workflow_blocking_unblock);
    } else {
        return;
    }

    ui_screen_stack_push(screen);
    bool blocking_result = workflow_blocking_block();
    ui_screen_stack_pop();
    if (!blocking_result) {
        // No meaningful error handling here.
    }
}
