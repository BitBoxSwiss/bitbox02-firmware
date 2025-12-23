// SPDX-License-Identifier: Apache-2.0

#ifndef _QTOUCH_H_
#define _QTOUCH_H_

#include <stdint.h>

void __wrap_qtouch_process(void);

uint8_t __wrap_qtouch_get_scroller_is_active(uint16_t sensor_node);

uint16_t __wrap_qtouch_get_scroller_position(uint16_t sensor_node);

#endif
