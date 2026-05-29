// SPDX-License-Identifier: Apache-2.0

#include "bootloader/bootloader_format.h"

#include <stdio.h>

void bootloader_format_pairing_code(char* out, size_t out_len, uint32_t pairing_code)
{
    snprintf(out, out_len, "%06u", (unsigned)pairing_code);
}

void bootloader_format_progress(char* out, size_t out_len, float progress)
{
    snprintf(out, out_len, "%2d%%", (int)(100 * progress));
}

void bootloader_format_hash_multiline(char* out, size_t out_len, const char* hash_hex)
{
    snprintf(
        out,
        out_len,
        "%.16s\n%.16s\n%.16s\n%.16s",
        &hash_hex[0],
        &hash_hex[16],
        &hash_hex[32],
        &hash_hex[48]);
}

void bootloader_format_timer(char* out, size_t out_len, uint8_t seconds)
{
    snprintf(out, out_len, "%ds", seconds);
}

void bootloader_format_unknown_command(char* out, size_t out_len, uint8_t command)
{
    snprintf(out, out_len, "Command: %u unknown", command);
}
