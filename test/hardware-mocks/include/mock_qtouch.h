// Copyright 2019 Shift Cryptosecurity AG
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

#ifndef _QTOUCH_H_
#define _QTOUCH_H_

#include <stdint.h>

void __wrap_qtouch_process(void);

uint8_t __wrap_qtouch_get_scroller_is_active(uint16_t sensor_node);

uint16_t __wrap_qtouch_get_scroller_position(uint16_t sensor_node);

void qtouch_process(void);

uint8_t qtouch_is_scroller_active(uint16_t sensor_node);

uint16_t qtouch_get_scroller_position(uint16_t sensor_node);

void qtouch_force_calibrate(void);

#endif
