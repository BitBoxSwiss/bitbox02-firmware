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
#include "flags.h"
#include "hardfault.h"
#include "inttypes.h"
#include "memory/mpu.h"
#include "qtouch.h"
#include "screen.h"
#include "ui/oled/oled.h"

#include <stdarg.h>
#include <stdlib.h>
#include <string.h>
#include <ui/fonts/arial_fonts.h>

uint32_t __stack_chk_guard = 0;

static void _screen_print_debug(const char* message, int duration)
{
    char print[300];
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
    char print[300];
    // There is a bug in clang-tidy
    // See https://bugs.llvm.org/show_bug.cgi?id=41311
    vsnprintf(print, sizeof(print), fmt, args); // NOLINT
    va_end(args);
    _screen_print_debug(print, duration);
}

/**
 * This flag will be set to 0
 * before performing a risky memory access.
 * MemManage_Handler will set it to one if invoked.
 */
__attribute__((aligned(4))) static volatile bool _memmanage_called;

/**
 * This flag will be set to 0
 * before performing a risky memory access.
 * MemManage_Handler will set it to one if invoked.
 */
__attribute__((aligned(4))) static volatile bool _hardfault_called;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    _screen_print_debug("Stack smashing detected", 0);
    while (1)
        ;
}

static UG_GUI guioled; // Global GUI structure for OLED screen

/** Prints a list of blocks/pages to a string. */
static void list_to_string(char* buf, size_t buf_size, uint8_t* numbers, size_t n_numbers)
{
    if (n_numbers == 0) {
        snprintf(buf, buf_size, "<empty>");
        return;
    }
    size_t pos = 0;
    // Compose the list of memory errors.
    for (size_t i = 0; i < n_numbers; ++i) {
        size_t this_chunk = snprintf(
            buf + pos, buf_size - pos, "%u (%u),", numbers[i], numbers[i] * FLASH_ERASE_PAGE_NUM);
        if (this_chunk + pos >= buf_size) {
            memcpy(buf + buf_size - 4, "...", 4);
            break;
        }
        pos += this_chunk;
    }
}

/** Keep track of which blocks have generated a MPU violation. */
#define N_FLASH_PAGES (2048)
#define N_BLOCKS (N_FLASH_PAGES / FLASH_ERASE_PAGE_NUM)

int main(void)
{
    init_mcu();
    system_init();
    oled_init();
    UG_Init(
        &guioled,
        (void (*)(UG_S16, UG_S16, UG_COLOR))oled_set_pixel,
        &font_font_a_9X9,
        SCREEN_WIDTH,
        SCREEN_HEIGHT);
    UG_ClearBuffer();
    UG_SendBuffer();

    mpu_bitbox02_init();

    /* Dummy variable in which to load our data. */
    volatile uint64_t read_value;

    uint8_t mem_errors[N_BLOCKS];
    size_t n_mem_errors = 0;

    /** Keep track of which blocks have generated a hard fault. */
    uint8_t hardfault_errors[N_BLOCKS];
    size_t n_hardfault_errors = 0;

    for (uint8_t i = 0; i < N_BLOCKS; ++i) {
        _screen_sprintf_debug(
            1, "Attempting to access:\n - Block #%u\n - Page #%u)", i, i * FLASH_ERASE_PAGE_NUM);

        /* The flags will be left to 0 on success. */
        _memmanage_called = 0;
        _hardfault_called = 0;
        uintptr_t base_addr = i * FLASH_PAGE_SIZE * FLASH_ERASE_PAGE_NUM;
        uint64_t* base_addr_ptr = (uint64_t*)base_addr;

// Avoid linting this, clang cries at what we're doing.
#ifndef __clang_analyzer__
        read_value = *base_addr_ptr;
        (void)read_value;
#else
        (void)base_addr_ptr;
#endif

        if (_memmanage_called) {
            mem_errors[n_mem_errors] = i;
            n_mem_errors++;
        } else if (_hardfault_called) {
            hardfault_errors[n_hardfault_errors] = i;
            n_hardfault_errors++;
        }
    }

    /* Format the errors we've found. */
    char mem_err_list[100];
    char hard_err_list[100];
    list_to_string(mem_err_list, 100, mem_errors, n_mem_errors);
    list_to_string(hard_err_list, 100, hardfault_errors, n_hardfault_errors);
    _screen_sprintf_debug(
        0, "Failed reads: %s\nHard faults: %s\nHave a nice day :)", mem_err_list, hard_err_list);
    while (1)
        ;
}

/**
 * MPU fault handler.
 *
 * This function will set the _memmanage_called variable
 * to 1, and then return to the instruction following the one
 * that causes the MPU fault.
 *
 * This is done by modifying (+4 - i.e. +1 instruction)
 * the return value stored in the stack (at SP+24).
 *
 * See Cortex-M3 Devices Generic User Guide, Section 2.3.7,
 * for details of how the stack is filled on exception entrance.
 *
 * Note that __attribute__((naked)) will remove all boilerplate
 * from the function code, so we are sure of how the stack looks
 * like for our code, but at the same time we have to handle returning
 * from the function manually.
 */
__attribute__((naked)) void MemManage_Handler(void)
{
    asm volatile(
        "push {r1}\n"
        /*
         * Use 28 as the stack offset as we've pushed R1
         * on top of it.
         */
        "ldr r1, [sp, #28]\n"
        "add r1, r1, #4\n"
        "str r1, [sp, #28]\n"
        /* Now set the _memmanage_called flag. */
        "mov r1, #1\n"
        "push {r2}\n"
        "ldr r2, =_memmanage_called\n"
        "strb r1, [r2]\n"
        /* Return */
        "pop {r2}\n"
        "pop {r1}\n"
        "bx lr\n");
}

/**
 * Hard fault handler.
 *
 * This function will set the _hardfault_called flag.
 *
 * See comments in MemManage_Handler for the details.
 */
__attribute__((naked)) void HardFault_Handler(void)
{
    asm volatile(
        "push {r1}\n"
        "ldr r1, [sp, #28]\n"
        "add r1, r1, #4\n"
        "str r1, [sp, #28]\n"
        "mov r1, #1\n"
        "push {r2}\n"
        "ldr r2, =_hardfault_called\n"
        "strb r1, [r2]\n"
        "pop {r2}\n"
        "pop {r1}\n"
        "bx lr\n");
}
