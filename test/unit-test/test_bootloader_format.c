// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <cmocka.h>

#include "bootloader/bootloader_format.h"

static void test_pairing_code(void** state)
{
    (void)state;
    char out[10];

    bootloader_format_pairing_code(out, sizeof(out), 42);
    assert_string_equal(out, "000042");

    bootloader_format_pairing_code(out, sizeof(out), 999999);
    assert_string_equal(out, "999999");
}

static void test_progress(void** state)
{
    (void)state;
    char out[5];

    bootloader_format_progress(out, sizeof(out), 0.01f);
    assert_string_equal(out, " 1%");

    bootloader_format_progress(out, sizeof(out), 0.42f);
    assert_string_equal(out, "42%");

    bootloader_format_progress(out, sizeof(out), 1.0f);
    assert_string_equal(out, "100%");
}

static void test_hash_multiline(void** state)
{
    (void)state;
    const char* hash_hex =
        "000102030405060708090a0b0c0d0e0f"
        "101112131415161718191a1b1c1d1e1f";
    char out[4 * 16 + 3 + 1];

    bootloader_format_hash_multiline(out, sizeof(out), hash_hex);
    assert_string_equal(
        out,
        "0001020304050607\n"
        "08090a0b0c0d0e0f\n"
        "1011121314151617\n"
        "18191a1b1c1d1e1f");
}

static void test_timer(void** state)
{
    (void)state;
    char out[4];

    bootloader_format_timer(out, sizeof(out), 9);
    assert_string_equal(out, "9s");

    bootloader_format_timer(out, sizeof(out), 10);
    assert_string_equal(out, "10s");
}

static void test_ble_firmware_version(void** state)
{
    (void)state;
    char out[sizeof("ble: 65535 (00112233)")];
    const uint8_t hash[] = {
        0x00,
        0x11,
        0x22,
        0x33,
        0x44,
    };

    bootloader_format_ble_firmware_version(out, sizeof(out), 65535, hash);
    assert_string_equal(out, "ble: 65535 (00112233)");
}

static void test_unknown_command(void** state)
{
    (void)state;
    char out[100];

    bootloader_format_unknown_command(out, sizeof(out), 42);
    assert_string_equal(out, "Command: 42 unknown");
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_pairing_code),
        cmocka_unit_test(test_progress),
        cmocka_unit_test(test_hash_multiline),
        cmocka_unit_test(test_timer),
        cmocka_unit_test(test_ble_firmware_version),
        cmocka_unit_test(test_unknown_command),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
