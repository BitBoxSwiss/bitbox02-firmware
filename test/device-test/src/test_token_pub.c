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

#include "driver_init.h"
#include "hardfault.h"
#include "platform_init.h"
#include "qtouch.h"
#include "screen.h"
#include "util.h"
#include <firmware_main_loop.h>
#include <string.h>
#include <workflow/verify_pub.h>

#include "../../src/apps/eth/eth_params.c"

const char* data[] = {
    "Bitcoin Legacy",
    "Bitcoin bech32",
    "Ethereum",
    "Basic Attention Token",
    "Beluga Banking Infrastructure Token",
    "BitCapitalVendorToken",
    "BLOCKMASON CREDIT PROTOCOL TOKEN",
};

uint32_t __stack_chk_guard = 0;

int main(void)
{
    init_mcu();
    system_init();
    platform_init();
    screen_init();
    qtouch_init();
    const char* addr = "cafebabecafebabecafebabecafebabe";
    for (size_t i = 0; i < sizeof(data) / sizeof(*data); ++i) {
        workflow_verify_pub(data[i], addr);
    }

    for (size_t i = 0; i < sizeof(_erc20_params) / sizeof(*_erc20_params); ++i) {
        workflow_verify_pub(_erc20_params[i].name, addr);
    }
}
