// Copyright 2025 Shift Cryptosecurity AG
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
