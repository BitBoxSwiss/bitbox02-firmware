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

#include <string.h>
#ifndef TESTING
#include "driver_init.h"
#include "qtouch.h"
#endif
#include <workflow/show_mnemonic.h>
#include <workflow/workflow.h>

#include <wally_bip39.h>

#include <firmware_main_loop.h>

#include "hardfault.h"
#include "keystore.h"
#include "random.h"
#include "screen.h"
#include "sd.h"
#include "util.h"

#include "securechip/securechip.h"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-function"

uint32_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    screen_print_debug("Stack smashing detected", 0);
    while (1) {
    }
}

static bool _mock_get_bip_39_mnemonic(char** mnemonic)
{
    const char* wordlist =
        "flight donkey evolve skirt inspire balcony accident aisle walk vivid weasel region "
        "sadness immense index champion almost avocado castle chaos defense crystal device emotion";
    *mnemonic = strdup(wordlist);
    return true;
}

static bool _unlock(const char* password)
{
    (void)password;
    Abort("unlock shouldn't be called in this test case");
    return false;
}

static const uint8_t host_entropy[32] = "host-entropy";

static void _create_and_store_seed(const char* password)
{
    if (!keystore_create_and_store_seed(password, host_entropy)) {
        Abort("create_and_store_seed");
    }
}

int main(void)
{
    system_init();
    screen_init();
    qtouch_init();

    workflow_show_mnemonic_create();

    firmware_main_loop();
}

#pragma GCC diagnostic pop
