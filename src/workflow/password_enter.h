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

#ifndef _PASSWORD_ENTER_H_
#define _PASSWORD_ENTER_H_

#include <compiler_util.h>
#include <ui/components/trinary_input_string.h> // for SET_PASSWORD_MAX_PASSWORD_LENGTH
#include <workflow/workflow.h>

#include <stdbool.h>

/**
 * Creates a new enter password workflow.
 *
 * @param[in] title title screen
 * @param[in] special_chars make the special characters keyboard available.
 * @param[in] callback Function to be invoked when the password is ready.
 *            It will receive a pointer to the password and the user-defined parameter.
 * @param[in] callback_param User-defined parameter that will be passed to the callback.
 * @return workflow_t object ready to be started.
 */
workflow_t* password_enter(
    const char* title,
    bool special_chars,
    void callback(const char* password, void* param),
    void* callback_param);

/**
 * Starts the enter password workflow.
 * This call blocks.
 * @param[in] title screen
 * @param[in] make the special characters keyboard available.
 * @param[out] password_out must be SET_PASSWORD_MAX_PASSWORD_LENGTH bytes (including null
 * terminator). Use `UTIL_CLEANUP_STR` to make sure that the password is destroyed after use.
 */
void password_enter_blocking(const char* title, bool special_chars, char* password_out);
#endif
