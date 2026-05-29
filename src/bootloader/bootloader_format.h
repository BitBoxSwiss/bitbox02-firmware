// SPDX-License-Identifier: Apache-2.0

#ifndef BOOTLOADER_FORMAT_H
#define BOOTLOADER_FORMAT_H

#include <stddef.h>
#include <stdint.h>

void bootloader_format_pairing_code(char* out, size_t out_len, uint32_t pairing_code);
void bootloader_format_progress(char* out, size_t out_len, float progress);
void bootloader_format_hash_multiline(char* out, size_t out_len, const char* hash_hex);
void bootloader_format_timer(char* out, size_t out_len, uint8_t seconds);
void bootloader_format_unknown_command(char* out, size_t out_len, uint8_t command);

#endif
