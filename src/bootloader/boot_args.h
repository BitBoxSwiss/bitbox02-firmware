// SPDX-License-Identifier: Apache-2.0

#ifndef BOOT_ARGS_H
#define BOOT_ARGS_H

#include "util.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define BOOT_ARGS_ADDR (0x20000000U)
#define BOOT_ARGS_LEN (512U)
#define BOOT_ARGS_MAGIC (0xB007A265U)
#define BOOTCMD_BOOTLOADER_WAIT (1)
#define BOOT_ARGS_FLAG_UPSIDE_DOWN (1U << 0)

typedef union {
    uint8_t raw[BOOT_ARGS_LEN - 3U * sizeof(uint32_t)];
} boot_args_command_args_t;

typedef struct {
    uint32_t magic;
    uint32_t command;
    uint32_t flags;
    boot_args_command_args_t command_args;
} boot_args_t;

_Static_assert(sizeof(boot_args_t) == BOOT_ARGS_LEN, "boot_args_t must occupy boot_args area");
_Static_assert(offsetof(boot_args_t, magic) == 0, "boot_args magic offset changed");
_Static_assert(offsetof(boot_args_t, command) == 4, "boot_args command offset changed");
_Static_assert(offsetof(boot_args_t, flags) == 8, "boot_args flags offset changed");
_Static_assert(offsetof(boot_args_t, command_args) == 12, "boot_args command_args offset changed");

static inline volatile boot_args_t* boot_args_ram(void)
{
    return (volatile boot_args_t*)BOOT_ARGS_ADDR;
}

static inline bool boot_args_is_valid(void)
{
    return boot_args_ram()->magic == BOOT_ARGS_MAGIC;
}

static inline bool boot_args_is_bootloader_wait(void)
{
    volatile boot_args_t* args = boot_args_ram();
    return boot_args_is_valid() && args->command == BOOTCMD_BOOTLOADER_WAIT;
}

static inline bool boot_args_is_upside_down(void)
{
    return (boot_args_ram()->flags & BOOT_ARGS_FLAG_UPSIDE_DOWN) != 0;
}

static inline void boot_args_write_bootloader_wait(bool upside_down)
{
    volatile boot_args_t* args = boot_args_ram();
    args->command = 0;
    args->magic = BOOT_ARGS_MAGIC;
    args->flags = upside_down ? BOOT_ARGS_FLAG_UPSIDE_DOWN : 0;
    args->command = BOOTCMD_BOOTLOADER_WAIT;
}

static inline void boot_args_clear_command(void)
{
    boot_args_ram()->command = 0;
}

#endif
