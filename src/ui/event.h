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

#ifndef _EVENT_H_
#define _EVENT_H_

#include <screen.h>
#include <stdint.h>

typedef struct {
    slider_location_t source;
    int16_t diff;
    uint16_t position;
    int32_t velocity;
} event_slider_data_t;

enum {
    EVENT_SLIDE,
    EVENT_SLIDE_RELEASED,
    EVENT_CONTINUOUS_TAP,
    EVENT_LONG_TAP,
    EVENT_SHORT_TAP,
};

typedef struct {
    uint8_t id;
    event_slider_data_t data;
} event_t;

#endif
