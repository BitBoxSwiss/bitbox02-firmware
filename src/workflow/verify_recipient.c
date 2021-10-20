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

#include "verify_recipient.h"

#include "blocking.h"

#include <ui/components/confirm_transaction.h>
#include <ui/screen_stack.h>

static void _callback(bool result, void* param)
{
    *(bool*)param = result;
    workflow_blocking_unblock();
}

bool workflow_verify_recipient(const char* recipient, const char* amount)
{
    bool result = false;
    ui_screen_stack_push(confirm_transaction_address_create(amount, recipient, _callback, &result));
    workflow_blocking_block();
    ui_screen_stack_pop();
    return result;
}
