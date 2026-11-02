// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <da14531/da14531_protocol.h>
#include <fake_uart.h>
#include <rust/rust.h>

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define STX 0x02
#define ACK 0x06

static const uint8_t _firmware[] = {0x42, 0x69, 0x74, 0x42, 0x6f, 0x78, 0x30, 0x32};
static const uint8_t _firmware_checksum = 0xa5;
static uint8_t* _firmware_allocation = NULL;
static size_t _firmware_free_count = 0;

bool __wrap_memory_spi_get_active_ble_firmware(
    uint8_t** firmware_out,
    size_t* size_out,
    uint8_t* checksum_out)
{
    assert_null(_firmware_allocation);
    _firmware_allocation = malloc(sizeof(_firmware));
    assert_non_null(_firmware_allocation);
    memcpy(_firmware_allocation, _firmware, sizeof(_firmware));
    *firmware_out = _firmware_allocation;
    *size_out = sizeof(_firmware);
    *checksum_out = _firmware_checksum;
    return true;
}

void __real_free(void* ptr);

void __wrap_free(void* ptr)
{
    if (ptr == _firmware_allocation) {
        _firmware_free_count++;
    }
    __real_free(ptr);
}

static void _poll(struct RustByteQueue* queue, uint8_t* input, uint16_t input_len)
{
    assert_null(da14531_protocol_poll(input, &input_len, NULL, queue));
    assert_int_equal(input_len, 0);
}

static void _poll_byte(struct RustByteQueue* queue, uint8_t input)
{
    _poll(queue, &input, 1);
}

static void _poll_empty(struct RustByteQueue* queue)
{
    uint8_t input = 0;
    _poll(queue, &input, 0);
}

static uint8_t _queue_get(struct RustByteQueue* queue)
{
    uint8_t value = 0;
    assert_true(rust_bytequeue_get(queue, &value));
    return value;
}

static void test_firmware_buffer_retained_until_tx_complete(void** state)
{
    (void)state;
    fake_uart_reset();
    _firmware_allocation = NULL;
    _firmware_free_count = 0;

    struct RustByteQueue* queue = rust_bytequeue_init(16);
    assert_non_null(queue);
    da14531_protocol_init();

    // Move the serial parser into loader mode, load the image, and request it.
    _poll_byte(queue, STX);
    _poll_empty(queue);
    _poll_byte(queue, STX);
    _poll_empty(queue);

    assert_int_equal(rust_bytequeue_num(queue), 3);
    assert_int_equal(_queue_get(queue), 0x01);
    assert_int_equal(_queue_get(queue), sizeof(_firmware) & 0xff);
    assert_int_equal(_queue_get(queue), sizeof(_firmware) >> 8);

    _poll_byte(queue, ACK);
    assert_ptr_equal(fake_uart_tx_buffer(), _firmware_allocation);
    assert_int_equal(fake_uart_tx_buffer_len(), sizeof(_firmware));

    // A peer can provide the checksum while the async driver still retains the allocation.
    _poll_byte(queue, _firmware_checksum);
    assert_int_equal(_firmware_free_count, 0);
    assert_int_equal(rust_bytequeue_num(queue), 0);

    for (size_t i = 0; i < sizeof(_firmware); i++) {
        uint8_t transmitted = 0;
        assert_true(fake_uart_transmit_next(&transmitted));
        assert_int_equal(transmitted, _firmware[i]);
    }
    uint8_t transmitted = 0;
    assert_false(fake_uart_transmit_next(&transmitted));
    assert_int_equal(_firmware_free_count, 0);

    fake_uart_tx_complete();
    _poll_empty(queue);
    assert_int_equal(_firmware_free_count, 1);
    assert_int_equal(rust_bytequeue_num(queue), 1);
    assert_int_equal(_queue_get(queue), ACK);

    _poll_empty(queue);
    assert_int_equal(_firmware_free_count, 1);
    _firmware_allocation = NULL;
    assert_true(rust_bytequeue_free(queue));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_firmware_buffer_retained_until_tx_complete),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
