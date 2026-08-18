// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <da14531/da14531.h>
#include <da14531/da14531_handler.h>
#include <da14531/da14531_protocol.h>
#include <rust/rust.h>
#include <ui/components/confirm.h>
#include <usb/usb_processing.h>

#include <string.h>

component_t* __wrap_confirm_create(
    const confirm_params_t* params,
    void (*callback)(bool, void*),
    void* callback_param)
{
    (void)params;
    (void)callback;
    (void)callback_param;
    fail_msg("pairing confirmation created during an HWW workflow");
    return NULL;
}

static void test_pairing_code_rejected_during_hww_workflow(void** state)
{
    (void)state;

    const uint8_t key[] = {0x12, 0x34, 0x56, 0x78};
    uint8_t frame_buf[sizeof(struct da14531_protocol_frame) + 1 + sizeof(key)] = {0};
    struct da14531_protocol_frame* frame = (struct da14531_protocol_frame*)frame_buf;
    frame->type = DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA;
    frame->payload_length = 1 + sizeof(key);
    frame->payload[0] = CTRL_CMD_PAIRING_CODE;
    memcpy(&frame->payload[1], key, sizeof(key));

    struct RustByteQueue* queue = rust_bytequeue_init(64);
    assert_non_null(queue);
    usb_processing_lock(usb_processing_hww());
    da14531_handler(frame, queue);
    usb_processing_unlock();

    uint8_t response_payload[18] = {0};
    response_payload[0] = CTRL_CMD_TK_CONFIRM;
    memcpy(&response_payload[1], key, sizeof(key));
    uint8_t expected[12 + sizeof(response_payload) * 2] = {0};
    const uint16_t expected_len = da14531_protocol_format(
        expected,
        sizeof(expected),
        DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA,
        response_payload,
        sizeof(response_payload));
    assert_int_equal(rust_bytequeue_num(queue), expected_len);
    for (uint16_t i = 0; i < expected_len; i++) {
        uint8_t actual;
        assert_true(rust_bytequeue_get(queue, &actual));
        assert_int_equal(actual, expected[i]);
    }
    assert_true(rust_bytequeue_free(queue));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_pairing_code_rejected_during_hww_workflow),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
