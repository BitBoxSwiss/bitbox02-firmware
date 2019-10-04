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

#ifndef _WORKFLOW_H_
#define _WORKFLOW_H_

#include <stdbool.h>
#include <stdint.h>

/**
 * Pushes a confirm string on the screen a with a "Dismiss" button, to show data
 * on the screen for the user to verify.
 */
void workflow_confirm_dismiss(const char* title, const char* body);

/**
 * Switches to either the initialization or the unlock state depending on if the
 * device is initialized or not.
 */
void workflow_start(void);

/**
 * Loads the "Select orientation" screen.
 */
void workflow_start_orientation_screen(void);

#endif
