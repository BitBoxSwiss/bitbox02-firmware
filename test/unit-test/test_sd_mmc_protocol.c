// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <cmocka.h>

#include "sd_mmc_protocol.h"

static void test_decode_transfer_speed_valid(void** state)
{
    (void)state;
    uint32_t speed;

    assert_true(sd_mmc_decode_transfer_speed(0x08, &speed));
    assert_int_equal(speed, 100000);

    assert_true(sd_mmc_decode_transfer_speed(0x32, &speed));
    assert_int_equal(speed, 25000000);

    assert_true(sd_mmc_decode_transfer_speed(0x7B, &speed));
    assert_int_equal(speed, 800000000);
}

static void test_decode_transfer_speed_reserved_unit(void** state)
{
    (void)state;

    for (uint8_t unit_code = 4; unit_code <= 7; unit_code++) {
        uint32_t speed = 0xA5A5A5A5;
        assert_false(sd_mmc_decode_transfer_speed((1 << 3) | unit_code, &speed));
        assert_int_equal(speed, 0xA5A5A5A5);
    }
}

static void test_decode_transfer_speed_reserved_multiplier(void** state)
{
    (void)state;

    for (uint8_t unit_code = 0; unit_code <= 3; unit_code++) {
        uint32_t speed = 0xA5A5A5A5;
        assert_false(sd_mmc_decode_transfer_speed(unit_code, &speed));
        assert_int_equal(speed, 0xA5A5A5A5);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_decode_transfer_speed_valid),
        cmocka_unit_test(test_decode_transfer_speed_reserved_unit),
        cmocka_unit_test(test_decode_transfer_speed_reserved_multiplier),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
