// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <hww.h>
#include <rust/rust.h>
#include <usb/usb_frame.h>
#include <usb/usb_processing.h>
#include <version.h>

static bool _u2f_workflow_active = false;
static unsigned int _session_resets = 0;
static unsigned int _aborts = 0;
static UsbResponse _pending_response = UsbResponseNotReady;

bool __wrap_rust_workflow_u2f_is_active(void)
{
    return _u2f_workflow_active;
}

void __wrap_rust_hww_reset_session(void)
{
    _session_resets++;
    _pending_response = UsbResponseNack;
}

void __wrap_rust_async_usb_cancel(void)
{
    _aborts++;
}

UsbResponse __wrap_rust_async_usb_copy_response(buffer_t* out)
{
    if (_pending_response == UsbResponseAck) {
        out->data[0] = 0xaa;
        out->len = 1;
    }
    return _pending_response;
}

static int _setup(void** state)
{
    RustUsbReportQueue* queue = rust_usb_report_queue_init();
    assert_non_null(queue);
    usb_processing_init(queue);
    usb_processing_init_u2f(queue);
    usb_processing_timeout_reset(0);
    _u2f_workflow_active = false;
    _session_resets = 0;
    _aborts = 0;
    _pending_response = UsbResponseNotReady;
    *state = queue;
    return 0;
}

static int _teardown(void** state)
{
    if (usb_processing_locked(usb_processing_hww()) ||
        usb_processing_locked(usb_processing_u2f())) {
        usb_processing_unlock();
    }
    assert_true(rust_usb_report_queue_free(*state));
    return 0;
}

static USB_FRAME _query(void** state, const uint8_t* request, size_t len)
{
    RustUsbReportQueue* queue = *state;
    const uint32_t cid = 0x12345678;
    assert_true(usb_processing_enqueue(usb_processing_hww(), request, len, HWW_MSG, cid));
    usb_processing_process(usb_processing_hww());

    USB_FRAME response;
    assert_true(rust_usb_report_queue_pull(queue, (uint8_t*)&response));
    assert_int_equal(response.cid, cid);
    assert_int_equal(response.init.cmd, HWW_MSG);
    USB_FRAME extra;
    assert_false(rust_usb_report_queue_pull(queue, (uint8_t*)&extra));
    return response;
}

static void test_hww_new_request_is_busy_while_u2f_active(void** state)
{
    _u2f_workflow_active = true;
    const uint8_t request[] = {0x00, 'h'};
    USB_FRAME response = _query(state, request, sizeof(request));
    assert_int_equal(FRAME_MSG_LEN(response), 1);
    assert_int_equal(response.init.data[0], 2); // HWW_RSP_BUSY
}

static void test_hww_reset_session(void** state)
{
    usb_processing_lock(usb_processing_hww());
    _pending_response = UsbResponseAck;
    const uint8_t request[] = {3}; // HWW_REQ_RESET
    for (unsigned int i = 0; i < 2; i++) {
        USB_FRAME response = _query(state, request, sizeof(request));
        assert_int_equal(FRAME_MSG_LEN(response), 1);
        assert_int_equal(response.init.data[0], 0); // HWW_RSP_ACK
        assert_int_equal(_session_resets, i + 1);
        assert_false(usb_processing_locked(usb_processing_hww()));
    }
    const uint8_t retry[] = {1}; // HWW_REQ_RETRY
    USB_FRAME response = _query(state, retry, sizeof(retry));
    assert_int_equal(response.init.data[0], 3); // HWW_RSP_NACK
}

static void test_hww_reset_session_is_busy_while_u2f_active(void** state)
{
    _u2f_workflow_active = true;
    const uint8_t request[] = {3};
    USB_FRAME response = _query(state, request, sizeof(request));
    assert_int_equal(response.init.data[0], 2); // HWW_RSP_BUSY
    assert_int_equal(_session_resets, 0);
}

static void test_hww_control_requests_respect_u2f_lock(void** state)
{
    usb_processing_lock(usb_processing_u2f());
    const uint8_t requests[] = {'i', 3};
    for (size_t i = 0; i < sizeof(requests); i++) {
        USB_FRAME response = _query(state, &requests[i], 1);
        assert_int_equal(response.init.data[0], 2); // HWW_RSP_BUSY
        assert_true(usb_processing_locked(usb_processing_u2f()));
    }
    assert_int_equal(_session_resets, 0);
}

static void test_hww_reset_session_rejects_payload(void** state)
{
    const uint8_t request[] = {3, 0};
    USB_FRAME response = _query(state, request, sizeof(request));
    assert_int_equal(response.init.data[0], 3); // HWW_RSP_NACK
    assert_int_equal(_session_resets, 0);
    usb_processing_lock(usb_processing_hww());
    response = _query(state, request, sizeof(request));
    assert_int_equal(response.init.data[0], 2); // HWW_RSP_BUSY
    assert_true(usb_processing_locked(usb_processing_hww()));
    assert_int_equal(_session_resets, 0);
}

static void test_hww_info_preserves_response(void** state)
{
    usb_processing_lock(usb_processing_hww());
    _pending_response = UsbResponseAck;
    const uint8_t request[] = {'i'};
    USB_FRAME response = _query(state, request, sizeof(request));
    const size_t version_len = sizeof(DIGITAL_BITBOX_VERSION_SHORT) - 1;
    assert_int_equal(FRAME_MSG_LEN(response), version_len + 5);
    assert_int_equal(response.init.data[0], version_len);
    assert_memory_equal(response.init.data + 1, DIGITAL_BITBOX_VERSION_SHORT, version_len);
    assert_true(usb_processing_locked(usb_processing_hww()));
    assert_int_equal(_aborts, 0);
    const uint8_t retry[] = {1};
    response = _query(state, retry, sizeof(retry));
    assert_int_equal(FRAME_MSG_LEN(response), 2);
    assert_int_equal(response.init.data[0], 0); // HWW_RSP_ACK
    assert_int_equal(response.init.data[1], 0xaa);
    assert_false(usb_processing_locked(usb_processing_hww()));
}

static void test_hww_info_does_not_refresh_timeout(void** state)
{
    usb_processing_lock(usb_processing_hww());
    usb_processing_timeout_reset(6); // More than 500 ms without polling.
    const uint8_t request[] = {'i'};
    USB_FRAME response = _query(state, request, sizeof(request));
    assert_int_equal(response.init.data[0], sizeof(DIGITAL_BITBOX_VERSION_SHORT) - 1);
    assert_int_equal(_aborts, 1);
    assert_false(usb_processing_locked(usb_processing_hww()));
}

int main(void)
{
    hww_setup();
    const struct CMUnitTest tests[] = {
        cmocka_unit_test_setup_teardown(
            test_hww_new_request_is_busy_while_u2f_active, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_hww_reset_session, _setup, _teardown),
        cmocka_unit_test_setup_teardown(
            test_hww_reset_session_is_busy_while_u2f_active, _setup, _teardown),
        cmocka_unit_test_setup_teardown(
            test_hww_control_requests_respect_u2f_lock, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_hww_reset_session_rejects_payload, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_hww_info_preserves_response, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_hww_info_does_not_refresh_timeout, _setup, _teardown),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
