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
#include "firmware_main_loop.h"
#include "hardfault.h"
#include "inttypes.h"
#include "memory/mpu.h"
#include "memory/smarteeprom.h"
#include "qtouch.h"
#include "screen.h"

#include <stdarg.h>
#include <stdlib.h>
#include <ui/fonts/arial_fonts.h>
uint32_t __stack_chk_guard = 0;

static void _screen_print_debug(const char* message, int duration)
{
    char print[100];
    snprintf(print, sizeof(print), "%s", message);
    UG_ClearBuffer();
    UG_FontSelect(&font_font_a_9X9);
    UG_PutString(0, 0, print, false);
    UG_SendBuffer();
    delay_ms(duration);
}

static void _screen_sprintf_debug(int duration, const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    char print[100];
    // There is a bug in clang-tidy
    // See https://bugs.llvm.org/show_bug.cgi?id=41311
    vsnprintf(print, sizeof(print), fmt, args); // NOLINT
    va_end(args);
    _screen_print_debug(print, duration);
}

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    _screen_print_debug("Stack smashing detected", 0);
    while (1) {
    }
}

int main(void)
{
    system_init();
    screen_init();
    mpu_bitbox02_init();
    if (smarteeprom_is_enabled()) {
        smarteeprom_bb02_config();
        _screen_print_debug("SmartEEPROM enabled.\n", 2000);
        for (int i = 0; i < 5; ++i) {
            uint64_t number;
            uint64_t read_number;
            smarteeprom_read(2, sizeof(number), (uint8_t*)&number);
            number = (number == 0 ? (uint64_t)-1 : number - 1);
            smarteeprom_write(2, sizeof(number), (uint8_t*)&number);
            _screen_sprintf_debug(
                1000,
                "%d: SEESTAT 0x%08" PRIx32 ",\nwritten 0x%0x.\n",
                i,
                NVMCTRL->SEESTAT.reg,
                number);
            smarteeprom_read(2, sizeof(read_number), (uint8_t*)&read_number);
            _screen_sprintf_debug(
                1000,
                "%d: SEESTAT 0x%08" PRIx32 ",\nread back 0x%0x.\n",
                i,
                NVMCTRL->SEESTAT.reg,
                read_number);
        }
        _screen_print_debug("Disabling SmartEEPROM.\n", 2000);
        smarteeprom_disable();
        _screen_print_debug("SmartEEPROM disabled.\nRebooting.", 2000);
        _reset_mcu();
    } else {
        _screen_print_debug("SmartEEPROM disabled.\nEnabling.", 2000);
        smarteeprom_setup();
        _screen_print_debug("SmartEEPROM enabled.\nRebooting.", 2000);
        _reset_mcu();
    }
    return 0;
}
