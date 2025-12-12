// SPDX-License-Identifier: Apache-2.0

#include "communication_mode.h"

#include "memory/memory_shared.h"

static bool _usb_hww_request_seen = false;

void communication_mode_ble_disable(void)
{
    _usb_hww_request_seen = true;
}

static bool _has_ble(void)
{
    static bool has_ble;
    static bool has_ble_initialized = false;
    if (!has_ble_initialized) {
        has_ble = memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS;
        has_ble_initialized = true;
    }
    return has_ble;
}

bool communication_mode_ble_enabled(void)
{
    return !_usb_hww_request_seen && _has_ble();
}
