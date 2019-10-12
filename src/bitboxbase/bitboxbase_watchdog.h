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

#include <stdbool.h>

// The watchdog creates a timer which increases a counter every second.
// If the counter is a bove a certain threshold, the `check` function will return true.
// Whenever the `reset` function is called the counter is reset to 0.
// This is used by the bitboxbase heartbeat. Every time there is an heartbeat the watchdog is reset
// and in every iteration of the main loop the watchdog is checked.

/**
 * Initialize watchdog
 */
void bitboxbase_watchdog_init(void);

/**
 * Check if watchdog counter has elapsed
 */
bool bitboxbase_watchdog_check(void);

/**
 * Reset the watchdog counter
 */
void bitboxbase_watchdog_reset(void);
