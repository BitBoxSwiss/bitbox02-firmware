// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <cmocka.h>

#include "usb/usb_processing.h"
#include <rust/rust.h>

/* Keep in sync with the thresholds in src/usb/usb_processing.c. */
static const int16_t USB_OUTSTANDING_OP_TIMEOUT_TICKS = 5;
static const int16_t USB_OUTSTANDING_OP_TIMEOUT_BLE_TICKS = 30;

static int hww_abort_count;
static int u2f_abort_count;

void __wrap_hww_abort_outstanding_op(void)
{
    hww_abort_count++;
}

void __wrap_u2f_abort_outstanding_op(void)
{
    u2f_abort_count++;
}

bool __wrap_rust_communication_mode_ble_enabled(void)
{
    return mock();
}

static void _test_hww_ble_uses_long_timeout(void** state)
{
    (void)state;

    hww_abort_count = 0;
    u2f_abort_count = 0;

    RustUsbReportQueue* hww_queue = rust_usb_report_queue_init();
    usb_processing_init(hww_queue);

    struct usb_processing* ctx = usb_processing_hww();

    usb_processing_lock(ctx);

    will_return(__wrap_rust_communication_mode_ble_enabled, true);
    usb_processing_timeout_reset((int16_t)(USB_OUTSTANDING_OP_TIMEOUT_TICKS + 1));
    usb_processing_process(ctx);
    assert_int_equal(hww_abort_count, 0);
    assert_int_equal(u2f_abort_count, 0);
    assert_true(usb_processing_locked(ctx));

    will_return(__wrap_rust_communication_mode_ble_enabled, true);
    usb_processing_timeout_reset(USB_OUTSTANDING_OP_TIMEOUT_BLE_TICKS);
    usb_processing_process(ctx);
    assert_int_equal(hww_abort_count, 0);
    assert_int_equal(u2f_abort_count, 0);
    assert_true(usb_processing_locked(ctx));

    will_return(__wrap_rust_communication_mode_ble_enabled, true);
    usb_processing_timeout_reset((int16_t)(USB_OUTSTANDING_OP_TIMEOUT_BLE_TICKS + 1));
    usb_processing_process(ctx);
    assert_int_equal(hww_abort_count, 1);
    assert_int_equal(u2f_abort_count, 0);
    assert_false(usb_processing_locked(ctx));

    rust_usb_report_queue_free(hww_queue);
}

static void _test_hww_usb_uses_short_timeout(void** state)
{
    (void)state;

    hww_abort_count = 0;
    u2f_abort_count = 0;

    RustUsbReportQueue* hww_queue = rust_usb_report_queue_init();
    usb_processing_init(hww_queue);

    struct usb_processing* ctx = usb_processing_hww();

    usb_processing_lock(ctx);

    will_return(__wrap_rust_communication_mode_ble_enabled, false);
    usb_processing_timeout_reset(USB_OUTSTANDING_OP_TIMEOUT_TICKS);
    usb_processing_process(ctx);
    assert_int_equal(hww_abort_count, 0);
    assert_int_equal(u2f_abort_count, 0);
    assert_true(usb_processing_locked(ctx));

    will_return(__wrap_rust_communication_mode_ble_enabled, false);
    usb_processing_timeout_reset((int16_t)(USB_OUTSTANDING_OP_TIMEOUT_TICKS + 1));
    usb_processing_process(ctx);
    assert_int_equal(hww_abort_count, 1);
    assert_int_equal(u2f_abort_count, 0);
    assert_false(usb_processing_locked(ctx));

    rust_usb_report_queue_free(hww_queue);
}

static void _test_u2f_keeps_short_timeout_when_ble_enabled(void** state)
{
    (void)state;

    hww_abort_count = 0;
    u2f_abort_count = 0;

    RustUsbReportQueue* hww_queue = rust_usb_report_queue_init();
    usb_processing_init(hww_queue);
    RustUsbReportQueue* u2f_queue = rust_usb_report_queue_init();
    usb_processing_init_u2f(u2f_queue);

    struct usb_processing* ctx = usb_processing_u2f();

    usb_processing_lock(ctx);

    /* rust_communication_mode_ble_enabled is not called for the U2F context. */
    usb_processing_timeout_reset(USB_OUTSTANDING_OP_TIMEOUT_TICKS);
    usb_processing_process(ctx);
    assert_int_equal(u2f_abort_count, 0);
    assert_int_equal(hww_abort_count, 0);
    assert_true(usb_processing_locked(ctx));

    usb_processing_timeout_reset((int16_t)(USB_OUTSTANDING_OP_TIMEOUT_TICKS + 1));
    usb_processing_process(ctx);
    assert_int_equal(u2f_abort_count, 1);
    assert_int_equal(hww_abort_count, 0);
    assert_false(usb_processing_locked(ctx));

    rust_usb_report_queue_free(u2f_queue);
    rust_usb_report_queue_free(hww_queue);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_hww_ble_uses_long_timeout),
        cmocka_unit_test(_test_hww_usb_uses_short_timeout),
        cmocka_unit_test(_test_u2f_keeps_short_timeout_when_ble_enabled),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
