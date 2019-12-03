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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include <cmocka.h>

#include <usart/usart_frame.h>
#include <usb/usb_processing.h>

/**
 * Request to process a complete incoming USB packet.
 */
bool __wrap_usb_processing_enqueue(
    struct usb_processing* ctx,
    const uint8_t* buf,
    size_t length,
    uint8_t cmd,
    uint32_t cid)
{
    check_expected(ctx);
    if (length > 0) {
        check_expected(buf);
    }
    check_expected(length);
    check_expected(cmd);
    (void)cid;
    return (bool)mock();
}

uint32_t checksum_v1(uint8_t endpoint, uint8_t* buf, size_t payload_len)
{
    uint32_t cs = (endpoint << 8) | 0x01;
    uint16_t* buf16 = (uint16_t*)buf;
    for (size_t i = 0; i < payload_len / 2; ++i) {
        cs += *buf16;
        buf16++;
    }
    if (payload_len % 2 != 0) {
        cs += buf[payload_len - 1];
    }
    while (cs > 0xFFFFU) {
        cs = (cs >> 16) + (cs & 0xFFFFU);
    }
    return cs;
}

static uint8_t* _malloc_or_die(size_t length)
{
    uint8_t* result = (uint8_t*)malloc(length);
    if (!result) {
        fprintf(stderr, "Malloc failed!");
        abort();
    }
    return result;
}

static uint8_t* _create_payload_buf(size_t length)
{
    return _malloc_or_die(length);
}

static uint8_t* _create_msg_buf(size_t length)
{
    /* Lazily allocate 2x length, that's the maximum size a frame of size length can have. */
    return _malloc_or_die(2 * length + 9);
}

static uint8_t* _insert_in_buf(uint8_t* pos, uint8_t to_put)
{
    if (to_put == 0x7E || to_put == 0x7D) {
        *pos = 0x7D;
        *(pos + 1) = to_put ^ 0x20;
        pos += 2;
    } else {
        *pos = to_put;
        pos++;
    }
    return pos;
}

/**
 * Encodes a random message of total given length into a buffer.
 * @param[in] buf Buffer to store the encoded message in. It should be 2 * length + 9 bytes wide.
 * @param[in] payload Buffer containing the payload to encode.
 * @param[in] length Length of the payload to be encoded.
 * @param[out] Resulting size of the encoded message that has been stored into buf.
 */
static size_t _encode_msg(uint8_t* buf, uint8_t endpoint, uint8_t* payload, size_t length)
{
    buf[0] = 0x7E;
    /* Put the version in first. */
    buf[1] = 1;
    uint8_t* pos = buf + 2;
    pos = _insert_in_buf(pos, endpoint);
    /* Now fill with random bytes. */
    for (size_t i = 0; i < length; ++i) {
        pos = _insert_in_buf(pos, payload[i]);
    }
    uint16_t cs = checksum_v1(endpoint, payload, length);
    uint8_t* cs_bytes = (uint8_t*)&cs;
    pos = _insert_in_buf(pos, cs_bytes[0]);
    pos = _insert_in_buf(pos, cs_bytes[1]);
    *pos = 0x7E;
    pos++;
    return pos - buf;
}

/**
 * Fills a buffer with random bytes.
 */
static void _random_payload(uint8_t* buf, size_t length)
{
    for (size_t i = 0; i < length; ++i) {
        buf[i] = rand();
    }
}

/**
 * Sends a buffer to the USART frame parser.
 *
 * Randomly splits the buffer into multiple segments.
 */
static void _tx_buffer(uint8_t* buf, size_t length)
{
    /* Maybe split the message into two calls to usart_frame_process_rx. */
    bool split = (rand() % 2 == 0) && (length > 1);
    if (split) {
        size_t split_point = rand() % (length - 1) + 1;
        usart_frame_process_rx(buf, split_point);
        usart_frame_process_rx(buf + split_point, length - split_point);
    } else {
        usart_frame_process_rx(buf, length);
    }
}

/**
 * Sends a valid random message to the parser.
 *
 * Checks that the message has been parsed correctly.
 *
 * @param[in] msg_len Length of the payload to put into the message.
 */
static void exchange_msg(size_t msg_len, bool is_valid_msg)
{
    uint8_t* payload = _create_payload_buf(msg_len);
    uint8_t* msg = _create_msg_buf(msg_len);
    uint8_t endpoint = rand();
    _random_payload(payload, msg_len);
    size_t bytes_to_send = _encode_msg(msg, endpoint, payload, msg_len);
    if (is_valid_msg) {
        expect_memory(__wrap_usb_processing_enqueue, buf, payload, msg_len);
        expect_value(__wrap_usb_processing_enqueue, ctx, usb_processing_hww());
        expect_value(__wrap_usb_processing_enqueue, length, msg_len);
        expect_value(__wrap_usb_processing_enqueue, cmd, endpoint);
        will_return(__wrap_usb_processing_enqueue, true);
    }
    _tx_buffer(msg, bytes_to_send);
    free(msg);
    free(payload);
}

static size_t _normal_msg_size(void)
{
    return (rand() % USART_FRAME_MAX_PAYLOAD_LEN);
}

static void exchange_normal_msg(void)
{
    size_t msg_len = _normal_msg_size();
    exchange_msg(msg_len, true);
}

#define N_NORMAL_MSG_TESTS (100)
static void test_normal_msgs(void** state)
{
    (void)state;
    srand(0x12346);
    for (int i = 0; i < N_NORMAL_MSG_TESTS; ++i) {
        exchange_normal_msg();
    }
}

static size_t _long_msg_size(void)
{
    return (rand() % 5000) + USART_FRAME_MAX_DATA_LEN;
}

