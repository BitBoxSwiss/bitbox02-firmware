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

#include "common_main.h"
#include "drivers/driver_init.h"
#include "hardfault.h"
#include "qtouch.h"
#include "screen.h"
#include "ui/screen_process.h"
#include "util.h"
#include "workflow/workflow.h"

uint32_t __stack_chk_guard = 0;

int main(void)
{
    init_mcu();
    system_init();
    __stack_chk_guard = common_stack_chk_guard();
    screen_init();
    screen_splash();
    qtouch_init();
    common_main();
    traceln("%s", "Device initialized");
    workflow_change_state(WORKFLOW_STATE_CHOOSE_ORIENTATION);
    ui_screen_process(NULL);
}
