// SPDX-License-Identifier: Apache-2.0

#ifndef BOOTLOADER_FORMAT_H
#define BOOTLOADER_FORMAT_H

#include <stddef.h>
#include <stdint.h>

/**
 * Format the six-digit BLE pairing code.
 *
 * pairing_code must be at most 999999. out_len must be at least
 * sizeof("000000").
 */
void bootloader_format_pairing_code(char* out, size_t out_len, uint32_t pairing_code);

/**
 * Format progress as a percentage.
 *
 * progress must be between 0 and 1. out_len must be at least sizeof("100%").
 */
void bootloader_format_progress(char* out, size_t out_len, float progress);

/**
 * Format a 64-character hex hash as four newline-separated 16-character lines.
 *
 * out_len must be at least 4 * 16 + 3 + 1.
 */
void bootloader_format_hash_multiline(char* out, size_t out_len, const char* hash_hex);

/**
 * Format a timer value with an "s" suffix.
 *
 * seconds must be at most 99. out_len must be at least sizeof("99s").
 */
void bootloader_format_timer(char* out, size_t out_len, uint8_t seconds);

/**
 * Format an unknown bootloader command message.
 *
 * out_len must be at least sizeof("Command: 255 unknown").
 */
void bootloader_format_unknown_command(char* out, size_t out_len, uint8_t command);

#endif
