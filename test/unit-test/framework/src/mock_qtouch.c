// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include "mock_qtouch.h"

uint8_t qtouch_is_scroller_active(uint16_t sensor_node)
{
    return (uint8_t)mock();
}

uint16_t qtouch_get_scroller_position(uint16_t sensor_node)
{
    return (uint16_t)mock();
}
