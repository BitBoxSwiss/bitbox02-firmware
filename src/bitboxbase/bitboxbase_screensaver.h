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

#ifndef _BITBOXBASE_SCREENSAVER_H
#define _BITBOXBASE_SCREENSAVER_H

/**
 * @file
 *
 * bitboxbase screensaver is a timer that keeps track of whether to turn off the display or not.
 */

/**
 * Initialize screensaver
 */
void bitboxbase_screensaver_init(void);

/**
 * Check if screensaver should be enabled and enable it
 */
void bitboxbase_screensaver_process(void);

/**
 * Reset counter and disable screensaver
 */
void bitboxbase_screensaver_reset(void);

#endif
