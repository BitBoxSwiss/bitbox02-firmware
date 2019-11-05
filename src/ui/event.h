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

#include <stdint.h>

enum {
    EVENT_CONFIRM,
    EVENT_CANCEL,
    EVENT_FORWARD,
    EVENT_BACKWARD,
    EVENT_TOGGLE_ALPHANUMERIC,
    EVENT_UPDATE_ALPHANUMERIC,
    EVENT_CHANGE,
    EVENT_WAIT_CONFIRM,
    EVENT_ABORT_CONFIRM,
    EVENT_BOTTOM_SLIDE,
    EVENT_TOP_SLIDE,
    EVENT_BOTTOM_SLIDE_RELEASED,
    EVENT_TOP_SLIDE_RELEASED,
    EVENT_TOP_CONTINUOUS_TAP,
    EVENT_BOTTOM_CONTINUOUS_TAP,
    EVENT_TOP_LONG_TAP,
    EVENT_BOTTOM_LONG_TAP,
    EVENT_TOP_SHORT_TAP,
    EVENT_BOTTOM_SHORT_TAP,
    EVENT_BUTTON_SHORT_TAP,
    EVENT_BUTTON_LONG_TAP,
    EVENT_BUTTON_CONTINUOUS_TAP,
};

typedef struct {
    void* data;
    uint8_t id;
} event_t;

#endif
