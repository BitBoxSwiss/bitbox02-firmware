// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <hww.h>
#include <rust/rust.h>
#include <usb/usb_frame.h>
#include <usb/usb_processing.h>

static bool _u2f_workflow_active = false;

bool __wrap_rust_workflow_u2f_is_active(void)
{
    return _u2f_workflow_active;
}

static void test_hww_new_request_is_busy_while_u2f_active(void** state)
{
    (void)state;

    RustUsbReportQueue* queue = rust_usb_report_queue_init();
    assert_non_null(queue);
    usb_processing_init(queue);
    hww_setup();

    _u2f_workflow_active = true;
    const uint8_t request[] = {0x00, 'h'};
    const uint32_t cid = 0x12345678;
    assert_true(
        usb_processing_enqueue(usb_processing_hww(), request, sizeof(request), HWW_MSG, cid));
    usb_processing_process(usb_processing_hww());

    USB_FRAME response;
    assert_true(rust_usb_report_queue_pull(queue, (uint8_t*)&response));
    assert_int_equal(response.cid, cid);
    assert_int_equal(response.init.cmd, HWW_MSG);
    assert_int_equal(FRAME_MSG_LEN(response), 1);
    assert_int_equal(response.init.data[0], 2); // HWW_RSP_BUSY
    assert_false(rust_usb_report_queue_pull(queue, (uint8_t*)&response));
    assert_true(rust_usb_report_queue_free(queue));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_hww_new_request_is_busy_while_u2f_active),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
