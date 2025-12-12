// SPDX-License-Identifier: Apache-2.0

#ifndef _MOTION_H_
#define _MOTION_H_

#include <stdbool.h>

#include <screen.h>
#include <ui/component.h>

#ifndef TESTING
    #include "qtouch.h"
    #define TOUCH_NUM_BUTTONS DEF_NUM_CHANNELS
    #define TOUCH_NUM_SLIDERS DEF_NUM_SCROLLERS
    #define MAX_SLIDER_POS (DEF_SCROLLER_RESOLUTION - 1)
#else
    #define TOUCH_NUM_BUTTONS (8)
    #define TOUCH_NUM_SLIDERS (2)
    #define MAX_SLIDER_POS (255)
#endif
#define SLIDER_POSITION_ONE_THIRD (MAX_SLIDER_POS / 3)
#define SLIDER_POSITION_TWO_THIRD (MAX_SLIDER_POS / 3 * 2)

/**
 * Detects a gestures and calls the respective callback.
 * @param[in] reset The flag indicates whether the gesture history should be
 * reset. This is the case, for example, if the screen component changes.
 * @param[in] emit_without_release The flag indicates if touch sensors must be
 * released before touch events can be emitted. This parameter is only used
 * when `reset` is `true`.
 */
void gestures_detect(bool reset, bool emit_without_release);

#endif
