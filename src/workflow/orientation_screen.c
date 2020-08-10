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

#include "blocking.h"

#include <screen.h>
#include <ui/components/orientation_arrows.h>
#include <ui/screen_stack.h>

static void _select_orientation_done(bool upside_down, void* cb_param)
{
    *(bool*)cb_param = upside_down;
    workflow_blocking_unblock();
}

void orientation_screen_blocking(void)
{
    bool upside_down;
    ui_screen_stack_push(orientation_arrows_create(_select_orientation_done, &upside_down));
    workflow_blocking_block();
    ui_screen_stack_pop_and_clean();
    if (upside_down) {
        screen_rotate();
    }
}
