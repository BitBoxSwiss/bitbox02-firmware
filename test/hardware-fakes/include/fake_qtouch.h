// SPDX-License-Identifier: Apache-2.0

#ifndef _QTOUCH_H_
#define _QTOUCH_H_

#include <stdint.h>

void qtouch_process(void);

uint8_t qtouch_is_scroller_active(uint16_t sensor_node);

uint16_t qtouch_get_scroller_position(uint16_t sensor_node);

void qtouch_force_calibrate(void);

#endif
