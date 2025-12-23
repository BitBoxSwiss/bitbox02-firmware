// SPDX-License-Identifier: Apache-2.0

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

enum event_types {
    EVENT_SLIDE,
    EVENT_SLIDE_RELEASED,
    EVENT_LONG_TAP,
    EVENT_SHORT_TAP,
    EVENT_CONTINUOUS_TAP,
    EVENT_ENUM_MAX, // MAX must always be last, indicates the number of items in the enumration
};

typedef struct {
    uint8_t id;
    event_slider_data_t data;
} event_t;

#endif
