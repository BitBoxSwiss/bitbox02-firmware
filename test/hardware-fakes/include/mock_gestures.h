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

#ifndef _MOCK_GESTURES_H_
#define _MOCK_GESTURES_H_

#include <screen.h>
#include <stdbool.h>
#include <stdint.h>

#include <touch/gestures.h>

/**
 * Mocks the sensors for sliding and tap or hold detection.
 * The flag 'expect_slide_detection' indicates whether the caller expects that the mocked gesture
 * detection returns after detecting a slide gesture. This flag needs to be set to false if the
 * caller requires mocking of the sensor node signal and sensor node reference driver functions.
 * @param[in] screen_position The position at which the touch slider was touched (between 0 and
 * 255).
 */
void mock_gestures_touch(slider_location_t location, uint8_t screen_position);

/**
 * Mocks the sensors for slider release detection.
 */
void mock_gestures_touch_release(void);

/**
 * Initializes the touch detection by resetting the detection history.
 */
void mock_gestures_touch_init(void);

#endif
