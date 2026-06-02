// SPDX-License-Identifier: Apache-2.0

#include "bootloader/bootloader_format.h"

#include <rust/rust.h>
#include <string.h>
#include <util.h>
#include <utils_assert.h>

void bootloader_format_pairing_code(char* out, size_t out_len, uint32_t pairing_code)
{
    ASSERT(out_len >= sizeof("000000"));
    rust_format_uint(rust_util_bytes_mut((uint8_t*)out, out_len), pairing_code, 6, '0');
}

void bootloader_format_progress(char* out, size_t out_len, float progress)
{
    ASSERT(out_len >= sizeof("100%"));
    size_t out_pos = rust_format_uint(
        rust_util_bytes_mut((uint8_t*)out, out_len - 1), (uint32_t)(100 * progress), 2, ' ');
    out[out_pos++] = '%';
    out[out_pos] = '\0';
}

void bootloader_format_hash_multiline(char* out, size_t out_len, const char* hash_hex)
{
    ASSERT(out_len >= 4 * 16 + 3 + 1);
    (void)out_len;
    for (size_t i = 0; i < 4; i++) {
        memcpy(&out[i * 17], &hash_hex[i * 16], 16);
        out[i * 17 + 16] = i == 3 ? '\0' : '\n';
    }
}

void bootloader_format_timer(char* out, size_t out_len, uint8_t seconds)
{
    ASSERT(out_len >= sizeof("99s"));
    size_t out_pos =
        rust_format_uint(rust_util_bytes_mut((uint8_t*)out, out_len - 1), seconds, 1, '0');
    out[out_pos++] = 's';
    out[out_pos] = '\0';
}

void bootloader_format_ble_firmware_version(
    char* out,
    size_t out_len,
    uint16_t version,
    const uint8_t* hash)
{
    ASSERT(out_len >= sizeof("ble: 65535 (00112233)"));

    util_strlcpy(out, "ble: ", out_len);
    size_t out_pos = strlen(out);
    out_pos += rust_format_uint(
        rust_util_bytes_mut((uint8_t*)&out[out_pos], out_len - out_pos), version, 1, '0');
    util_strlcpy(&out[out_pos], " (", out_len - out_pos);
    out_pos = strlen(out);
    util_uint8_to_hex(hash, 4, &out[out_pos]);
    out_pos += 8;
    util_strlcpy(&out[out_pos], ")", out_len - out_pos);
}

void bootloader_format_unknown_command(char* out, size_t out_len, uint8_t command)
{
    const char prefix[] = "Command: ";
    const char suffix[] = " unknown";

    ASSERT(out_len >= sizeof("Command: 255 unknown"));
    size_t out_pos = sizeof(prefix) - 1;
    memcpy(out, prefix, out_pos);
    out_pos += rust_format_uint(
        rust_util_bytes_mut((uint8_t*)&out[out_pos], out_len - out_pos), command, 1, '0');
    memcpy(&out[out_pos], suffix, sizeof(suffix));
}
