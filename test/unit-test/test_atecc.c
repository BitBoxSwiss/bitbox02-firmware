// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <stdint.h>

// i2c_ecc.h intentionally hides the hardware interface in test builds. Expose its declarations
// while including atecc.c below.
#undef TESTING
#include <i2c_ecc.h>
#define TESTING

#include "../../src/atecc/atecc.c"

static uint8_t _read_result;
static uint8_t _response_length;
static uint32_t _received_capacity;

uint8_t i2c_ecc_read(uint8_t* rxdata, uint32_t rxlen)
{
    _received_capacity = rxlen;
    rxdata[0] = _response_length;
    return _read_result;
}

uint8_t i2c_ecc_write(uint8_t* txdata, uint32_t txlen)
{
    (void)txdata;
    (void)txlen;
    return 0;
}

uint8_t i2c_ecc_idle(void)
{
    return 0;
}

uint8_t i2c_ecc_sleep(void)
{
    return 0;
}

uint8_t i2c_ecc_wake(void)
{
    return I2C_ECC_WAKE;
}

static void _test_receive(uint8_t response_length, uint16_t capacity, ATCA_STATUS expected_status)
{
    uint8_t response[16] = {0};
    _read_result = 0;
    _response_length = response_length;
    _received_capacity = 0;

    uint16_t length = capacity;
    assert_int_equal(_receive(NULL, 0, response, &length), expected_status);
    assert_int_equal(_received_capacity, capacity);
    if (expected_status == ATCA_SUCCESS) {
        assert_int_equal(length, response_length);
    } else {
        assert_int_equal(length, capacity);
    }
}

static void test_receive_valid_length(void** state)
{
    (void)state;
    _test_receive(7, 16, ATCA_SUCCESS);
}

static void test_receive_oversized_length(void** state)
{
    (void)state;
    _test_receive(8, 7, ATCA_SMALL_BUFFER);
}

static void test_receive_undersized_length(void** state)
{
    (void)state;
    _test_receive(ATCA_RSP_SIZE_MIN - 1, 16, ATCA_RX_FAIL);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_receive_valid_length),
        cmocka_unit_test(test_receive_oversized_length),
        cmocka_unit_test(test_receive_undersized_length),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