#define N_LONG_MSG_TESTS (100)
static void exchange_long_msg(void)
{
    size_t msg_len = _long_msg_size();
    exchange_msg(msg_len, false);
}

static void test_long_msgs(void** state)
{
    (void)state;
    srand(0x12345);
    for (int i = 0; i < N_LONG_MSG_TESTS; ++i) {
        exchange_long_msg();
    }
}

#define N_SHORT_MSG_TESTS (100)
static void test_short_msgs(void** state)
{
    (void)state;
    uint8_t short_msgs[] = {0x7E,
                            0x01,
                            0x7E,
                            0x7D,
                            0x5E,
                            0x7E,
                            0x7E,
                            0x7E,
                            0x7D,
                            0x5D,
                            0x7E,
                            0x00,
                            0x00,
                            0x00,
                            0x7E,
                            0x01,
                            0x00,
                            0x7E};
    usart_frame_process_rx(short_msgs, sizeof(short_msgs));
}

/**
 * Send a list of hard-coded test strings,
 * check that they are decoded properly.
 */
static void test_basic(void** state)
{
    const char* encoded[] = {"\x7E\x01\x42Hello\x25\x14\x7E",
                             "\x7E\x01\x00\x7D\x5D\x7D\x5D\x7D\x5E\x7D\x5E\xFC\xFB\x7E",
                             "\x7E\x01\x05\x01\x05\x7E"};
    /* Can't use strlen() because of NULL bytes... */
    int encoded_len[] = {11, 14, 6};
    const char* payload[] = {"Hello", "\x7D\x7D\x7E\x7E", ""};
    int endpoints[] = {0x42, 0x00, 0x05};
    for (size_t i = 0; i < sizeof(payload) / sizeof(*payload); ++i) {
        size_t msg_len = strlen(payload[i]);
        if (msg_len != 0) {
            expect_memory(__wrap_usb_processing_enqueue, buf, payload[i], msg_len);
        }
        expect_value(__wrap_usb_processing_enqueue, ctx, usb_processing_hww());
        expect_value(__wrap_usb_processing_enqueue, length, msg_len);
        expect_value(__wrap_usb_processing_enqueue, cmd, endpoints[i]);
        will_return(__wrap_usb_processing_enqueue, true);
        _tx_buffer(encoded[i], encoded_len[i]);
    }
}

static uint8_t* _realloc_or_die(uint8_t* buf, size_t new_size)
{
    uint8_t* new_buf = realloc(buf, new_size);
    if (!new_buf) {
        fprintf(stderr, "realloc() failed.\n");
        abort();
    }
    return new_buf;
}

static void exchange_consecutive_msgs(void)
{
    size_t n_msgs = rand() % 100;
    uint8_t* buf = NULL;
    uint8_t* payload[n_msgs];
    uint8_t endpoint[n_msgs];
    size_t payload_size[n_msgs];
    for (size_t i = 0; i < n_msgs; ++i) {
        payload[i] = NULL;
    }
    size_t total_len = 0;
    for (size_t i = 0; i < n_msgs; ++i) {
        bool valid = rand() % 2;
        bool long_msg = !valid && (rand() % 2 == 0);
        size_t append_length;
        uint8_t* msg;
        if (valid || long_msg) {
            size_t msg_size;
            if (long_msg) {
                /* Inject a very long message. */
                msg_size = _long_msg_size();
            } else {
                msg_size = _normal_msg_size();
            }
            payload[i] = _create_payload_buf(msg_size);
            msg = _create_msg_buf(msg_size);
            endpoint[i] = rand();
            _random_payload(payload[i], msg_size);
            payload_size[i] = msg_size;
            size_t encoded_size = _encode_msg(msg, endpoint[i], payload[i], msg_size);
            append_length = encoded_size - 1;
        } else {
            /* Insert a vary short message. */
            size_t msg_size = rand() % 4;
            msg = _malloc_or_die(msg_size * 2 + 1);
            msg[0] = 0x7E;
            uint8_t* pos = msg + 1;
            for (size_t j = 0; j < msg_size; ++j) {
                pos = _insert_in_buf(pos, rand());
            }
            append_length = pos - msg;
        }
        /* Append messages together, without adding the last 0x7E byte */
        buf = _realloc_or_die(buf, total_len + append_length);
        memcpy(buf + total_len, msg, append_length);
        total_len += append_length;
        if (valid) {
            expect_memory(__wrap_usb_processing_enqueue, buf, payload[i], payload_size[i]);
            expect_value(__wrap_usb_processing_enqueue, ctx, usb_processing_hww());
            expect_value(__wrap_usb_processing_enqueue, length, payload_size[i]);
            expect_value(__wrap_usb_processing_enqueue, cmd, endpoint[i]);
            will_return(__wrap_usb_processing_enqueue, true);
        }
        free(msg);
    }
    buf = _realloc_or_die(buf, total_len + 1);
    buf[total_len] = 0x7E;
    total_len++;
    _tx_buffer(buf, total_len);
    for (size_t i = 0; i < n_msgs; ++i) {
        free(payload[i]);
    }
    free(buf);
}

#define N_CONSECUTIVE_MSG_TESTS (100)
static void test_consecutive_msgs(void** state)
{
    (void)state;
    for (int i = 0; i < N_CONSECUTIVE_MSG_TESTS; ++i) {
        exchange_consecutive_msgs();
    }
}

static int _setup_test(void** state)
{
    (void)state;
    usart_frame_init();
    return 0;
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test_setup(test_basic, _setup_test),
        cmocka_unit_test_setup(test_normal_msgs, _setup_test),
        cmocka_unit_test_setup(test_short_msgs, _setup_test),
        cmocka_unit_test_setup(test_consecutive_msgs, _setup_test),
        cmocka_unit_test_setup(test_long_msgs, _setup_test),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
