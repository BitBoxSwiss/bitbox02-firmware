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
#include <ui/screen_stack.h>
#include <usb/usb_processing.h>

#include <stdlib.h>
#include <string.h>

static bool _hww_idle = true;
static bool _u2f_active = false;
static const uint8_t _pairing_key[] = {0x12, 0x34, 0x56, 0x78};
static void (*_pairing_callback)(bool, void*);
static void* _pairing_callback_param;

bool __wrap_rust_async_usb_is_idle(void)
{
    return _hww_idle;
}

bool __wrap_rust_workflow_u2f_is_active(void)
{
    return _u2f_active;
}

component_t* __wrap_confirm_create(
    const confirm_params_t* params,
    void (*callback)(bool, void*),
    void* callback_param)
{
    function_called();
    assert_string_equal(params->title, "Pairing code");
    assert_non_null(callback);
    _pairing_callback = callback;
    _pairing_callback_param = callback_param;
    component_t* component = calloc(1, sizeof(component_t));
    assert_non_null(component);
    return component;
}

static struct RustByteQueue* _send_pairing_code(const uint8_t key[4])
{
    uint8_t frame_buf[sizeof(struct da14531_protocol_frame) + 1 + sizeof(_pairing_key)] = {0};
    struct da14531_protocol_frame* frame = (struct da14531_protocol_frame*)frame_buf;
    frame->type = DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA;
    frame->payload_length = 1 + sizeof(_pairing_key);
    frame->payload[0] = CTRL_CMD_PAIRING_CODE;
    memcpy(&frame->payload[1], key, sizeof(_pairing_key));

    struct RustByteQueue* queue = rust_bytequeue_init(64);
    assert_non_null(queue);
    da14531_handler(frame, queue);
    return queue;
}

static void _assert_pairing_response(struct RustByteQueue* queue, const uint8_t key[4], bool ok)
{
    uint8_t response_payload[18] = {0};
    response_payload[0] = CTRL_CMD_TK_CONFIRM;
    memcpy(&response_payload[1], key, sizeof(_pairing_key));
    response_payload[17] = ok ? 1 : 0;
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
}

static void _assert_pairing_rejected(void)
{
    struct RustByteQueue* queue = _send_pairing_code(_pairing_key);
    assert_null(ui_screen_stack_top());
    _assert_pairing_response(queue, _pairing_key, false);
    assert_true(rust_bytequeue_free(queue));
}

static void test_pairing_code_rejected_while_hww_locked(void** state)
{
    (void)state;
    usb_processing_lock(usb_processing_hww());
    _assert_pairing_rejected();
    usb_processing_unlock();
}

static void test_pairing_code_rejected_during_unlocked_hww_workflow(void** state)
{
    (void)state;
    assert_false(usb_processing_locked(usb_processing_hww()));
    _hww_idle = false;
    _assert_pairing_rejected();
    _hww_idle = true;
}

static void test_pairing_code_rejected_during_unlocked_u2f_workflow(void** state)
{
    (void)state;
    assert_false(usb_processing_locked(usb_processing_u2f()));
    _u2f_active = true;
    _assert_pairing_rejected();
    _u2f_active = false;
}

static void test_pairing_code_accepted_while_idle(void** state)
{
    (void)state;
    expect_function_call(__wrap_confirm_create);
    struct RustByteQueue* queue = _send_pairing_code(_pairing_key);
    assert_non_null(ui_screen_stack_top());
    assert_int_equal(rust_bytequeue_num(queue), 0);
    ui_screen_stack_pop_and_clean();
    assert_true(rust_bytequeue_free(queue));
}

static void test_pairing_code_preserves_pending_prompt(void** state)
{
    (void)state;
    expect_function_call(__wrap_confirm_create);
    struct RustByteQueue* first_queue = _send_pairing_code(_pairing_key);
    component_t* first_prompt = ui_screen_stack_top();
    assert_non_null(first_prompt);

    const uint8_t second_key[4] = {0x87, 0x65, 0x43, 0x21};
    struct RustByteQueue* second_queue = _send_pairing_code(second_key);
    assert_ptr_equal(ui_screen_stack_top(), first_prompt);
    _assert_pairing_response(second_queue, second_key, false);
    assert_int_equal(rust_bytequeue_num(first_queue), 0);

    _pairing_callback(true, _pairing_callback_param);
    assert_null(ui_screen_stack_top());
    _assert_pairing_response(first_queue, _pairing_key, true);
    ui_screen_stack_cleanup();
    assert_true(rust_bytequeue_free(first_queue));
    assert_true(rust_bytequeue_free(second_queue));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_pairing_code_rejected_while_hww_locked),
        cmocka_unit_test(test_pairing_code_rejected_during_unlocked_hww_workflow),
        cmocka_unit_test(test_pairing_code_rejected_during_unlocked_u2f_workflow),
        cmocka_unit_test(test_pairing_code_preserves_pending_prompt),
        cmocka_unit_test(test_pairing_code_accepted_while_idle),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
